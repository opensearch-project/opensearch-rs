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

//! Response types of the `common_mapping` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AggregateMetricDoubleProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub default_metric: String,
    pub metrics: Vec<String>,
    pub r#type: crate::types::common_mapping::AggregateMetricDoublePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AggregateMetricDoublePropertyType {
    #[doc = "`aggregate_metric_double`"]
    AggregateMetricDouble,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AggregateMetricDoublePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AggregateMetricDoublePropertyType::AggregateMetricDouble => "aggregate_metric_double",
            AggregateMetricDoublePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AggregateMetricDoublePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AggregateMetricDoublePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aggregate_metric_double" => AggregateMetricDoublePropertyType::AggregateMetricDouble,
            _ => AggregateMetricDoublePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllField {
    pub analyzer: String,
    pub enabled: bool,
    pub omit_norms: bool,
    pub search_analyzer: String,
    pub similarity: String,
    pub store: bool,
    pub store_term_vector_offsets: bool,
    pub store_term_vector_payloads: bool,
    pub store_term_vector_positions: bool,
    pub store_term_vectors: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BinaryProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::BinaryPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryPropertyType {
    #[doc = "`binary`"]
    Binary,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BinaryPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BinaryPropertyType::Binary => "binary",
            BinaryPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BinaryPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BinaryPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "binary" => BinaryPropertyType::Binary,
            _ => BinaryPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BooleanProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<bool>,
    pub r#type: crate::types::common_mapping::BooleanPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BooleanPropertyType {
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl BooleanPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            BooleanPropertyType::Boolean => "boolean",
            BooleanPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for BooleanPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for BooleanPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "boolean" => BooleanPropertyType::Boolean,
            _ => BooleanPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ByteNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Byte>,
    pub r#type: crate::types::common_mapping::ByteNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ByteNumberPropertyType {
    #[doc = "`byte`"]
    Byte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ByteNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ByteNumberPropertyType::Byte => "byte",
            ByteNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ByteNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ByteNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "byte" => ByteNumberPropertyType::Byte,
            _ => ByteNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompletionProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<crate::types::common_mapping::SuggestContext>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_input_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_position_increments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_separators: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    pub r#type: crate::types::common_mapping::CompletionPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CompletionPropertyType {
    #[doc = "`completion`"]
    Completion,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CompletionPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CompletionPropertyType::Completion => "completion",
            CompletionPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CompletionPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CompletionPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "completion" => CompletionPropertyType::Completion,
            _ => CompletionPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConstantKeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::ConstantKeywordPropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Box<serde_json::value::RawValue>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstantKeywordPropertyType {
    #[doc = "`constant_keyword`"]
    ConstantKeyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConstantKeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConstantKeywordPropertyType::ConstantKeyword => "constant_keyword",
            ConstantKeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConstantKeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConstantKeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "constant_keyword" => ConstantKeywordPropertyType::ConstantKeyword,
            _ => ConstantKeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DataStreamTimestamp {
    pub enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateNanosProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision_step: Option<i64>,
    pub r#type: crate::types::common_mapping::DateNanosPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DateNanosPropertyType {
    #[doc = "`date_nanos`"]
    DateNanos,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DateNanosPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DateNanosPropertyType::DateNanos => "date_nanos",
            DateNanosPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DateNanosPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DateNanosPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date_nanos" => DateNanosPropertyType::DateNanos,
            _ => DateNanosPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<crate::types::indices_common::NumericFielddata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::DateTime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision_step: Option<i64>,
    pub r#type: crate::types::common_mapping::DatePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatePropertyType {
    #[doc = "`date`"]
    Date,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DatePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DatePropertyType::Date => "date",
            DatePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DatePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DatePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date" => DatePropertyType::Date,
            _ => DatePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    pub r#type: crate::types::common_mapping::DateRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DateRangePropertyType {
    #[doc = "`date_range`"]
    DateRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DateRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DateRangePropertyType::DateRange => "date_range",
            DateRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DateRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DateRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date_range" => DateRangePropertyType::DateRange,
            _ => DateRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoubleNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::DoubleNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DoubleNumberPropertyType {
    #[doc = "`double`"]
    Double,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DoubleNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DoubleNumberPropertyType::Double => "double",
            DoubleNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DoubleNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DoubleNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "double" => DoubleNumberPropertyType::Double,
            _ => DoubleNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoubleRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::DoubleRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DoubleRangePropertyType {
    #[doc = "`double_range`"]
    DoubleRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DoubleRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DoubleRangePropertyType::DoubleRange => "double_range",
            DoubleRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DoubleRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DoubleRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "double_range" => DoubleRangePropertyType::DoubleRange,
            _ => DoubleRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum DynamicMapping {
    String(crate::types::common_mapping::DynamicMappingString),
    Boolean(bool),
}
impl<'de> serde::Deserialize<'de> for DynamicMapping {
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
            Some(b'"') => serde_json::from_str(text).map(DynamicMapping::String),
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(DynamicMapping::Boolean),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "DynamicMapping"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DynamicMappingString {
    #[doc = "`false`"]
    False,
    #[doc = "`strict`"]
    Strict,
    #[doc = "`strict_allow_templates`"]
    StrictAllowTemplates,
    #[doc = "`true`"]
    True,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DynamicMappingString {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DynamicMappingString::False => "false",
            DynamicMappingString::Strict => "strict",
            DynamicMappingString::StrictAllowTemplates => "strict_allow_templates",
            DynamicMappingString::True => "true",
            DynamicMappingString::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DynamicMappingString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DynamicMappingString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "false" => DynamicMappingString::False,
            "strict" => DynamicMappingString::Strict,
            "strict_allow_templates" => DynamicMappingString::StrictAllowTemplates,
            "true" => DynamicMappingString::True,
            _ => DynamicMappingString::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping: Option<crate::types::common_mapping::Property>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#match: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_mapping_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub match_pattern: Option<crate::types::common_mapping::MatchType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_match: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_unmatch: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unmatch: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldAliasProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<crate::types::common::Field>,
    pub r#type: crate::types::common_mapping::FieldAliasPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldAliasPropertyType {
    #[doc = "`alias`"]
    Alias,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldAliasPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldAliasPropertyType::Alias => "alias",
            FieldAliasPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldAliasPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldAliasPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "alias" => FieldAliasPropertyType::Alias,
            _ => FieldAliasPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldMapping {
    pub full_name: String,
    pub mapping: std::collections::BTreeMap<String, crate::types::common_mapping::Property>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldNamesField {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldType {
    #[doc = "`aggregate_metric_double`"]
    AggregateMetricDouble,
    #[doc = "`alias`"]
    Alias,
    #[doc = "`binary`"]
    Binary,
    #[doc = "`boolean`"]
    Boolean,
    #[doc = "`byte`"]
    Byte,
    #[doc = "`completion`"]
    Completion,
    #[doc = "`constant_keyword`"]
    ConstantKeyword,
    #[doc = "`date`"]
    Date,
    #[doc = "`date_nanos`"]
    DateNanos,
    #[doc = "`date_range`"]
    DateRange,
    #[doc = "`double`"]
    Double,
    #[doc = "`double_range`"]
    DoubleRange,
    #[doc = "`flat_object`"]
    FlatObject,
    #[doc = "`float`"]
    Float,
    #[doc = "`float_range`"]
    FloatRange,
    #[doc = "`geo_point`"]
    GeoPoint,
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "`half_float`"]
    HalfFloat,
    #[doc = "`histogram`"]
    Histogram,
    #[doc = "`icu_collation_keyword`"]
    IcuCollationKeyword,
    #[doc = "`integer`"]
    Integer,
    #[doc = "`integer_range`"]
    IntegerRange,
    #[doc = "`ip`"]
    Ip,
    #[doc = "`ip_range`"]
    IpRange,
    #[doc = "`join`"]
    Join,
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "`knn_vector`"]
    KnnVector,
    #[doc = "`long`"]
    Long,
    #[doc = "`long_range`"]
    LongRange,
    #[doc = "`match_only_text`"]
    MatchOnlyText,
    #[doc = "`murmur3`"]
    Murmur3,
    #[doc = "`nested`"]
    Nested,
    #[doc = "`object`"]
    Object,
    #[doc = "`percolator`"]
    Percolator,
    #[doc = "`rank_feature`"]
    RankFeature,
    #[doc = "`rank_features`"]
    RankFeatures,
    #[doc = "`scaled_float`"]
    ScaledFloat,
    #[doc = "`search_as_you_type`"]
    SearchAsYouType,
    #[doc = "`semantic`"]
    Semantic,
    #[doc = "`short`"]
    Short,
    #[doc = "`text`"]
    Text,
    #[doc = "`token_count`"]
    TokenCount,
    #[doc = "`unsigned_long`"]
    UnsignedLong,
    #[doc = "`version`"]
    Version,
    #[doc = "`wildcard`"]
    Wildcard,
    #[doc = "`xy_point`"]
    XyPoint,
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldType::AggregateMetricDouble => "aggregate_metric_double",
            FieldType::Alias => "alias",
            FieldType::Binary => "binary",
            FieldType::Boolean => "boolean",
            FieldType::Byte => "byte",
            FieldType::Completion => "completion",
            FieldType::ConstantKeyword => "constant_keyword",
            FieldType::Date => "date",
            FieldType::DateNanos => "date_nanos",
            FieldType::DateRange => "date_range",
            FieldType::Double => "double",
            FieldType::DoubleRange => "double_range",
            FieldType::FlatObject => "flat_object",
            FieldType::Float => "float",
            FieldType::FloatRange => "float_range",
            FieldType::GeoPoint => "geo_point",
            FieldType::GeoShape => "geo_shape",
            FieldType::HalfFloat => "half_float",
            FieldType::Histogram => "histogram",
            FieldType::IcuCollationKeyword => "icu_collation_keyword",
            FieldType::Integer => "integer",
            FieldType::IntegerRange => "integer_range",
            FieldType::Ip => "ip",
            FieldType::IpRange => "ip_range",
            FieldType::Join => "join",
            FieldType::Keyword => "keyword",
            FieldType::KnnVector => "knn_vector",
            FieldType::Long => "long",
            FieldType::LongRange => "long_range",
            FieldType::MatchOnlyText => "match_only_text",
            FieldType::Murmur3 => "murmur3",
            FieldType::Nested => "nested",
            FieldType::Object => "object",
            FieldType::Percolator => "percolator",
            FieldType::RankFeature => "rank_feature",
            FieldType::RankFeatures => "rank_features",
            FieldType::ScaledFloat => "scaled_float",
            FieldType::SearchAsYouType => "search_as_you_type",
            FieldType::Semantic => "semantic",
            FieldType::Short => "short",
            FieldType::Text => "text",
            FieldType::TokenCount => "token_count",
            FieldType::UnsignedLong => "unsigned_long",
            FieldType::Version => "version",
            FieldType::Wildcard => "wildcard",
            FieldType::XyPoint => "xy_point",
            FieldType::XyShape => "xy_shape",
            FieldType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "aggregate_metric_double" => FieldType::AggregateMetricDouble,
            "alias" => FieldType::Alias,
            "binary" => FieldType::Binary,
            "boolean" => FieldType::Boolean,
            "byte" => FieldType::Byte,
            "completion" => FieldType::Completion,
            "constant_keyword" => FieldType::ConstantKeyword,
            "date" => FieldType::Date,
            "date_nanos" => FieldType::DateNanos,
            "date_range" => FieldType::DateRange,
            "double" => FieldType::Double,
            "double_range" => FieldType::DoubleRange,
            "flat_object" => FieldType::FlatObject,
            "float" => FieldType::Float,
            "float_range" => FieldType::FloatRange,
            "geo_point" => FieldType::GeoPoint,
            "geo_shape" => FieldType::GeoShape,
            "half_float" => FieldType::HalfFloat,
            "histogram" => FieldType::Histogram,
            "icu_collation_keyword" => FieldType::IcuCollationKeyword,
            "integer" => FieldType::Integer,
            "integer_range" => FieldType::IntegerRange,
            "ip" => FieldType::Ip,
            "ip_range" => FieldType::IpRange,
            "join" => FieldType::Join,
            "keyword" => FieldType::Keyword,
            "knn_vector" => FieldType::KnnVector,
            "long" => FieldType::Long,
            "long_range" => FieldType::LongRange,
            "match_only_text" => FieldType::MatchOnlyText,
            "murmur3" => FieldType::Murmur3,
            "nested" => FieldType::Nested,
            "object" => FieldType::Object,
            "percolator" => FieldType::Percolator,
            "rank_feature" => FieldType::RankFeature,
            "rank_features" => FieldType::RankFeatures,
            "scaled_float" => FieldType::ScaledFloat,
            "search_as_you_type" => FieldType::SearchAsYouType,
            "semantic" => FieldType::Semantic,
            "short" => FieldType::Short,
            "text" => FieldType::Text,
            "token_count" => FieldType::TokenCount,
            "unsigned_long" => FieldType::UnsignedLong,
            "version" => FieldType::Version,
            "wildcard" => FieldType::Wildcard,
            "xy_point" => FieldType::XyPoint,
            "xy_shape" => FieldType::XyShape,
            _ => FieldType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlatObjectProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aggregatable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub searchable: Option<bool>,
    pub r#type: crate::types::common_mapping::FlatObjectPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlatObjectPropertyType {
    #[doc = "`flat_object`"]
    FlatObject,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FlatObjectPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FlatObjectPropertyType::FlatObject => "flat_object",
            FlatObjectPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FlatObjectPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FlatObjectPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "flat_object" => FlatObjectPropertyType::FlatObject,
            _ => FlatObjectPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::FloatNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatNumberPropertyType {
    #[doc = "`float`"]
    Float,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FloatNumberPropertyType::Float => "float",
            FloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float" => FloatNumberPropertyType::Float,
            _ => FloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FloatRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::FloatRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FloatRangePropertyType {
    #[doc = "`float_range`"]
    FloatRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FloatRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FloatRangePropertyType::FloatRange => "float_range",
            FloatRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FloatRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FloatRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float_range" => FloatRangePropertyType::FloatRange,
            _ => FloatRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoOrientation {
    #[doc = "`left` (also accepted: `LEFT`)"]
    Left,
    #[doc = "`clockwise`"]
    Clockwise,
    #[doc = "`cw`"]
    Cw,
    #[doc = "`right` (also accepted: `RIGHT`)"]
    Right,
    #[doc = "`counterclockwise`"]
    Counterclockwise,
    #[doc = "`ccw`"]
    Ccw,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoOrientation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoOrientation::Left => "left",
            GeoOrientation::Clockwise => "clockwise",
            GeoOrientation::Cw => "cw",
            GeoOrientation::Right => "right",
            GeoOrientation::Counterclockwise => "counterclockwise",
            GeoOrientation::Ccw => "ccw",
            GeoOrientation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoOrientation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoOrientation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "left" | "LEFT" => GeoOrientation::Left,
            "clockwise" => GeoOrientation::Clockwise,
            "cw" => GeoOrientation::Cw,
            "right" | "RIGHT" => GeoOrientation::Right,
            "counterclockwise" => GeoOrientation::Counterclockwise,
            "ccw" => GeoOrientation::Ccw,
            _ => GeoOrientation::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoPointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::GeoLocation>,
    pub r#type: crate::types::common_mapping::GeoPointPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoPointPropertyType {
    #[doc = "`geo_point`"]
    GeoPoint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoPointPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoPointPropertyType::GeoPoint => "geo_point",
            GeoPointPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoPointPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoPointPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_point" => GeoPointPropertyType::GeoPoint,
            _ => GeoPointPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoShapeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub distance_error_pct: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strategy: Option<crate::types::common_mapping::GeoStrategy>,
    pub r#type: crate::types::common_mapping::GeoShapePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoShapePropertyType {
    #[doc = "`geo_shape`"]
    GeoShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoShapePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoShapePropertyType::GeoShape => "geo_shape",
            GeoShapePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoShapePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoShapePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "geo_shape" => GeoShapePropertyType::GeoShape,
            _ => GeoShapePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GeoStrategy {
    #[doc = "`recursive`"]
    Recursive,
    #[doc = "`term`"]
    Term,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl GeoStrategy {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            GeoStrategy::Recursive => "recursive",
            GeoStrategy::Term => "term",
            GeoStrategy::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for GeoStrategy {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GeoStrategy {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "recursive" => GeoStrategy::Recursive,
            "term" => GeoStrategy::Term,
            _ => GeoStrategy::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HalfFloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::HalfFloatNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HalfFloatNumberPropertyType {
    #[doc = "`half_float`"]
    HalfFloat,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HalfFloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HalfFloatNumberPropertyType::HalfFloat => "half_float",
            HalfFloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HalfFloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HalfFloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "half_float" => HalfFloatNumberPropertyType::HalfFloat,
            _ => HalfFloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    pub r#type: crate::types::common_mapping::HistogramPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HistogramPropertyType {
    #[doc = "`histogram`"]
    Histogram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HistogramPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HistogramPropertyType::Histogram => "histogram",
            HistogramPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HistogramPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HistogramPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "histogram" => HistogramPropertyType::Histogram,
            _ => HistogramPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuCollationKeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_level: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiragana_quaternary_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_mapping::IcuCollationKeywordPropertyType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_top: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationKeywordPropertyType {
    #[doc = "`icu_collation_keyword`"]
    IcuCollationKeyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationKeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationKeywordPropertyType::IcuCollationKeyword => "icu_collation_keyword",
            IcuCollationKeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationKeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationKeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_collation_keyword" => IcuCollationKeywordPropertyType::IcuCollationKeyword,
            _ => IcuCollationKeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexField {
    pub enabled: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IndexOptions {
    #[doc = "`docs`"]
    Docs,
    #[doc = "`freqs`"]
    Freqs,
    #[doc = "`offsets`"]
    Offsets,
    #[doc = "`positions`"]
    Positions,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IndexOptions {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IndexOptions::Docs => "docs",
            IndexOptions::Freqs => "freqs",
            IndexOptions::Offsets => "offsets",
            IndexOptions::Positions => "positions",
            IndexOptions::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IndexOptions {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IndexOptions {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "docs" => IndexOptions::Docs,
            "freqs" => IndexOptions::Freqs,
            "offsets" => IndexOptions::Offsets,
            "positions" => IndexOptions::Positions,
            _ => IndexOptions::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegerNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<i64>,
    pub r#type: crate::types::common_mapping::IntegerNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegerNumberPropertyType {
    #[doc = "`integer`"]
    Integer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntegerNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntegerNumberPropertyType::Integer => "integer",
            IntegerNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntegerNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntegerNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "integer" => IntegerNumberPropertyType::Integer,
            _ => IntegerNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegerRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IntegerRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IntegerRangePropertyType {
    #[doc = "`integer_range`"]
    IntegerRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IntegerRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IntegerRangePropertyType::IntegerRange => "integer_range",
            IntegerRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IntegerRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IntegerRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "integer_range" => IntegerRangePropertyType::IntegerRange,
            _ => IntegerRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::IpPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpPropertyType {
    #[doc = "`ip`"]
    Ip,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IpPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IpPropertyType::Ip => "ip",
            IpPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IpPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ip" => IpPropertyType::Ip,
            _ => IpPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::IpRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IpRangePropertyType {
    #[doc = "`ip_range`"]
    IpRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IpRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IpRangePropertyType::IpRange => "ip_range",
            IpRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IpRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IpRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ip_range" => IpRangePropertyType::IpRange,
            _ => IpRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JoinProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub relations: Option<
        std::collections::BTreeMap<
            String,
            crate::types::common_mapping::JoinPropertyRelationsValue,
        >,
    >,
    pub r#type: crate::types::common_mapping::JoinPropertyType,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum JoinPropertyRelationsValue {
    RelationName(crate::types::common::RelationName),
    Array(Vec<crate::types::common::RelationName>),
}
impl<'de> serde::Deserialize<'de> for JoinPropertyRelationsValue {
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
            Some(b'"') => serde_json::from_str(text).map(JoinPropertyRelationsValue::RelationName),
            Some(b'[') => serde_json::from_str(text).map(JoinPropertyRelationsValue::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "JoinPropertyRelationsValue"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum JoinPropertyType {
    #[doc = "`join`"]
    Join,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl JoinPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            JoinPropertyType::Join => "join",
            JoinPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for JoinPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for JoinPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "join" => JoinPropertyType::Join,
            _ => JoinPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_queries_on_whitespace: Option<bool>,
    pub r#type: crate::types::common_mapping::KeywordPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordPropertyType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordPropertyType::Keyword => "keyword",
            KeywordPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordPropertyType::Keyword,
            _ => KeywordPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnVectorMethod {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub engine: Option<String>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KnnVectorProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    pub dimension: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_type: Option<String>,
    pub r#type: crate::types::common_mapping::KnnVectorPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KnnVectorPropertyType {
    #[doc = "`knn_vector`"]
    KnnVector,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KnnVectorPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KnnVectorPropertyType::KnnVector => "knn_vector",
            KnnVectorPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KnnVectorPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KnnVectorPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "knn_vector" => KnnVectorPropertyType::KnnVector,
            _ => KnnVectorPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<i64>,
    pub r#type: crate::types::common_mapping::LongNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LongNumberPropertyType {
    #[doc = "`long`"]
    Long,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LongNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LongNumberPropertyType::Long => "long",
            LongNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LongNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LongNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "long" => LongNumberPropertyType::Long,
            _ => LongNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongRangeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    pub r#type: crate::types::common_mapping::LongRangePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LongRangePropertyType {
    #[doc = "`long_range`"]
    LongRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LongRangePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LongRangePropertyType::LongRange => "long_range",
            LongRangePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LongRangePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LongRangePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "long_range" => LongRangePropertyType::LongRange,
            _ => LongRangePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatchOnlyTextProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[doc = "Multi-fields allow the same string value to be indexed in multiple ways for different purposes, such as one field for search and a multi-field for sorting and aggregations, or the same string value analyzed by different analyzers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    pub r#type: crate::types::common_mapping::MatchOnlyTextPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchOnlyTextPropertyType {
    #[doc = "`match_only_text`"]
    MatchOnlyText,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MatchOnlyTextPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MatchOnlyTextPropertyType::MatchOnlyText => "match_only_text",
            MatchOnlyTextPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MatchOnlyTextPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MatchOnlyTextPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "match_only_text" => MatchOnlyTextPropertyType::MatchOnlyText,
            _ => MatchOnlyTextPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MatchType {
    #[doc = "`regex`"]
    Regex,
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MatchType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MatchType::Regex => "regex",
            MatchType::Simple => "simple",
            MatchType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MatchType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MatchType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "regex" => MatchType::Regex,
            "simple" => MatchType::Simple,
            _ => MatchType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Murmur3HashProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::Murmur3HashPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Murmur3HashPropertyType {
    #[doc = "`murmur3`"]
    Murmur3,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Murmur3HashPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Murmur3HashPropertyType::Murmur3 => "murmur3",
            Murmur3HashPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Murmur3HashPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Murmur3HashPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "murmur3" => Murmur3HashPropertyType::Murmur3,
            _ => Murmur3HashPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_in_parent: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_in_root: Option<bool>,
    pub r#type: crate::types::common_mapping::NestedPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NestedPropertyType {
    #[doc = "`nested`"]
    Nested,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NestedPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NestedPropertyType::Nested => "nested",
            NestedPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NestedPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NestedPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nested" => NestedPropertyType::Nested,
            _ => NestedPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_mapping::ObjectPropertyType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectPropertyType {
    #[doc = "`object`"]
    Object,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ObjectPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ObjectPropertyType::Object => "object",
            ObjectPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ObjectPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ObjectPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "object" => ObjectPropertyType::Object,
            _ => ObjectPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PercolatorProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::PercolatorPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PercolatorPropertyType {
    #[doc = "`percolator`"]
    Percolator,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PercolatorPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PercolatorPropertyType::Percolator => "percolator",
            PercolatorPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PercolatorPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PercolatorPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "percolator" => PercolatorPropertyType::Percolator,
            _ => PercolatorPropertyType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Property {
    raw: Box<serde_json::value::RawValue>,
}
impl Property {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `binary_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_binary_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::BinaryProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "binary_property")
    }
    #[doc = "Parses the value as the `boolean_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_boolean_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::BooleanProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "boolean_property")
    }
    #[doc = "Parses the value as the `join_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_join_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::JoinProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "join_property")
    }
    #[doc = "Parses the value as the `keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::KeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_property")
    }
    #[doc = "Parses the value as the `match_only_text_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_match_only_text_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::MatchOnlyTextProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "match_only_text_property")
    }
    #[doc = "Parses the value as the `percolator_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_percolator_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::PercolatorProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "percolator_property")
    }
    #[doc = "Parses the value as the `rank_feature_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rank_feature_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::RankFeatureProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rank_feature_property")
    }
    #[doc = "Parses the value as the `rank_features_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rank_features_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::RankFeaturesProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rank_features_property")
    }
    #[doc = "Parses the value as the `search_as_you_type_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_search_as_you_type_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::SearchAsYouTypeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "search_as_you_type_property")
    }
    #[doc = "Parses the value as the `text_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_text_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::TextProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "text_property")
    }
    #[doc = "Parses the value as the `version_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_version_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::VersionProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "version_property")
    }
    #[doc = "Parses the value as the `wildcard_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_wildcard_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::WildcardProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "wildcard_property")
    }
    #[doc = "Parses the value as the `date_nanos_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_nanos_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateNanosProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_nanos_property")
    }
    #[doc = "Parses the value as the `date_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_property")
    }
    #[doc = "Parses the value as the `aggregate_metric_double_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_aggregate_metric_double_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::AggregateMetricDoubleProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "aggregate_metric_double_property")
    }
    #[doc = "Parses the value as the `flat_object_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_flat_object_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FlatObjectProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "flat_object_property")
    }
    #[doc = "Parses the value as the `nested_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nested_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::NestedProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nested_property")
    }
    #[doc = "Parses the value as the `object_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ObjectProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_property")
    }
    #[doc = "Parses the value as the `completion_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_completion_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::CompletionProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "completion_property")
    }
    #[doc = "Parses the value as the `constant_keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_constant_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ConstantKeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "constant_keyword_property")
    }
    #[doc = "Parses the value as the `field_alias_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_field_alias_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FieldAliasProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "field_alias_property")
    }
    #[doc = "Parses the value as the `histogram_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_histogram_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::HistogramProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "histogram_property")
    }
    #[doc = "Parses the value as the `ip_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ip_property(
        &self,
    ) -> std::result::Result<crate::types::common_mapping::IpProperty, crate::types::UnionBranchError>
    {
        crate::types::union::parse_branch(&self.raw, "ip_property")
    }
    #[doc = "Parses the value as the `murmur_3_hash_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_murmur_3_hash_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::Murmur3HashProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "murmur_3_hash_property")
    }
    #[doc = "Parses the value as the `token_count_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_token_count_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::TokenCountProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "token_count_property")
    }
    #[doc = "Parses the value as the `geo_point_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_point_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::GeoPointProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_point_property")
    }
    #[doc = "Parses the value as the `geo_shape_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_shape_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::GeoShapeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_shape_property")
    }
    #[doc = "Parses the value as the `xy_point_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_xy_point_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::XyPointProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "xy_point_property")
    }
    #[doc = "Parses the value as the `xy_shape_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_xy_shape_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::XyShapeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "xy_shape_property")
    }
    #[doc = "Parses the value as the `byte_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_byte_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ByteNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "byte_number_property")
    }
    #[doc = "Parses the value as the `double_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_double_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DoubleNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "double_number_property")
    }
    #[doc = "Parses the value as the `float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "float_number_property")
    }
    #[doc = "Parses the value as the `half_float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_half_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::HalfFloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "half_float_number_property")
    }
    #[doc = "Parses the value as the `integer_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IntegerNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "integer_number_property")
    }
    #[doc = "Parses the value as the `long_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_long_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::LongNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "long_number_property")
    }
    #[doc = "Parses the value as the `scaled_float_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_scaled_float_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ScaledFloatNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "scaled_float_number_property")
    }
    #[doc = "Parses the value as the `semantic_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_semantic_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::SemanticProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "semantic_property")
    }
    #[doc = "Parses the value as the `short_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_short_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::ShortNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "short_number_property")
    }
    #[doc = "Parses the value as the `unsigned_long_number_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_unsigned_long_number_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::UnsignedLongNumberProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "unsigned_long_number_property")
    }
    #[doc = "Parses the value as the `date_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DateRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_range_property")
    }
    #[doc = "Parses the value as the `double_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_double_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::DoubleRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "double_range_property")
    }
    #[doc = "Parses the value as the `float_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_float_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::FloatRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "float_range_property")
    }
    #[doc = "Parses the value as the `integer_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_integer_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IntegerRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "integer_range_property")
    }
    #[doc = "Parses the value as the `ip_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ip_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IpRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ip_range_property")
    }
    #[doc = "Parses the value as the `long_range_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_long_range_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::LongRangeProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "long_range_property")
    }
    #[doc = "Parses the value as the `knn_vector_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_knn_vector_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::KnnVectorProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "knn_vector_property")
    }
    #[doc = "Parses the value as the `icu_collation_keyword_property` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_collation_keyword_property(
        &self,
    ) -> std::result::Result<
        crate::types::common_mapping::IcuCollationKeywordProperty,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_collation_keyword_property")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeatureProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub positive_score_impact: Option<bool>,
    pub r#type: crate::types::common_mapping::RankFeaturePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankFeaturePropertyType {
    #[doc = "`rank_feature`"]
    RankFeature,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RankFeaturePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RankFeaturePropertyType::RankFeature => "rank_feature",
            RankFeaturePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RankFeaturePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RankFeaturePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rank_feature" => RankFeaturePropertyType::RankFeature,
            _ => RankFeaturePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RankFeaturesProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    pub r#type: crate::types::common_mapping::RankFeaturesPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RankFeaturesPropertyType {
    #[doc = "`rank_features`"]
    RankFeatures,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RankFeaturesPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RankFeaturesPropertyType::RankFeatures => "rank_features",
            RankFeaturesPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RankFeaturesPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RankFeaturesPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rank_features" => RankFeaturesPropertyType::RankFeatures,
            _ => RankFeaturesPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoutingField {
    pub required: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScaledFloatNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scaling_factor: Option<f64>,
    pub r#type: crate::types::common_mapping::ScaledFloatNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScaledFloatNumberPropertyType {
    #[doc = "`scaled_float`"]
    ScaledFloat,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScaledFloatNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScaledFloatNumberPropertyType::ScaledFloat => "scaled_float",
            ScaledFloatNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScaledFloatNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScaledFloatNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "scaled_float" => ScaledFloatNumberPropertyType::ScaledFloat,
            _ => ScaledFloatNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchAsYouTypeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::SearchAsYouTypePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchAsYouTypePropertyType {
    #[doc = "`search_as_you_type`"]
    SearchAsYouType,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SearchAsYouTypePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SearchAsYouTypePropertyType::SearchAsYouType => "search_as_you_type",
            SearchAsYouTypePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SearchAsYouTypePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SearchAsYouTypePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "search_as_you_type" => SearchAsYouTypePropertyType::SearchAsYouType,
            _ => SearchAsYouTypePropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticChunkingStrategy {
    pub algorithm: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticDenseEmbeddingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compression_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::common_mapping::KnnVectorMethod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chunking: Option<crate::types::common_mapping::SemanticPropertyChunking>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dense_embedding_config: Option<crate::types::common_mapping::SemanticDenseEmbeddingConfig>,
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_field_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_field_search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_info_field_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub skip_existing_embedding: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sparse_encoding_config: Option<crate::types::common_mapping::SemanticSparseEncodingConfig>,
    pub r#type: crate::types::common_mapping::SemanticPropertyType,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SemanticPropertyChunking {
    Boolean(bool),
    Array(Vec<crate::types::common_mapping::SemanticChunkingStrategy>),
}
impl<'de> serde::Deserialize<'de> for SemanticPropertyChunking {
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
            Some(b't') | Some(b'f') => {
                serde_json::from_str(text).map(SemanticPropertyChunking::Boolean)
            }
            Some(b'[') => serde_json::from_str(text).map(SemanticPropertyChunking::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SemanticPropertyChunking"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SemanticPropertyType {
    #[doc = "`semantic`"]
    Semantic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SemanticPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SemanticPropertyType::Semantic => "semantic",
            SemanticPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SemanticPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SemanticPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "semantic" => SemanticPropertyType::Semantic,
            _ => SemanticPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SemanticSparseEncodingConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune_ratio: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune_type: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShortNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Short>,
    pub r#type: crate::types::common_mapping::ShortNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShortNumberPropertyType {
    #[doc = "`short`"]
    Short,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShortNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShortNumberPropertyType::Short => "short",
            ShortNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShortNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShortNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "short" => ShortNumberPropertyType::Short,
            _ => ShortNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SizeField {
    pub enabled: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SourceField {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compress_threshold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub excludes: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SuggestContext {
    pub name: crate::types::common::Name,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<crate::types::common::Field>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precision: Option<crate::types::common_mapping::SuggestContextPrecision>,
    pub r#type: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SuggestContextPrecision {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SuggestContextPrecision {
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
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SuggestContextPrecision::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SuggestContextPrecision::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SuggestContextPrecision"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TermVectorOption {
    #[doc = "`no`"]
    No,
    #[doc = "`with_offsets`"]
    WithOffsets,
    #[doc = "`with_positions`"]
    WithPositions,
    #[doc = "`with_positions_offsets`"]
    WithPositionsOffsets,
    #[doc = "`with_positions_offsets_payloads`"]
    WithPositionsOffsetsPayloads,
    #[doc = "`with_positions_payloads`"]
    WithPositionsPayloads,
    #[doc = "`yes`"]
    Yes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TermVectorOption {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TermVectorOption::No => "no",
            TermVectorOption::WithOffsets => "with_offsets",
            TermVectorOption::WithPositions => "with_positions",
            TermVectorOption::WithPositionsOffsets => "with_positions_offsets",
            TermVectorOption::WithPositionsOffsetsPayloads => "with_positions_offsets_payloads",
            TermVectorOption::WithPositionsPayloads => "with_positions_payloads",
            TermVectorOption::Yes => "yes",
            TermVectorOption::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TermVectorOption {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TermVectorOption {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "no" => TermVectorOption::No,
            "with_offsets" => TermVectorOption::WithOffsets,
            "with_positions" => TermVectorOption::WithPositions,
            "with_positions_offsets" => TermVectorOption::WithPositionsOffsets,
            "with_positions_offsets_payloads" => TermVectorOption::WithPositionsOffsetsPayloads,
            "with_positions_payloads" => TermVectorOption::WithPositionsPayloads,
            "yes" => TermVectorOption::Yes,
            _ => TermVectorOption::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextIndexPrefixes {
    pub max_chars: i64,
    pub min_chars: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TextProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub eager_global_ordinals: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fielddata_frequency_filter: Option<crate::types::indices_common::FielddataFrequencyFilter>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_options: Option<crate::types::common_mapping::IndexOptions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_phrases: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_prefixes: Option<crate::types::common_mapping::TextIndexPrefixes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub norms: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_increment_gap: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_quote_analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub term_vector: Option<crate::types::common_mapping::TermVectorOption>,
    pub r#type: crate::types::common_mapping::TextPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TextPropertyType {
    #[doc = "`text`"]
    Text,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TextPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TextPropertyType::Text => "text",
            TextPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TextPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TextPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "text" => TextPropertyType::Text,
            _ => TextPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenCountProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub analyzer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_position_increments: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<f64>,
    pub r#type: crate::types::common_mapping::TokenCountPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenCountPropertyType {
    #[doc = "`token_count`"]
    TokenCount,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TokenCountPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TokenCountPropertyType::TokenCount => "token_count",
            TokenCountPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TokenCountPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TokenCountPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "token_count" => TokenCountPropertyType::TokenCount,
            _ => TokenCountPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TypeMapping {
    #[serde(rename = "_data_stream_timestamp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data_stream_timestamp: Option<crate::types::common_mapping::DataStreamTimestamp>,
    #[serde(rename = "_field_names")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_names: Option<crate::types::common_mapping::FieldNamesField>,
    #[serde(rename = "_meta")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(rename = "_routing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common_mapping::RoutingField>,
    #[serde(rename = "_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<crate::types::common_mapping::SizeField>,
    #[serde(rename = "_source")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<crate::types::common_mapping::SourceField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all_field: Option<crate::types::common_mapping::AllField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_detection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_date_formats: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic_templates: Option<
        Vec<std::collections::BTreeMap<String, crate::types::common_mapping::DynamicTemplate>>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_field: Option<crate::types::common_mapping::IndexField>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_detection: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnsignedLongNumberProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub boost: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::Ulong>,
    pub r#type: crate::types::common_mapping::UnsignedLongNumberPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UnsignedLongNumberPropertyType {
    #[doc = "`unsigned_long`"]
    UnsignedLong,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UnsignedLongNumberPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UnsignedLongNumberPropertyType::UnsignedLong => "unsigned_long",
            UnsignedLongNumberPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UnsignedLongNumberPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UnsignedLongNumberPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "unsigned_long" => UnsignedLongNumberPropertyType::UnsignedLong,
            _ => UnsignedLongNumberPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VersionProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    pub r#type: crate::types::common_mapping::VersionPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionPropertyType {
    #[doc = "`version`"]
    Version,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl VersionPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            VersionPropertyType::Version => "version",
            VersionPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for VersionPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for VersionPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "version" => VersionPropertyType::Version,
            _ => VersionPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WildcardProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalizer: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<String>,
    pub r#type: crate::types::common_mapping::WildcardPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WildcardPropertyType {
    #[doc = "`wildcard`"]
    Wildcard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WildcardPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WildcardPropertyType::Wildcard => "wildcard",
            WildcardPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WildcardPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WildcardPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "wildcard" => WildcardPropertyType::Wildcard,
            _ => WildcardPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyPointProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub null_value: Option<crate::types::common::XyLocation>,
    pub r#type: crate::types::common_mapping::XyPointPropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XyPointPropertyType {
    #[doc = "`xy_point`"]
    XyPoint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl XyPointPropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            XyPointPropertyType::XyPoint => "xy_point",
            XyPointPropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for XyPointPropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for XyPointPropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "xy_point" => XyPointPropertyType::XyPoint,
            _ => XyPointPropertyType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct XyShapeProperty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::common_mapping::DynamicMapping>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_above: Option<i64>,
    #[doc = "Metadata about the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<std::collections::BTreeMap<String, String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub properties:
        Option<std::collections::BTreeMap<String, crate::types::common_mapping::Property>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_to: Option<crate::types::common::Fields>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub similarity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_values: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coerce: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_malformed: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_z_value: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orientation: Option<crate::types::common_mapping::GeoOrientation>,
    pub r#type: crate::types::common_mapping::XyShapePropertyType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum XyShapePropertyType {
    #[doc = "`xy_shape`"]
    XyShape,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl XyShapePropertyType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            XyShapePropertyType::XyShape => "xy_shape",
            XyShapePropertyType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for XyShapePropertyType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for XyShapePropertyType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "xy_shape" => XyShapePropertyType::XyShape,
            _ => XyShapePropertyType::Unknown(s),
        })
    }
}
