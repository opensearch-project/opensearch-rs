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

use std::ffi::OsStr;

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
use opensearch::cert::CertificateValidation;
use opensearch::{
    auth::Credentials,
    http::{
        response::Response,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        StatusCode,
    },
    indices::IndicesExistsParts,
    params::Refresh,
    BulkOperation, BulkParts, Error, OpenSearch, DEFAULT_ADDRESS,
};
use serde_json::json;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};
use url::Url;

/// Gets the address to the OpenSearch instance from environment variables
/// and assumes an instance running locally on the default port otherwise
pub fn cluster_addr() -> String {
    match std::env::var("OPENSEARCH_URL") {
        Ok(server) => server,
        Err(_) => DEFAULT_ADDRESS.into(),
    }
}

/// Checks if Fiddler proxy process is running
fn running_proxy() -> bool {
    let system = System::new_with_specifics(
        RefreshKind::nothing().with_processes(ProcessRefreshKind::default()),
    );
    let has_fiddler = system
        .processes_by_name(OsStr::new("Fiddler"))
        .next()
        .is_some();
    has_fiddler
}

pub struct TestClientBuilder(TransportBuilder);

impl TestClientBuilder {
    pub fn new() -> Self {
        Self::with_url(&cluster_addr())
    }

    pub fn with_url(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        let secure = url.scheme() == "https";
        let conn_pool = SingleNodeConnectionPool::new(url);
        let mut builder = TransportBuilder::new(conn_pool);

        // assume if we're running with HTTPS then authentication is also enabled and disable
        // certificate validation - we'll change this for tests that need to.
        if secure {
            builder = builder.auth(Credentials::Basic(
                "admin".into(),
                std::env::var("OPENSEARCH_PASSWORD").unwrap_or("admin".into()),
            ));

            #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
            {
                builder = builder.cert_validation(CertificateValidation::None);
            }
        }

        Self(builder)
    }

    pub fn with(mut self, configurator: impl FnOnce(TransportBuilder) -> TransportBuilder) -> Self {
        self.0 = configurator(self.0);
        self
    }

    pub fn build(self) -> OpenSearch {
        let mut builder = self.0;

        if running_proxy() {
            let proxy_url = Url::parse("http://localhost:8888").unwrap();
            builder = builder.proxy(proxy_url, None, None);
        }

        let transport = builder.build().unwrap();
        OpenSearch::new(transport)
    }
}

impl Default for TestClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub fn builder() -> TestClientBuilder {
    TestClientBuilder::new()
}

pub fn builder_with_url(url: &str) -> TestClientBuilder {
    TestClientBuilder::with_url(url)
}

pub fn create() -> OpenSearch {
    builder().build()
}

pub fn create_with(configurator: impl FnOnce(TransportBuilder) -> TransportBuilder) -> OpenSearch {
    builder().with(configurator).build()
}

pub fn create_with_url(url: &str) -> OpenSearch {
    builder_with_url(url).build()
}

/// index some documents into a posts index. If the posts index already exists, do nothing.
///
/// As an async fn, this can end up running multiple times concurrently, and indexing documents
/// several times. In this instance, this is fine.
///
/// TODO: This is a temporary measure until https://github.com/elastic/elasticsearch-rs/issues/19 is implemented.
pub async fn index_documents(client: &OpenSearch) -> Result<Response, Error> {
    let index = "posts";
    let exists_response = client
        .indices()
        .exists(IndicesExistsParts::Index(&[index]))
        .send()
        .await?;

    if exists_response.status_code() == StatusCode::NOT_FOUND {
        let mut body: Vec<BulkOperation<_>> = vec![];
        for i in 1..=10 {
            let op = BulkOperation::index(json!({"title":"OpenSearch"}))
                .id(i.to_string())
                .into();
            body.push(op);
        }

        client
            .bulk(BulkParts::Index(index))
            .body(body)
            .refresh(Refresh::WaitFor)
            .send()
            .await
    } else {
        Ok(exists_response)
    }
}
