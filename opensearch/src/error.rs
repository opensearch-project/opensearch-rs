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
/* Error type based on the error type from es-rs:
 *
 * Copyright 2015-2018 Ben Ashford
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::{
    cert::CertificateError,
    http::{
        middleware::{RequestHandlerError, RequestHandlerErrorKind},
        response::Exception,
        transport, StatusCode, Url,
    },
};
use std::fmt;

pub(crate) type BoxError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;

/// An error with the client.
///
/// Errors that can occur include IO and parsing errors, as well as specific
/// errors from OpenSearch and internal errors from the client.
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct Error(Kind);

impl<E> From<E> for Error
where
    Kind: From<E>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

#[derive(Debug, thiserror::Error)]
enum Kind {
    #[error("transport builder error: {0}")]
    TransportBuilder(#[from] transport::BuildError),

    #[error("certificate error: {0}")]
    Certificate(#[from] CertificateError),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::error::Error),

    #[cfg(feature = "aws-auth")]
    #[error("AwsSigV4 error: {0}")]
    AwsSigV4(#[from] crate::http::aws_auth::AwsSigV4Error),

    #[error("request initializer error: {0}")]
    RequestInitializer(#[source] BoxError<'static>),

    #[error("request handler error: {0}")]
    RequestHandler(#[source] BoxError<'static>),

    #[error("{0}")]
    Api(Box<ApiError>),

    #[error("error decoding the response body of {url} as {type_name}: {source}")]
    Decode {
        url: Box<Url>,
        type_name: &'static str,
        #[source]
        source: serde_json::Error,
    },

    #[error("typed response unavailable: {0}")]
    TypedResponseUnavailable(&'static str),
}

impl From<RequestHandlerError> for Kind {
    fn from(err: RequestHandlerError) -> Self {
        use RequestHandlerErrorKind::*;

        match err.0 {
            Handler(err) => Self::RequestHandler(err),
            Http(err) => Self::Http(err),
        }
    }
}

use Kind::*;

impl Error {
    pub(crate) fn request_initializer(err: BoxError<'static>) -> Self {
        Self(RequestInitializer(err))
    }

    pub(crate) fn api(status: StatusCode, url: Url, body: &[u8]) -> Self {
        Self(Api(Box::new(ApiError {
            status,
            url,
            exception: serde_json::from_slice(body).ok(),
            body: String::from_utf8_lossy(body).into_owned(),
        })))
    }

    pub(crate) fn decode(url: Url, type_name: &'static str, source: serde_json::Error) -> Self {
        Self(Decode {
            url: Box::new(url),
            type_name,
            source,
        })
    }

    pub(crate) fn typed_response_unavailable(reason: &'static str) -> Self {
        Self(TypedResponseUnavailable(reason))
    }

    /// The status code, if the error was generated from a response
    pub fn status_code(&self) -> Option<StatusCode> {
        match &self.0 {
            Http(err) => err.status(),
            Api(err) => Some(err.status),
            _ => None,
        }
    }

    /// The error OpenSearch returned, when a typed call (`send_typed()`)
    /// received a status code whose body is not the typed response, e.g.
    /// 404 `index_not_found_exception`
    pub fn api_error(&self) -> Option<&ApiError> {
        match &self.0 {
            Api(err) => Some(err),
            _ => None,
        }
    }

    /// Returns true if the error is related to a timeout
    pub fn is_timeout(&self) -> bool {
        match &self.0 {
            Http(err) => err.is_timeout(),
            _ => false,
        }
    }

    /// Returns true if the error is related to serialization or deserialization
    pub fn is_json(&self) -> bool {
        matches!(self.0, Json(_) | Decode { .. })
    }
}

/// An error response from OpenSearch, returned by a typed call
/// (`send_typed()`) for a status code whose body is not the typed response.
///
/// The body is kept as text, and decoded as an [Exception] when it has the
/// usual `{"error": ..., "status": ...}` shape. Some plugins (e.g. the
/// security plugin) answer errors in their own shape, for which
/// [ApiError::exception] is `None` and only [ApiError::body] is available.
#[derive(Debug, Clone)]
pub struct ApiError {
    status: StatusCode,
    url: Url,
    exception: Option<Exception>,
    body: String,
}

impl ApiError {
    /// The HTTP status code of the response
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// The URL of the request
    pub fn url(&self) -> &Url {
        &self.url
    }

    /// The error body decoded as an [Exception], if it has that shape
    pub fn exception(&self) -> Option<&Exception> {
        self.exception.as_ref()
    }

    /// The raw response body
    pub fn body(&self) -> &str {
        &self.body
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "OpenSearch returned {} for {}", self.status, self.url)?;
        let error = self.exception.as_ref().map(Exception::error);
        match (error.and_then(|e| e.ty()), error.and_then(|e| e.reason())) {
            (Some(ty), Some(reason)) => write!(f, ": {}: {}", ty, reason),
            (Some(ty), None) => write!(f, ": {}", ty),
            _ if !self.body.is_empty() => {
                const MAX: usize = 200;
                match self.body.char_indices().nth(MAX) {
                    Some((end, _)) => write!(f, ": {}...", &self.body[..end]),
                    None => write!(f, ": {}", self.body),
                }
            }
            _ => Ok(()),
        }
    }
}
