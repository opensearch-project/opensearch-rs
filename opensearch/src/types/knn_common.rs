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

//! Response types of the `knn_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeletedModel {
    pub model_id: String,
    pub result: crate::types::knn_common::DeletionResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeletionResult {
    #[doc = "`deleted`"]
    Deleted,
    #[doc = "`error`"]
    Error,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DeletionResult {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DeletionResult::Deleted => "deleted",
            DeletionResult::Error => "error",
            DeletionResult::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DeletionResult {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DeletionResult {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "deleted" => DeletionResult::Deleted,
            "error" => DeletionResult::Error,
            _ => DeletionResult::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphMergeStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_docs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_docs: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphRefreshStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GraphStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge: Option<crate::types::knn_common::GraphMergeStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<crate::types::knn_common::GraphRefreshStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_capacity_reached: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub faiss_initialized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_index_errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_index_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_memory_usage: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_memory_usage_percentage: Option<crate::types::common::PercentageNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_query_errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_query_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_stats: Option<crate::types::knn_common::GraphStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_from_model_degraded: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices_in_cache:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knn_query_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub knn_query_with_filter_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_exception_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_success_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lucene_initialized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance_query_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_distance_query_with_filter_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score_query_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score_query_with_filter_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nmslib_initialized: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_vector_index_build_stats:
        Option<crate::types::knn_common::RemoteVectorIndexBuildStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_compilation_errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_compilations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_query_errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_query_requests: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_load_time: Option<crate::types::common::DurationValueUnitNanos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_errors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_memory_usage: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_memory_usage_percentage: Option<crate::types::common::PercentageNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub training_requests: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVectorIndexBuildStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_stats: Option<crate::types::knn_common::RemoteVectorIndexBuildStatsDetails>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_stats: Option<crate::types::knn_common::RemoteVectorIndexClientStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_stats: Option<crate::types::knn_common::RemoteVectorIndexRepositoryStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVectorIndexBuildStatsDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_current_flush_operations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_current_flush_size: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_current_merge_operations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_current_merge_size: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_flush_time_in_millis:
        Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_index_build_merge_time_in_millis:
        Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVectorIndexClientStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_request_failure_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_request_success_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_build_failure_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_build_success_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_request_failure_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_request_success_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiting_time_in_ms: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteVectorIndexRepositoryStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_failure_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_success_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_read_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_write_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_failure_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_success_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circuit_breaker_triggered: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_index_status: Option<crate::types::common::HealthStatus>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::knn_common::NodeStats>>,
}
