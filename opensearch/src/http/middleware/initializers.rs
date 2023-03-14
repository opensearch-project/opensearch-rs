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

use super::shared_middleware;
use crate::BoxError;
use reqwest::{ClientBuilder, RequestBuilder};

pub trait ClientInitializer: Send + Sync + 'static {
    fn init(&self, client: ClientBuilder) -> Result<ClientBuilder, BoxError<'static>>;
}

impl<F> ClientInitializer for F
where
    F: Fn(ClientBuilder) -> Result<ClientBuilder, BoxError<'static>> + Send + Sync + 'static,
{
    fn init(&self, client: ClientBuilder) -> Result<ClientBuilder, BoxError<'static>> {
        self(client)
    }
}

pub trait RequestInitializer: Send + Sync + 'static {
    fn init(&self, request: RequestBuilder) -> Result<RequestBuilder, BoxError<'static>>;
}

impl<F> RequestInitializer for F
where
    F: Fn(RequestBuilder) -> Result<RequestBuilder, BoxError<'static>> + Send + Sync + 'static,
{
    fn init(&self, request: RequestBuilder) -> Result<RequestBuilder, BoxError<'static>> {
        self(request)
    }
}

shared_middleware!(
    SharedClientInitializer(ClientInitializer),
    SharedRequestInitializer(RequestInitializer)
);
