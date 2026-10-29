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

//! Response types of the `cat_snapshots` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotsRecord {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<crate::types::common::TimeOfDay>,
    #[doc = "The number of failed shards in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_shards: Option<String>,
    #[doc = "The unique identifier for the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The number of indexes in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<String>,
    #[doc = "The reason for any snapshot failures."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[doc = "The repository name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_epoch: Option<crate::types::common::StringifiedEpochTimeUnitSeconds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::TimeOfDay>,
    #[doc = "The state of the snapshot process. Returned values include: `FAILED`: The snapshot process failed. `INCOMPATIBLE`: The snapshot process is incompatible with the current cluster version. `IN_PROGRESS`: The snapshot process started but has not completed. `PARTIAL`: The snapshot process completed with a partial success. `SUCCESS`: The snapshot process completed with a full success."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "The number of successful shards in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_shards: Option<String>,
    #[doc = "The total number of shards in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_shards: Option<String>,
}

#[doc = "The response of the `cat.snapshots` API"]
pub type SnapshotsResponse = crate::types::cat::SnapshotsResponse;
