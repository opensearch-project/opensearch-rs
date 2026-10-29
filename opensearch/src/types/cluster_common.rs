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

//! Response types of the `cluster_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentTemplate {
    pub component_template: crate::types::cluster_common::ComponentTemplateNode,
    pub name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentTemplateNode {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub template: crate::types::cluster_common::ComponentTemplateSummary,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComponentTemplateSummary {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::AliasDefinition>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::IndexSettings>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}
