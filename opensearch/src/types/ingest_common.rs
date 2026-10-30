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

//! Response types of the `ingest_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppendProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "If `false`, the processor does not append values already present in the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_duplicates: Option<bool>,
    pub field: crate::types::common::Field,
    #[doc = "The value to be appended. Supports template snippets."]
    pub value: Vec<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AttachmentProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and field does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "The number of chars being used for extraction to prevent huge fields. Use `-1` for no limit."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_chars: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub indexed_chars_field: Option<crate::types::common::Field>,
    #[doc = "Array of properties to select to be stored. Can be `content`, `title`, `name`, `author`, `keywords`, `date`, `content_type`, `content_length`, `language`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[doc = "Field containing the name of the resource to decode. If specified, the processor passes this resource name to the underlying Tika library to enable Resource Name Based Detection."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BytesProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CircleProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The difference between the resulting inscribed distance from center to side and the circle's radius (measured in meters for `geo_shape`, unit-less for `shape`)."]
    pub error_distance: f64,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    pub shape_type: crate::types::ingest_common::ShapeType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConvertProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
    pub r#type: crate::types::ingest_common::ConvertType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConvertType {
    #[doc = "`auto`"]
    Auto,
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "`double`"]
    Double,
    #[doc = "`float`"]
    Float,
    #[doc = "`integer`"]
    Integer,
    #[doc = "`long`"]
    Long,
    #[doc = "`string`"]
    String,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConvertType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConvertType::Auto => "auto",
            ConvertType::Boolean => "boolean",
            ConvertType::Double => "double",
            ConvertType::Float => "float",
            ConvertType::Integer => "integer",
            ConvertType::Long => "long",
            ConvertType::String => "string",
            ConvertType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConvertType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConvertType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "auto" => ConvertType::Auto,
            "boolean" => ConvertType::Boolean,
            "double" => ConvertType::Double,
            "float" => ConvertType::Float,
            "integer" => ConvertType::Integer,
            "long" => ConvertType::Long,
            "string" => ConvertType::String,
            _ => ConvertType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CsvProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "Value used to fill empty fields. Empty fields are skipped if this is not provided. An empty field is one with no value (2 consecutive separators) or empty quotes (`\"\"`)."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub empty_value: Option<Box<serde_json::value::RawValue>>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "Quote used in CSV, has to be single character string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote: Option<String>,
    #[doc = "Separator used in CSV, has to be single character string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    pub target_fields: crate::types::common::Fields,
    #[doc = "Trim white spaces in unquoted fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateIndexNameProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "An array of the expected date formats for parsing dates/timestamps in the document being preprocessed. Can be a java time pattern or one of the following formats: `ISO8601`, `UNIX`, `UNIX_MS`, or `TAI64N`."]
    pub date_formats: Vec<String>,
    #[doc = "How to round the date when formatting the date into the index name. Valid values are: `y` (year), `M` (month), `w` (week), `d` (day), `h` (hour), `m` (minute) and `s` (second). Supports template snippets."]
    pub date_rounding: String,
    pub field: crate::types::common::Field,
    #[doc = "The format to be used when printing the parsed date into the index name. A valid java time pattern is expected here. Supports template snippets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_name_format: Option<String>,
    #[doc = "A prefix of the index name to be prepended before the printed date. Supports template snippets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_name_prefix: Option<String>,
    #[doc = "The locale to use when parsing the date from the document being preprocessed, relevant when parsing month names or week days."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[doc = "The time zone to use when parsing the date and when date math index supports resolves expressions into concrete index names."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "An array of the expected date formats. Can be a java time pattern or one of the following formats: ISO8601, UNIX, UNIX_MS, or TAI64N."]
    pub formats: Vec<String>,
    #[doc = "The locale to use when parsing the date, relevant when parsing month names or week days. Supports template snippets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "The time zone to use when parsing the date. Supports template snippets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DissectProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The character(s) that separate the appended fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub append_separator: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "The pattern to apply to the field."]
    pub pattern: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DotExpanderProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "The field that contains the field to expand. Only required if the field to expand is part another object field, because the `field` option can only understand leaf fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DropProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FailProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The error message thrown by the processor. Supports template snippets."]
    pub message: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ForeachProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true`, the processor silently exits without changing the document if the `field` is `null` or missing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    pub processor: Box<crate::types::ingest_common::ProcessorContainer>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoIpProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "The database filename referring to a database the module ships with (`GeoLite2-City.mmdb`, `GeoLite2-Country.mmdb`, or `GeoLite2-ASN.mmdb`) or a custom database in the `ingest-geoip` config directory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub database_file: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true`, only the first found geoip data will be returned, even if the field contains an array."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_only: Option<bool>,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "Controls what properties are added to the `target_field` based on the geoip lookup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GrokProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "A map of pattern-name and pattern tuples defining custom patterns to be used by the current processor. Patterns matching existing names will override the pre-existing definition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern_definitions: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "An ordered list of grok expression to match and extract named captures with. Returns on the first expression in the list that matches."]
    pub patterns: Vec<String>,
    #[doc = "When `true`, `_ingest._grok_match_index` will be inserted into your matched document's metadata with the index into the pattern found in `patterns` that matched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_match: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GsubProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "The pattern to be replaced."]
    pub pattern: String,
    #[doc = "The string to replace the matching patterns with."]
    pub replacement: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JoinProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "The separator character."]
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JsonProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "Flag that forces the parsed JSON to be added at the top level of the document. `target_field` must not be set when this option is chosen."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_to_root: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_to_root_conflict_strategy:
        Option<crate::types::ingest_common::JsonProcessorConflictStrategy>,
    #[doc = "When set to `true`, the JSON parser will not fail if the JSON contains duplicate keys. Instead, the last encountered value for any duplicate key wins."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allow_duplicate_keys: Option<bool>,
    pub field: crate::types::common::Field,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JsonProcessorConflictStrategy {
    #[doc = "`merge`"]
    Merge,
    #[doc = "`replace`"]
    Replace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl JsonProcessorConflictStrategy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            JsonProcessorConflictStrategy::Merge => "merge",
            JsonProcessorConflictStrategy::Replace => "replace",
            JsonProcessorConflictStrategy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for JsonProcessorConflictStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for JsonProcessorConflictStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "merge" => JsonProcessorConflictStrategy::Merge,
            "replace" => JsonProcessorConflictStrategy::Replace,
            _ => JsonProcessorConflictStrategy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeyValueProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "List of keys to exclude from document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_keys: Option<Vec<String>>,
    pub field: crate::types::common::Field,
    #[doc = "Regex pattern to use for splitting key-value pairs."]
    pub field_split: String,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "List of keys to filter and insert into document. Defaults to including all keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_keys: Option<Vec<String>>,
    #[doc = "Prefix to be added to extracted keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[doc = "If `true`. strip brackets `()`, `<>`, `[]` as well as quotes `'` and `\"` from extracted values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strip_brackets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
    #[doc = "String of characters to trim from extracted keys."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim_key: Option<String>,
    #[doc = "String of characters to trim from extracted values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim_value: Option<String>,
    #[doc = "Regex pattern to use for splitting the key from the value within a key-value pair."]
    pub value_split: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pipeline {
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "Description of the ingest pipeline."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Processors to run immediately after a processor failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Processors used to perform transformations on documents before indexing. Processors run sequentially in the order specified."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub processors: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PipelineProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[doc = "Whether to ignore missing pipelines instead of failing."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing_pipeline: Option<bool>,
    pub name: crate::types::common::Name,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessorContainer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub append: Option<crate::types::ingest_common::AppendProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment: Option<crate::types::ingest_common::AttachmentProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bytes: Option<crate::types::ingest_common::BytesProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub circle: Option<crate::types::ingest_common::CircleProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert: Option<crate::types::ingest_common::ConvertProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub csv: Option<crate::types::ingest_common::CsvProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::types::ingest_common::DateProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_index_name: Option<crate::types::ingest_common::DateIndexNameProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dissect: Option<crate::types::ingest_common::DissectProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dot_expander: Option<crate::types::ingest_common::DotExpanderProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub drop: Option<crate::types::ingest_common::DropProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fail: Option<crate::types::ingest_common::FailProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub foreach: Option<Box<crate::types::ingest_common::ForeachProcessor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geoip: Option<crate::types::ingest_common::GeoIpProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grok: Option<crate::types::ingest_common::GrokProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gsub: Option<crate::types::ingest_common::GsubProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<crate::types::ingest_common::JoinProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub json: Option<crate::types::ingest_common::JsonProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kv: Option<crate::types::ingest_common::KeyValueProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<crate::types::ingest_common::LowercaseProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<crate::types::ingest_common::PipelineProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<crate::types::ingest_common::RemoveProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename: Option<crate::types::ingest_common::RenameProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<crate::types::common::Script>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<crate::types::ingest_common::SetProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set_security_user: Option<crate::types::ingest_common::SetSecurityUserProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: Option<crate::types::ingest_common::SortProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split: Option<crate::types::ingest_common::SplitProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text_embedding: Option<crate::types::ingest_common::TextEmbeddingProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trim: Option<crate::types::ingest_common::TrimProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uppercase: Option<crate::types::ingest_common::UppercaseProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub urldecode: Option<crate::types::ingest_common::UrlDecodeProcessor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<crate::types::ingest_common::UserAgentProcessor>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Fields,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenameProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    pub target_field: crate::types::common::Field,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_from: Option<crate::types::common::Field>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `value` is a template snippet that evaluates to `null` or the empty string, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_empty_value: Option<bool>,
    #[doc = "The media type for encoding `value`. Applies only when value is a template snippet. Must be one of `application/json`, `text/plain`, or `application/x-www-form-urlencoded`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[doc = "If `true` processor will update fields with pre-existing non-null-valued field. When set to `false`, such fields will not be touched."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#override: Option<bool>,
    #[doc = "The value to be set for the field. Supports template snippets. May specify only one of `value` or `copy_from`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SetSecurityUserProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "Controls what user related properties are added to the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShapeType {
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShapeType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShapeType::GeoShape => "geo_shape",
            ShapeType::XyShape => "xy_shape",
            ShapeType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShapeType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShapeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_shape" => ShapeType::GeoShape,
            "xy_shape" => ShapeType::XyShape,
            _ => ShapeType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SortProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<crate::types::common::SortOrder>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[doc = "Preserves empty trailing fields, if any."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_trailing: Option<bool>,
    #[doc = "A regex which matches the separator, for example, `,` or `\\s+`."]
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextEmbeddingProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[doc = "Contains key-value pairs that specify the mapping of a text field to a vector field."]
    pub field_map: std::collections::BTreeMap<String, String>,
    pub model_id: crate::types::common::Id,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrimProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UppercaseProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UrlDecodeProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist or is `null`, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAgentProcessor {
    #[doc = "Description of the processor. Useful for describing the purpose of the processor or its configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Conditionally execute the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#if: Option<String>,
    #[doc = "Ignore failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[doc = "Handle failures for the processor."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_failure: Option<Vec<crate::types::ingest_common::ProcessorContainer>>,
    #[doc = "Identifier for the processor. Useful for debugging and metrics."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub field: crate::types::common::Field,
    #[doc = "If `true` and `field` does not exist, the processor quietly exits without modifying the document."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_missing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<crate::types::ingest_common::UserAgentProperty>>,
    #[doc = "The name of the file in the `config/ingest-user-agent` directory containing the regular expressions for parsing the user agent string. Both the directory and the file have to be created before starting OpenSearch. If not specified, ingest-user-agent will use the `regexes.yaml` from uap-core it ships with."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regex_file: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<crate::types::common::Field>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserAgentProperty {
    #[doc = "`BUILD`"]
    Build,
    #[doc = "`DEVICE`"]
    Device,
    #[doc = "`MAJOR`"]
    Major,
    #[doc = "`MINOR`"]
    Minor,
    #[doc = "`NAME`"]
    Name,
    #[doc = "`OS`"]
    Os,
    #[doc = "`OS_MAJOR`"]
    OsMajor,
    #[doc = "`OS_MINOR`"]
    OsMinor,
    #[doc = "`OS_NAME`"]
    OsName,
    #[doc = "`PATCH`"]
    Patch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UserAgentProperty {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UserAgentProperty::Build => "BUILD",
            UserAgentProperty::Device => "DEVICE",
            UserAgentProperty::Major => "MAJOR",
            UserAgentProperty::Minor => "MINOR",
            UserAgentProperty::Name => "NAME",
            UserAgentProperty::Os => "OS",
            UserAgentProperty::OsMajor => "OS_MAJOR",
            UserAgentProperty::OsMinor => "OS_MINOR",
            UserAgentProperty::OsName => "OS_NAME",
            UserAgentProperty::Patch => "PATCH",
            UserAgentProperty::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UserAgentProperty {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UserAgentProperty {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "BUILD" => UserAgentProperty::Build,
            "DEVICE" => UserAgentProperty::Device,
            "MAJOR" => UserAgentProperty::Major,
            "MINOR" => UserAgentProperty::Minor,
            "NAME" => UserAgentProperty::Name,
            "OS" => UserAgentProperty::Os,
            "OS_MAJOR" => UserAgentProperty::OsMajor,
            "OS_MINOR" => UserAgentProperty::OsMinor,
            "OS_NAME" => UserAgentProperty::OsName,
            "PATCH" => UserAgentProperty::Patch,
            _ => UserAgentProperty::Unknown(s),
        })
    }
}
