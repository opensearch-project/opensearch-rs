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

//! Response types of the `geospatial_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataSource {
    pub database: crate::types::geospatial_common::Database,
    #[doc = "URL endpoint for the data source."]
    pub endpoint: String,
    pub name: crate::types::common::Name,
    pub next_update_at_in_epoch_millis: crate::types::common::EpochTimeUnitMillis,
    #[doc = "State of the data source."]
    pub state: crate::types::geospatial_common::DataSourceState,
    #[doc = "Update interval."]
    pub update_interval_in_days: i64,
    pub update_stats: crate::types::geospatial_common::UpdateStats,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DataSourceState {
    #[doc = "`AVAILABLE`"]
    Available,
    #[doc = "`CREATE_FAILED`"]
    CreateFailed,
    #[doc = "`CREATING`"]
    Creating,
    #[doc = "`DELETING`"]
    Deleting,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DataSourceState {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DataSourceState::Available => "AVAILABLE",
            DataSourceState::CreateFailed => "CREATE_FAILED",
            DataSourceState::Creating => "CREATING",
            DataSourceState::Deleting => "DELETING",
            DataSourceState::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DataSourceState {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DataSourceState {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "AVAILABLE" => DataSourceState::Available,
            "CREATE_FAILED" => DataSourceState::CreateFailed,
            "CREATING" => DataSourceState::Creating,
            "DELETING" => DataSourceState::Deleting,
            _ => DataSourceState::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Database {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(rename = "sha256_hash")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sha_256_hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at_in_epoch_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub valid_for_in_days: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoSpatialGeojsonUploadResponse {
    #[doc = "Whether there were any errors."]
    pub errors: bool,
    #[doc = "Number of features that failed to upload."]
    pub failure: i64,
    #[doc = "Number of features successfully uploaded."]
    pub success: i64,
    pub took: crate::types::common::DurationValueUnitMillis,
    #[doc = "Total number of features processed."]
    pub total: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoSpatialUploadStats {
    pub metrics: Vec<crate::types::geospatial_common::UploadStatsMetric>,
    pub total: crate::types::geospatial_common::UploadStatsTotal,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetDataSourceResponse {
    pub datasources: Vec<crate::types::geospatial_common::DataSource>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_processing_time_in_millis: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_succeeded_at_in_epoch_millis: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadStatsMetric {
    pub duration: crate::types::common::DurationValueUnitMillis,
    #[doc = "Number of failed features in this upload."]
    pub failed: i64,
    pub id: crate::types::common::Id,
    pub node_id: crate::types::common::NodeId,
    #[doc = "Number of successful features in this upload."]
    pub success: i64,
    #[doc = "Type of upload (such as \"GeoJSON\")."]
    pub r#type: String,
    #[doc = "Number of features in this upload."]
    pub upload: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UploadStatsTotal {
    pub duration: crate::types::common::DurationValueUnitMillis,
    #[doc = "Total failed uploads."]
    pub failed: i64,
    #[doc = "Total number of upload requests."]
    pub request_count: i64,
    #[doc = "Total successful uploads."]
    pub success: i64,
    #[doc = "Total number of uploads."]
    pub upload: i64,
}
