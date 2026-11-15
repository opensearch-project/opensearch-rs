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

//! Response types of the `sm_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreationConfig {
    pub schedule: crate::types::sm_common::CronSchedule,
    #[doc = "The maximum amount of time for allowed snapshot creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CronExpression {
    #[doc = "The cron expression that defines the schedule."]
    pub expression: String,
    #[doc = "The timezone used for the cron schedule."]
    pub timezone: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CronSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<crate::types::sm_common::CronExpression>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeletionCondition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeletionConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<crate::types::sm_common::DeletionCondition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<crate::types::sm_common::CronSchedule>,
    #[doc = "The maximum amount of time allowed for snapshot deletion operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_limit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionInfo {
    #[doc = "The reason policy execution failed, if applicable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cause: Option<String>,
    #[doc = "A detailed message about the policy execution result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExecutionMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::types::sm_common::ExecutionInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainedPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation: Option<crate::types::sm_common::StateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<crate::types::sm_common::StateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_primary_term: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_seq_no: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetPoliciesResponse {
    #[doc = "A list of snapshot management policies."]
    pub policies: Vec<crate::types::sm_common::ListedPolicy>,
    #[doc = "The total number of policies found."]
    pub total_policies: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalConfig {
    #[doc = "The number of time units between executions."]
    pub period: i64,
    #[doc = "When the interval schedule should start."]
    pub start_time: i64,
    #[doc = "The time unit for the interval period (Days, Hours, or Minutes)."]
    pub unit: crate::types::sm_common::IntervalUnit,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntervalSchedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<crate::types::sm_common::IntervalConfig>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntervalUnit {
    #[doc = "`Days`"]
    Days,
    #[doc = "`Hours`"]
    Hours,
    #[doc = "`Minutes`"]
    Minutes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntervalUnit {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntervalUnit::Days => "Days",
            IntervalUnit::Hours => "Hours",
            IntervalUnit::Minutes => "Minutes",
            IntervalUnit::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntervalUnit {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntervalUnit {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Days" => IntervalUnit::Days,
            "Hours" => IntervalUnit::Hours,
            "Minutes" => IntervalUnit::Minutes,
            _ => IntervalUnit::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListedPolicy {
    #[doc = "The policy ID."]
    #[serde(rename = "_id")]
    pub id: String,
    #[doc = "The primary term for optimistic concurrency control."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[doc = "The sequence number for optimistic concurrency control."]
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<i64>,
    #[doc = "The complete snapshot management policy configuration."]
    pub sm_policy: crate::types::sm_common::SMPolicy,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationChannel {
    pub id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationConditions {
    #[doc = "Whether to send notifications for a successful snapshot creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation: Option<bool>,
    #[doc = "Whether to send notifications for a successful snapshot deletion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<bool>,
    #[doc = "Whether to send notifications for failed operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure: Option<bool>,
    #[doc = "Whether to send notifications when operations exceed time limits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_limit_exceeded: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<crate::types::sm_common::NotificationChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<crate::types::sm_common::NotificationConditions>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyExplanation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::types::sm_common::ExplainedPolicy>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyResponse {
    #[doc = "The policy ID."]
    #[serde(rename = "_id")]
    pub id: String,
    #[doc = "The primary term for optimistic concurrency control."]
    #[serde(rename = "_primary_term")]
    pub primary_term: i64,
    #[doc = "The sequence number for optimistic concurrency control."]
    #[serde(rename = "_seq_no")]
    pub seq_no: i64,
    #[doc = "The version number of the policy document."]
    #[serde(rename = "_version")]
    pub version: i64,
    #[doc = "The complete snapshot management policy configuration."]
    pub sm_policy: crate::types::sm_common::SMPolicy,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetryMetadata {
    #[doc = "The number of retry attempts made."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SMPolicy {
    pub creation: crate::types::sm_common::CreationConfig,
    #[doc = "The configuration for snapshot deletion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletion: Option<crate::types::sm_common::DeletionConfig>,
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "When the policy was last enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_time: Option<i64>,
    #[doc = "When the policy was last modified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The unique identifier for the snapshot policy."]
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<crate::types::sm_common::NotificationConfig>,
    #[doc = "The system-generated schedule metadata returned by the API."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<crate::types::sm_common::IntervalSchedule>,
    #[doc = "The version number of the policy schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    pub snapshot_config: crate::types::sm_common::SnapshotConfig,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotConfig {
    #[doc = "Default \"yyyy-MM-dd'T'HH:mm:ss\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_format: Option<String>,
    #[doc = "Default false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_unavailable: Option<bool>,
    #[doc = "Default true"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_global_state: Option<bool>,
    #[doc = "Default \"*\""]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "Default false"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial: Option<bool>,
    pub repository: String,
    #[doc = "Default UTC"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateMetadata {
    #[doc = "The current state of the policy execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[doc = "Details about the most recent policy execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_execution: Option<crate::types::sm_common::ExecutionMetadata>,
    #[doc = "Information about any retry attempts that occurred."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<crate::types::sm_common::RetryMetadata>,
    #[doc = "Any information about what triggered the policy execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<crate::types::sm_common::TriggerMetadata>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TriggerMetadata {
    #[doc = "When the policy was triggered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
