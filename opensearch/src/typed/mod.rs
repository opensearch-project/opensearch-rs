/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The typed client: APIs whose `send` returns the response type
//! ([`crate::types`]) instead of a generic [`Response`](crate::http::response::Response).
//!
//! The typed client shares the transport of the generic client it is
//! created from, and offers every API whose specification declares a JSON
//! response, through the same namespaces (`client.typed().cluster().health(..)`). Its builders take the same parts and options as the
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
//! `filter_path`, which removes fields the response type requires, does not exist
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
//! A status whose body is not the response type (e.g. 404 `index_not_found_exception`,
//! 400 for an invalid request) is an error, whose status code and body are
//! available from [`Error::api_error`](crate::Error::api_error). A status the
//! API answers with the response type is decoded, e.g. 404 of `get` for a document
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
//! # Fields unknown to the response types
//!
//! Keys of the response that the response type does not define (fields added by
//! plugins or newer OpenSearch versions, or missing from the specification)
//! are ignored by default, as by the other OpenSearch clients. A typed client
//! set with [`ignore_unknown_fields(false)`](TypedOpenSearch::ignore_unknown_fields)
//! reports them instead, as an error listing their paths, available from
//! [`Error::unknown_fields`](crate::Error::unknown_fields):
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # async fn doc() {
//! # let client = OpenSearch::default();
//! let strict = client.typed().ignore_unknown_fields(false);
//! match strict.search(SearchParts::Index(&["movies"])).send().await {
//!     Ok(resp) => println!("took {} ms", resp.took),
//!     Err(e) => match e.unknown_fields() {
//!         // e.g. [".ext"] with a search pipeline adding `ext`
//!         Some(fields) => println!("not in the response type: {:?}", fields),
//!         None => println!("request failed: {}", e),
//!     },
//! }
//! # }
//! ```
//!
//! Values kept as raw JSON by the response types (documents, aggregations, maps the
//! specification leaves open) are not checked.
//!
//! # The typed and the generic client
//!
//! The generic client ([`OpenSearch`]) is unchanged: its `send` returns the
//! [`Response`](crate::http::response::Response) for any status. Use it to read the status code or the
//! headers (e.g. deprecation warnings), for options the typed builders do not
//! offer, and for the APIs without a response type. Its body can still be decoded
//! into a response type with `Response::json`:
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

/// The typed client: the APIs with a response type, whose `send`
/// returns the response type. Created with [`OpenSearch::typed`], or from a
/// [`Transport`].
#[derive(Clone, Debug)]
pub struct TypedOpenSearch {
    transport: Transport,
    ignore_unknown_fields: bool,
}

impl Default for TypedOpenSearch {
    fn default() -> Self {
        Self::new(Transport::default())
    }
}

impl TypedOpenSearch {
    /// Creates a typed client using the given transport
    pub fn new(transport: Transport) -> Self {
        Self {
            transport,
            ignore_unknown_fields: true,
        }
    }

    /// The transport used by this client
    pub fn transport(&self) -> &Transport {
        &self.transport
    }

    /// Whether keys of a response that its response type does not define are
    /// ignored (`true`, the default), or make the call fail with an error
    /// listing them (`false`). See [Fields unknown to the response types](crate::typed#fields-unknown-to-the-response-types).
    pub fn ignore_unknown_fields(mut self, ignore: bool) -> Self {
        self.ignore_unknown_fields = ignore;
        self
    }

    /// Whether the typed builders created by this client report unknown fields
    #[doc(hidden)]
    pub fn reports_unknown_fields(&self) -> bool {
        !self.ignore_unknown_fields
    }
}

/// The paths of the keys of `body` that `value` does not keep: `value` is
/// serialized back and compared with the body, so that a key is reported
/// wherever the response type drops it, including inside unions. A key whose value
/// is `null` is not reported, as optional fields are skipped when absent.
pub(crate) fn unknown_fields<T: serde::Serialize>(
    body: &[u8],
    value: &T,
) -> Result<Vec<String>, serde_json::Error> {
    use serde_json::Value;

    fn walk(original: &Value, back: &Value, path: &mut String, out: &mut Vec<String>) {
        match (original, back) {
            (Value::Object(a), Value::Object(b)) => {
                for (key, value) in a {
                    let len = path.len();
                    path.push('.');
                    path.push_str(key);
                    match b.get(key) {
                        Some(kept) => walk(value, kept, path, out),
                        None if !value.is_null() => out.push(path.clone()),
                        None => {}
                    }
                    path.truncate(len);
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                for (i, (value, kept)) in a.iter().zip(b).enumerate() {
                    let len = path.len();
                    path.push_str(&format!("[{}]", i));
                    walk(value, kept, path, out);
                    path.truncate(len);
                }
            }
            _ => {}
        }
    }

    let original: Value = serde_json::from_slice(body)?;
    let back = serde_json::to_value(value)?;
    let mut out = Vec::new();
    walk(&original, &back, &mut String::new(), &mut out);
    Ok(out)
}

impl OpenSearch {
    /// The typed client of this client, sharing its transport: the APIs with
    /// a response type, whose `send` returns the response type. Keys of the
    /// responses unknown to the response types are ignored; see
    /// [`TypedOpenSearch::ignore_unknown_fields`] and [`crate::typed`].
    pub fn typed(&self) -> TypedOpenSearch {
        TypedOpenSearch::new(self.transport().clone())
    }
}

impl From<OpenSearch> for TypedOpenSearch {
    fn from(client: OpenSearch) -> Self {
        Self::new(client.transport().clone())
    }
}
