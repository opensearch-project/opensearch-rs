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

//! Response types of the `indices_resolve_index` specification namespace

#![allow(missing_docs)]

#[doc = "The resolved information for an index alias."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolveIndexAliasItem {
    pub indices: crate::types::common::Indices,
    pub name: crate::types::common::Name,
}

#[doc = "The resolved information for a data stream."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolveIndexDataStreamsItem {
    pub backing_indices: crate::types::common::Indices,
    pub name: crate::types::common::DataStreamName,
    pub timestamp_field: crate::types::common::Field,
}

#[doc = "The resolved information for an index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResolveIndexItem {
    #[doc = "The list of aliases associated with the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[doc = "The list of index attributes."]
    pub attributes: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_stream: Option<crate::types::common::DataStreamName>,
    pub name: crate::types::common::Name,
}

#[doc = "The response of the `indices.resolve_index` API"]
pub type ResolveIndexResponse = crate::types::indices::ResolveIndexResponse;
