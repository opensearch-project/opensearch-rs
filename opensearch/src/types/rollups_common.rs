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

//! Response types of the `rollups_common` specification namespace

#![allow(missing_docs)]

#[doc = "Defines a cron schedule for the rollup job execution."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Cron {
    #[doc = "The cron expression that defines the schedule."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
    #[doc = "The timezone for interpreting the cron expression."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[doc = "Configures a date histogram dimension for rollup aggregation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateHistogramDimension {
    #[doc = "The calendar interval for the date histogram."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_interval: Option<String>,
    #[doc = "The fixed interval for the date histogram."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_interval: Option<String>,
    #[doc = "The date format pattern for the histogram values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[doc = "The field from the source documents to use for the date histogram."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the date histogram results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
    #[doc = "The timezone for the date histogram calculations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

#[doc = "Defines a dimension for aggregating data in a rollup job."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DimensionsConfigItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date_histogram: Option<crate::types::rollups_common::DateHistogramDimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub histogram: Option<crate::types::rollups_common::HistogramDimension>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms: Option<crate::types::rollups_common::TermsDimension>,
}

#[doc = "Provides explanation details for a rollup job."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Explain {
    #[doc = "The metadata ID for the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[doc = "The metadata associated with the rollup job execution."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollup_metadata:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[doc = "Maps rollup IDs to their explanation details."]
pub type ExplainEntities =
    std::collections::BTreeMap<String, crate::types::rollups_common::Explain>;

#[doc = "Configures a numeric histogram dimension for rollup aggregation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramDimension {
    #[doc = "The interval for the histogram buckets."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    #[doc = "The field from the source documents to use for the histogram."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the histogram results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[doc = "Specifies the timing details for rollup job execution."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Interval {
    #[doc = "The cron schedule configuration for the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<crate::types::rollups_common::IntervalCron>,
    #[doc = "The numerical value for the execution interval."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period: Option<f64>,
    #[doc = "The delay in milliseconds before starting the rollup process."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule_delay: Option<f64>,
    #[doc = "When the rollup job should start, in epoch milliseconds."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<f64>,
    #[doc = "The time unit for the execution interval, such as `MINUTES` or `HOURS`."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}

#[doc = "The cron schedule configuration for the rollup job."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IntervalCron {
    Array(Vec<crate::types::rollups_common::Cron>),
    Cron(crate::types::rollups_common::Cron),
}
impl<'de> serde::Deserialize<'de> for IntervalCron {
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
            Some(b'[') => serde_json::from_str(text).map(IntervalCron::Array),
            Some(b'{') => serde_json::from_str(text).map(IntervalCron::Cron),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IntervalCron"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Defines metric aggregations for a field in a rollup job."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsConfigItem {
    #[doc = "The list of metrics to calculate for the field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::types::rollups_common::MetricsConfigMetrics>>,
    #[doc = "The field from the source documents to calculate metrics on."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[doc = "The field name prefix to use in the target index for the metric results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[doc = "Specifies a metric aggregation type to calculate."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetricsConfigMetrics {
    #[doc = "Calculates the average value of a field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Finds the maximum value of a field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Finds the minimum value of a field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Calculates the sum of values for a field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Counts the number of values for a field."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_count: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[doc = "Defines the configuration for a rollup job."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Rollup {
    #[doc = "Whether the rollup job runs continuously."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuous: Option<bool>,
    #[doc = "The time delay in milliseconds between consecutive rollup executions."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<f64>,
    #[doc = "The description of the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The dimensions to aggregate data in the rollup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Vec<crate::types::rollups_common::DimensionsConfigItem>>,
    #[doc = "Whether the rollup job is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "When the rollup job was enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled_time: Option<f64>,
    #[doc = "Set up a Mustache message template for error notifications. For example, if an index rollup job fails, the system can send  a message to a Slack channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_notification: Option<String>,
    #[doc = "When the rollup job was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<f64>,
    #[doc = "The metadata ID associated with the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata_id: Option<String>,
    #[doc = "The metrics to calculate in the rollup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Vec<crate::types::rollups_common::MetricsConfigItem>>,
    #[doc = "The number of documents to process per iteration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<f64>,
    #[doc = "The ID for the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollup_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schedule: Option<crate::types::rollups_common::Schedule>,
    #[doc = "The schema version for the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<f64>,
    #[doc = "The source index pattern for the rollup job."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_index: Option<String>,
    #[doc = "The target index where rollup results are stored."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index_settings: Option<crate::types::indices_common::IndexSettings>,
}

#[doc = "A rollup job entity with metadata."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RollupEntity {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<i64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollup: Option<crate::types::rollups_common::Rollup>,
}

#[doc = "Defines the execution schedule for a rollup job."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<crate::types::rollups_common::Interval>,
}

#[doc = "Configures a terms dimension for rollup aggregation."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TermsDimension {
    #[doc = "The field from the source documents to use for terms aggregation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_field: Option<String>,
    #[doc = "The field name to use in the target index for the terms aggregation results."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}
