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

//! Response types of the `flow_framework_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlowFrameworkGetResponse {
    #[doc = "When the workflow was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<i64>,
    #[doc = "The description of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "When the workflow was last provisioned."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_provisioned_time: Option<f64>,
    #[doc = "When the workflow was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The name of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "A use case ID, which can be used with the Search Workflow API to find related workflows."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub use_case: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::types::flow_framework_common::User>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::flow_framework_common::Version>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflows: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Hits {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<crate::types::flow_framework_common::ItemsObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::flow_framework_common::Total>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ItemsObject {
    #[doc = "The document ID."]
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The index name where the document resides."]
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[doc = "The score of the search hit."]
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[doc = "The sequence number of the document."]
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<i64>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::flow_framework_common::FlowFrameworkGetResponse>,
    #[doc = "The version of the document."]
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourcesCreated {
    #[doc = "The unique identifier for the created resource."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[doc = "The type of resource created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<String>,
    #[doc = "The unique identifier for the workflow step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_step_id: Option<String>,
    #[doc = "The name of the workflow step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_step_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchStateResponse {
    #[doc = "When the provisioning operation completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_end_time: Option<crate::types::common::DateTime>,
    #[doc = "When the provisioning operation started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_start_time: Option<crate::types::common::DateTime>,
    #[doc = "The current progress status of the provisioning operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning_progress: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources_created:
        Option<crate::types::flow_framework_common::SearchStateResponseResourcesCreated>,
    #[doc = "The current state of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::types::flow_framework_common::User>,
    #[doc = "The ID of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SearchStateResponseResourcesCreated {
    ResourcesCreated(crate::types::flow_framework_common::ResourcesCreated),
    Array(Vec<crate::types::flow_framework_common::ResourcesCreated>),
}
impl<'de> serde::Deserialize<'de> for SearchStateResponseResourcesCreated {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'{') => serde_json::from_str(text)
                .map(SearchStateResponseResourcesCreated::ResourcesCreated),
            Some(b'[') => {
                serde_json::from_str(text).map(SearchStateResponseResourcesCreated::Array)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SearchStateResponseResourcesCreated"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Shards {
    #[doc = "The number of shards where the deletion failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[doc = "The number of shards where the deletion was skipped."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skipped: Option<i64>,
    #[doc = "The number of shards where the deletion was successful."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful: Option<i64>,
    #[doc = "The total number of shards involved in the deletion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateHits {
    #[doc = "The list of search hits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<Vec<crate::types::flow_framework_common::StateItems>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_score: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::flow_framework_common::Total>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StateItems {
    #[doc = "The document ID."]
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The index name where the document resides."]
    #[serde(rename = "_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[doc = "The score of the search hit."]
    #[serde(rename = "_score")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<f64>,
    #[doc = "The sequence number of the document."]
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<i64>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::flow_framework_common::SearchStateResponse>,
    #[doc = "The version of the document."]
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Total {
    #[doc = "The relation of the total hits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relation: Option<String>,
    #[doc = "The total number of matching documents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[doc = "The list of backend roles assigned to the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The list of custom attributes associated with the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_names: Option<Vec<String>>,
    #[doc = "The name of the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of roles assigned to the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[doc = "The tenant requested by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_requested_tenant: Option<String>,
}

#[doc = "A key-value map with two fields, template, which identifies the template version, and compatibility, which identifies a list of minimum required OpenSearch versions."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Version {
    #[doc = "A list of minimum required OpenSearch versions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatibility: Option<Vec<String>>,
    #[doc = "The template version of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WorkFlowState {
    #[doc = "`COMPLETED`"]
    Completed,
    #[doc = "`FAILED`"]
    Failed,
    #[doc = "`NOT_STARTED`"]
    NotStarted,
    #[doc = "`PROVISIONING`"]
    Provisioning,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WorkFlowState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WorkFlowState::Completed => "COMPLETED",
            WorkFlowState::Failed => "FAILED",
            WorkFlowState::NotStarted => "NOT_STARTED",
            WorkFlowState::Provisioning => "PROVISIONING",
            WorkFlowState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WorkFlowState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WorkFlowState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "COMPLETED" => WorkFlowState::Completed,
            "FAILED" => WorkFlowState::Failed,
            "NOT_STARTED" => WorkFlowState::NotStarted,
            "PROVISIONING" => WorkFlowState::Provisioning,
            _ => WorkFlowState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkFlowStatusDefaultResponse {
    #[doc = "The error message if the workflow failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "A list of resources created by the workflow"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources_created: Option<Vec<String>>,
    #[doc = "The current state of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The ID of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkFlowStatusFullResponse {
    #[doc = "The error message if the workflow failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[doc = "When the provisioning process completed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_end_time: Option<String>,
    #[doc = "When the provisioning process started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provision_start_time: Option<String>,
    #[doc = "The current progress of the provisioning process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provisioning_progress: Option<String>,
    #[doc = "A list of resources created by the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resources_created: Option<Vec<String>>,
    #[doc = "The current state of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::types::flow_framework_common::WorkFlowState>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<crate::types::flow_framework_common::User>,
    #[doc = "The list of outputs generated by the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_outputs: Option<Vec<String>>,
    #[doc = "The ID of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowIDResponse {
    #[doc = "The ID of the workflow."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workflow_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowSearchResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::flow_framework_common::Shards>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<crate::types::flow_framework_common::Hits>,
    #[doc = "Whether the request timed out."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    #[doc = "The amount of time in milliseconds that the request took to complete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub took: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowSearchStateResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::flow_framework_common::Shards>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hits: Option<crate::types::flow_framework_common::StateHits>,
    #[doc = "Whether the request timed out."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timed_out: Option<bool>,
    #[doc = "The amount of time in milliseconds that the request took to complete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub took: Option<i64>,
}

#[doc = "A map of workflow steps and their configurations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WorkflowStep {
    #[doc = "The list of required inputs for the workflow step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inputs: Option<Vec<String>>,
    #[doc = "The list of outputs generated by the workflow step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outputs: Option<Vec<String>>,
    #[doc = "The list of plugins required for the workflow step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub required_plugins: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::types::common::Duration>,
}

pub type WorkflowSteps =
    std::collections::BTreeMap<String, crate::types::flow_framework_common::WorkflowStep>;
