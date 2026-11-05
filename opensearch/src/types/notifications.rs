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

//! Response types of the `notifications` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CreateConfigResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListChannelsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_list: Option<Vec<crate::types::notifications_common::NotificationChannel>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hit_relation: Option<crate::types::notifications_common::TotalHitRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListFeaturesResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_config_type_list:
        Option<Vec<crate::types::notifications_common::NotificationConfigType>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_features: Option<crate::types::notifications_common::NotificationsPluginFeaturesMap>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SendTestResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_source: Option<crate::types::notifications_common::EventSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_list: Option<Vec<crate::types::notifications_common::EventStatus>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateConfigResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
}
