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

//! Response types of the `asynchronous_search_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsynchronousSearchStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initialized: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persist_failed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persisted: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running_current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_completed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_failed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submitted: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodesStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub asynchronous_search_stats:
        Option<crate::types::asynchronous_search_common::AsynchronousSearchStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseBody {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time_in_millis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<crate::types::search::SearchResponse>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_in_millis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub took: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsResponse {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::asynchronous_search_common::NodesStats>,
    >,
}
