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

//! Response types of the `ltr_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcknowledgedResponse {
    #[doc = "Indicates whether the request was acknowledged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[doc = "The name of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "Indicates whether the required active shards were acknowledged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards_acknowledged: Option<bool>,
}

#[doc = "Aggregate cache statistics across all nodes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheAllStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featuresets: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::ltr_common::CacheItemStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheItemStats {
    #[doc = "Count of cached items."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Memory usage in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ram: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eviction_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_usage_in_bytes: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature: Option<crate::types::ltr_common::CacheStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featureset: Option<crate::types::ltr_common::CacheStat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<crate::types::ltr_common::CacheStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStatsResponse {
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<crate::types::ltr_common::CacheAllStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<crate::types::common::Name>,
    #[doc = "Cache statistics per node."]
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::ltr_common::NodeDetails>>,
    #[doc = "Cache statistics by store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stores: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListStoresResponse {
    #[doc = "Map of available feature stores."]
    pub stores: std::collections::BTreeMap<String, crate::types::ltr_common::StoreDetails>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeDetails {
    #[doc = "Node hostname."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[doc = "Node name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::ltr_common::NodeStatsDetails>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<crate::types::ltr_common::CacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_error_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_total_count: Option<i64>,
}

#[doc = "Cache statistics for a node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeStatsDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub features: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featuresets: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<crate::types::ltr_common::CacheItemStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::ltr_common::CacheItemStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<crate::types::common::Name>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<std::collections::BTreeMap<String, crate::types::ltr_common::NodeStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stores: Option<crate::types::ltr_common::StoreStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoreDetails {
    #[doc = "Count statistics for this store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub counts: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The index name for this store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The name of the store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[doc = "Store version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoreExistsResponse {
    #[doc = "Indicates whether the store exists."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exists: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StoreStat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feature_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featureset_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

pub type StoreStats = std::collections::BTreeMap<String, crate::types::ltr_common::StoreStat>;
