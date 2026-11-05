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

//! Response types of the `snapshot_restore` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `snapshot.restore` API"]
pub type RestoreResponse = crate::types::snapshot::RestoreResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnapshotRestore {
    #[doc = "The list of indexes that were restored."]
    pub indices: Vec<crate::types::common::IndexName>,
    #[doc = "Any statistics about the restored shards."]
    pub shards: crate::types::common::ShardStatistics,
    #[doc = "The name of the snapshot that was restored."]
    pub snapshot: String,
}
