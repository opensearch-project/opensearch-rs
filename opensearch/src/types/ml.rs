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

//! Response types of the `ml` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddAgenticMemoryResponse {
    #[doc = "Session ID associated with the memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "The ID for the creating working memory entry."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_memory_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChunkModelResponse {
    #[doc = "The status of the chunk upload operation."]
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateConnectorResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateControllerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::ml_common::Status>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMemoryContainerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_container_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMemoryContainerSessionResponse {
    #[doc = "The ID of the created session."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[doc = "Satus of the creation operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMemoryResponse {
    pub memory_id: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateMessageResponse {
    pub message_id: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateModelMetaResponse {
    pub model_id: String,
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteMemoryResponse {
    #[doc = "Whether the memory was deleted successfully."]
    pub success: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeployModelResponse {
    pub status: String,
    pub task_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAllMemoriesResponse {
    pub memories: Vec<crate::types::ml_common::Memory>,
    #[doc = "The index of the next memory after the last memory in the returned list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_token: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetAllMessagesResponse {
    pub messages: Vec<crate::types::ml_common::Message>,
    #[doc = "The index of the next message after the last message in the returned list."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_token: Option<i32>,
}

pub type GetAllToolsResponse = Vec<crate::types::ml_common::Tool>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetControllerResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_rate_limiter:
        Option<std::collections::BTreeMap<String, crate::types::ml_common::RateLimiter>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetMessageTracesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_token: Option<i32>,
    pub traces: Vec<crate::types::ml_common::Message>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LoadModelResponse {
    pub status: String,
    pub task_id: String,
    pub task_type: crate::types::ml_common::MlTaskType,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterAgentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterModelMetaResponse {
    pub model_id: String,
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RegisterModelResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    pub status: String,
    pub task_id: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadChunkResponse {
    #[doc = "The status of the chunk upload operation."]
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadModelResponse {
    pub status: String,
    pub task_id: String,
}
