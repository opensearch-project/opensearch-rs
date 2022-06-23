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
    auth::Credentials,
    cert::CertificateValidation,
    cluster::ClusterHealthParts,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Method, StatusCode,
    },
    indices::{
        IndicesDeleteParts,
    },
    params::{ExpandWildcards, WaitForStatus},
    snapshot::{SnapshotDeleteParts, SnapshotDeleteRepositoryParts},
    tasks::TasksCancelParts,
    Error, OpenSearch, DEFAULT_ADDRESS,
};
use serde_json::{Value};
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
            "admin".into()
        };

        let password = match url.password() {
            Some(p) => {
                let pass = p.to_string();
                url.set_password(None).unwrap();
                pass
            }
            None => "admin".into(),
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
    delete_indices(client).await?;
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
