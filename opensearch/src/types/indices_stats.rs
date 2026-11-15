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

//! Response types of the `indices_stats` specification namespace

#![allow(missing_docs)]

#[doc = "The statistics for all indexes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllIndicesStats {
    pub primaries: crate::types::indices_stats::IndexStats,
    pub total: crate::types::indices_stats::IndexStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexShardStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::common::CompletionStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<crate::types::common::DocStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::common::FielddataStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush: Option<crate::types::common::FlushStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::GetStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing: Option<crate::types::common::IndexingStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges: Option<crate::types::common::MergesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<crate::types::common::RecoveryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<crate::types::common::RefreshStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::common::SearchStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<crate::types::common::SegmentsStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::common::StoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog: Option<crate::types::common::TranslogStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer: Option<crate::types::common::WarmerStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::types::indices_stats::ShardCommit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_leases: Option<crate::types::indices_stats::ShardRetentionLeases>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::indices_stats::ShardRouting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::indices_stats::ShardSequenceNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_path: Option<crate::types::indices_stats::ShardPath>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::common::CompletionStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<crate::types::common::DocStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::common::FielddataStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush: Option<crate::types::common::FlushStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::GetStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing: Option<crate::types::common::IndexingStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges: Option<crate::types::common::MergesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<crate::types::common::RecoveryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<crate::types::common::RefreshStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::common::SearchStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<crate::types::common::SegmentsStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::common::StoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog: Option<crate::types::common::TranslogStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer: Option<crate::types::common::WarmerStats>,
}

#[doc = "The statistics for specific indexes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesStats {
    pub primaries: crate::types::indices_stats::IndexStats,
    #[doc = "The statistics for individual shards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<
        std::collections::BTreeMap<String, Vec<crate::types::indices_stats::IndexShardStats>>,
    >,
    pub total: crate::types::indices_stats::IndexStats,
    pub uuid: crate::types::common::Uuid,
}

#[doc = "The commit information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardCommit {
    #[doc = "The generation number of the commit."]
    pub generation: i32,
    pub id: crate::types::common::Id,
    #[doc = "The number of documents in the commit."]
    pub num_docs: i64,
    #[doc = "The user-defined data associated with the commit."]
    pub user_data: std::collections::BTreeMap<String, String>,
}

#[doc = "The size information for shard files."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardFileSizeInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "The number of files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The description of the file type."]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    pub size_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The retention lease details."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardLease {
    pub id: crate::types::common::Id,
    pub retaining_seq_no: crate::types::common::SequenceNumber,
    pub source: String,
    pub timestamp: i64,
}

#[doc = "The path information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardPath {
    #[doc = "The path to the shard's data."]
    pub data_path: String,
    #[doc = "Whether the shard uses a custom data path."]
    pub is_custom_data_path: bool,
    #[doc = "The path to the shard's state."]
    pub state_path: String,
}

#[doc = "The retention lease information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardRetentionLeases {
    #[doc = "The list of retention leases."]
    pub leases: Vec<crate::types::indices_stats::ShardLease>,
    #[doc = "The primary term of the retention leases."]
    pub primary_term: i64,
    pub version: crate::types::common::VersionNumber,
}

#[doc = "The routing information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardRouting {
    #[doc = "The node containing the shard."]
    pub node: String,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    #[doc = "The node where the shard is relocating to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relocating_node: Option<String>,
    pub state: crate::types::indices_stats::ShardRoutingState,
}

#[doc = "The state of shard routing."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardRoutingState {
    #[doc = "`INITIALIZING`"]
    Initializing,
    #[doc = "`RELOCATING`"]
    Relocating,
    #[doc = "`STARTED`"]
    Started,
    #[doc = "`UNASSIGNED`"]
    Unassigned,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardRoutingState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardRoutingState::Initializing => "INITIALIZING",
            ShardRoutingState::Relocating => "RELOCATING",
            ShardRoutingState::Started => "STARTED",
            ShardRoutingState::Unassigned => "UNASSIGNED",
            ShardRoutingState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardRoutingState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardRoutingState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "INITIALIZING" => ShardRoutingState::Initializing,
            "RELOCATING" => ShardRoutingState::Relocating,
            "STARTED" => ShardRoutingState::Started,
            "UNASSIGNED" => ShardRoutingState::Unassigned,
            _ => ShardRoutingState::Unknown(s),
        })
    }
}

#[doc = "The sequence number information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSequenceNumber {
    #[doc = "The global checkpoint for the shard."]
    pub global_checkpoint: i64,
    #[doc = "The local checkpoint for the shard."]
    pub local_checkpoint: i64,
    pub max_seq_no: crate::types::common::SequenceNumber,
}

#[doc = "The response of the `indices.stats` API"]
pub type StatsResponse = crate::types::indices::StatsResponse;
