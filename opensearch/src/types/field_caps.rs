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

//! Response types of the `field_caps` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldCapability {
    #[doc = "Whether this field can be aggregated on all indexes."]
    pub aggregatable: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<crate::types::common::Indices>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[doc = "Whether this field is registered as a metadata field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_field: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_aggregatable_indices: Option<crate::types::common::Indices>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_searchable_indices: Option<crate::types::common::Indices>,
    #[doc = "Whether this field is indexed for search on all indexes."]
    pub searchable: bool,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldCapsResponse {
    pub fields: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, crate::types::field_caps::FieldCapability>,
    >,
    pub indices: crate::types::common::Indices,
}
