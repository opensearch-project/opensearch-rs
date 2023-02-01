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

use std::time::SystemTime;

use aws_credential_types::{
    provider::{ProvideCredentials, SharedCredentialsProvider},
    Credentials,
};
use aws_sigv4::{
    http_request::{
        sign, PayloadChecksumKind, SignableBody, SignableRequest, SigningParams, SigningSettings,
    },
    signing_params::BuildError,
};
use aws_types::region::Region;
use reqwest::Request;

fn get_signing_params<'a>(
    credentials: &'a Credentials,
    service_name: &'a str,
    region: &'a Region,
) -> Result<SigningParams<'a>, BuildError> {
    let mut signing_settings = SigningSettings::default();
    signing_settings.payload_checksum_kind = PayloadChecksumKind::XAmzSha256; // required for OpenSearch Serverless

    let mut builder = SigningParams::builder()
        .access_key(credentials.access_key_id())
        .secret_key(credentials.secret_access_key())
        .service_name(service_name)
        .region(region.as_ref())
        .time(SystemTime::now())
        .settings(signing_settings);

    builder.set_security_token(credentials.session_token());

    builder.build()
}

pub async fn sign_request(
    request: &mut Request,
    credentials_provider: &SharedCredentialsProvider,
    service_name: &str,
    region: &Region,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials = credentials_provider.provide_credentials().await?;

    let params = get_signing_params(&credentials, service_name, region)?;

    let uri = request.url().as_str().parse()?;

    let signable_request = SignableRequest::new(
        request.method(),
        &uri,
        request.headers(),
        SignableBody::Bytes(request.body().and_then(|b| b.as_bytes()).unwrap_or(&[])),
    );

    let (mut instructions, _) = sign(signable_request, &params)?.into_parts();

    if let Some(new_headers) = instructions.take_headers() {
        for (name, value) in new_headers.into_iter() {
            request.headers_mut().insert(
                name.expect("AWS signing header name must never be None"),
                value,
            );
        }
    }

    Ok(())
}
