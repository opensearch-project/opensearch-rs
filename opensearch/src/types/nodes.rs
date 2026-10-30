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

//! Response types of the `nodes` specification namespace

#![allow(missing_docs)]

#[doc = "The response containing node information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct InfoResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    #[serde(rename = "nodes")]
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_info::NodeInfo>,
}

#[doc = "The response containing secure settings reload results."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReloadSecureSettingsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    #[serde(rename = "nodes")]
    pub _nodes: std::collections::BTreeMap<
        String,
        crate::types::nodes_reload_secure_settings::NodeReloadResult,
    >,
}

#[doc = "The response containing node statistics."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<crate::types::common::Name>,
    #[serde(rename = "nodes")]
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_stats::Stats>,
}

#[doc = "The response containing node usage information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    pub cluster_name: crate::types::common::Name,
    #[serde(rename = "nodes")]
    pub _nodes: std::collections::BTreeMap<String, crate::types::nodes_usage::NodeUsage>,
}
