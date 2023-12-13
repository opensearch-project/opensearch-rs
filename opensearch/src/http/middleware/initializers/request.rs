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
use reqwest::RequestBuilder;

pub trait RequestInitializer: Clone + std::fmt::Debug + Send + Sync + 'static {
    type Result: InitializerResult<RequestBuilder>;

    fn init(&self, request: RequestBuilder) -> Self::Result;
}

#[derive(Clone)]
pub struct RequestInitializerFn<F>(F);

pub fn request_initializer_fn<F>(f: F) -> RequestInitializerFn<F> {
    RequestInitializerFn(f)
}

impl<F> std::fmt::Debug for RequestInitializerFn<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(RequestInitializerFn)).finish()
    }
}

impl<F, R> RequestInitializer for RequestInitializerFn<F>
where
    F: Fn(RequestBuilder) -> R + Clone + Send + Sync + 'static,
    R: InitializerResult<RequestBuilder>,
{
    type Result = R;

    fn init(&self, request: RequestBuilder) -> Self::Result {
        self.0(request)
    }
}

pub(crate) trait BoxedRequestInitializer:
    dyn_clone::DynClone + std::fmt::Debug + Send + Sync + 'static
{
    fn init(&self, request: RequestBuilder) -> Result<RequestBuilder, BoxError<'static>>;
}

impl<T> BoxedRequestInitializer for T
where
    T: RequestInitializer,
{
    fn init(&self, request: RequestBuilder) -> Result<RequestBuilder, BoxError<'static>> {
        RequestInitializer::init(self, request)
            .into_result()
            .map_err(Into::into)
    }
}

dyn_clone::clone_trait_object!(BoxedRequestInitializer);
