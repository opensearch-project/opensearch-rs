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

//! Response types of the `ingestion_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ErrorPolicy {
    #[doc = "`BLOCK`"]
    Block,
    #[doc = "`DROP`"]
    Drop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ErrorPolicy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ErrorPolicy::Block => "BLOCK",
            ErrorPolicy::Drop => "DROP",
            ErrorPolicy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ErrorPolicy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ErrorPolicy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BLOCK" => ErrorPolicy::Block,
            "DROP" => ErrorPolicy::Drop,
            _ => ErrorPolicy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetIngestionStateResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "Ingestion state for a given index and its shards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingestion_state: Option<
        std::collections::BTreeMap<
            String,
            Vec<crate::types::ingestion_common::ShardIngestionState>,
        >,
    >,
    #[doc = "Specifies the next page token. Pass this token in the request to retrieve contents of the next page."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestionStateShardFailure {
    #[doc = "Indicates the shard level error message."]
    pub error: String,
    #[doc = "The shard number."]
    pub shard: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PauseIngestionResponse {
    #[doc = "Indicates if the pause request has been acknowledged."]
    pub acknowledged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::ingestion_common::IngestionStateShardFailure>>,
    #[doc = "Indicates if the pause request has been acknowledged by individual shards."]
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PollerState {
    #[doc = "`CLOSED`"]
    Closed,
    #[doc = "`NONE`"]
    None,
    #[doc = "`PAUSED`"]
    Paused,
    #[doc = "`POLLING`"]
    Polling,
    #[doc = "`PROCESSING`"]
    Processing,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PollerState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PollerState::Closed => "CLOSED",
            PollerState::None => "NONE",
            PollerState::Paused => "PAUSED",
            PollerState::Polling => "POLLING",
            PollerState::Processing => "PROCESSING",
            PollerState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PollerState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PollerState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "CLOSED" => PollerState::Closed,
            "NONE" => PollerState::None,
            "PAUSED" => PollerState::Paused,
            "POLLING" => PollerState::Polling,
            "PROCESSING" => PollerState::Processing,
            _ => PollerState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResumeIngestionResponse {
    #[doc = "Indicates if the resume request has been acknowledged."]
    pub acknowledged: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<Vec<crate::types::ingestion_common::IngestionStateShardFailure>>,
    #[doc = "Indicates if the resume request has been acknowledged by individual shards."]
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIngestionState {
    #[doc = "Indicates the shard pointer from where the poller will resume during shard recovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_start_pointer: Option<String>,
    #[doc = "Indicates the policy used to handle failed messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_policy: Option<crate::types::ingestion_common::ErrorPolicy>,
    #[doc = "Indicates if the poller has been paused."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poller_paused: Option<bool>,
    #[doc = "Indicates the current state of the poller."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poller_state: Option<crate::types::ingestion_common::PollerState>,
    #[doc = "The shard number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<i64>,
    #[doc = "Indicates if a write block has been created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_block_enabled: Option<bool>,
}
