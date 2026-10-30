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

//! Response types of the `cat_thread_pool` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadPoolRecord {
    #[doc = "The number of active threads in the current thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<String>,
    #[doc = "The number of completed tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<String>,
    #[doc = "The core number of active threads allowed in a scaling thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<String>,
    #[doc = "The ephemeral node identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ephemeral_node_id: Option<String>,
    #[doc = "The hostname for the current node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The IP address for the current node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The thread keep alive time."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
    #[doc = "The highest number of active threads in the current thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub largest: Option<String>,
    #[doc = "The maximum number of active threads allowed in a scaling thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<String>,
    #[doc = "The thread pool name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_id: Option<crate::types::common::NodeId>,
    #[doc = "The node name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    #[doc = "The process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[doc = "The number of threads in the current thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pool_size: Option<String>,
    #[doc = "The bound transport port for the current node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The number of tasks currently in queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<String>,
    #[doc = "The maximum number of tasks permitted in the queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<String>,
    #[doc = "The number of rejected tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<String>,
    #[doc = "The number of active threads allowed in a fixed thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wait_time: Option<String>,
    #[doc = "The thread pool type. Returned values include `fixed`, `fixed_auto_queue_size`, `direct`, and `scaling`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[doc = "The response of the `cat.thread_pool` API"]
pub type ThreadPoolResponse = crate::types::cat::ThreadPoolResponse;
