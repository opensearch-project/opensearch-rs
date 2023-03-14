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

//! Request pipeline types
//!
//! Examples:
//!
//! ```no_run
//! use futures_util::future::BoxFuture;
//! use opensearch::http::{
//!     middleware::{RequestPipeline, RequestPipelineError},
//!     reqwest::{Request, Response},
//! };
//!
//! fn logger<'a>(req: Request, next: RequestPipeline<'a>) -> BoxFuture<'a, Result<Response, RequestPipelineError>> {
//!     Box::pin(async move {
//!         println!("sending request to {}", req.url());
//!         let now = std::time::Instant::now();
//!         let res = next.run(req).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     })
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! #     use opensearch::http::{
//! #         transport::{SingleNodeConnectionPool, TransportBuilder},
//! #         Url,
//! #     };
//! #     let conn_pool = SingleNodeConnectionPool::new(Url::parse("http://localhost:9200").unwrap());
//! #     let _ = TransportBuilder::new(conn_pool).with_handler(logger);
//! # }
//! ```
//!
//! ```no_run
//! use futures_util::future::BoxFuture;
//! use opensearch::http::{
//!     middleware::{async_trait, RequestHandler, RequestPipeline, RequestPipelineError},
//!     reqwest::{Request, Response},
//! };
//!
//! struct Logger;
//!
//! #[async_trait]
//! impl RequestHandler for Logger {
//!     async fn handle(&self, request: Request, next: RequestPipeline<'_>) -> Result<Response, RequestPipelineError> {
//!         println!("sending request to {}", request.url());
//!         let now = std::time::Instant::now();
//!         let res = next.run(request).await?;
//!         println!("request completed ({:?})", now.elapsed());
//!         Ok(res)
//!     }
//! }
//!
//! # #[tokio::main]
//! # async fn main() {
//! #     use opensearch::http::{
//! #         transport::{SingleNodeConnectionPool, TransportBuilder},
//! #         Url,
//! #     };
//! #     let conn_pool = SingleNodeConnectionPool::new(Url::parse("http://localhost:9200").unwrap());
//! #     let _ = TransportBuilder::new(conn_pool).with_handler(Logger);
//! # }
//! ```

use super::{async_trait, shared_middleware};
use crate::BoxError;
use futures_util::future::BoxFuture;
use reqwest::{Client, Request, Response};
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum RequestPipelineError {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Pipeline error: {0}")]
    Pipeline(#[from] BoxError<'static>),
}

#[async_trait]
pub trait RequestHandler: Send + Sync + 'static {
    async fn handle(
        &self,
        request: Request,
        next: RequestPipeline<'_>,
    ) -> Result<Response, RequestPipelineError>;
}

#[async_trait]
impl<F> RequestHandler for F
where
    F: for<'a> Fn(
            Request,
            RequestPipeline<'a>,
        ) -> BoxFuture<'a, Result<Response, RequestPipelineError>>
        + Send
        + Sync
        + 'static,
{
    async fn handle(
        &self,
        request: Request,
        next: RequestPipeline<'_>,
    ) -> Result<Response, RequestPipelineError> {
        self(request, next).await
    }
}

shared_middleware!(SharedRequestHandler(RequestHandler));

pub struct RequestPipeline<'a> {
    pub client: &'a Client,
    pipeline: &'a [SharedRequestHandler],
}

impl<'a> RequestPipeline<'a> {
    pub(crate) fn new(client: &'a Client, pipeline: &'a [SharedRequestHandler]) -> Self {
        Self { client, pipeline }
    }

    pub fn run(
        mut self,
        request: Request,
    ) -> BoxFuture<'a, Result<Response, RequestPipelineError>> {
        if let Some((head, tail)) = self.pipeline.split_first() {
            self.pipeline = tail;
            head.handle(request, self)
        } else {
            Box::pin(async move { self.client.execute(request).await.map_err(Into::into) })
        }
    }
}
