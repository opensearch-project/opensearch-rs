/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The fields of the generated response models declared as raw JSON, with
//! the reason each cannot, or should not, be typed. See the parent module.
//!
//! Generation fails when this table and the generated models disagree, and
//! prints the declarations to add. Keep the entries grouped by namespace and
//! sorted by key.

#[allow(unused_imports)]
use super::UNREVIEWED;
use super::{raw, RawField};
use crate::openapi::responses::RawCause::*;

pub const RAW_FIELDS: &[RawField] = &[
    // --- _common ---
    // bare
    raw(
        "_common:ErrorCause/[additional]",
        NoType,
        "error-specific extra properties: each exception type adds its own keys",
    ),
    // map<bare>
    raw(
        "_common:InlineGetDictUserDefined/[additional]",
        OpenObject,
        "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata",
    ),
    // map<bare>
    raw(
        "_common:InlineGetDictUserDefined/_source",
        OpenObject,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<map<bare>>
    raw(
        "_common:InlineGetDictUserDefined/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
    // bare
    raw(
        "_common:Metadata/[additional]",
        OpenObject,
        "user-defined `_meta`: arbitrary JSON by design",
    ),

    // --- _common.aggregations ---
    // bare
    raw(
        "_common.aggregations:ScriptedMetricAggregate/value",
        NoType,
        "scripted_metric: the reduce script decides the result type",
    ),

    // --- _core.get ---
    // bare
    raw(
        "_core.get:GetResponse/_source",
        GenericParameter,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.get:GetResponse/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),

    // --- _core.search ---
    // bare
    raw(
        "_core.search:HitsMetadataHitsItem/[additional]",
        NoType,
        "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata",
    ),
    // bare
    raw(
        "_core.search:HitsMetadataHitsItem/_source",
        GenericParameter,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.search:HitsMetadataHitsItem/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
    // bare
    raw(
        "_core.search:HitsMetadataJsonValueHitsItem/[additional]",
        NoType,
        "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata",
    ),
    // bare
    raw(
        "_core.search:HitsMetadataJsonValueHitsItem/_source",
        NoType,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.search:HitsMetadataJsonValueHitsItem/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
    // bare
    raw(
        "_core.search:ProcessorExecutionDetail/input_data",
        NoType,
        "search pipeline processor input/output: request, response or document fragments depending on the processor",
    ),
    // bare
    raw(
        "_core.search:ProcessorExecutionDetail/output_data",
        NoType,
        "search pipeline processor input/output: request, response or document fragments depending on the processor",
    ),
    // bare
    raw(
        "_core.search:SearchResponseHitsHitsItem/[additional]",
        NoType,
        "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata",
    ),
    // bare
    raw(
        "_core.search:SearchResponseHitsHitsItem/_source",
        GenericParameter,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.search:SearchResponseHitsHitsItem/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
    // bare
    raw(
        "_core.search:SuggestCompletionOptionsArrayItem/_source",
        GenericParameter,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.search:SuggestCompletionOptionsArrayItem/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
    // bare
    raw(
        "_core.search:SuggestCompletionOptionsObject/_source",
        GenericParameter,
        "document body: its shape is user-defined (decode with serde_json or a caller type)",
    ),
    // map<bare>
    raw(
        "_core.search:SuggestCompletionOptionsObject/fields",
        OpenObject,
        "stored/docvalue fields requested per query: names and value types depend on the mapping",
    ),
];
