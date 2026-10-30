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

//! Response types of the `cat_segment_replication` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentReplicationRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_behind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_fetched: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_percent: Option<crate::types::common::PercentageString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_total: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoints_behind: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_lag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_diff_stage_time_taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_fetched: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_percent: Option<crate::types::common::PercentageString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_total: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalize_replication_stage_time_taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_checkpoint_info_stage_time_taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_files_stage_time_taken: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_completed_lag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected_requests: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicating_stage_time_taken: Option<String>,
    #[serde(rename = "shardId")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_node: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
}

#[doc = "The response of the `cat.segment_replication` API"]
pub type SegmentReplicationResponse = crate::types::cat::SegmentReplicationResponse;
