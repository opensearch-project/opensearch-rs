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

//! Response types of the `cat_segments` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentsRecord {
    #[doc = "If `true`, the segment is synced to disk. Segments that are synced can survive a hard reboot. If `false`, the data from uncommitted segments is also stored in the transaction log so that OpenSearch is able to replay changes on the next start."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub committed: Option<String>,
    #[doc = "If `true`, the segment is stored in a compound file. This means Lucene merged all files from the segment in a single file to save file descriptors."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compound: Option<String>,
    #[doc = "The number of documents in the segment. This excludes deleted documents and counts any nested documents separately from their parents. It also excludes documents which were indexed recently and do not yet belong to a segment."]
    #[serde(rename = "docs.count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_count: Option<String>,
    #[doc = "The number of deleted documents in the segment, which might be higher or lower than the number of delete operations you have performed. This number excludes deletes that were performed recently and do not yet belong to a segment. Deleted documents are cleaned up by the automatic merge process if it makes sense to do so. Also, OpenSearch creates extra deleted documents to internally track the recent history of operations on a shard."]
    #[serde(rename = "docs.deleted")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_deleted: Option<String>,
    #[doc = "The segment generation number. OpenSearch increments this generation number for each segment written then uses this number to derive the segment name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::NodeId>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::common::IndexName>,
    #[doc = "The IP address of the node where it lives."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    #[doc = "The shard type: `primary` or `replica`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prirep: Option<String>,
    #[doc = "If `true`, the segment is searchable. If `false`, the segment has most likely been written to disk but needs a refresh to be searchable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<String>,
    #[doc = "The segment name, which is derived from the segment generation and used internally to create file names in the directory of the shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segment: Option<String>,
    #[doc = "The shard name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shard: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(rename = "size.memory")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_memory: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[doc = "The response of the `cat.segments` API"]
pub type SegmentsResponse = crate::types::cat::SegmentsResponse;
