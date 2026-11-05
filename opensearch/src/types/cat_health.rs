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

//! Response types of the `cat_health` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthRecord {
    #[doc = "active number of shards in percent"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_shards_percent: Option<crate::types::common::PercentageString>,
    #[doc = "cluster name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster: Option<String>,
    #[doc = "cluster manager is discovered or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_cluster_manager: Option<String>,
    #[doc = "cluster manager is discovered or not"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovered_master: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    #[doc = "number of initializing nodes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<String>,
    #[doc = "wait time of longest task pending"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_task_wait_time: Option<String>,
    #[doc = "number of nodes that can store data"]
    #[serde(rename = "node.data")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_data: Option<String>,
    #[doc = "total number of nodes"]
    #[serde(rename = "node.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_total: Option<String>,
    #[doc = "number of pending tasks"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending_tasks: Option<String>,
    #[doc = "number of primary shards"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[doc = "number of relocating nodes"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relo: Option<String>,
    #[doc = "total number of shards"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<String>,
    #[doc = "health status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::TimeOfDay>,
    #[doc = "number of unassigned shards"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassign: Option<String>,
}

#[doc = "The response of the `cat.health` API"]
pub type HealthResponse = crate::types::cat::HealthResponse;
