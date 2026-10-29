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

//! Response types of the `cat_recovery` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecoveryRecord {
    #[doc = "The number of bytes to recover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The bytes recovered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_recovered: Option<String>,
    #[doc = "The total number of bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes_total: Option<String>,
    #[doc = "The number of files to recover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The files recovered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_recovered: Option<String>,
    #[doc = "The total number of files."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub files_total: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The shard name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[doc = "The snapshot name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
    #[doc = "The source host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_host: Option<String>,
    #[doc = "The source node name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_node: Option<String>,
    #[doc = "The recovery stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_time_millis: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "The target host."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_host: Option<String>,
    #[doc = "The target node name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_node: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[doc = "The number of translog operations to recover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog_ops: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog_ops_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The translog operations recovered."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog_ops_recovered: Option<String>,
    #[doc = "The recovery type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[doc = "The response of the `cat.recovery` API"]
pub type RecoveryResponse = crate::types::cat::RecoveryResponse;
