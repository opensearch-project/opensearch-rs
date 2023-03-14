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

mod initializers;
mod request_pipeline;

pub use async_trait::async_trait;
pub use initializers::*;
pub use request_pipeline::*;

macro_rules! shared_middleware {
    ($($shared:ident($trait:ident)),*) => {
        $(
            #[derive(Clone)]
            pub struct $shared(std::sync::Arc<dyn $trait>);

            impl<M> From<M> for $shared
            where M: $trait
            {
                fn from(middleware: M) -> Self {
                    Self(std::sync::Arc::new(middleware))
                }
            }

            impl<M> From<std::sync::Arc<M>> for $shared
            where M: $trait
            {
                fn from(middleware: std::sync::Arc<M>) -> Self {
                    Self(middleware)
                }
            }

            impl From<std::sync::Arc<dyn $trait>> for $shared {
                fn from(middleware: std::sync::Arc<dyn $trait>) -> Self {
                    Self(middleware)
                }
            }

            impl std::ops::Deref for $shared {
                type Target = dyn $trait;

                fn deref(&self) -> &Self::Target {
                    self.0.as_ref()
                }
            }
        )*
    }
}

pub(self) use shared_middleware;
