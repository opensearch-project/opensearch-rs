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

//! Response types of the `nodes_stats` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdaptiveSelection {
    #[doc = "The exponentially weighted moving average queue size of search requests on the keyed node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_queue_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_response_time: Option<crate::types::common::Duration>,
    #[doc = "The exponentially weighted moving average response time, in nanoseconds, of search requests on the keyed node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_response_time_ns: Option<crate::types::common::DurationValueUnitNanos>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_service_time: Option<crate::types::common::Duration>,
    #[doc = "The exponentially weighted moving average service time, in nanoseconds, of search requests on the keyed node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_service_time_ns: Option<crate::types::common::DurationValueUnitNanos>,
    #[doc = "The number of outstanding search requests to the keyed node from the node these stats are for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outgoing_searches: Option<i64>,
    #[doc = "The rank of this node; used for shard selection when routing search requests."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Breaker {
    #[doc = "Estimated memory used for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Estimated memory used, in bytes, for the operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub estimated_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory limit for the circuit breaker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory limit, in bytes, for the circuit breaker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "A constant that all estimates for the circuit breaker are multiplied with to calculate a final estimate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overhead: Option<f64>,
    #[doc = "Total number of times the circuit breaker has been triggered and prevented an out of memory error."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tripped: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheIndicesStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evictions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheShardStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evictions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CacheStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evictions: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hit_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub miss_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::CacheIndicesStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::CacheShardStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_name: Option<String>,
}

