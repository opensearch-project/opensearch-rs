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

//! Response types of the `indices_get_field_mapping` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `indices.get_field_mapping` API"]
pub type GetFieldMappingResponse = crate::types::indices::GetFieldMappingResponse;

#[doc = "The field mapping configuration for a type."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeFieldMappings {
    #[doc = "The map of field names to their mapping definitions."]
    pub mappings: std::collections::BTreeMap<String, crate::types::common_mapping::FieldMapping>,
}
