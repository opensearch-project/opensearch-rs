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

//! Response types of the `indices_close` specification namespace

#![allow(missing_docs)]

#[doc = "The result of a close index operation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloseIndexResult {
    #[doc = "Whether the index was successfully closed."]
    pub closed: bool,
    #[doc = "The status of individual shards during the close operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards:
        Option<std::collections::BTreeMap<String, crate::types::indices_close::CloseShardResult>>,
}

#[doc = "The response of the `indices.close` API"]
pub type CloseResponse = crate::types::indices::CloseResponse;

#[doc = "The result of closing shards in an index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CloseShardResult {
    #[doc = "The list of failures that occurred while closing shards."]
    pub failures: Vec<crate::types::common::ShardFailure>,
}
