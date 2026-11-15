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

//! Response types of the `nodes_info` specification namespace

#![allow(missing_docs)]

#[doc = "The response of the `nodes.info` API"]
pub type InfoResponse = crate::types::nodes::InfoResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfo {
    #[doc = "Information about available aggregation types."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregations:
        Option<std::collections::BTreeMap<String, crate::types::nodes_info::NodeInfoAggregation>>,
    #[doc = "The custom attributes of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build_flavor: Option<String>,
    #[doc = "Short hash of the last git commit in this release."]
    pub build_hash: String,
    #[doc = "The type of build, such as `tar`, `docker`, `rpm`."]
    pub build_type: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<crate::types::common::Host>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<crate::types::nodes_info::NodeInfoHttp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::nodes_info::NodeInfoIngest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<crate::types::common::Ip>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jvm: Option<crate::types::nodes_info::NodeJvmInfo>,
    #[doc = "The list of modules installed on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modules: Option<Vec<crate::types::common::PluginStats>>,
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::types::nodes_info::NodeInfoNetwork>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub os: Option<crate::types::nodes_info::NodeOperatingSystemInfo>,
    #[doc = "The list of plugins installed on the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<Vec<crate::types::common::PluginStats>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process: Option<crate::types::nodes_info::NodeProcessInfo>,
    pub roles: crate::types::common::NodeRoles,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_pipelines: Option<crate::types::nodes_info::NodeInfoSearchPipelines>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::types::nodes_info::NodeInfoSettings>,
    #[doc = "Information about thread pool configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thread_pool:
        Option<std::collections::BTreeMap<String, crate::types::nodes_info::NodeThreadPoolInfo>>,
    #[doc = "Total heap allowed to be used to hold recently indexed documents before they must be written to disk. This size is a shared pool across all shards on this node, and is controlled by Indexing Buffer settings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_indexing_buffer: Option<crate::types::nodes_info::NodeInfoTotalIndexingBuffer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_indexing_buffer_in_bytes:
        Option<crate::types::nodes_info::NodeInfoTotalIndexingBufferInBytes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<crate::types::nodes_info::NodeInfoTransport>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_address: Option<crate::types::common::TransportAddress>,
    pub version: crate::types::common::VersionString,
}

#[doc = "The action-related configuration settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoAction {
    #[doc = "Whether destructive actions require explicit names."]
    pub destructive_requires_name: String,
}

#[doc = "The aggregation configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoAggregation {
    #[doc = "The list of supported aggregation types."]
    pub types: Vec<String>,
}

#[doc = "The bootstrap configuration settings for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoBootstrap {
    #[doc = "The memory lock setting for the bootstrap process."]
    pub memory_lock: String,
}

#[doc = "The client node configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoClient {
    #[doc = "The type of client node configuration."]
    pub r#type: String,
}

#[doc = "The node discovery configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoDiscovery {
    #[doc = "The configured seed hosts for node discovery."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seed_hosts: Option<String>,
    #[doc = "The discovery mechanism type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

#[doc = "Provides HTTP connection information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoHttp {
    #[doc = "The addresses the node is bound to for HTTP traffic."]
    pub bound_address: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_content_length: Option<crate::types::common::HumanReadableByteCount>,
    pub max_content_length_in_bytes: crate::types::common::ByteCount,
    #[doc = "The HTTP address published to other nodes."]
    pub publish_address: String,
}

#[doc = "The ingest pipeline configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoIngest {
    #[doc = "The list of configured ingest processors."]
    pub processors: Vec<crate::types::nodes_info::NodeInfoIngestProcessor>,
}

#[doc = "The ingest processor download configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoIngestDownloader {
    #[doc = "Whether the downloader is enabled."]
    pub enabled: String,
}

#[doc = "The ingest processor configuration details."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoIngestInfo {
    pub downloader: crate::types::nodes_info::NodeInfoIngestDownloader,
}

#[doc = "The ingest processor configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoIngestProcessor {
    #[doc = "The type of ingest processor."]
    pub r#type: String,
}

