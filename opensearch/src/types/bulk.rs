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

//! Response types of the `bulk` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct BulkResponse {
    pub errors: bool,
    pub ingest_took: Option<i64>,
    pub items: Vec<crate::types::bulk::Item>,
    pub took: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BulkResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BulkResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BulkResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_errors: Option<bool> = None;
                let mut f_ingest_took: Option<i64> = None;
                let mut seen_ingest_took = false;
                let mut f_items: Option<Vec<crate::types::bulk::Item>> = None;
                let mut f_took: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "errors" => {
                            if f_errors.is_some() {
                                return Err(serde::de::Error::duplicate_field("errors"));
                            }
                            f_errors = Some(map.next_value::<bool>()?);
                        }
                        "ingest_took" => {
                            if seen_ingest_took {
                                return Err(serde::de::Error::duplicate_field("ingest_took"));
                            }
                            seen_ingest_took = true;
                            f_ingest_took = map.next_value::<Option<i64>>()?;
                        }
                        "items" => {
                            if f_items.is_some() {
                                return Err(serde::de::Error::duplicate_field("items"));
                            }
                            f_items = Some(map.next_value::<Vec<crate::types::bulk::Item>>()?);
                        }
                        "took" => {
                            if f_took.is_some() {
                                return Err(serde::de::Error::duplicate_field("took"));
                            }
                            f_took = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BulkResponse {
                    errors: f_errors.ok_or_else(|| serde::de::Error::missing_field("errors"))?,
                    ingest_took: f_ingest_took,
                    items: f_items.ok_or_else(|| serde::de::Error::missing_field("items"))?,
                    took: f_took.ok_or_else(|| serde::de::Error::missing_field("took"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BulkResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("errors", &self.errors)?;
        if let Some(v) = &self.ingest_took {
            map.serialize_entry("ingest_took", v)?;
        }
        map.serialize_entry("items", &self.items)?;
        map.serialize_entry("took", &self.took)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub create: Option<crate::types::bulk::ResponseItem>,
    pub delete: Option<crate::types::bulk::ResponseItem>,
    pub index: Option<crate::types::bulk::ResponseItem>,
    pub update: Option<crate::types::bulk::ResponseItem>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for Item {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = Item;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a Item object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_create: Option<crate::types::bulk::ResponseItem> = None;
                let mut seen_create = false;
                let mut f_delete: Option<crate::types::bulk::ResponseItem> = None;
                let mut seen_delete = false;
                let mut f_index: Option<crate::types::bulk::ResponseItem> = None;
                let mut seen_index = false;
                let mut f_update: Option<crate::types::bulk::ResponseItem> = None;
                let mut seen_update = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "create" => {
                            if seen_create {
                                return Err(serde::de::Error::duplicate_field("create"));
                            }
                            seen_create = true;
                            f_create =
                                map.next_value::<Option<crate::types::bulk::ResponseItem>>()?;
                        }
                        "delete" => {
                            if seen_delete {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            seen_delete = true;
                            f_delete =
                                map.next_value::<Option<crate::types::bulk::ResponseItem>>()?;
                        }
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::bulk::ResponseItem>>()?;
                        }
                        "update" => {
                            if seen_update {
                                return Err(serde::de::Error::duplicate_field("update"));
                            }
                            seen_update = true;
                            f_update =
                                map.next_value::<Option<crate::types::bulk::ResponseItem>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(Item {
                    create: f_create,
                    delete: f_delete,
                    index: f_index,
                    update: f_update,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for Item {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.create {
            map.serialize_entry("create", v)?;
        }
        if let Some(v) = &self.delete {
            map.serialize_entry("delete", v)?;
        }
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.update {
            map.serialize_entry("update", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ResponseItem {
    #[doc = "The document ID associated with the operation."]
    pub id: Option<String>,
    #[doc = "The name of the index associated with the operation. If the operation targets a data stream, this is the backing index into which the document was written."]
    pub index: String,
    #[doc = "The primary term assigned to the document for the operation."]
    pub primary_term: Option<i64>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    pub shards: Option<crate::types::common::ShardInfo>,
    #[doc = "The document type."]
    pub r#type: Option<String>,
    pub version: Option<crate::types::common::VersionNumber>,
    pub error: Option<crate::types::common::ErrorCause>,
    pub forced_refresh: Option<bool>,
    pub get: Option<crate::types::common::InlineGetDictUserDefined>,
    #[doc = "Result of the operation. Successful values are `created`, `deleted`, and `updated`."]
    pub result: Option<String>,
    #[doc = "HTTP status code returned for the operation."]
    pub status: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ResponseItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ResponseItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ResponseItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<String> = None;
                let mut seen_id = false;
                let mut f_index: Option<String> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_shards: Option<crate::types::common::ShardInfo> = None;
                let mut seen_shards = false;
                let mut f_type: Option<String> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut seen_version = false;
                let mut f_error: Option<crate::types::common::ErrorCause> = None;
                let mut seen_error = false;
                let mut f_forced_refresh: Option<bool> = None;
                let mut seen_forced_refresh = false;
                let mut f_get: Option<crate::types::common::InlineGetDictUserDefined> = None;
                let mut seen_get = false;
                let mut f_result: Option<String> = None;
                let mut seen_result = false;
                let mut f_status: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if seen_id {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            seen_id = true;
                            f_id = map.next_value::<Option<String>>()?;
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<String>()?);
                        }
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_shards" => {
                            if seen_shards {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            seen_shards = true;
                            f_shards =
                                map.next_value::<Option<crate::types::common::ShardInfo>>()?;
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<String>>()?;
                        }
                        "_version" => {
                            if seen_version {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            seen_version = true;
                            f_version =
                                map.next_value::<Option<crate::types::common::VersionNumber>>()?;
                        }
                        "error" => {
                            if seen_error {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            seen_error = true;
                            f_error =
                                map.next_value::<Option<crate::types::common::ErrorCause>>()?;
                        }
                        "forced_refresh" => {
                            if seen_forced_refresh {
                                return Err(serde::de::Error::duplicate_field("forced_refresh"));
                            }
                            seen_forced_refresh = true;
                            f_forced_refresh = map.next_value::<Option<bool>>()?;
                        }
                        "get" => {
                            if seen_get {
                                return Err(serde::de::Error::duplicate_field("get"));
                            }
                            seen_get = true;
                            f_get = map.next_value::<Option<crate::types::common::InlineGetDictUserDefined>>()?;
                        }
                        "result" => {
                            if seen_result {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            seen_result = true;
                            f_result = map.next_value::<Option<String>>()?;
                        }
                        "status" => {
                            if f_status.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            f_status = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ResponseItem {
                    id: f_id,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    primary_term: f_primary_term,
                    seq_no: f_seq_no,
                    shards: f_shards,
                    r#type: f_type,
                    version: f_version,
                    error: f_error,
                    forced_refresh: f_forced_refresh,
                    get: f_get,
                    result: f_result,
                    status: f_status.ok_or_else(|| serde::de::Error::missing_field("status"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ResponseItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.id {
            map.serialize_entry("_id", v)?;
        }
        map.serialize_entry("_index", &self.index)?;
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.shards {
            map.serialize_entry("_shards", v)?;
        }
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        if let Some(v) = &self.version {
            map.serialize_entry("_version", v)?;
        }
        if let Some(v) = &self.error {
            map.serialize_entry("error", v)?;
        }
        if let Some(v) = &self.forced_refresh {
            map.serialize_entry("forced_refresh", v)?;
        }
        if let Some(v) = &self.get {
            map.serialize_entry("get", v)?;
        }
        if let Some(v) = &self.result {
            map.serialize_entry("result", v)?;
        }
        map.serialize_entry("status", &self.status)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
