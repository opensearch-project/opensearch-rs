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

//! Response types of the `indices_common` specification namespace

#![allow(missing_docs)]

#[doc = "The index alias configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Alias {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_routing: Option<crate::types::common::Routing>,
    #[doc = "When `true`, the alias is hidden. All indexes for the alias must have the same `is_hidden` value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "When `true`, the index is the write index for the alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_write_index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_routing: Option<crate::types::common::Routing>,
}

#[doc = "The configuration for an index alias."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AliasDefinition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[doc = "The value used to route indexing operations to a specific shard. When specified, this overwrites the `routing` value for indexing operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_routing: Option<String>,
    #[doc = "When `true`, the alias is hidden. All indexes for the alias must have the same `is_hidden` value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "When `true`, the index is the write index for the alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_write_index: Option<bool>,
    #[doc = "The value used to route indexing and search operations to a specific shard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<String>,
    #[doc = "The value used to route search operations to a specific shard. When specified, this overwrites the `routing` value for search operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_routing: Option<String>,
}

#[doc = "The type of storage implementation."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltinStorageType {
    #[doc = "`fs`"]
    Fs,
    #[doc = "`hybridfs`"]
    Hybridfs,
    #[doc = "`mmapfs`"]
    Mmapfs,
    #[doc = "`niofs`"]
    Niofs,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BuiltinStorageType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BuiltinStorageType::Fs => "fs",
            BuiltinStorageType::Hybridfs => "hybridfs",
            BuiltinStorageType::Mmapfs => "mmapfs",
            BuiltinStorageType::Niofs => "niofs",
            BuiltinStorageType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BuiltinStorageType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BuiltinStorageType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fs" => BuiltinStorageType::Fs,
            "hybridfs" => BuiltinStorageType::Hybridfs,
            "mmapfs" => BuiltinStorageType::Mmapfs,
            "niofs" => BuiltinStorageType::Niofs,
            _ => BuiltinStorageType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStream {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "When `true`, the data stream allows custom routing on write request."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_custom_routing: Option<bool>,
    #[doc = "The current generation number for the data stream. This number acts as a cumulative count of the stream's rollovers, starting at 1."]
    pub generation: i64,
    #[doc = "When `true`, the data stream is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ilm_policy: Option<crate::types::common::Name>,
    #[doc = "An array of objects containing information about the data stream's backing indexes. The last item in this array contains information about the stream's current write index."]
    pub indices: Vec<crate::types::indices_common::DataStreamIndex>,
    pub name: crate::types::common::DataStreamName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_generation_managed_by: Option<crate::types::indices_common::ManagedBy>,
    #[doc = "Indicates if ILM should take precedence over DSL in case both are configured to managed this data stream."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_ilm: Option<bool>,
    #[doc = "When `true`, the data stream is created and managed by cross-cluster replication and the local cluster can not write into this data stream or change its mappings."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replicated: Option<bool>,
    pub status: crate::types::common::HealthStatus,
    #[doc = "When `true`, the data stream is created and managed by an OpenSearch stack component and cannot be modified through normal user interaction."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,
    pub template: crate::types::common::Name,
    pub timestamp_field: crate::types::indices_common::DataStreamTimestampField,
}

#[doc = "The configuration for a data stream's backing index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamIndex {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ilm_policy: Option<crate::types::common::Name>,
    pub index_name: crate::types::common::IndexName,
    pub index_uuid: crate::types::common::Uuid,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_by: Option<crate::types::indices_common::ManagedBy>,
    #[doc = "Indicates if ILM should take precedence over DSL in case both are configured to manage this index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefer_ilm: Option<bool>,
}

#[doc = "The statistics for a data stream."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamStats {
    #[doc = "Current number of backing indexes for the data stream."]
    pub backing_indices: i32,
    pub data_stream: crate::types::common::Name,
    pub maximum_timestamp: crate::types::common::EpochTimeUnitMillis,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The total size, in bytes, of all shards for the data stream's backing indexes."]
    pub store_size_bytes: crate::types::common::ByteCount,
}

#[doc = "The timestamp field configuration for a data stream."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamTimestampField {
    pub name: crate::types::common::Field,
}

#[doc = "The frequency filter configuration for field data."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FielddataFrequencyFilter {
    #[doc = "The maximum frequency threshold."]
    pub max: f64,
    #[doc = "The minimum frequency threshold."]
    pub min: f64,
    #[doc = "The minimum segment size to apply the filter."]
    pub min_segment_size: i64,
}

#[doc = "The type of check to perform when starting an index."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexCheckOnStartup {
    #[doc = "`checksum`"]
    Checksum,
    #[doc = "`false`"]
    False,
    #[doc = "`true`"]
    True,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexCheckOnStartup {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexCheckOnStartup::Checksum => "checksum",
            IndexCheckOnStartup::False => "false",
            IndexCheckOnStartup::True => "true",
            IndexCheckOnStartup::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexCheckOnStartup {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexCheckOnStartup {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "checksum" => IndexCheckOnStartup::Checksum,
            "false" => IndexCheckOnStartup::False,
            "true" => IndexCheckOnStartup::True,
            _ => IndexCheckOnStartup::Unknown(s),
        })
    }
}

#[doc = "The upgrade status information for indexes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexGetUpgradeStatus {
    #[doc = "The upgrade status for individual indexes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indices:
        Option<std::collections::BTreeMap<String, crate::types::indices_common::UpgradeStatus>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_to_upgrade_ancient_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount>,
}

#[doc = "The routing configuration for index operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRouting {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<crate::types::indices_common::IndexRoutingAllocation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rebalance: Option<crate::types::indices_common::IndexRoutingRebalance>,
}

#[doc = "The configuration for shard allocation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRoutingAllocation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disk: Option<crate::types::indices_common::IndexRoutingAllocationDisk>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable: Option<crate::types::indices_common::IndexRoutingAllocationOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<crate::types::indices_common::IndexRoutingAllocationInclude>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub initial_recovery:
        Option<crate::types::indices_common::IndexRoutingAllocationInitialRecovery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_primary_shards_per_node: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_shards_per_node: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The disk-based allocation settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRoutingAllocationDisk {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold_enabled: Option<crate::types::common::StringifiedBoolean>,
}

#[doc = "The inclusion rules for shard allocation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRoutingAllocationInclude {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "The data tier preference for the index."]
    #[serde(rename = "_tier_preference")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tier_preference: Option<String>,
}

#[doc = "The configuration for initial shard recovery."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRoutingAllocationInitialRecovery {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
}

#[doc = "The options for shard allocation control."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexRoutingAllocationOptions {
    #[doc = "`all`"]
    All,
    #[doc = "`new_primaries`"]
    NewPrimaries,
    #[doc = "`none`"]
    None,
    #[doc = "`primaries`"]
    Primaries,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexRoutingAllocationOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexRoutingAllocationOptions::All => "all",
            IndexRoutingAllocationOptions::NewPrimaries => "new_primaries",
            IndexRoutingAllocationOptions::None => "none",
            IndexRoutingAllocationOptions::Primaries => "primaries",
            IndexRoutingAllocationOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexRoutingAllocationOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexRoutingAllocationOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => IndexRoutingAllocationOptions::All,
            "new_primaries" => IndexRoutingAllocationOptions::NewPrimaries,
            "none" => IndexRoutingAllocationOptions::None,
            "primaries" => IndexRoutingAllocationOptions::Primaries,
            _ => IndexRoutingAllocationOptions::Unknown(s),
        })
    }
}

#[doc = "The configuration for shard rebalancing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexRoutingRebalance {
    pub enable: crate::types::indices_common::IndexRoutingRebalanceOptions,
}

#[doc = "The options for shard rebalancing control."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexRoutingRebalanceOptions {
    #[doc = "`all`"]
    All,
    #[doc = "`none`"]
    None,
    #[doc = "`primaries`"]
    Primaries,
    #[doc = "`replicas`"]
    Replicas,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexRoutingRebalanceOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexRoutingRebalanceOptions::All => "all",
            IndexRoutingRebalanceOptions::None => "none",
            IndexRoutingRebalanceOptions::Primaries => "primaries",
            IndexRoutingRebalanceOptions::Replicas => "replicas",
            IndexRoutingRebalanceOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexRoutingRebalanceOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexRoutingRebalanceOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "all" => IndexRoutingRebalanceOptions::All,
            "none" => IndexRoutingRebalanceOptions::None,
            "primaries" => IndexRoutingRebalanceOptions::Primaries,
            "replicas" => IndexRoutingRebalanceOptions::Replicas,
            _ => IndexRoutingRebalanceOptions::Unknown(s),
        })
    }
}

