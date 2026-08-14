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

/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 *
 * Modifications Copyright OpenSearch Contributors. See
 * GitHub history for details.
 */

//! API parameters

use core::fmt;
use serde::{de, de::Visitor, Deserializer, Serializer};

// GENERATED-BEGIN:spec-params
// Generated code - do not edit until the next GENERATED-END marker

use serde::{Deserialize, Serialize};
#[doc = "Used to filter by alert state. Optional."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum AlertState {
    #[serde(rename = "ACKNOWLEDGED")]
    Acknowledged,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "DELETED")]
    Deleted,
    #[serde(rename = "ERROR")]
    Error,
}
#[doc = "The units used to display byte values."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Bytes {
    #[serde(rename = "b")]
    B,
    #[serde(rename = "kb")]
    Kb,
    #[serde(rename = "k")]
    K,
    #[serde(rename = "mb")]
    Mb,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "gb")]
    Gb,
    #[serde(rename = "g")]
    G,
    #[serde(rename = "tb")]
    Tb,
    #[serde(rename = "t")]
    T,
    #[serde(rename = "pb")]
    Pb,
    #[serde(rename = "p")]
    P,
}
#[doc = "Type of notification configuration."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum ConfigType {
    #[serde(rename = "chime")]
    Chime,
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "email_group")]
    EmailGroup,
    #[serde(rename = "microsoft_teams")]
    MicrosoftTeams,
    #[serde(rename = "ses_account")]
    SesAccount,
    #[serde(rename = "slack")]
    Slack,
    #[serde(rename = "smtp_account")]
    SmtpAccount,
    #[serde(rename = "sns")]
    Sns,
    #[serde(rename = "webhook")]
    Webhook,
}
#[doc = "What to do if delete by query hits version conflicts: `abort` or `proceed`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Conflicts {
    #[serde(rename = "abort")]
    Abort,
    #[serde(rename = "proceed")]
    Proceed,
}
#[doc = "The default operator for query string query: `AND` or `OR`.\nThis parameter can only be used when the `q` query string parameter is specified."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum DefaultOperator {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
#[doc = "The detection type that dictates the retrieval type for the findings. When the detection type is `threat`, it fetches threat intelligence feeds. When the detection type is `rule`, findings are fetched based on the detector’s rule. Optional."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum DetectionType {
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "threat")]
    Threat,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum ExpandWildcards {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "closed")]
    Closed,
    #[serde(rename = "hidden")]
    Hidden,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "open")]
    Open,
}
#[doc = "Groups tasks by parent/child relationships or nodes."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum GroupBy {
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "parents")]
    Parents,
}
#[doc = "Limits indexes based on their health status. Supported values are `green`, `yellow`, and `red`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Health {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Level {
    #[serde(rename = "awareness_attributes")]
    AwarenessAttributes,
    #[serde(rename = "cluster")]
    Cluster,
    #[serde(rename = "indices")]
    Indices,
    #[serde(rename = "shards")]
    Shards,
}
#[doc = "Limits the information returned to the specified metrics."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Metric {
    #[serde(rename = "_all")]
    All,
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "cluster_manager_node")]
    ClusterManagerNode,
    #[serde(rename = "master_node")]
    MasterNode,
    #[serde(rename = "metadata")]
    Metadata,
    #[serde(rename = "nodes")]
    Nodes,
    #[serde(rename = "routing_nodes")]
    RoutingNodes,
    #[serde(rename = "routing_table")]
    RoutingTable,
    #[serde(rename = "version")]
    Version,
}
#[doc = "Set to create to only index the document if it does not already exist (put if absent).\nIf a document with the specified `_id` already exists, the indexing operation will fail.\nSame as using the `&lt;index&gt;/_create` endpoint.\nValid values: `index`, `create`.\nIf document id is specified, it defaults to `index`.\nOtherwise, it defaults to `create`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum OpType {
    #[serde(rename = "create")]
    Create,
    #[serde(rename = "index")]
    Index,
}
#[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Refresh {
    #[serde(rename = "false")]
    False,
    #[serde(rename = "true")]
    True,
    #[serde(rename = "wait_for")]
    WaitFor,
}
#[doc = "The type of the search operation.\nAvailable options: `query_then_fetch`, `dfs_query_then_fetch`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum SearchType {
    #[serde(rename = "dfs_query_then_fetch")]
    DfsQueryThenFetch,
    #[serde(rename = "query_then_fetch")]
    QueryThenFetch,
}
#[doc = "The rule severity for which retrieve findings. Severity can be `critical`, `high`, `medium`, or `low`. Optional."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Severity {
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
}
#[doc = "Defines order in which indexes will be displayed. Accepted values are `asc` and `desc`. If `desc`, most recently created indexes would be displayed first."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Sort {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
#[doc = "The order used to sort the list of findings. Possible values are `asc` or `desc`. Optional."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum SortOrder {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
#[doc = "A list of shard health statuses used to limit the request."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Status {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "yellow")]
    Yellow,
}
#[doc = "Specify suggest mode."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum SuggestMode {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "popular")]
    Popular,
}
#[doc = "The unit used to display time values."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Time {
    #[serde(rename = "nanos")]
    Nanos,
    #[serde(rename = "micros")]
    Micros,
    #[serde(rename = "ms")]
    Ms,
    #[serde(rename = "s")]
    S,
    #[serde(rename = "m")]
    M,
    #[serde(rename = "h")]
    H,
    #[serde(rename = "d")]
    D,
}
#[doc = "Get top n queries by a specific metric."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum Type {
    #[serde(rename = "cpu")]
    Cpu,
    #[serde(rename = "latency")]
    Latency,
    #[serde(rename = "memory")]
    Memory,
}
#[doc = "The specific version type: `external`, `external_gte`."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum VersionType {
    #[serde(rename = "external")]
    External,
    #[serde(rename = "external_gte")]
    ExternalGte,
    #[serde(rename = "internal")]
    Internal,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum WaitForEvents {
    #[serde(rename = "immediate")]
    Immediate,
    #[serde(rename = "urgent")]
    Urgent,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "languid")]
    Languid,
}
#[doc = "Waits until the cluster health reaches the specified status or better."]
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, Copy)]
pub enum WaitForStatus {
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "red")]
    Red,
}
// GENERATED-END

