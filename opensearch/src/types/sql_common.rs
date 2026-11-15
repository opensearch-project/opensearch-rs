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

//! Response types of the `sql_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cursor {
    #[doc = "The time to keep cursors alive."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_alive: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainBody {
    #[doc = "The child steps in the execution plan."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<crate::types::sql_common::ExplainBody>>,
    #[doc = "The details about the execution step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The name of the execution step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root: Option<crate::types::sql_common::ExplainBody>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Plugins {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ppl: Option<crate::types::sql_common::Ppl>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::types::sql_common::PluginsQuery>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sql: Option<crate::types::sql_common::Sql>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginsQuery {
    #[doc = "The maximum memory allowed for queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_limit: Option<String>,
    #[doc = "The maximum result set size."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_limit: Option<crate::types::sql_common::PluginsQuerySizeLimit>,
}

#[doc = "The maximum result set size."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PluginsQuerySizeLimit {
    String(String),
    Integer(i64),
}
impl<'de> serde::Deserialize<'de> for PluginsQuerySizeLimit {
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
            Some(b'"') => serde_json::from_str(text).map(PluginsQuerySizeLimit::String),
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(PluginsQuerySizeLimit::Integer)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "PluginsQuerySizeLimit"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ppl {
    #[doc = "Whether the PPL plugin is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<crate::types::sql_common::PplEnabled>,
}

#[doc = "Whether the PPL plugin is enabled."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PplEnabled {
    Boolean(bool),
    String(String),
}
impl<'de> serde::Deserialize<'de> for PplEnabled {
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
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(PplEnabled::Boolean),
            Some(b'"') => serde_json::from_str(text).map(PplEnabled::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "PplEnabled"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryResponse {
    #[doc = "The cursor identifier for pagination."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[doc = "The rows of data returned by the query."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub datarows: Option<Vec<Vec<Box<serde_json::value::RawValue>>>>,
    #[doc = "The schema of the query result."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema: Option<Vec<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>>,
    #[doc = "The number of rows in the current response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    #[doc = "The status code of the response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[doc = "The total number of hits."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Sql {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<crate::types::sql_common::Cursor>,
    #[doc = "Whether the SQL plugin is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<crate::types::sql_common::SqlEnabled>,
    #[doc = "The threshold in milliseconds for logging slow queries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slowlog: Option<crate::types::sql_common::SqlSlowlog>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlCloseResponse {
    #[doc = "Whether the cursor was closed successfully."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<bool>,
}

#[doc = "Whether the SQL plugin is enabled."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SqlEnabled {
    Boolean(bool),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SqlEnabled {
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
            Some(b't') | Some(b'f') => serde_json::from_str(text).map(SqlEnabled::Boolean),
            Some(b'"') => serde_json::from_str(text).map(SqlEnabled::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SqlEnabled"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SqlSettingsResponse {
    #[doc = "Whether the settings were acknowledged."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub acknowledged: Option<bool>,
    #[doc = "The permanent settings that persist through restarts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub persistent: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transient: Option<crate::types::sql_common::Transient>,
}

#[doc = "The threshold in milliseconds for logging slow queries."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SqlSlowlog {
    Integer(i64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SqlSlowlog {
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
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(SqlSlowlog::Integer),
            Some(b'"') => serde_json::from_str(text).map(SqlSlowlog::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SqlSlowlog"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transient {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugins: Option<crate::types::sql_common::Plugins>,
}
