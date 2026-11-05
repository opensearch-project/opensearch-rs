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

//! Response types of the `indices` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AddBlockResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub indices: Vec<crate::types::indices_add_block::IndicesBlockStatus>,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyzeResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<crate::types::indices_analyze::AnalyzeDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<crate::types::indices_analyze::AnalyzeToken>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloneResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloseResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub indices: std::collections::BTreeMap<String, crate::types::indices_close::CloseIndexResult>,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateResponse {
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamsStatsResponse {
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "Total number of backing indexes for the selected data streams."]
    pub backing_indices: i32,
    #[doc = "Total number of selected data streams."]
    pub data_stream_count: i32,
    #[doc = "Contains statistics for the selected data streams."]
    pub data_streams: Vec<crate::types::indices_common::DataStreamStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_store_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all shards for the selected data streams."]
    pub total_store_size_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForcemergeResponse {
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "Task contains a task id returned when `wait_for_completion=false`, you can use the `task_id` to get the status of the task at _tasks/<task_id>."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<String>,
}

pub type GetAliasResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_alias::IndexAliases>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetDataStreamResponse {
    pub data_streams: Vec<crate::types::indices_common::DataStream>,
}

pub type GetFieldMappingResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_field_mapping::TypeFieldMappings>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetIndexTemplateResponse {
    pub index_templates: Vec<crate::types::indices_get_index_template::IndexTemplateItem>,
}

pub type GetMappingResponse =
    std::collections::BTreeMap<String, crate::types::indices_get_mapping::IndexMappingRecord>;

pub type GetResponse = std::collections::BTreeMap<String, crate::types::indices_common::IndexState>;

pub type GetSettingsResponse =
    std::collections::BTreeMap<String, crate::types::indices_common::IndexState>;

pub type GetTemplateResponse =
    std::collections::BTreeMap<String, crate::types::indices_common::TemplateMapping>;

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct OpenResponse {
    raw: Box<serde_json::value::RawValue>,
}
impl OpenResponse {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::indices::OpenResponseObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::indices::OpenResponseObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenResponseObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<crate::types::common::TaskId>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OpenResponseObjectObject {
    pub acknowledged: bool,
    pub shards_acknowledged: bool,
}

pub type RecoveryResponse =
    std::collections::BTreeMap<String, crate::types::indices_recovery::RecoveryStatus>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolveIndexResponse {
    pub aliases: Vec<crate::types::indices_resolve_index::ResolveIndexAliasItem>,
    pub data_streams: Vec<crate::types::indices_resolve_index::ResolveIndexDataStreamsItem>,
    pub indices: Vec<crate::types::indices_resolve_index::ResolveIndexItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RolloverResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub conditions: std::collections::BTreeMap<String, bool>,
    pub dry_run: bool,
    pub new_index: String,
    pub old_index: String,
    pub rolled_over: bool,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentsResponse {
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    pub indices: std::collections::BTreeMap<String, crate::types::indices_segments::IndexSegment>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardStoresResponse {
    pub indices:
        std::collections::BTreeMap<String, crate::types::indices_shard_stores::IndicesShardStores>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShrinkResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulateIndexTemplateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlapping: Option<Vec<crate::types::indices_simulate_template::Overlapping>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<crate::types::indices_simulate_template::Template>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulateTemplateResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overlapping: Option<Vec<crate::types::indices_simulate_template::Overlapping>>,
    pub template: crate::types::indices_simulate_template::Template,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitResponse {
    #[doc = "For a successful response, this value is always true. On failure, an exception is returned instead."]
    pub acknowledged: bool,
    pub index: crate::types::common::IndexName,
    pub shards_acknowledged: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsResponse {
    #[serde(rename = "_all")]
    pub all: crate::types::indices_stats::AllIndicesStats,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_stats::IndicesStats>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpgradeResponse {
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgraded_indices: Option<
        std::collections::BTreeMap<String, crate::types::indices_common::UpgradeVersionStatus>,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidateQueryResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanations:
        Option<Vec<crate::types::indices_validate_query::IndicesValidationExplanation>>,
    pub valid: bool,
}
