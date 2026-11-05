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

//! Response types of the `cat_cluster_manager` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterManagerRecord {
    #[doc = "hostname"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "node id"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "IP address"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "node name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
}

#[doc = "The response of the `cat.cluster_manager` API"]
pub type ClusterManagerResponse = crate::types::cat::ClusterManagerResponse;
