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

//! Response types of the `snapshot_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileCountSnapshotStats {
    #[doc = "The number of files in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i32>,
    #[doc = "The total size of files in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
}

#[doc = "The name of the repository to store the snapshot."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Repository {
    #[doc = "The configuration settings for the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::types::snapshot_common::RepositorySettings>,
    #[doc = "The type of the snapshot repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[doc = "The unique identifier for the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<crate::types::common::Uuid>,
}

#[doc = "The settings for the snapshot repository."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepositorySettings {
    #[doc = "The chunk size for the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunk_size: Option<String>,
    #[doc = "Whether compression is enabled for the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "The number of concurrent streams for the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_streams: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The location where snapshots are stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[doc = "Whether the repository is read-only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<crate::types::common::StringifiedBoolean>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotIndexStats {
    #[doc = "The status of individual shards in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<
        std::collections::BTreeMap<String, crate::types::snapshot_common::SnapshotShardsStatus>,
    >,
    #[doc = "The statistics about snapshot shards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats>,
    #[doc = "The detailed statistics about the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::snapshot_common::SnapshotStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotInfo {
    #[doc = "The list of data streams included in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_streams: Option<Vec<String>>,
    #[doc = "The total time taken to complete the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<crate::types::common::Duration>,
    #[doc = "The total time taken to complete the snapshot in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The time when the snapshot completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<crate::types::common::DateTime>,
    #[doc = "The time when the snapshot completed in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The list of shard failures that occurred during the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::snapshot_common::SnapshotShardFailure>>,
    #[doc = "Whether the snapshot includes the cluster state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_global_state: Option<bool>,
    #[doc = "The list of indexes included in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<crate::types::common::IndexName>>,
    #[doc = "The custom metadata attached to the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::types::common::Metadata>,
    #[doc = "The timestamp when the snapshot was pinned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pinned_timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The reason for the snapshot creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "Whether the snapshot uses remote store index shallow copy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_store_index_shallow_copy: Option<bool>,
    #[doc = "The statistics about the shards included in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "The name of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::common::Name>,
    #[doc = "The time when the snapshot started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::DateTime>,
    #[doc = "The time when the snapshot started in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The current state of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The unique identifier for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<crate::types::common::Uuid>,
    #[doc = "The version of OpenSearch when the snapshot was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[doc = "The internal version number of OpenSearch when the snapshot was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_id: Option<crate::types::common::VersionNumber>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotShardFailure {
    #[doc = "The name of the index containing the failed shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The ID of the node where the shard failure occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<crate::types::common::Id>,
    #[doc = "The reason for the shard failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The ID of the failed shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<crate::types::common::Id>,
    #[doc = "The status of the failed shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotShardsStats {
    #[doc = "The number of completed shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<i64>,
    #[doc = "The number of failed shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[doc = "The number of finalizing shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalizing: Option<i64>,
    #[doc = "The number of initializing shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initializing: Option<i64>,
    #[doc = "The number of started shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started: Option<i64>,
    #[doc = "The total number of shard snapshots."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnapshotShardsStatsStage {
    #[doc = "`DONE`"]
    Done,
    #[doc = "`FAILURE`"]
    Failure,
    #[doc = "`FINALIZE`"]
    Finalize,
    #[doc = "`INIT`"]
    Init,
    #[doc = "`STARTED`"]
    Started,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnapshotShardsStatsStage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnapshotShardsStatsStage::Done => "DONE",
            SnapshotShardsStatsStage::Failure => "FAILURE",
            SnapshotShardsStatsStage::Finalize => "FINALIZE",
            SnapshotShardsStatsStage::Init => "INIT",
            SnapshotShardsStatsStage::Started => "STARTED",
            SnapshotShardsStatsStage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnapshotShardsStatsStage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnapshotShardsStatsStage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DONE" => SnapshotShardsStatsStage::Done,
            "FAILURE" => SnapshotShardsStatsStage::Failure,
            "FINALIZE" => SnapshotShardsStatsStage::Finalize,
            "INIT" => SnapshotShardsStatsStage::Init,
            "STARTED" => SnapshotShardsStatsStage::Started,
            _ => SnapshotShardsStatsStage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotShardsStatsSummary {
    #[doc = "The incremental statistics for the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
    #[doc = "The processed statistics for the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
    #[doc = "When the shard snapshot started in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The total time taken for the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time taken for the shard snapshot in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total statistics for the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::snapshot_common::SnapshotShardsStatsSummaryItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotShardsStatsSummaryItem {
    #[doc = "The number of files in the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i64>,
    #[doc = "The total size of files in the shard snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotShardsStatus {
    #[doc = "The ID of the node containing the shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<crate::types::common::NodeId>,
    #[doc = "The reason for the current shard status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The current stage of the shard snapshot."]
    pub stage: crate::types::snapshot_common::SnapshotShardsStatsStage,
    #[doc = "The statistical summary of the shard snapshot."]
    pub stats: crate::types::snapshot_common::SnapshotShardsStatsSummary,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotStats {
    #[doc = "The incremental statistics for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incremental: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
    #[doc = "The processed statistics for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processed: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
    #[doc = "When the snapshot started in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The total time taken for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The total time taken for the snapshot in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total statistics for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::snapshot_common::FileCountSnapshotStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotStatus {
    #[doc = "Whether the snapshot includes the cluster state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_global_state: Option<bool>,
    #[doc = "The status of indexes in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<
        std::collections::BTreeMap<String, crate::types::snapshot_common::SnapshotIndexStats>,
    >,
    #[doc = "The name of the repository containing the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The statistics about snapshot shards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards_stats: Option<crate::types::snapshot_common::SnapshotShardsStats>,
    #[doc = "The name of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[doc = "The current state of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The detailed statistics about the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::snapshot_common::SnapshotStats>,
    #[doc = "The unique identifier for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<crate::types::common::Uuid>,
}
