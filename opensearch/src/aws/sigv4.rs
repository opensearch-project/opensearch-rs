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

use std::{borrow::Cow, convert::TryFrom, str::Utf8Error};

use async_trait::async_trait;
use aws_credential_types::provider::{ProvideCredentials, SharedCredentialsProvider};
use aws_sigv4::{
    http_request::{
        sign, PayloadChecksumKind, SignableBody, SignableRequest, SignatureLocation, SigningParams,
        SigningSettings,
    },
    sign::v4,
};
use aws_smithy_runtime_api::client::identity::Identity;
use aws_types::{sdk_config::SharedTimeSource, SdkConfig};
use reqwest::{
    header::{HeaderName, HeaderValue, CONTENT_LENGTH, USER_AGENT},
    Request, Response,
};
use thiserror::Error;

use crate::http::{
    middleware::{RequestHandler, RequestPipeline, RequestPipelineError},
    transport::TransportBuilder,
};

#[derive(Error, Debug)]
pub enum AwsSigV4BuildError {
    #[error("the region for signing must be provided")]
    MissingRegion,
    #[error("the credentials provider for signing must be provided")]
    MissingCredentialsProvider,
}

#[derive(Debug, Clone)]
pub struct AwsSigV4Builder {
    service_name: Option<String>,
    credentials_provider: Option<SharedCredentialsProvider>,
    region: Option<String>,
    ignored_headers: Vec<String>,
    time_source: Option<SharedTimeSource>,
}

impl AwsSigV4Builder {
    pub fn service_name(mut self, service_name: impl AsRef<str>) -> Self {
        self.service_name = Some(service_name.as_ref().to_owned());
        self
    }

    pub fn credentials_provider(
        mut self,
        credentials_provider: impl ProvideCredentials + 'static,
    ) -> Self {
        self.credentials_provider = Some(SharedCredentialsProvider::new(credentials_provider));
        self
    }

    pub fn region(mut self, region: impl AsRef<str>) -> Self {
        self.region = Some(region.as_ref().to_owned());
        self
    }

    pub fn ignore_header(mut self, header_name: impl AsRef<str>) -> Self {
        self.ignored_headers.push(header_name.as_ref().to_owned());
        self
    }

    #[doc(hidden)]
    pub fn time_source(mut self, time_source: impl Into<SharedTimeSource>) -> Self {
        self.time_source = Some(time_source.into());
        self
    }

    pub fn build(self) -> Result<AwsSigV4, AwsSigV4BuildError> {
        Ok(AwsSigV4 {
            service_name: self.service_name.unwrap_or_else(|| "es".into()),
            credentials_provider: self
                .credentials_provider
                .ok_or(AwsSigV4BuildError::MissingCredentialsProvider)?,
            region: self.region.ok_or(AwsSigV4BuildError::MissingRegion)?,
            ignored_headers: self.ignored_headers.into_iter().map(Cow::Owned).collect(),
            time_source: self.time_source.unwrap_or_default(),
        })
    }
}

impl Default for AwsSigV4Builder {
    fn default() -> Self {
        Self {
            service_name: None,
            credentials_provider: None,
            region: None,
            ignored_headers: vec![USER_AGENT.as_str().into(), CONTENT_LENGTH.as_str().into()],
            time_source: None,
        }
    }
}

impl From<&SdkConfig> for AwsSigV4Builder {
    fn from(value: &SdkConfig) -> Self {
        Self {
            credentials_provider: value.credentials_provider(),
            region: value.region().map(|r| r.to_string()),
            time_source: value.time_source(),
            ..Default::default()
        }
    }
}

impl From<SdkConfig> for AwsSigV4Builder {
    fn from(value: SdkConfig) -> Self {
        <Self as From<&SdkConfig>>::from(&value)
    }
}

