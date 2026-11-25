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

//! Response types of the `wlm_common` specification namespace

#![allow(missing_docs)]

#[doc = "Workload management statistics for every workload group on a node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeWlmStats {
    #[doc = "A map of query group ID to statistics for that group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_groups:
        Option<std::collections::BTreeMap<String, crate::types::wlm_common::WorkloadGroupStats>>,
    #[doc = "A map of workload group ID to statistics for that group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workload_groups:
        Option<std::collections::BTreeMap<String, crate::types::wlm_common::WorkloadGroupStats>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponse {
    #[doc = "The ID of the query group, which can be used to associate query requests with the group and enforce the group's resource limits."]
    #[serde(rename = "_id")]
    pub id: String,
    #[doc = "The name of the query group."]
    pub name: String,
    #[doc = "The resiliency mode of the query group."]
    pub resiliency_mode: crate::types::wlm_common::ResiliencyMode,
    #[doc = "The resource limits for query requests in the query group."]
    pub resource_limits: crate::types::wlm_common::QueryGroupResponseResourceLimits,
    #[doc = "The time at which the query group was last updated."]
    pub updated_at: i64,
}

#[doc = "The resource limits for query requests in the query group."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct QueryGroupResponseResourceLimits {
    raw: Box<serde_json::value::RawValue>,
}
impl QueryGroupResponseResourceLimits {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponseResourceLimitsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    pub memory: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponseResourceLimitsObjectObject {
    pub cpu: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResiliencyMode {
    #[doc = "`enforced`"]
    Enforced,
    #[doc = "`monitor`"]
    Monitor,
    #[doc = "`soft`"]
    Soft,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ResiliencyMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ResiliencyMode::Enforced => "enforced",
            ResiliencyMode::Monitor => "monitor",
            ResiliencyMode::Soft => "soft",
            ResiliencyMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ResiliencyMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResiliencyMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "enforced" => ResiliencyMode::Enforced,
            "monitor" => ResiliencyMode::Monitor,
            "soft" => ResiliencyMode::Soft,
            _ => ResiliencyMode::Unknown(s),
        })
    }
}

#[doc = "Point-in-time resource usage statistics for a workload group on a node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WlmResourceStats {
    #[doc = "The number of cancellations caused by this resource reaching its cancellation threshold."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellations: Option<i64>,
    #[doc = "The last recorded usage for this resource, updated on the workload management enforcement interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_usage: Option<f64>,
    #[doc = "The number of rejections caused by this resource reaching its rejection threshold."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejections: Option<i64>,
}

