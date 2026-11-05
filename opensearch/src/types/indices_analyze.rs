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

//! Response types of the `indices_analyze` specification namespace

#![allow(missing_docs)]

#[doc = "The detailed analysis results."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyzeDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<crate::types::indices_analyze::AnalyzerDetail>,
    #[doc = "The list of character filter results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub charfilters: Option<Vec<crate::types::indices_analyze::CharFilterDetail>>,
    #[doc = "Whether a custom analyzer was used."]
    pub custom_analyzer: bool,
    #[doc = "The list of token filter results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenfilters: Option<Vec<crate::types::indices_analyze::TokenDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<crate::types::indices_analyze::TokenDetail>,
}

#[doc = "The response of the `indices.analyze` API"]
pub type AnalyzeResponse = crate::types::indices::AnalyzeResponse;

#[doc = "The basic token information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyzeToken {
    #[doc = "The ending character offset of the token."]
    pub end_offset: i32,
    #[doc = "The position of the token."]
    pub position: i32,
    #[doc = "The length of the token position."]
    #[serde(rename = "positionLength")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_length: Option<i32>,
    #[doc = "The starting character offset of the token."]
    pub start_offset: i32,
    #[doc = "The token string."]
    pub token: String,
    #[doc = "The type of the token."]
    pub r#type: String,
}

#[doc = "The analyzer results."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalyzerDetail {
    #[doc = "The name of the analyzer."]
    pub name: String,
    #[doc = "The list of tokens produced by the analyzer."]
    pub tokens: Vec<crate::types::indices_analyze::ExplainAnalyzeToken>,
}

#[doc = "The character filter results."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharFilterDetail {
    #[doc = "The text after character filtering."]
    pub filtered_text: Vec<String>,
    #[doc = "The name of the character filter."]
    pub name: String,
}

#[doc = "The detailed token information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainAnalyzeToken {
    #[doc = "The bytes representation of the token."]
    pub bytes: String,
    #[doc = "The ending character offset of the token."]
    pub end_offset: i32,
    #[doc = "Whether the token is a keyword."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keyword: Option<bool>,
    #[doc = "The position of the token."]
    pub position: i32,
    #[doc = "The length of the token position."]
    #[serde(rename = "positionLength")]
    pub position_length: i32,
    #[doc = "The starting character offset of the token."]
    pub start_offset: i32,
    #[doc = "The frequency of the term."]
    #[serde(rename = "termFrequency")]
    pub term_frequency: i32,
    #[doc = "The token string."]
    pub token: String,
    #[doc = "The type of the token."]
    pub r#type: String,
}

#[doc = "The token filter or tokenizer results."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenDetail {
    #[doc = "The name of the token filter or tokenizer."]
    pub name: String,
    #[doc = "The list of tokens produced."]
    pub tokens: Vec<crate::types::indices_analyze::ExplainAnalyzeToken>,
}
