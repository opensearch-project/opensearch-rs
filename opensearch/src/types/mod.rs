/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Typed response models generated from the
//! [OpenSearch API specification](https://github.com/opensearch-project/opensearch-api-specification),
//! together with the hand-written runtime support they rely on.
//!
//! # Getting the typed response of an API
//!
//! Models are generated for the search, index, bulk and get APIs so far
//! (the selection is `TYPED_OPERATION_GROUPS` in the API generator); the
//! generator supports the whole specification, and more APIs are added by
//! extending that list. They are obtained through the typed client
//! ([`crate::typed`]), whose `send` returns the model of the API, so neither
//! the type nor its path needs to be written out:
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
//! // a `types::search::SearchResponse`
//! let resp = client
//!     .typed()
//!     .search(SearchParts::Index(&["movies"]))
//!     .send()
//!     .await?;
//! // `None` when the request disabled counting; see `TotalHitCount` for
//! // telling an exact count from a lower bound
//! println!("{:?} hits", resp.hits.total_value());
//! for hit in &resp.hits.hits {
//!     if let Some(movie) = hit.source_as::<Movie>()? {
//!         println!("{}", movie.title);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! See [`crate::typed`] for errors, and for when to use the generic client
//! instead. With the generic client, a body can still be decoded into a
//! model with [`Response::json`](crate::http::response::Response::json).
//!
//! # Design notes
//!
//! 100% typing is a non-goal: fields whose shape is decided at runtime
//! (documents, aggregation values, ...) intentionally hold raw JSON and
//! expose it through explicit escape hatches. Additions of such untyped
//! fields are reviewed: each is declared with its reason in the API
//! generator, and generation fails on an undeclared one.
//!
//! Nothing the server sends is dropped. Every struct collects the keys
//! the specification does not define for it (fields added by plugins or
//! newer OpenSearch versions, or missing from the specification) into a
//! `unknown_fields` map of raw JSON, and serializes them back unchanged,
//! so a response round-trips even when the specification lags behind the
//! server. Structs whose specification declares `additionalProperties`
//! keep that typed map (`additional`, or `aggregations` in buckets)
//! instead. The maps are filled by generated `Deserialize` impls rather
//! than `#[serde(flatten)]`, which cannot decode the `Box<RawValue>`
//! fields these structs contain.
//!
//! # Filtered responses (`filter_path`)
//!
//! The models describe the full response of an API. The `filter_path`
//! parameter, accepted by almost every API, removes from the response
//! everything it does not match, including fields the models require:
//! decoding a filtered search response as
//! [`SearchResponse`](search::SearchResponse) fails with an error such as
//! ``missing field `_shards` ``. This is expected, and the models are not
//! loosened for it, as making every field optional would burden the
//! unfiltered use. The typed client ([`crate::typed`]) therefore does not
//! offer `filter_path`.
//!
//! Send a filtered request with the generic client and decode the response
//! into a type of your own that mirrors what the filter keeps, or into a
//! [`serde_json::Value`]:
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # use serde::Deserialize;
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! // only what `hits.hits._source.title` keeps
//! #[derive(Deserialize)]
//! struct Titles {
//!     hits: Hits,
//! }
//! #[derive(Deserialize)]
//! struct Hits {
//!     hits: Vec<Hit>,
//! }
//! #[derive(Deserialize)]
//! struct Hit {
//!     _source: Movie,
//! }
//! #[derive(Deserialize)]
//! struct Movie {
//!     title: String,
//! }
//!
//! let titles: Titles = client
//!     .search(SearchParts::Index(&["movies"]))
//!     .filter_path(&["hits.hits._source.title"])
//!     .send()
//!     .await?
//!     .json()
//!     .await?;
//! for hit in titles.hits.hits {
//!     println!("{}", hit._source.title);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! A filter can also remove a whole section: with
//! `filter_path=hits.hits._id` a search with no match returns `{}`, so
//! make the fields of such a type optional where the filter may leave
//! nothing.

mod total_hits;
pub mod union;

pub use total_hits::TotalHitCount;
pub use union::UnionBranchError;

// GENERATED-BEGIN:response-modules
// Generated code - do not edit until the next GENERATED-END marker

pub mod bulk;
pub mod common;
pub mod common_aggregations;
pub mod explain;
pub mod get;
pub mod index;
pub mod search;

// GENERATED-END