/// Control how the total number of hits should be tracked.
///
/// When set to `Track` with a value `true`, the response will always track the number of hits that
/// match the query accurately.
///
/// When set to `Count` with an integer value `n`, the response accurately tracks the total
/// hit count that match the query up to `n` documents.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TrackTotalHits {
    /// Whether to accurately track the number of hits that match the query accurately
    Track(bool),
    /// Accurately track the number of hits up to the specified value
    Count(i64),
}

impl From<bool> for TrackTotalHits {
    fn from(b: bool) -> Self {
        TrackTotalHits::Track(b)
    }
}

impl From<i64> for TrackTotalHits {
    fn from(i: i64) -> Self {
        TrackTotalHits::Count(i)
    }
}

/// Control how the `_source` field is returned with every hit.
///
/// By default operations return the contents of the `_source` field
/// unless you have used the `stored_fields` parameter or if the `_source` field is disabled.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SourceFilter {
    /// Whether `_source` retrieval should be enabled (`true`) or disabled (`false`)
    Enable(bool),

    /// A wildcard pattern to control what parts of `_source` should be returned
    Include(String),

    /// A collection of wildcard patterns to control what parts of `_source` should be returned
    Includes(Vec<String>),

    /// A collection of wildcard patterns to control what parts of `_source` should
    /// and should not be returned
    IncludesExcludes {
        includes: Vec<String>,
        excludes: Vec<String>,
    },
}

impl From<bool> for SourceFilter {
    fn from(b: bool) -> Self {
        SourceFilter::Enable(b)
    }
}

impl From<String> for SourceFilter {
    fn from(include: String) -> Self {
        SourceFilter::Include(include)
    }
}

impl<'a> From<&'a str> for SourceFilter {
    fn from(include: &'a str) -> Self {
        SourceFilter::Include(include.to_owned())
    }
}

impl From<Vec<String>> for SourceFilter {
    fn from(includes: Vec<String>) -> Self {
        SourceFilter::Includes(includes)
    }
}

impl<'a> From<Vec<&'a str>> for SourceFilter {
    fn from(includes: Vec<&'a str>) -> Self {
        SourceFilter::Includes(includes.iter().map(|s| (*s).to_string()).collect())
    }
}

impl From<(Vec<String>, Vec<String>)> for SourceFilter {
    fn from(includes_excludes: (Vec<String>, Vec<String>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes.0,
            excludes: includes_excludes.1,
        }
    }
}

impl<'a> From<(Vec<&'a str>, Vec<&'a str>)> for SourceFilter {
    fn from(includes_excludes: (Vec<&'a str>, Vec<&'a str>)) -> Self {
        SourceFilter::IncludesExcludes {
            includes: includes_excludes
                .0
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
            excludes: includes_excludes
                .1
                .iter()
                .map(|s| (*s).to_string())
                .collect(),
        }
    }
}

/// Control the number of slices a task should be divided into. Defaults to `Slices::Count(1)`,
/// meaning the task is not sliced.
///
/// When set to `Auto`, a task is automatically divided into a reasonable number of slices
///
/// When set to `Count` with an integer value `n`, divides the task into that number of slices
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Slices {
    /// Automatically divide the task into a reasonable number of slices
    Auto,
    /// Number of slices to divide a task into
    Count(i32),
}

impl Serialize for Slices {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            Slices::Auto => serializer.serialize_str("auto"),
            Slices::Count(i) => serializer.serialize_i32(i),
        }
    }
}

impl<'de> Deserialize<'de> for Slices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct SlicesVisitor;

        impl<'de> Visitor<'de> for SlicesVisitor {
            type Value = Slices;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "expected integer or string")
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i32::max_value() as i64 {
                    Ok(Slices::Count(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= i32::max_value() as u64 {
                    Ok(Slices::Count(value as i32))
                } else {
                    Err(E::custom(format!("i32 out of range: {}", value)))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match value {
                    "auto" => Ok(Slices::Auto),
                    n => match n.parse::<i32>() {
                        Ok(i) => Ok(Slices::Count(i)),
                        Err(_) => Err(E::custom(format!(
                            "expected 'auto' or i32 but received: {}",
                            n
                        ))),
                    },
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(SlicesVisitor)
    }
}

impl Default for Slices {
    fn default() -> Self {
        Slices::Count(1)
    }
}

impl From<i32> for Slices {
    fn from(i: i32) -> Self {
        Slices::Count(i)
    }
}

#[cfg(test)]
mod tests {
    use crate::params::Slices;

    #[test]
    fn serialize_slices_auto() {
        let json = serde_json::to_string(&Slices::Auto).unwrap();
        assert_eq!("\"auto\"", &json);
        let slices: Slices = serde_json::from_str(&json).unwrap();
        assert_eq!(Slices::Auto, slices);
    }

    #[test]
    fn serialize_slices_count() {
        let json = serde_json::to_string(&Slices::Count(100)).unwrap();
        assert_eq!("100", &json);
        let slices: Slices = serde_json::from_str(&json).unwrap();
        assert_eq!(Slices::Count(100), slices);
    }
}
