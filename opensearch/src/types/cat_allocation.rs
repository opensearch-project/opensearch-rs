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

//! Response types of the `cat_allocation` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllocationRecord {
    #[doc = "Free disk space available to OpenSearch. OpenSearch retrieves this metric from the node's operating system. Disk-based shard allocation uses this metric to assign shards to nodes based on available disk space."]
    #[serde(rename = "disk.avail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_avail: Option<String>,
    #[doc = "Disk space used by the node's shards. Does not include disk space for the translog or unassigned shards. IMPORTANT: This metric double-counts disk space for hard-linked files, such as those created when shrinking, splitting, or cloning an index."]
    #[serde(rename = "disk.indices")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_indices: Option<String>,
    #[doc = "Total percentage of disk space in use. Calculated as `disk.used / disk.total`."]
    #[serde(rename = "disk.percent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_percent: Option<crate::types::common::PercentageString>,
    #[doc = "Total disk space for the node, including in-use and available space."]
    #[serde(rename = "disk.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_total: Option<String>,
    #[doc = "Total disk space in use. OpenSearch retrieves this metric from the node's operating system (OS). The metric includes disk space for: OpenSearch, including the translog and unassigned shards; the node's operating system; any other applications or files on the node. Unlike `disk.indices`, this metric does not double-count disk space for hard-linked files."]
    #[serde(rename = "disk.used")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_used: Option<String>,
    #[doc = "Network host for the node. Set using the `network.host` setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[doc = "IP address and port for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::common::Ip>,
    #[doc = "Name for the node. Set using the `node.name` setting."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[doc = "Number of primary and replica shards assigned to the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<String>,
}

#[doc = "The response of the `cat.allocation` API"]
pub type AllocationResponse = crate::types::cat::AllocationResponse;