pub type CachesStats = std::collections::BTreeMap<String, crate::types::nodes_stats::CacheStats>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupCpuAcctStats {
    #[doc = "The `cpuacct` control group to which the OpenSearch process belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_nanos: Option<crate::types::common::DurationValueUnitNanos>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupCpuStat {
    #[doc = "The number of reporting periods (as specified by `cfs_period_micros`) that have elapsed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_elapsed_periods: Option<i64>,
    #[doc = "The number of times all tasks in the same cgroup as the OpenSearch process have been throttled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_times_throttled: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_throttled_nanos: Option<crate::types::common::DurationValueUnitNanos>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupCpuStats {
    #[doc = "The period of time, in microseconds, for how regularly all tasks in the same cgroup as the OpenSearch process should have their access to CPU resources reallocated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cfs_period_micros: Option<crate::types::common::DurationValueUnitMicros>,
    #[doc = "The total amount of time, in microseconds, for which all tasks in the same cgroup as the OpenSearch process can run during one period `cfs_period_micros`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cfs_quota_micros: Option<crate::types::common::DurationValueUnitMicros>,
    #[doc = "The `cpu` control group to which the OpenSearch process belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_group: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stat: Option<crate::types::nodes_stats::CgroupCpuStat>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupMemoryStats {
    #[doc = "The `memory` control group to which the OpenSearch process belongs."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_group: Option<String>,
    #[doc = "The maximum amount of user memory (including file cache) allowed for all tasks in the same cgroup as the OpenSearch process. This value can be too big to store in a `long`, so is returned as a string so that the value returned can exactly match what the underlying operating system interface returns. Any value that is too large to parse into a `long` almost certainly means no limit has been set for the cgroup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_in_bytes: Option<String>,
    #[doc = "The total current memory usage by processes in the cgroup, in bytes, by all tasks in the same cgroup as the OpenSearch process. This value is stored as a string for consistency with `limit_in_bytes`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_in_bytes: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CgroupStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::nodes_stats::CgroupCpuStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpuacct: Option<crate::types::nodes_stats::CgroupCpuAcctStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::nodes_stats::CgroupMemoryStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterStateOverallStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterStateQueue {
    #[doc = "Number of committed cluster states in queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed: Option<i32>,
    #[doc = "Number of pending cluster states in queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<i32>,
    #[doc = "Total number of cluster states in queue."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterStateStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overall: Option<crate::types::nodes_stats::ClusterStateOverallStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataPathStats {
    #[doc = "Total amount of disk space available to this Java virtual machine on this file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to this Java virtual machine on this file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total number of cache bytes reserved available to this Java virtual machine on this file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of cache bytes reserved available to this Java virtual machine on this file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of unallocated disk space in the file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in the file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Mount point of the file store (for example: `/dev/sda2`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mount: Option<String>,
    #[doc = "Path to the file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Total size of the file store."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of the file store in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Type of the file store (ex: ext4)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Discovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_state_queue: Option<crate::types::nodes_stats::ClusterStateQueue>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_state_stats: Option<crate::types::nodes_stats::ClusterStateStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published_cluster_states: Option<crate::types::nodes_stats::PublishedClusterStates>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedMemoryStats {
    #[doc = "Amount of free physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of free physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount of physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount of used physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of used physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Percentage of free memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Percentage of used memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_percent: Option<crate::types::common::PercentageNumber>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileSystem {
    #[doc = "List of all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<crate::types::nodes_stats::DataPathStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_stats: Option<crate::types::nodes_stats::IoStats>,
    #[doc = "Last time the file stores statistics were refreshed. Recorded in milliseconds since the Unix Epoch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::nodes_stats::FileSystemTotal>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FileSystemTotal {
    #[doc = "Total disk space available to this Java virtual machine on all file stores. Depending on OS or process level restrictions, this might appear less than `free`. This is the actual amount of free disk space the OpenSearch node can utilize."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to this Java virtual machine on all file stores. Depending on OS or process level restrictions, this might appear less than `free_in_bytes`. This is the actual amount of free disk space the OpenSearch node can utilize."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total size of cache bytes reserved in all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of cache bytes reserved in all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total unallocated disk space in all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total size of all file stores."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size of all file stores in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GarbageCollector {
    #[doc = "Contains statistics about JVM garbage collectors for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectors: Option<
        std::collections::BTreeMap<String, crate::types::nodes_stats::GarbageCollectorTotal>,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GarbageCollectorTotal {
    #[doc = "Total number of JVM garbage collectors that collect objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_count: Option<i64>,
    #[doc = "Total time spent by JVM collecting objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_time: Option<crate::types::common::Duration>,
    #[doc = "Total time, in milliseconds, spent by JVM collecting objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Http {
    #[doc = "Current number of open HTTP connections for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_open: Option<i64>,
    #[doc = "Total number of HTTP connections opened for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_opened: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<crate::types::nodes_stats::IndexingPressureMemory>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressureMemory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<crate::types::nodes_stats::PressureMemory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Configured memory limit, in bytes, for the indexing requests. Replica requests have an automatic limit that is 1.5x this value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::nodes_stats::PressureMemory>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ingest {
    #[doc = "Contains statistics about ingest pipelines for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipelines:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::IngestTotal>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::nodes_stats::IngestTotal>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestTotal {
    #[doc = "Total number of documents ingested during the lifetime of this node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Total number of documents currently being ingested."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[doc = "Total number of failed ingest operations during the lifetime of this node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[doc = "Total number of ingest processors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors:
        Option<Vec<std::collections::BTreeMap<String, crate::types::nodes_stats::KeyedProcessor>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IoStatDevice {
    #[doc = "The Linux device name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[doc = "The total number of read and write operations for the device completed since starting OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue_size: Option<i64>,
    #[doc = "The total number of kilobytes read for the device since starting OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_kilobytes: Option<i64>,
    #[doc = "The total number of read operations for the device completed since starting OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_operations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_time: Option<i64>,
    #[doc = "The total number of kilobytes written for the device since starting OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_kilobytes: Option<i64>,
    #[doc = "The total number of write operations for the device completed since starting OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_operations: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_time: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IoStats {
    #[doc = "Array of disk metrics for each device that is backing an OpenSearch data path. These disk metrics are probed periodically and averages between the last probe and the current probe are computed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<crate::types::nodes_stats::IoStatDevice>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::nodes_stats::IoStatDevice>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Jvm {
    #[doc = "Contains statistics about JVM buffer pools for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buffer_pools:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::NodeBufferPool>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classes: Option<crate::types::nodes_stats::JvmClasses>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gc: Option<crate::types::nodes_stats::GarbageCollector>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem: Option<crate::types::nodes_stats::JvmMemoryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads: Option<crate::types::nodes_stats::JvmThreads>,
    #[doc = "Last time JVM statistics were refreshed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "Human-readable JVM uptime. Only returned if the `human` query parameter is `true`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<crate::types::common::Duration>,
    #[doc = "JVM uptime in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JvmClasses {
    #[doc = "Number of classes currently loaded by JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_loaded_count: Option<i64>,
    #[doc = "Total number of classes loaded since the JVM started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_loaded_count: Option<i64>,
    #[doc = "Total number of classes unloaded since the JVM started."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_unloaded_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JvmMemoryStats {
    #[doc = "Amount of memory available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_committed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of memory, in bytes, available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_committed_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Maximum amount of memory available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory currently in use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, currently in use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Percentage of memory currently in use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_used_percent: Option<crate::types::common::PercentageNumber>,
    #[doc = "Amount of non-heap memory available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_committed: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of non-heap memory available, in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_committed_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Non-heap memory used."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Non-heap memory used, in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Contains statistics about heap memory usage for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pools: Option<std::collections::BTreeMap<String, crate::types::nodes_stats::Pool>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JvmThreads {
    #[doc = "Number of active threads in use by JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Highest number of threads used by JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyedProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::nodes_stats::Processor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LastGcStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage_percent: Option<crate::types::common::PercentageNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MemoryStats {
    #[doc = "Amount of free physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of free physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total amount of physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount of physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount of used physical memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount of used physical memory in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeBufferPool {
    #[doc = "Number of buffer pools."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Total capacity of buffer pools."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_capacity: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total capacity of buffer pools in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_capacity_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Size of buffer pools."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size of buffer pools in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeIndexShardStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::common::CompletionStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<crate::types::common::DocStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::common::FielddataStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush: Option<crate::types::common::FlushStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::GetStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing: Option<crate::types::common::IndexingStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges: Option<crate::types::common::MergesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<crate::types::common::RecoveryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<crate::types::common::RefreshStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::common::SearchStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<crate::types::common::SegmentsStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::common::StoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog: Option<crate::types::common::TranslogStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer: Option<crate::types::common::WarmerStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit: Option<crate::types::indices_stats::ShardCommit>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_leases: Option<crate::types::indices_stats::ShardRetentionLeases>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::indices_stats::ShardRouting>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::indices_stats::ShardSequenceNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_path: Option<crate::types::indices_stats::ShardPath>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeIndicesStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::common::CompletionStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<crate::types::common::DocStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::common::FielddataStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush: Option<crate::types::common::FlushStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get: Option<crate::types::common::GetStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing: Option<crate::types::common::IndexingStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges: Option<crate::types::common::MergesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recovery: Option<crate::types::common::RecoveryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh: Option<crate::types::common::RefreshStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache: Option<crate::types::common::RequestCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::common::SearchStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<crate::types::common::SegmentsStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::common::StoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub translog: Option<crate::types::common::TranslogStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer: Option<crate::types::common::WarmerStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_stats::IndexStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<
        std::collections::BTreeMap<
            String,
            Vec<std::collections::BTreeMap<String, crate::types::nodes_stats::NodeIndexShardStats>>,
        >,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cgroup: Option<crate::types::nodes_stats::CgroupStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::nodes_stats::OperatingSystemCpuStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem: Option<crate::types::nodes_stats::ExtendedMemoryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<crate::types::nodes_stats::MemoryStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemCpuStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_average: Option<std::collections::BTreeMap<String, f64>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<crate::types::common::PercentageNumber>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pool {
    #[doc = "Most recent GC cycle stats for a particular memory pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_gc_stats: Option<crate::types::nodes_stats::LastGcStats>,
    #[doc = "Maximum amount of memory available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Largest amount of memory historically used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Largest amount of memory, in bytes, historically used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_max_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Largest amount of memory historically used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Largest amount of memory, in bytes, historically used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peak_used_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Memory used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, used by the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PressureMemory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating, primary, or replica stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combined_coordinating_and_primary: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating or primary stage. This value is not the sum of coordinating and primary as a node can reuse the coordinating memory if the primary stage is executed locally."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combined_coordinating_and_primary_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinating: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the coordinating stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinating_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the coordinating stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinating_rejections: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the primary stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the primary stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_rejections: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory consumed, in bytes, by indexing requests in the replica stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Number of indexing requests rejected in the replica stage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica_rejections: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Process {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::nodes_stats::ProcessCpuStats>,
    #[doc = "Maximum number of file descriptors allowed on the system, or `-1` if not supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_file_descriptors: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem: Option<crate::types::nodes_stats::ProcessMemoryStats>,
    #[doc = "Number of opened file descriptors associated with the current or `-1` if not supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_file_descriptors: Option<i64>,
    #[doc = "Last time the statistics were refreshed. Recorded in milliseconds since the Unix Epoch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessCpuStats {
    pub percent: crate::types::common::PercentageNumber,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::Duration>,
    pub total_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessMemoryStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_virtual: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_virtual_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Processor {
    #[doc = "Number of documents transformed by the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "Number of documents currently being transformed by the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[doc = "Number of failed operations for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PublishedClusterStates {
    #[doc = "Number of compatible differences between published cluster states."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compatible_diffs: Option<i64>,
    #[doc = "Number of published cluster states."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_states: Option<i64>,
    #[doc = "Number of incompatible differences between published cluster states."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incompatible_diffs: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoteStoreStats {
    #[doc = "Timestamp for the last successful fetch of pinned timestamps."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_fetch_of_pinned_timestamps:
        Option<crate::types::common::EpochTimeUnitSeconds>,
}

#[doc = "Statistics snapshot about a repository."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RepositoryStatsSnapshot {
    #[doc = "The location configuration of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_location: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The name of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_name: Option<String>,
    #[doc = "The type of the repository."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository_type: Option<String>,
    #[doc = "The count of requests by operation type (when detailed mode is disabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_counts: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The count of failed requests by operation type (when detailed mode is enabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_failures_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The total retry count by operation type (when detailed mode is enabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_retry_count_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The count of successful requests by operation type (when detailed mode is enabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_success_total: Option<std::collections::BTreeMap<String, i64>>,
    #[doc = "The total request time in milliseconds by operation type (when detailed mode is enabled)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_time_in_millis: Option<std::collections::BTreeMap<String, i64>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptCacheStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<crate::types::nodes_stats::ScriptContextStats>>,
    pub sum: crate::types::nodes_stats::ScriptStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptContextStats {
    #[doc = "Total number of times the script cache has evicted old data."]
    pub cache_evictions: i64,
    #[doc = "Total number of times the script compilation circuit breaker has limited inline script compilations."]
    pub compilation_limit_triggered: i64,
    #[doc = "Total number of inline script compilations performed by the node."]
    pub compilations: i64,
    pub context: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptStats {
    #[doc = "Total number of times the script cache has evicted old data."]
    pub cache_evictions: i64,
    #[doc = "Total number of times the script compilation circuit breaker has limited inline script compilations."]
    pub compilation_limit_triggered: i64,
    #[doc = "Total number of inline script compilations performed by the node."]
    pub compilations: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardAdmissionControlStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_cpu_usage: Option<crate::types::nodes_stats::UsageStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_io_usage: Option<crate::types::nodes_stats::UsageStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardClusterManagerThrottlingStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::nodes_stats::ShardClusterManagerThrottlingStatsDetail>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardClusterManagerThrottlingStatsDetail {
    pub throttled_tasks_per_task_type: std::collections::BTreeMap<String, i64>,
    pub total_throttled_tasks: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardIndexingStats {
    pub coordinating_count: i64,
    pub coordinating_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub primary_count: i64,
    pub primary_time_in_millis: crate::types::common::DurationValueUnitMillis,
    pub replica_count: i64,
    pub replica_time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardLastSuccessfulTimestamp {
    pub coordinating_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
    pub primary_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
    pub replica_last_successful_request_timestamp_in_millis:
        crate::types::common::EpochTimeUnitMillis,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardMemoryAllocationCurrentStats {
    pub current_coordinating_and_primary_bytes: crate::types::common::ByteCount,
    pub current_replica_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardMemoryAllocationLimitStats {
    pub current_coordinating_and_primary_limits_in_bytes: crate::types::common::ByteCount,
    pub current_replica_limits_in_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardMemoryAllocationStats {
    pub current:
        crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationCurrentStats,
    pub limit: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationLimitStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardMemoryStats {
    pub current: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
    pub total: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStatsDetails,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardMemoryStatsDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinating: Option<crate::types::common::HumanReadableByteCount>,
    pub coordinating_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<crate::types::common::HumanReadableByteCount>,
    pub primary_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica: Option<crate::types::common::HumanReadableByteCount>,
    pub replica_in_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardRejectionCoordinatingStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub coordinating_rejections: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardRejectionPrimaryStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub primary_rejections: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardRejectionReplicaStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup: Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    pub replica_rejections: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardRejectionStats {
    pub coordinating:
        crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionCoordinatingStats,
    pub primary: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionPrimaryStats,
    pub replica: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionReplicaStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressurePerShardStats {
    pub indexing: crate::types::nodes_stats::ShardIndexingPressurePerShardIndexingStats,
    pub last_successful_timestamp:
        crate::types::nodes_stats::ShardIndexingPressurePerShardLastSuccessfulTimestamp,
    pub memory: crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryStats,
    pub memory_allocation:
        crate::types::nodes_stats::ShardIndexingPressurePerShardMemoryAllocationStats,
    pub rejection: crate::types::nodes_stats::ShardIndexingPressurePerShardRejectionStats,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressureRejectionsBreakupStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_successful_request_limits: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_limits: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throughput_degradation_limits: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardIndexingPressureStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enforced: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<
        std::collections::BTreeMap<
            String,
            crate::types::nodes_stats::ShardIndexingPressurePerShardStats,
        >,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_rejections_breakup:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_rejections_breakup_shadow_mode:
        Option<crate::types::nodes_stats::ShardIndexingPressureRejectionsBreakupStats>,
}

pub type ShardRepositoriesStats = Vec<crate::types::nodes_stats::RepositoryStatsSnapshot>;

pub type ShardResourceUsageStats =
    std::collections::BTreeMap<String, crate::types::nodes_stats::ShardResourceUsageStatsDetail>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardResourceUsageStatsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_utilization_percent: Option<crate::types::common::PercentageString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub io_usage_stats: Option<crate::types::nodes_stats::ShardResourceUsageStatsIoUsageStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_utilization_percent: Option<crate::types::common::PercentageString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardResourceUsageStatsIoUsageStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_io_utilization_percent: Option<crate::types::common::PercentageString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardSearchBackpressureMode {
    #[doc = "`disabled`"]
    Disabled,
    #[doc = "`enforced`"]
    Enforced,
    #[doc = "`monitor_only`"]
    MonitorOnly,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardSearchBackpressureMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardSearchBackpressureMode::Disabled => "disabled",
            ShardSearchBackpressureMode::Enforced => "enforced",
            ShardSearchBackpressureMode::MonitorOnly => "monitor_only",
            ShardSearchBackpressureMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardSearchBackpressureMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardSearchBackpressureMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "disabled" => ShardSearchBackpressureMode::Disabled,
            "enforced" => ShardSearchBackpressureMode::Enforced,
            "monitor_only" => ShardSearchBackpressureMode::MonitorOnly,
            _ => ShardSearchBackpressureMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::nodes_stats::ShardSearchBackpressureMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_shard_task: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_task: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskCancellationStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_limit_reached_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_task_percentage: Option<crate::types::common::PercentageNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_cancellation_eligible_tasks_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg_millis: Option<crate::types::common::DurationValueUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_avg_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_max_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_avg: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolling_avg_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskResourceTrackerStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerCpuUsageTrackerStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub elapsed_time_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerElapsedTimeTrackerStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_usage_tracker: Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerHeapUsageTrackerStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchBackpressureTaskStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_stats:
        Option<crate::types::nodes_stats::ShardSearchBackpressureTaskCancellationStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_tracker_stats:
        Option<crate::types::nodes_stats::ShardSearchBackpressureTaskResourceTrackerStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchPipelineOperationStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_millis: Option<crate::types::common::DurationValueUnitMillis>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchPipelinePerPipelineProcessorStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchPipelinePerPipelineStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_processors:
        Option<Vec<crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_processors: Option<
        Vec<
            std::collections::BTreeMap<
                String,
                crate::types::nodes_stats::ShardSearchPipelinePerPipelineProcessorStats,
            >,
        >,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSearchPipelineStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipelines: Option<
        std::collections::BTreeMap<
            String,
            crate::types::nodes_stats::ShardSearchPipelinePerPipelineStats,
        >,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_request: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_response: Option<crate::types::nodes_stats::ShardSearchPipelineOperationStats>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardSegmentReplicationBackpressureStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_rejected_requests: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardTaskCancellationStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_shard_task: Option<crate::types::nodes_stats::ShardTaskCancellationStatsDetail>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_task: Option<crate::types::nodes_stats::ShardTaskCancellationStatsDetail>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardTaskCancellationStatsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_count_post_cancel: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_count_post_cancel: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardWeightedRoutingStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats: Option<crate::types::nodes_stats::ShardWeightedRoutingStatsDetail>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardWeightedRoutingStatsDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail_open_count: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[doc = "Statistics about adaptive replica selection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adaptive_selection:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::AdaptiveSelection>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admission_control: Option<crate::types::nodes_stats::ShardAdmissionControlStats>,
    #[doc = "Contains a list of attributes for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "Statistics about the field data circuit breaker."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breakers: Option<std::collections::BTreeMap<String, crate::types::nodes_stats::Breaker>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caches: Option<crate::types::nodes_stats::CachesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager_throttling:
        Option<crate::types::nodes_stats::ShardClusterManagerThrottlingStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<crate::types::nodes_stats::Discovery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<crate::types::nodes_stats::FileSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<crate::types::nodes_stats::Http>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_pressure: Option<crate::types::nodes_stats::IndexingPressure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices: Option<crate::types::nodes_stats::NodeIndicesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::nodes_stats::Ingest>,
    #[doc = "IP address and port for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::nodes_stats::StatsIp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm: Option<crate::types::nodes_stats::Jvm>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<crate::types::nodes_stats::OperatingSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<crate::types::nodes_stats::Process>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_store: Option<crate::types::nodes_stats::RemoteStoreStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<crate::types::nodes_stats::ShardRepositoriesStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_usage_stats: Option<crate::types::nodes_stats::ShardResourceUsageStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<crate::types::common::NodeRoles>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::nodes_stats::ScriptStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_cache: Option<crate::types::nodes_stats::ScriptCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_backpressure: Option<crate::types::nodes_stats::ShardSearchBackpressureStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_pipeline: Option<crate::types::nodes_stats::ShardSearchPipelineStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment_replication_backpressure:
        Option<crate::types::nodes_stats::ShardSegmentReplicationBackpressureStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard_indexing_pressure: Option<crate::types::nodes_stats::ShardIndexingPressureStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_cancellation: Option<crate::types::nodes_stats::ShardTaskCancellationStats>,
    #[doc = "Statistics about each thread pool, including current size, queue and rejected tasks."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_pool:
        Option<std::collections::BTreeMap<String, crate::types::nodes_stats::ThreadCount>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<crate::types::nodes_stats::Transport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_address: Option<crate::types::common::TransportAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weighted_routing: Option<crate::types::nodes_stats::ShardWeightedRoutingStats>,
}

#[doc = "IP address and port for the node."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StatsIp {
    Ip(crate::types::common::Ip),
    Array(Vec<crate::types::common::Ip>),
}
impl<'de> serde::Deserialize<'de> for StatsIp {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'"') => serde_json::from_str(text).map(StatsIp::Ip),
            Some(b'[') => serde_json::from_str(text).map(StatsIp::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "StatsIp"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The response of the `nodes.stats` API"]
pub type StatsResponse = crate::types::nodes::StatsResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ThreadCount {
    #[doc = "Number of active threads in the thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<i64>,
    #[doc = "Number of tasks completed by the thread pool executor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed: Option<i64>,
    #[doc = "Highest number of active threads in the thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub largest: Option<i64>,
    #[doc = "Number of tasks in queue for the thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub queue: Option<i64>,
    #[doc = "Number of tasks rejected by the thread pool executor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
    #[doc = "Number of threads in the thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threads: Option<i64>,
    #[doc = "The total amount of time that tasks spend waiting in the thread pool queue. Currently, only `search`, `search_throttled`, and `index_searcher` thread pools support this metric. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wait_time: Option<crate::types::common::Duration>,
    #[doc = "The total amount of time that tasks spend waiting in the thread pool queue. Currently, only `search`, `search_throttled`, and `index_searcher` thread pools support this metric. "]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wait_time_in_nanos: Option<crate::types::common::DurationValueUnitNanos>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transport {
    #[doc = "Total number of RX (receive) packets received by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx_count: Option<i64>,
    #[doc = "Size of RX packets received by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size, in bytes, of RX packets received by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rx_size_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Current number of inbound TCP connections used for internal communication between nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub server_open: Option<i64>,
    #[doc = "The cumulative number of outbound transport connections that this node has opened since it started. Each transport connection may comprise multiple TCP connections but is only counted once in this statistic. Transport connections are typically long-lived so this statistic should remain constant in a stable cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_outbound_connections: Option<i64>,
    #[doc = "Total number of TX (transmit) packets sent by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_count: Option<i64>,
    #[doc = "Size of TX packets sent by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Size, in bytes, of TX packets sent by the node during internal cluster communication."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tx_size_in_bytes: Option<crate::types::common::ByteCount>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransportUsageStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejection_count: Option<std::collections::BTreeMap<String, i64>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UsageStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<crate::types::nodes_stats::TransportUsageStats>,
}