#[doc = "Provides JVM memory settings and statistics."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoJvmMemory {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direct_max: Option<crate::types::common::HumanReadableByteCount>,
    pub direct_max_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_init: Option<crate::types::common::HumanReadableByteCount>,
    pub heap_init_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_max: Option<crate::types::common::HumanReadableByteCount>,
    pub heap_max_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_init: Option<crate::types::common::HumanReadableByteCount>,
    pub non_heap_init_in_bytes: crate::types::common::ByteCount,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub non_heap_max: Option<crate::types::common::HumanReadableByteCount>,
    pub non_heap_max_in_bytes: crate::types::common::ByteCount,
}

#[doc = "Provides memory information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoMemory {
    pub total: crate::types::common::HumanReadableByteCount,
    pub total_in_bytes: crate::types::common::ByteCount,
}

#[doc = "Provides network-related information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoNetwork {
    pub primary_interface: crate::types::nodes_info::NodeInfoNetworkInterface,
    #[doc = "The interval for refreshing network information."]
    pub refresh_interval: i32,
}

#[doc = "Describes a network interface on the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoNetworkInterface {
    #[doc = "The IP address of the interface."]
    pub address: String,
    #[doc = "The MAC address of the interface."]
    pub mac_address: String,
    pub name: crate::types::common::Name,
}

#[doc = "Provides CPU information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoOSCPU {
    pub cache_size: crate::types::common::HumanReadableByteCount,
    pub cache_size_in_bytes: crate::types::common::ByteCount,
    #[doc = "The number of cores per CPU socket."]
    pub cores_per_socket: i32,
    #[doc = "The CPU clock speed in MHz."]
    pub mhz: i32,
    #[doc = "The CPU model name."]
    pub model: String,
    #[doc = "The total number of CPU cores."]
    pub total_cores: i32,
    #[doc = "The total number of CPU sockets."]
    pub total_sockets: i32,
    #[doc = "The CPU vendor name."]
    pub vendor: String,
}

#[doc = "The `filesystem` path configurations for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoPath {
    #[doc = "The paths configured for data storage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    #[doc = "The OpenSearch home directory path."]
    pub home: String,
    #[doc = "The path to node log files."]
    pub logs: String,
    #[doc = "The paths configured for snapshot repositories."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repo: Option<Vec<String>>,
}

#[doc = "The repository configuration information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoRepositories {
    pub url: crate::types::nodes_info::NodeInfoRepositoriesUrl,
}

#[doc = "Specifies allowed repository URLs."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoRepositoriesUrl {
    #[doc = "The pattern for allowed repository URLs."]
    pub allowed_urls: String,
}

#[doc = "The script execution configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoScript {
    #[doc = "The types of scripts allowed."]
    pub allowed_types: String,
    #[doc = "The setting to disable maximum script compilation rate."]
    pub disable_max_compilations_rate: String,
}

#[doc = "The search operation configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSearch {
    pub remote: crate::types::nodes_info::NodeInfoSearchRemote,
}

#[doc = "The search pipeline processor configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSearchPipelineProcessor {
    #[doc = "The type of search pipeline processor."]
    pub r#type: String,
}

#[doc = "The search pipeline configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSearchPipelines {
    #[doc = "The list of phase results processors in the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_results_processors:
        Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
    #[doc = "The list of request processors in the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_processors: Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
    #[doc = "The list of response processors in the pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_processors: Option<Vec<crate::types::nodes_info::NodeInfoSearchPipelineProcessor>>,
}

#[doc = "The remote search connection configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSearchRemote {
    #[doc = "The remote connection settings."]
    pub connect: String,
}

#[doc = "The configuration settings for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettings {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::types::nodes_info::NodeInfoAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bootstrap: Option<crate::types::nodes_info::NodeInfoBootstrap>,
    pub client: crate::types::nodes_info::NodeInfoClient,
    pub cluster: crate::types::nodes_info::NodeInfoSettingsCluster,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery: Option<crate::types::nodes_info::NodeInfoDiscovery>,
    pub http: crate::types::nodes_info::NodeInfoSettingsHttp,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::nodes_info::NodeInfoSettingsIndex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ingest: Option<crate::types::nodes_info::NodeInfoSettingsIngest>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network: Option<crate::types::nodes_info::NodeInfoSettingsNetwork>,
    pub node: crate::types::nodes_info::NodeInfoSettingsNode,
    pub path: crate::types::nodes_info::NodeInfoPath,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<crate::types::nodes_info::NodeInfoSettingsPlugins>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repositories: Option<crate::types::nodes_info::NodeInfoRepositories>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::nodes_info::NodeInfoScript>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<crate::types::nodes_info::NodeInfoSearch>,
    pub transport: crate::types::nodes_info::NodeInfoSettingsTransport,
}

