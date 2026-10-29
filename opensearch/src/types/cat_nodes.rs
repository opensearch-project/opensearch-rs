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

//! Response types of the `cat_nodes` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NodesRecord {
    #[doc = "The OpenSearch build hash."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub build: Option<String>,
    #[doc = "The average size in bytes of shard bulk."]
    #[serde(rename = "bulk.avg_size_in_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_avg_size_in_bytes: Option<String>,
    #[doc = "The average time spend in shard bulk."]
    #[serde(rename = "bulk.avg_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_avg_time: Option<String>,
    #[doc = "The number of bulk shard operations."]
    #[serde(rename = "bulk.total_operations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_operations: Option<String>,
    #[doc = "The total size in bytes of shard bulk."]
    #[serde(rename = "bulk.total_size_in_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_size_in_bytes: Option<String>,
    #[doc = "The time spend in shard bulk."]
    #[serde(rename = "bulk.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_time: Option<String>,
    #[doc = "Indicates whether the node is the elected cluster-manager node. Returned values include `*`(elected cluster-manager) and `-`(not elected cluster-manager)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_manager: Option<String>,
    #[doc = "The size of completion."]
    #[serde(rename = "completion.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_size: Option<String>,
    #[doc = "The recent system CPU usage as a percentage."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(rename = "disk.avail")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_avail: Option<String>,
    #[serde(rename = "disk.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_total: Option<String>,
    #[serde(rename = "disk.used")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_used: Option<String>,
    #[serde(rename = "disk.used_percent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk_used_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The field data evictions."]
    #[serde(rename = "fielddata.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_evictions: Option<String>,
    #[doc = "The used field data cache."]
    #[serde(rename = "fielddata.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_memory_size: Option<String>,
    #[doc = "The used file descriptors."]
    #[serde(rename = "file_desc.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_desc_current: Option<String>,
    #[doc = "The maximum number of file descriptors."]
    #[serde(rename = "file_desc.max")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_desc_max: Option<String>,
    #[serde(rename = "file_desc.percent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_desc_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The OpenSearch distribution flavor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flavor: Option<String>,
    #[doc = "The number of flushes."]
    #[serde(rename = "flush.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total: Option<String>,
    #[doc = "The time spent in flush."]
    #[serde(rename = "flush.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total_time: Option<String>,
    #[doc = "The number of current get ops."]
    #[serde(rename = "get.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_current: Option<String>,
    #[doc = "The time spent in successful gets."]
    #[serde(rename = "get.exists_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_time: Option<String>,
    #[doc = "The number of successful get operations."]
    #[serde(rename = "get.exists_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_total: Option<String>,
    #[doc = "The time spent in failed gets."]
    #[serde(rename = "get.missing_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_time: Option<String>,
    #[doc = "The number of failed gets."]
    #[serde(rename = "get.missing_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_total: Option<String>,
    #[doc = "The time spent in get."]
    #[serde(rename = "get.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_time: Option<String>,
    #[doc = "The number of get ops."]
    #[serde(rename = "get.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_total: Option<String>,
    #[doc = "The used heap."]
    #[serde(rename = "heap.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_current: Option<String>,
    #[doc = "The maximum configured heap."]
    #[serde(rename = "heap.max")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_max: Option<String>,
    #[serde(rename = "heap.percent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heap_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The bound HTTP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "The number of current deletions."]
    #[serde(rename = "indexing.delete_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_current: Option<String>,
    #[doc = "The time spent in deletions."]
    #[serde(rename = "indexing.delete_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_time: Option<String>,
    #[doc = "The number of delete operations."]
    #[serde(rename = "indexing.delete_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_total: Option<String>,
    #[doc = "The number of current indexing operations."]
    #[serde(rename = "indexing.index_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_current: Option<String>,
    #[doc = "The number of failed indexing operations."]
    #[serde(rename = "indexing.index_failed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_failed: Option<String>,
    #[doc = "The time spent in indexing."]
    #[serde(rename = "indexing.index_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_time: Option<String>,
    #[doc = "The number of indexing operations."]
    #[serde(rename = "indexing.index_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_total: Option<String>,
    #[doc = "The IP address."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The Java version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub jdk: Option<String>,
    #[doc = "The load average for the last fifteen minutes."]
    #[serde(rename = "load_15m")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_1_5m: Option<String>,
    #[doc = "The load average for the most recent minute."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_1m: Option<String>,
    #[doc = "The load average for the last five minutes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub load_5m: Option<String>,
    #[doc = "Indicates whether the node is the elected cluster-manager node. Returned values include `*`(elected cluster manager) and `-`(not elected cluster manager)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub master: Option<String>,
    #[doc = "The number of current merges."]
    #[serde(rename = "merges.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current: Option<String>,
    #[doc = "The number of current merging docs."]
    #[serde(rename = "merges.current_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_docs: Option<String>,
    #[doc = "The size of current merges."]
    #[serde(rename = "merges.current_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_size: Option<String>,
    #[doc = "The number of completed merge operations."]
    #[serde(rename = "merges.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total: Option<String>,
    #[doc = "The docs merged."]
    #[serde(rename = "merges.total_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_docs: Option<String>,
    #[doc = "The size merged."]
    #[serde(rename = "merges.total_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_size: Option<String>,
    #[doc = "The time spent in merges."]
    #[serde(rename = "merges.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common::Name>,
    #[doc = "The roles of the node. Returned values include `c`(cold node), `d`(data node), `f`(frozen node), `h`(hot node), `i`(ingest node), `l`(machine learning node), `m` (cluster-manager eligible node), `r`(remote cluster client node), `s`(content node), `t`(transform node), `v`(voting-only node), `w`(warm node),and `-`(coordinating node only)."]
    #[serde(rename = "node.role")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_role: Option<String>,
    #[doc = "A comma-separated list of the roles of the node. Returned values may include but are not limited to: `data`, `ingest`, `master`, `cluster_manager`, `remote_cluster_client`, `search`, and `-`(coordinating node only)."]
    #[serde(rename = "node.roles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_roles: Option<String>,
    #[doc = "The process identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pid: Option<String>,
    #[doc = "The bound transport port."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<String>,
    #[doc = "The query cache evictions."]
    #[serde(rename = "query_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_evictions: Option<String>,
    #[doc = "The query cache hit counts."]
    #[serde(rename = "query_cache.hit_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_hit_count: Option<String>,
    #[doc = "The used query cache."]
    #[serde(rename = "query_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_memory_size: Option<String>,
    #[doc = "The query cache miss counts."]
    #[serde(rename = "query_cache.miss_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_miss_count: Option<String>,
    #[doc = "The used machine memory."]
    #[serde(rename = "ram.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ram_current: Option<String>,
    #[doc = "The total machine memory."]
    #[serde(rename = "ram.max")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ram_max: Option<String>,
    #[serde(rename = "ram.percent")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ram_percent: Option<crate::types::common::PercentageString>,
    #[doc = "The time spent in external refreshes."]
    #[serde(rename = "refresh.external_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_external_time: Option<String>,
    #[doc = "The total external refreshes."]
    #[serde(rename = "refresh.external_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_external_total: Option<String>,
    #[doc = "The number of pending refresh listeners."]
    #[serde(rename = "refresh.listeners")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_listeners: Option<String>,
    #[doc = "The time spent in refreshes."]
    #[serde(rename = "refresh.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<String>,
    #[doc = "The total refreshes."]
    #[serde(rename = "refresh.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_total: Option<String>,
    #[doc = "The request cache evictions."]
    #[serde(rename = "request_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_evictions: Option<String>,
    #[doc = "The request cache hit counts."]
    #[serde(rename = "request_cache.hit_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_hit_count: Option<String>,
    #[doc = "The used request cache."]
    #[serde(rename = "request_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_memory_size: Option<String>,
    #[doc = "The request cache miss counts."]
    #[serde(rename = "request_cache.miss_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_miss_count: Option<String>,
    #[doc = "The total compiled scripts evicted from the cache."]
    #[serde(rename = "script.cache_evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_cache_evictions: Option<String>,
    #[doc = "The script cache compilation limit triggered."]
    #[serde(rename = "script.compilation_limit_triggered")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_compilation_limit_triggered: Option<String>,
    #[doc = "The total script compilations."]
    #[serde(rename = "script.compilations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script_compilations: Option<String>,
    #[serde(rename = "search.concurrent_avg_slice_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_concurrent_avg_slice_count: Option<String>,
    #[serde(rename = "search.concurrent_query_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_concurrent_query_current: Option<String>,
    #[serde(rename = "search.concurrent_query_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_concurrent_query_time: Option<String>,
    #[serde(rename = "search.concurrent_query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_concurrent_query_total: Option<String>,
    #[doc = "The current fetch phase operations."]
    #[serde(rename = "search.fetch_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_current: Option<String>,
    #[doc = "The time spent in fetch phase."]
    #[serde(rename = "search.fetch_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_time: Option<String>,
    #[doc = "The total fetch operations."]
    #[serde(rename = "search.fetch_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_total: Option<String>,
    #[doc = "The open search contexts."]
    #[serde(rename = "search.open_contexts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_open_contexts: Option<String>,
    #[serde(rename = "search.point_in_time_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_point_in_time_current: Option<String>,
    #[serde(rename = "search.point_in_time_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_point_in_time_time: Option<String>,
    #[serde(rename = "search.point_in_time_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_point_in_time_total: Option<String>,
    #[doc = "The current query phase operations."]
    #[serde(rename = "search.query_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_current: Option<String>,
    #[doc = "The time spent in query phase."]
    #[serde(rename = "search.query_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_time: Option<String>,
    #[doc = "The total query phase operations."]
    #[serde(rename = "search.query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_total: Option<String>,
    #[doc = "The open scroll contexts."]
    #[serde(rename = "search.scroll_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_current: Option<String>,
    #[doc = "The time scroll contexts held open."]
    #[serde(rename = "search.scroll_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_time: Option<String>,
    #[doc = "The completed scroll contexts."]
    #[serde(rename = "search.scroll_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_total: Option<String>,
    #[doc = "The number of segments."]
    #[serde(rename = "segments.count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_count: Option<String>,
    #[doc = "The memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields."]
    #[serde(rename = "segments.fixed_bitset_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_fixed_bitset_memory: Option<String>,
    #[doc = "The memory used by the index writer."]
    #[serde(rename = "segments.index_writer_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_index_writer_memory: Option<String>,
    #[doc = "The memory used by segments."]
    #[serde(rename = "segments.memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_memory: Option<String>,
    #[doc = "The memory used by the version map."]
    #[serde(rename = "segments.version_map_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_version_map_memory: Option<String>,
    #[doc = "The number of current suggest operations."]
    #[serde(rename = "suggest.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_current: Option<String>,
    #[doc = "The time spend in suggest."]
    #[serde(rename = "suggest.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_time: Option<String>,
    #[doc = "The number of suggest operations."]
    #[serde(rename = "suggest.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_total: Option<String>,
    #[doc = "The OpenSearch distribution type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[doc = "The node uptime."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uptime: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[doc = "The response of the `cat.nodes` API"]
pub type NodesResponse = crate::types::cat::NodesResponse;
