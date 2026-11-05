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

//! Response types of the `snapshot_cleanup_repository` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `snapshot.cleanup_repository` API"]
pub type CleanupRepositoryResponse = crate::types::snapshot::CleanupRepositoryResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CleanupRepositoryResults {
    #[doc = "The number of binary large objects (blobs) removed during cleanup."]
    pub deleted_blobs: i64,
    #[doc = "The number of bytes freed by cleanup operations."]
    pub deleted_bytes: i64,
}
