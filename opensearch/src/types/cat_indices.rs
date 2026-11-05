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

//! Response types of the `cat_indices` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesRecord {
    #[doc = "size of completion"]
    #[serde(rename = "completion.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completion_size: Option<String>,
    #[doc = "index creation date (millisecond value)"]
    #[serde(rename = "creation.date")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date: Option<String>,
    #[doc = "index creation date (as string)"]
    #[serde(rename = "creation.date.string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_date_string: Option<String>,
    #[doc = "available docs"]
    #[serde(rename = "docs.count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_count: Option<String>,
    #[doc = "deleted docs"]
    #[serde(rename = "docs.deleted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,
    #[doc = "field data evictions"]
    #[serde(rename = "fielddata.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_evictions: Option<String>,
    #[doc = "used field data cache"]
    #[serde(rename = "fielddata.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_memory_size: Option<String>,
    #[doc = "number of flushes"]
    #[serde(rename = "flush.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total: Option<String>,
    #[doc = "time spent in flush"]
    #[serde(rename = "flush.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_total_time: Option<String>,
    #[doc = "number of current get ops"]
    #[serde(rename = "get.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_current: Option<String>,
    #[doc = "time spent in successful gets"]
    #[serde(rename = "get.exists_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_time: Option<String>,
    #[doc = "number of successful gets"]
    #[serde(rename = "get.exists_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_exists_total: Option<String>,
    #[doc = "time spent in failed gets"]
    #[serde(rename = "get.missing_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_time: Option<String>,
    #[doc = "number of failed gets"]
    #[serde(rename = "get.missing_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_missing_total: Option<String>,
    #[doc = "time spent in get"]
    #[serde(rename = "get.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_time: Option<String>,
    #[doc = "number of get ops"]
    #[serde(rename = "get.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub get_total: Option<String>,
    #[doc = "current health status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health: Option<String>,
    #[doc = "index name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "number of current deletions"]
    #[serde(rename = "indexing.delete_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_current: Option<String>,
    #[doc = "time spent in deletions"]
    #[serde(rename = "indexing.delete_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_time: Option<String>,
    #[doc = "number of delete ops"]
    #[serde(rename = "indexing.delete_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_delete_total: Option<String>,
    #[doc = "number of current indexing ops"]
    #[serde(rename = "indexing.index_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_current: Option<String>,
    #[doc = "number of failed indexing ops"]
    #[serde(rename = "indexing.index_failed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_failed: Option<String>,
    #[doc = "time spent in indexing"]
    #[serde(rename = "indexing.index_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_time: Option<String>,
    #[doc = "number of indexing ops"]
    #[serde(rename = "indexing.index_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_index_total: Option<String>,
    #[doc = "total used memory"]
    #[serde(rename = "memory.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_total: Option<String>,
    #[doc = "number of current merges"]
    #[serde(rename = "merges.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current: Option<String>,
    #[doc = "number of current merging docs"]
    #[serde(rename = "merges.current_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_docs: Option<String>,
    #[doc = "size of current merges"]
    #[serde(rename = "merges.current_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_current_size: Option<String>,
    #[doc = "number of completed merge ops"]
    #[serde(rename = "merges.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total: Option<String>,
    #[doc = "docs merged"]
    #[serde(rename = "merges.total_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_docs: Option<String>,
    #[doc = "size merged"]
    #[serde(rename = "merges.total_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_size: Option<String>,
    #[doc = "time spent in merges"]
    #[serde(rename = "merges.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merges_total_time: Option<String>,
    #[doc = "number of primary shards"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri: Option<String>,
    #[doc = "size of completion"]
    #[serde(rename = "pri.completion.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_completion_size: Option<String>,
    #[doc = "field data evictions"]
    #[serde(rename = "pri.fielddata.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_fielddata_evictions: Option<String>,
    #[doc = "used field data cache"]
    #[serde(rename = "pri.fielddata.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_fielddata_memory_size: Option<String>,
    #[doc = "number of flushes"]
    #[serde(rename = "pri.flush.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_flush_total: Option<String>,
    #[doc = "time spent in flush"]
    #[serde(rename = "pri.flush.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_flush_total_time: Option<String>,
    #[doc = "number of current get ops"]
    #[serde(rename = "pri.get.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_current: Option<String>,
    #[doc = "time spent in successful gets"]
    #[serde(rename = "pri.get.exists_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_exists_time: Option<String>,
    #[doc = "number of successful gets"]
    #[serde(rename = "pri.get.exists_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_exists_total: Option<String>,
    #[doc = "time spent in failed gets"]
    #[serde(rename = "pri.get.missing_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_missing_time: Option<String>,
    #[doc = "number of failed gets"]
    #[serde(rename = "pri.get.missing_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_missing_total: Option<String>,
    #[doc = "time spent in get"]
    #[serde(rename = "pri.get.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_time: Option<String>,
    #[doc = "number of get ops"]
    #[serde(rename = "pri.get.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_get_total: Option<String>,
    #[doc = "number of current deletions"]
    #[serde(rename = "pri.indexing.delete_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_delete_current: Option<String>,
    #[doc = "time spent in deletions"]
    #[serde(rename = "pri.indexing.delete_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_delete_time: Option<String>,
    #[doc = "number of delete ops"]
    #[serde(rename = "pri.indexing.delete_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_delete_total: Option<String>,
    #[doc = "number of current indexing ops"]
    #[serde(rename = "pri.indexing.index_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_index_current: Option<String>,
    #[doc = "number of failed indexing ops"]
    #[serde(rename = "pri.indexing.index_failed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_index_failed: Option<String>,
    #[doc = "time spent in indexing"]
    #[serde(rename = "pri.indexing.index_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_index_time: Option<String>,
    #[doc = "number of indexing ops"]
    #[serde(rename = "pri.indexing.index_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_indexing_index_total: Option<String>,
    #[doc = "total user memory"]
    #[serde(rename = "pri.memory.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_memory_total: Option<String>,
    #[doc = "number of current merges"]
    #[serde(rename = "pri.merges.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_current: Option<String>,
    #[doc = "number of current merging docs"]
    #[serde(rename = "pri.merges.current_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_current_docs: Option<String>,
    #[doc = "size of current merges"]
    #[serde(rename = "pri.merges.current_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_current_size: Option<String>,
    #[doc = "number of completed merge ops"]
    #[serde(rename = "pri.merges.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_total: Option<String>,
    #[doc = "docs merged"]
    #[serde(rename = "pri.merges.total_docs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_total_docs: Option<String>,
    #[doc = "size merged"]
    #[serde(rename = "pri.merges.total_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_total_size: Option<String>,
    #[doc = "time spent in merges"]
    #[serde(rename = "pri.merges.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_merges_total_time: Option<String>,
    #[doc = "query cache evictions"]
    #[serde(rename = "pri.query_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_query_cache_evictions: Option<String>,
    #[doc = "used query cache"]
    #[serde(rename = "pri.query_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_query_cache_memory_size: Option<String>,
    #[doc = "time spent in external refreshes"]
    #[serde(rename = "pri.refresh.external_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_refresh_external_time: Option<String>,
    #[doc = "total external refreshes"]
    #[serde(rename = "pri.refresh.external_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_refresh_external_total: Option<String>,
    #[doc = "number of pending refresh listeners"]
    #[serde(rename = "pri.refresh.listeners")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_refresh_listeners: Option<String>,
    #[doc = "time spent in refreshes"]
    #[serde(rename = "pri.refresh.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_refresh_time: Option<String>,
    #[doc = "total refreshes"]
    #[serde(rename = "pri.refresh.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_refresh_total: Option<String>,
    #[doc = "request cache evictions"]
    #[serde(rename = "pri.request_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_request_cache_evictions: Option<String>,
    #[doc = "request cache hit count"]
    #[serde(rename = "pri.request_cache.hit_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_request_cache_hit_count: Option<String>,
    #[doc = "used request cache"]
    #[serde(rename = "pri.request_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_request_cache_memory_size: Option<String>,
    #[doc = "request cache miss count"]
    #[serde(rename = "pri.request_cache.miss_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_request_cache_miss_count: Option<String>,
    #[serde(rename = "pri.search.concurrent_avg_slice_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_concurrent_avg_slice_count: Option<String>,
    #[serde(rename = "pri.search.concurrent_query_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_concurrent_query_current: Option<String>,
    #[serde(rename = "pri.search.concurrent_query_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_concurrent_query_time: Option<String>,
    #[serde(rename = "pri.search.concurrent_query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_concurrent_query_total: Option<String>,
    #[doc = "current fetch phase ops"]
    #[serde(rename = "pri.search.fetch_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_fetch_current: Option<String>,
    #[doc = "time spent in fetch phase"]
    #[serde(rename = "pri.search.fetch_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_fetch_time: Option<String>,
    #[doc = "total fetch ops"]
    #[serde(rename = "pri.search.fetch_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_fetch_total: Option<String>,
    #[doc = "open search contexts"]
    #[serde(rename = "pri.search.open_contexts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_open_contexts: Option<String>,
    #[serde(rename = "pri.search.point_in_time_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_point_in_time_current: Option<String>,
    #[serde(rename = "pri.search.point_in_time_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_point_in_time_time: Option<String>,
    #[serde(rename = "pri.search.point_in_time_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_point_in_time_total: Option<String>,
    #[doc = "current query phase ops"]
    #[serde(rename = "pri.search.query_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_query_current: Option<String>,
    #[doc = "time spent in query phase"]
    #[serde(rename = "pri.search.query_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_query_time: Option<String>,
    #[doc = "total query phase ops"]
    #[serde(rename = "pri.search.query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_query_total: Option<String>,
    #[doc = "open scroll contexts"]
    #[serde(rename = "pri.search.scroll_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_scroll_current: Option<String>,
    #[doc = "time scroll contexts held open"]
    #[serde(rename = "pri.search.scroll_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_scroll_time: Option<String>,
    #[doc = "completed scroll contexts"]
    #[serde(rename = "pri.search.scroll_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_search_scroll_total: Option<String>,
    #[doc = "number of segments"]
    #[serde(rename = "pri.segments.count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_segments_count: Option<String>,
    #[doc = "memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields"]
    #[serde(rename = "pri.segments.fixed_bitset_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_segments_fixed_bitset_memory: Option<String>,
    #[doc = "memory used by index writer"]
    #[serde(rename = "pri.segments.index_writer_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_segments_index_writer_memory: Option<String>,
    #[doc = "memory used by segments"]
    #[serde(rename = "pri.segments.memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_segments_memory: Option<String>,
    #[doc = "memory used by version map"]
    #[serde(rename = "pri.segments.version_map_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_segments_version_map_memory: Option<String>,
    #[doc = "store size of primaries"]
    #[serde(rename = "pri.store.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_store_size: Option<String>,
    #[doc = "number of current suggest ops"]
    #[serde(rename = "pri.suggest.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_suggest_current: Option<String>,
    #[doc = "time spend in suggest"]
    #[serde(rename = "pri.suggest.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_suggest_time: Option<String>,
    #[doc = "number of suggest ops"]
    #[serde(rename = "pri.suggest.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_suggest_total: Option<String>,
    #[doc = "current warmer ops"]
    #[serde(rename = "pri.warmer.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_warmer_current: Option<String>,
    #[doc = "total warmer ops"]
    #[serde(rename = "pri.warmer.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_warmer_total: Option<String>,
    #[doc = "time spent in warmers"]
    #[serde(rename = "pri.warmer.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pri_warmer_total_time: Option<String>,
    #[doc = "query cache evictions"]
    #[serde(rename = "query_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_evictions: Option<String>,
    #[doc = "used query cache"]
    #[serde(rename = "query_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_cache_memory_size: Option<String>,
    #[doc = "time spent in external refreshes"]
    #[serde(rename = "refresh.external_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_external_time: Option<String>,
    #[doc = "total external refreshes"]
    #[serde(rename = "refresh.external_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_external_total: Option<String>,
    #[doc = "number of pending refresh listeners"]
    #[serde(rename = "refresh.listeners")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_listeners: Option<String>,
    #[doc = "time spent in refreshes"]
    #[serde(rename = "refresh.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_time: Option<String>,
    #[doc = "total refreshes"]
    #[serde(rename = "refresh.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_total: Option<String>,
    #[doc = "number of replica shards"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rep: Option<String>,
    #[doc = "request cache evictions"]
    #[serde(rename = "request_cache.evictions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_evictions: Option<String>,
    #[doc = "request cache hit count"]
    #[serde(rename = "request_cache.hit_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_hit_count: Option<String>,
    #[doc = "used request cache"]
    #[serde(rename = "request_cache.memory_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_memory_size: Option<String>,
    #[doc = "request cache miss count"]
    #[serde(rename = "request_cache.miss_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_cache_miss_count: Option<String>,
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
    #[doc = "current fetch phase ops"]
    #[serde(rename = "search.fetch_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_current: Option<String>,
    #[doc = "time spent in fetch phase"]
    #[serde(rename = "search.fetch_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_time: Option<String>,
    #[doc = "total fetch ops"]
    #[serde(rename = "search.fetch_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_fetch_total: Option<String>,
    #[doc = "open search contexts"]
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
    #[doc = "current query phase ops"]
    #[serde(rename = "search.query_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_current: Option<String>,
    #[doc = "time spent in query phase"]
    #[serde(rename = "search.query_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_time: Option<String>,
    #[doc = "total query phase ops"]
    #[serde(rename = "search.query_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_query_total: Option<String>,
    #[doc = "open scroll contexts"]
    #[serde(rename = "search.scroll_current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_current: Option<String>,
    #[doc = "time scroll contexts held open"]
    #[serde(rename = "search.scroll_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_time: Option<String>,
    #[doc = "completed scroll contexts"]
    #[serde(rename = "search.scroll_total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_scroll_total: Option<String>,
    #[doc = "indicates if the index is search throttled"]
    #[serde(rename = "search.throttled")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_throttled: Option<String>,
    #[doc = "number of segments"]
    #[serde(rename = "segments.count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_count: Option<String>,
    #[doc = "memory used by fixed bit sets for nested object field types and export type filters for types referred in `_parent` fields"]
    #[serde(rename = "segments.fixed_bitset_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_fixed_bitset_memory: Option<String>,
    #[doc = "memory used by index writer"]
    #[serde(rename = "segments.index_writer_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_index_writer_memory: Option<String>,
    #[doc = "memory used by segments"]
    #[serde(rename = "segments.memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_memory: Option<String>,
    #[doc = "memory used by version map"]
    #[serde(rename = "segments.version_map_memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_version_map_memory: Option<String>,
    #[doc = "open/close status"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "store size of primaries and replicas"]
    #[serde(rename = "store.size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_size: Option<String>,
    #[doc = "number of current suggest ops"]
    #[serde(rename = "suggest.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_current: Option<String>,
    #[doc = "time spend in suggest"]
    #[serde(rename = "suggest.time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_time: Option<String>,
    #[doc = "number of suggest ops"]
    #[serde(rename = "suggest.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggest_total: Option<String>,
    #[doc = "index UUID"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    #[doc = "current warmer ops"]
    #[serde(rename = "warmer.current")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_current: Option<String>,
    #[doc = "total warmer ops"]
    #[serde(rename = "warmer.total")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_total: Option<String>,
    #[doc = "time spent in warmers"]
    #[serde(rename = "warmer.total_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warmer_total_time: Option<String>,
}

#[doc = "The response of the `cat.indices` API"]
pub type IndicesResponse = crate::types::cat::IndicesResponse;
