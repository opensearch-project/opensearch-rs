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

//! Response types of the `cluster_reroute` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerouteDecision {
    pub decider: String,
    pub decision: String,
    pub explanation: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerouteExplanation {
    pub command: String,
    pub decisions: Vec<crate::types::cluster_reroute::RerouteDecision>,
    pub parameters: crate::types::cluster_reroute::RerouteParameters,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerouteParameters {
    pub allow_primary: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from_node: Option<crate::types::common::NodeName>,
    pub index: crate::types::common::IndexName,
    pub node: crate::types::common::NodeName,
    pub shard: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to_node: Option<crate::types::common::NodeName>,
}

#[doc = "The response of the `cluster.reroute` API"]
pub type RerouteResponse = crate::types::cluster::RerouteResponse;
