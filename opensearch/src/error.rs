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
    http::{transport, StatusCode},
};

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
        Self(Kind::from(error))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum Kind {
    #[error("transport builder error: {0}")]
    TransportBuilder(#[from] transport::BuildError),

    #[error("certificate error: {0}")]
    Certificate(#[from] CertificateError),

    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::error::Error),

    #[cfg(feature = "aws-auth")]
    #[error("AwsSigV4 error: {0}")]
    AwsSigV4(#[from] crate::http::aws_auth::AwsSigV4Error),
}

impl Error {
    /// The status code, if the error was generated from a response
    pub fn status_code(&self) -> Option<StatusCode> {
        match &self.0 {
            Kind::Reqwest(err) => err.status(),
            _ => None,
        }
    }

    /// Returns true if the error is related to a timeout
    pub fn is_timeout(&self) -> bool {
        match &self.0 {
            Kind::Reqwest(err) => err.is_timeout(),
            _ => false,
        }
    }

    /// Returns true if the error is related to serialization or deserialization
    pub fn is_json(&self) -> bool {
        matches!(self.0, Kind::Json(_))
    }
}
