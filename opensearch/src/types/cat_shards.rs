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

//! Response types of the `cat_shards` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardsRecord {
    #[doc = "The average size in bytes of shard bulk operations."]
    #[serde(rename = "bulk.avg_size_in_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_avg_size_in_bytes: Option<String>,
    #[doc = "The average time spent in shard bulk operations."]
    #[serde(rename = "bulk.avg_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_avg_time: Option<String>,
    #[doc = "The number of bulk shard operations."]
    #[serde(rename = "bulk.total_operations")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_operations: Option<String>,
    #[doc = "The total size in bytes of shard bulk operations."]
    #[serde(rename = "bulk.total_size_in_bytes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_size_in_bytes: Option<String>,
    #[doc = "The time spent in shard bulk operations."]
    #[serde(rename = "bulk.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bulk_total_time: Option<String>,
    #[doc = "The size of completion."]
    #[serde(rename = "completion.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_size: Option<String>,
    #[doc = "The number of documents in the shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs: Option<String>,
    #[serde(rename = "docs.deleted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,
    #[doc = "The field data cache evictions."]
    #[serde(rename = "fielddata.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_evictions: Option<String>,
    #[doc = "The used field data cache memory."]
    #[serde(rename = "fielddata.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_memory_size: Option<String>,
    #[doc = "The number of flushes."]
    #[serde(rename = "flush.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total: Option<String>,
    #[doc = "The time spent in flush."]
    #[serde(rename = "flush.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total_time: Option<String>,
    #[doc = "The number of current get operations."]
    #[serde(rename = "get.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_current: Option<String>,
    #[doc = "The time spent in successful get operations."]
    #[serde(rename = "get.exists_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_time: Option<String>,
    #[doc = "The number of successful get operations."]
    #[serde(rename = "get.exists_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_total: Option<String>,
    #[doc = "The time spent in failed get operations."]
    #[serde(rename = "get.missing_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_time: Option<String>,
    #[doc = "The number of failed get operations."]
    #[serde(rename = "get.missing_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_total: Option<String>,
    #[doc = "The time spent in get operations."]
    #[serde(rename = "get.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_time: Option<String>,
    #[doc = "The number of get operations."]
    #[serde(rename = "get.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_total: Option<String>,
    #[doc = "The unique identifier for the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "The index name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The number of current deletion operations."]
    #[serde(rename = "indexing.delete_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_current: Option<String>,
    #[doc = "The time spent in deletion operations."]
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
    #[doc = "The time spent in indexing operations."]
    #[serde(rename = "indexing.index_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_time: Option<String>,
    #[doc = "The number of indexing operations."]
    #[serde(rename = "indexing.index_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_total: Option<String>,
    #[doc = "The IP address of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The number of current merge operations."]
    #[serde(rename = "merges.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current: Option<String>,
    #[doc = "The number of current merging documents."]
    #[serde(rename = "merges.current_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_docs: Option<String>,
    #[doc = "The size of current merge operations."]
    #[serde(rename = "merges.current_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_size: Option<String>,
    #[doc = "The number of completed merge operations."]
    #[serde(rename = "merges.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total: Option<String>,
    #[doc = "The number of merged documents."]
    #[serde(rename = "merges.total_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_docs: Option<String>,
    #[doc = "The size of current merges."]
    #[serde(rename = "merges.total_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_size: Option<String>,
    #[doc = "The time spent merging documents."]
    #[serde(rename = "merges.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_time: Option<String>,
    #[doc = "The name of node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node: Option<String>,
    #[doc = "The shard data path."]
    #[serde(rename = "path.data")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_data: Option<String>,
    #[doc = "The shard state path."]
    #[serde(rename = "path.state")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_state: Option<String>,
    #[doc = "The shard type: `primary` or `replica`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prirep: Option<String>,
    #[doc = "The query cache evictions."]
    #[serde(rename = "query_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_evictions: Option<String>,
    #[doc = "The used query cache memory."]
    #[serde(rename = "query_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_memory_size: Option<String>,
    #[doc = "The type of recovery source."]
    #[serde(rename = "recoverysource.type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recoverysource_type: Option<String>,
    #[doc = "The time spent in external refreshes."]
    #[serde(rename = "refresh.external_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_external_time: Option<String>,
    #[doc = "The total number of external refreshes."]
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
    #[doc = "The total number of refreshes."]
    #[serde(rename = "refresh.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_total: Option<String>,
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
    #[doc = "The total number of fetch operations."]
    #[serde(rename = "search.fetch_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_total: Option<String>,
    #[doc = "The number of open search contexts."]
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
    #[doc = "The total number of query phase operations."]
    #[serde(rename = "search.query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_total: Option<String>,
    #[doc = "The open scroll contexts."]
    #[serde(rename = "search.scroll_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_current: Option<String>,
    #[doc = "The time scroll contexts were held open."]
    #[serde(rename = "search.scroll_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_time: Option<String>,
    #[doc = "The number of completed scroll contexts."]
    #[serde(rename = "search.scroll_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_total: Option<String>,
    #[serde(rename = "search.search_idle_reactivate_count_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_search_idle_reactivate_count_total: Option<String>,
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
    #[doc = "The global checkpoint."]
    #[serde(rename = "seq_no.global_checkpoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no_global_checkpoint: Option<String>,
    #[doc = "The local checkpoint."]
    #[serde(rename = "seq_no.local_checkpoint")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no_local_checkpoint: Option<String>,
    #[doc = "The maximum sequence number."]
    #[serde(rename = "seq_no.max")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no_max: Option<String>,
    #[doc = "The shard name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[doc = "The shard state. Returned values include: `INITIALIZING`: The shard is recovering from a peer shard or gateway. `RELOCATING`: The shard is relocating. `STARTED`: The shard has started. `UNASSIGNED`: The shard is not assigned to any node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[doc = "The disk space used by the shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<String>,
    #[doc = "The sync identifier."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_id: Option<String>,
    #[doc = "The time at which the shard became unassigned in Coordinated Universal Time (UTC)."]
    #[serde(rename = "unassigned.at")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_at: Option<String>,
    #[doc = "Additional details as to why the shard became unassigned. It does not explain why the shard is not assigned; use the cluster allocation explain API for that information."]
    #[serde(rename = "unassigned.details")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_details: Option<String>,
    #[doc = "The time at which the shard was requested to be unassigned in Coordinated Universal Time (UTC)."]
    #[serde(rename = "unassigned.for")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_for: Option<String>,
    #[doc = "The reason for the last change to the state of an unassigned shard. It does not explain why the shard is currently unassigned; use the cluster allocation explain API for that information. Returned values include: `ALLOCATION_FAILED`: Unassigned as a result of a failed allocation of the shard. `CLUSTER_RECOVERED`: Unassigned as a result of a full cluster recovery. `DANGLING_INDEX_IMPORTED`: Unassigned as a result of importing a dangling index. `EXISTING_INDEX_RESTORED`: Unassigned as a result of restoring into a closed index. `FORCED_EMPTY_PRIMARY`: The shard's allocation was last modified by forcing an empty primary using the cluster reroute API. `INDEX_CLOSED`: Unassigned because the index was closed. `INDEX_CREATED`: Unassigned as a result of an API creation of an index. `INDEX_REOPENED`: Unassigned as a result of opening a closed index. `MANUAL_ALLOCATION`: The shard's allocation was last modified by the cluster reroute API. `NEW_INDEX_RESTORED`: Unassigned as a result of restoring into a new index. `NODE_LEFT`: Unassigned as a result of the node hosting it leaving the cluster. `NODE_RESTARTING`: Similar to `NODE_LEFT`, except that the node was registered as restarting using the node shutdown API. `PRIMARY_FAILED`: The shard was initializing as a replica, but the primary shard failed before the initialization completed. `REALLOCATED_REPLICA`: A better replica location is identified and causes the existing replica allocation to be cancelled. `REINITIALIZED`: When a shard moves from started back to initializing. `REPLICA_ADDED`: Unassigned as a result of explicit addition of a replica. `REROUTE_CANCELLED`: Unassigned as a result of explicit cancel reroute command."]
    #[serde(rename = "unassigned.reason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unassigned_reason: Option<String>,
    #[doc = "The number of current warmer operations."]
    #[serde(rename = "warmer.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_current: Option<String>,
    #[doc = "The total number of warmer operations."]
    #[serde(rename = "warmer.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_total: Option<String>,
    #[doc = "The time spent in warmer operations."]
    #[serde(rename = "warmer.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_total_time: Option<String>,
}

#[doc = "The response of the `cat.shards` API"]
pub type ShardsResponse = crate::types::cat::ShardsResponse;