#[doc = "Provides cluster-level settings information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsCluster {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deprecation_indexing: Option<crate::types::nodes_info::NodeInfoSettingsDeprecationIndexing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub election: Option<crate::types::nodes_info::NodeInfoSettingsClusterElection>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_cluster_manager_nodes: Option<crate::types::common::StringOrStringArray>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_master_nodes: Option<crate::types::common::StringOrStringArray>,
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::indices_common::IndexRouting>,
}

#[doc = "The cluster election strategy configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsClusterElection {
    pub strategy: crate::types::common::Name,
}

#[doc = "Controls deprecation notice indexing behavior."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsDeprecationIndexing {
    pub enabled: crate::types::common::StringifiedBoolean,
}

#[doc = "The HTTP server configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsHttp {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<crate::types::common::StringifiedInteger>,
    pub r#type: crate::types::nodes_info::NodeInfoSettingsHttpType,
    #[doc = "The default HTTP server type."]
    #[serde(rename = "type.default")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_default: Option<String>,
}

#[doc = "The HTTP server type configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoSettingsHttpType {
    Type(String),
    Config(crate::types::nodes_info::NodeInfoSettingsHttpTypeConfig),
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsHttpType {
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
            Some(b'"') => serde_json::from_str(text).map(NodeInfoSettingsHttpType::Type),
            Some(b'{') => serde_json::from_str(text).map(NodeInfoSettingsHttpType::Config),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "NodeInfoSettingsHttpType"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The HTTP server type configuration details."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsHttpTypeConfig {
    #[doc = "The default HTTP server implementation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[doc = "The index-level settings configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsIndex {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<crate::types::nodes_info::NodeInfoSettingsIndexStore>,
}

#[doc = "The hybrid store configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsIndexHybrid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mmap: Option<crate::types::nodes_info::NodeInfoSettingsIndexStoreMmap>,
}

#[doc = "The index store configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsIndexStore {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<crate::types::nodes_info::NodeInfoSettingsIndexHybrid>,
}

#[doc = "The memory-mapped store configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsIndexStoreMmap {
    #[doc = "The file extensions configured for memory mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
}

#[doc = "The ingest processor configurations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsIngest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub append: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circle: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csv: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_index_name: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dissect: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot_expander: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enrich: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreach: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geoip: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grok: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gsub: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inference: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kv: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_security_user: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uppercase: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urldecode: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<crate::types::nodes_info::NodeInfoIngestInfo>,
}

#[doc = "The network configuration settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsNetwork {
    pub host: crate::types::common::Host,
}

#[doc = "The node-specific configuration settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsNode {
    #[doc = "The custom attributes assigned to the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attr: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The maximum number of nodes allowed to store data locally"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_local_storage_nodes: Option<String>,
    pub name: crate::types::common::Name,
}

#[doc = "The plugin-specific settings."]
pub type NodeInfoSettingsPlugins =
    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The transport layer configuration settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsTransport {
    pub r#type: crate::types::nodes_info::NodeInfoSettingsTransportType,
    #[doc = "The default transport type."]
    #[serde(rename = "type.default")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_default: Option<String>,
}

