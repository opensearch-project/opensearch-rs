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

#[tokio::main]
#[cfg(feature = "aws-auth")]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use opensearch::{
        auth::Credentials,
        cat::CatIndicesParts,
        http::transport::{SingleNodeConnectionPool, TransportBuilder},
        OpenSearch,
    };
    use url::Url;

    let aws_config = aws_config::load_from_env().await;

    let host = ""; // e.g. https://search-mydomain.us-west-1.es.amazonaws.com
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(
        Url::parse(host).unwrap(),
    ))
    .auth(Credentials::Aws(
        aws_config.credentials_provider().unwrap().clone(),
        aws_config.region().unwrap().clone(),
    ))
    .build()?;
    let client = OpenSearch::new(transport);

    let response = client
        .cat()
        .indices(CatIndicesParts::None)
        .v(true)
        .send()
        .await?;

    let text = response.text().await?;
    println!("{}", text);
    Ok(())
}

#[cfg(not(feature = "aws-auth"))]
pub fn main() {
    panic!("Requires the `aws-auth` feature to be enabled")
}
