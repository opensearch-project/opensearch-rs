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

//! Response types of the `search_relevance_common` specification namespace

#![allow(missing_docs)]

#[doc = "The drift status of an experiment's inputs."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExperimentValidationStatus {
    #[doc = "`DRIFTED`"]
    Drifted,
    #[doc = "`UNAVAILABLE`"]
    Unavailable,
    #[doc = "`VALID`"]
    Valid,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ExperimentValidationStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ExperimentValidationStatus::Drifted => "DRIFTED",
            ExperimentValidationStatus::Unavailable => "UNAVAILABLE",
            ExperimentValidationStatus::Valid => "VALID",
            ExperimentValidationStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ExperimentValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ExperimentValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DRIFTED" => ExperimentValidationStatus::Drifted,
            "UNAVAILABLE" => ExperimentValidationStatus::Unavailable,
            "VALID" => ExperimentValidationStatus::Valid,
            _ => ExperimentValidationStatus::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatchExperimentResponse {
    #[doc = "The ID of the updated experiment."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[doc = "The lowercase write result of the update, for example `updated`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostQuerySetsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_set_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_set_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PostScheduledExperimentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutExperimentResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub experiment_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutJudgmentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judgment_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutQuerySetsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_set_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_set_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutSearchConfigurationResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_configuration_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_configuration_result: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetryJudgmentsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judgment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateJudgmentRatingsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judgment_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

#[doc = "Describes whether an experiment's inputs have drifted from the fingerprint recorded at execution time."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValidateExperimentResponse {
    #[doc = "The list of inputs that have drifted."]
    pub drifted_inputs: Vec<String>,
    #[doc = "A human-readable message describing the validation result."]
    pub message: String,
    pub status: crate::types::search_relevance_common::ExperimentValidationStatus,
}
