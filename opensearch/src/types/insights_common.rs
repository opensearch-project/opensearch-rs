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

//! Response types of the `insights_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupingType {
    #[doc = "`NONE` (also accepted: `none`)"]
    None,
    #[doc = "`SIMILARITY` (also accepted: `similarity`)"]
    Similarity,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GroupingType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GroupingType::None => "NONE",
            GroupingType::Similarity => "SIMILARITY",
            GroupingType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GroupingType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GroupingType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "NONE" | "none" => GroupingType::None,
            "SIMILARITY" | "similarity" => GroupingType::Similarity,
            _ => GroupingType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Measurement {
    #[serde(rename = "aggregationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Measurements {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::insights_common::Measurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latency: Option<crate::types::insights_common::Measurement>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::insights_common::Measurement>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Source {
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::search::SourceConfig>,
    #[doc = "Defines the aggregations that are run as part of the search request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collapse: Option<crate::types::search::FieldCollapse>,
    #[doc = "Array of wildcard (`*`) patterns. The request returns doc values for field names matching these patterns in the `hits.fields` property of the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docvalue_fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "If `true`, returns detailed information about score computation as part of a hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explain: Option<bool>,
    #[doc = "Configuration of search extensions defined by OpenSearch plugins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ext: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[doc = "Array of wildcard (`*`) patterns. The request returns values for field names matching these patterns in the `hits.fields` property of the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common_query_dsl::FieldAndFormat>>,
    #[doc = "Starting document offset. Needs to be non-negative. By default, you cannot page through more than 10,000 hits using the `from` and `size` parameters. To page through more hits, use the `search_after` parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highlight: Option<crate::types::search::Highlight>,
    #[doc = "Boosts the `_score` of documents from specified indexes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices_boost: Option<Vec<std::collections::BTreeMap<String, f64>>>,
    #[doc = "Minimum `_score` for matching documents. Documents with a lower `_score` are not included in the search results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pit: Option<crate::types::search::PointInTimeReference>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Set to `true` to return detailed timing information about the execution of individual components in a search request. NOTE: This is a debugging tool and adds significant overhead to search execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "Retrieve a script evaluation (based on different fields) for each hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_fields:
        Option<std::collections::BTreeMap<String, crate::types::common::ScriptField>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_after: Option<crate::types::common::SortResults>,
    #[doc = "If `true`, returns sequence number and primary term of the last modification of each hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no_primary_term: Option<bool>,
    #[doc = "The number of hits to return. By default, you cannot page through more than 10,000 hits using the `from` and `size` parameters. To page through more hits, use the `search_after` parameter."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slice: Option<crate::types::common::SlicedScroll>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::types::common::Sort>,
    #[doc = "Stats groups to associate with the search. Each group maintains a statistics aggregation for its associated searches. You can retrieve these stats using the indexes stats API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stored_fields: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest: Option<crate::types::search::Suggester>,
    #[doc = "Maximum number of documents to collect for each shard. If a query reaches this limit, OpenSearch terminates the query early. OpenSearch collects documents before sorting. Use with caution. OpenSearch applies this parameter to each shard handling the request. When possible, let OpenSearch perform early termination automatically. Avoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers. If set to `0` (default), the query does not terminate early."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terminate_after: Option<i32>,
    #[doc = "Specifies the period of time to wait for a response from each shard. If no response is received before the timeout expires, the request fails and returns an error. Defaults to no timeout."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[doc = "If `true`, calculate and return document scores, even if the scores are not used for sorting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_scores: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub track_total_hits: Option<crate::types::search::TrackHits>,
    #[doc = "If `true`, returns document version as part of a hit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskResourceUsage {
    #[doc = "The CPU time used in nanoseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_time_in_nanos: Option<i64>,
    #[doc = "The memory usage in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_in_bytes: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskResourceUsages {
    #[doc = "The action type of the task."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[doc = "The node ID where the task was executed."]
    #[serde(rename = "nodeId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[doc = "The parent task ID."]
    #[serde(rename = "parentTaskId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<i64>,
    #[doc = "The task ID."]
    #[serde(rename = "taskId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_id: Option<i64>,
    #[serde(rename = "taskResourceUsage")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_resource_usage: Option<crate::types::insights_common::TaskResourceUsage>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopQueriesResponse {
    pub top_queries: Vec<crate::types::insights_common::TopQuery>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopQuery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<crate::types::insights_common::GroupingType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The indexes involved in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<String>>,
    #[doc = "Additional labels for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurements: Option<crate::types::insights_common::Measurements>,
    #[doc = "The node ID associated with the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_latency_map:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The hash code of the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_hashcode: Option<String>,
    #[doc = "The search query type (for example, `query_then_fetch`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::insights_common::Source>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_resource_usages: Option<Vec<crate::types::insights_common::TaskResourceUsages>>,
    #[doc = "The timestamp of the query execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[doc = "The total number of shards involved in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_shards: Option<i64>,
}
