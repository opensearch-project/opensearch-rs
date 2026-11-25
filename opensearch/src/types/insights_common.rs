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
pub enum AggregationType {
    #[doc = "`AVERAGE`"]
    Average,
    #[doc = "`NONE`"]
    None,
    #[doc = "`SUM`"]
    Sum,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AggregationType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AggregationType::Average => "AVERAGE",
            AggregationType::None => "NONE",
            AggregationType::Sum => "SUM",
            AggregationType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AggregationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AggregationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "AVERAGE" => AggregationType::Average,
            "NONE" => AggregationType::None,
            "SUM" => AggregationType::Sum,
            _ => AggregationType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GroupingType {
    #[doc = "`NONE`"]
    None,
    #[doc = "`SIMILARITY`"]
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
            "NONE" => GroupingType::None,
            "SIMILARITY" => GroupingType::Similarity,
            _ => GroupingType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Measurement {
    #[serde(rename = "aggregationType")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregation_type: Option<crate::types::insights_common::AggregationType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<f64>,
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
    #[doc = "The backend roles of the user who initiated the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "A description of the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether the search request failed during execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_by: Option<crate::types::insights_common::GroupingType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The indexes involved in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<String>>,
    #[doc = "Whether the search query was cancelled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_cancelled: Option<bool>,
    #[doc = "Additional labels for the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub measurements: Option<crate::types::insights_common::Measurements>,
    #[doc = "The node ID associated with the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_latency_map: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The hash code of the query group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_group_hashcode: Option<String>,
    #[doc = "The search query type (for example, `query_then_fetch`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_type: Option<String>,
    #[doc = "The search request body source associated with the query, serialized as a string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Whether the source was truncated due to length limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_truncated: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_resource_usages: Option<Vec<crate::types::insights_common::TaskResourceUsages>>,
    #[doc = "The timestamp of the query execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
    #[doc = "The total number of shards involved in the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_shards: Option<i32>,
    #[doc = "The roles of the user who initiated the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_roles: Option<Vec<String>>,
    #[doc = "The username who initiated the search query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[doc = "The workload management group ID associated with the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wlm_group_id: Option<String>,
}
