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

//! Response types of the `indices_recovery` specification namespace

#![allow(missing_docs)]

#[doc = "The details of a recovered file."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileDetails {
    #[doc = "The length of the file in bytes."]
    pub length: i64,
    #[doc = "The name of the file."]
    pub name: String,
    #[doc = "The number of bytes recovered."]
    pub recovered: i64,
}

#[doc = "The byte-level recovery statistics."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryBytes {
    pub percent: crate::types::common::PercentageString,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovered: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovered_from_snapshot: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovered_from_snapshot_in_bytes: Option<crate::types::common::ByteCount>,
    pub recovered_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reused: Option<crate::types::common::HumanReadableByteCount>,
    pub reused_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    pub total_in_bytes: crate::types::common::ByteCount,
}

#[doc = "The file-level recovery statistics."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryFiles {
    #[doc = "The detailed information about recovered files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::types::indices_recovery::FileDetails>>,
    pub percent: crate::types::common::PercentageString,
    #[doc = "The number of recovered files."]
    pub recovered: i64,
    #[doc = "The number of reused files."]
    pub reused: i64,
    #[doc = "The total number of files."]
    pub total: i64,
}

#[doc = "The index recovery status."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryIndexStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<crate::types::indices_recovery::RecoveryBytes>,
    pub files: crate::types::indices_recovery::RecoveryFiles,
    pub size: crate::types::indices_recovery::RecoveryBytes,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_throttle_time: Option<crate::types::common::Duration>,
    pub source_throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_throttle_time: Option<crate::types::common::Duration>,
    pub target_throttle_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The source or target of a recovery operation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryOrigin {
    #[doc = "Whether a new history UUID was bootstrapped."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bootstrap_new_history_uuid: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[doc = "The hostname of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::common::Ip>,
    #[doc = "Whether this is a searchable snapshot."]
    #[serde(rename = "isSearchableSnapshot")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_searchable_snapshot: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[doc = "Whether this is a shallow copy from a remote store."]
    #[serde(rename = "remoteStoreIndexShallowCopy")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_store_index_shallow_copy: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<crate::types::common::Name>,
    #[serde(rename = "restoreUUID")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restore_uuid: Option<crate::types::common::Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::common::Name>,
    #[doc = "The source remote store repository."]
    #[serde(rename = "sourceRemoteStoreRepository")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_remote_store_repository: Option<String>,
    #[doc = "The source remote translog repository."]
    #[serde(rename = "sourceRemoteTranslogRepository")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_remote_translog_repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_address: Option<crate::types::common::TransportAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[doc = "The response of the `indices.recovery` API"]
pub type RecoveryResponse = crate::types::indices::RecoveryResponse;

#[doc = "The status at the start of recovery."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryStartStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_index_time: Option<crate::types::common::Duration>,
    pub check_index_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The status of recovery operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryStatus {
    #[doc = "The list of shard recovery statuses."]
    pub shards: Vec<crate::types::indices_recovery::ShardRecovery>,
}

#[doc = "The recovery status of a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardRecovery {
    #[doc = "The shard ID."]
    pub id: i32,
    pub index: crate::types::indices_recovery::RecoveryIndexStatus,
    #[doc = "Whether this is a primary shard."]
    pub primary: bool,
    pub source: crate::types::indices_recovery::RecoveryOrigin,
    #[doc = "The current recovery stage."]
    pub stage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<crate::types::indices_recovery::RecoveryStartStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::DateTime>,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    pub target: crate::types::indices_recovery::RecoveryOrigin,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub translog: crate::types::indices_recovery::TranslogStatus,
    #[doc = "The type of recovery operation."]
    pub r#type: String,
    pub verify_index: crate::types::indices_recovery::VerifyIndex,
}

#[doc = "The transaction log recovery status."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslogStatus {
    pub percent: crate::types::common::PercentageString,
    #[doc = "The number of recovered transaction log operations."]
    pub recovered: i32,
    #[doc = "The total number of transaction log operations."]
    pub total: i32,
    #[doc = "The number of transaction log operations at start."]
    pub total_on_start: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The index verification status."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerifyIndex {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub check_index_time: Option<crate::types::common::Duration>,
    pub check_index_time_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time: Option<crate::types::common::Duration>,
    pub total_time_in_millis: crate::types::common::DurationValueUnitMillis,
}
