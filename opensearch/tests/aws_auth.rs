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
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials as AwsCredentials};
use aws_smithy_async::time::StaticTimeSource;
use aws_types::region::Region;
use common::{server::MockServer, tracing_init};
use opensearch::{
    http::{headers::HOST, transport::TransportBuilder},
    indices::IndicesCreateParts,
};
use reqwest::header::HeaderValue;
use serde_json::json;
use test_case::test_case;

fn sigv4_config(transport: TransportBuilder, service_name: &str) -> TransportBuilder {
    let aws_creds = AwsCredentials::new("test-access-key", "test-secret-key", None, None, "test");
    let region = Region::new("ap-southeast-2");
    let time_source = StaticTimeSource::from_secs(1673626117); // 2023-01-13 16:08:37 +0000

    transport
        .auth(opensearch::auth::Credentials::AwsSigV4(
            SharedCredentialsProvider::new(aws_creds),
            region,
        ))
        .service_name(service_name)
        .sigv4_time_source(time_source.into())
}

const LOCALHOST: HeaderValue = HeaderValue::from_static("localhost");

#[test_case("es", "10c9be415f4b9f15b12abbb16bd3e3730b2e6c76e0cf40db75d08a44ed04a3a1"; "when service name is es")]
#[test_case("aoss", "34903aef90423aa7dd60575d3d45316c6ef2d57bbe564a152b41bf8f5917abf6"; "when service name is aoss")]
#[test_case("arbitrary", "156e65c504ea2b2722a481b7515062e7692d27217b477828854e715f507e6a36"; "when service name is arbitrary")]
#[tokio::test]
async fn aws_auth_signs_correctly(
    service_name: &str,
    expected_signature: &str,
) -> anyhow::Result<()> {
    tracing_init();

    let mut server = MockServer::start()?;

    let host = format!("aaabbbcccddd111222333.ap-southeast-2.{service_name}.amazonaws.com");

    let client =
        server.client_with(|b| sigv4_config(b, service_name).header(HOST, host.parse().unwrap()));

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

    let mut server = MockServer::start()?;

    let client = server.client_with(|b| sigv4_config(b, "custom").header(HOST, LOCALHOST));

    let _ = client.ping().send().await?;

    let sent_req = server.received_request().await?;

    assert_eq!(sent_req.header("authorization"), Some("AWS4-HMAC-SHA256 Credential=test-access-key/20230113/ap-southeast-2/custom/aws4_request, SignedHeaders=accept;content-type;host;x-amz-content-sha256;x-amz-date, Signature=e5aa6e5d9e1b86b86ed31fbb10dd62b4e93423b77830f8189701421d3e9f65bd"));
    assert_eq!(
        sent_req.header("x-amz-content-sha256"),
        Some("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855")
    ); // SHA of zero-length body

    Ok(())
}

#[tokio::test]
async fn aws_auth_post() -> anyhow::Result<()> {
    tracing_init();

    let mut server = MockServer::start()?;

    let client = server.client_with(|b| sigv4_config(b, "custom").header(HOST, LOCALHOST));

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