#[doc = "The configuration for segment sorting."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSegmentSort {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub missing: Option<crate::types::indices_common::IndexSegmentSortMissing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::indices_common::IndexSegmentSortMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::indices_common::IndexSegmentSortOrder>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortMissing {
    SegmentSortMissing(crate::types::indices_common::SegmentSortMissing),
    Array(Vec<crate::types::indices_common::SegmentSortMissing>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortMissing {
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
            Some(b'"') => {
                serde_json::from_str(text).map(IndexSegmentSortMissing::SegmentSortMissing)
            }
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortMissing::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IndexSegmentSortMissing"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortMode {
    SegmentSortMode(crate::types::indices_common::SegmentSortMode),
    Array(Vec<crate::types::indices_common::SegmentSortMode>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortMode {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSegmentSortMode::SegmentSortMode),
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortMode::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IndexSegmentSortMode"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSegmentSortOrder {
    SegmentSortOrder(crate::types::indices_common::SegmentSortOrder),
    Array(Vec<crate::types::indices_common::SegmentSortOrder>),
}
impl<'de> serde::Deserialize<'de> for IndexSegmentSortOrder {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSegmentSortOrder::SegmentSortOrder),
            Some(b'[') => serde_json::from_str(text).map(IndexSegmentSortOrder::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IndexSegmentSortOrder"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The block settings that control index operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingBlocks {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only_allow_delete: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "When true, the index is in search-only mode, allowing only read operations"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_only: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write: Option<crate::types::common::StringifiedBoolean>,
}

#[doc = "The configuration settings for an index."]
#[derive(Debug, Clone)]
pub struct IndexSettings {
    pub analysis: Option<crate::types::indices_common::IndexSettingsAnalysis>,
    pub analyze: Option<crate::types::indices_common::IndexSettingsAnalyze>,
    pub analyze_max_token_count: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The range of replicas to maintain."]
    pub auto_expand_replicas: Option<String>,
    pub blocks: Option<crate::types::indices_common::IndexSettingBlocks>,
    pub blocks_metadata: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read_only: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_read_only_allow_delete: Option<crate::types::common::StringifiedBoolean>,
    pub blocks_write: Option<crate::types::common::StringifiedBoolean>,
    pub check_on_startup: Option<crate::types::indices_common::IndexCheckOnStartup>,
    #[doc = "The compression type for stored data."]
    pub codec: Option<String>,
    pub composite_index_star_tree: Option<crate::types::indices_common::IndexSettingsStarTree>,
    pub creation_date: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    pub creation_date_string: Option<crate::types::common::DateTime>,
    pub default_pipeline: Option<crate::types::common::PipelineName>,
    pub final_pipeline: Option<crate::types::common::PipelineName>,
    pub format: Option<crate::types::common::StringifiedInteger>,
    pub gc_deletes: Option<crate::types::common::Duration>,
    pub hidden: Option<crate::types::common::StringifiedBoolean>,
    pub highlight: Option<crate::types::indices_common::IndexSettingsHighlight>,
    pub highlight_max_analyzed_offset: Option<crate::types::common::StringifiedInteger>,
    pub index: Option<Box<crate::types::indices_common::IndexSettings>>,
    pub indexing: Option<crate::types::indices_common::IndexSettingsIndexing>,
    pub indexing_pressure: Option<crate::types::indices_common::IndexingPressure>,
    pub ingestion_source: Option<crate::types::indices_common::IngestionSource>,
    pub knn: Option<crate::types::common::StringifiedBoolean>,
    pub knn_algo_param_ef_search: Option<crate::types::common::StringifiedInteger>,
    pub lifecycle: Option<crate::types::indices_common::IndexSettingsLifecycle>,
    pub lifecycle_name: Option<crate::types::common::Name>,
    pub load_fixed_bitset_filters_eagerly: Option<crate::types::common::StringifiedBoolean>,
    pub mapping: Option<crate::types::indices_common::IndexSettingsMapping>,
    pub max_docvalue_fields_search: Option<crate::types::common::StringifiedInteger>,
    pub max_inner_result_window: Option<crate::types::common::StringifiedInteger>,
    pub max_ngram_diff: Option<crate::types::common::StringifiedInteger>,
    pub max_refresh_listeners: Option<crate::types::common::StringifiedInteger>,
    pub max_regex_length: Option<crate::types::common::StringifiedInteger>,
    pub max_rescore_window: Option<crate::types::common::StringifiedInteger>,
    pub max_result_window: Option<crate::types::common::StringifiedInteger>,
    pub max_script_fields: Option<crate::types::common::StringifiedInteger>,
    pub max_shingle_diff: Option<crate::types::common::StringifiedInteger>,
    pub max_slices_per_scroll: Option<crate::types::common::StringifiedInteger>,
    pub max_terms_count: Option<crate::types::common::StringifiedInteger>,
    pub merge: Option<crate::types::indices_common::IndexSettingsMerge>,
    pub merge_scheduler_max_thread_count: Option<crate::types::common::StringifiedInteger>,
    pub mode: Option<String>,
    pub number_of_replicas: Option<crate::types::common::StringifiedInteger>,
    pub number_of_routing_shards: Option<crate::types::common::StringifiedInteger>,
    pub number_of_shards: Option<crate::types::common::StringifiedInteger>,
    pub priority: Option<crate::types::common::StringifiedInteger>,
    pub provided_name: Option<crate::types::common::Name>,
    pub queries: Option<crate::types::indices_common::IndexSettingsQueries>,
    pub query_string: Option<crate::types::indices_common::IndexSettingsQueryString>,
    pub query_string_lenient: Option<crate::types::common::StringifiedBoolean>,
    pub refresh_interval: Option<crate::types::common::Duration>,
    #[doc = "Defines the replication type."]
    pub replication_type: Option<crate::types::indices_common::ReplicationType>,
    pub routing: Option<crate::types::indices_common::IndexRouting>,
    pub routing_partition_size: Option<crate::types::common::StringifiedInteger>,
    pub routing_path: Option<crate::types::common::StringOrStringArray>,
    pub search: Option<crate::types::indices_common::IndexSettingsSearch>,
    pub search_idle_after: Option<crate::types::common::Duration>,
    pub settings: Option<Box<crate::types::indices_common::IndexSettings>>,
    pub similarity: Option<crate::types::indices_common::IndexSettingsSimilarity>,
    pub soft_deletes: Option<crate::types::indices_common::SoftDeletes>,
    pub soft_deletes_retention_lease_period: Option<crate::types::common::Duration>,
    pub sort: Option<crate::types::indices_common::IndexSegmentSort>,
    pub store: Option<crate::types::indices_common::IndexSettingsStore>,
    pub top_metrics_max_size: Option<crate::types::common::StringifiedInteger>,
    pub translog: Option<crate::types::indices_common::Translog>,
    pub translog_durability: Option<crate::types::indices_common::TranslogDurability>,
    pub translog_flush_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    pub uuid: Option<crate::types::common::Uuid>,
    pub verified_before_close: Option<crate::types::common::StringifiedBoolean>,
    pub version: Option<crate::types::indices_common::IndexVersioning>,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IndexSettings {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IndexSettings;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IndexSettings object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_analysis: Option<crate::types::indices_common::IndexSettingsAnalysis> =
                    None;
                let mut seen_analysis = false;
                let mut f_analyze: Option<crate::types::indices_common::IndexSettingsAnalyze> =
                    None;
                let mut seen_analyze = false;
                let mut f_analyze_max_token_count: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_analyze_max_token_count = false;
                let mut f_auto_expand_replicas: Option<String> = None;
                let mut seen_auto_expand_replicas = false;
                let mut f_blocks: Option<crate::types::indices_common::IndexSettingBlocks> = None;
                let mut seen_blocks = false;
                let mut f_blocks_metadata: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_metadata = false;
                let mut f_blocks_read: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_read = false;
                let mut f_blocks_read_only: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_read_only = false;
                let mut f_blocks_read_only_allow_delete: Option<
                    crate::types::common::StringifiedBoolean,
                > = None;
                let mut seen_blocks_read_only_allow_delete = false;
                let mut f_blocks_write: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_blocks_write = false;
                let mut f_check_on_startup: Option<
                    crate::types::indices_common::IndexCheckOnStartup,
                > = None;
                let mut seen_check_on_startup = false;
                let mut f_codec: Option<String> = None;
                let mut seen_codec = false;
                let mut f_composite_index_star_tree: Option<
                    crate::types::indices_common::IndexSettingsStarTree,
                > = None;
                let mut seen_composite_index_star_tree = false;
                let mut f_creation_date: Option<
                    crate::types::common::StringifiedEpochTimeUnitMillis,
                > = None;
                let mut seen_creation_date = false;
                let mut f_creation_date_string: Option<crate::types::common::DateTime> = None;
                let mut seen_creation_date_string = false;
                let mut f_default_pipeline: Option<crate::types::common::PipelineName> = None;
                let mut seen_default_pipeline = false;
                let mut f_final_pipeline: Option<crate::types::common::PipelineName> = None;
                let mut seen_final_pipeline = false;
                let mut f_format: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_format = false;
                let mut f_gc_deletes: Option<crate::types::common::Duration> = None;
                let mut seen_gc_deletes = false;
                let mut f_hidden: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_hidden = false;
                let mut f_highlight: Option<crate::types::indices_common::IndexSettingsHighlight> =
                    None;
                let mut seen_highlight = false;
                let mut f_highlight_max_analyzed_offset: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_highlight_max_analyzed_offset = false;
                let mut f_index: Option<Box<crate::types::indices_common::IndexSettings>> = None;
                let mut seen_index = false;
                let mut f_indexing: Option<crate::types::indices_common::IndexSettingsIndexing> =
                    None;
                let mut seen_indexing = false;
                let mut f_indexing_pressure: Option<
                    crate::types::indices_common::IndexingPressure,
                > = None;
                let mut seen_indexing_pressure = false;
                let mut f_ingestion_source: Option<crate::types::indices_common::IngestionSource> =
                    None;
                let mut seen_ingestion_source = false;
                let mut f_knn: Option<crate::types::common::StringifiedBoolean> = None;
                let mut seen_knn = false;
                let mut f_knn_algo_param_ef_search: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_knn_algo_param_ef_search = false;
                let mut f_lifecycle: Option<crate::types::indices_common::IndexSettingsLifecycle> =
                    None;
                let mut seen_lifecycle = false;
                let mut f_lifecycle_name: Option<crate::types::common::Name> = None;
                let mut seen_lifecycle_name = false;
                let mut f_load_fixed_bitset_filters_eagerly: Option<
                    crate::types::common::StringifiedBoolean,
                > = None;
                let mut seen_load_fixed_bitset_filters_eagerly = false;
                let mut f_mapping: Option<crate::types::indices_common::IndexSettingsMapping> =
                    None;
                let mut seen_mapping = false;
                let mut f_max_docvalue_fields_search: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_max_docvalue_fields_search = false;
                let mut f_max_inner_result_window: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_max_inner_result_window = false;
                let mut f_max_ngram_diff: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_ngram_diff = false;
                let mut f_max_refresh_listeners: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_refresh_listeners = false;
                let mut f_max_regex_length: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_regex_length = false;
                let mut f_max_rescore_window: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_rescore_window = false;
                let mut f_max_result_window: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_result_window = false;
                let mut f_max_script_fields: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_script_fields = false;
                let mut f_max_shingle_diff: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_shingle_diff = false;
                let mut f_max_slices_per_scroll: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_max_slices_per_scroll = false;
                let mut f_max_terms_count: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_max_terms_count = false;
                let mut f_merge: Option<crate::types::indices_common::IndexSettingsMerge> = None;
                let mut seen_merge = false;
                let mut f_merge_scheduler_max_thread_count: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_merge_scheduler_max_thread_count = false;
                let mut f_mode: Option<String> = None;
                let mut seen_mode = false;
                let mut f_number_of_replicas: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_number_of_replicas = false;
                let mut f_number_of_routing_shards: Option<
                    crate::types::common::StringifiedInteger,
                > = None;
                let mut seen_number_of_routing_shards = false;
                let mut f_number_of_shards: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_number_of_shards = false;
                let mut f_priority: Option<crate::types::common::StringifiedInteger> = None;
                let mut seen_priority = false;
                let mut f_provided_name: Option<crate::types::common::Name> = None;
                let mut seen_provided_name = false;
                let mut f_queries: Option<crate::types::indices_common::IndexSettingsQueries> =
                    None;
                let mut seen_queries = false;
                let mut f_query_string: Option<
                    crate::types::indices_common::IndexSettingsQueryString,
                > = None;
                let mut seen_query_string = false;
                let mut f_query_string_lenient: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_query_string_lenient = false;
                let mut f_refresh_interval: Option<crate::types::common::Duration> = None;
                let mut seen_refresh_interval = false;
                let mut f_replication_type: Option<crate::types::indices_common::ReplicationType> =
                    None;
                let mut seen_replication_type = false;
                let mut f_routing: Option<crate::types::indices_common::IndexRouting> = None;
                let mut seen_routing = false;
                let mut f_routing_partition_size: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_routing_partition_size = false;
                let mut f_routing_path: Option<crate::types::common::StringOrStringArray> = None;
                let mut seen_routing_path = false;
                let mut f_search: Option<crate::types::indices_common::IndexSettingsSearch> = None;
                let mut seen_search = false;
                let mut f_search_idle_after: Option<crate::types::common::Duration> = None;
                let mut seen_search_idle_after = false;
                let mut f_settings: Option<Box<crate::types::indices_common::IndexSettings>> = None;
                let mut seen_settings = false;
                let mut f_similarity: Option<
                    crate::types::indices_common::IndexSettingsSimilarity,
                > = None;
                let mut seen_similarity = false;
                let mut f_soft_deletes: Option<crate::types::indices_common::SoftDeletes> = None;
                let mut seen_soft_deletes = false;
                let mut f_soft_deletes_retention_lease_period: Option<
                    crate::types::common::Duration,
                > = None;
                let mut seen_soft_deletes_retention_lease_period = false;
                let mut f_sort: Option<crate::types::indices_common::IndexSegmentSort> = None;
                let mut seen_sort = false;
                let mut f_store: Option<crate::types::indices_common::IndexSettingsStore> = None;
                let mut seen_store = false;
                let mut f_top_metrics_max_size: Option<crate::types::common::StringifiedInteger> =
                    None;
                let mut seen_top_metrics_max_size = false;
                let mut f_translog: Option<crate::types::indices_common::Translog> = None;
                let mut seen_translog = false;
                let mut f_translog_durability: Option<
                    crate::types::indices_common::TranslogDurability,
                > = None;
                let mut seen_translog_durability = false;
                let mut f_translog_flush_threshold_size: Option<
                    crate::types::common::HumanReadableByteCount,
                > = None;
                let mut seen_translog_flush_threshold_size = false;
                let mut f_uuid: Option<crate::types::common::Uuid> = None;
                let mut seen_uuid = false;
                let mut f_verified_before_close: Option<crate::types::common::StringifiedBoolean> =
                    None;
                let mut seen_verified_before_close = false;
                let mut f_version: Option<crate::types::indices_common::IndexVersioning> = None;
                let mut seen_version = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "analysis" => {
                            if seen_analysis {
                                return Err(serde::de::Error::duplicate_field("analysis"));
                            }
                            seen_analysis = true;
                            f_analysis = map.next_value::<Option<crate::types::indices_common::IndexSettingsAnalysis>>()?;
                        }
                        "analyze" => {
                            if seen_analyze {
                                return Err(serde::de::Error::duplicate_field("analyze"));
                            }
                            seen_analyze = true;
                            f_analyze = map.next_value::<Option<crate::types::indices_common::IndexSettingsAnalyze>>()?;
                        }
                        "analyze.max_token_count" => {
                            if seen_analyze_max_token_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "analyze.max_token_count",
                                ));
                            }
                            seen_analyze_max_token_count = true;
                            f_analyze_max_token_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "auto_expand_replicas" => {
                            if seen_auto_expand_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "auto_expand_replicas",
                                ));
                            }
                            seen_auto_expand_replicas = true;
                            f_auto_expand_replicas = map.next_value::<Option<String>>()?;
                        }
                        "blocks" => {
                            if seen_blocks {
                                return Err(serde::de::Error::duplicate_field("blocks"));
                            }
                            seen_blocks = true;
                            f_blocks = map.next_value::<Option<crate::types::indices_common::IndexSettingBlocks>>()?;
                        }
                        "blocks.metadata" => {
                            if seen_blocks_metadata {
                                return Err(serde::de::Error::duplicate_field("blocks.metadata"));
                            }
                            seen_blocks_metadata = true;
                            f_blocks_metadata = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read" => {
                            if seen_blocks_read {
                                return Err(serde::de::Error::duplicate_field("blocks.read"));
                            }
                            seen_blocks_read = true;
                            f_blocks_read = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read_only" => {
                            if seen_blocks_read_only {
                                return Err(serde::de::Error::duplicate_field("blocks.read_only"));
                            }
                            seen_blocks_read_only = true;
                            f_blocks_read_only = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.read_only_allow_delete" => {
                            if seen_blocks_read_only_allow_delete {
                                return Err(serde::de::Error::duplicate_field(
                                    "blocks.read_only_allow_delete",
                                ));
                            }
                            seen_blocks_read_only_allow_delete = true;
                            f_blocks_read_only_allow_delete = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "blocks.write" => {
                            if seen_blocks_write {
                                return Err(serde::de::Error::duplicate_field("blocks.write"));
                            }
                            seen_blocks_write = true;
                            f_blocks_write = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "check_on_startup" => {
                            if seen_check_on_startup {
                                return Err(serde::de::Error::duplicate_field("check_on_startup"));
                            }
                            seen_check_on_startup = true;
                            f_check_on_startup = map.next_value::<Option<crate::types::indices_common::IndexCheckOnStartup>>()?;
                        }
                        "codec" => {
                            if seen_codec {
                                return Err(serde::de::Error::duplicate_field("codec"));
                            }
                            seen_codec = true;
                            f_codec = map.next_value::<Option<String>>()?;
                        }
                        "composite_index.star_tree" => {
                            if seen_composite_index_star_tree {
                                return Err(serde::de::Error::duplicate_field(
                                    "composite_index.star_tree",
                                ));
                            }
                            seen_composite_index_star_tree = true;
                            f_composite_index_star_tree = map.next_value::<Option<crate::types::indices_common::IndexSettingsStarTree>>()?;
                        }
                        "creation_date" => {
                            if seen_creation_date {
                                return Err(serde::de::Error::duplicate_field("creation_date"));
                            }
                            seen_creation_date = true;
                            f_creation_date = map.next_value::<Option<crate::types::common::StringifiedEpochTimeUnitMillis>>()?;
                        }
                        "creation_date_string" => {
                            if seen_creation_date_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "creation_date_string",
                                ));
                            }
                            seen_creation_date_string = true;
                            f_creation_date_string =
                                map.next_value::<Option<crate::types::common::DateTime>>()?;
                        }
                        "default_pipeline" => {
                            if seen_default_pipeline {
                                return Err(serde::de::Error::duplicate_field("default_pipeline"));
                            }
                            seen_default_pipeline = true;
                            f_default_pipeline =
                                map.next_value::<Option<crate::types::common::PipelineName>>()?;
                        }
                        "final_pipeline" => {
                            if seen_final_pipeline {
                                return Err(serde::de::Error::duplicate_field("final_pipeline"));
                            }
                            seen_final_pipeline = true;
                            f_final_pipeline =
                                map.next_value::<Option<crate::types::common::PipelineName>>()?;
                        }
                        "format" => {
                            if seen_format {
                                return Err(serde::de::Error::duplicate_field("format"));
                            }
                            seen_format = true;
                            f_format = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "gc_deletes" => {
                            if seen_gc_deletes {
                                return Err(serde::de::Error::duplicate_field("gc_deletes"));
                            }
                            seen_gc_deletes = true;
                            f_gc_deletes =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "hidden" => {
                            if seen_hidden {
                                return Err(serde::de::Error::duplicate_field("hidden"));
                            }
                            seen_hidden = true;
                            f_hidden = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "highlight" => {
                            if seen_highlight {
                                return Err(serde::de::Error::duplicate_field("highlight"));
                            }
                            seen_highlight = true;
                            f_highlight = map.next_value::<Option<crate::types::indices_common::IndexSettingsHighlight>>()?;
                        }
                        "highlight.max_analyzed_offset" => {
                            if seen_highlight_max_analyzed_offset {
                                return Err(serde::de::Error::duplicate_field(
                                    "highlight.max_analyzed_offset",
                                ));
                            }
                            seen_highlight_max_analyzed_offset = true;
                            f_highlight_max_analyzed_offset = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<Box<crate::types::indices_common::IndexSettings>>>()?;
                        }
                        "indexing" => {
                            if seen_indexing {
                                return Err(serde::de::Error::duplicate_field("indexing"));
                            }
                            seen_indexing = true;
                            f_indexing = map.next_value::<Option<crate::types::indices_common::IndexSettingsIndexing>>()?;
                        }
                        "indexing_pressure" => {
                            if seen_indexing_pressure {
                                return Err(serde::de::Error::duplicate_field("indexing_pressure"));
                            }
                            seen_indexing_pressure = true;
                            f_indexing_pressure = map.next_value::<Option<crate::types::indices_common::IndexingPressure>>()?;
                        }
                        "ingestion_source" => {
                            if seen_ingestion_source {
                                return Err(serde::de::Error::duplicate_field("ingestion_source"));
                            }
                            seen_ingestion_source = true;
                            f_ingestion_source = map.next_value::<Option<crate::types::indices_common::IngestionSource>>()?;
                        }
                        "knn" => {
                            if seen_knn {
                                return Err(serde::de::Error::duplicate_field("knn"));
                            }
                            seen_knn = true;
                            f_knn = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "knn.algo_param.ef_search" => {
                            if seen_knn_algo_param_ef_search {
                                return Err(serde::de::Error::duplicate_field(
                                    "knn.algo_param.ef_search",
                                ));
                            }
                            seen_knn_algo_param_ef_search = true;
                            f_knn_algo_param_ef_search = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "lifecycle" => {
                            if seen_lifecycle {
                                return Err(serde::de::Error::duplicate_field("lifecycle"));
                            }
                            seen_lifecycle = true;
                            f_lifecycle = map.next_value::<Option<crate::types::indices_common::IndexSettingsLifecycle>>()?;
                        }
                        "lifecycle.name" => {
                            if seen_lifecycle_name {
                                return Err(serde::de::Error::duplicate_field("lifecycle.name"));
                            }
                            seen_lifecycle_name = true;
                            f_lifecycle_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "load_fixed_bitset_filters_eagerly" => {
                            if seen_load_fixed_bitset_filters_eagerly {
                                return Err(serde::de::Error::duplicate_field(
                                    "load_fixed_bitset_filters_eagerly",
                                ));
                            }
                            seen_load_fixed_bitset_filters_eagerly = true;
                            f_load_fixed_bitset_filters_eagerly = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "mapping" => {
                            if seen_mapping {
                                return Err(serde::de::Error::duplicate_field("mapping"));
                            }
                            seen_mapping = true;
                            f_mapping = map.next_value::<Option<crate::types::indices_common::IndexSettingsMapping>>()?;
                        }
                        "max_docvalue_fields_search" => {
                            if seen_max_docvalue_fields_search {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_docvalue_fields_search",
                                ));
                            }
                            seen_max_docvalue_fields_search = true;
                            f_max_docvalue_fields_search = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_inner_result_window" => {
                            if seen_max_inner_result_window {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_inner_result_window",
                                ));
                            }
                            seen_max_inner_result_window = true;
                            f_max_inner_result_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_ngram_diff" => {
                            if seen_max_ngram_diff {
                                return Err(serde::de::Error::duplicate_field("max_ngram_diff"));
                            }
                            seen_max_ngram_diff = true;
                            f_max_ngram_diff = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_refresh_listeners" => {
                            if seen_max_refresh_listeners {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_refresh_listeners",
                                ));
                            }
                            seen_max_refresh_listeners = true;
                            f_max_refresh_listeners = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_regex_length" => {
                            if seen_max_regex_length {
                                return Err(serde::de::Error::duplicate_field("max_regex_length"));
                            }
                            seen_max_regex_length = true;
                            f_max_regex_length = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_rescore_window" => {
                            if seen_max_rescore_window {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_rescore_window",
                                ));
                            }
                            seen_max_rescore_window = true;
                            f_max_rescore_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_result_window" => {
                            if seen_max_result_window {
                                return Err(serde::de::Error::duplicate_field("max_result_window"));
                            }
                            seen_max_result_window = true;
                            f_max_result_window = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_script_fields" => {
                            if seen_max_script_fields {
                                return Err(serde::de::Error::duplicate_field("max_script_fields"));
                            }
                            seen_max_script_fields = true;
                            f_max_script_fields = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_shingle_diff" => {
                            if seen_max_shingle_diff {
                                return Err(serde::de::Error::duplicate_field("max_shingle_diff"));
                            }
                            seen_max_shingle_diff = true;
                            f_max_shingle_diff = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_slices_per_scroll" => {
                            if seen_max_slices_per_scroll {
                                return Err(serde::de::Error::duplicate_field(
                                    "max_slices_per_scroll",
                                ));
                            }
                            seen_max_slices_per_scroll = true;
                            f_max_slices_per_scroll = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "max_terms_count" => {
                            if seen_max_terms_count {
                                return Err(serde::de::Error::duplicate_field("max_terms_count"));
                            }
                            seen_max_terms_count = true;
                            f_max_terms_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "merge" => {
                            if seen_merge {
                                return Err(serde::de::Error::duplicate_field("merge"));
                            }
                            seen_merge = true;
                            f_merge = map.next_value::<Option<crate::types::indices_common::IndexSettingsMerge>>()?;
                        }
                        "merge.scheduler.max_thread_count" => {
                            if seen_merge_scheduler_max_thread_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "merge.scheduler.max_thread_count",
                                ));
                            }
                            seen_merge_scheduler_max_thread_count = true;
                            f_merge_scheduler_max_thread_count = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "mode" => {
                            if seen_mode {
                                return Err(serde::de::Error::duplicate_field("mode"));
                            }
                            seen_mode = true;
                            f_mode = map.next_value::<Option<String>>()?;
                        }
                        "number_of_replicas" => {
                            if seen_number_of_replicas {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_replicas",
                                ));
                            }
                            seen_number_of_replicas = true;
                            f_number_of_replicas = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "number_of_routing_shards" => {
                            if seen_number_of_routing_shards {
                                return Err(serde::de::Error::duplicate_field(
                                    "number_of_routing_shards",
                                ));
                            }
                            seen_number_of_routing_shards = true;
                            f_number_of_routing_shards = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "number_of_shards" => {
                            if seen_number_of_shards {
                                return Err(serde::de::Error::duplicate_field("number_of_shards"));
                            }
                            seen_number_of_shards = true;
                            f_number_of_shards = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "priority" => {
                            if seen_priority {
                                return Err(serde::de::Error::duplicate_field("priority"));
                            }
                            seen_priority = true;
                            f_priority = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "provided_name" => {
                            if seen_provided_name {
                                return Err(serde::de::Error::duplicate_field("provided_name"));
                            }
                            seen_provided_name = true;
                            f_provided_name =
                                map.next_value::<Option<crate::types::common::Name>>()?;
                        }
                        "queries" => {
                            if seen_queries {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            seen_queries = true;
                            f_queries = map.next_value::<Option<crate::types::indices_common::IndexSettingsQueries>>()?;
                        }
                        "query_string" => {
                            if seen_query_string {
                                return Err(serde::de::Error::duplicate_field("query_string"));
                            }
                            seen_query_string = true;
                            f_query_string = map.next_value::<Option<crate::types::indices_common::IndexSettingsQueryString>>()?;
                        }
                        "query_string.lenient" => {
                            if seen_query_string_lenient {
                                return Err(serde::de::Error::duplicate_field(
                                    "query_string.lenient",
                                ));
                            }
                            seen_query_string_lenient = true;
                            f_query_string_lenient = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "refresh_interval" => {
                            if seen_refresh_interval {
                                return Err(serde::de::Error::duplicate_field("refresh_interval"));
                            }
                            seen_refresh_interval = true;
                            f_refresh_interval =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "replication.type" => {
                            if seen_replication_type {
                                return Err(serde::de::Error::duplicate_field("replication.type"));
                            }
                            seen_replication_type = true;
                            f_replication_type = map.next_value::<Option<crate::types::indices_common::ReplicationType>>()?;
                        }
                        "routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("routing"));
                            }
                            seen_routing = true;
                            f_routing = map
                                .next_value::<Option<crate::types::indices_common::IndexRouting>>(
                                )?;
                        }
                        "routing_partition_size" => {
                            if seen_routing_partition_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "routing_partition_size",
                                ));
                            }
                            seen_routing_partition_size = true;
                            f_routing_partition_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "routing_path" => {
                            if seen_routing_path {
                                return Err(serde::de::Error::duplicate_field("routing_path"));
                            }
                            seen_routing_path = true;
                            f_routing_path = map
                                .next_value::<Option<crate::types::common::StringOrStringArray>>(
                                )?;
                        }
                        "search" => {
                            if seen_search {
                                return Err(serde::de::Error::duplicate_field("search"));
                            }
                            seen_search = true;
                            f_search = map.next_value::<Option<crate::types::indices_common::IndexSettingsSearch>>()?;
                        }
                        "search.idle.after" => {
                            if seen_search_idle_after {
                                return Err(serde::de::Error::duplicate_field("search.idle.after"));
                            }
                            seen_search_idle_after = true;
                            f_search_idle_after =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "settings" => {
                            if seen_settings {
                                return Err(serde::de::Error::duplicate_field("settings"));
                            }
                            seen_settings = true;
                            f_settings = map.next_value::<Option<Box<crate::types::indices_common::IndexSettings>>>()?;
                        }
                        "similarity" => {
                            if seen_similarity {
                                return Err(serde::de::Error::duplicate_field("similarity"));
                            }
                            seen_similarity = true;
                            f_similarity = map.next_value::<Option<crate::types::indices_common::IndexSettingsSimilarity>>()?;
                        }
                        "soft_deletes" => {
                            if seen_soft_deletes {
                                return Err(serde::de::Error::duplicate_field("soft_deletes"));
                            }
                            seen_soft_deletes = true;
                            f_soft_deletes = map
                                .next_value::<Option<crate::types::indices_common::SoftDeletes>>(
                                )?;
                        }
                        "soft_deletes.retention_lease.period" => {
                            if seen_soft_deletes_retention_lease_period {
                                return Err(serde::de::Error::duplicate_field(
                                    "soft_deletes.retention_lease.period",
                                ));
                            }
                            seen_soft_deletes_retention_lease_period = true;
                            f_soft_deletes_retention_lease_period =
                                map.next_value::<Option<crate::types::common::Duration>>()?;
                        }
                        "sort" => {
                            if seen_sort {
                                return Err(serde::de::Error::duplicate_field("sort"));
                            }
                            seen_sort = true;
                            f_sort = map.next_value::<Option<crate::types::indices_common::IndexSegmentSort>>()?;
                        }
                        "store" => {
                            if seen_store {
                                return Err(serde::de::Error::duplicate_field("store"));
                            }
                            seen_store = true;
                            f_store = map.next_value::<Option<crate::types::indices_common::IndexSettingsStore>>()?;
                        }
                        "top_metrics_max_size" => {
                            if seen_top_metrics_max_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "top_metrics_max_size",
                                ));
                            }
                            seen_top_metrics_max_size = true;
                            f_top_metrics_max_size = map
                                .next_value::<Option<crate::types::common::StringifiedInteger>>()?;
                        }
                        "translog" => {
                            if seen_translog {
                                return Err(serde::de::Error::duplicate_field("translog"));
                            }
                            seen_translog = true;
                            f_translog =
                                map.next_value::<Option<crate::types::indices_common::Translog>>()?;
                        }
                        "translog.durability" => {
                            if seen_translog_durability {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog.durability",
                                ));
                            }
                            seen_translog_durability = true;
                            f_translog_durability = map.next_value::<Option<crate::types::indices_common::TranslogDurability>>()?;
                        }
                        "translog.flush_threshold_size" => {
                            if seen_translog_flush_threshold_size {
                                return Err(serde::de::Error::duplicate_field(
                                    "translog.flush_threshold_size",
                                ));
                            }
                            seen_translog_flush_threshold_size = true;
                            f_translog_flush_threshold_size = map
                                .next_value::<Option<crate::types::common::HumanReadableByteCount>>(
                                )?;
                        }
                        "uuid" => {
                            if seen_uuid {
                                return Err(serde::de::Error::duplicate_field("uuid"));
                            }
                            seen_uuid = true;
                            f_uuid = map.next_value::<Option<crate::types::common::Uuid>>()?;
                        }
                        "verified_before_close" => {
                            if seen_verified_before_close {
                                return Err(serde::de::Error::duplicate_field(
                                    "verified_before_close",
                                ));
                            }
                            seen_verified_before_close = true;
                            f_verified_before_close = map
                                .next_value::<Option<crate::types::common::StringifiedBoolean>>()?;
                        }
                        "version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("version"));
                            }
                            seen_version = true;
                            f_version = map.next_value::<Option<crate::types::indices_common::IndexVersioning>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IndexSettings {
                    analysis: f_analysis,
                    analyze: f_analyze,
                    analyze_max_token_count: f_analyze_max_token_count,
                    auto_expand_replicas: f_auto_expand_replicas,
                    blocks: f_blocks,
                    blocks_metadata: f_blocks_metadata,
                    blocks_read: f_blocks_read,
                    blocks_read_only: f_blocks_read_only,
                    blocks_read_only_allow_delete: f_blocks_read_only_allow_delete,
                    blocks_write: f_blocks_write,
                    check_on_startup: f_check_on_startup,
                    codec: f_codec,
                    composite_index_star_tree: f_composite_index_star_tree,
                    creation_date: f_creation_date,
                    creation_date_string: f_creation_date_string,
                    default_pipeline: f_default_pipeline,
                    final_pipeline: f_final_pipeline,
                    format: f_format,
                    gc_deletes: f_gc_deletes,
                    hidden: f_hidden,
                    highlight: f_highlight,
                    highlight_max_analyzed_offset: f_highlight_max_analyzed_offset,
                    index: f_index,
                    indexing: f_indexing,
                    indexing_pressure: f_indexing_pressure,
                    ingestion_source: f_ingestion_source,
                    knn: f_knn,
                    knn_algo_param_ef_search: f_knn_algo_param_ef_search,
                    lifecycle: f_lifecycle,
                    lifecycle_name: f_lifecycle_name,
                    load_fixed_bitset_filters_eagerly: f_load_fixed_bitset_filters_eagerly,
                    mapping: f_mapping,
                    max_docvalue_fields_search: f_max_docvalue_fields_search,
                    max_inner_result_window: f_max_inner_result_window,
                    max_ngram_diff: f_max_ngram_diff,
                    max_refresh_listeners: f_max_refresh_listeners,
                    max_regex_length: f_max_regex_length,
                    max_rescore_window: f_max_rescore_window,
                    max_result_window: f_max_result_window,
                    max_script_fields: f_max_script_fields,
                    max_shingle_diff: f_max_shingle_diff,
                    max_slices_per_scroll: f_max_slices_per_scroll,
                    max_terms_count: f_max_terms_count,
                    merge: f_merge,
                    merge_scheduler_max_thread_count: f_merge_scheduler_max_thread_count,
                    mode: f_mode,
                    number_of_replicas: f_number_of_replicas,
                    number_of_routing_shards: f_number_of_routing_shards,
                    number_of_shards: f_number_of_shards,
                    priority: f_priority,
                    provided_name: f_provided_name,
                    queries: f_queries,
                    query_string: f_query_string,
                    query_string_lenient: f_query_string_lenient,
                    refresh_interval: f_refresh_interval,
                    replication_type: f_replication_type,
                    routing: f_routing,
                    routing_partition_size: f_routing_partition_size,
                    routing_path: f_routing_path,
                    search: f_search,
                    search_idle_after: f_search_idle_after,
                    settings: f_settings,
                    similarity: f_similarity,
                    soft_deletes: f_soft_deletes,
                    soft_deletes_retention_lease_period: f_soft_deletes_retention_lease_period,
                    sort: f_sort,
                    store: f_store,
                    top_metrics_max_size: f_top_metrics_max_size,
                    translog: f_translog,
                    translog_durability: f_translog_durability,
                    translog_flush_threshold_size: f_translog_flush_threshold_size,
                    uuid: f_uuid,
                    verified_before_close: f_verified_before_close,
                    version: f_version,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IndexSettings {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.analysis {
            map.serialize_entry("analysis", v)?;
        }
        if let Some(v) = &self.analyze {
            map.serialize_entry("analyze", v)?;
        }
        if let Some(v) = &self.analyze_max_token_count {
            map.serialize_entry("analyze.max_token_count", v)?;
        }
        if let Some(v) = &self.auto_expand_replicas {
            map.serialize_entry("auto_expand_replicas", v)?;
        }
        if let Some(v) = &self.blocks {
            map.serialize_entry("blocks", v)?;
        }
        if let Some(v) = &self.blocks_metadata {
            map.serialize_entry("blocks.metadata", v)?;
        }
        if let Some(v) = &self.blocks_read {
            map.serialize_entry("blocks.read", v)?;
        }
        if let Some(v) = &self.blocks_read_only {
            map.serialize_entry("blocks.read_only", v)?;
        }
        if let Some(v) = &self.blocks_read_only_allow_delete {
            map.serialize_entry("blocks.read_only_allow_delete", v)?;
        }
        if let Some(v) = &self.blocks_write {
            map.serialize_entry("blocks.write", v)?;
        }
        if let Some(v) = &self.check_on_startup {
            map.serialize_entry("check_on_startup", v)?;
        }
        if let Some(v) = &self.codec {
            map.serialize_entry("codec", v)?;
        }
        if let Some(v) = &self.composite_index_star_tree {
            map.serialize_entry("composite_index.star_tree", v)?;
        }
        if let Some(v) = &self.creation_date {
            map.serialize_entry("creation_date", v)?;
        }
        if let Some(v) = &self.creation_date_string {
            map.serialize_entry("creation_date_string", v)?;
        }
        if let Some(v) = &self.default_pipeline {
            map.serialize_entry("default_pipeline", v)?;
        }
        if let Some(v) = &self.final_pipeline {
            map.serialize_entry("final_pipeline", v)?;
        }
        if let Some(v) = &self.format {
            map.serialize_entry("format", v)?;
        }
        if let Some(v) = &self.gc_deletes {
            map.serialize_entry("gc_deletes", v)?;
        }
        if let Some(v) = &self.hidden {
            map.serialize_entry("hidden", v)?;
        }
        if let Some(v) = &self.highlight {
            map.serialize_entry("highlight", v)?;
        }
        if let Some(v) = &self.highlight_max_analyzed_offset {
            map.serialize_entry("highlight.max_analyzed_offset", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.indexing {
            map.serialize_entry("indexing", v)?;
        }
        if let Some(v) = &self.indexing_pressure {
            map.serialize_entry("indexing_pressure", v)?;
        }
        if let Some(v) = &self.ingestion_source {
            map.serialize_entry("ingestion_source", v)?;
        }
        if let Some(v) = &self.knn {
            map.serialize_entry("knn", v)?;
        }
        if let Some(v) = &self.knn_algo_param_ef_search {
            map.serialize_entry("knn.algo_param.ef_search", v)?;
        }
        if let Some(v) = &self.lifecycle {
            map.serialize_entry("lifecycle", v)?;
        }
        if let Some(v) = &self.lifecycle_name {
            map.serialize_entry("lifecycle.name", v)?;
        }
        if let Some(v) = &self.load_fixed_bitset_filters_eagerly {
            map.serialize_entry("load_fixed_bitset_filters_eagerly", v)?;
        }
        if let Some(v) = &self.mapping {
            map.serialize_entry("mapping", v)?;
        }
        if let Some(v) = &self.max_docvalue_fields_search {
            map.serialize_entry("max_docvalue_fields_search", v)?;
        }
        if let Some(v) = &self.max_inner_result_window {
            map.serialize_entry("max_inner_result_window", v)?;
        }
        if let Some(v) = &self.max_ngram_diff {
            map.serialize_entry("max_ngram_diff", v)?;
        }
        if let Some(v) = &self.max_refresh_listeners {
            map.serialize_entry("max_refresh_listeners", v)?;
        }
        if let Some(v) = &self.max_regex_length {
            map.serialize_entry("max_regex_length", v)?;
        }
        if let Some(v) = &self.max_rescore_window {
            map.serialize_entry("max_rescore_window", v)?;
        }
        if let Some(v) = &self.max_result_window {
            map.serialize_entry("max_result_window", v)?;
        }
        if let Some(v) = &self.max_script_fields {
            map.serialize_entry("max_script_fields", v)?;
        }
        if let Some(v) = &self.max_shingle_diff {
            map.serialize_entry("max_shingle_diff", v)?;
        }
        if let Some(v) = &self.max_slices_per_scroll {
            map.serialize_entry("max_slices_per_scroll", v)?;
        }
        if let Some(v) = &self.max_terms_count {
            map.serialize_entry("max_terms_count", v)?;
        }
        if let Some(v) = &self.merge {
            map.serialize_entry("merge", v)?;
        }
        if let Some(v) = &self.merge_scheduler_max_thread_count {
            map.serialize_entry("merge.scheduler.max_thread_count", v)?;
        }
        if let Some(v) = &self.mode {
            map.serialize_entry("mode", v)?;
        }
        if let Some(v) = &self.number_of_replicas {
            map.serialize_entry("number_of_replicas", v)?;
        }
        if let Some(v) = &self.number_of_routing_shards {
            map.serialize_entry("number_of_routing_shards", v)?;
        }
        if let Some(v) = &self.number_of_shards {
            map.serialize_entry("number_of_shards", v)?;
        }
        if let Some(v) = &self.priority {
            map.serialize_entry("priority", v)?;
        }
        if let Some(v) = &self.provided_name {
            map.serialize_entry("provided_name", v)?;
        }
        if let Some(v) = &self.queries {
            map.serialize_entry("queries", v)?;
        }
        if let Some(v) = &self.query_string {
            map.serialize_entry("query_string", v)?;
        }
        if let Some(v) = &self.query_string_lenient {
            map.serialize_entry("query_string.lenient", v)?;
        }
        if let Some(v) = &self.refresh_interval {
            map.serialize_entry("refresh_interval", v)?;
        }
        if let Some(v) = &self.replication_type {
            map.serialize_entry("replication.type", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("routing", v)?;
        }
        if let Some(v) = &self.routing_partition_size {
            map.serialize_entry("routing_partition_size", v)?;
        }
        if let Some(v) = &self.routing_path {
            map.serialize_entry("routing_path", v)?;
        }
        if let Some(v) = &self.search {
            map.serialize_entry("search", v)?;
        }
        if let Some(v) = &self.search_idle_after {
            map.serialize_entry("search.idle.after", v)?;
        }
        if let Some(v) = &self.settings {
            map.serialize_entry("settings", v)?;
        }
        if let Some(v) = &self.similarity {
            map.serialize_entry("similarity", v)?;
        }
        if let Some(v) = &self.soft_deletes {
            map.serialize_entry("soft_deletes", v)?;
        }
        if let Some(v) = &self.soft_deletes_retention_lease_period {
            map.serialize_entry("soft_deletes.retention_lease.period", v)?;
        }
        if let Some(v) = &self.sort {
            map.serialize_entry("sort", v)?;
        }
        if let Some(v) = &self.store {
            map.serialize_entry("store", v)?;
        }
        if let Some(v) = &self.top_metrics_max_size {
            map.serialize_entry("top_metrics_max_size", v)?;
        }
        if let Some(v) = &self.translog {
            map.serialize_entry("translog", v)?;
        }
        if let Some(v) = &self.translog_durability {
            map.serialize_entry("translog.durability", v)?;
        }
        if let Some(v) = &self.translog_flush_threshold_size {
            map.serialize_entry("translog.flush_threshold_size", v)?;
        }
        if let Some(v) = &self.uuid {
            map.serialize_entry("uuid", v)?;
        }
        if let Some(v) = &self.verified_before_close {
            map.serialize_entry("verified_before_close", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("version", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The text analysis configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsAnalysis {
    #[doc = "The custom analyzer configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Analyzer>>,
    #[doc = "The custom character filter configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_filter:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::CharFilter>>,
    #[doc = "The custom token filter configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::TokenFilter>>,
    #[doc = "The custom normalizer configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Normalizer>>,
    #[doc = "The custom tokenizer configurations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer:
        Option<std::collections::BTreeMap<String, crate::types::common_analysis::Tokenizer>>,
}

#[doc = "The configuration for text analysis."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsAnalyze {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_count: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The configuration for highlighting."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsHighlight {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_analyzed_offset: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The indexing-related settings for an index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsIndexing {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slowlog: Option<crate::types::indices_common::IndexingSlowlog>,
}

#[doc = "The index lifecycle management configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsLifecycle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexing_complete: Option<crate::types::common::StringifiedBoolean>,
    pub name: crate::types::common::Name,
    #[doc = "The timestamp used to calculate the index age for its phase transitions. Use this setting if you create a new index that contains old data and want to use the original creation date to calculate the index age. Specified as a Unix epoch value in milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origination_date: Option<crate::types::common::StringifiedEpochTimeUnitMillis>,
    #[doc = "When `true`, parses the origination date from the index name. This origination date is used to calculate the index age for its phase transitions. The index name must match the pattern `^.*-{date_format}-\\\\d+`, where the `date_format` is `yyyy.MM.dd` and the trailing digits are optional. An index that was rolled over would normally match the full format, for example `logs-2016.10.31-000002`). If the index name doesn't match the pattern, index creation fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parse_origination_date: Option<bool>,
    #[doc = "The index alias to update when the index rolls over. Specify when using a policy that contains a rollover action. When the index rolls over, the alias is updated to reflect that the index is no longer the write index. For more information about rolling indexes, see Rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollover_alias: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<crate::types::indices_common::IndexSettingsLifecycleStep>,
}

#[doc = "The configuration for the lifecycle step execution."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsLifecycleStep {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_time_threshold: Option<crate::types::common::Duration>,
}

#[doc = "The mapping configuration for the index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<crate::types::indices_common::IndexSettingsMappingLimitDepth>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimension_fields:
        Option<crate::types::indices_common::IndexSettingsMappingLimitDimensionFields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name_length:
        Option<crate::types::indices_common::IndexSettingsMappingLimitFieldNameLength>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested_fields: Option<crate::types::indices_common::IndexSettingsMappingLimitNestedFields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nested_objects:
        Option<crate::types::indices_common::IndexSettingsMappingLimitNestedObjects>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_fields: Option<crate::types::indices_common::IndexSettingsMappingLimitTotalFields>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitDepth {
    #[doc = "The maximum depth for a field, which is measured as the number of inner objects. For instance, if all fields are defined at the root object level, then the depth is `1`. If there is one object mapping, then the depth is `2`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitDimensionFields {
    #[doc = "[preview] This functionality is in technical preview and may be changed or removed in a future release. OpenSearch will work to fix any issues, but features in technical preview are not subject to the support SLA of official GA features."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitFieldNameLength {
    #[doc = "Setting for the maximum length of a field name. This setting isn't really something that addresses mappings explosion but might still be useful if you want to limit the field length. It usually shouldn't be necessary to set this setting. The default is okay unless a user starts to add a huge number of fields with really long names. Default is `Long.MAX_VALUE` (no limit)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitNestedFields {
    #[doc = "The maximum number of distinct nested mappings in an index. The nested type should only be used in special cases, when arrays of objects need to be queried independently of each other. To safeguard against poorly designed mappings, this setting limits the number of unique nested types per index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitNestedObjects {
    #[doc = "The maximum number of nested JSON objects that a single document can contain across all nested types. This limit helps to prevent out of memory errors when a document contains too many nested objects."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMappingLimitTotalFields {
    #[doc = "The maximum number of fields in an index. Field and object mappings, as well as field aliases count towards this limit. The limit is in place to prevent mappings and searches from becoming too large. Higher values can lead to performance degradations and memory issues, especially in clusters with a high load or few resources."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::common::StringifiedLong>,
}

#[doc = "The configuration for segment merging."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMerge {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_byte_size_policy:
        Option<crate::types::indices_common::IndexSettingsMergeLogByteSizePolicy>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::types::indices_common::IndexSettingsMergePolicy>,
    #[serde(rename = "policy.deletes_pct_allowed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_deletes_pct_allowed: Option<crate::types::common::StringifiedDouble>,
    #[serde(rename = "policy.expunge_deletes_allowed")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_expunge_deletes_allowed: Option<crate::types::common::StringifiedDouble>,
    #[serde(rename = "policy.floor_segment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_floor_segment: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(rename = "policy.max_merge_at_once")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_max_merge_at_once: Option<crate::types::common::StringifiedInteger>,
    #[serde(rename = "policy.max_merged_segment")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_max_merged_segment: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(rename = "policy.reclaim_deletes_weight")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_reclaim_deletes_weight: Option<crate::types::common::StringifiedDouble>,
    #[serde(rename = "policy.segments_per_tier")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_segments_per_tier: Option<crate::types::common::StringifiedDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheduler: Option<crate::types::indices_common::IndexSettingsMergeScheduler>,
}

#[doc = "The configuration for log byte size merge policy."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMergeLogByteSizePolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merge_segment: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merge_segment_forced_merge: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merged_docs: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub merge_factor: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_merge: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_cfs_ratio: Option<crate::types::common::StringifiedDouble>,
}

#[doc = "The policy configuration for segment merging."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexSettingsMergePolicy {
    Name(crate::types::indices_common::IndexSettingsMergePolicyName),
    TieredPolicyConfig(crate::types::indices_common::IndexSettingsMergeTieredPolicy),
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergePolicy {
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
            Some(b'"') => serde_json::from_str(text).map(IndexSettingsMergePolicy::Name),
            Some(b'{') => {
                serde_json::from_str(text).map(IndexSettingsMergePolicy::TieredPolicyConfig)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IndexSettingsMergePolicy"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The type of merge policy."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsMergePolicyName {
    #[doc = "`default`"]
    Default,
    #[doc = "`log_byte_size`"]
    LogByteSize,
    #[doc = "`tiered`"]
    Tiered,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsMergePolicyName {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsMergePolicyName::Default => "default",
            IndexSettingsMergePolicyName::LogByteSize => "log_byte_size",
            IndexSettingsMergePolicyName::Tiered => "tiered",
            IndexSettingsMergePolicyName::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsMergePolicyName {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsMergePolicyName {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "default" => IndexSettingsMergePolicyName::Default,
            "log_byte_size" => IndexSettingsMergePolicyName::LogByteSize,
            "tiered" => IndexSettingsMergePolicyName::Tiered,
            _ => IndexSettingsMergePolicyName::Unknown(s),
        })
    }
}

#[doc = "The configuration for merge scheduling."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMergeScheduler {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_throttle: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merge_count: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_thread_count: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The configuration for tiered merge policy."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsMergeTieredPolicy {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deletes_pct_allowed: Option<crate::types::common::StringifiedDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expunge_deletes_allowed: Option<crate::types::common::StringifiedDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_segment: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merge_at_once: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merge_at_once_explicit: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_merged_segment: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reclaim_deletes_weight: Option<crate::types::common::StringifiedDouble>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub segments_per_tier: Option<crate::types::common::StringifiedDouble>,
}

#[doc = "The configuration for query caching."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsQueries {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cache: Option<crate::types::indices_common::IndexSettingsQueriesCache>,
}

#[doc = "The query cache settings."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsQueriesCache {
    pub enabled: crate::types::common::StringifiedBoolean,
}

#[doc = "The configuration for query string parsing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsQueryString {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<crate::types::common::StringifiedBoolean>,
}

#[doc = "The configuration for search operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent: Option<crate::types::indices_common::IndexSettingsSearchConcurrent>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub concurrent_segment_search:
        Option<crate::types::indices_common::IndexSettingsSearchConcurrentSegmentSearch>,
    #[doc = "The default search pipeline to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_pipeline: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle: Option<crate::types::indices_common::SearchIdle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slowlog: Option<crate::types::indices_common::SearchSlowlog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub star_tree_index: Option<crate::types::indices_common::IndexSettingsSearchStarTreeIndex>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub throttled: Option<crate::types::common::StringifiedBoolean>,
}

