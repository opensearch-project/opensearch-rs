/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

// -----------------------------------------------
// This file is generated, Please do not edit it manually.
// Run the following in the root of the repo to regenerate:
//
// cargo make generate-api
// -----------------------------------------------

//! Response types of the `search` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregationBreakdown {
    pub build_aggregation: i64,
    pub build_aggregation_count: i64,
    pub build_leaf_collector: i64,
    pub build_leaf_collector_count: i64,
    pub collect: i64,
    pub collect_count: i64,
    pub initialize: i64,
    pub initialize_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_collection: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_collection_count: Option<i64>,
    pub reduce: i64,
    pub reduce_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregationProfile {
    pub breakdown: crate::types::search::AggregationBreakdown,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::search::AggregationProfile>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<crate::types::search::AggregationProfileDebug>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregationProfileDebug {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub built_buckets: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chars_fetched: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collect_analyzed_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collect_analyzed_ns: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_strategy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deferred_aggregators: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delegate_debug: Option<crate::types::search::AggregationProfileDelegateDebug>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_collectors_used: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extract_ns: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_filter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub map_reducer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_collectors_used: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinals_collectors_overhead_too_high: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ordinals_collectors_used: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_strategy: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_with_multi_valued_ords: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_with_single_valued_ords: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_hashing_collectors_used: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surviving_buckets: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_buckets: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub values_fetched: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregationProfileDelegateDebug {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<crate::types::search::AggregationProfileDelegateDebugFilter>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_collected: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_counted: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_with_deleted_docs: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_with_doc_count_field: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregationProfileDelegateDebugFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub results_from_metadata: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_counted_in_constant_time: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub specialized_for: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BoundaryScanner {
    #[doc = "`chars`"]
    Chars,
    #[doc = "`sentence`"]
    Sentence,
    #[doc = "`word`"]
    Word,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BoundaryScanner {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BoundaryScanner::Chars => "chars",
            BoundaryScanner::Sentence => "sentence",
            BoundaryScanner::Word => "word",
            BoundaryScanner::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BoundaryScanner {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BoundaryScanner {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "chars" => BoundaryScanner::Chars,
            "sentence" => BoundaryScanner::Sentence,
            "word" => BoundaryScanner::Word,
            _ => BoundaryScanner::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinHighlighterType {
    #[doc = "`plain`"]
    Plain,
    #[doc = "`fvh`"]
    Fvh,
    #[doc = "`unified`"]
    Unified,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BuiltinHighlighterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinHighlighterType::Plain => "plain",
            BuiltinHighlighterType::Fvh => "fvh",
            BuiltinHighlighterType::Unified => "unified",
            BuiltinHighlighterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BuiltinHighlighterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BuiltinHighlighterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "plain" => BuiltinHighlighterType::Plain,
            "fvh" => BuiltinHighlighterType::Fvh,
            "unified" => BuiltinHighlighterType::Unified,
            _ => BuiltinHighlighterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Collector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::search::Collector>>,
    pub name: String,
    pub reason: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CompletionContext {
    raw: Box<serde_json::value::RawValue>,
}
impl CompletionContext {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `context` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_context(
        &self,
    ) -> std::result::Result<crate::types::search::Context, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "context")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::search::CompletionContextObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionContextObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    pub context: crate::types::search::Context,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neighbours: Option<Vec<crate::types::common::GeoHashPrecision>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<crate::types::common::GeoHashPrecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionSuggester {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts:
        Option<std::collections::BTreeMap<String, Vec<crate::types::search::CompletionContext>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuzzy: Option<crate::types::search::SuggestFuzziness>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_duplicates: Option<bool>,
}

#[doc = "Text or location that we want similar documents for or a lookup to a document's field for the text."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Context {
    raw: Box<serde_json::value::RawValue>,
}
impl Context {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `category` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_category(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "category")
    }
    #[doc = "Parses the value as the `location` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_location(
        &self,
    ) -> std::result::Result<crate::types::common::GeoLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "location")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DirectGenerator {
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_edits: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_inspections: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_term_freq: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_freq: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_filter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_mode: Option<crate::types::common::SuggestMode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchProfile {
    pub breakdown: crate::types::search::FetchProfileBreakdown,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::search::FetchProfile>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<crate::types::search::FetchProfileDebug>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchProfileBreakdown {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_source: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_source_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_stored_fields: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_stored_fields_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_reader: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_reader_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_count: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FetchProfileDebug {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fast_path: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldCollapse {
    pub field: crate::types::common::Field,
    #[doc = "The number of inner hits and their sort order."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inner_hits: Option<Box<crate::types::search::FieldCollapseInnerHits>>,
    #[doc = "The number of concurrent requests that are allowed to be retrieved by the `inner_hits` parameter per group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_group_searches: Option<i64>,
}

#[doc = "The number of inner hits and their sort order."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FieldCollapseInnerHits {
    InnerHits(Box<crate::types::search::InnerHits>),
    Array(Vec<crate::types::search::InnerHits>),
}
impl<'de> serde::Deserialize<'de> for FieldCollapseInnerHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(FieldCollapseInnerHits::InnerHits),
            Some(b'[') => serde_json::from_str(text).map(FieldCollapseInnerHits::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "FieldCollapseInnerHits"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldSuggester {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::search::CompletionSuggester>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase: Option<crate::types::search::PhraseSuggester>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term: Option<crate::types::search::TermSuggester>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Highlight {
    #[doc = "A string that contains each boundary character."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_chars: Option<String>,
    #[doc = "How far to scan for boundary characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_max_scan: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_scanner: Option<crate::types::search::BoundaryScanner>,
    #[doc = "Controls which locale is used to search for sentence and word boundaries. This parameter takes the form of a language tag, for example, `\"en-US\"`, `\"fr-FR\"`, or `\"ja-JP\"`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_scanner_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_source: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_offset: Option<i32>,
    #[doc = "The size of the highlighted fragment in characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragmenter: Option<crate::types::search::HighlighterFragmenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight_filter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight_query: Option<Box<crate::types::common_query_dsl::QueryContainer>>,
    #[doc = "If set to a non-negative value, highlighting stops at this defined maximum limit. The rest of the text is not processed or highlighted, and no error is returned. The `max_analyzer_offset` query setting does not override the `index.highlight.max_analyzed_offset` setting, which takes precedence when it is set to a lower value than the query setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_analyzer_offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_fragment_length: Option<i32>,
    #[doc = "The amount of text you want to return from the beginning of the field if there are no matching fragments to highlight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_match_size: Option<i32>,
    #[doc = "The maximum number of fragments to return. When the number of fragments is set to `0`, no fragments are returned. Instead, the entirety of a field's contents are highlighted and returned. This is useful when you need to highlight short texts, such as a title or address, in which fragmentation is not required. If `number_of_fragments` is set to `0`, the `fragment_size` is ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_fragments: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::search::HighlighterOrder>,
    #[doc = "Controls the number of matching phrases in a document that are considered. This prevents the `fvh` highlighter from analyzing too many phrases and consuming too much memory. When using `matched_fields`, phrase-limited phrases per matched field are considered. Raising the limit increases the query time and consumes more memory. This setting is only supported by the `fvh` highlighter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase_limit: Option<i32>,
    #[doc = "When used in conjunction with `pre_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_tags: Option<Vec<String>>,
    #[doc = "When used in conjunction with `post_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_tags: Option<Vec<String>>,
    #[doc = "By default, only fields that contains a query match are highlighted. Set to `false` to highlight all fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_schema: Option<crate::types::search::HighlighterTagsSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::search::HighlighterType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoder: Option<crate::types::search::HighlighterEncoder>,
    pub fields: crate::types::search::HighlightFields,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HighlightField {
    #[doc = "A string that contains each boundary character."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_chars: Option<String>,
    #[doc = "How far to scan for boundary characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_max_scan: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_scanner: Option<crate::types::search::BoundaryScanner>,
    #[doc = "Controls which locale is used to search for sentence and word boundaries. This parameter takes the form of a language tag, for example, `\"en-US\"`, `\"fr-FR\"`, or `\"ja-JP\"`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boundary_scanner_locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_source: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_offset: Option<i32>,
    #[doc = "The size of the highlighted fragment in characters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragment_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fragmenter: Option<crate::types::search::HighlighterFragmenter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight_filter: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight_query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "If set to a non-negative value, highlighting stops at this defined maximum limit. The rest of the text is not processed or highlighted, and no error is returned. The `max_analyzer_offset` query setting does not override the `index.highlight.max_analyzed_offset` setting, which takes precedence when it is set to a lower value than the query setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_analyzer_offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_fragment_length: Option<i32>,
    #[doc = "The amount of text you want to return from the beginning of the field if there are no matching fragments to highlight."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_match_size: Option<i32>,
    #[doc = "The maximum number of fragments to return. When the number of fragments is set to `0`, no fragments are returned. Instead, the entirety of a field's contents are highlighted and returned. This is useful when you need to highlight short texts, such as a title or address, in which fragmentation is not required. If `number_of_fragments` is set to `0`, the `fragment_size` is ignored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_fragments: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::search::HighlighterOrder>,
    #[doc = "Controls the number of matching phrases in a document that are considered. This prevents the `fvh` highlighter from analyzing too many phrases and consuming too much memory. When using `matched_fields`, phrase-limited phrases per matched field are considered. Raising the limit increases the query time and consumes more memory. This setting is only supported by the `fvh` highlighter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phrase_limit: Option<i32>,
    #[doc = "When used in conjunction with `pre_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_tags: Option<Vec<String>>,
    #[doc = "When used in conjunction with `post_tags`, defines the HTML tags to use for the highlighted text. By default, highlighted text is wrapped in `<em>` and `</em>` tags."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pre_tags: Option<Vec<String>>,
    #[doc = "By default, only fields that contains a query match are highlighted. Set to `false` to highlight all fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require_field_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags_schema: Option<crate::types::search::HighlighterTagsSchema>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::search::HighlighterType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matched_fields: Option<crate::types::common::Fields>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HighlightFields {
    Object(std::collections::BTreeMap<String, crate::types::search::HighlightField>),
    Array(Vec<std::collections::BTreeMap<String, crate::types::search::HighlightField>>),
}
impl<'de> serde::Deserialize<'de> for HighlightFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(HighlightFields::Object),
            Some(b'[') => serde_json::from_str(text).map(HighlightFields::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "HighlightFields"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterEncoder {
    #[doc = "`default`"]
    Default,
    #[doc = "`html`"]
    Html,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterEncoder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterEncoder::Default => "default",
            HighlighterEncoder::Html => "html",
            HighlighterEncoder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterEncoder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterEncoder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => HighlighterEncoder::Default,
            "html" => HighlighterEncoder::Html,
            _ => HighlighterEncoder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterFragmenter {
    #[doc = "`simple`"]
    Simple,
    #[doc = "`span`"]
    Span,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterFragmenter {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterFragmenter::Simple => "simple",
            HighlighterFragmenter::Span => "span",
            HighlighterFragmenter::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterFragmenter {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterFragmenter {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple" => HighlighterFragmenter::Simple,
            "span" => HighlighterFragmenter::Span,
            _ => HighlighterFragmenter::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterOrder {
    #[doc = "`score`"]
    Score,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterOrder::Score => "score",
            HighlighterOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "score" => HighlighterOrder::Score,
            _ => HighlighterOrder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HighlighterTagsSchema {
    #[doc = "`default`"]
    Default,
    #[doc = "`styled`"]
    Styled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HighlighterTagsSchema {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HighlighterTagsSchema::Default => "default",
            HighlighterTagsSchema::Styled => "styled",
            HighlighterTagsSchema::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HighlighterTagsSchema {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HighlighterTagsSchema {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => HighlighterTagsSchema::Default,
            "styled" => HighlighterTagsSchema::Styled,
            _ => HighlighterTagsSchema::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct HighlighterType {
    raw: Box<serde_json::value::RawValue>,
}
impl HighlighterType {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `builtin` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_builtin(
        &self,
    ) -> std::result::Result<
        crate::types::search::BuiltinHighlighterType,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "builtin")
    }
    #[doc = "Parses the value as the `custom` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "custom")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsMetadata {
    pub hits: Vec<crate::types::search::HitsMetadataHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::search::HitsMetadataTotal>,
}
impl HitsMetadata {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadataHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::HitsMetadataHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadataHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadataHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadataHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::HitsMetadataHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries =
                                map.next_value::<Option<
                                    crate::types::search::HitsMetadataHitsItemMatchedQueries,
                                >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadataHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadataHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl HitsMetadataHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'[') => serde_json::from_str(text).map(HitsMetadataHitsItemMatchedQueries::Names),
            Some(b'{') => {
                serde_json::from_str(text).map(HitsMetadataHitsItemMatchedQueries::Scores)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "HitsMetadataHitsItemMatchedQueries"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HitsMetadataJsonValue {
    pub hits: Vec<crate::types::search::HitsMetadataJsonValueHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::search::HitsMetadataJsonValueTotal>,
}
impl HitsMetadataJsonValue {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct HitsMetadataJsonValueHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HitsMetadataJsonValueHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HitsMetadataJsonValueHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::HitsMetadataJsonValueHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HitsMetadataJsonValueHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HitsMetadataJsonValueHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl HitsMetadataJsonValueHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataJsonValueHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed =
            match text.as_bytes().first() {
                Some(b'[') => serde_json::from_str(text)
                    .map(HitsMetadataJsonValueHitsItemMatchedQueries::Names),
                Some(b'{') => serde_json::from_str(text)
                    .map(HitsMetadataJsonValueHitsItemMatchedQueries::Scores),
                other => {
                    // the type name is an argument, not part of the literal, so
                    // that rustfmt can wrap the line for long names
                    return Err(serde::de::Error::custom(format_args!(
                        "invalid type: {}, expected a {} value",
                        crate::types::union::json_kind_name(other.copied()),
                        "HitsMetadataJsonValueHitsItemMatchedQueries"
                    )));
                }
            };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataJsonValueTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataJsonValueTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(HitsMetadataJsonValueTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(HitsMetadataJsonValueTotal::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "HitsMetadataJsonValueTotal"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for HitsMetadataJsonValueTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl HitsMetadataJsonValueTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HitsMetadataTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for HitsMetadataTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(HitsMetadataTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(HitsMetadataTotal::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "HitsMetadataTotal"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for HitsMetadataTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl HitsMetadataTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InnerHits {
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::search::SourceConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapse: Option<Box<crate::types::search::FieldCollapse>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docvalue_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "The inner hit that initiates document offset."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<Box<crate::types::search::Highlight>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unmapped: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_fields:
        Option<std::collections::BTreeMap<String, crate::types::common::ScriptField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no_primary_term: Option<bool>,
    #[doc = "The maximum number of hits to return per `inner_hits`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::types::common::Sort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_fields: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_scores: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InnerHitsResult {
    pub hits: crate::types::search::HitsMetadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LaplaceSmoothingModel {
    pub alpha: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LinearInterpolationSmoothingModel {
    pub bigram_lambda: f64,
    pub trigram_lambda: f64,
    pub unigram_lambda: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedIdentity {
    #[serde(rename = "_nested")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested: Option<Box<crate::types::search::NestedIdentity>>,
    pub field: crate::types::common::Field,
    pub offset: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggest {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::PhraseSuggestOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggestCollate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,
    pub query: crate::types::search::PhraseSuggestCollateQuery,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggestCollateQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggestHighlight {
    pub post_tag: String,
    pub pre_tag: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggestOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlighted: Option<String>,
    pub score: f64,
    pub text: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PhraseSuggestOptions {
    PhraseSuggestOption(crate::types::search::PhraseSuggestOption),
    Array(Vec<crate::types::search::PhraseSuggestOption>),
}
impl<'de> serde::Deserialize<'de> for PhraseSuggestOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(PhraseSuggestOptions::PhraseSuggestOption),
            Some(b'[') => serde_json::from_str(text).map(PhraseSuggestOptions::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "PhraseSuggestOptions"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhraseSuggester {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate: Option<crate::types::search::PhraseSuggestCollate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_generator: Option<Vec<crate::types::search::DirectGenerator>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_unigrams: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gram_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<crate::types::search::PhraseSuggestHighlight>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_errors: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub real_word_error_likelihood: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smoothing: Option<crate::types::search::SmoothingModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_limit: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PointInTimeReference {
    pub id: crate::types::common::Id,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<crate::types::common::Duration>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorExecutionDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_data: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_data: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Profile {
    pub shards: Vec<crate::types::search::ShardProfile>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryBreakdown {
    pub advance: i64,
    pub advance_count: i64,
    pub build_scorer: i64,
    pub build_scorer_count: i64,
    pub compute_max_score: i64,
    pub compute_max_score_count: i64,
    pub create_weight: i64,
    pub create_weight_count: i64,
    pub r#match: i64,
    pub match_count: i64,
    pub next_doc: i64,
    pub next_doc_count: i64,
    pub score: i64,
    pub score_count: i64,
    pub set_min_competitive_score: i64,
    pub set_min_competitive_score_count: i64,
    pub shallow_advance: i64,
    pub shallow_advance_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryProfile {
    pub breakdown: crate::types::search::QueryBreakdown,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::search::QueryProfile>>,
    pub description: String,
    pub time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchProfile {
    pub collector: Vec<crate::types::search::Collector>,
    pub query: Vec<crate::types::search::QueryProfile>,
    pub rewrite_time: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResponse {
    #[serde(rename = "_clusters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<crate::types::common::ClusterStatistics>,
    #[serde(rename = "_scroll_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<crate::types::common::ScrollId>,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>>,
    pub hits: crate::types::search::SearchResponseHits,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_reduce_phases: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_took: Option<crate::types::common::PhaseTook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pit_id: Option<crate::types::common::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<crate::types::search::ProcessorExecutionDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::types::search::Profile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated_early: Option<bool>,
    pub timed_out: bool,
    pub took: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResponseHits {
    pub hits: Vec<crate::types::search::SearchResponseHitsHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::search::SearchResponseHitsTotal>,
}
impl SearchResponseHits {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct SearchResponseHitsHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::SearchResponseHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResponseHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResponseHitsHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::SearchResponseHitsHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::SearchResponseHitsHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResponseHitsHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResponseHitsHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SearchResponseHitsHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResponseHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'[') => {
                serde_json::from_str(text).map(SearchResponseHitsHitsItemMatchedQueries::Names)
            }
            Some(b'{') => {
                serde_json::from_str(text).map(SearchResponseHitsHitsItemMatchedQueries::Scores)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SearchResponseHitsHitsItemMatchedQueries"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResponseHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SearchResponseHitsTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(SearchResponseHitsTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SearchResponseHitsTotal::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SearchResponseHitsTotal"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for SearchResponseHitsTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl SearchResponseHitsTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    #[serde(rename = "_clusters")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clusters: Option<crate::types::common::ClusterStatistics>,
    #[serde(rename = "_scroll_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scroll_id: Option<crate::types::common::ScrollId>,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>>,
    pub hits: crate::types::search::SearchResultHits,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_reduce_phases: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_took: Option<crate::types::common::PhaseTook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pit_id: Option<crate::types::common::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<crate::types::search::ProcessorExecutionDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<crate::types::search::Profile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Suggest>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminated_early: Option<bool>,
    pub timed_out: bool,
    pub took: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResultHits {
    pub hits: Vec<crate::types::search::SearchResultHitsHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::search::SearchResultHitsTotal>,
}
impl SearchResultHits {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct SearchResultHitsHitsItem {
    pub explanation: Option<crate::types::explain::Explanation>,
    pub id: Option<crate::types::common::Id>,
    pub ignored: Option<Vec<String>>,
    pub index: Option<crate::types::common::IndexName>,
    pub nested: Option<crate::types::search::NestedIdentity>,
    pub node: Option<String>,
    pub primary_term: Option<i64>,
    pub routing: Option<String>,
    pub score: Option<f64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shard: Option<String>,
    pub source: Option<Box<serde_json::value::RawValue>>,
    pub r#type: Option<crate::types::common::Type>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub highlight: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub ignored_field_values: Option<std::collections::BTreeMap<String, Vec<String>>>,
    pub inner_hits:
        Option<std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>>,
    #[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
    pub matched_queries: Option<crate::types::search::SearchResultHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SearchResultHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SearchResultHitsHitsItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_explanation: Option<crate::types::explain::Explanation> = None;
                let mut seen_explanation = false;
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut seen_id = false;
                let mut f_ignored: Option<Vec<String>> = None;
                let mut seen_ignored = false;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_nested: Option<crate::types::search::NestedIdentity> = None;
                let mut seen_nested = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<String> = None;
                let mut seen_routing = false;
                let mut f_score: Option<f64> = None;
                let mut seen_score = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shard: Option<String> = None;
                let mut seen_shard = false;
                let mut f_source: Option<Box<serde_json::value::RawValue>> = None;
                let mut seen_source = false;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_fields = false;
                let mut f_highlight: Option<std::collections::BTreeMap<String, Vec<String>>> = None;
                let mut seen_highlight = false;
                let mut f_ignored_field_values: Option<
                    std::collections::BTreeMap<String, Vec<String>>,
                > = None;
                let mut seen_ignored_field_values = false;
                let mut f_inner_hits: Option<
                    std::collections::BTreeMap<String, crate::types::search::InnerHitsResult>,
                > = None;
                let mut seen_inner_hits = false;
                let mut f_matched_queries: Option<
                    crate::types::search::SearchResultHitsHitsItemMatchedQueries,
                > = None;
                let mut seen_matched_queries = false;
                let mut f_sort: Option<crate::types::common::SortResults> = None;
                let mut seen_sort = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_explanation" => {
                            if seen_explanation {
                                return Err(serde::de::Error::duplicate_field("_explanation"));
                            }
                            seen_explanation = true;
                            f_explanation =
                                map.next_value::<Option<crate::types::explain::Explanation>>()?;
                        }
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "_ignored" => {
                            if seen_ignored {
                                return Err(serde::de::Error::duplicate_field("_ignored"));
                            }
                            seen_ignored = true;
                            f_ignored = map.next_value::<Option<Vec<String>>>()?;
                        }
                        "_index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "_nested" => {
                            if seen_nested {
                                return Err(serde::de::Error::duplicate_field("_nested"));
                            }
                            seen_nested = true;
                            f_nested =
                                map.next_value::<Option<crate::types::search::NestedIdentity>>()?;
                        }
                        "_node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("_node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing = map.next_value::<Option<String>>()?;
                        }
                        "_score" => {
                            if seen_score {
                                return Err(serde::de::Error::duplicate_field("_score"));
                            }
                            seen_score = true;
                            f_score = map.next_value::<Option<f64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shard" => {
                            if seen_shard {
                                return Err(serde::de::Error::duplicate_field("_shard"));
                            }
                            seen_shard = true;
                            f_shard = map.next_value::<Option<String>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source =
                                map.next_value::<Option<Box<serde_json::value::RawValue>>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "ignored_field_values" => {
                            if seen_ignored_field_values {
                                return Err(serde::de::Error::duplicate_field(
                                    "ignored_field_values",
                                ));
                            }
                            seen_ignored_field_values = true;
                            f_ignored_field_values = map.next_value::<Option<std::collections::BTreeMap<String, Vec<String>>>>()?;
                        }
                        "inner_hits" => {
                            if seen_inner_hits {
                                return Err(serde::de::Error::duplicate_field("inner_hits"));
                            }
                            seen_inner_hits = true;
                            f_inner_hits = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::search::InnerHitsResult,
                                >,
                            >>()?;
                        }
                        "matched_queries" => {
                            if seen_matched_queries {
                                return Err(serde::de::Error::duplicate_field("matched_queries"));
                            }
                            seen_matched_queries = true;
                            f_matched_queries = map.next_value::<Option<
                                crate::types::search::SearchResultHitsHitsItemMatchedQueries,
                            >>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort =
                                map.next_value::<Option<crate::types::common::SortResults>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SearchResultHitsHitsItem {
                    explanation: f_explanation,
                    id: f_id,
                    ignored: f_ignored,
                    index: f_index,
                    nested: f_nested,
                    node: f_node,
                    primary_term: f_primary_term,
                    routing: f_routing,
                    score: f_score,
                    seq_no: f_seq_no,
                    shard: f_shard,
                    source: f_source,
                    r#type: f_type,
                    version: f_version,
                    fields: f_fields,
                    highlight: f_highlight,
                    ignored_field_values: f_ignored_field_values,
                    inner_hits: f_inner_hits,
                    matched_queries: f_matched_queries,
                    sort: f_sort,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SearchResultHitsHitsItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.explanation {
            map.serialize_entry("_explanation", v)?;
        }
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        if let Some(v) = &self.ignored {
            map.serialize_entry("_ignored", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("_index", v)?;
        }
        if let Some(v) = &self.nested {
            map.serialize_entry("_nested", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("_node", v)?;
        }
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.score {
            map.serialize_entry("_score", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shard {
            map.serialize_entry("_shard", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.ignored_field_values {
            map.serialize_entry("ignored_field_values", v)?;
        }
        if let Some(v) = &self.inner_hits {
            map.serialize_entry("inner_hits", v)?;
        }
        if let Some(v) = &self.matched_queries {
            map.serialize_entry("matched_queries", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
impl SearchResultHitsHitsItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[doc = "The names of queries that matched the document. When `include_named_queries_score` is false (default), returns an array of query names. When true, returns an object mapping query names to their scores."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResultHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsHitsItemMatchedQueries {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'[') => {
                serde_json::from_str(text).map(SearchResultHitsHitsItemMatchedQueries::Names)
            }
            Some(b'{') => {
                serde_json::from_str(text).map(SearchResultHitsHitsItemMatchedQueries::Scores)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SearchResultHitsHitsItemMatchedQueries"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchResultHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for SearchResultHitsTotal {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(SearchResultHitsTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SearchResultHitsTotal::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SearchResultHitsTotal"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for SearchResultHitsTotal {
    fn value(&self) -> i64 {
        match self {
            Self::TotalHits(total) => total.value,
            Self::Integer(value) => *value,
        }
    }

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        match self {
            Self::TotalHits(total) => &total.relation,
            Self::Integer(_) => &crate::types::total_hits::ACCURATE,
        }
    }
}

impl SearchResultHitsTotal {
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {
        crate::types::TotalHitCount::value(self)
    }

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {
        crate::types::TotalHitCount::relation(self)
    }

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {
        crate::types::TotalHitCount::is_accurate(self)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardProfile {
    pub aggregations: Vec<crate::types::search::AggregationProfile>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetch: Option<crate::types::search::FetchProfile>,
    pub id: String,
    pub searches: Vec<crate::types::search::SearchProfile>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmoothingModel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub laplace: Option<crate::types::search::LaplaceSmoothingModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linear_interpolation: Option<crate::types::search::LinearInterpolationSmoothingModel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stupid_backoff: Option<crate::types::search::StupidBackoffSmoothingModel>,
}

#[doc = "Defines how to fetch a source. Fetching can be disabled entirely, or the source can be filtered."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SourceConfig {
    raw: Box<serde_json::value::RawValue>,
}
impl SourceConfig {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `fetch` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fetch(&self) -> std::result::Result<bool, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "fetch")
    }
    #[doc = "Parses the value as the `filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filter(
        &self,
    ) -> std::result::Result<crate::types::search::SourceFilter, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "filter")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SourceFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl SourceFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `includes` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_includes(
        &self,
    ) -> std::result::Result<crate::types::common::Fields, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "includes")
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<crate::types::search::SourceFilterObject, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceFilterObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<crate::types::common::Fields>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StringDistance {
    #[doc = "`damerau_levenshtein`"]
    DamerauLevenshtein,
    #[doc = "`internal`"]
    Internal,
    #[doc = "`jaro_winkler`"]
    JaroWinkler,
    #[doc = "`levenshtein`"]
    Levenshtein,
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StringDistance {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StringDistance::DamerauLevenshtein => "damerau_levenshtein",
            StringDistance::Internal => "internal",
            StringDistance::JaroWinkler => "jaro_winkler",
            StringDistance::Levenshtein => "levenshtein",
            StringDistance::Ngram => "ngram",
            StringDistance::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StringDistance {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StringDistance {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "damerau_levenshtein" => StringDistance::DamerauLevenshtein,
            "internal" => StringDistance::Internal,
            "jaro_winkler" => StringDistance::JaroWinkler,
            "levenshtein" => StringDistance::Levenshtein,
            "ngram" => StringDistance::Ngram,
            _ => StringDistance::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StupidBackoffSmoothingModel {
    pub discount: f64,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Suggest {
    raw: Box<serde_json::value::RawValue>,
}
impl Suggest {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `completion` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_completion(
        &self,
    ) -> std::result::Result<crate::types::search::SuggestCompletion, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "completion")
    }
    #[doc = "Parses the value as the `phrase` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phrase(
        &self,
    ) -> std::result::Result<crate::types::search::PhraseSuggest, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "phrase")
    }
    #[doc = "Parses the value as the `term` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_term(
        &self,
    ) -> std::result::Result<crate::types::search::TermSuggest, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "term")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestCompletion {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::SuggestCompletionOptions,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SuggestCompletionOptions {
    Object(crate::types::search::SuggestCompletionOptionsObject),
    Array(Vec<crate::types::search::SuggestCompletionOptionsArrayItem>),
}
impl<'de> serde::Deserialize<'de> for SuggestCompletionOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(SuggestCompletionOptions::Object),
            Some(b'[') => serde_json::from_str(text).map(SuggestCompletionOptions::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SuggestCompletionOptions"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestCompletionOptionsArrayItem {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(rename = "_routing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Context>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(rename = "score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _score: Option<f64>,
    pub text: String,
}
impl SuggestCompletionOptionsArrayItem {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestCompletionOptionsObject {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(rename = "_routing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<std::collections::BTreeMap<String, Vec<crate::types::search::Context>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(rename = "score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _score: Option<f64>,
    pub text: String,
}
impl SuggestCompletionOptionsObject {
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {
        self.source
            .as_deref()
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestFuzziness {
    pub fuzziness: String,
    pub min_length: i32,
    pub prefix_length: i32,
    pub transpositions: bool,
    pub unicode_aware: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SuggestSort {
    #[doc = "`frequency`"]
    Frequency,
    #[doc = "`score`"]
    Score,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SuggestSort {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SuggestSort::Frequency => "frequency",
            SuggestSort::Score => "score",
            SuggestSort::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SuggestSort {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SuggestSort {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "frequency" => SuggestSort::Frequency,
            "score" => SuggestSort::Score,
            _ => SuggestSort::Unknown(s),
        })
    }
}

#[derive(Debug, Clone)]
pub struct Suggester {
    #[doc = "The global suggest text, which avoids repetition when the same text is used in several suggesters."]
    pub text: Option<String>,
    pub additional: std::collections::BTreeMap<String, crate::types::search::FieldSuggester>,
}
impl<'de> serde::Deserialize<'de> for Suggester {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Suggester;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Suggester object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_text: Option<String> = None;
                let mut seen_text = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::search::FieldSuggester,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "text" => {
                            if seen_text {
                                return Err(serde::de::Error::duplicate_field("text"));
                            }
                            seen_text = true;
                            f_text = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Suggester {
                    text: f_text,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Suggester {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.text {
            map.serialize_entry("text", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermSuggest {
    pub length: i32,
    pub offset: i32,
    pub text: String,
    pub options: crate::types::search::TermSuggestOptions,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermSuggestOption {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collate_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freq: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlighted: Option<String>,
    pub score: f64,
    pub text: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TermSuggestOptions {
    TermSuggestOption(crate::types::search::TermSuggestOption),
    Array(Vec<crate::types::search::TermSuggestOption>),
}
impl<'de> serde::Deserialize<'de> for TermSuggestOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text).map(TermSuggestOptions::TermSuggestOption),
            Some(b'[') => serde_json::from_str(text).map(TermSuggestOptions::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TermSuggestOptions"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermSuggester {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase_terms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_edits: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_inspections: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_term_freq: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_freq: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix_length: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_size: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::types::search::SuggestSort>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_distance: Option<crate::types::search::StringDistance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_mode: Option<crate::types::common::SuggestMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TotalHits {
    pub relation: crate::types::search::TotalHitsRelation,
    pub value: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalHitsRelation {
    #[doc = "`eq`"]
    Eq,
    #[doc = "`gte`"]
    Gte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TotalHitsRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TotalHitsRelation::Eq => "eq",
            TotalHitsRelation::Gte => "gte",
            TotalHitsRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TotalHitsRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TotalHitsRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "eq" => TotalHitsRelation::Eq,
            "gte" => TotalHitsRelation::Gte,
            _ => TotalHitsRelation::Unknown(s),
        })
    }
}

#[doc = "The number of hits matching the query. When `true`, the exact number of hits is returned at the cost of some performance. When `false`, the response does not include the total number of hits matching the query. Default is `10,000` hits."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TrackHits {
    Enabled(bool),
    Count(i32),
}
impl<'de> serde::Deserialize<'de> for TrackHits {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(TrackHits::Enabled),
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(TrackHits::Count),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TrackHits"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
