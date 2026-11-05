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

//! Response types of the `indices_validate_query` specification namespace

#![allow(missing_docs)]

#[doc = "The validation results for an index query."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesValidationExplanation {
    #[doc = "The error message if validation failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "The explanation of why the query is valid or invalid."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
    pub index: crate::types::common::IndexName,
    #[doc = "Whether the query is valid for the index."]
    pub valid: bool,
}

#[doc = "The response of the `indices.validate_query` API"]
pub type ValidateQueryResponse = crate::types::indices::ValidateQueryResponse;
