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

mod client;
mod request;

use crate::BoxError;
use std::convert::Infallible;

pub use client::*;
pub use request::*;

pub trait InitializerResult<T> {
    type Error: Into<BoxError<'static>>;

    fn into_result(self) -> Result<T, Self::Error>;
}

impl<T, E> InitializerResult<T> for Result<T, E>
where
    E: Into<BoxError<'static>>,
{
    type Error = E;

    fn into_result(self) -> Result<T, Self::Error> {
        self
    }
}

impl<T> InitializerResult<T> for T {
    type Error = Infallible;

    fn into_result(self) -> Result<T, Infallible> {
        Ok(self)
    }
}