#[doc = "The configuration for concurrent search operations."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSearchConcurrent {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_slice_count: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The configuration for concurrent segment search."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSearchConcurrentSegmentSearch {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "The mode of concurrent segment search."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSearchStarTreeIndex {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
}

#[doc = "The configuration for similarity algorithms."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarity {
    #[serde(rename = "bm25")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bm_25: Option<crate::types::indices_common::IndexSettingsSimilarityBm25>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfi: Option<crate::types::indices_common::IndexSettingsSimilarityDfi>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dfr: Option<crate::types::indices_common::IndexSettingsSimilarityDfr>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ib: Option<crate::types::indices_common::IndexSettingsSimilarityIb>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lmd: Option<crate::types::indices_common::IndexSettingsSimilarityLmd>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lmj: Option<crate::types::indices_common::IndexSettingsSimilarityLmj>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scripted_tfidf: Option<crate::types::indices_common::IndexSettingsSimilarityScriptedTfidf>,
}

#[doc = "The BM25 similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityBm25 {
    #[doc = "The length normalization parameter."]
    pub b: f64,
    #[doc = "Whether to discount overlapping tokens."]
    pub discount_overlaps: bool,
    #[doc = "The term frequency normalization parameter."]
    #[serde(rename = "k1")]
    pub k_1: f64,
    #[doc = "The BM25 similarity algorithm."]
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityBm25Type,
}

