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

//! Response types of the `query_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Credentials {
    #[doc = "The username for authentication."]
    pub password: String,
    #[doc = "The username for authentication."]
    pub username: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSource {
    #[serde(rename = "allowedRoles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<crate::types::query_common::DataSourceConfiguration>,
    #[doc = "The connector type for the data source."]
    pub connector: String,
    #[doc = "The description of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the data source."]
    pub name: String,
    #[doc = "The configuration properties for the data source."]
    pub properties: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The index where query results are stored."]
    #[serde(rename = "resultIndex")]
    pub result_index: String,
    #[doc = "The current status of the data source."]
    pub status: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSourceConfiguration {
    pub credentials: crate::types::query_common::Credentials,
    #[doc = "The connection endpoint for the data source."]
    pub endpoint: String,
}

pub type DataSourceList = Vec<crate::types::query_common::DataSource>;

#[doc = "Represents the fundamental reason for an error."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSourceRetrieve {
    #[doc = "The roles allowed to access this data source."]
    #[serde(rename = "allowedRoles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub configuration: Option<crate::types::query_common::DataSourceConfiguration>,
    #[doc = "The connector type for the data source."]
    pub connector: String,
    #[doc = "The description of the data source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The name of the data source."]
    pub name: String,
    #[doc = "The configuration properties for the data source."]
    pub properties: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The index where query results are stored."]
    #[serde(rename = "resultIndex")]
    pub result_index: String,
    #[doc = "The current status of the data source."]
    pub status: String,
}
