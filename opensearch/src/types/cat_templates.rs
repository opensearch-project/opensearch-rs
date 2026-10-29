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

//! Response types of the `cat_templates` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplatesRecord {
    #[doc = "The component templates that comprise the index template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composed_of: Option<String>,
    #[doc = "The template index patterns."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_patterns: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[doc = "The template application order or priority number."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[doc = "The template version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[doc = "The response of the `cat.templates` API"]
pub type TemplatesResponse = crate::types::cat::TemplatesResponse;
