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

//! Response types of the `get_script_context` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Context {
    pub methods: Vec<crate::types::get_script_context::ContextMethod>,
    pub name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextMethod {
    pub name: crate::types::common::Name,
    pub params: Vec<crate::types::get_script_context::ContextMethodParam>,
    pub return_type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContextMethodParam {
    pub name: crate::types::common::Name,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetScriptContextResponse {
    pub contexts: Vec<crate::types::get_script_context::Context>,
}
