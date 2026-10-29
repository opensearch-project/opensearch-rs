/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Runtime support for lazily decoded unions.
//!
//! Some response values cannot be decoded into a concrete type at
//! deserialization time, either because the JSON carries no discriminator
//! and the branch shapes overlap, or because trying every branch would be
//! prohibitively expensive (aggregations have on the order of a hundred
//! variants). Such values are captured as raw JSON and only parsed into a
//! concrete branch when the caller — who knows which branch to expect,
//! having written the request — asks for it.

use serde::Deserialize;
use serde_json::value::RawValue;
use std::fmt;

/// The error returned when a lazily decoded union value cannot be parsed
/// as the requested branch.
///
/// This usually means a different branch accessor should be used: e.g.
/// calling `as_avg()` on an aggregation that was requested as `terms`.
/// The raw JSON remains accessible through the union's `raw()` method.
#[derive(Debug)]
pub struct UnionBranchError {
    /// The name of the branch that was requested
    attempted: &'static str,
    /// The underlying deserialization error
    source: serde_json::Error,
}

impl UnionBranchError {
    pub(crate) fn new(attempted: &'static str, source: serde_json::Error) -> Self {
        Self { attempted, source }
    }

    /// The name of the branch that could not be parsed
    pub fn attempted(&self) -> &'static str {
        self.attempted
    }
}

impl fmt::Display for UnionBranchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "value could not be parsed as union branch \"{}\": {}",
            self.attempted, self.source
        )
    }
}

impl std::error::Error for UnionBranchError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Parses raw JSON as the given union branch, mapping failures to
/// [UnionBranchError]. Used by generated branch accessors.
pub(crate) fn parse_branch<'de, T>(
    raw: &'de RawValue,
    branch: &'static str,
) -> Result<T, UnionBranchError>
where
    T: Deserialize<'de>,
{
    serde_json::from_str(raw.get()).map_err(|e| UnionBranchError::new(branch, e))
}

/// The JSON kind a value starting with the given byte has, in the wording
/// of serde's "invalid type" errors. Used by generated dispatch unions,
/// which route on the first byte of the raw JSON text.
#[doc(hidden)]
pub fn json_kind_name(first: Option<u8>) -> &'static str {
    match first {
        Some(b'{') => "map",
        Some(b'[') => "sequence",
        Some(b'"') => "string",
        Some(b't') | Some(b'f') => "boolean",
        Some(b'n') => "null",
        Some(b'-') | Some(b'0'..=b'9') => "number",
        _ => "empty value",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Deserialize)]
    struct Terms {
        buckets: Vec<serde_json::Value>,
    }

    #[test]
    fn parse_branch_succeeds_for_matching_shape() {
        let raw: Box<RawValue> = serde_json::from_str(r#"{"buckets": []}"#).unwrap();
        let terms: Terms = parse_branch(&raw, "terms").unwrap();
        assert!(terms.buckets.is_empty());
    }

    #[test]
    fn json_kind_names_follow_the_first_byte() {
        assert_eq!(json_kind_name(Some(b'{')), "map");
        assert_eq!(json_kind_name(Some(b'[')), "sequence");
        assert_eq!(json_kind_name(Some(b'"')), "string");
        assert_eq!(json_kind_name(Some(b't')), "boolean");
        assert_eq!(json_kind_name(Some(b'n')), "null");
        assert_eq!(json_kind_name(Some(b'-')), "number");
        assert_eq!(json_kind_name(Some(b'7')), "number");
        assert_eq!(json_kind_name(None), "empty value");
    }

    #[test]
    fn parse_branch_reports_attempted_branch() {
        let raw: Box<RawValue> = serde_json::from_str(r#"{"value": 42.0}"#).unwrap();
        let err = parse_branch::<Terms>(&raw, "terms").unwrap_err();
        assert_eq!(err.attempted(), "terms");
        assert!(err.to_string().contains("terms"));
    }
}
