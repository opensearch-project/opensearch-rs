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

#![cfg(feature = "aws-auth")]

pub mod common;
use aws_credential_types::Credentials as AwsCredentials;
use aws_smithy_async::time::StaticTimeSource;
use aws_types::region::Region;
use common::{server::MockServer, tracing_init};
use opensearch::{
    aws::{AwsSigV4, AwsSigV4BuildError, AwsSigV4Builder},
    http::headers::HOST,
    indices::IndicesCreateParts,
};
use serde_json::json;
use test_case::test_case;

fn sigv4_config_builder(service_name: &str) -> AwsSigV4Builder {
    let aws_creds = AwsCredentials::new("test-access-key", "test-secret-key", None, None, "test");
    let region = Region::new("ap-southeast-2");
    let time_source = StaticTimeSource::from_secs(1673626117); // 2023-01-13 16:08:37 +0000

    AwsSigV4::builder()
        .credentials_provider(aws_creds)
        .region(region)
        .service_name(service_name.to_owned())
        .time_source(time_source)
}

fn sigv4_config(service_name: &str) -> Result<AwsSigV4, AwsSigV4BuildError> {
    sigv4_config_builder(service_name).build()
}

#[test_case("es", "10c9be415f4b9f15b12abbb16bd3e3730b2e6c76e0cf40db75d08a44ed04a3a1"; "when service name is es")]
#[test_case("aoss", "34903aef90423aa7dd60575d3d45316c6ef2d57bbe564a152b41bf8f5917abf6"; "when service name is aoss")]
#[test_case("arbitrary", "156e65c504ea2b2722a481b7515062e7692d27217b477828854e715f507e6a36"; "when service name is arbitrary")]
#[tokio::test]
async fn aws_auth_signs_correctly(
    service_name: &str,
    expected_signature: &str,
) -> anyhow::Result<()> {
    tracing_init();

    let mut server = MockServer::start().await?;

    let host = format!("aaabbbcccddd111222333.ap-southeast-2.{service_name}.amazonaws.com");
    let sigv4 = sigv4_config(service_name)?;

    let client = server.client_with(|b| b.aws_sigv4(sigv4).header(HOST, host.parse().unwrap()));

    let _ = client
        .indices()
        .create(IndicesCreateParts::Index("sample-index1"))
        .body(json!({
            "aliases": {
                "sample-alias1": {}
            },
            "mappings": {
                "properties": {
                    "age": {
                        "type": "integer"
                    }
                }
            },
            "settings": {
                "index.number_of_replicas": 1,
                "index.number_of_shards": 2
            }
        }))
        .send()
        .await?;

    let sent_req = server.received_request().await?;

    assert_eq!(sent_req.header("accept"), Some("application/json"));
    assert_eq!(sent_req.header("content-type"), Some("application/json"));
    assert_eq!(sent_req.header("host"), Some(host.as_str()));
    assert_eq!(sent_req.header("x-amz-date"), Some("20230113T160837Z"));
    assert_eq!(
        sent_req.header("x-amz-content-sha256"),
        Some("4c770eaed349122a28302ff73d34437cad600acda5a9dd373efc7da2910f8564")
    );
    assert_eq!(sent_req.header("authorization"), Some(format!("AWS4-HMAC-SHA256 Credential=test-access-key/20230113/ap-southeast-2/{service_name}/aws4_request, SignedHeaders=accept;content-type;host;x-amz-content-sha256;x-amz-date, Signature={expected_signature}").as_str()));

    Ok(())
}

#[tokio::test]
async fn aws_auth_get() -> anyhow::Result<()> {
    tracing_init();

    let mut server = MockServer::start().await?;
    let sigv4 = sigv4_config_builder("custom")
        .ignore_header("host")
        .build()?;

    let client = server.client_with(|b| b.aws_sigv4(sigv4));

    let _ = client.ping().send().await?;

    let sent_req = server.received_request().await?;

    assert_eq!(sent_req.header("authorization"), Some("AWS4-HMAC-SHA256 Credential=test-access-key/20230113/ap-southeast-2/custom/aws4_request, SignedHeaders=accept;content-type;x-amz-content-sha256;x-amz-date, Signature=8c882ad6cff05cb6c5bc91a030a92582787f34ef4af858a728c6f943c4ff2f21"));
    assert_eq!(
        sent_req.header("x-amz-content-sha256"),
        Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    );

    Ok(())
}

#[tokio::test]
async fn aws_auth_post() -> anyhow::Result<()> {
    tracing_init();

    let mut server = MockServer::start().await?;
    let sigv4 = sigv4_config("custom")?;

    let client = server.client_with(|b| b.aws_sigv4(sigv4));

    let _ = client
        .index(opensearch::IndexParts::Index("movies"))
        .body(serde_json::json!({
                "title": "Moneyball",
                "director": "Bennett Miller",
                "year": 2011
            }
        ))
        .send()
        .await?;

    let sent_req = server.received_request().await?;

    assert_eq!(
        sent_req.header("x-amz-content-sha256"),
        Some("f3a842f988a653a734ebe4e57c45f19293a002241a72f0b3abbff71e4f5297b9")
    ); // SHA of the JSON

    Ok(())
}
