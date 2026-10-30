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

//! Response types of the `indices_simulate_template` specification namespace

#![allow(missing_docs)]

#[doc = "The configuration for overlapping index templates."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Overlapping {
    #[doc = "The list of index patterns that may overlap with other templates."]
    pub index_patterns: Vec<String>,
    pub name: crate::types::common::Name,
}

#[doc = "The response of the `indices.simulate_template` API"]
pub type SimulateTemplateResponse = crate::types::indices::SimulateTemplateResponse;

#[doc = "The configuration for an index template simulation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Template {
    #[doc = "The map of alias names to their configurations."]
    pub aliases: std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    pub settings: crate::types::indices_common::IndexSettings,
}
