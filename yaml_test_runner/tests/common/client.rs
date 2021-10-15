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

use opensearch::{
    auth::Credentials,
    cat::CatTemplatesParts,
    cert::CertificateValidation,
    cluster::ClusterHealthParts,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Method, StatusCode,
    },
    ilm::IlmRemovePolicyParts,
    indices::{
        IndicesDeleteIndexTemplateParts, IndicesDeleteParts, IndicesDeleteTemplateParts,
        IndicesRefreshParts,
    },
    ml::{
        MlCloseJobParts, MlDeleteDatafeedParts, MlDeleteJobParts, MlGetDatafeedsParts,
        MlGetJobsParts, MlStopDatafeedParts,
    },
    params::{ExpandWildcards, WaitForStatus},
    security::{
        SecurityDeletePrivilegesParts, SecurityDeleteRoleParts, SecurityDeleteUserParts,
        SecurityGetPrivilegesParts, SecurityGetRoleParts, SecurityGetUserParts,
        SecurityPutUserParts,
    },
    snapshot::{SnapshotDeleteParts, SnapshotDeleteRepositoryParts},
    tasks::TasksCancelParts,
    transform::{
        TransformDeleteTransformParts, TransformGetTransformParts, TransformStopTransformParts,
    },
    watcher::WatcherDeleteWatchParts,
    OpenSearch, Error, DEFAULT_ADDRESS,
};
use once_cell::sync::Lazy;
use serde_json::{json, Value};
use std::ops::Deref;
use sysinfo::SystemExt;
use url::Url;

