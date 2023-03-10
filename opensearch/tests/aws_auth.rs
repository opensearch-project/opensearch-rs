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
use common::*;
use opensearch::OpenSearch;
use regex::Regex;

use aws_config::SdkConfig;
use aws_credential_types::provider::SharedCredentialsProvider;
use aws_credential_types::Credentials;
use aws_types::region::Region;
use std::convert::TryInto;

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
        let amz_content_sha256_header = req.headers()["x-amz-content-sha256"].to_str().unwrap();
        assert_eq!(
            amz_content_sha256_header,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        ); // SHA of empty string
        http::Response::default()
    });

    let client = create_aws_client(format!("http://{}", server.addr()).as_ref())?;
    let _response = client.ping().send().await?;

    Ok(())
}

#[tokio::test]
async fn aws_auth_post() -> anyhow::Result<()> {
    let server = server::http(move |req| async move {
        let amz_content_sha256_header = req.headers()["x-amz-content-sha256"].to_str().unwrap();
        assert_eq!(
            amz_content_sha256_header,
            "f3a842f988a653a734ebe4e57c45f19293a002241a72f0b3abbff71e4f5297b9"
        ); // SHA of the JSON
        http::Response::default()
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
    let aws_creds = Credentials::new("id", "secret", None, None, "token");
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
