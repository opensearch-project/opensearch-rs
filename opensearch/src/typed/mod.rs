/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The typed client: APIs whose `send` returns the typed response model
//! ([`crate::types`]) instead of a generic [`Response`](crate::http::response::Response).
//!
//! The typed client shares the transport of the generic client it is
//! created from, and offers the APIs that have a response model so far (see
//! [`crate::types`]). Its builders take the same parts and options as the
//! generic builders, except the options that change the shape of the
//! response, which the typed builders do not offer at all:
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # use serde::Deserialize;
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! #[derive(Deserialize)]
//! struct Movie {
//!     title: String,
//! }
//!
//! let typed = client.typed();
//! let resp = typed
//!     .search(SearchParts::Index(&["movies"]))
//!     .size(10)
//!     .send() // a types::search::SearchResponse
//!     .await?;
//! for hit in &resp.hits.hits {
//!     if let Some(movie) = hit.source_as::<Movie>()? {
//!         println!("{}", movie.title);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! `filter_path`, which removes fields the model requires, does not exist
//! on the typed builders:
//!
//! ```rust,compile_fail
//! # use opensearch::{OpenSearch, SearchParts};
//! # async fn doc(client: OpenSearch) {
//! client
//!     .typed()
//!     .search(SearchParts::None)
//!     .filter_path(&["hits.hits._id"]) // no such method
//!     .send()
//!     .await;
//! # }
//! ```
//!
//! # Errors
//!
//! A status whose body is not the model (e.g. 404 `index_not_found_exception`,
//! 400 for an invalid request) is an error, whose status code and body are
//! available from [`Error::api_error`](crate::Error::api_error). A status the
//! API answers with the model is decoded, e.g. 404 of `get` for a document
//! that is not found (`found: false`):
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, GetParts, SearchParts};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let typed = client.typed();
//!
//! let doc = typed.get(GetParts::IndexId("movies", "1")).send().await?;
//! println!("found: {}", doc.found);
//!
//! match typed.search(SearchParts::Index(&["missing"])).send().await {
//!     Ok(resp) => println!("took {} ms", resp.took),
//!     Err(e) => match e.api_error() {
//!         Some(api) => println!("OpenSearch answered {}: {}", api.status(), api.body()),
//!         None => println!("request failed: {}", e),
//!     },
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # The typed and the generic client
//!
//! The generic client ([`OpenSearch`]) is unchanged: its `send` returns the
//! [`Response`](crate::http::response::Response) for any status. Use it to read the status code or the
//! headers (e.g. deprecation warnings), for options the typed builders do not
//! offer, and for the APIs without a model. Its body can still be decoded
//! into a model with `Response::json`:
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client.search(SearchParts::Index(&["movies"])).send().await?;
//! for warning in response.warning_headers() {
//!     println!("{}", warning);
//! }
//! let body: opensearch::types::search::SearchResponse = response.json().await?;
//! # Ok(())
//! # }
//! ```

use crate::{client::OpenSearch, http::transport::Transport};

mod builders;
pub use builders::*;

/// The typed client: the APIs with a typed response model, whose `send`
/// returns the model. Created with [`OpenSearch::typed`], or from a
/// [`Transport`].
#[derive(Clone, Debug, Default)]
pub struct TypedOpenSearch {
    transport: Transport,
}

impl TypedOpenSearch {
    /// Creates a typed client using the given transport
    pub fn new(transport: Transport) -> Self {
        Self { transport }
    }

    /// The transport used by this client
    pub fn transport(&self) -> &Transport {
        &self.transport
    }
}

impl OpenSearch {
    /// The typed client of this client, sharing its transport: the APIs with
    /// a typed response model, whose `send` returns the model. See
    /// [`crate::typed`].
    pub fn typed(&self) -> TypedOpenSearch {
        TypedOpenSearch::new(self.transport().clone())
    }
}

impl From<OpenSearch> for TypedOpenSearch {
    fn from(client: OpenSearch) -> Self {
        Self::new(client.transport().clone())
    }
}
