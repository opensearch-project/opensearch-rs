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

//! Response types of the `cluster_stats` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharFilterTypes {
    #[doc = "Contains statistics about analyzer types used in selected nodes."]
    pub analyzer_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in analyzers used in selected nodes."]
    pub built_in_analyzers: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in character filters used in selected nodes."]
    pub built_in_char_filters: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in token filters used in selected nodes."]
    pub built_in_filters: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about built-in tokenizers used in selected nodes."]
    pub built_in_tokenizers: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about character filter types used in selected nodes."]
    pub char_filter_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about token filter types used in selected nodes."]
    pub filter_types: Vec<crate::types::cluster_stats::FieldTypes>,
    #[doc = "Contains statistics about tokenizer types used in selected nodes."]
    pub tokenizer_types: Vec<crate::types::cluster_stats::FieldTypes>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterFileSystem {
    #[doc = "Total number of bytes available to JVM in file stores across all selected nodes. Depending on operating system or process-level restrictions, this number may be less than `nodes.fs.free_in_byes`. This is the actual amount of free disk space the selected OpenSearch nodes can use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub available: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of bytes available to JVM in file stores across all selected nodes. Depending on operating system or process-level restrictions, this number may be less than `nodes.fs.free_in_byes`. This is the actual amount of free disk space the selected OpenSearch nodes can use."]
    pub available_in_bytes: crate::types::common::ByteCount,
    #[doc = "Total size, of all cache reserved across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all cache reserved across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache_reserved_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Total number of unallocated bytes in file stores across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total number of unallocated bytes in file stores across all selected nodes."]
    pub free_in_bytes: crate::types::common::ByteCount,
    #[doc = "Total size, of all file stores across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total size, in bytes, of all file stores across all selected nodes."]
    pub total_in_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterIndices {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analysis: Option<crate::types::cluster_stats::CharFilterTypes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion: Option<crate::types::common::CompletionStats>,
    #[doc = "Total number of indexes with shards assigned to selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<crate::types::common::DocStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::common::FielddataStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<crate::types::cluster_stats::FieldTypesMappings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache: Option<crate::types::common::QueryCacheStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments: Option<crate::types::common::SegmentsStats>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::cluster_stats::ClusterIndicesShards>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::common::StoreStats>,
    #[doc = "Contains statistics about analyzers and analyzer components used in selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::types::cluster_stats::IndicesVersions>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterIndicesShards {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::cluster_stats::ClusterIndicesShardsIndex>,
    #[doc = "Number of primary shards assigned to selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primaries: Option<i32>,
    #[doc = "Ratio of replica shards to primary shards across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replication: Option<i32>,
    #[doc = "Total number of shards assigned to selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterIndicesShardsIndex {
    #[doc = "Contains statistics about the number of primary shards assigned to selected nodes."]
    pub primaries: crate::types::cluster_stats::ClusterShardMetrics,
    #[doc = "Contains statistics about the number of replication shards assigned to selected nodes."]
    pub replication: crate::types::cluster_stats::ClusterShardMetrics,
    #[doc = "Contains statistics about the number of shards assigned to selected nodes."]
    pub shards: crate::types::cluster_stats::ClusterShardMetrics,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterIngest {
    pub number_of_pipelines: i32,
    pub processor_stats:
        std::collections::BTreeMap<String, crate::types::cluster_stats::ClusterProcessor>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterJvm {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_uptime: Option<crate::types::common::Duration>,
    pub max_uptime_in_millis: crate::types::common::DurationValueUnitMillis,
    pub mem: crate::types::cluster_stats::ClusterJvmMemory,
    #[doc = "Number of active threads in use by JVM across all selected nodes."]
    pub threads: i64,
    #[doc = "Contains statistics about the JVM versions used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::types::cluster_stats::ClusterJvmVersion>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterJvmMemory {
    #[doc = "Maximum amount of memory available for use by the heap across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Maximum amount of memory, in bytes, available for use by the heap across all selected nodes."]
    pub heap_max_in_bytes: crate::types::common::ByteCount,
    #[doc = "Memory currently in use by the heap across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Memory, in bytes, currently in use by the heap across all selected nodes."]
    pub heap_used_in_bytes: crate::types::common::ByteCount,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterJvmVersion {
    #[doc = "Always `true`. All distributions come with a bundled Java Development Kit (JDK)."]
    pub bundled_jdk: bool,
    #[doc = "Total number of selected nodes using JVM."]
    pub count: i32,
    #[doc = "If `true`, a bundled JDK is in use by JVM."]
    pub using_bundled_jdk: bool,
    pub version: crate::types::common::VersionString,
    #[doc = "Name of the JVM."]
    pub vm_name: String,
    #[doc = "Vendor of the JVM."]
    pub vm_vendor: String,
    pub vm_version: crate::types::common::VersionString,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterNetworkTypes {
    #[doc = "Contains statistics about the HTTP network types used by selected nodes."]
    pub http_types: std::collections::BTreeMap<String, i32>,
    #[doc = "Contains statistics about the transport network types used by selected nodes."]
    pub transport_types: std::collections::BTreeMap<String, i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterNodeCount {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager: Option<i32>,
    pub coordinating_only: i32,
    pub data: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_cold: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_content: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_frozen: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_hot: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_warm: Option<i32>,
    pub ingest: i32,
    pub master: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml: Option<i32>,
    pub remote_cluster_client: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<i32>,
    pub total: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voting_only: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warm: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterNodes {
    #[doc = "Contains counts for nodes selected by the request's node filters."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<crate::types::cluster_stats::ClusterNodeCount>,
    #[doc = "Contains statistics about the discovery types used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_types: Option<std::collections::BTreeMap<String, i32>>,
    #[doc = "Contains statistics about file stores by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<crate::types::cluster_stats::ClusterFileSystem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_pressure: Option<crate::types::cluster_stats::IndexingPressure>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::cluster_stats::ClusterIngest>,
    #[doc = "Contains statistics about the Java Virtual Machines (JVMs) used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm: Option<crate::types::cluster_stats::ClusterJvm>,
    #[doc = "Contains statistics about the transport and HTTP networks used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_types: Option<crate::types::cluster_stats::ClusterNetworkTypes>,
    #[doc = "Contains statistics about the operating systems used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<crate::types::cluster_stats::ClusterOperatingSystem>,
    #[doc = "Contains statistics about OpenSearch distributions installed on selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub packaging_types: Option<Vec<crate::types::cluster_stats::NodePackagingType>>,
    #[doc = "Contains statistics about installed plugins and modules by selected nodes. If no plugins or modules are installed, this array is empty."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<crate::types::common::PluginStats>>,
    #[doc = "Contains statistics about processes used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<crate::types::cluster_stats::ClusterProcess>,
    #[doc = "Array of OpenSearch versions used on selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub versions: Option<Vec<crate::types::common::VersionString>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterOperatingSystem {
    #[doc = "Number of processors used to calculate thread pool size across all selected nodes. This number can be set with the processors setting of a node and defaults to the number of processors reported by the operating system. In both cases, this number will never be larger than 32."]
    pub allocated_processors: i32,
    #[doc = "Contains statistics about processor architectures (for example, x86_64 or aarch64) used by selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub architectures: Option<Vec<crate::types::cluster_stats::ClusterOperatingSystemArchitecture>>,
    #[doc = "Number of processors available to JVM across all selected nodes."]
    pub available_processors: i32,
    pub mem: crate::types::cluster_stats::OperatingSystemMemoryInfo,
    #[doc = "Contains statistics about operating systems used by selected nodes."]
    pub names: Vec<crate::types::cluster_stats::ClusterOperatingSystemName>,
    #[doc = "Contains statistics about operating systems used by selected nodes."]
    pub pretty_names: Vec<crate::types::cluster_stats::ClusterOperatingSystemPrettyName>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterOperatingSystemArchitecture {
    #[doc = "Name of an architecture used by one or more selected nodes."]
    pub arch: String,
    #[doc = "Number of selected nodes using the architecture."]
    pub count: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterOperatingSystemName {
    #[doc = "Number of selected nodes using the operating system."]
    pub count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterOperatingSystemPrettyName {
    #[doc = "Number of selected nodes using the operating system."]
    pub count: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pretty_name: Option<crate::types::common::Name>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterProcess {
    pub cpu: crate::types::cluster_stats::ClusterProcessCpu,
    pub open_file_descriptors: crate::types::cluster_stats::ClusterProcessOpenFileDescriptors,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterProcessCpu {
    #[doc = "Percentage of CPU used across all selected nodes. Returns `-1` if not supported."]
    pub percent: crate::types::common::PercentageNumber,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterProcessOpenFileDescriptors {
    #[doc = "Average number of concurrently open file descriptors. Returns `-1` if not supported."]
    pub avg: i64,
    #[doc = "Maximum number of concurrently open file descriptors allowed across all selected nodes. Returns `-1` if not supported."]
    pub max: i64,
    #[doc = "Minimum number of concurrently open file descriptors across all selected nodes. Returns -1 if not supported."]
    pub min: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterProcessor {
    pub count: i64,
    pub current: i64,
    pub failed: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<crate::types::common::Duration>,
    pub time_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterShardMetrics {
    #[doc = "Mean number of shards in an index, counting only shards assigned to selected nodes."]
    pub avg: f64,
    #[doc = "Maximum number of shards in an index, counting only shards assigned to selected nodes."]
    pub max: f64,
    #[doc = "Minimum number of shards in an index, counting only shards assigned to selected nodes."]
    pub min: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldTypes {
    #[doc = "The number of occurrences of the field type in selected nodes."]
    pub count: i32,
    #[doc = "The number of indexes containing the field type in selected nodes."]
    pub index_count: i32,
    pub name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldTypesMappings {
    #[doc = "Contains statistics about field data types used in selected nodes."]
    pub field_types: Vec<crate::types::cluster_stats::FieldTypes>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressure {
    pub memory: crate::types::cluster_stats::IndexingPressureMemory,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressureMemory {
    pub current: crate::types::cluster_stats::IndexingPressureMemorySummary,
    pub limit_in_bytes: crate::types::common::ByteCount,
    pub total: crate::types::cluster_stats::IndexingPressureMemorySummary,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressureMemorySummary {
    pub all_in_bytes: crate::types::common::ByteCount,
    pub combined_coordinating_and_primary_in_bytes: crate::types::common::ByteCount,
    pub coordinating_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coordinating_rejections: Option<f64>,
    pub primary_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_rejections: Option<f64>,
    pub replica_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica_rejections: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesVersions {
    pub index_count: i32,
    pub primary_shard_count: i32,
    pub total_primary_bytes: crate::types::common::ByteCount,
    pub version: crate::types::common::VersionString,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodePackagingType {
    #[doc = "Number of selected nodes using the distribution flavor and file type."]
    pub count: i32,
    #[doc = "Type of OpenSearch distribution. This is always `default`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
    #[doc = "File type (such as `tar` or `zip`) used for the distribution package."]
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemMemoryInfo {
    #[doc = "Total amount, in bytes, of memory across all selected nodes, but using the value specified using the `es.total_memory_bytes` system property instead of measured total memory for those nodes where that system property was set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjusted_total_in_bytes: Option<crate::types::common::ByteCount>,
    #[doc = "Amount, in bytes, of free physical memory across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub free: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount, in bytes, of free physical memory across all selected nodes."]
    pub free_in_bytes: crate::types::common::ByteCount,
    #[doc = "Percentage of free physical memory across all selected nodes."]
    pub free_percent: crate::types::common::PercentageNumber,
    #[doc = "Total amount of physical memory across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Total amount, in bytes, of physical memory across all selected nodes."]
    pub total_in_bytes: crate::types::common::ByteCount,
    #[doc = "Amount of physical memory in use across all selected nodes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub used: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "Amount, in bytes, of physical memory in use across all selected nodes."]
    pub used_in_bytes: crate::types::common::ByteCount,
    #[doc = "Percentage of physical memory in use across all selected nodes."]
    pub used_percent: crate::types::common::PercentageNumber,
}

#[doc = "The response of the `cluster.stats` API"]
pub type StatsResponse = crate::types::cluster::StatsResponse;