fn cluster_addr() -> String {
    match std::env::var("OPENSEARCH_URL") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Determines if Fiddler.exe proxy process is running
fn running_proxy() -> bool {
    let system = sysinfo::System::new();
    !system.get_process_by_name("Fiddler").is_empty()
}

static GLOBAL_CLIENT: Lazy<OpenSearch> = Lazy::new(|| {
    let mut url = Url::parse(cluster_addr().as_ref()).unwrap();

    // if the url is https and specifies a username and password, remove from the url and set credentials
    let credentials = if url.scheme() == "https" {
        let username = if !url.username().is_empty() {
            let u = url.username().to_string();
            url.set_username("").unwrap();
            u
        } else {
            "elastic".into()
        };

        let password = match url.password() {
            Some(p) => {
                let pass = p.to_string();
                url.set_password(None).unwrap();
                pass
            }
            None => "changeme".into(),
        };

        Some(Credentials::Basic(username, password))
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
) -> Result<(Method, StatusCode, String, Value), failure::Error> {
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

/// general setup step for an OSS yaml test
pub async fn general_cluster_setup() -> Result<(), Error> {
    let client = get();
    delete_data_streams(client).await?;
    delete_indices(client).await?;
    delete_templates(client).await?;
    delete_snapshots(client).await?;

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

    if cat_repo_response.len() > 0 {
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

async fn wait_for_yellow_status(client: &OpenSearch) -> Result<(), Error> {
    let cluster_health = client
        .cluster()
        .health(ClusterHealthParts::None)
        .wait_for_status(WaitForStatus::Yellow)
        .send()
        .await?;

    assert_response_success!(cluster_health);
    Ok(())
}

async fn delete_data_streams(client: &OpenSearch) -> Result<(), Error> {
    // Hand-crafted request as the indices.delete_data_stream spec doesn't yet have the
    // "expand_wildcards" parameter that is needed to delete ILM data streams
    //
    // Not deleting data streams yields errors like this when trying to delete hidden indices:
    // {
    //   "type":"illegal_argument_exception"
    //   "reason":"index [.ds-ilm-history-5-2021.02.14-000001] is the write index for data
    //             stream [ilm-history-5] and cannot be deleted"
    // }
    //
    // Quoting the docs:
    // You cannot delete the current write index of a data stream. To delete the index,
    // you must roll over the data stream so a new write index is created. You can then use
    // the delete index API to delete the previous write index.
    //
    let delete_response = client
        .transport()
        .send(
            Method::Delete,
            "/_data_stream/*",
            opensearch::http::headers::HeaderMap::new(),
            Some(&[("expand_wildcards", "hidden")]),
            None::<()>, // body
            None,       // timeout
        )
        .await?;

    assert_response_success!(delete_response);

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

async fn stop_and_delete_transforms(client: &OpenSearch) -> Result<(), Error> {
    let transforms_response = client
        .transform()
        .get_transform(TransformGetTransformParts::TransformId("_all"))
        .send()
        .await?
        .json::<Value>()
        .await?;

    for transform in transforms_response["transforms"].as_array().unwrap() {
        let id = transform["id"].as_str().unwrap();
        let response = client
            .transform()
            .stop_transform(TransformStopTransformParts::TransformId(id))
            .send()
            .await?;

        assert_response_success!(response);

        let response = client
            .transform()
            .delete_transform(TransformDeleteTransformParts::TransformId(id))
            .send()
            .await?;

        assert_response_success!(response);
    }

    Ok(())
}

async fn cancel_tasks(client: &OpenSearch) -> Result<(), Error> {
    let rollup_response = client.tasks().list().send().await?.json::<Value>().await?;

    for (_node_id, nodes) in rollup_response["nodes"].as_object().unwrap() {
        for (task_id, task) in nodes["tasks"].as_object().unwrap() {
            if let Some(b) = task["cancellable"].as_bool() {
                if b {
                    let response = client
                        .tasks()
                        .cancel(TasksCancelParts::TaskId(task_id))
                        .send()
                        .await?;

                    assert_response_success!(response);
                }
            }
        }
    }

    Ok(())
}

async fn delete_templates(client: &OpenSearch) -> Result<(), Error> {
    // There are "legacy templates and "new templates"

    let cat_template_response = client
        .cat()
        .templates(CatTemplatesParts::Name("*"))
        .h(&["name"])
        .send()
        .await?
        .text()
        .await?;

    let all_templates: Vec<&str> = cat_template_response
        .split_terminator('\n')
        .filter(|s| !s.starts_with('.') && s != &"security-audit-log")
        .collect();

    for template in all_templates {
        if template == "ilm-history" {
            // We may need to extend this to mimic ESRestTestCase.isXPackTemplate() from the ES
            // test harness
            continue;
        }

        let mut delete_template_response = client
            .indices()
            .delete_index_template(IndicesDeleteIndexTemplateParts::Name(&template))
            .send()
            .await?;

        if delete_template_response.status_code().as_u16() == 404 {
            // Certainly an old-style template
            delete_template_response = client
                .indices()
                .delete_template(IndicesDeleteTemplateParts::Name(&template))
                .send()
                .await?;
        }
        assert_response_success!(delete_template_response);
    }

    Ok(())
}

async fn delete_users(client: &OpenSearch) -> Result<(), Error> {
    let users_response = client
        .security()
        .get_user(SecurityGetUserParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in users_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_user(SecurityDeleteUserParts::Username(k))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}

async fn delete_roles(client: &OpenSearch) -> Result<(), Error> {
    let roles_response = client
        .security()
        .get_role(SecurityGetRoleParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in roles_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_role(SecurityDeleteRoleParts::Name(k))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}

async fn delete_privileges(client: &OpenSearch) -> Result<(), Error> {
    let privileges_response = client
        .security()
        .get_privileges(SecurityGetPrivilegesParts::None)
        .send()
        .await?
        .json::<Value>()
        .await?;

    for (k, v) in privileges_response.as_object().unwrap() {
        if let Some(b) = v["metadata"]["_reserved"].as_bool() {
            if !b {
                let response = client
                    .security()
                    .delete_privileges(SecurityDeletePrivilegesParts::ApplicationName(k, "_all"))
                    .send()
                    .await?;

                assert_response_success!(response);
            }
        }
    }

    Ok(())
}
