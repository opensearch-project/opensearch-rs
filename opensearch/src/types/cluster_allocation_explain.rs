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

//! Response types of the `cluster_allocation_explain` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationDecision {
    pub decider: String,
    pub decision: crate::types::cluster_allocation_explain::AllocationExplainDecision,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AllocationExplainDecision {
    #[doc = "`ALWAYS`"]
    Always,
    #[doc = "`NO`"]
    No,
    #[doc = "`THROTTLE`"]
    Throttle,
    #[doc = "`YES`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AllocationExplainDecision {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AllocationExplainDecision::Always => "ALWAYS",
            AllocationExplainDecision::No => "NO",
            AllocationExplainDecision::Throttle => "THROTTLE",
            AllocationExplainDecision::Yes => "YES",
            AllocationExplainDecision::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AllocationExplainDecision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AllocationExplainDecision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ALWAYS" => AllocationExplainDecision::Always,
            "NO" => AllocationExplainDecision::No,
            "THROTTLE" => AllocationExplainDecision::Throttle,
            "YES" => AllocationExplainDecision::Yes,
            _ => AllocationExplainDecision::Unknown(s),
        })
    }
}

#[doc = "The response of the `cluster.allocation_explain` API"]
pub type AllocationExplainResponse = crate::types::cluster::AllocationExplainResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationStore {
    pub allocation_id: String,
    pub found: bool,
    pub in_sync: bool,
    pub matching_size_in_bytes: crate::types::common::ByteCount,
    pub matching_sync_id: bool,
    pub store_exception: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterInfo {
    pub nodes:
        std::collections::BTreeMap<String, crate::types::cluster_allocation_explain::NodeDiskUsage>,
    pub reserved_sizes: Vec<crate::types::cluster_allocation_explain::ReservedSize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_data_set_sizes: Option<std::collections::BTreeMap<String, String>>,
    pub shard_paths: std::collections::BTreeMap<String, String>,
    pub shard_sizes: std::collections::BTreeMap<
        String,
        crate::types::cluster_allocation_explain::ClusterInfoShardSizesValue,
    >,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum ClusterInfoShardSizesValue {
    ByteCount(crate::types::common::ByteCount),
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
}
impl<'de> serde::Deserialize<'de> for ClusterInfoShardSizesValue {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(ClusterInfoShardSizesValue::ByteCount)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(ClusterInfoShardSizesValue::HumanReadableByteCount)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "ClusterInfoShardSizesValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CurrentNode {
    pub attributes: std::collections::BTreeMap<String, String>,
    pub id: crate::types::common::Id,
    pub name: crate::types::common::Name,
    pub transport_address: crate::types::common::TransportAddress,
    pub weight_ranking: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decision {
    #[doc = "`allocation_delayed`"]
    AllocationDelayed,
    #[doc = "`awaiting_info`"]
    AwaitingInfo,
    #[doc = "`no`"]
    No,
    #[doc = "`no_attempt`"]
    NoAttempt,
    #[doc = "`no_valid_shard_copy`"]
    NoValidShardCopy,
    #[doc = "`throttled`"]
    Throttled,
    #[doc = "`worse_balance`"]
    WorseBalance,
    #[doc = "`yes`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Decision {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Decision::AllocationDelayed => "allocation_delayed",
            Decision::AwaitingInfo => "awaiting_info",
            Decision::No => "no",
            Decision::NoAttempt => "no_attempt",
            Decision::NoValidShardCopy => "no_valid_shard_copy",
            Decision::Throttled => "throttled",
            Decision::WorseBalance => "worse_balance",
            Decision::Yes => "yes",
            Decision::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Decision {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Decision {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "allocation_delayed" => Decision::AllocationDelayed,
            "awaiting_info" => Decision::AwaitingInfo,
            "no" => Decision::No,
            "no_attempt" => Decision::NoAttempt,
            "no_valid_shard_copy" => Decision::NoValidShardCopy,
            "throttled" => Decision::Throttled,
            "worse_balance" => Decision::WorseBalance,
            "yes" => Decision::Yes,
            _ => Decision::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiskUsage {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    pub free_bytes: crate::types::common::ByteCount,
    pub free_disk_percent: crate::types::common::PercentageNumber,
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    pub total_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    pub used_bytes: crate::types::common::ByteCount,
    pub used_disk_percent: crate::types::common::PercentageNumber,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeAllocationExplanation {
    pub deciders: Vec<crate::types::cluster_allocation_explain::AllocationDecision>,
    pub node_attributes: std::collections::BTreeMap<String, String>,
    pub node_decision: crate::types::cluster_allocation_explain::Decision,
    pub node_id: crate::types::common::Id,
    pub node_name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::cluster_allocation_explain::AllocationStore>,
    pub transport_address: crate::types::common::TransportAddress,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight_ranking: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeDiskUsage {
    pub least_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub most_available: crate::types::cluster_allocation_explain::DiskUsage,
    pub node_name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReservedSize {
    pub node_id: crate::types::common::Id,
    pub path: String,
    pub shards: Vec<String>,
    pub total: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnassignedInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation_status: Option<String>,
    pub at: crate::types::common::DateTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delayed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_allocation_attempts: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_allocation_status: Option<String>,
    pub reason: crate::types::cluster_allocation_explain::UnassignedInformationReason,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnassignedInformationReason {
    #[doc = "`ALLOCATION_FAILED`"]
    AllocationFailed,
    #[doc = "`CLUSTER_RECOVERED`"]
    ClusterRecovered,
    #[doc = "`DANGLING_INDEX_IMPORTED`"]
    DanglingIndexImported,
    #[doc = "`EXISTING_INDEX_RESTORED`"]
    ExistingIndexRestored,
    #[doc = "`FORCED_EMPTY_PRIMARY`"]
    ForcedEmptyPrimary,
    #[doc = "`INDEX_CREATED`"]
    IndexCreated,
    #[doc = "`INDEX_REOPENED`"]
    IndexReopened,
    #[doc = "`MANUAL_ALLOCATION`"]
    ManualAllocation,
    #[doc = "`NEW_INDEX_RESTORED`"]
    NewIndexRestored,
    #[doc = "`NODE_LEFT`"]
    NodeLeft,
    #[doc = "`PRIMARY_FAILED`"]
    PrimaryFailed,
    #[doc = "`REALLOCATED_REPLICA`"]
    ReallocatedReplica,
    #[doc = "`REINITIALIZED`"]
    Reinitialized,
    #[doc = "`REPLICA_ADDED`"]
    ReplicaAdded,
    #[doc = "`REROUTE_CANCELLED`"]
    RerouteCancelled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UnassignedInformationReason {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UnassignedInformationReason::AllocationFailed => "ALLOCATION_FAILED",
            UnassignedInformationReason::ClusterRecovered => "CLUSTER_RECOVERED",
            UnassignedInformationReason::DanglingIndexImported => "DANGLING_INDEX_IMPORTED",
            UnassignedInformationReason::ExistingIndexRestored => "EXISTING_INDEX_RESTORED",
            UnassignedInformationReason::ForcedEmptyPrimary => "FORCED_EMPTY_PRIMARY",
            UnassignedInformationReason::IndexCreated => "INDEX_CREATED",
            UnassignedInformationReason::IndexReopened => "INDEX_REOPENED",
            UnassignedInformationReason::ManualAllocation => "MANUAL_ALLOCATION",
            UnassignedInformationReason::NewIndexRestored => "NEW_INDEX_RESTORED",
            UnassignedInformationReason::NodeLeft => "NODE_LEFT",
            UnassignedInformationReason::PrimaryFailed => "PRIMARY_FAILED",
            UnassignedInformationReason::ReallocatedReplica => "REALLOCATED_REPLICA",
            UnassignedInformationReason::Reinitialized => "REINITIALIZED",
            UnassignedInformationReason::ReplicaAdded => "REPLICA_ADDED",
            UnassignedInformationReason::RerouteCancelled => "REROUTE_CANCELLED",
            UnassignedInformationReason::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UnassignedInformationReason {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UnassignedInformationReason {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ALLOCATION_FAILED" => UnassignedInformationReason::AllocationFailed,
            "CLUSTER_RECOVERED" => UnassignedInformationReason::ClusterRecovered,
            "DANGLING_INDEX_IMPORTED" => UnassignedInformationReason::DanglingIndexImported,
            "EXISTING_INDEX_RESTORED" => UnassignedInformationReason::ExistingIndexRestored,
            "FORCED_EMPTY_PRIMARY" => UnassignedInformationReason::ForcedEmptyPrimary,
            "INDEX_CREATED" => UnassignedInformationReason::IndexCreated,
            "INDEX_REOPENED" => UnassignedInformationReason::IndexReopened,
            "MANUAL_ALLOCATION" => UnassignedInformationReason::ManualAllocation,
            "NEW_INDEX_RESTORED" => UnassignedInformationReason::NewIndexRestored,
            "NODE_LEFT" => UnassignedInformationReason::NodeLeft,
            "PRIMARY_FAILED" => UnassignedInformationReason::PrimaryFailed,
            "REALLOCATED_REPLICA" => UnassignedInformationReason::ReallocatedReplica,
            "REINITIALIZED" => UnassignedInformationReason::Reinitialized,
            "REPLICA_ADDED" => UnassignedInformationReason::ReplicaAdded,
            "REROUTE_CANCELLED" => UnassignedInformationReason::RerouteCancelled,
            _ => UnassignedInformationReason::Unknown(s),
        })
    }
}
