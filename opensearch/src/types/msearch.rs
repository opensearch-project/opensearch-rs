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

//! Response types of the `msearch` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `msearch` API"]
pub type MsearchResponse = crate::types::msearch::MultiSearchResult;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiSearchItem {
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
    pub hits: crate::types::msearch::MultiSearchItemHits,
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiSearchItemHits {
    pub hits: Vec<crate::types::msearch::MultiSearchItemHitsHitsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::msearch::MultiSearchItemHitsTotal>,
}
impl MultiSearchItemHits {
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {
        self.total.as_ref().map(|total| total.value())
    }
}

#[derive(Debug, Clone)]
pub struct MultiSearchItemHitsHitsItem {
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
    pub matched_queries: Option<crate::types::msearch::MultiSearchItemHitsHitsItemMatchedQueries>,
    pub sort: Option<crate::types::common::SortResults>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MultiSearchItemHitsHitsItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiSearchItemHitsHitsItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiSearchItemHitsHitsItem object")
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
                    crate::types::msearch::MultiSearchItemHitsHitsItemMatchedQueries,
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
                                crate::types::msearch::MultiSearchItemHitsHitsItemMatchedQueries,
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
                Ok(MultiSearchItemHitsHitsItem {
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
impl serde::Serialize for MultiSearchItemHitsHitsItem {
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
impl MultiSearchItemHitsHitsItem {
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
pub enum MultiSearchItemHitsHitsItemMatchedQueries {
    Names(Vec<String>),
    Scores(std::collections::BTreeMap<String, f64>),
}
impl<'de> serde::Deserialize<'de> for MultiSearchItemHitsHitsItemMatchedQueries {
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
                serde_json::from_str(text).map(MultiSearchItemHitsHitsItemMatchedQueries::Names)
            }
            Some(b'{') => {
                serde_json::from_str(text).map(MultiSearchItemHitsHitsItemMatchedQueries::Scores)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "MultiSearchItemHitsHitsItemMatchedQueries"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The total number of hits, present only if `track_total_hits` is not set to `false` in the search request."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MultiSearchItemHitsTotal {
    TotalHits(crate::types::search::TotalHits),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for MultiSearchItemHitsTotal {
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
            Some(b'{') => serde_json::from_str(text).map(MultiSearchItemHitsTotal::TotalHits),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(MultiSearchItemHitsTotal::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "MultiSearchItemHitsTotal"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
impl crate::types::TotalHitCount for MultiSearchItemHitsTotal {
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

impl MultiSearchItemHitsTotal {
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
pub struct MultiSearchResult {
    pub responses: Vec<crate::types::msearch::ResponseItem>,
    pub took: f64,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ResponseItem {
    raw: Box<serde_json::value::RawValue>,
}
impl ResponseItem {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `multi_search_item` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_multi_search_item(
        &self,
    ) -> std::result::Result<crate::types::msearch::MultiSearchItem, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "multi_search_item")
    }
    #[doc = "Parses the value as the `error_response_base` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_error_response_base(
        &self,
    ) -> std::result::Result<crate::types::common::ErrorResponseBase, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "error_response_base")
    }
}
