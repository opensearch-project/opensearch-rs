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

//! Response types of the `termvectors` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldStatistics {
    pub doc_count: i32,
    pub sum_doc_freq: i64,
    pub sum_ttf: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Term {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_freq: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    pub term_freq: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<crate::types::termvectors::Token>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttf: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermVector {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_statistics: Option<crate::types::termvectors::FieldStatistics>,
    pub terms: std::collections::BTreeMap<String, crate::types::termvectors::Term>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermvectorsResponse {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::Type>,
    #[serde(rename = "_version")]
    pub version: crate::types::common::VersionNumber,
    pub found: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vectors:
        Option<std::collections::BTreeMap<String, crate::types::termvectors::TermVector>>,
    pub took: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Token {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_offset: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub payload: Option<String>,
    pub position: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_offset: Option<i32>,
}
