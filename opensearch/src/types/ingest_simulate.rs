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

//! Response types of the `ingest_simulate` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DocumentSimulation {
    #[serde(rename = "_id")]
    pub id: crate::types::common::Id,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
    #[serde(rename = "_ingest")]
    pub ingest: crate::types::ingest_simulate::Ingest,
    #[doc = "Value used to send the document to a specific primary shard."]
    #[serde(rename = "_routing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
    #[doc = "JSON body for the document."]
    #[serde(rename = "_source")]
    pub source: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::StringifiedVersionNumber>,
    #[serde(rename = "_version_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_type: Option<crate::types::common::VersionType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ingest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<crate::types::common::Name>,
    pub timestamp: crate::types::common::DateTime,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineSimulation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc: Option<crate::types::ingest_simulate::DocumentSimulation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_results: Option<Vec<crate::types::ingest_simulate::PipelineSimulation>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processor_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::common::ActionStatusOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[doc = "The response of the `ingest.simulate` API"]
pub type SimulateResponse = crate::types::ingest::SimulateResponse;