#[derive(Error, Debug)]
pub enum AwsSigV4Error {
    #[error("invalid signing params: {0}")]
    InvalidSigningParams(#[from] v4::signing_params::BuildError),
    #[error("unable to retrieve credentials: {0}")]
    FailedCredentialsRetrieval(#[from] aws_credential_types::provider::error::CredentialsError),
    #[error("unable to sign request: {0}")]
    FailedSigning(#[from] aws_sigv4::http_request::SigningError),
    #[error("unable to sign a non UTF-8 header {0:?}: {1}")]
    NonUtf8Header(HeaderName, Utf8Error),
}

#[derive(Debug, Clone)]
pub struct AwsSigV4 {
    service_name: String,
    credentials_provider: SharedCredentialsProvider,
    region: String,
    ignored_headers: Vec<Cow<'static, str>>,
    time_source: SharedTimeSource,
}

impl AwsSigV4 {
    pub fn builder() -> AwsSigV4Builder {
        AwsSigV4Builder::default()
    }

    async fn sign_request(&self, request: &mut Request) -> Result<(), AwsSigV4Error> {
        let identity = self
            .credentials_provider
            .provide_credentials()
            .await?
            .into();

        let params = self.build_params(&identity)?;

        let signable_request = self.build_signable_request(request)?;

        let (new_headers, new_query_params) = {
            let (instructions, _) = sign(signable_request, &params)?.into_parts();
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

    fn build_params<'a>(
        &'a self,
        identity: &'a Identity,
    ) -> Result<SigningParams<'a>, AwsSigV4Error> {
        let mut signing_settings = SigningSettings::default();
        signing_settings.signature_location = SignatureLocation::Headers;
        signing_settings.payload_checksum_kind = PayloadChecksumKind::XAmzSha256; // required for OpenSearch Serverless
        signing_settings.excluded_headers = Some(self.ignored_headers.clone());

        let params = v4::SigningParams::builder()
            .identity(&identity)
            .name(&self.service_name)
            .region(self.region.as_ref())
            .time(self.time_source.now())
            .settings(signing_settings)
            .build()?;

        Ok(SigningParams::V4(params))
    }

    fn build_signable_request<'a>(
        &'a self,
        request: &'a Request,
    ) -> Result<SignableRequest<'a>, AwsSigV4Error> {
        let method = request.method().as_str();
        let uri = request.url().as_str();

        let mut headers = Vec::with_capacity(request.headers().len());
        for (name, value) in request.headers().iter() {
            let value = std::str::from_utf8(value.as_bytes())
                .map_err(|e| AwsSigV4Error::NonUtf8Header(name.clone(), e))?;
            headers.push((name.as_str(), value))
        }

        let body = match request.body() {
            Some(b) => match b.as_bytes() {
                Some(bytes) => SignableBody::Bytes(bytes),
                None => SignableBody::UnsignedPayload, // Body is not in memory (ie. streaming), so we can't sign it
            },
            None => SignableBody::Bytes(&[]),
        };

        SignableRequest::new(method, uri, headers.into_iter(), body).map_err(Into::into)
    }
}

impl TryFrom<&SdkConfig> for AwsSigV4 {
    type Error = AwsSigV4BuildError;

    fn try_from(value: &SdkConfig) -> Result<Self, Self::Error> {
        AwsSigV4Builder::from(value).build()
    }
}

impl TryFrom<SdkConfig> for AwsSigV4 {
    type Error = AwsSigV4BuildError;

    fn try_from(value: SdkConfig) -> Result<Self, Self::Error> {
        <Self as TryFrom<&SdkConfig>>::try_from(&value)
    }
}

#[async_trait]
impl RequestHandler for AwsSigV4 {
    async fn handle(
        &self,
        mut request: Request,
        next: RequestPipeline<'_>,
    ) -> Result<Response, RequestPipelineError> {
        self.sign_request(&mut request)
            .await
            .map_err(|e| RequestPipelineError::Pipeline(e.into()))?;
        next.run(request).await
    }
}

impl TransportBuilder {
    pub fn aws_sigv4(self, aws_sigv4: AwsSigV4) -> Self {
        self.with_handler(aws_sigv4)
    }
}
