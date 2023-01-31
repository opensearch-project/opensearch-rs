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
    Credentials, 
    provider::{ProvideCredentials, SharedCredentialsProvider}
};
use aws_sigv4::{
    http_request::{sign, SignableBody, SignableRequest, SigningParams, SigningSettings},
    signing_params::BuildError,
};
use aws_types::region::Region;
use reqwest::Request;

const SERVICE_NAME: &str = "es";

fn get_signing_params<'a>(
    credentials: &'a Credentials,
    region: &'a Region,
) -> Result<SigningParams<'a>, BuildError> {
    let mut builder = SigningParams::builder()
        .access_key(credentials.access_key_id())
        .secret_key(credentials.secret_access_key())
        .service_name(SERVICE_NAME)
        .region(region.as_ref())
        .time(SystemTime::now())
        .settings(SigningSettings::default());

    builder.set_security_token(credentials.session_token());

    builder.build()
}

pub async fn sign_request(
    request: &mut Request,
    credentials_provider: &SharedCredentialsProvider,
    region: &Region,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let credentials = credentials_provider.provide_credentials().await?;

    let params = get_signing_params(&credentials, region)?;

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