#[doc = "The BM25 similarity algorithm."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityBm25Type {
    #[doc = "`BM25`"]
    Bm25,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityBm25Type {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityBm25Type::Bm25 => "BM25",
            IndexSettingsSimilarityBm25Type::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityBm25Type {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityBm25Type {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BM25" => IndexSettingsSimilarityBm25Type::Bm25,
            _ => IndexSettingsSimilarityBm25Type::Unknown(s),
        })
    }
}

#[doc = "The divergence from independence (DFI) similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityDfi {
    pub independence_measure: crate::types::common::DFIIndependenceMeasure,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityDfiType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityDfiType {
    #[doc = "`DFI`"]
    Dfi,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityDfiType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityDfiType::Dfi => "DFI",
            IndexSettingsSimilarityDfiType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfiType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfiType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DFI" => IndexSettingsSimilarityDfiType::Dfi,
            _ => IndexSettingsSimilarityDfiType::Unknown(s),
        })
    }
}

#[doc = "The divergence from randomness (DFR) similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityDfr {
    pub after_effect: crate::types::common::DFRAfterEffect,
    pub basic_model: crate::types::common::DFRBasicModel,
    pub normalization: crate::types::common::TermFrequencyNormalization,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityDfrType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityDfrType {
    #[doc = "`DFR`"]
    Dfr,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityDfrType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityDfrType::Dfr => "DFR",
            IndexSettingsSimilarityDfrType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityDfrType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityDfrType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DFR" => IndexSettingsSimilarityDfrType::Dfr,
            _ => IndexSettingsSimilarityDfrType::Unknown(s),
        })
    }
}

