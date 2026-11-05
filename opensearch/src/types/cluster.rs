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

//! Response types of the `cluster` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationExplainResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocate_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_delay: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_allocate: Option<crate::types::cluster_allocation_explain::Decision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_move_to_other_node: Option<crate::types::cluster_allocation_explain::Decision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_rebalance_cluster: Option<crate::types::cluster_allocation_explain::Decision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_rebalance_cluster_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::AllocationDecision>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_rebalance_to_other_node: Option<crate::types::cluster_allocation_explain::Decision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_remain_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::AllocationDecision>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub can_remain_on_current_node: Option<crate::types::cluster_allocation_explain::Decision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_info: Option<crate::types::cluster_allocation_explain::ClusterInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configured_delay: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configured_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_node: Option<crate::types::cluster_allocation_explain::CurrentNode>,
    pub current_state: String,
    pub index: crate::types::common::IndexName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub move_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_allocation_decisions:
        Option<Vec<crate::types::cluster_allocation_explain::NodeAllocationExplanation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebalance_explanation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_delay: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_delay_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    pub shard: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_info: Option<crate::types::cluster_allocation_explain::UnassignedInformation>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetComponentTemplateResponse {
    pub component_templates: Vec<crate::types::cluster_common::ComponentTemplate>,
}

pub type GetDecommissionAwarenessResponse = std::collections::BTreeMap<
    String,
    crate::types::cluster_decommission_awareness::DecommissionStatus,
>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetSettingsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    pub persistent: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub transient: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetWeightedRoutingResponse {
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_cluster_manager: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingTasksResponse {
    pub tasks: Vec<crate::types::cluster_pending_tasks::PendingTask>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutSettingsResponse {
    pub acknowledged: bool,
    pub persistent: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    pub transient: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}

pub type RemoteInfoResponse =
    std::collections::BTreeMap<String, crate::types::cluster_remote_info::ClusterRemoteInfo>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerouteResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanations: Option<Vec<crate::types::cluster_reroute::RerouteExplanation>>,
    #[doc = "Shows the internal representation of the structure, which can differ from the external representation.                  "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<Box<serde_json::value::RawValue>>,
}

pub type StateResponse = Box<serde_json::value::RawValue>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[doc = "The name of the cluster."]
    pub cluster_name: crate::types::common::Name,
    #[doc = "The unique identifier of the cluster."]
    pub cluster_uuid: crate::types::common::Uuid,
    #[doc = "Displays statistics about indexes in the cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<crate::types::cluster_stats::ClusterIndices>,
    #[doc = "Displays statistics about nodes selected by the request's node filters."]
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<crate::types::cluster_stats::ClusterNodes>,
    #[doc = "The health status of the cluster, based on the state of its primary and replica shards."]
    pub status: crate::types::common::HealthStatus,
    #[doc = "The Unix timestamp, in milliseconds, indicating when any cluster statistics were last refreshed."]
    pub timestamp: crate::types::common::EpochTimeUnitMillis,
}
