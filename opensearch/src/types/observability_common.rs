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

//! Response types of the `observability_common` specification namespace

#![allow(missing_docs)]

#[doc = "The core observability object containing visualization and query data."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObservabilityObject {
    #[doc = "When the object was created in milliseconds."]
    #[serde(rename = "createdTimeMs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time_ms: Option<i64>,
    #[doc = "When the object was last updated in milliseconds."]
    #[serde(rename = "lastUpdatedTimeMs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_ms: Option<i64>,
    #[doc = "The unique identifier of the observability object."]
    #[serde(rename = "objectId")]
    pub object_id: String,
    #[serde(rename = "operationalPanel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operational_panel: Option<crate::types::observability_common::OperationalPanel>,
    #[serde(rename = "savedQuery")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_query: Option<crate::types::observability_common::SavedQuery>,
    #[serde(rename = "savedVisualization")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub saved_visualization: Option<crate::types::observability_common::SavedVisualization>,
    #[doc = "The tenant associated with the observability object."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
}

#[doc = "A list of observability objects with pagination details."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObservabilityObjectList {
    #[doc = "The list of observability objects."]
    #[serde(rename = "observabilityObjectList")]
    pub observability_object_list: Vec<crate::types::observability_common::ObservabilityObject>,
    #[doc = "The starting index of the returned objects."]
    #[serde(rename = "startIndex")]
    pub start_index: i64,
    #[doc = "The relationship of total hits to the query."]
    #[serde(rename = "totalHitRelation")]
    pub total_hit_relation: String,
    #[doc = "The total number of matching objects."]
    #[serde(rename = "totalHits")]
    pub total_hits: i64,
}

#[doc = "The panel configuration for operational visualizations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperationalPanel {
    #[doc = "The identifier of the associated application."]
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[doc = "The name of the operational panel."]
    pub name: String,
    #[serde(rename = "queryFilter")]
    pub query_filter: crate::types::observability_common::QueryFilter,
    #[serde(rename = "timeRange")]
    pub time_range: crate::types::observability_common::TimeRange,
    #[doc = "The list of visualizations in the panel."]
    pub visualizations: Vec<crate::types::observability_common::Visualization>,
}

#[doc = "The filter configuration for queries."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryFilter {
    #[doc = "The query language used."]
    pub language: String,
    #[doc = "The query string."]
    pub query: String,
}

#[doc = "The configuration for a saved query."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SavedQuery {
    #[doc = "The description of the saved query."]
    pub description: String,
    #[doc = "The name of the saved query."]
    pub name: String,
    #[doc = "The query string."]
    pub query: String,
    pub selected_date_range: crate::types::observability_common::SelectedDateRange,
    pub selected_fields: crate::types::observability_common::SelectedFields,
    pub selected_timestamp: crate::types::observability_common::SelectedTimestamp,
}

#[doc = "The configuration for a saved visualization."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SavedVisualization {
    #[doc = "The description of the saved visualization."]
    pub description: String,
    #[doc = "The name of the saved visualization."]
    pub name: String,
    #[doc = "The query associated with the visualization."]
    pub query: String,
    pub selected_date_range: crate::types::observability_common::SelectedDateRange,
    pub selected_fields: crate::types::observability_common::SelectedFields,
    pub selected_timestamp: crate::types::observability_common::SelectedTimestamp,
    #[doc = "The type of visualization."]
    pub r#type: String,
}

#[doc = "The date range selection configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SelectedDateRange {
    #[doc = "The end date of the range."]
    pub end: String,
    #[doc = "The start date of the range."]
    pub start: String,
    #[doc = "The text representation of the date range."]
    pub text: String,
}

#[doc = "The field selection configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SelectedFields {
    #[doc = "The text representation of selected fields."]
    pub text: String,
    #[doc = "The list of field tokens."]
    pub tokens: Vec<crate::types::observability_common::Token>,
}

#[doc = "The timestamp field selection configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SelectedTimestamp {
    #[doc = "The name of the selected timestamp field."]
    pub name: String,
    #[doc = "The type of the timestamp field."]
    pub r#type: String,
}

#[doc = "The time range configuration for queries and visualizations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimeRange {
    #[doc = "The start time of the range."]
    pub from: String,
    #[doc = "The end time of the range."]
    pub to: String,
}

#[doc = "The configuration for a field token."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Token {
    #[doc = "The name of the token."]
    pub name: String,
    #[doc = "The type of the token."]
    pub r#type: String,
}

#[doc = "The configuration for a single visualization."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Visualization {
    #[doc = "The height of the visualization."]
    pub h: i64,
    #[doc = "The unique identifier of the visualization."]
    pub id: String,
    #[doc = "The identifier of the saved visualization."]
    #[serde(rename = "savedVisualizationId")]
    pub saved_visualization_id: String,
    #[doc = "The width of the visualization."]
    pub w: i64,
    #[doc = "The x-coordinate position of the visualization."]
    pub x: i64,
    #[doc = "The y-coordinate position of the visualization."]
    pub y: i64,
}