#[doc = "The information-based similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityIb {
    pub distribution: crate::types::common::IBDistribution,
    pub lambda: crate::types::common::IBLambda,
    pub normalization: crate::types::common::TermFrequencyNormalization,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityIbType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityIbType {
    #[doc = "`IB`"]
    Ib,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityIbType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityIbType::Ib => "IB",
            IndexSettingsSimilarityIbType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityIbType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityIbType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "IB" => IndexSettingsSimilarityIbType::Ib,
            _ => IndexSettingsSimilarityIbType::Unknown(s),
        })
    }
}

#[doc = "The LM Dirichlet similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityLmd {
    #[doc = "The smoothing parameter."]
    pub mu: f64,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityLmdType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityLmdType {
    #[doc = "`LMDirichlet`"]
    Lmdirichlet,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityLmdType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityLmdType::Lmdirichlet => "LMDirichlet",
            IndexSettingsSimilarityLmdType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmdType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmdType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LMDirichlet" => IndexSettingsSimilarityLmdType::Lmdirichlet,
            _ => IndexSettingsSimilarityLmdType::Unknown(s),
        })
    }
}

#[doc = "The LM Jelinek-Mercer similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityLmj {
    #[doc = "The optimal mixture parameter."]
    pub lambda: f64,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityLmjType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityLmjType {
    #[doc = "`LMJelinekMercer`"]
    LmjelinekMercer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityLmjType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityLmjType::LmjelinekMercer => "LMJelinekMercer",
            IndexSettingsSimilarityLmjType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityLmjType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityLmjType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "LMJelinekMercer" => IndexSettingsSimilarityLmjType::LmjelinekMercer,
            _ => IndexSettingsSimilarityLmjType::Unknown(s),
        })
    }
}

