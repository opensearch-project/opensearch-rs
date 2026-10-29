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

//! Response types of the `cluster_pending_tasks` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PendingTask {
    #[doc = "Indicates whether the pending tasks are currently executing or not."]
    pub executing: bool,
    #[doc = "The number that represents when the task has been inserted into the task queue."]
    pub insert_order: i32,
    #[doc = "The priority of the pending task. The valid priorities in descending priority order are: `IMMEDIATE` > `URGENT` > `HIGH` > `NORMAL` > `LOW` > `LANGUID`."]
    pub priority: String,
    #[doc = "A general description of the cluster task that may include a reason and origin."]
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_execution: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_execution_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_queue: Option<crate::types::common::Duration>,
    pub time_in_queue_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The response of the `cluster.pending_tasks` API"]
pub type PendingTasksResponse = crate::types::cluster::PendingTasksResponse;
