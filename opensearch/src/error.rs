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

use crate::http::{middleware::RequestPipelineError, transport::BuildError, StatusCode};

pub type BoxError<'a> = Box<dyn std::error::Error + Send + Sync + 'a>;

/// An error with the client.
///
/// Errors that can occur include IO and parsing errors, as well as specific
/// errors from OpenSearch and internal errors from the client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An error building the client
    #[error("Error building the client: {0}")]
    Build(#[from] BuildError),

    /// A general error from this library
    #[error("Library error: {0}")]
    Lib(String),

    /// Reqwest error
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// URL parse error
    #[error("URL parse error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::error::Error),

    /// Request initializer error
    #[error("Request initializer error: {0}")]
    RequestInitializer(#[source] BoxError<'static>),

    /// Request pipeline error
    #[error("Request pipeline error: {0}")]
    RequestPipeline(#[source] BoxError<'static>),
}

impl From<RequestPipelineError> for Error {
    fn from(err: RequestPipelineError) -> Self {
        match err {
            RequestPipelineError::Reqwest(err) => Self::Reqwest(err),
            RequestPipelineError::Pipeline(err) => Self::RequestPipeline(err),
        }
    }
}

pub(crate) fn lib(err: impl Into<String>) -> Error {
    Error::Lib(err.into())
}

impl Error {
    /// The status code, if the error was generated from a response
    pub fn status_code(&self) -> Option<StatusCode> {
        match &self {
            Self::Reqwest(err) => err.status(),
            _ => None,
        }
    }

    /// Returns true if the error is related to a timeout
    pub fn is_timeout(&self) -> bool {
        match &self {
            Self::Reqwest(err) => err.is_timeout(),
            _ => false,
        }
    }

    /// Returns true if the error is related to serialization or deserialization
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json(_))
    }
}
