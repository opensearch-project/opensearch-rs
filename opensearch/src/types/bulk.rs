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

//! Response types of the `bulk` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BulkResponse {
    pub errors: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest_took: Option<i64>,
    pub items: Vec<crate::types::bulk::Item>,
    pub took: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Item {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create: Option<crate::types::bulk::ResponseItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<crate::types::bulk::ResponseItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::bulk::ResponseItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<crate::types::bulk::ResponseItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseItem {
    #[doc = "The document ID associated with the operation."]
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The name of the index associated with the operation. If the operation targets a data stream, this is the backing index into which the document was written."]
    #[serde(rename = "_index")]
    pub index: String,
    #[doc = "The primary term assigned to the document for the operation."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardInfo>,
    #[doc = "The document type."]
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::types::common::ErrorCause>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub forced_refresh: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::InlineGetDictUserDefined>,
    #[doc = "Result of the operation. Successful values are `created`, `deleted`, and `updated`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
    #[doc = "HTTP status code returned for the operation."]
    pub status: i32,
}
