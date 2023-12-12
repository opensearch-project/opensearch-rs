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

use crate::{http::headers::HeaderValue, BoxError};
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_sigv4::{
    http_request::{
        sign, PayloadChecksumKind, SignableBody, SignableRequest, SigningParams, SigningSettings,
    },
    sign::v4,
};
use aws_smithy_runtime_api::client::identity::Identity;
use aws_types::{region::Region, sdk_config::SharedTimeSource};
use reqwest::Request;

#[derive(Debug, thiserror::Error)]
pub(crate) enum AwsSigV4Error {
    #[error("SdkConfig is does not have a credentials provider configured")]
    MissingCredentialsProvider,
    #[error("SdkConfig is does not have a region configured")]
    MissingRegion,
    #[error("signing error: {0}")]
    SigningError(#[source] BoxError<'static>),
}

fn signing_error<E: Into<BoxError<'static>>>(e: E) -> AwsSigV4Error {
    AwsSigV4Error::SigningError(e.into())
}

pub(crate) async fn sign_request(
    request: &mut Request,
    credentials_provider: &SharedCredentialsProvider,
    service_name: &str,
    region: &Region,
    time_source: &SharedTimeSource,
) -> Result<(), AwsSigV4Error> {
    let identity = {
        let c = credentials_provider
            .provide_credentials()
            .await
            .map_err(signing_error)?;
        let e = c.expiry();
        Identity::new(c, e)
    };

    let signing_settings = {
        let mut s = SigningSettings::default();
        s.payload_checksum_kind = PayloadChecksumKind::XAmzSha256; // required for OpenSearch Serverless
        s
    };

    let params = {
        let p = v4::SigningParams::builder()
            .identity(&identity)
            .name(service_name)
            .region(region.as_ref())
            .time(time_source.now())
            .settings(signing_settings)
            .build()
            .map_err(signing_error)?;
        SigningParams::V4(p)
    };

    let signable_request = {
        let method = request.method().as_str();
        let uri = request.url().as_str();
        let headers = request.headers().iter().map(|(k, v)| {
            (
                k.as_str(),
                std::str::from_utf8(v.as_bytes()).expect("only utf-8 headers are signable"),
            )
        });
        let body = match request.body() {
            Some(b) => match b.as_bytes() {
                Some(bytes) => SignableBody::Bytes(bytes),
                None => SignableBody::UnsignedPayload, // Body is not in memory (ie. streaming), so we can't sign it
            },
            None => SignableBody::Bytes(&[]),
        };

        SignableRequest::new(method, uri, headers, body).map_err(signing_error)?
    };

    let (new_headers, new_query_params) = {
        let (instructions, _) = sign(signable_request, &params)
            .map_err(signing_error)?
            .into_parts();
        instructions.into_parts()
    };

    for header in new_headers.into_iter() {
        let mut value = HeaderValue::from_str(header.value())
            .expect("AWS signing header value must be a valid header");
        value.set_sensitive(header.sensitive());

        request.headers_mut().insert(header.name(), value);
    }

    for (key, value) in new_query_params.into_iter() {
        request.url_mut().query_pairs_mut().append_pair(key, &value);
    }

    Ok(())
}
