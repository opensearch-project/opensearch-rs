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

//! Response types of the `cluster_health` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AwarenessAttributeStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_shards: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_nodes: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initializing_shards: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocating_shards: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_shards: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

#[doc = "The response of the `cluster.health` API"]
pub type HealthResponse = crate::types::cluster_health::HealthResponseBody;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthResponseBody {
    #[doc = "The number of active primary shards."]
    pub active_primary_shards: i32,
    #[doc = "The total number of active primary and replica shards."]
    pub active_shards: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_shards_percent: Option<crate::types::common::PercentageString>,
    pub active_shards_percent_as_number: crate::types::common::PercentageNumber,
    #[doc = "Cluster health information for each awareness attribute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub awareness_attributes: Option<
        std::collections::BTreeMap<String, crate::types::cluster_health::AwarenessAttributeStats>,
    >,
    pub cluster_name: crate::types::common::Name,
    #[doc = "The number of shards whose allocation has been delayed by the timeout settings."]
    pub delayed_unassigned_shards: i32,
    #[doc = "True if the cluster-manager node has been discovered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_cluster_manager: Option<bool>,
    #[doc = "True if the cluster-manager node has been discovered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_master: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::cluster_health::IndexHealthStats>>,
    #[doc = "The number of shards that are under initialization."]
    pub initializing_shards: i32,
    #[doc = "The number of nodes that are dedicated data nodes."]
    pub number_of_data_nodes: i32,
    #[doc = "The number of unfinished fetches."]
    pub number_of_in_flight_fetch: i32,
    #[doc = "The number of nodes within the cluster."]
    pub number_of_nodes: i32,
    #[doc = "The number of cluster-level changes that have not yet been executed."]
    pub number_of_pending_tasks: i32,
    #[doc = "The number of shards that are under relocation."]
    pub relocating_shards: i32,
    pub status: crate::types::common::HealthStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_max_waiting_in_queue: Option<crate::types::common::Duration>,
    pub task_max_waiting_in_queue_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "If `false` the response returned within the period of time that is specified by the timeout parameter (`30s` by default)."]
    pub timed_out: bool,
    #[doc = "The number of shards that are not allocated."]
    pub unassigned_shards: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexHealthStats {
    pub active_primary_shards: i32,
    pub active_shards: i32,
    pub initializing_shards: i32,
    pub number_of_replicas: i32,
    pub number_of_shards: i32,
    pub relocating_shards: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards:
        Option<std::collections::BTreeMap<String, crate::types::cluster_health::ShardHealthStats>>,
    pub status: crate::types::common::HealthStatus,
    pub unassigned_shards: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardHealthStats {
    pub active_shards: i32,
    pub initializing_shards: i32,
    pub primary_active: bool,
    pub relocating_shards: i32,
    pub status: crate::types::common::HealthStatus,
    pub unassigned_shards: i32,
}
