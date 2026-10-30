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

//! Response types of the `security_analytics_alerts` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionExecutionResult {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Alert {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_execution_results:
        Option<Vec<crate::types::security_analytics_alerts::ActionExecutionResult>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_history: Option<Vec<crate::types::security_analytics_alerts::AlertError>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detector_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finding_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_notification_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_doc_ids: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::types::security_analytics_alerts::AlertState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AlertError {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AlertState {
    #[doc = "`ACKNOWLEDGED`"]
    Acknowledged,
    #[doc = "`ACTIVE`"]
    Active,
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`DELETED`"]
    Deleted,
    #[doc = "`ERROR`"]
    Error,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AlertState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AlertState::Acknowledged => "ACKNOWLEDGED",
            AlertState::Active => "ACTIVE",
            AlertState::Completed => "COMPLETED",
            AlertState::Deleted => "DELETED",
            AlertState::Error => "ERROR",
            AlertState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AlertState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AlertState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ACKNOWLEDGED" => AlertState::Acknowledged,
            "ACTIVE" => AlertState::Active,
            "COMPLETED" => AlertState::Completed,
            "DELETED" => AlertState::Deleted,
            "ERROR" => AlertState::Error,
            _ => AlertState::Unknown(s),
        })
    }
}

pub type Alerts = Vec<crate::types::security_analytics_alerts::Alert>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAlertsResponse {
    pub alerts: crate::types::security_analytics_alerts::Alerts,
    pub total_alerts: i64,
}
