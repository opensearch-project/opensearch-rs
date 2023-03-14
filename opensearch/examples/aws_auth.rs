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

#[cfg(feature = "aws-auth")]
#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::convert::TryInto;

    use aws_config::BehaviorVersion;
    use opensearch::{
        cat::CatIndicesParts,
        http::{
            transport::{SingleNodeConnectionPool, TransportBuilder},
            Url,
        },
        OpenSearch,
    };

    let aws_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let host = ""; // e.g. https://search-mydomain.us-west-1.es.amazonaws.com
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(Url::parse(host).unwrap()))
        .aws_sigv4(aws_config.try_into()?)
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
fn main() {
    panic!("This example requires the `aws-auth` feature to be enabled")
}
