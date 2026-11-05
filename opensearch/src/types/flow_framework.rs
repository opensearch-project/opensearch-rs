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

//! Response types of the `flow_framework` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateResponse {
    #[doc = "The ID of the workflow."]
    pub workflow_id: String,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GetStatusResponse {
    raw: Box<serde_json::value::RawValue>,
}
impl GetStatusResponse {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `work_flow_status_full_response` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_work_flow_status_full_response(
        &self,
    ) -> std::result::Result<
        crate::types::flow_framework_common::WorkFlowStatusFullResponse,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "work_flow_status_full_response")
    }
    #[doc = "Parses the value as the `work_flow_status_default_response` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_work_flow_status_default_response(
        &self,
    ) -> std::result::Result<
        crate::types::flow_framework_common::WorkFlowStatusDefaultResponse,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "work_flow_status_default_response")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateResponse {
    #[doc = "The ID of the workflow to be updated. Required for the PUT method."]
    pub workflow_id: String,
}
