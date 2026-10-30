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

//! Response types of the `reindex_rethrottle` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReindexNode {
    #[doc = "The attributes of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::common::Ip>,
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<crate::types::common::NodeRoles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub tasks: std::collections::BTreeMap<String, crate::types::reindex_rethrottle::ReindexTask>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReindexRethrottleResponse {
    pub nodes: std::collections::BTreeMap<String, crate::types::reindex_rethrottle::ReindexNode>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReindexStatus {
    #[doc = "The number of scroll responses shown by the reindex."]
    pub batches: i64,
    #[doc = "The number of documents that were successfully created."]
    pub created: i64,
    #[doc = "The number of documents that were successfully deleted."]
    pub deleted: i64,
    #[doc = "The number of documents that were ignored because the script used for the reindex operation returned a `noop` value for `ctx.op`."]
    pub noops: i64,
    #[doc = "The number of successful requests per second during the reindex operation."]
    pub requests_per_second: f64,
    pub retries: crate::types::common::Retries,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled: Option<crate::types::common::Duration>,
    pub throttled_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_until: Option<crate::types::common::Duration>,
    pub throttled_until_millis: crate::types::common::DurationValueUnitMillis,
    #[doc = "The number of documents that were successfully processed."]
    pub total: i64,
    #[doc = "The number of documents that were successfully updated."]
    pub updated: i64,
    #[doc = "The number of version conflicts that reindex hits."]
    pub version_conflicts: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReindexTask {
    pub action: String,
    pub cancellable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    pub description: String,
    pub headers: crate::types::common::HttpHeaders,
    pub id: i64,
    pub node: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    pub status: crate::types::reindex_rethrottle::ReindexStatus,
    pub r#type: String,
}
