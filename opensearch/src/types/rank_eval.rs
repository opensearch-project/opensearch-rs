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

//! Response types of the `rank_eval` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankEvalHit {
    #[serde(rename = "_id")]
    pub id: crate::types::common::Id,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
    #[serde(rename = "_score")]
    pub score: f64,
    #[serde(rename = "_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common::Type>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankEvalHitItem {
    pub hit: crate::types::rank_eval::RankEvalHit,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankEvalMetricDetail {
    #[doc = "The `hits` section provides a grouping of the search results with their supplied ratings."]
    pub hits: Vec<crate::types::rank_eval::RankEvalHitItem>,
    #[doc = "The `metric_details` section provides additional information about the calculated quality metric indicating the number of relevant retrieved documents. The content varies for each metric but allows for better interpretation of the results."]
    pub metric_details: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    >,
    #[doc = "The `metric_score`, found in the `metric_details` section, shows the contribution of this query to the global quality metric score."]
    pub metric_score: f64,
    #[doc = "The `unrated_docs` section contains an `_index` and `_id` entry for each document that didn't have a `ratings` value. This can be used to ask the user to supply ratings for these documents."]
    pub unrated_docs: Vec<crate::types::rank_eval::UnratedDocument>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankEvalResponse {
    #[doc = "The details section contains one entry for every query in the original requests section, keyed by the search request id"]
    pub details: std::collections::BTreeMap<String, crate::types::rank_eval::RankEvalMetricDetail>,
    pub failures: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[doc = "The overall evaluation quality calculated by the defined metric"]
    pub metric_score: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnratedDocument {
    #[serde(rename = "_id")]
    pub id: crate::types::common::Id,
    #[serde(rename = "_index")]
    pub index: crate::types::common::IndexName,
}