#[doc = "A paginated or complete tabular listing of workload management statistics."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum WlmStatsListResponse {
    Array(Vec<crate::types::wlm_common::WlmStatsRecord>),
    Object(crate::types::wlm_common::WlmStatsListResponseObject),
}
impl<'de> serde::Deserialize<'de> for WlmStatsListResponse {
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
            Some(b'[') => serde_json::from_str(text).map(WlmStatsListResponse::Array),
            Some(b'{') => serde_json::from_str(text).map(WlmStatsListResponse::Object),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "WlmStatsListResponse"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WlmStatsListResponseObject {
    #[doc = "The token used to retrieve the next page. `null` when there are no more pages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    #[doc = "The workload management statistics in the current page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wlm_stats: Option<Vec<crate::types::wlm_common::WlmStatsRecord>>,
}

#[doc = "A tabular row of workload management statistics for a node and workload group."]
#[derive(Debug, Clone)]
pub struct WlmStatsRecord {
    #[doc = "The current CPU usage, or `NA` when CPU statistics are unavailable."]
    pub cpu_usage: Option<String>,
    #[doc = "The current memory usage, or `NA` when memory statistics are unavailable."]
    pub memory_usage: Option<String>,
    #[doc = "The ID of the node that reported the statistics."]
    pub node_id: Option<String>,
    #[doc = "The total number of cancelled requests as a string."]
    pub total_cancellations: Option<String>,
    #[doc = "The total number of completed requests as a string."]
    pub total_completions: Option<String>,
    #[doc = "The total number of rejected requests as a string."]
    pub total_rejections: Option<String>,
    #[doc = "The ID of the workload group."]
    pub workload_group_id: Option<String>,
    pub additional: std::collections::BTreeMap<String, String>,
}
impl<'de> serde::Deserialize<'de> for WlmStatsRecord {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WlmStatsRecord;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WlmStatsRecord object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_cpu_usage: Option<String> = None;
                let mut seen_cpu_usage = false;
                let mut f_memory_usage: Option<String> = None;
                let mut seen_memory_usage = false;
                let mut f_node_id: Option<String> = None;
                let mut seen_node_id = false;
                let mut f_total_cancellations: Option<String> = None;
                let mut seen_total_cancellations = false;
                let mut f_total_completions: Option<String> = None;
                let mut seen_total_completions = false;
                let mut f_total_rejections: Option<String> = None;
                let mut seen_total_rejections = false;
                let mut f_workload_group_id: Option<String> = None;
                let mut seen_workload_group_id = false;
                let mut residual: std::collections::BTreeMap<String, String> = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "CPU_USAGE" => {
                            if seen_cpu_usage {
                                return Err(serde::de::Error::duplicate_field("CPU_USAGE"));
                            }
                            seen_cpu_usage = true;
                            f_cpu_usage = map.next_value::<Option<String>>()?;
                        }
                        "MEMORY_USAGE" => {
                            if seen_memory_usage {
                                return Err(serde::de::Error::duplicate_field("MEMORY_USAGE"));
                            }
                            seen_memory_usage = true;
                            f_memory_usage = map.next_value::<Option<String>>()?;
                        }
                        "NODE_ID" => {
                            if seen_node_id {
                                return Err(serde::de::Error::duplicate_field("NODE_ID"));
                            }
                            seen_node_id = true;
                            f_node_id = map.next_value::<Option<String>>()?;
                        }
                        "TOTAL_CANCELLATIONS" => {
                            if seen_total_cancellations {
                                return Err(serde::de::Error::duplicate_field(
                                    "TOTAL_CANCELLATIONS",
                                ));
                            }
                            seen_total_cancellations = true;
                            f_total_cancellations = map.next_value::<Option<String>>()?;
                        }
                        "TOTAL_COMPLETIONS" => {
                            if seen_total_completions {
                                return Err(serde::de::Error::duplicate_field("TOTAL_COMPLETIONS"));
                            }
                            seen_total_completions = true;
                            f_total_completions = map.next_value::<Option<String>>()?;
                        }
                        "TOTAL_REJECTIONS" => {
                            if seen_total_rejections {
                                return Err(serde::de::Error::duplicate_field("TOTAL_REJECTIONS"));
                            }
                            seen_total_rejections = true;
                            f_total_rejections = map.next_value::<Option<String>>()?;
                        }
                        "WORKLOAD_GROUP_ID" => {
                            if seen_workload_group_id {
                                return Err(serde::de::Error::duplicate_field("WORKLOAD_GROUP_ID"));
                            }
                            seen_workload_group_id = true;
                            f_workload_group_id = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WlmStatsRecord {
                    cpu_usage: f_cpu_usage,
                    memory_usage: f_memory_usage,
                    node_id: f_node_id,
                    total_cancellations: f_total_cancellations,
                    total_completions: f_total_completions,
                    total_rejections: f_total_rejections,
                    workload_group_id: f_workload_group_id,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WlmStatsRecord {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.cpu_usage {
            map.serialize_entry("CPU_USAGE", v)?;
        }
        if let Some(v) = &self.memory_usage {
            map.serialize_entry("MEMORY_USAGE", v)?;
        }
        if let Some(v) = &self.node_id {
            map.serialize_entry("NODE_ID", v)?;
        }
        if let Some(v) = &self.total_cancellations {
            map.serialize_entry("TOTAL_CANCELLATIONS", v)?;
        }
        if let Some(v) = &self.total_completions {
            map.serialize_entry("TOTAL_COMPLETIONS", v)?;
        }
        if let Some(v) = &self.total_rejections {
            map.serialize_entry("TOTAL_REJECTIONS", v)?;
        }
        if let Some(v) = &self.workload_group_id {
            map.serialize_entry("WORKLOAD_GROUP_ID", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Cluster-level workload management statistics keyed by node ID."]
#[derive(Debug, Clone)]
pub struct WlmStatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request."]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[doc = "The name of the cluster."]
    pub cluster_name: Option<crate::types::common::Name>,
    pub additional: std::collections::BTreeMap<String, crate::types::wlm_common::NodeWlmStats>,
}
impl<'de> serde::Deserialize<'de> for WlmStatsResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WlmStatsResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WlmStatsResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_nodes: Option<crate::types::common::NodeStatistics> = None;
                let mut seen_nodes = false;
                let mut f_cluster_name: Option<crate::types::common::Name> = None;
                let mut seen_cluster_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::wlm_common::NodeWlmStats,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_nodes" => {
                            if seen_nodes {
                                return Err(serde::de::Error::duplicate_field("_nodes"));
                            }
                            seen_nodes = true;
                            f_nodes =
                                map.next_value::<Option<crate::types::common::NodeStatistics>>()?;
                        }
                        "cluster_name" => {
                            if seen_cluster_name {
                                return Err(serde::de::Error::duplicate_field("cluster_name"));
                            }
                            seen_cluster_name = true;
                            f_cluster_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WlmStatsResponse {
                    nodes: f_nodes,
                    cluster_name: f_cluster_name,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WlmStatsResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.nodes {
            map.serialize_entry("_nodes", v)?;
        }
        if let Some(v) = &self.cluster_name {
            map.serialize_entry("cluster_name", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Resource limits for a workload group. At least one of `memory` or `cpu` is required."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkloadGroupResourceLimits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkloadGroupResponse {
    #[doc = "The ID of the workload group, which can be used to associate query requests with the group and enforce the group's resource limits."]
    #[serde(rename = "_id")]
    pub id: String,
    #[doc = "The name of the workload group."]
    pub name: String,
    #[doc = "The resiliency mode of the workload group."]
    pub resiliency_mode: crate::types::wlm_common::ResiliencyMode,
    #[doc = "The resource limits for query requests in the workload group."]
    pub resource_limits: crate::types::wlm_common::WorkloadGroupResourceLimits,
    #[doc = "The settings for the workload group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::types::wlm_common::WorkloadGroupSettings>,
    #[doc = "The time at which the workload group was last updated."]
    pub updated_at: i64,
}

#[doc = "Search settings applied to requests in the workload group."]
pub type WorkloadGroupSettings = std::collections::BTreeMap<String, String>;

#[doc = "Workload group statistics collected on a single node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkloadGroupStats {
    #[doc = "CPU resource statistics for the workload group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::wlm_common::WlmResourceStats>,
    #[doc = "Memory resource statistics for the workload group."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::wlm_common::WlmResourceStats>,
    #[doc = "The total number of cancelled requests for the workload group on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_cancellations: Option<i64>,
    #[doc = "The total number of completed shard-level and coordinator-level requests for the workload group on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_completions: Option<i64>,
    #[doc = "The total number of rejected shard-level and coordinator-level requests for the workload group on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_rejections: Option<i64>,
    #[doc = "The total number of requests rejected because of a throttling limit for the workload group on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_throttled: Option<i64>,
    #[doc = "The total number of requests that a `monitor` mode workload group would have rejected on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_would_throttle: Option<i64>,
}
