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

//! Response types of the `snapshot` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CleanupRepositoryResponse {
    pub results: crate::types::snapshot_cleanup_repository::CleanupRepositoryResults,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateResponse {
    #[doc = "Returns `true` if the snapshot was accepted. Present when the request had `wait_for_completion` set to `false`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::snapshot_common::SnapshotInfo>,
}

pub type GetRepositoryResponse =
    std::collections::BTreeMap<String, crate::types::snapshot_common::Repository>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetResponse {
    pub snapshots: Vec<crate::types::snapshot_common::SnapshotInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RestoreResponse {
    #[doc = "Returns `true` if the restore was accepted. Present when the request had `wait_for_completion` set to `false`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::snapshot_restore::SnapshotRestore>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatusResponse {
    pub snapshots: Vec<crate::types::snapshot_common::SnapshotStatus>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VerifyRepositoryResponse {
    pub nodes: std::collections::BTreeMap<
        String,
        crate::types::snapshot_verify_repository::CompactNodeInfo,
    >,
}
