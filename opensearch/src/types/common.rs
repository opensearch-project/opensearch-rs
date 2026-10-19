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

//! Response types of the `common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone)]
pub struct ClusterStatistics {
    pub skipped: i32,
    pub successful: i32,
    pub total: i32,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ClusterStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ClusterStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ClusterStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_skipped: Option<i32> = None;
                let mut f_successful: Option<i32> = None;
                let mut f_total: Option<i32> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "skipped" => {
                            if f_skipped.is_some() {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            f_skipped = Some(map.next_value::<i32>()?);
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<i32>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<i32>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ClusterStatistics {
                    skipped: f_skipped.ok_or_else(|| serde::de::Error::missing_field("skipped"))?,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ClusterStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("skipped", &self.skipped)?;
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The geographical bounds specified using coordinate values."]
#[derive(Debug, Clone)]
pub struct CoordsGeoBounds {
    #[doc = "The bottom boundary latitude."]
    pub bottom: f64,
    #[doc = "The left boundary longitude."]
    pub left: f64,
    #[doc = "The right boundary longitude."]
    pub right: f64,
    #[doc = "The top boundary latitude."]
    pub top: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CoordsGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CoordsGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CoordsGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom: Option<f64> = None;
                let mut f_left: Option<f64> = None;
                let mut f_right: Option<f64> = None;
                let mut f_top: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom" => {
                            if f_bottom.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom"));
                            }
                            f_bottom = Some(map.next_value::<f64>()?);
                        }
                        "left" => {
                            if f_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("left"));
                            }
                            f_left = Some(map.next_value::<f64>()?);
                        }
                        "right" => {
                            if f_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("right"));
                            }
                            f_right = Some(map.next_value::<f64>()?);
                        }
                        "top" => {
                            if f_top.is_some() {
                                return Err(serde::de::Error::duplicate_field("top"));
                            }
                            f_top = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CoordsGeoBounds {
                    bottom: f_bottom.ok_or_else(|| serde::de::Error::missing_field("bottom"))?,
                    left: f_left.ok_or_else(|| serde::de::Error::missing_field("left"))?,
                    right: f_right.ok_or_else(|| serde::de::Error::missing_field("right"))?,
                    top: f_top.ok_or_else(|| serde::de::Error::missing_field("top"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CoordsGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom", &self.bottom)?;
        map.serialize_entry("left", &self.left)?;
        map.serialize_entry("right", &self.right)?;
        map.serialize_entry("top", &self.top)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A date histogram interval, similar to `Duration`, with support for additional units: `w` (week), `M` (month), `q` (quarter), and `y` (year)."]
pub type DurationLarge = String;

pub type DurationValueUnitNanos = crate::types::common::UnitNanos;

pub type EpochTimeUnitMillis = crate::types::common::UnitMillis;

#[derive(Debug, Clone)]
pub struct ErrorCause {
    pub caused_by: Option<Box<crate::types::common::ErrorCause>>,
    pub header:
        Option<std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>>,
    #[doc = "A human-readable explanation of the error, in English."]
    pub reason: Option<String>,
    pub root_cause: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The server stack trace, present only if the `error_trace=true` parameter was sent with the request."]
    pub stack_trace: Option<String>,
    pub suppressed: Option<Vec<crate::types::common::ErrorCause>>,
    #[doc = "The type of error."]
    pub r#type: String,
    pub additional: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ErrorCause {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorCause;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorCause object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_caused_by: Option<Box<crate::types::common::ErrorCause>> = None;
                let mut seen_caused_by = false;
                let mut f_header: Option<
                    std::collections::BTreeMap<String, crate::types::common::StringOrStringArray>,
                > = None;
                let mut seen_header = false;
                let mut f_reason: Option<String> = None;
                let mut seen_reason = false;
                let mut f_root_cause: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_root_cause = false;
                let mut f_stack_trace: Option<String> = None;
                let mut seen_stack_trace = false;
                let mut f_suppressed: Option<Vec<crate::types::common::ErrorCause>> = None;
                let mut seen_suppressed = false;
                let mut f_type: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "caused_by" => {
                            if seen_caused_by {
                                return Err(serde::de::Error::duplicate_field("caused_by"));
                            }
                            seen_caused_by = true;
                            f_caused_by =
                                map.next_value::<Option<Box<crate::types::common::ErrorCause>>>()?;
                        }
                        "header" => {
                            if seen_header {
                                return Err(serde::de::Error::duplicate_field("header"));
                            }
                            seen_header = true;
                            f_header = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    crate::types::common::StringOrStringArray,
                                >,
                            >>()?;
                        }
                        "reason" => {
                            if seen_reason {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            seen_reason = true;
                            f_reason = map.next_value::<Option<String>>()?;
                        }
                        "root_cause" => {
                            if seen_root_cause {
                                return Err(serde::de::Error::duplicate_field("root_cause"));
                            }
                            seen_root_cause = true;
                            f_root_cause =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "stack_trace" => {
                            if seen_stack_trace {
                                return Err(serde::de::Error::duplicate_field("stack_trace"));
                            }
                            seen_stack_trace = true;
                            f_stack_trace = map.next_value::<Option<String>>()?;
                        }
                        "suppressed" => {
                            if seen_suppressed {
                                return Err(serde::de::Error::duplicate_field("suppressed"));
                            }
                            seen_suppressed = true;
                            f_suppressed =
                                map.next_value::<Option<Vec<crate::types::common::ErrorCause>>>()?;
                        }
                        "type" => {
                            if f_type.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            f_type = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorCause {
                    caused_by: f_caused_by,
                    header: f_header,
                    reason: f_reason,
                    root_cause: f_root_cause,
                    stack_trace: f_stack_trace,
                    suppressed: f_suppressed,
                    r#type: f_type.ok_or_else(|| serde::de::Error::missing_field("type"))?,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorCause {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.caused_by {
            map.serialize_entry("caused_by", v)?;
        }
        if let Some(v) = &self.header {
            map.serialize_entry("header", v)?;
        }
        if let Some(v) = &self.reason {
            map.serialize_entry("reason", v)?;
        }
        if let Some(v) = &self.root_cause {
            map.serialize_entry("root_cause", v)?;
        }
        if let Some(v) = &self.stack_trace {
            map.serialize_entry("stack_trace", v)?;
        }
        if let Some(v) = &self.suppressed {
            map.serialize_entry("suppressed", v)?;
        }
        map.serialize_entry("type", &self.r#type)?;
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The path to a field or an array of paths. Some APIs support wildcards in the path, which allows you to select multiple fields."]
pub type Field = String;

#[doc = "A field value."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FieldValue {
    Boolean(bool),
    Number(f64),
    String(String),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for FieldValue {
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
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(FieldValue::Boolean),
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(FieldValue::Number),
            Some(b'"') => serde_json::from_str(text).map(FieldValue::String),
            Some(b'n') => return Ok(FieldValue::Null),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "FieldValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "A geo-bounding box. It can be represented in the following ways:  - As 4 top/bottom/left/right coordinates.  - As 2 top_left/bottom_right points.  - As 2 top_right/bottom_left points.  - As a Well Known Text (WKT) bounding box."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GeoBounds {
    raw: Box<serde_json::value::RawValue>,
}
impl GeoBounds {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `coords` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_coords(
        &self,
    ) -> std::result::Result<crate::types::common::CoordsGeoBounds, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "coords")
    }
    #[doc = "Parses the value as the `tlbr` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_tlbr(
        &self,
    ) -> std::result::Result<
        crate::types::common::TopLeftBottomRightGeoBounds,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "tlbr")
    }
    #[doc = "Parses the value as the `trbl` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_trbl(
        &self,
    ) -> std::result::Result<
        crate::types::common::TopRightBottomLeftGeoBounds,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "trbl")
    }
    #[doc = "Parses the value as the `wkt` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_wkt(
        &self,
    ) -> std::result::Result<crate::types::common::WktGeoBounds, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "wkt")
    }
}

