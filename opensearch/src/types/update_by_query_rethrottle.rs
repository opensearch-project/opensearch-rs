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

//! Response types of the `update_by_query_rethrottle` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateByQueryRethrottleNode {
    #[doc = "The attributes of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::common::Ip>,
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<crate::types::common::NodeRoles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub tasks: std::collections::BTreeMap<String, crate::types::tasks_common::TaskInfo>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateByQueryRethrottleResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_failures: Option<Vec<crate::types::common::ErrorCause>>,
    pub nodes: std::collections::BTreeMap<
        String,
        crate::types::update_by_query_rethrottle::UpdateByQueryRethrottleNode,
    >,
}
