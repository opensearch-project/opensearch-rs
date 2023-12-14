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

use super::{async_trait, BoxFuture};
use crate::BoxError;
use reqwest::{Client, Request, Response};

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RequestHandlerError(pub(crate) RequestHandlerErrorKind);

impl RequestHandlerError {
    pub fn new(err: impl Into<BoxError<'static>>) -> Self {
        Self(RequestHandlerErrorKind::Handler(err.into()))
    }

    fn http(err: reqwest::Error) -> Self {
        Self(RequestHandlerErrorKind::Http(err))
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum RequestHandlerErrorKind {
    #[error("http error: {0}")]
    Http(#[source] reqwest::Error),

    #[error("handler error: {0}")]
    Handler(#[source] BoxError<'static>),
}

#[async_trait]
pub trait RequestHandler: std::fmt::Debug + Send + Sync + 'static {
    async fn handle(
        &self,
        request: Request,
        next: RequestHandlerChain<'_>,
    ) -> Result<Response, RequestHandlerError>;
}

#[derive(Clone)]
pub struct RequestHandlerFn<F>(F);

pub fn request_handler_fn<F>(f: F) -> RequestHandlerFn<F> {
    RequestHandlerFn(f)
}

impl<F> std::fmt::Debug for RequestHandlerFn<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(RequestHandlerFn)).finish()
    }
}

#[async_trait]
impl<F> RequestHandler for RequestHandlerFn<F>
where
    F: for<'a> Fn(
            Request,
            RequestHandlerChain<'a>,
        ) -> BoxFuture<'a, Result<Response, RequestHandlerError>>
        + Send
        + Sync
        + 'static,
{
    async fn handle(
        &self,
        request: Request,
        next: RequestHandlerChain<'_>,
    ) -> Result<Response, RequestHandlerError> {
        self.0(request, next).await
    }
}

#[async_trait]
impl<R> RequestHandler for std::sync::Arc<R>
where
    R: RequestHandler,
{
    async fn handle(
        &self,
        request: Request,
        next: RequestHandlerChain<'_>,
    ) -> Result<Response, RequestHandlerError> {
        self.as_ref().handle(request, next).await
    }
}

#[async_trait]
impl RequestHandler for std::sync::Arc<dyn RequestHandler> {
    async fn handle(
        &self,
        request: Request,
        next: RequestHandlerChain<'_>,
    ) -> Result<Response, RequestHandlerError> {
        self.as_ref().handle(request, next).await
    }
}

pub(crate) trait BoxedRequestHandler: RequestHandler + dyn_clone::DynClone {}

impl<T> BoxedRequestHandler for T where T: RequestHandler + Clone {}

dyn_clone::clone_trait_object!(BoxedRequestHandler);

pub struct RequestHandlerChain<'a> {
    client: &'a Client,
    chain: &'a [Box<dyn BoxedRequestHandler>],
}

impl<'a> RequestHandlerChain<'a> {
    pub(crate) fn new(client: &'a Client, chain: &'a [Box<dyn BoxedRequestHandler>]) -> Self {
        Self { client, chain }
    }

    pub fn client(&self) -> &'a Client {
        self.client
    }

    pub async fn run(mut self, request: Request) -> Result<Response, RequestHandlerError> {
        if let Some((head, tail)) = self.chain.split_first() {
            self.chain = tail;
            head.handle(request, self).await
        } else {
            self.client
                .execute(request)
                .await
                .map_err(RequestHandlerError::http)
        }
    }
}