#[doc = "The geohash string representation of a geographical location."]
pub type GeoHash = String;

#[doc = "The geographical location specified using a geohash."]
#[derive(Debug, Clone)]
pub struct GeoHashLocation {
    pub geohash: crate::types::common::GeoHash,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoHashLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoHashLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoHashLocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_geohash: Option<crate::types::common::GeoHash> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "geohash" => {
                            if f_geohash.is_some() {
                                return Err(serde::de::Error::duplicate_field("geohash"));
                            }
                            f_geohash = Some(map.next_value::<crate::types::common::GeoHash>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoHashLocation {
                    geohash: f_geohash.ok_or_else(|| serde::de::Error::missing_field("geohash"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoHashLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("geohash", &self.geohash)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "A latitude/longitude as a two-dimensional point. It can be represented in the following ways:  - As a `{lat, long}` object.  - As a geohash value.  - As a `[lon, lat]` array.  - As a string in `<lat>, <lon>` or WKT point format."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GeoLocation {
    raw: Box<serde_json::value::RawValue>,
}
impl GeoLocation {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `latlon` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_latlon(
        &self,
    ) -> std::result::Result<crate::types::common::LatLonGeoLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "latlon")
    }
    #[doc = "Parses the value as the `geohash` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geohash(
        &self,
    ) -> std::result::Result<crate::types::common::GeoHashLocation, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "geohash")
    }
    #[doc = "Parses the value as the `coords` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_coords(&self) -> std::result::Result<Vec<f64>, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "coords")
    }
    #[doc = "Parses the value as the `text` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_text(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "text")
    }
}

