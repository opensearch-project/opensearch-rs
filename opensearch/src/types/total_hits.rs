/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Reading the total number of hits of a search (`hits.total`).

use crate::types::search::{TotalHits, TotalHitsRelation};

/// The total hit count of a search: `hits.total` of a response.
///
/// The server reports it in one of two forms: an object
/// (`{"value": 3, "relation": "eq"}`), or a bare integer when the request
/// sets `rest_total_hits_as_int=true`. The specification declares the field
/// inline in each kind of hits object, so each has its own enum for it
/// (e.g. [`SearchResponseHitsTotal`](crate::types::search::SearchResponseHitsTotal)
/// for a search, [`HitsMetadataTotal`](crate::types::search::HitsMetadataTotal)
/// for inner hits). All of them, and the object form [`TotalHits`],
/// implement this trait, so one function can read any of them:
///
/// ```
/// use opensearch::types::TotalHitCount;
///
/// fn describe(total: &impl TotalHitCount) -> String {
///     if total.is_accurate() {
///         format!("{} hits", total.value())
///     } else {
///         format!("at least {} hits", total.value())
///     }
/// }
/// ```
///
/// Each enum also has the same methods as inherent methods, callable
/// without importing this trait, and the hits objects have
/// `total_value()` for the common case of reading the number alone.
pub trait TotalHitCount {
    /// The number of hits: exact when [`is_accurate`](Self::is_accurate)
    /// is true, and otherwise a lower bound. By default the count is only
    /// tracked accurately up to 10,000 hits (`track_total_hits`).
    fn value(&self) -> i64;

    /// How [`value`](Self::value) relates to the actual number of hits:
    /// [`Eq`](TotalHitsRelation::Eq) when it is the exact number,
    /// [`Gte`](TotalHitsRelation::Gte) when it is a lower bound. The integer
    /// form is always `Eq`: the server refuses `rest_total_hits_as_int`
    /// when the count is not tracked accurately.
    fn relation(&self) -> &TotalHitsRelation;

    /// Whether [`value`](Self::value) is the exact number of hits
    /// (relation `eq`). False for a lower bound, and for a relation this
    /// client does not know, which cannot be taken as exact.
    fn is_accurate(&self) -> bool {
        matches!(self.relation(), TotalHitsRelation::Eq)
    }
}

/// The relation of the integer form of the count, which is always exact
pub(crate) static ACCURATE: TotalHitsRelation = TotalHitsRelation::Eq;

impl TotalHitCount for TotalHits {
    fn value(&self) -> i64 {
        self.value
    }

    fn relation(&self) -> &TotalHitsRelation {
        &self.relation
    }
}
