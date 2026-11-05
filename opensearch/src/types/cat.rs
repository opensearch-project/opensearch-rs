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

//! Response types of the `cat` specification namespace

#![allow(missing_docs)]

pub type AliasesResponse = Vec<crate::types::cat_aliases::AliasesRecord>;

pub type AllPitSegmentsResponse = Vec<crate::types::cat_segments::SegmentsRecord>;

pub type AllocationResponse = Vec<crate::types::cat_allocation::AllocationRecord>;

pub type ClusterManagerResponse = Vec<crate::types::cat_cluster_manager::ClusterManagerRecord>;

pub type CountResponse = Vec<crate::types::cat_count::CountRecord>;

pub type FielddataResponse = Vec<crate::types::cat_fielddata::FielddataRecord>;

pub type HealthResponse = Vec<crate::types::cat_health::HealthRecord>;

pub type IndicesResponse = Vec<crate::types::cat_indices::IndicesRecord>;

pub type MasterResponse = Vec<crate::types::cat_master::MasterRecord>;

pub type NodeattrsResponse = Vec<crate::types::cat_nodeattrs::NodeAttributesRecord>;

pub type NodesResponse = Vec<crate::types::cat_nodes::NodesRecord>;

pub type PendingTasksResponse = Vec<crate::types::cat_pending_tasks::PendingTasksRecord>;

pub type PitSegmentsResponse = Vec<crate::types::cat_segments::SegmentsRecord>;

pub type PluginsResponse = Vec<crate::types::cat_plugins::PluginsRecord>;

pub type RecoveryResponse = Vec<crate::types::cat_recovery::RecoveryRecord>;

pub type RepositoriesResponse = Vec<crate::types::cat_repositories::RepositoriesRecord>;

pub type SegmentReplicationResponse =
    Vec<crate::types::cat_segment_replication::SegmentReplicationRecord>;

pub type SegmentsResponse = Vec<crate::types::cat_segments::SegmentsRecord>;

pub type ShardsResponse = Vec<crate::types::cat_shards::ShardsRecord>;

pub type SnapshotsResponse = Vec<crate::types::cat_snapshots::SnapshotsRecord>;

pub type TasksResponse = Vec<crate::types::cat_tasks::TasksRecord>;

pub type TemplatesResponse = Vec<crate::types::cat_templates::TemplatesRecord>;

pub type ThreadPoolResponse = Vec<crate::types::cat_thread_pool::ThreadPoolRecord>;