#[doc = "The scripted TF/IDF similarity algorithm configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsSimilarityScriptedTfidf {
    pub script: crate::types::common::Script,
    pub r#type: crate::types::indices_common::IndexSettingsSimilarityScriptedTfidfType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsSimilarityScriptedTfidfType {
    #[doc = "`scripted`"]
    Scripted,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsSimilarityScriptedTfidfType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsSimilarityScriptedTfidfType::Scripted => "scripted",
            IndexSettingsSimilarityScriptedTfidfType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsSimilarityScriptedTfidfType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsSimilarityScriptedTfidfType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "scripted" => IndexSettingsSimilarityScriptedTfidfType::Scripted,
            _ => IndexSettingsSimilarityScriptedTfidfType::Unknown(s),
        })
    }
}

#[doc = "The star tree indexing configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStarTree {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<crate::types::indices_common::IndexSettingsStarTreeDefault>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<crate::types::indices_common::IndexSettingsStarTreeField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_fields: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The default settings for star tree indexing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStarTreeDefault {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_leaf_docs: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The field-specific settings for star tree indexing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStarTreeField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<crate::types::indices_common::IndexSettingsStarTreeFieldDefault>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_base_metrics: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_date_intervals: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_dimensions: Option<crate::types::common::StringifiedInteger>,
}