#[doc = "The transport layer type configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoSettingsTransportType {
    Type(String),
    Config(crate::types::nodes_info::NodeInfoSettingsTransportTypeConfig),
}
impl<'de> serde::Deserialize<'de> for NodeInfoSettingsTransportType {
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
            Some(b'"') => serde_json::from_str(text).map(NodeInfoSettingsTransportType::Type),
            Some(b'{') => serde_json::from_str(text).map(NodeInfoSettingsTransportType::Config),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "NodeInfoSettingsTransportType"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The detailed transport type configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoSettingsTransportTypeConfig {
    #[doc = "The default transport implementation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
}

#[doc = "Total heap allowed to be used to hold recently indexed documents before they must be written to disk. This size is a shared pool across all shards on this node, and is controlled by Indexing Buffer settings."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoTotalIndexingBuffer {
    ByteCount(crate::types::common::ByteCount),
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
}
impl<'de> serde::Deserialize<'de> for NodeInfoTotalIndexingBuffer {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(NodeInfoTotalIndexingBuffer::ByteCount)
            }
            Some(b'"') => {
                serde_json::from_str(text).map(NodeInfoTotalIndexingBuffer::HumanReadableByteCount)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "NodeInfoTotalIndexingBuffer"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum NodeInfoTotalIndexingBufferInBytes {
    HumanReadableByteCount(crate::types::common::HumanReadableByteCount),
    ByteCount(crate::types::common::ByteCount),
}
impl<'de> serde::Deserialize<'de> for NodeInfoTotalIndexingBufferInBytes {
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
            Some(b'"') => serde_json::from_str(text)
                .map(NodeInfoTotalIndexingBufferInBytes::HumanReadableByteCount),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(NodeInfoTotalIndexingBufferInBytes::ByteCount)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "NodeInfoTotalIndexingBufferInBytes"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The transport layer information."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeInfoTransport {
    #[doc = "The addresses the node is bound to for transport traffic."]
    pub bound_address: Vec<String>,
    #[doc = "The configured transport profiles."]
    pub profiles: std::collections::BTreeMap<String, String>,
    #[doc = "The published transport address."]
    pub publish_address: String,
}

#[doc = "Provides JVM-specific information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeJvmInfo {
    #[doc = "Whether the JDK is bundled with OpenSearch."]
    pub bundled_jdk: bool,
    #[doc = "The garbage collectors enabled in the JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gc_collectors: Option<Vec<String>>,
    #[doc = "The command line arguments passed to the JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub input_arguments: Option<Vec<String>>,
    pub mem: crate::types::nodes_info::NodeInfoJvmMemory,
    #[doc = "The memory pools available to the JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_pools: Option<Vec<String>>,
    #[doc = "The process identifier of the JVM."]
    pub pid: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::DateTime>,
    pub start_time_in_millis: crate::types::common::EpochTimeUnitMillis,
    #[doc = "Whether the node is using the bundled JDK."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub using_bundled_jdk: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub using_compressed_ordinary_object_pointers: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_name: Option<crate::types::common::Name>,
    #[doc = "The vendor of the JVM."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_vendor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vm_version: Option<crate::types::common::VersionString>,
}

#[doc = "Provides operating system information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeOperatingSystemInfo {
    #[doc = "The number of processors actually used to calculate thread pool size. This number can be set with the `node.processors` setting of a node and defaults to the number of processors reported by the OS."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocated_processors: Option<i32>,
    #[doc = "Name of the JVM architecture, for example `amd64` or `x86`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub arch: Option<String>,
    #[doc = "The number of processors available to the Java virtual machine."]
    pub available_processors: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<crate::types::nodes_info::NodeInfoOSCPU>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mem: Option<crate::types::nodes_info::NodeInfoMemory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pretty_name: Option<crate::types::common::Name>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<crate::types::common::Duration>,
    pub refresh_interval_in_millis: crate::types::common::DurationValueUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub swap: Option<crate::types::nodes_info::NodeInfoMemory>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[doc = "Provides process-related information for the node."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeProcessInfo {
    #[doc = "The process identifier (PID)"]
    pub id: i64,
    #[doc = "Indicates if the process address space has been successfully locked in memory."]
    pub mlockall: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_interval: Option<crate::types::common::Duration>,
    pub refresh_interval_in_millis: crate::types::common::DurationValueUnitMillis,
}

#[doc = "The thread pool configuration details."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodeThreadPoolInfo {
    #[doc = "The core number of threads."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub core: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<crate::types::common::Duration>,
    #[doc = "The maximum number of threads."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[doc = "The size of the task queue."]
    pub queue_size: i32,
    #[doc = "The current size of the thread pool."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    #[doc = "The type of thread pool executor."]
    pub r#type: String,
}
