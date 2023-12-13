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

use super::InitializerResult;
use crate::BoxError;
use reqwest::ClientBuilder;

pub trait ClientInitializer: 'static {
    type Result: InitializerResult<ClientBuilder>;

    fn init(self, client: ClientBuilder) -> Self::Result;
}

impl<F, R> ClientInitializer for F
where
    F: FnOnce(ClientBuilder) -> R + 'static,
    R: InitializerResult<ClientBuilder>,
{
    type Result = R;

    fn init(self, client: ClientBuilder) -> Self::Result {
        self(client)
    }
}

pub(crate) trait BoxedClientInitializer {
    fn init(self: Box<Self>, client: ClientBuilder) -> Result<ClientBuilder, BoxError<'static>>;
}

impl<T> BoxedClientInitializer for T
where
    T: ClientInitializer + Sized,
{
    fn init(self: Box<Self>, client: ClientBuilder) -> Result<ClientBuilder, BoxError<'static>> {
        ClientInitializer::init(*self, client)
            .into_result()
            .map_err(Into::into)
    }
}