#[doc = "The field-specific settings for star tree indexing."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStarTreeFieldDefault {
    #[doc = "The list of date intervals to use."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_intervals: Option<Vec<String>>,
    #[doc = "The list of metrics to calculate."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStore {
    #[doc = "Whether memory-mapping is allowed. You can restrict the use of the `mmapfs` and the related `hybridfs` store types with the setting `node.store.allow_mmap`. This setting is useful, for example, if you are in an environment where you can not control the ability to create a lot of memory maps so you need disable the ability to use memory-mapping."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_mmap: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs: Option<crate::types::indices_common::IndexSettingsStoreFs>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hybrid: Option<crate::types::indices_common::IndexSettingsStoreHybrid>,
    #[doc = "The list of files to preload into memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preload: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_refresh_interval: Option<crate::types::common::Duration>,
    pub r#type: crate::types::indices_common::StorageType,
}

#[doc = "The filesystem storage configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStoreFs {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fs_lock: Option<crate::types::indices_common::IndexSettingsStoreFsLock>,
}

#[doc = "The type of file system lock."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexSettingsStoreFsLock {
    #[doc = "`native`"]
    Native,
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexSettingsStoreFsLock {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexSettingsStoreFsLock::Native => "native",
            IndexSettingsStoreFsLock::Simple => "simple",
            IndexSettingsStoreFsLock::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexSettingsStoreFsLock {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexSettingsStoreFsLock {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "native" => IndexSettingsStoreFsLock::Native,
            "simple" => IndexSettingsStoreFsLock::Simple,
            _ => IndexSettingsStoreFsLock::Unknown(s),
        })
    }
}

#[doc = "The hybrid storage type configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStoreHybrid {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mmap: Option<crate::types::indices_common::IndexSettingsStoreHybridMmap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nio: Option<crate::types::indices_common::IndexSettingsStoreHybridNio>,
}

#[doc = "The memory-mapped storage configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStoreHybridMmap {
    #[doc = "The file extensions to use memory-mapping for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
}

#[doc = "The NIO storage configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexSettingsStoreHybridNio {
    #[doc = "The file extensions to use NIO for."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<Vec<String>>,
}

#[doc = "The state configuration of an index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexState {
    #[doc = "The aliases associated with the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<std::collections::BTreeMap<String, crate::types::indices_common::Alias>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_stream: Option<crate::types::common::DataStreamName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub defaults: Option<crate::types::indices_common::IndexSettings>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::types::indices_common::IndexSettings>,
}

#[doc = "The configuration for an index template."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexTemplate {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "Whether the template allows automatic index creation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_auto_create: Option<bool>,
    #[doc = "An ordered list of component template names. Component templates are merged in the order specified, meaning that the last component template specified has the highest precedence."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub composed_of: Option<Vec<crate::types::common::Name>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_stream: Option<crate::types::indices_common::IndexTemplateDataStreamConfiguration>,
    pub index_patterns: crate::types::common::Names,
    #[doc = "The priority to determine index template precedence when a new data stream or index is created. The index template with the highest priority is chosen. If no priority is specified the template is treated as though it is of priority 0 (lowest priority). This number is not automatically generated by OpenSearch."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template: Option<crate::types::indices_common::IndexTemplateSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}

#[doc = "The data stream configuration for an index template."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexTemplateDataStreamConfiguration {
    #[doc = "When `true`, the data stream supports custom routing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_custom_routing: Option<bool>,
    #[doc = "When `true`, the data stream is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_field: Option<crate::types::indices_common::DataStreamTimestampField>,
}

#[doc = "The summary configuration for an index template."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexTemplateSummary {
    #[doc = "The aliases to add. If the index template includes a `data_stream` object, these are data stream aliases. Otherwise, these are index aliases. Data stream aliases ignore the `index_routing`, `routing`, and `search_routing` options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<std::collections::BTreeMap<String, crate::types::indices_common::Alias>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<crate::types::common_mapping::TypeMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<crate::types::indices_common::IndexSettings>,
}

#[doc = "The versioning information for the index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexVersioning {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::types::common::VersionString>,
    #[doc = "The string representation of the version when the index was created."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_string: Option<String>,
}

#[doc = "The configuration for indexing backpressure."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressure {
    pub memory: crate::types::indices_common::IndexingPressureMemory,
}

#[doc = "The memory-related settings for indexing backpressure."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingPressureMemory {
    #[doc = "The number of outstanding bytes that may be consumed by indexing requests. When this limit is reached or exceeded, the node will reject new coordinating and primary operations. When replica operations consume 1.5x this limit, the node will reject new replica operations. Defaults to 10% of the heap."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<crate::types::indices_common::IndexingPressureMemoryLimit>,
}

