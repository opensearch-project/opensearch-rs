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

//! Response types of the `replication_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoFollowStats {
    #[doc = "The list of indices that failed to replicate for this pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_indices: Option<Vec<String>>,
    #[doc = "When the last execution of this auto-follow pattern occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<f64>,
    #[doc = "The name of the auto-follow pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The number of failed calls to the leader cluster for this pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_leader_calls: Option<f64>,
    #[doc = "The number of failed replication starts for this pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_start_replication: Option<f64>,
    #[doc = "The number of successful replication starts for this pattern."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_success_start_replication: Option<f64>,
    #[doc = "The pattern used for auto-following indices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoFollowStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub autofollow_stats: Option<Vec<crate::types::replication_common::AutoFollowStats>>,
    #[doc = "The list of indices that failed to replicate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_indices: Option<Vec<String>>,
    #[doc = "The number of failed calls to the leader cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_leader_calls: Option<f64>,
    #[doc = "The number of failed replication starts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_start_replication: Option<f64>,
    #[doc = "The number of successful replication starts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_success_start_replication: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FollowerStatus {
    #[doc = "The number of failed read requests during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_read_requests: Option<f64>,
    #[doc = "The number of failed write requests during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_write_requests: Option<f64>,
    #[doc = "The current checkpoint of the follower index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_checkpoint: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_stats: Option<
        std::collections::BTreeMap<String, crate::types::replication_common::IndexFollowerStatus>,
    >,
    #[doc = "The current checkpoint of the leader index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_checkpoint: Option<f64>,
    #[doc = "The number of indexes currently bootstrapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_bootstrapping_indices: Option<f64>,
    #[doc = "The number of indexes that have failed replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_failed_indices: Option<f64>,
    #[doc = "The number of active index-level replication tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_index_tasks: Option<f64>,
    #[doc = "The number of indexes currently paused."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_paused_indices: Option<f64>,
    #[doc = "The number of active shard-level replication tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_shard_tasks: Option<f64>,
    #[doc = "The number of indexes currently syncing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_syncing_indices: Option<f64>,
    #[doc = "The total number of operations read during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read: Option<f64>,
    #[doc = "The total number of operations written during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_written: Option<f64>,
    #[doc = "The number of throttled read requests during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_read_requests: Option<f64>,
    #[doc = "The number of throttled write requests during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_write_requests: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_write_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexFollowerStatus {
    #[doc = "The number of failed read requests for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_read_requests: Option<f64>,
    #[doc = "The number of failed write requests for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_write_requests: Option<f64>,
    #[doc = "The current checkpoint of the follower index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_checkpoint: Option<f64>,
    #[doc = "The current checkpoint of the leader index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_checkpoint: Option<f64>,
    #[doc = "The number of operations read for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations written for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_written: Option<f64>,
    #[doc = "The number of throttled read requests for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_read_requests: Option<f64>,
    #[doc = "The number of throttled write requests for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_write_requests: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_write_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<crate::types::common::ByteCount>,
    #[doc = "The number of operations read for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations read from Lucene for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read_lucene: Option<f64>,
    #[doc = "The number of operations read from the translog for this index during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read_translog: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_read_time_lucene_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_read_time_translog_millis:
        Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog_size_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LeaderStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_read: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_stats:
        Option<std::collections::BTreeMap<String, crate::types::replication_common::IndexStatus>>,
    #[doc = "The number of indexes being replicated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_replicated_indices: Option<f64>,
    #[doc = "The total number of operations read during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read: Option<f64>,
    #[doc = "The number of operations read from Lucene during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read_lucene: Option<f64>,
    #[doc = "The number of operations read from the translog during replication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations_read_translog: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_read_time_lucene_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_read_time_translog_millis:
        Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog_size_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplicationStatus {
    #[doc = "`BOOTSTRAPPING`"]
    Bootstrapping,
    #[doc = "`PAUSED`"]
    Paused,
    #[doc = "`REPLICATION NOT IN PROGRESS`"]
    ReplicationNotInProgress,
    #[doc = "`RUNNING`"]
    Running,
    #[doc = "`SYNCING`"]
    Syncing,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReplicationStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReplicationStatus::Bootstrapping => "BOOTSTRAPPING",
            ReplicationStatus::Paused => "PAUSED",
            ReplicationStatus::ReplicationNotInProgress => "REPLICATION NOT IN PROGRESS",
            ReplicationStatus::Running => "RUNNING",
            ReplicationStatus::Syncing => "SYNCING",
            ReplicationStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReplicationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReplicationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BOOTSTRAPPING" => ReplicationStatus::Bootstrapping,
            "PAUSED" => ReplicationStatus::Paused,
            "REPLICATION NOT IN PROGRESS" => ReplicationStatus::ReplicationNotInProgress,
            "RUNNING" => ReplicationStatus::Running,
            "SYNCING" => ReplicationStatus::Syncing,
            _ => ReplicationStatus::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Status {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_index: Option<crate::types::common::IndexName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_alias: Option<crate::types::common::IndexAlias>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_index: Option<crate::types::common::IndexName>,
    #[doc = "The reason for the current replication status."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The current status of the replication process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::replication_common::ReplicationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub syncing_details: Option<crate::types::replication_common::SyncingDetails>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SyncingDetails {
    #[doc = "The checkpoint of the follower index in the replication process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_checkpoint: Option<i64>,
    #[doc = "The checkpoint of the leader index in the replication process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leader_checkpoint: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
}
