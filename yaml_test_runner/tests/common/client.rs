/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 *
 * Modifications Copyright OpenSearch Contributors. See
 * GitHub history for details.
 */

use once_cell::sync::Lazy;
use opensearch::{
    auth::{ClientCertificate, Credentials},
    cert::CertificateValidation,
    cluster::ClusterDeleteComponentTemplateParts,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Method, StatusCode,
    },
    indices::{IndicesDeleteIndexTemplateParts, IndicesDeleteParts, IndicesDeleteTemplateParts},
    params::ExpandWildcards,
    snapshot::{SnapshotDeleteParts, SnapshotDeleteRepositoryParts},
    Error, OpenSearch, DEFAULT_ADDRESS,
};
use serde_json::Value;
use std::{ffi::OsStr, ops::Deref};
use url::Url;

use crate::assert_response_success;

const KIRK_P12: &[u8] = include_bytes!("../../../.ci/certs/kirk.p12");

fn cluster_addr() -> String {
    match std::env::var("OPENSEARCH_URL") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Determines if Fiddler.exe proxy process is running
fn running_proxy() -> bool {
    let mut system = sysinfo::System::new();
    system.refresh_processes(sysinfo::ProcessesToUpdate::All, true);
    let running = system
        .processes_by_name(OsStr::new("Fiddler"))
        .next()
        .is_some();
    running
}

static GLOBAL_CLIENT: Lazy<OpenSearch> = Lazy::new(|| {
    let url = Url::parse(cluster_addr().as_ref()).unwrap();

    // if the url is https, set credentials
    let credentials = if url.scheme() == "https" {
        let cert = ClientCertificate::Pkcs12(KIRK_P12.into(), Some("kirk".to_string()));
        Some(Credentials::Certificate(cert))
    } else {
        None
    };

    let conn_pool = SingleNodeConnectionPool::new(url);
    let mut builder = TransportBuilder::new(conn_pool);

    builder = match credentials {
        Some(c) => builder.auth(c).cert_validation(CertificateValidation::None),
        None => builder,
    };

    if running_proxy() {
        let proxy_url = Url::parse("http://localhost:8888").unwrap();
        builder = builder.proxy(proxy_url, None, None);
    }

    let transport = builder.build().unwrap();
    OpenSearch::new(transport)
});

/// Gets the client to use in tests
pub fn get() -> &'static OpenSearch {
    GLOBAL_CLIENT.deref()
}

/// Reads the response from OpenSearch, returning the method, status code, text response,
/// and the response parsed from json or yaml
pub async fn read_response(
    response: Response,
) -> anyhow::Result<(Method, StatusCode, String, Value)> {
    let is_json = response.content_type().starts_with("application/json");
    let is_yaml = response.content_type().starts_with("application/yaml");
    let method = response.method();
    let status_code = response.status_code();
    let text = response.text().await?;
    let json = if is_json && !text.is_empty() {
        serde_json::from_str::<Value>(text.as_ref())?
    } else if is_yaml && !text.is_empty() {
        serde_yaml::from_str::<Value>(text.as_ref())?
    } else {
        Value::Null
    };

    Ok((method, status_code, text, json))
}

/// How typed-response divergences are reported, from the `TYPED_ASSERT`
/// environment variable: `off` skips the check entirely, `fail` panics,
/// anything else (the default) logs a warning to stderr.
fn typed_assert_mode() -> String {
    std::env::var("TYPED_ASSERT").unwrap_or_else(|_| "warn".to_string())
}

/// Shadow typing verification: checks that the successful JSON response of
/// a typed endpoint deserializes into its generated response model `T` and
/// serializes back to the same JSON (i.e. the model loses no data).
///
/// This runs alongside the untyped yaml assertions of the generated tests,
/// turning every yaml test against a real cluster into a verification
/// corpus for the generated response models. A divergence means either a
/// bug in the type generation or a bug in the API specification.
///
/// The comparison normalizes two representation differences that are not
/// data loss: integer vs float encodings of the same number (`42` vs
/// `42.0`), and an explicit `null` vs an absent optional field.
///
/// Non-JSON responses are skipped. Reporting is controlled by the
/// `TYPED_ASSERT` environment variable, see [typed_assert_mode].
pub fn check_typed_response<T>(text: &str, endpoint: &str)
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let mode = typed_assert_mode();
    if mode == "off" {
        return;
    }
    if let Some(message) = typed_divergence::<T>(text, endpoint) {
        if mode == "fail" {
            panic!("{}", message);
        } else {
            eprintln!("WARN {}", message);
        }
    }
}

