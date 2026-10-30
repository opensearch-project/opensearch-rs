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

//! Response types of the `neural_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlatNeuralInfoStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<crate::types::neural_common::InfoStringStat>,
    #[serde(rename = "processors.ingest.skip_existing_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_skip_existing_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.sparse_encoding_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_sparse_encoding_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_delimiter_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_delimiter_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_fixed_char_length_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_fixed_char_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_fixed_token_length_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_fixed_token_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_embedding_processors_in_pipelines")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_embedding_processors_in_pipelines:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.ingest.text_image_embedding_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_image_embedding_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_arithmetic_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_arithmetic_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_geometric_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_geometric_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_harmonic_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_harmonic_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_rrf_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_rrf_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_l2_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_l_2_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_minmax_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_minmax_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_zscore_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_zscore_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.normalization_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_normalization_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.hybrid.rank_based_normalization_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_rank_based_normalization_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.neural_query_enricher_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_neural_query_enricher_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.neural_sparse_two_phase_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_neural_sparse_two_phase_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.rerank_by_field_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_rerank_by_field_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "processors.search.rerank_ml_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_rerank_ml_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlatNeuralNodeStats {
    #[serde(rename = "processors.ingest.semantic_field_chunking_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_semantic_field_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.semantic_field_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_semantic_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.skip_existing_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_skip_existing_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.sparse_encoding_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_sparse_encoding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_delimiter_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_delimiter_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_fixed_char_length_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_fixed_char_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_chunking_fixed_token_length_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_chunking_fixed_token_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_embedding_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.ingest.text_image_embedding_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_ingest_text_image_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_arithmetic_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_arithmetic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_geometric_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_geometric_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_harmonic_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_harmonic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.comb_rrf_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_comb_rrf_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_l2_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_l_2_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_minmax_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_minmax_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.norm_zscore_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_norm_zscore_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.normalization_processor_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.hybrid.rank_based_normalization_processor_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_hybrid_rank_based_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.neural_query_enricher_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_neural_query_enricher_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.neural_sparse_two_phase_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_neural_sparse_two_phase_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.rerank_by_field_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_rerank_by_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "processors.search.rerank_ml_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors_search_rerank_ml_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.hybrid.hybrid_query_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hybrid_hybrid_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.hybrid.hybrid_query_with_filter_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hybrid_hybrid_query_with_filter_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.hybrid.hybrid_query_with_inner_hits_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hybrid_hybrid_query_with_inner_hits_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.hybrid.hybrid_query_with_pagination_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hybrid_hybrid_query_with_pagination_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.neural.neural_query_against_knn_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_neural_neural_query_against_knn_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.neural.neural_query_against_semantic_dense_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_neural_neural_query_against_semantic_dense_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.neural.neural_query_against_semantic_sparse_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_neural_neural_query_against_semantic_sparse_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.neural.neural_query_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_neural_neural_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "query.neural_sparse.neural_sparse_query_requests")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_neural_sparse_neural_sparse_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "semantic_highlighting.semantic_highlighting_request_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_highlighting_semantic_highlighting_request_count:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlatNeuralStats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<crate::types::neural_common::FlatNeuralNodeStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::types::neural_common::FlatNeuralInfoStats>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::neural_common::FlatNeuralNodeStats>,
    >,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoCounterStat {
    Integer(i64),
    Object(crate::types::neural_common::InfoCounterStatObject),
}
impl<'de> serde::Deserialize<'de> for InfoCounterStat {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoCounterStat::Integer)
            }
            Some(b'{') => serde_json::from_str(text).map(InfoCounterStat::Object),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "InfoCounterStat"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InfoCounterStatObject {
    pub stat_type: crate::types::neural_common::InfoCounterStatObjectStatType,
    pub value: crate::types::neural_common::InfoCounterStatObjectValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfoCounterStatObjectStatType {
    #[doc = "`info_counter`"]
    InfoCounter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl InfoCounterStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            InfoCounterStatObjectStatType::InfoCounter => "info_counter",
            InfoCounterStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for InfoCounterStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InfoCounterStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info_counter" => InfoCounterStatObjectStatType::InfoCounter,
            _ => InfoCounterStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoCounterStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for InfoCounterStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoCounterStatObjectValue::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(InfoCounterStatObjectValue::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "InfoCounterStatObjectValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoStringStat {
    String(String),
    Object(crate::types::neural_common::InfoStringStatObject),
}
impl<'de> serde::Deserialize<'de> for InfoStringStat {
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
            Some(b'"') => serde_json::from_str(text).map(InfoStringStat::String),
            Some(b'{') => serde_json::from_str(text).map(InfoStringStat::Object),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "InfoStringStat"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InfoStringStatObject {
    pub stat_type: crate::types::neural_common::InfoStringStatObjectStatType,
    pub value: crate::types::neural_common::InfoStringStatObjectValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InfoStringStatObjectStatType {
    #[doc = "`info_string`"]
    InfoString,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl InfoStringStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            InfoStringStatObjectStatType::InfoString => "info_string",
            InfoStringStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for InfoStringStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for InfoStringStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info_string" => InfoStringStatObjectStatType::InfoString,
            _ => InfoStringStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum InfoStringStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for InfoStringStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(InfoStringStatObjectValue::Integer)
            }
            Some(b'"') => serde_json::from_str(text).map(InfoStringStatObjectValue::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "InfoStringStatObjectValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralInfoStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<crate::types::neural_common::InfoStringStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessors>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralInfoStatsProcessors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsIngest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearch>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralInfoStatsProcessorsIngest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_existing_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_encoding_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_delimiter_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_fixed_char_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_fixed_token_length_processors:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_embedding_processors_in_pipelines:
        Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_image_embedding_processors: Option<crate::types::neural_common::InfoCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralInfoStatsProcessorsSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<crate::types::neural_common::NestedNeuralInfoStatsProcessorsSearchHybrid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_enricher_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_sparse_two_phase_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerank_by_field_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerank_ml_processors: Option<crate::types::neural_common::InfoCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralInfoStatsProcessorsSearchHybrid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_arithmetic_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_geometric_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_harmonic_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_rrf_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(rename = "norm_l2_processors")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_l_2_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_minmax_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_zscore_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalization_processors: Option<crate::types::neural_common::InfoCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_based_normalization_processors: Option<crate::types::neural_common::InfoCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessors>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::types::neural_common::NestedNeuralNodeStatsQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_highlighting:
        Option<crate::types::neural_common::NestedNeuralNodeStatsSemanticHighlighting>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsProcessors {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsIngest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearch>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsProcessorsIngest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_field_chunking_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_field_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_existing_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_encoding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_delimiter_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_fixed_char_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_chunking_fixed_token_length_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_embedding_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_image_embedding_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsProcessorsSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<crate::types::neural_common::NestedNeuralNodeStatsProcessorsSearchHybrid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_enricher_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_sparse_two_phase_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerank_by_field_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerank_ml_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsProcessorsSearchHybrid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_arithmetic_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_geometric_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_harmonic_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comb_rrf_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(rename = "norm_l2_executions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_l_2_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_minmax_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norm_zscore_executions: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_based_normalization_processor_executions:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryHybrid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryNeural>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_sparse: Option<crate::types::neural_common::NestedNeuralNodeStatsQueryNeuralSparse>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsQueryHybrid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid_query_requests: Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid_query_with_filter_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid_query_with_inner_hits_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid_query_with_pagination_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsQueryNeural {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_against_knn_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_against_semantic_dense_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_against_semantic_sparse_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_query_requests: Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsQueryNeuralSparse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_sparse_query_requests:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralNodeStatsSemanticHighlighting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_highlighting_request_count:
        Option<crate::types::neural_common::TimestampedEventCounterStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedNeuralStats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_nodes: Option<crate::types::neural_common::NestedNeuralNodeStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::types::neural_common::NestedNeuralInfoStats>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::neural_common::NestedNeuralNodeStats>,
    >,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NeuralStats {
    raw: Box<serde_json::value::RawValue>,
}
impl NeuralStats {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `nested_neural_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nested_neural_stats(
        &self,
    ) -> std::result::Result<
        crate::types::neural_common::NestedNeuralStats,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nested_neural_stats")
    }
    #[doc = "Parses the value as the `flat_neural_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_flat_neural_stats(
        &self,
    ) -> std::result::Result<
        crate::types::neural_common::FlatNeuralStats,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "flat_neural_stats")
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TimestampedEventCounterStat {
    Integer(i64),
    Object(crate::types::neural_common::TimestampedEventCounterStatObject),
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStat {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(TimestampedEventCounterStat::Integer)
            }
            Some(b'{') => serde_json::from_str(text).map(TimestampedEventCounterStat::Object),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TimestampedEventCounterStat"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimestampedEventCounterStatObject {
    pub stat_type: crate::types::neural_common::TimestampedEventCounterStatObjectStatType,
    pub value: crate::types::neural_common::TimestampedEventCounterStatObjectValue,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minutes_since_last_event: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trailing_interval_value: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TimestampedEventCounterStatObjectStatType {
    #[doc = "`timestamped_event_counter`"]
    TimestampedEventCounter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TimestampedEventCounterStatObjectStatType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TimestampedEventCounterStatObjectStatType::TimestampedEventCounter => {
                "timestamped_event_counter"
            }
            TimestampedEventCounterStatObjectStatType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TimestampedEventCounterStatObjectStatType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStatObjectStatType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "timestamped_event_counter" => {
                TimestampedEventCounterStatObjectStatType::TimestampedEventCounter
            }
            _ => TimestampedEventCounterStatObjectStatType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TimestampedEventCounterStatObjectValue {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for TimestampedEventCounterStatObjectValue {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(TimestampedEventCounterStatObjectValue::Integer)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(TimestampedEventCounterStatObjectValue::String)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TimestampedEventCounterStatObjectValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