#[doc = "A map tile reference, represented as `{zoom}/{x}/{y}`."]
pub type GeoTile = String;

#[doc = "The unique identifier for a resource."]
pub type Id = String;

pub type IndexName = String;

#[doc = "The inline get operation results for user-defined dictionaries."]
#[derive(Debug, Clone)]
pub struct InlineGetDictUserDefined {
    #[doc = "The primary term of the document."]
    pub primary_term: Option<i64>,
    pub routing: Option<crate::types::common::Routing>,
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[doc = "The source of the document."]
    pub source: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The fields retrieved from the document."]
    pub fields: Option<
        std::collections::BTreeMap<
            String,
            std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
        >,
    >,
    #[doc = "Whether the document was found."]
    pub found: bool,
    pub additional: std::collections::BTreeMap<
        String,
        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
    >,
}
impl<'de> serde::Deserialize<'de> for InlineGetDictUserDefined {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = InlineGetDictUserDefined;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a InlineGetDictUserDefined object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_primary_term: Option<i64> = None;
                let mut seen_primary_term = false;
                let mut f_routing: Option<crate::types::common::Routing> = None;
                let mut seen_routing = false;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut seen_seq_no = false;
                let mut f_source: Option<
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = None;
                let mut seen_source = false;
                let mut f_fields: Option<
                    std::collections::BTreeMap<
                        String,
                        std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                    >,
                > = None;
                let mut seen_fields = false;
                let mut f_found: Option<bool> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_primary_term" => {
                            if seen_primary_term {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            seen_primary_term = true;
                            f_primary_term = map.next_value::<Option<i64>>()?;
                        }
                        "_routing" => {
                            if seen_routing {
                                return Err(serde::de::Error::duplicate_field("_routing"));
                            }
                            seen_routing = true;
                            f_routing =
                                map.next_value::<Option<crate::types::common::Routing>>()?;
                        }
                        "_seq_no" => {
                            if seen_seq_no {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            seen_seq_no = true;
                            f_seq_no =
                                map.next_value::<Option<crate::types::common::SequenceNumber>>()?;
                        }
                        "_source" => {
                            if seen_source {
                                return Err(serde::de::Error::duplicate_field("_source"));
                            }
                            seen_source = true;
                            f_source = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    Box<serde_json::value::RawValue>,
                                >,
                            >>()?;
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields = map.next_value::<Option<
                                std::collections::BTreeMap<
                                    String,
                                    std::collections::BTreeMap<
                                        String,
                                        Box<serde_json::value::RawValue>,
                                    >,
                                >,
                            >>()?;
                        }
                        "found" => {
                            if f_found.is_some() {
                                return Err(serde::de::Error::duplicate_field("found"));
                            }
                            f_found = Some(map.next_value::<bool>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(InlineGetDictUserDefined {
                    primary_term: f_primary_term,
                    routing: f_routing,
                    seq_no: f_seq_no,
                    source: f_source,
                    fields: f_fields,
                    found: f_found.ok_or_else(|| serde::de::Error::missing_field("found"))?,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for InlineGetDictUserDefined {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.primary_term {
            map.serialize_entry("_primary_term", v)?;
        }
        if let Some(v) = &self.routing {
            map.serialize_entry("_routing", v)?;
        }
        if let Some(v) = &self.seq_no {
            map.serialize_entry("_seq_no", v)?;
        }
        if let Some(v) = &self.source {
            map.serialize_entry("_source", v)?;
        }
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        map.serialize_entry("found", &self.found)?;
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The geographical location specified using latitude and longitude."]
#[derive(Debug, Clone)]
pub struct LatLonGeoLocation {
    #[doc = "The latitude coordinate."]
    pub lat: f64,
    #[doc = "The longitude coordinate."]
    pub lon: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LatLonGeoLocation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LatLonGeoLocation;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LatLonGeoLocation object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lat: Option<f64> = None;
                let mut f_lon: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lat" => {
                            if f_lat.is_some() {
                                return Err(serde::de::Error::duplicate_field("lat"));
                            }
                            f_lat = Some(map.next_value::<f64>()?);
                        }
                        "lon" => {
                            if f_lon.is_some() {
                                return Err(serde::de::Error::duplicate_field("lon"));
                            }
                            f_lon = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LatLonGeoLocation {
                    lat: f_lat.ok_or_else(|| serde::de::Error::missing_field("lat"))?,
                    lon: f_lon.ok_or_else(|| serde::de::Error::missing_field("lon"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LatLonGeoLocation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("lat", &self.lat)?;
        map.serialize_entry("lon", &self.lon)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The custom metadata attached to a resource."]
pub type Metadata = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The time taken by different phases of the search."]
#[derive(Debug, Clone)]
pub struct PhaseTook {
    #[doc = "The time taken for the `can_match` phase."]
    pub can_match: i64,
    #[doc = "The time taken for the distributed frequency search `pre-query` phase."]
    pub dfs_pre_query: i64,
    #[doc = "The time taken for the distributed frequency search query phase."]
    pub dfs_query: i64,
    #[doc = "The time taken for the `expand` phase."]
    pub expand: i64,
    #[doc = "The time taken for the `fetch` phase."]
    pub fetch: i64,
    #[doc = "The time taken for the `query` phase."]
    pub query: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PhaseTook {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PhaseTook;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PhaseTook object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_can_match: Option<i64> = None;
                let mut f_dfs_pre_query: Option<i64> = None;
                let mut f_dfs_query: Option<i64> = None;
                let mut f_expand: Option<i64> = None;
                let mut f_fetch: Option<i64> = None;
                let mut f_query: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "can_match" => {
                            if f_can_match.is_some() {
                                return Err(serde::de::Error::duplicate_field("can_match"));
                            }
                            f_can_match = Some(map.next_value::<i64>()?);
                        }
                        "dfs_pre_query" => {
                            if f_dfs_pre_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("dfs_pre_query"));
                            }
                            f_dfs_pre_query = Some(map.next_value::<i64>()?);
                        }
                        "dfs_query" => {
                            if f_dfs_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("dfs_query"));
                            }
                            f_dfs_query = Some(map.next_value::<i64>()?);
                        }
                        "expand" => {
                            if f_expand.is_some() {
                                return Err(serde::de::Error::duplicate_field("expand"));
                            }
                            f_expand = Some(map.next_value::<i64>()?);
                        }
                        "fetch" => {
                            if f_fetch.is_some() {
                                return Err(serde::de::Error::duplicate_field("fetch"));
                            }
                            f_fetch = Some(map.next_value::<i64>()?);
                        }
                        "query" => {
                            if f_query.is_some() {
                                return Err(serde::de::Error::duplicate_field("query"));
                            }
                            f_query = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PhaseTook {
                    can_match: f_can_match
                        .ok_or_else(|| serde::de::Error::missing_field("can_match"))?,
                    dfs_pre_query: f_dfs_pre_query
                        .ok_or_else(|| serde::de::Error::missing_field("dfs_pre_query"))?,
                    dfs_query: f_dfs_query
                        .ok_or_else(|| serde::de::Error::missing_field("dfs_query"))?,
                    expand: f_expand.ok_or_else(|| serde::de::Error::missing_field("expand"))?,
                    fetch: f_fetch.ok_or_else(|| serde::de::Error::missing_field("fetch"))?,
                    query: f_query.ok_or_else(|| serde::de::Error::missing_field("query"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PhaseTook {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("can_match", &self.can_match)?;
        map.serialize_entry("dfs_pre_query", &self.dfs_pre_query)?;
        map.serialize_entry("dfs_query", &self.dfs_query)?;
        map.serialize_entry("expand", &self.expand)?;
        map.serialize_entry("fetch", &self.fetch)?;
        map.serialize_entry("query", &self.query)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Result {
    #[doc = "`created`"]
    Created,
    #[doc = "`deleted`"]
    Deleted,
    #[doc = "`noop`"]
    Noop,
    #[doc = "`not_found`"]
    NotFound,
    #[doc = "`updated`"]
    Updated,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Result {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Result::Created => "created",
            Result::Deleted => "deleted",
            Result::Noop => "noop",
            Result::NotFound => "not_found",
            Result::Updated => "updated",
            Result::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Result {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Result {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "created" => Result::Created,
            "deleted" => Result::Deleted,
            "noop" => Result::Noop,
            "not_found" => Result::NotFound,
            "updated" => Result::Updated,
            _ => Result::Unknown(s),
        })
    }
}

#[doc = "The routing value for the document."]
pub type Routing = String;

pub type ScrollId = String;

#[doc = "The sequence number of the document."]
pub type SequenceNumber = i64;

#[derive(Debug, Clone)]
pub struct ShardFailure {
    pub index: Option<crate::types::common::IndexName>,
    pub node: Option<String>,
    pub primary: bool,
    pub reason: crate::types::common::ErrorCause,
    pub shard: i64,
    pub status: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut seen_index = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_primary: Option<bool> = None;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_shard: Option<i64> = None;
                let mut f_status: Option<String> = None;
                let mut seen_status = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index =
                                map.next_value::<Option<crate::types::common::IndexName>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "primary" => {
                            if f_primary.is_some() {
                                return Err(serde::de::Error::duplicate_field("primary"));
                            }
                            f_primary = Some(map.next_value::<bool>()?);
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i64>()?);
                        }
                        "status" => {
                            if seen_status {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            seen_status = true;
                            f_status = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardFailure {
                    index: f_index,
                    node: f_node,
                    primary: f_primary.ok_or_else(|| serde::de::Error::missing_field("primary"))?,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    status: f_status,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("primary", &self.primary)?;
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("shard", &self.shard)?;
        if let Some(v) = &self.status {
            map.serialize_entry("status", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardInfo {
    pub failed: crate::types::common::Uint,
    pub failures: Option<Vec<crate::types::common::ShardFailure>>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardInfo {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardInfo;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardInfo object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::Uint> = None;
                let mut f_failures: Option<Vec<crate::types::common::ShardFailure>> = None;
                let mut seen_failures = false;
                let mut f_successful: Option<crate::types::common::Uint> = None;
                let mut f_total: Option<crate::types::common::Uint> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map
                                .next_value::<Option<Vec<crate::types::common::ShardFailure>>>()?;
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardInfo {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    failures: f_failures,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardInfo {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Represents a failure to search on a specific shard. Used in search responses."]
#[derive(Debug, Clone)]
pub struct ShardSearchFailure {
    #[doc = "The index name where the failure occurred."]
    pub index: Option<String>,
    #[doc = "The node id where the failure occurred."]
    pub node: Option<String>,
    pub reason: crate::types::common::ErrorCause,
    #[doc = "The shard id where the failure occurred."]
    pub shard: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardSearchFailure {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardSearchFailure;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardSearchFailure object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_index: Option<String> = None;
                let mut seen_index = false;
                let mut f_node: Option<String> = None;
                let mut seen_node = false;
                let mut f_reason: Option<crate::types::common::ErrorCause> = None;
                let mut f_shard: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "index" => {
                            if seen_index {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            seen_index = true;
                            f_index = map.next_value::<Option<String>>()?;
                        }
                        "node" => {
                            if seen_node {
                                return Err(serde::de::Error::duplicate_field("node"));
                            }
                            seen_node = true;
                            f_node = map.next_value::<Option<String>>()?;
                        }
                        "reason" => {
                            if f_reason.is_some() {
                                return Err(serde::de::Error::duplicate_field("reason"));
                            }
                            f_reason = Some(map.next_value::<crate::types::common::ErrorCause>()?);
                        }
                        "shard" => {
                            if f_shard.is_some() {
                                return Err(serde::de::Error::duplicate_field("shard"));
                            }
                            f_shard = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardSearchFailure {
                    index: f_index,
                    node: f_node,
                    reason: f_reason.ok_or_else(|| serde::de::Error::missing_field("reason"))?,
                    shard: f_shard.ok_or_else(|| serde::de::Error::missing_field("shard"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardSearchFailure {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.index {
            map.serialize_entry("index", v)?;
        }
        if let Some(v) = &self.node {
            map.serialize_entry("node", v)?;
        }
        map.serialize_entry("reason", &self.reason)?;
        map.serialize_entry("shard", &self.shard)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ShardStatistics {
    pub failed: crate::types::common::Uint,
    pub failures: Option<Vec<crate::types::common::ShardSearchFailure>>,
    pub skipped: Option<crate::types::common::Uint>,
    pub successful: crate::types::common::Uint,
    pub total: crate::types::common::Uint,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ShardStatistics {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ShardStatistics;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ShardStatistics object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_failed: Option<crate::types::common::Uint> = None;
                let mut f_failures: Option<Vec<crate::types::common::ShardSearchFailure>> = None;
                let mut seen_failures = false;
                let mut f_skipped: Option<crate::types::common::Uint> = None;
                let mut seen_skipped = false;
                let mut f_successful: Option<crate::types::common::Uint> = None;
                let mut f_total: Option<crate::types::common::Uint> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "failed" => {
                            if f_failed.is_some() {
                                return Err(serde::de::Error::duplicate_field("failed"));
                            }
                            f_failed = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "failures" => {
                            if seen_failures {
                                return Err(serde::de::Error::duplicate_field("failures"));
                            }
                            seen_failures = true;
                            f_failures = map.next_value::<Option<Vec<crate::types::common::ShardSearchFailure>>>()?;
                        }
                        "skipped" => {
                            if seen_skipped {
                                return Err(serde::de::Error::duplicate_field("skipped"));
                            }
                            seen_skipped = true;
                            f_skipped = map.next_value::<Option<crate::types::common::Uint>>()?;
                        }
                        "successful" => {
                            if f_successful.is_some() {
                                return Err(serde::de::Error::duplicate_field("successful"));
                            }
                            f_successful = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        "total" => {
                            if f_total.is_some() {
                                return Err(serde::de::Error::duplicate_field("total"));
                            }
                            f_total = Some(map.next_value::<crate::types::common::Uint>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ShardStatistics {
                    failed: f_failed.ok_or_else(|| serde::de::Error::missing_field("failed"))?,
                    failures: f_failures,
                    skipped: f_skipped,
                    successful: f_successful
                        .ok_or_else(|| serde::de::Error::missing_field("successful"))?,
                    total: f_total.ok_or_else(|| serde::de::Error::missing_field("total"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ShardStatistics {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("failed", &self.failed)?;
        if let Some(v) = &self.failures {
            map.serialize_entry("failures", v)?;
        }
        if let Some(v) = &self.skipped {
            map.serialize_entry("skipped", v)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("total", &self.total)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type SortResults = Vec<crate::types::common::FieldValue>;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringOrStringArray {
    String(String),
    Array(Vec<String>),
}
impl<'de> serde::Deserialize<'de> for StringOrStringArray {
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
            Some(b'"') => serde_json::from_str(text).map(StringOrStringArray::String),
            Some(b'[') => serde_json::from_str(text).map(StringOrStringArray::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "StringOrStringArray"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "The geographical bounds specified using upper-left and lower-right points."]
#[derive(Debug, Clone)]
pub struct TopLeftBottomRightGeoBounds {
    #[doc = "The lower-right corner coordinates."]
    pub bottom_right: crate::types::common::GeoLocation,
    #[doc = "The upper-left corner coordinates."]
    pub top_left: crate::types::common::GeoLocation,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopLeftBottomRightGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopLeftBottomRightGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopLeftBottomRightGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom_right: Option<crate::types::common::GeoLocation> = None;
                let mut f_top_left: Option<crate::types::common::GeoLocation> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom_right" => {
                            if f_bottom_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom_right"));
                            }
                            f_bottom_right =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "top_left" => {
                            if f_top_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("top_left"));
                            }
                            f_top_left =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopLeftBottomRightGeoBounds {
                    bottom_right: f_bottom_right
                        .ok_or_else(|| serde::de::Error::missing_field("bottom_right"))?,
                    top_left: f_top_left
                        .ok_or_else(|| serde::de::Error::missing_field("top_left"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopLeftBottomRightGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom_right", &self.bottom_right)?;
        map.serialize_entry("top_left", &self.top_left)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The geographical bounds specified using upper-right and lower-left points."]
#[derive(Debug, Clone)]
pub struct TopRightBottomLeftGeoBounds {
    #[doc = "The lower-left corner coordinates."]
    pub bottom_left: crate::types::common::GeoLocation,
    #[doc = "The upper-right corner coordinates."]
    pub top_right: crate::types::common::GeoLocation,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopRightBottomLeftGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopRightBottomLeftGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopRightBottomLeftGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_bottom_left: Option<crate::types::common::GeoLocation> = None;
                let mut f_top_right: Option<crate::types::common::GeoLocation> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "bottom_left" => {
                            if f_bottom_left.is_some() {
                                return Err(serde::de::Error::duplicate_field("bottom_left"));
                            }
                            f_bottom_left =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        "top_right" => {
                            if f_top_right.is_some() {
                                return Err(serde::de::Error::duplicate_field("top_right"));
                            }
                            f_top_right =
                                Some(map.next_value::<crate::types::common::GeoLocation>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopRightBottomLeftGeoBounds {
                    bottom_left: f_bottom_left
                        .ok_or_else(|| serde::de::Error::missing_field("bottom_left"))?,
                    top_right: f_top_right
                        .ok_or_else(|| serde::de::Error::missing_field("top_right"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopRightBottomLeftGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("bottom_left", &self.bottom_left)?;
        map.serialize_entry("top_right", &self.top_right)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The type of document or resource."]
pub type Type = String;

pub type Uint = i64;

#[doc = "The time unit for milliseconds."]
pub type UnitMillis = i64;

#[doc = "Time unit for nanoseconds."]
pub type UnitNanos = i64;

pub type VersionNumber = i64;

#[doc = "The absence of any type. This is commonly used in APIs that don't return a body.  Although \"void\" is generally used for a unit type that has only one value, this is interpreted as the bottom type, which has no value. Most languages have a unit type, but few have a bottom type.  See https://en.m.wikipedia.org/wiki/Unit_type and https://en.m.wikipedia.org/wiki/Bottom_type."]
pub type Void = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The geographical bounds specified using Well-Known Text (WKT) format."]
#[derive(Debug, Clone)]
pub struct WktGeoBounds {
    #[doc = "The WKT string representation of the geographical bounds."]
    pub wkt: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WktGeoBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WktGeoBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WktGeoBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_wkt: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "wkt" => {
                            if f_wkt.is_some() {
                                return Err(serde::de::Error::duplicate_field("wkt"));
                            }
                            f_wkt = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WktGeoBounds {
                    wkt: f_wkt.ok_or_else(|| serde::de::Error::missing_field("wkt"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WktGeoBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("wkt", &self.wkt)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The base response for write operations."]
#[derive(Debug, Clone)]
pub struct WriteResponseBase {
    pub id: crate::types::common::Id,
    pub index: crate::types::common::IndexName,
    #[doc = "The primary term of the document."]
    pub primary_term: i64,
    pub seq_no: crate::types::common::SequenceNumber,
    pub shards: crate::types::common::ShardStatistics,
    pub r#type: Option<crate::types::common::Type>,
    pub version: crate::types::common::VersionNumber,
    pub forced_refresh: Option<bool>,
    pub result: crate::types::common::Result,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WriteResponseBase {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WriteResponseBase;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WriteResponseBase object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_id: Option<crate::types::common::Id> = None;
                let mut f_index: Option<crate::types::common::IndexName> = None;
                let mut f_primary_term: Option<i64> = None;
                let mut f_seq_no: Option<crate::types::common::SequenceNumber> = None;
                let mut f_shards: Option<crate::types::common::ShardStatistics> = None;
                let mut f_type: Option<crate::types::common::Type> = None;
                let mut seen_type = false;
                let mut f_version: Option<crate::types::common::VersionNumber> = None;
                let mut f_forced_refresh: Option<bool> = None;
                let mut seen_forced_refresh = false;
                let mut f_result: Option<crate::types::common::Result> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "_id" => {
                            if f_id.is_some() {
                                return Err(serde::de::Error::duplicate_field("_id"));
                            }
                            f_id = Some(map.next_value::<crate::types::common::Id>()?);
                        }
                        "_index" => {
                            if f_index.is_some() {
                                return Err(serde::de::Error::duplicate_field("_index"));
                            }
                            f_index = Some(map.next_value::<crate::types::common::IndexName>()?);
                        }
                        "_primary_term" => {
                            if f_primary_term.is_some() {
                                return Err(serde::de::Error::duplicate_field("_primary_term"));
                            }
                            f_primary_term = Some(map.next_value::<i64>()?);
                        }
                        "_seq_no" => {
                            if f_seq_no.is_some() {
                                return Err(serde::de::Error::duplicate_field("_seq_no"));
                            }
                            f_seq_no =
                                Some(map.next_value::<crate::types::common::SequenceNumber>()?);
                        }
                        "_shards" => {
                            if f_shards.is_some() {
                                return Err(serde::de::Error::duplicate_field("_shards"));
                            }
                            f_shards =
                                Some(map.next_value::<crate::types::common::ShardStatistics>()?);
                        }
                        "_type" => {
                            if seen_type {
                                return Err(serde::de::Error::duplicate_field("_type"));
                            }
                            seen_type = true;
                            f_type = map.next_value::<Option<crate::types::common::Type>>()?;
                        }
                        "_version" => {
                            if f_version.is_some() {
                                return Err(serde::de::Error::duplicate_field("_version"));
                            }
                            f_version =
                                Some(map.next_value::<crate::types::common::VersionNumber>()?);
                        }
                        "forced_refresh" => {
                            if seen_forced_refresh {
                                return Err(serde::de::Error::duplicate_field("forced_refresh"));
                            }
                            seen_forced_refresh = true;
                            f_forced_refresh = map.next_value::<Option<bool>>()?;
                        }
                        "result" => {
                            if f_result.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            f_result = Some(map.next_value::<crate::types::common::Result>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WriteResponseBase {
                    id: f_id.ok_or_else(|| serde::de::Error::missing_field("_id"))?,
                    index: f_index.ok_or_else(|| serde::de::Error::missing_field("_index"))?,
                    primary_term: f_primary_term
                        .ok_or_else(|| serde::de::Error::missing_field("_primary_term"))?,
                    seq_no: f_seq_no.ok_or_else(|| serde::de::Error::missing_field("_seq_no"))?,
                    shards: f_shards.ok_or_else(|| serde::de::Error::missing_field("_shards"))?,
                    r#type: f_type,
                    version: f_version
                        .ok_or_else(|| serde::de::Error::missing_field("_version"))?,
                    forced_refresh: f_forced_refresh,
                    result: f_result.ok_or_else(|| serde::de::Error::missing_field("result"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WriteResponseBase {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("_id", &self.id)?;
        map.serialize_entry("_index", &self.index)?;
        map.serialize_entry("_primary_term", &self.primary_term)?;
        map.serialize_entry("_seq_no", &self.seq_no)?;
        map.serialize_entry("_shards", &self.shards)?;
        if let Some(v) = &self.r#type {
            map.serialize_entry("_type", v)?;
        }
        map.serialize_entry("_version", &self.version)?;
        if let Some(v) = &self.forced_refresh {
            map.serialize_entry("forced_refresh", v)?;
        }
        map.serialize_entry("result", &self.result)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
