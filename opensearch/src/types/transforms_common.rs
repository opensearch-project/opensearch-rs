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

//! Response types of the `transforms_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ContinuousStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents_behind: Option<std::collections::BTreeMap<String, f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_timestamp: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateHistogramGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Explain {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_metadata: Option<crate::types::transforms_common::TransformMetadata>,
}

pub type ExplainResponse =
    std::collections::BTreeMap<String, crate::types::transforms_common::Explain>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents_indexed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents_processed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_time_in_millis: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pages_processed: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_time_in_millis: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupsConfigItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_histogram: Option<crate::types::transforms_common::DateHistogramGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histogram: Option<crate::types::transforms_common::HistogramGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<crate::types::transforms_common::TermsGroup>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Preview {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents:
        Option<Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    pub interval: crate::types::transforms_common::ScheduleInterval,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScheduleInterval {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermsGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations: Option<
        std::collections::BTreeMap<String, crate::types::transforms_common::GroupsConfigItem>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_selection_query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<crate::types::transforms_common::GroupsConfigItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<crate::types::transforms_common::Schedule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_index: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous_stats: Option<crate::types::transforms_common::ContinuousStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_at: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::transforms_common::ExplainStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformResponse {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<crate::types::transforms_common::Transform>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransformsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_transforms: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<crate::types::transforms_common::TransformResponse>>,
}