#[doc = "The number of outstanding bytes that may be consumed by indexing requests. When this limit is reached or exceeded, the node will reject new coordinating and primary operations. When replica operations consume 1.5x this limit, the node will reject new replica operations. Defaults to 10% of the heap."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IndexingPressureMemoryLimit {
    String(String),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for IndexingPressureMemoryLimit {
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
            Some(b'"') => serde_json::from_str(text).map(IndexingPressureMemoryLimit::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(IndexingPressureMemoryLimit::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IndexingPressureMemoryLimit"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The configuration for indexing slow log."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingSlowlog {
    #[doc = "The log level for slow indexing operations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reformat: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<crate::types::indices_common::IndexingSlowlogThresholds>,
}

#[doc = "The threshold configuration for indexing slow log."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexingSlowlogThresholds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<crate::types::indices_common::SlowlogThresholdLevels>,
}

#[doc = "The pull-based ingestion settings controlling how data is read from streaming sources."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestionSource {
    #[doc = "Indicates if all-active ingestion is enabled for this index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_active: Option<crate::types::common::StringifiedBoolean>,
    #[doc = "Defines the policy for error handling."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_strategy: Option<crate::types::ingestion_common::ErrorPolicy>,
    #[doc = "Defines the size of the internal blocking queue in pull-based ingestion."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_queue_size: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Defines the number of processor or writer threads."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_processor_threads: Option<crate::types::common::StringifiedInteger>,
    #[doc = "Custom parameters for the ingestion source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub param: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer: Option<crate::types::indices_common::IngestionSourcePointer>,
    #[serde(rename = "pointer.init.reset")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer_init_reset: Option<crate::types::indices_common::IngestionSourcePointerInitReset>,
    #[doc = "Specifies the offset value or timestamp in milliseconds."]
    #[serde(rename = "pointer.init.reset.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pointer_init_reset_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: Option<crate::types::indices_common::IngestionSourcePoll>,
    #[doc = "The maximum number of records to retrieve in each poll operation."]
    #[serde(rename = "poll.max_batch_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_max_batch_size: Option<crate::types::common::StringifiedLong>,
    #[doc = "Determines the Poll timeout value."]
    #[serde(rename = "poll.timeout")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll_timeout: Option<crate::types::common::StringifiedInteger>,
    #[doc = "The streaming source type."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::indices_common::IngestionSourceType>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestionSourcePointer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub init: Option<crate::types::indices_common::IngestionSourcePointerInit>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestionSourcePointerInit {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset: Option<crate::types::indices_common::IngestionSourcePointerInitReset>,
    #[doc = "Specifies the offset value or timestamp in milliseconds."]
    #[serde(rename = "reset.value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reset_value: Option<String>,
}

#[doc = "Determines where to start reading from the stream."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestionSourcePointerInitReset {
    #[doc = "`EARLIEST` (also accepted: `earliest`)"]
    Earliest,
    #[doc = "`LATEST` (also accepted: `latest`)"]
    Latest,
    #[doc = "`NONE` (also accepted: `none`)"]
    None,
    #[doc = "`RESET_BY_OFFSET` (also accepted: `reset_by_offset`)"]
    ResetByOffset,
    #[doc = "`RESET_BY_TIMESTAMP` (also accepted: `reset_by_timestamp`)"]
    ResetByTimestamp,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IngestionSourcePointerInitReset {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IngestionSourcePointerInitReset::Earliest => "EARLIEST",
            IngestionSourcePointerInitReset::Latest => "LATEST",
            IngestionSourcePointerInitReset::None => "NONE",
            IngestionSourcePointerInitReset::ResetByOffset => "RESET_BY_OFFSET",
            IngestionSourcePointerInitReset::ResetByTimestamp => "RESET_BY_TIMESTAMP",
            IngestionSourcePointerInitReset::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IngestionSourcePointerInitReset {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IngestionSourcePointerInitReset {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "EARLIEST" | "earliest" => IngestionSourcePointerInitReset::Earliest,
            "LATEST" | "latest" => IngestionSourcePointerInitReset::Latest,
            "NONE" | "none" => IngestionSourcePointerInitReset::None,
            "RESET_BY_OFFSET" | "reset_by_offset" => IngestionSourcePointerInitReset::ResetByOffset,
            "RESET_BY_TIMESTAMP" | "reset_by_timestamp" => {
                IngestionSourcePointerInitReset::ResetByTimestamp
            }
            _ => IngestionSourcePointerInitReset::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IngestionSourcePoll {
    #[doc = "The maximum number of records to retrieve in each poll operation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_batch_size: Option<crate::types::common::StringifiedLong>,
    #[doc = "Determines the Poll timeout value."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::types::common::StringifiedInteger>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IngestionSourceType {
    #[doc = "`file`"]
    File,
    #[doc = "`kafka`"]
    Kafka,
    #[doc = "`kinesis`"]
    Kinesis,
    #[doc = "`none`"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IngestionSourceType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IngestionSourceType::File => "file",
            IngestionSourceType::Kafka => "kafka",
            IngestionSourceType::Kinesis => "kinesis",
            IngestionSourceType::None => "none",
            IngestionSourceType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IngestionSourceType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IngestionSourceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "file" => IngestionSourceType::File,
            "kafka" => IngestionSourceType::Kafka,
            "kinesis" => IngestionSourceType::Kinesis,
            "none" => IngestionSourceType::None,
            _ => IngestionSourceType::Unknown(s),
        })
    }
}

#[doc = "The component managing the data stream or index."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ManagedBy {
    #[doc = "`Data stream lifecycle`"]
    DataStreamLifecycle,
    #[doc = "`Index Lifecycle Management`"]
    IndexLifecycleManagement,
    #[doc = "`Unmanaged`"]
    Unmanaged,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ManagedBy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ManagedBy::DataStreamLifecycle => "Data stream lifecycle",
            ManagedBy::IndexLifecycleManagement => "Index Lifecycle Management",
            ManagedBy::Unmanaged => "Unmanaged",
            ManagedBy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ManagedBy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ManagedBy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Data stream lifecycle" => ManagedBy::DataStreamLifecycle,
            "Index Lifecycle Management" => ManagedBy::IndexLifecycleManagement,
            "Unmanaged" => ManagedBy::Unmanaged,
            _ => ManagedBy::Unknown(s),
        })
    }
}

#[doc = "The configuration for numeric field data."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NumericFielddata {
    pub format: crate::types::indices_common::NumericFielddataFormat,
}

#[doc = "The format for numeric field data."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NumericFielddataFormat {
    #[doc = "`array`"]
    Array,
    #[doc = "`disabled`"]
    Disabled,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NumericFielddataFormat {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NumericFielddataFormat::Array => "array",
            NumericFielddataFormat::Disabled => "disabled",
            NumericFielddataFormat::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NumericFielddataFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NumericFielddataFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "array" => NumericFielddataFormat::Array,
            "disabled" => NumericFielddataFormat::Disabled,
            _ => NumericFielddataFormat::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReplicationType {
    #[doc = "`DOCUMENT`"]
    Document,
    #[doc = "`SEGMENT`"]
    Segment,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReplicationType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReplicationType::Document => "DOCUMENT",
            ReplicationType::Segment => "SEGMENT",
            ReplicationType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReplicationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReplicationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "DOCUMENT" => ReplicationType::Document,
            "SEGMENT" => ReplicationType::Segment,
            _ => ReplicationType::Unknown(s),
        })
    }
}

#[doc = "The retention lease configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetentionLease {
    pub period: crate::types::common::Duration,
}

#[doc = "The configuration for search idle behavior."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchIdle {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after: Option<crate::types::common::Duration>,
}

#[doc = "The configuration for search slow log."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchSlowlog {
    #[doc = "The log level for slow searches."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[doc = "Whether to reformat the logged search source."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reformat: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<crate::types::indices_common::SearchSlowlogThresholds>,
}

#[doc = "The threshold configuration for search slow log."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchSlowlogThresholds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fetch: Option<crate::types::indices_common::SlowlogThresholdLevels>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::types::indices_common::SlowlogThresholdLevels>,
}

#[doc = "The handling of missing values in segment sorting."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortMissing {
    #[doc = "`_first`"]
    First,
    #[doc = "`_last`"]
    Last,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortMissing {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortMissing::First => "_first",
            SegmentSortMissing::Last => "_last",
            SegmentSortMissing::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortMissing {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortMissing {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "_first" => SegmentSortMissing::First,
            "_last" => SegmentSortMissing::Last,
            _ => SegmentSortMissing::Unknown(s),
        })
    }
}

#[doc = "The sort mode for segments."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortMode {
    #[doc = "`max`"]
    Max,
    #[doc = "`min`"]
    Min,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortMode::Max => "max",
            SegmentSortMode::Min => "min",
            SegmentSortMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "max" => SegmentSortMode::Max,
            "min" => SegmentSortMode::Min,
            _ => SegmentSortMode::Unknown(s),
        })
    }
}

#[doc = "The sort order for segments."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SegmentSortOrder {
    #[doc = "`asc`"]
    Asc,
    #[doc = "`desc`"]
    Desc,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SegmentSortOrder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SegmentSortOrder::Asc => "asc",
            SegmentSortOrder::Desc => "desc",
            SegmentSortOrder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SegmentSortOrder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SegmentSortOrder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "asc" => SegmentSortOrder::Asc,
            "desc" => SegmentSortOrder::Desc,
            _ => SegmentSortOrder::Unknown(s),
        })
    }
}

#[doc = "The threshold levels for slow log."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlowlogThresholdLevels {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub debug: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub warn: Option<crate::types::common::Duration>,
}

#[doc = "The configuration for soft deletes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftDeletes {
    #[doc = "Whether soft deletes are enabled on the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<crate::types::indices_common::SoftDeletesRetention>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention_lease: Option<crate::types::indices_common::RetentionLease>,
}

#[doc = "The retention configuration for soft deletes."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SoftDeletesRetention {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operations: Option<crate::types::common::StringifiedLong>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct StorageType {
    raw: Box<serde_json::value::RawValue>,
}
impl StorageType {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `builtin` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_builtin(
        &self,
    ) -> std::result::Result<
        crate::types::indices_common::BuiltinStorageType,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "builtin")
    }
    #[doc = "Parses the value as the `custom` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "custom")
    }
}

#[doc = "The mapping configuration for a template."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TemplateMapping {
    pub aliases: std::collections::BTreeMap<String, crate::types::indices_common::Alias>,
    pub index_patterns: Vec<crate::types::common::Name>,
    pub mappings: crate::types::common_mapping::TypeMapping,
    #[doc = "The order in which the template should be applied."]
    pub order: i32,
    #[doc = "The index settings to apply."]
    pub settings: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}

#[doc = "The translog configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Translog {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub durability: Option<crate::types::indices_common::TranslogDurability>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flush_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generation_threshold_size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retention: Option<crate::types::indices_common::TranslogRetention>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sync_interval: Option<crate::types::common::Duration>,
}

#[doc = "The durability settings for the translog."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TranslogDurability {
    #[doc = "`ASYNC` (also accepted: `async`)"]
    Async,
    #[doc = "`REQUEST` (also accepted: `request`)"]
    Request,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TranslogDurability {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TranslogDurability::Async => "ASYNC",
            TranslogDurability::Request => "REQUEST",
            TranslogDurability::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TranslogDurability {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TranslogDurability {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ASYNC" | "async" => TranslogDurability::Async,
            "REQUEST" | "request" => TranslogDurability::Request,
            _ => TranslogDurability::Unknown(s),
        })
    }
}

#[doc = "The retention configuration for the translog."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TranslogRetention {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub age: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common::HumanReadableByteCount>,
}

#[doc = "The upgrade status details for an index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpgradeStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_to_upgrade_ancient_in_bytes: Option<crate::types::common::ByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_to_upgrade_in_bytes: Option<crate::types::common::ByteCount>,
}

#[doc = "The version status for an upgrade operation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpgradeVersionStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oldest_lucene_segment_version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_version: Option<crate::types::common::VersionString>,
}
