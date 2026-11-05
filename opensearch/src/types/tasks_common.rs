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

//! Response types of the `tasks_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistentTaskStatus {
    pub state: String,
}

pub type RawTaskStatus = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReplicationTaskStatus {
    pub phase: String,
}

#[doc = "Task status information can vary wildly from task to task."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Status {
    raw: Box<serde_json::value::RawValue>,
}
impl Status {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `replication_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_replication_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::ReplicationTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "replication_task_status")
    }
    #[doc = "Parses the value as the `bulk_by_scroll_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_bulk_by_scroll_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::common::BulkByScrollTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "bulk_by_scroll_task_status")
    }
    #[doc = "Parses the value as the `persistent_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_persistent_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::PersistentTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "persistent_task_status")
    }
    #[doc = "Parses the value as the `raw_task_status` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_raw_task_status(
        &self,
    ) -> std::result::Result<
        crate::types::tasks_common::RawTaskStatus,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "raw_task_status")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskExecutingNode {
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
    pub tasks: std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskGroup {
    pub action: String,
    pub cancellable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_time_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub headers: std::collections::BTreeMap<String, String>,
    pub id: i64,
    pub node: crate::types::common::NodeId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<crate::types::common::TaskId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running_time: Option<crate::types::common::Duration>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::tasks_common::Status>,
    pub r#type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::tasks_common::TaskGroup>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskInfo {
    pub action: String,
    pub cancellable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_time_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub headers: std::collections::BTreeMap<String, String>,
    pub id: i64,
    pub node: crate::types::common::NodeId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_task_id: Option<crate::types::common::TaskId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_stats: Option<crate::types::common::ResourceStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running_time: Option<crate::types::common::Duration>,
    pub running_time_in_nanos: crate::types::common::DurationValueUnitNanos,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::tasks_common::Status>,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TaskInfos {
    GroupedByNone(Vec<crate::types::tasks_common::TaskInfo>),
    GroupedByParents(std::collections::BTreeMap<String, crate::types::tasks_common::TaskGroup>),
}
impl<'de> serde::Deserialize<'de> for TaskInfos {
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
            Some(b'[') => serde_json::from_str(text).map(TaskInfos::GroupedByNone),
            Some(b'{') => serde_json::from_str(text).map(TaskInfos::GroupedByParents),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TaskInfos"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TaskListResponseBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_failures: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "Task information grouped by node, if `group_by` was set to `node` (the default)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes:
        Option<std::collections::BTreeMap<String, crate::types::tasks_common::TaskExecutingNode>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_failures: Option<Vec<crate::types::common::TaskFailure>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tasks: Option<crate::types::tasks_common::TaskInfos>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum TaskResponse {
    BulkByScrollResponseBase(crate::types::common::BulkByScrollResponseBase),
}
impl<'de> serde::Deserialize<'de> for TaskResponse {
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
            Some(b'{') => serde_json::from_str(text).map(TaskResponse::BulkByScrollResponseBase),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "TaskResponse"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}