/// The failure message of the typed-response verification for the given
/// response body, or `None` when the body is not JSON (nothing to verify)
/// or the model round-trips it faithfully
fn typed_divergence<T>(text: &str, endpoint: &str) -> Option<String>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    // only JSON bodies are described by the response models
    let original: Value = serde_json::from_str(text).ok()?;

    let typed: T = match serde_json::from_str(text) {
        Ok(t) => t,
        Err(e) => {
            return Some(format!(
                "typed response check [{}]: response does not deserialize into the generated model: {}. Response: {}",
                endpoint, e, text
            ));
        }
    };

    let back = match serde_json::to_value(&typed) {
        Ok(v) => v,
        Err(e) => {
            return Some(format!(
                "typed response check [{}]: re-serialization failed: {}",
                endpoint, e
            ));
        }
    };

    first_divergence(&original, &back, String::new()).map(|path| {
        format!(
            "typed response check [{}]: round-trip diverged at `{}`. Response: {}",
            endpoint, path, text
        )
    })
}

/// The path of the first divergence between the original response JSON and
/// the round-tripped JSON, or `None` when they are equivalent.
///
/// Two normalizations apply: numbers are compared by value regardless of
/// integer/float encoding, and a missing object key is equivalent to an
/// explicit `null` (optional model fields are skipped when `None`).
fn first_divergence(original: &Value, back: &Value, path: String) -> Option<String> {
    match (original, back) {
        (Value::Object(a), Value::Object(b)) => {
            let keys: std::collections::BTreeSet<&String> = a.keys().chain(b.keys()).collect();
            for key in keys {
                let left = a.get(key).unwrap_or(&Value::Null);
                let right = b.get(key).unwrap_or(&Value::Null);
                let child = format!("{}.{}", path, key);
                if let Some(p) = first_divergence(left, right, child) {
                    return Some(p);
                }
            }
            None
        }
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return Some(format!(
                    "{} (array length {} vs {})",
                    path,
                    a.len(),
                    b.len()
                ));
            }
            for (i, (left, right)) in a.iter().zip(b.iter()).enumerate() {
                let child = format!("{}[{}]", path, i);
                if let Some(p) = first_divergence(left, right, child) {
                    return Some(p);
                }
            }
            None
        }
        (Value::Number(a), Value::Number(b)) => {
            // compare by value: `42` and `42.0` encode the same number.
            // Integers beyond f64 precision compare exactly first.
            let eq = a == b
                || match (a.as_f64(), b.as_f64()) {
                    (Some(x), Some(y)) => x == y,
                    _ => false,
                };
            if eq {
                None
            } else {
                Some(format!("{} ({} vs {})", path, a, b))
            }
        }
        (left, right) => {
            if left == right {
                None
            } else {
                Some(if path.is_empty() {
                    "(root)".to_string()
                } else {
                    path
                })
            }
        }
    }
}

#[cfg(test)]
mod typed_response_tests {
    use super::{first_divergence, typed_divergence};
    use opensearch::types::search::SearchResponse;
    use serde_json::json;

    fn divergence(a: &serde_json::Value, b: &serde_json::Value) -> Option<String> {
        first_divergence(a, b, String::new())
    }

    #[test]
    fn a_realistic_search_response_passes_the_check() {
        // full path through the generated model, including a lazily
        // decoded aggregation and an explicit null the model drops
        let text = json!({
            "took": 5,
            "timed_out": false,
            "_shards": { "total": 2, "successful": 2, "skipped": 0, "failed": 0 },
            "hits": {
                "total": { "value": 42, "relation": "eq" },
                "max_score": null,
                "hits": [
                    { "_index": "movies", "_id": "1", "_score": 1.3,
                      "_source": { "title": "Moneyball", "year": 2011 } }
                ]
            },
            "aggregations": {
                "avg_year": { "value": 2001.25 },
                "some_plugin_agg": { "custom": { "shape": [1, 2] } }
            }
        })
        .to_string();
        assert_eq!(typed_divergence::<SearchResponse>(&text, "search"), None);
    }

    #[test]
    fn a_missing_required_field_is_reported() {
        // `took` is required by the model
        let text = json!({
            "timed_out": false,
            "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
            "hits": { "hits": [] }
        })
        .to_string();
        let message = typed_divergence::<SearchResponse>(&text, "search").unwrap();
        assert!(message.contains("does not deserialize"), "{}", message);
        assert!(message.contains("[search]"), "{}", message);
    }

    #[test]
    fn non_json_bodies_are_skipped() {
        assert_eq!(
            typed_divergence::<SearchResponse>("green open index1\n", "search"),
            None
        );
        assert_eq!(typed_divergence::<SearchResponse>("", "search"), None);
    }

    #[test]
    fn the_call_shape_emitted_by_the_generator_compiles() {
        // mirrors the exact call emitted into generated tests, where
        // `text` is the String binding produced by client::read_response
        let text = json!({
            "took": 1,
            "timed_out": false,
            "_shards": { "total": 1, "successful": 1, "skipped": 0, "failed": 0 },
            "hits": { "hits": [] }
        })
        .to_string();
        super::check_typed_response::<SearchResponse>(&text, "search");
    }

