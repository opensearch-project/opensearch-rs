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

//! Response types of the `cat_pending_tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingTasksRecord {
    #[doc = "The task insertion order."]
    #[serde(rename = "insertOrder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub insert_order: Option<String>,
    #[doc = "The task priority."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<String>,
    #[doc = "The task source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[doc = "Indicates how long the task has been in queue."]
    #[serde(rename = "timeInQueue")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_queue: Option<String>,
}

#[doc = "The response of the `cat.pending_tasks` API"]
pub type PendingTasksResponse = crate::types::cat::PendingTasksResponse;
