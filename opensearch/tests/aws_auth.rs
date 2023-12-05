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
use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials as AwsCredentials;
use aws_smithy_async::time::StaticTimeSource;
use aws_types::region::Region;
use common::*;
use opensearch::{auth::Credentials, indices::IndicesCreateParts, OpenSearch};
use regex::Regex;
use reqwest::header::HOST;
use serde_json::json;
use std::convert::TryInto;
use test_case::test_case;

#[test_case("es", "10c9be415f4b9f15b12abbb16bd3e3730b2e6c76e0cf40db75d08a44ed04a3a1"; "when service name is es")]
#[test_case("aoss", "34903aef90423aa7dd60575d3d45316c6ef2d57bbe564a152b41bf8f5917abf6"; "when service name is aoss")]
#[test_case("arbitrary", "156e65c504ea2b2722a481b7515062e7692d27217b477828854e715f507e6a36"; "when service name is arbitrary")]
#[tokio::test]
async fn aws_auth_signs_correctly(
    service_name: &str,
    expected_signature: &str,
) -> anyhow::Result<()> {
    tracing_init();

    let (server, mut rx) = server::capturing_http();

    let aws_creds = AwsCredentials::new("test-access-key", "test-secret-key", None, None, "test");
    let region = Region::new("ap-southeast-2");
    let time_source = StaticTimeSource::from_secs(1673626117); // 2023-01-13 16:08:37 +0000
    let host = format!("aaabbbcccddd111222333.ap-southeast-2.{service_name}.amazonaws.com");

    let transport_builder = client::create_builder(&format!("http://{}", server.addr()))
        .auth(Credentials::AwsSigV4(
            SharedCredentialsProvider::new(aws_creds),
            region,
        ))
        .service_name(service_name)
        .sigv4_time_source(time_source.into())
        .header(HOST, host.parse().unwrap());
    let client = client::create(transport_builder);

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

    let sent_req = rx.recv().await.expect("should have sent a request");

    assert_header_eq!(sent_req, "accept", "application/json");
    assert_header_eq!(sent_req, "content-type", "application/json");
    assert_header_eq!(sent_req, "host", host);
    assert_header_eq!(sent_req, "x-amz-date", "20230113T160837Z");
    assert_header_eq!(
        sent_req,
        "x-amz-content-sha256",
        "4c770eaed349122a28302ff73d34437cad600acda5a9dd373efc7da2910f8564"
    );
    assert_header_eq!(sent_req, "authorization", format!("AWS4-HMAC-SHA256 Credential=test-access-key/20230113/ap-southeast-2/{service_name}/aws4_request, SignedHeaders=accept;content-type;host;x-amz-content-sha256;x-amz-date, Signature={expected_signature}"));

    Ok(())
}

#[tokio::test]
async fn aws_auth_get() -> anyhow::Result<()> {
    let server = server::http(move |req| async move {
        let authorization_header = req.headers()["authorization"].to_str().unwrap();
        let re = Regex::new(r"^AWS4-HMAC-SHA256 Credential=id/\d*/us-west-1/custom/aws4_request, SignedHeaders=accept;content-type;host;x-amz-content-sha256;x-amz-date, Signature=[a-f,0-9].*$").unwrap();
        assert!(
            re.is_match(authorization_header),
            "{}",
            authorization_header
        );
        assert_header_eq!(
            req,
            "x-amz-content-sha256",
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ); // SHA of empty string
        server::empty_response()
    });

    let client = create_aws_client(format!("http://{}", server.addr()).as_ref())?;
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn aws_auth_post() -> anyhow::Result<()> {
    let server = server::http(move |req| async move {
        assert_header_eq!(
            req,
            "x-amz-content-sha256",
            "f3a842f988a653a734ebe4e57c45f19293a002241a72f0b3abbff71e4f5297b9"
        ); // SHA of the JSON
        server::empty_response()
    });

    let client = create_aws_client(format!("http://{}", server.addr()).as_ref())?;
    client
        .index(opensearch::IndexParts::Index("movies"))
        .body(serde_json::json!({
                "title": "Moneyball",
                "director": "Bennett Miller",
                "year": 2011
            }
        ))
        .send()
        .await?;

    Ok(())
}

fn create_aws_client(addr: &str) -> anyhow::Result<OpenSearch> {
    let aws_creds = AwsCredentials::new("id", "secret", None, None, "token");
    let creds_provider = SharedCredentialsProvider::new(aws_creds);
    let aws_config = SdkConfig::builder()
        .credentials_provider(creds_provider)
        .region(Region::new("us-west-1"))
        .build();
    let builder = client::create_builder(addr)
        .auth(aws_config.clone().try_into()?)
        .service_name("custom");
    Ok(client::create(builder))
}