    #[test]
    fn equal_json_has_no_divergence() {
        let v = json!({ "took": 5, "hits": { "hits": [{ "_id": "1" }] } });
        assert_eq!(divergence(&v, &v), None);
    }

    #[test]
    fn integer_and_float_encodings_are_equivalent() {
        assert_eq!(
            divergence(&json!({ "value": 42 }), &json!({ "value": 42.0 })),
            None
        );
        assert_eq!(
            divergence(&json!({ "value": 42.5 }), &json!({ "value": 42 })),
            Some(".value (42.5 vs 42)".to_string())
        );
    }

    #[test]
    fn explicit_null_and_missing_key_are_equivalent() {
        // optional model fields serialize as absent, servers send explicit null
        assert_eq!(
            divergence(
                &json!({ "max_score": null, "total": 1 }),
                &json!({ "total": 1 })
            ),
            None
        );
    }

    #[test]
    fn a_dropped_field_is_reported_with_its_path() {
        assert_eq!(
            divergence(
                &json!({ "hits": { "total": 1, "extra": "x" } }),
                &json!({ "hits": { "total": 1 } })
            ),
            Some(".hits.extra".to_string())
        );
    }

    #[test]
    fn a_changed_array_element_is_reported_with_its_index() {
        assert_eq!(
            divergence(
                &json!({ "items": [{ "a": 1 }, { "a": 2 }] }),
                &json!({ "items": [{ "a": 1 }, { "a": 3 }] })
            ),
            Some(".items[1].a (2 vs 3)".to_string())
        );
    }

    #[test]
    fn array_length_mismatch_is_reported() {
        assert_eq!(
            divergence(&json!([1, 2, 3]), &json!([1, 2])),
            Some(" (array length 3 vs 2)".to_string())
        );
    }

    #[test]
    fn type_mismatch_at_root_is_reported() {
        assert_eq!(
            divergence(&json!("a"), &json!(1)),
            Some("(root)".to_string())
        );
    }
}

/// general setup step for an OSS yaml test
pub async fn general_cluster_setup() -> Result<(), Error> {
    let client = get();
    reset_cluster_settings(client).await?;
    delete_indices(client).await?;
    delete_snapshots(client).await?;
    delete_templates(client).await?;

    Ok(())
}

/// Clears the persistent and transient cluster settings a previous test may
/// have left behind: a test disabling shard allocation would otherwise make
/// every later test fail with unassigned shards
async fn reset_cluster_settings(client: &OpenSearch) -> Result<(), Error> {
    let response = client
        .cluster()
        .put_settings()
        .body(serde_json::json!({
            "persistent": { "*": null },
            "transient": { "*": null }
        }))
        .send()
        .await?;

    assert_response_success!(response);
    Ok(())
}

pub async fn delete_snapshots(client: &OpenSearch) -> Result<(), Error> {
    let cat_repo_response = client
        .cat()
        .repositories()
        .h(&["id"])
        .send()
        .await?
        .error_for_status_code()?
        .text()
        .await?;

    if !cat_repo_response.is_empty() {
        let repositories: Vec<&str> = cat_repo_response.split_terminator('\n').collect();

        // Delete snapshots in each repository
        for repo in repositories {
            let delete_snapshots_response = client
                .snapshot()
                .delete(SnapshotDeleteParts::RepositorySnapshot(repo, "*"))
                .send()
                .await?;

            assert_response_success!(delete_snapshots_response);
        }

        // Delete all snapshot repositories
        let delete_repo_response = client
            .snapshot()
            .delete_repository(SnapshotDeleteRepositoryParts::Repository(&["*"]))
            .send()
            .await?;

        assert_response_success!(delete_repo_response);
    }

    Ok(())
}

async fn delete_indices(client: &OpenSearch) -> Result<(), Error> {
    let delete_response = client
        .indices()
        .delete(IndicesDeleteParts::Index(&["*"]))
        .expand_wildcards(&[
            ExpandWildcards::Open,
            ExpandWildcards::Closed,
            ExpandWildcards::Hidden,
        ])
        .send()
        .await?;

    assert_response_success!(delete_response);
    Ok(())
}

async fn delete_templates(client: &OpenSearch) -> Result<(), Error> {
    let delete_response = client
        .indices()
        .delete_template(IndicesDeleteTemplateParts::Name("*"))
        .send()
        .await?;

    assert_response_success!(delete_response);

    let delete_response = client
        .indices()
        .delete_index_template(IndicesDeleteIndexTemplateParts::Name("*"))
        .send()
        .await?;

    assert_response_success!(delete_response);

    let delete_response = client
        .cluster()
        .delete_component_template(ClusterDeleteComponentTemplateParts::Name("*"))
        .send()
        .await?;

    assert_response_success!(delete_response);

    Ok(())
}
