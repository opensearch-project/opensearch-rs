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

//! Response types of the `update` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `update` API"]
pub type UpdateResponse = crate::types::update::UpdateWriteResponseBase;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateWriteResponseBase {
    #[serde(rename = "_id")]
    pub id: crate::types::common::Id,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    pub primary_term: i64,
    #[serde(rename = "_seq_no")]
    pub seq_no: crate::types::common::SequenceNumber,
    #[serde(rename = "_shards")]
    pub shards: crate::types::common::ShardStatistics,
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::Type>,
    #[serde(rename = "_version")]
    pub version: crate::types::common::VersionNumber,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forced_refresh: Option<bool>,
    pub result: crate::types::common::Result,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::InlineGet>,
}
