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

//! Response types of the `indices_shard_stores` specification namespace

#![allow(missing_docs)]

#[doc = "The shard store information for indices."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndicesShardStores {
    #[doc = "The store information for each shard."]
    pub shards:
        std::collections::BTreeMap<String, crate::types::indices_shard_stores::ShardStoreWrapper>,
}

#[doc = "The store information for a shard."]
#[derive(Debug, Clone)]
pub struct ShardStore {
    pub allocation: crate::types::indices_shard_stores::ShardStoreAllocation,
    pub allocation_id: Option<crate::types::common::Id>,
    pub store_exception: Option<crate::types::indices_shard_stores::ShardStoreException>,
    pub additional: std::collections::BTreeMap<String, crate::types::common::NodeAttributes>,
}
impl<'de> serde::Deserialize<'de> for ShardStore {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStore;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStore object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_allocation: Option<
                    crate::types::indices_shard_stores::ShardStoreAllocation,
                > = None;
                let mut f_allocation_id: Option<crate::types::common::Id> = None;
                let mut seen_allocation_id = false;
                let mut f_store_exception: Option<
                    crate::types::indices_shard_stores::ShardStoreException,
                > = None;
                let mut seen_store_exception = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common::NodeAttributes,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "allocation" => {
                            if f_allocation.is_some() {
                                return Err(serde::de::Error::duplicate_field("allocation"));
                            }
                            f_allocation = Some(map.next_value::<crate::types::indices_shard_stores::ShardStoreAllocation>()?);
                        }
                        "allocation_id" => {
                            if seen_allocation_id {
                                return Err(serde::de::Error::duplicate_field("allocation_id"));
                            }
                            seen_allocation_id = true;
                            f_allocation_id =
                                map.next_value::<Option<crate::types::common::Id>>()?;
                        }
                        "store_exception" => {
                            if seen_store_exception {
                                return Err(serde::de::Error::duplicate_field("store_exception"));
                            }
                            seen_store_exception = true;
                            f_store_exception = map.next_value::<Option<crate::types::indices_shard_stores::ShardStoreException>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStore {
                    allocation: f_allocation
                        .ok_or_else(|| serde::de::Error::missing_field("allocation"))?,
                    allocation_id: f_allocation_id,
                    store_exception: f_store_exception,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStore {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("allocation", &self.allocation)?;
        if let Some(v) = &self.allocation_id {
            map.serialize_entry("allocation_id", v)?;
        }
        if let Some(v) = &self.store_exception {
            map.serialize_entry("store_exception", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The allocation type of the shard store."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShardStoreAllocation {
    #[doc = "`primary`"]
    Primary,
    #[doc = "`replica`"]
    Replica,
    #[doc = "`unused`"]
    Unused,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShardStoreAllocation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShardStoreAllocation::Primary => "primary",
            ShardStoreAllocation::Replica => "replica",
            ShardStoreAllocation::Unused => "unused",
            ShardStoreAllocation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShardStoreAllocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShardStoreAllocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "primary" => ShardStoreAllocation::Primary,
            "replica" => ShardStoreAllocation::Replica,
            "unused" => ShardStoreAllocation::Unused,
            _ => ShardStoreAllocation::Unknown(s),
        })
    }
}

#[doc = "The exception information for a shard store."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardStoreException {
    #[doc = "The reason for the store exception."]
    pub reason: String,
    #[doc = "The type of store exception."]
    pub r#type: String,
}

#[doc = "The wrapper containing store information for a shard."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShardStoreWrapper {
    #[doc = "The list of stores for the shard."]
    pub stores: Vec<crate::types::indices_shard_stores::ShardStore>,
}

#[doc = "The response of the `indices.shard_stores` API"]
pub type ShardStoresResponse = crate::types::indices::ShardStoresResponse;
