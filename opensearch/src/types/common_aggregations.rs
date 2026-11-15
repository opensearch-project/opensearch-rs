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

//! Response types of the `common_aggregations` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AdjacencyMatrixAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::AdjacencyMatrixAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum AdjacencyMatrixAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<
            String,
            crate::types::common_aggregations::AdjacencyMatrixBucket,
        >,
    ),
    Array(Vec<crate::types::common_aggregations::AdjacencyMatrixBucket>),
}
impl<'de> serde::Deserialize<'de> for AdjacencyMatrixAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(AdjacencyMatrixAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(AdjacencyMatrixAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "AdjacencyMatrixAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct AdjacencyMatrixBucket {
    pub doc_count: i64,
    pub key: String,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for AdjacencyMatrixBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AdjacencyMatrixBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AdjacencyMatrixBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AdjacencyMatrixBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AdjacencyMatrixBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Aggregate {
    raw: Box<serde_json::value::RawValue>,
}
impl Aggregate {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `adjacency_matrix` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_adjacency_matrix(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::AdjacencyMatrixAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "adjacency_matrix")
    }
    #[doc = "Parses the value as the `auto_date_histogram` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_auto_date_histogram(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::AutoDateHistogramAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "auto_date_histogram")
    }
    #[doc = "Parses the value as the `avg` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_avg(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::AvgAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "avg")
    }
    #[doc = "Parses the value as the `box_plot` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_box_plot(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::BoxPlotAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "box_plot")
    }
    #[doc = "Parses the value as the `bucket_metric_value` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_bucket_metric_value(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::BucketMetricValueAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "bucket_metric_value")
    }
    #[doc = "Parses the value as the `cardinality` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cardinality(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::CardinalityAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cardinality")
    }
    #[doc = "Parses the value as the `children` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_children(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ChildrenAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "children")
    }
    #[doc = "Parses the value as the `composite` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_composite(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::CompositeAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "composite")
    }
    #[doc = "Parses the value as the `date_histogram` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_histogram(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::DateHistogramAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_histogram")
    }
    #[doc = "Parses the value as the `date_range` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_date_range(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::DateRangeAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "date_range")
    }
    #[doc = "Parses the value as the `derivative` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_derivative(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::DerivativeAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "derivative")
    }
    #[doc = "Parses the value as the `dterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_dterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::DoubleTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "dterms")
    }
    #[doc = "Parses the value as the `extended_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_extended_stats(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ExtendedStatsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "extended_stats")
    }
    #[doc = "Parses the value as the `extended_stats_bucket` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_extended_stats_bucket(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ExtendedStatsBucketAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "extended_stats_bucket")
    }
    #[doc = "Parses the value as the `filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::FilterAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "filter")
    }
    #[doc = "Parses the value as the `filters` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filters(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::FiltersAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "filters")
    }
    #[doc = "Parses the value as the `geo_bounds` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_bounds(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GeoBoundsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_bounds")
    }
    #[doc = "Parses the value as the `geo_centroid` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_centroid(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GeoCentroidAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_centroid")
    }
    #[doc = "Parses the value as the `geo_distance` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geo_distance(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GeoDistanceAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geo_distance")
    }
    #[doc = "Parses the value as the `geohash_grid` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geohash_grid(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GeoHashGridAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geohash_grid")
    }
    #[doc = "Parses the value as the `geotile_grid` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_geotile_grid(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GeoTileGridAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "geotile_grid")
    }
    #[doc = "Parses the value as the `global` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_global(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::GlobalAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "global")
    }
    #[doc = "Parses the value as the `hdr_percentiles` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hdr_percentiles(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::HdrPercentilesAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hdr_percentiles")
    }
    #[doc = "Parses the value as the `hdr_percentile_ranks` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hdr_percentile_ranks(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::HdrPercentileRanksAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hdr_percentile_ranks")
    }
    #[doc = "Parses the value as the `histogram` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_histogram(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::HistogramAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "histogram")
    }
    #[doc = "Parses the value as the `ip_range` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ip_range(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::IpRangeAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ip_range")
    }
    #[doc = "Parses the value as the `lrareterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lrareterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::LongRareTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lrareterms")
    }
    #[doc = "Parses the value as the `lterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::LongTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lterms")
    }
    #[doc = "Parses the value as the `matrix_stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_matrix_stats(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MatrixStatsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "matrix_stats")
    }
    #[doc = "Parses the value as the `max` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_max(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MaxAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "max")
    }
    #[doc = "Parses the value as the `median_absolute_deviation` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_median_absolute_deviation(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MedianAbsoluteDeviationAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "median_absolute_deviation")
    }
    #[doc = "Parses the value as the `min` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_min(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MinAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "min")
    }
    #[doc = "Parses the value as the `missing` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_missing(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MissingAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "missing")
    }
    #[doc = "Parses the value as the `multi_terms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_multi_terms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::MultiTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "multi_terms")
    }
    #[doc = "Parses the value as the `nested` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nested(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::NestedAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nested")
    }
    #[doc = "Parses the value as the `parent` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_parent(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ParentAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "parent")
    }
    #[doc = "Parses the value as the `percentiles_bucket` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_percentiles_bucket(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::PercentilesBucketAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "percentiles_bucket")
    }
    #[doc = "Parses the value as the `range` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_range(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::RangeAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "range")
    }
    #[doc = "Parses the value as the `rate` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rate(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::RateAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rate")
    }
    #[doc = "Parses the value as the `reverse_nested` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_reverse_nested(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ReverseNestedAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "reverse_nested")
    }
    #[doc = "Parses the value as the `sampler` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sampler(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::SamplerAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sampler")
    }
    #[doc = "Parses the value as the `scripted_metric` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_scripted_metric(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ScriptedMetricAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "scripted_metric")
    }
    #[doc = "Parses the value as the `siglterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_siglterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::SignificantLongTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "siglterms")
    }
    #[doc = "Parses the value as the `sigsterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sigsterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::SignificantStringTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sigsterms")
    }
    #[doc = "Parses the value as the `simple_long_value` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_long_value(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::CumulativeCardinalityAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_long_value")
    }
    #[doc = "Parses the value as the `simple_value` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_value(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::SimpleValueAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_value")
    }
    #[doc = "Parses the value as the `stats` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stats(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::StatsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stats")
    }
    #[doc = "Parses the value as the `stats_bucket` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stats_bucket(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::StatsBucketAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stats_bucket")
    }
    #[doc = "Parses the value as the `srareterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_srareterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::StringRareTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "srareterms")
    }
    #[doc = "Parses the value as the `sterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::StringTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sterms")
    }
    #[doc = "Parses the value as the `sum` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sum(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::SumAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sum")
    }
    #[doc = "Parses the value as the `tdigest_percentiles` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_tdigest_percentiles(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::TDigestPercentilesAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "tdigest_percentiles")
    }
    #[doc = "Parses the value as the `tdigest_percentile_ranks` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_tdigest_percentile_ranks(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::TDigestPercentileRanksAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "tdigest_percentile_ranks")
    }
    #[doc = "Parses the value as the `t_test` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_t_test(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::TTestAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "t_test")
    }
    #[doc = "Parses the value as the `top_hits` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_top_hits(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::TopHitsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "top_hits")
    }
    #[doc = "Parses the value as the `ulterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ulterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::UnsignedLongTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ulterms")
    }
    #[doc = "Parses the value as the `umrareterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_umrareterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::UnmappedRareTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "umrareterms")
    }
    #[doc = "Parses the value as the `umsigterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_umsigterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::UnmappedSignificantTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "umsigterms")
    }
    #[doc = "Parses the value as the `umterms` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_umterms(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::UnmappedTermsAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "umterms")
    }
    #[doc = "Parses the value as the `value_count` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_value_count(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::ValueCountAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "value_count")
    }
    #[doc = "Parses the value as the `variable_width_histogram` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_variable_width_histogram(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::VariableWidthHistogramAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "variable_width_histogram")
    }
    #[doc = "Parses the value as the `weighted_avg` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_weighted_avg(
        &self,
    ) -> std::result::Result<
        crate::types::common_aggregations::WeightedAvgAggregate,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "weighted_avg")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArrayPercentilesItem {
    pub key: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AutoDateHistogramAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::AutoDateHistogramAggregateBuckets,
    pub interval: crate::types::common::DurationLarge,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum AutoDateHistogramAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<String, crate::types::common_aggregations::DateHistogramBucket>,
    ),
    Array(Vec<crate::types::common_aggregations::DateHistogramBucket>),
}
impl<'de> serde::Deserialize<'de> for AutoDateHistogramAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(AutoDateHistogramAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(AutoDateHistogramAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "AutoDateHistogramAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AvgAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BoxPlotAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub lower: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_as_string: Option<String>,
    pub max: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,
    pub min: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,
    #[serde(rename = "q1")]
    pub q_1: f64,
    #[serde(rename = "q1_as_string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q_1_as_string: Option<String>,
    #[serde(rename = "q2")]
    pub q_2: f64,
    #[serde(rename = "q2_as_string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q_2_as_string: Option<String>,
    #[serde(rename = "q3")]
    pub q_3: f64,
    #[serde(rename = "q3_as_string")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub q_3_as_string: Option<String>,
    pub upper: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BucketMetricValueAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
    pub keys: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CardinalityAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub value: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChildrenAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompositeAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::CompositeAggregateBuckets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub after_key: Option<crate::types::common_aggregations::CompositeAggregateKey>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum CompositeAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::CompositeBucket>),
    Array(Vec<crate::types::common_aggregations::CompositeBucket>),
}
impl<'de> serde::Deserialize<'de> for CompositeAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(CompositeAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(CompositeAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "CompositeAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

pub type CompositeAggregateKey =
    std::collections::BTreeMap<String, crate::types::common::FieldValue>;

#[derive(Debug, Clone)]
pub struct CompositeBucket {
    pub doc_count: i64,
    pub key: crate::types::common_aggregations::CompositeAggregateKey,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for CompositeBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompositeBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompositeBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<crate::types::common_aggregations::CompositeAggregateKey> =
                    None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<crate::types::common_aggregations::CompositeAggregateKey>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CompositeBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CompositeBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CumulativeCardinalityAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub value: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateHistogramAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::DateHistogramAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum DateHistogramAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<String, crate::types::common_aggregations::DateHistogramBucket>,
    ),
    Array(Vec<crate::types::common_aggregations::DateHistogramBucket>),
}
impl<'de> serde::Deserialize<'de> for DateHistogramAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(DateHistogramAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(DateHistogramAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "DateHistogramAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct DateHistogramBucket {
    pub doc_count: i64,
    pub key: crate::types::common::EpochTimeUnitMillis,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for DateHistogramBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateHistogramBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateHistogramBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<crate::types::common::EpochTimeUnitMillis> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(
                                map.next_value::<crate::types::common::EpochTimeUnitMillis>()?,
                            );
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateHistogramBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateHistogramBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DateRangeAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::DateRangeAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum DateRangeAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::RangeBucket>),
    Array(Vec<crate::types::common_aggregations::RangeBucket>),
}
impl<'de> serde::Deserialize<'de> for DateRangeAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(DateRangeAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(DateRangeAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "DateRangeAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DerivativeAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalized_value_as_string: Option<String>,
}

#[doc = "Result of a `terms` aggregation when the field is some kind of decimal number like a float, double, or distance."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DoubleTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::DoubleTermsBucket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct DoubleTermsBucket {
    pub doc_count: i64,
    #[doc = "Upper bound of the document count error. Only present when `show_term_doc_count_error` is true."]
    pub doc_count_error_upper_bound: Option<i64>,
    pub key: f64,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for DoubleTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DoubleTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DoubleTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_key: Option<f64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "doc_count_error_upper_bound" => {
                            if seen_doc_count_error_upper_bound {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_count_error_upper_bound",
                                ));
                            }
                            seen_doc_count_error_upper_bound = true;
                            f_doc_count_error_upper_bound = map.next_value::<Option<i64>>()?;
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<f64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DoubleTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DoubleTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedStatsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_as_string: Option<String>,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,
    pub sum: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds: Option<crate::types::common_aggregations::StandardDeviationBounds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds_as_string:
        Option<crate::types::common_aggregations::StandardDeviationBoundsAsString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_sampling: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_of_squares_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_population_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_sampling: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_sampling_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedStatsBucketAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_as_string: Option<String>,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,
    pub sum: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds: Option<crate::types::common_aggregations::StandardDeviationBounds>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_bounds_as_string:
        Option<crate::types::common_aggregations::StandardDeviationBoundsAsString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub std_deviation_sampling: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_of_squares: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_of_squares_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_population_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_sampling: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variance_sampling_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilterAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FiltersAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::FiltersAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum FiltersAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::FiltersBucket>),
    Array(Vec<crate::types::common_aggregations::FiltersBucket>),
}
impl<'de> serde::Deserialize<'de> for FiltersAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(FiltersAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(FiltersAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "FiltersAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct FiltersBucket {
    pub doc_count: i64,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for FiltersBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FiltersBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FiltersBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FiltersBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FiltersBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoBoundsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bounds: Option<crate::types::common::GeoBounds>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoCentroidAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<crate::types::common::GeoLocation>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoDistanceAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoDistanceAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum GeoDistanceAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::RangeBucket>),
    Array(Vec<crate::types::common_aggregations::RangeBucket>),
}
impl<'de> serde::Deserialize<'de> for GeoDistanceAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(GeoDistanceAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(GeoDistanceAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "GeoDistanceAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoHashGridAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoHashGridAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum GeoHashGridAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::GeoHashGridBucket>),
    Array(Vec<crate::types::common_aggregations::GeoHashGridBucket>),
}
impl<'de> serde::Deserialize<'de> for GeoHashGridAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(GeoHashGridAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(GeoHashGridAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "GeoHashGridAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct GeoHashGridBucket {
    pub doc_count: i64,
    pub key: crate::types::common::GeoHash,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for GeoHashGridBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoHashGridBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoHashGridBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<crate::types::common::GeoHash> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<crate::types::common::GeoHash>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoHashGridBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoHashGridBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeoTileGridAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoTileGridAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum GeoTileGridAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::GeoTileGridBucket>),
    Array(Vec<crate::types::common_aggregations::GeoTileGridBucket>),
}
impl<'de> serde::Deserialize<'de> for GeoTileGridAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(GeoTileGridAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(GeoTileGridAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "GeoTileGridAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct GeoTileGridBucket {
    pub doc_count: i64,
    pub key: crate::types::common::GeoTile,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for GeoTileGridBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoTileGridBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoTileGridBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<crate::types::common::GeoTile> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<crate::types::common::GeoTile>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoTileGridBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoTileGridBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GlobalAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HdrPercentileRanksAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HdrPercentilesAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HistogramAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::HistogramAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum HistogramAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::HistogramBucket>),
    Array(Vec<crate::types::common_aggregations::HistogramBucket>),
}
impl<'de> serde::Deserialize<'de> for HistogramAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(HistogramAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(HistogramAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "HistogramAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct HistogramBucket {
    pub doc_count: i64,
    pub key: f64,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for HistogramBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HistogramBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HistogramBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<f64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<f64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HistogramBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HistogramBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IpRangeAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::IpRangeAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum IpRangeAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::IpRangeBucket>),
    Array(Vec<crate::types::common_aggregations::IpRangeBucket>),
}
impl<'de> serde::Deserialize<'de> for IpRangeAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(IpRangeAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(IpRangeAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "IpRangeAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct IpRangeBucket {
    pub doc_count: i64,
    pub from: Option<String>,
    pub key: Option<String>,
    pub to: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for IpRangeBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IpRangeBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IpRangeBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_from: Option<String> = None;
                let mut seen_from = false;
                let mut f_key: Option<String> = None;
                let mut seen_key = false;
                let mut f_to: Option<String> = None;
                let mut seen_to = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "from" => {
                            if seen_from {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            seen_from = true;
                            f_from = map.next_value::<Option<String>>()?;
                        }
                        "key" => {
                            if seen_key {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            seen_key = true;
                            f_key = map.next_value::<Option<String>>()?;
                        }
                        "to" => {
                            if seen_to {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            seen_to = true;
                            f_to = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IpRangeBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    from: f_from,
                    key: f_key,
                    to: f_to,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IpRangeBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.from {
            map.serialize_entry("from", v)?;
        }
        if let Some(v) = &self.key {
            map.serialize_entry("key", v)?;
        }
        if let Some(v) = &self.to {
            map.serialize_entry("to", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

pub type KeyedPercentiles = std::collections::BTreeMap<
    String,
    crate::types::common_aggregations::KeyedPercentilesAdditional,
>;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum KeyedPercentilesAdditional {
    Number(f64),
    String(String),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for KeyedPercentilesAdditional {
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
                serde_json::from_str(text).map(KeyedPercentilesAdditional::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(KeyedPercentilesAdditional::String),
            Some(b'n') => return Ok(KeyedPercentilesAdditional::Null),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "KeyedPercentilesAdditional"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Result of the `rare_terms` aggregation when the field is some kind of whole number like a integer, long, or a date."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongRareTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::LongRareTermsAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum LongRareTermsAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<String, crate::types::common_aggregations::LongRareTermsBucket>,
    ),
    Array(Vec<crate::types::common_aggregations::LongRareTermsBucket>),
}
impl<'de> serde::Deserialize<'de> for LongRareTermsAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(LongRareTermsAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(LongRareTermsAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "LongRareTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct LongRareTermsBucket {
    pub doc_count: i64,
    pub key: i64,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for LongRareTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongRareTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongRareTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<i64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<i64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongRareTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongRareTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of a `terms` aggregation when the field is some kind of whole number like a integer, long, or a date."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LongTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::LongTermsBucket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct LongTermsBucket {
    pub doc_count: i64,
    #[doc = "Upper bound of the document count error. Only present when `show_term_doc_count_error` is true."]
    pub doc_count_error_upper_bound: Option<i64>,
    pub key: crate::types::common_aggregations::LongTermsBucketKey,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for LongTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_key: Option<crate::types::common_aggregations::LongTermsBucketKey> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "doc_count_error_upper_bound" => {
                            if seen_doc_count_error_upper_bound {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_count_error_upper_bound",
                                ));
                            }
                            seen_doc_count_error_upper_bound = true;
                            f_doc_count_error_upper_bound = map.next_value::<Option<i64>>()?;
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<crate::types::common_aggregations::LongTermsBucketKey>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum LongTermsBucketKey {
    Signed(i64),
    Unsigned(String),
}
impl<'de> serde::Deserialize<'de> for LongTermsBucketKey {
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
                serde_json::from_str(text).map(LongTermsBucketKey::Signed)
            }
            Some(b'"') => serde_json::from_str(text).map(LongTermsBucketKey::Unsigned),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "LongTermsBucketKey"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatrixStatsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<crate::types::common_aggregations::MatrixStatsFields>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MatrixStatsFields {
    pub correlation: std::collections::BTreeMap<String, f64>,
    pub count: i64,
    pub covariance: std::collections::BTreeMap<String, f64>,
    pub kurtosis: f64,
    pub mean: f64,
    pub name: crate::types::common::Field,
    pub skewness: f64,
    pub variance: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MaxAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MedianAbsoluteDeviationAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MinAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MissingAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::MultiTermsAggregateBuckets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum MultiTermsAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::MultiTermsBucket>),
    Array(Vec<crate::types::common_aggregations::MultiTermsBucket>),
}
impl<'de> serde::Deserialize<'de> for MultiTermsAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(MultiTermsAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(MultiTermsAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "MultiTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct MultiTermsBucket {
    pub doc_count: i64,
    pub doc_count_error_upper_bound: Option<i64>,
    pub key: Vec<crate::types::common::FieldValue>,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for MultiTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_key: Option<Vec<crate::types::common::FieldValue>> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "doc_count_error_upper_bound" => {
                            if seen_doc_count_error_upper_bound {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_count_error_upper_bound",
                                ));
                            }
                            seen_doc_count_error_upper_bound = true;
                            f_doc_count_error_upper_bound = map.next_value::<Option<i64>>()?;
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key =
                                Some(map.next_value::<Vec<crate::types::common::FieldValue>>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MultiTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MultiTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NestedAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParentAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum Percentiles {
    Keyed(crate::types::common_aggregations::KeyedPercentiles),
    Array(Vec<crate::types::common_aggregations::ArrayPercentilesItem>),
}
impl<'de> serde::Deserialize<'de> for Percentiles {
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
            Some(b'{') => serde_json::from_str(text).map(Percentiles::Keyed),
            Some(b'[') => serde_json::from_str(text).map(Percentiles::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "Percentiles"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PercentilesBucketAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RangeAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::RangeAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum RangeAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common_aggregations::RangeBucket>),
    Array(Vec<crate::types::common_aggregations::RangeBucket>),
}
impl<'de> serde::Deserialize<'de> for RangeAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(RangeAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(RangeAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "RangeAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct RangeBucket {
    pub doc_count: i64,
    pub from: Option<f64>,
    pub from_as_string: Option<String>,
    #[doc = "The bucket key. Present if the aggregation is not keyed."]
    pub key: Option<String>,
    pub to: Option<f64>,
    pub to_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for RangeBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RangeBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RangeBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_from: Option<f64> = None;
                let mut seen_from = false;
                let mut f_from_as_string: Option<String> = None;
                let mut seen_from_as_string = false;
                let mut f_key: Option<String> = None;
                let mut seen_key = false;
                let mut f_to: Option<f64> = None;
                let mut seen_to = false;
                let mut f_to_as_string: Option<String> = None;
                let mut seen_to_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "from" => {
                            if seen_from {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            seen_from = true;
                            f_from = map.next_value::<Option<f64>>()?;
                        }
                        "from_as_string" => {
                            if seen_from_as_string {
                                return Err(serde::de::Error::duplicate_field("from_as_string"));
                            }
                            seen_from_as_string = true;
                            f_from_as_string = map.next_value::<Option<String>>()?;
                        }
                        "key" => {
                            if seen_key {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            seen_key = true;
                            f_key = map.next_value::<Option<String>>()?;
                        }
                        "to" => {
                            if seen_to {
                                return Err(serde::de::Error::duplicate_field("to"));
                            }
                            seen_to = true;
                            f_to = map.next_value::<Option<f64>>()?;
                        }
                        "to_as_string" => {
                            if seen_to_as_string {
                                return Err(serde::de::Error::duplicate_field("to_as_string"));
                            }
                            seen_to_as_string = true;
                            f_to_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RangeBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    from: f_from,
                    from_as_string: f_from_as_string,
                    key: f_key,
                    to: f_to,
                    to_as_string: f_to_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RangeBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.from {
            map.serialize_entry("from", v)?;
        }
        if let Some(v) = &self.from_as_string {
            map.serialize_entry("from_as_string", v)?;
        }
        if let Some(v) = &self.key {
            map.serialize_entry("key", v)?;
        }
        if let Some(v) = &self.to {
            map.serialize_entry("to", v)?;
        }
        if let Some(v) = &self.to_as_string {
            map.serialize_entry("to_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RateAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub value: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReverseNestedAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SamplerAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScriptedMetricAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub value: Box<serde_json::value::RawValue>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignificantLongTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::SignificantLongTermsAggregateBuckets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SignificantLongTermsAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<
            String,
            crate::types::common_aggregations::SignificantLongTermsBucket,
        >,
    ),
    Array(Vec<crate::types::common_aggregations::SignificantLongTermsBucket>),
}
impl<'de> serde::Deserialize<'de> for SignificantLongTermsAggregateBuckets {
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
            Some(b'{') => {
                serde_json::from_str(text).map(SignificantLongTermsAggregateBuckets::Keyed)
            }
            Some(b'[') => {
                serde_json::from_str(text).map(SignificantLongTermsAggregateBuckets::Array)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SignificantLongTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SignificantLongTermsBucket {
    pub doc_count: i64,
    pub bg_count: i64,
    pub score: f64,
    pub key: i64,
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for SignificantLongTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SignificantLongTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SignificantLongTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_bg_count: Option<i64> = None;
                let mut f_score: Option<f64> = None;
                let mut f_key: Option<i64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "bg_count" => {
                            if f_bg_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("bg_count"));
                            }
                            f_bg_count = Some(map.next_value::<i64>()?);
                        }
                        "score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            f_score = Some(map.next_value::<f64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<i64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SignificantLongTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    bg_count: f_bg_count
                        .ok_or_else(|| serde::de::Error::missing_field("bg_count"))?,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("score"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SignificantLongTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("bg_count", &self.bg_count)?;
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SignificantStringTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::SignificantStringTermsAggregateBuckets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SignificantStringTermsAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<
            String,
            crate::types::common_aggregations::SignificantStringTermsBucket,
        >,
    ),
    Array(Vec<crate::types::common_aggregations::SignificantStringTermsBucket>),
}
impl<'de> serde::Deserialize<'de> for SignificantStringTermsAggregateBuckets {
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
            Some(b'{') => {
                serde_json::from_str(text).map(SignificantStringTermsAggregateBuckets::Keyed)
            }
            Some(b'[') => {
                serde_json::from_str(text).map(SignificantStringTermsAggregateBuckets::Array)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SignificantStringTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct SignificantStringTermsBucket {
    pub doc_count: i64,
    pub bg_count: i64,
    pub score: f64,
    pub key: String,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for SignificantStringTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SignificantStringTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SignificantStringTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_bg_count: Option<i64> = None;
                let mut f_score: Option<f64> = None;
                let mut f_key: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "bg_count" => {
                            if f_bg_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("bg_count"));
                            }
                            f_bg_count = Some(map.next_value::<i64>()?);
                        }
                        "score" => {
                            if f_score.is_some() {
                                return Err(serde::de::Error::duplicate_field("score"));
                            }
                            f_score = Some(map.next_value::<f64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SignificantStringTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    bg_count: f_bg_count
                        .ok_or_else(|| serde::de::Error::missing_field("bg_count"))?,
                    score: f_score.ok_or_else(|| serde::de::Error::missing_field("score"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SignificantStringTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("bg_count", &self.bg_count)?;
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleValueAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardDeviationBounds {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_sampling: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_population: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_sampling: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardDeviationBoundsAsString {
    pub lower: String,
    pub lower_population: String,
    pub lower_sampling: String,
    pub upper: String,
    pub upper_population: String,
    pub upper_sampling: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_as_string: Option<String>,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,
    pub sum: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StatsBucketAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avg_as_string: Option<String>,
    pub count: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_as_string: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_as_string: Option<String>,
    pub sum: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_as_string: Option<String>,
}

#[doc = "Result of the `rare_terms` aggregation when the field is a string."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StringRareTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::StringRareTermsAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum StringRareTermsAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<
            String,
            crate::types::common_aggregations::StringRareTermsBucket,
        >,
    ),
    Array(Vec<crate::types::common_aggregations::StringRareTermsBucket>),
}
impl<'de> serde::Deserialize<'de> for StringRareTermsAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(StringRareTermsAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(StringRareTermsAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "StringRareTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct StringRareTermsBucket {
    pub doc_count: i64,
    pub key: String,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for StringRareTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StringRareTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringRareTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StringRareTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StringRareTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of a `terms` aggregation when the field is a string."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StringTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::StringTermsBucket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct StringTermsBucket {
    pub doc_count: i64,
    #[doc = "Upper bound of the document count error. Only present when `show_term_doc_count_error` is true."]
    pub doc_count_error_upper_bound: Option<i64>,
    pub key: String,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for StringTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StringTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_key: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "doc_count_error_upper_bound" => {
                            if seen_doc_count_error_upper_bound {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_count_error_upper_bound",
                                ));
                            }
                            seen_doc_count_error_upper_bound = true;
                            f_doc_count_error_upper_bound = map.next_value::<Option<i64>>()?;
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StringTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StringTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        map.serialize_entry("key", &self.key)?;
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SumAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TDigestPercentileRanksAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TDigestPercentilesAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TTestAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TopHitsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub hits: crate::types::search::HitsMetadataJsonValue,
}

#[doc = "Result of a `rare_terms` aggregation when the field is unmapped. `buckets` is always empty."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnmappedRareTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::UnmappedRareTermsAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum UnmappedRareTermsAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common::Void>),
    Array(Vec<crate::types::common::Void>),
}
impl<'de> serde::Deserialize<'de> for UnmappedRareTermsAggregateBuckets {
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
            Some(b'{') => serde_json::from_str(text).map(UnmappedRareTermsAggregateBuckets::Keyed),
            Some(b'[') => serde_json::from_str(text).map(UnmappedRareTermsAggregateBuckets::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "UnmappedRareTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnmappedSignificantTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::UnmappedSignificantTermsAggregateBuckets,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bg_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum UnmappedSignificantTermsAggregateBuckets {
    Keyed(std::collections::BTreeMap<String, crate::types::common::Void>),
    Array(Vec<crate::types::common::Void>),
}
impl<'de> serde::Deserialize<'de> for UnmappedSignificantTermsAggregateBuckets {
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
            Some(b'{') => {
                serde_json::from_str(text).map(UnmappedSignificantTermsAggregateBuckets::Keyed)
            }
            Some(b'[') => {
                serde_json::from_str(text).map(UnmappedSignificantTermsAggregateBuckets::Array)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "UnmappedSignificantTermsAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Result of a `terms` aggregation when the field is unmapped. `buckets` is always empty."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnmappedTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common::Void>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[doc = "Result of a `terms` aggregation when the field is an `unsigned_long` (64-bit unsigned integer). OpenSearch stores `unsigned_long` values as Long or BigInteger internally. In JSON responses, bucket keys are represented as numbers, which may lose precision for values larger than 2^53--1 due to IEEE 754 double precision limitations. In protocol buffer representations, keys are manually defined as uint64 to preserve the full 64-bit unsigned range. Use `key_as_string` in buckets for exact string representation of large unsigned long values."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UnsignedLongTermsAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::UnsignedLongTermsBucket>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub doc_count_error_upper_bound: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sum_other_doc_count: Option<i64>,
}

#[doc = "Terms bucket for `unsigned_long` field type. The server returns 64-bit unsigned integer values as Long or BigInteger types. Due to JSON specification limitations (IEEE 754 double precision provides only 53 bits of integer precision), the key is represented as `type: number` in the OpenAPI spec. For protocol buffer definitions, this is manually overridden to `uint64` to preserve the full 64-bit unsigned integer range in binary encoding."]
#[derive(Debug, Clone)]
pub struct UnsignedLongTermsBucket {
    pub doc_count: i64,
    #[doc = "Upper bound of the document count error. Only present when `show_term_doc_count_error` is true."]
    pub doc_count_error_upper_bound: Option<i64>,
    #[doc = "The unsigned 64-bit integer bucket key. While JSON represents this as a number, values larger than 2^53--1 may lose precision in JSON serialization. The gRPC protocol buffer definition uses uint64 to preserve full precision. Use `key_as_string` for exact string representation of large values. "]
    pub key: f64,
    #[doc = "String representation of the key. Only present when a custom `format` parameter is specified in the aggregation request."]
    pub key_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for UnsignedLongTermsBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnsignedLongTermsBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnsignedLongTermsBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_key: Option<f64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "doc_count_error_upper_bound" => {
                            if seen_doc_count_error_upper_bound {
                                return Err(serde::de::Error::duplicate_field(
                                    "doc_count_error_upper_bound",
                                ));
                            }
                            seen_doc_count_error_upper_bound = true;
                            f_doc_count_error_upper_bound = map.next_value::<Option<i64>>()?;
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<f64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnsignedLongTermsBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnsignedLongTermsBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ValueCountAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct VariableWidthHistogramAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::VariableWidthHistogramAggregateBuckets,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum VariableWidthHistogramAggregateBuckets {
    Keyed(
        std::collections::BTreeMap<
            String,
            crate::types::common_aggregations::VariableWidthHistogramBucket,
        >,
    ),
    Array(Vec<crate::types::common_aggregations::VariableWidthHistogramBucket>),
}
impl<'de> serde::Deserialize<'de> for VariableWidthHistogramAggregateBuckets {
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
            Some(b'{') => {
                serde_json::from_str(text).map(VariableWidthHistogramAggregateBuckets::Keyed)
            }
            Some(b'[') => {
                serde_json::from_str(text).map(VariableWidthHistogramAggregateBuckets::Array)
            }
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "VariableWidthHistogramAggregateBuckets"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone)]
pub struct VariableWidthHistogramBucket {
    pub doc_count: i64,
    pub key: f64,
    pub key_as_string: Option<String>,
    pub max: f64,
    pub max_as_string: Option<String>,
    pub min: f64,
    pub min_as_string: Option<String>,
    pub aggregations:
        std::collections::BTreeMap<String, crate::types::common_aggregations::Aggregate>,
}
impl<'de> serde::Deserialize<'de> for VariableWidthHistogramBucket {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = VariableWidthHistogramBucket;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a VariableWidthHistogramBucket object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_doc_count: Option<i64> = None;
                let mut f_key: Option<f64> = None;
                let mut f_key_as_string: Option<String> = None;
                let mut seen_key_as_string = false;
                let mut f_max: Option<f64> = None;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::common_aggregations::Aggregate,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<f64>()?);
                        }
                        "key_as_string" => {
                            if seen_key_as_string {
                                return Err(serde::de::Error::duplicate_field("key_as_string"));
                            }
                            seen_key_as_string = true;
                            f_key_as_string = map.next_value::<Option<String>>()?;
                        }
                        "max" => {
                            if f_max.is_some() {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            f_max = Some(map.next_value::<f64>()?);
                        }
                        "max_as_string" => {
                            if seen_max_as_string {
                                return Err(serde::de::Error::duplicate_field("max_as_string"));
                            }
                            seen_max_as_string = true;
                            f_max_as_string = map.next_value::<Option<String>>()?;
                        }
                        "min" => {
                            if f_min.is_some() {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            f_min = Some(map.next_value::<f64>()?);
                        }
                        "min_as_string" => {
                            if seen_min_as_string {
                                return Err(serde::de::Error::duplicate_field("min_as_string"));
                            }
                            seen_min_as_string = true;
                            f_min_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(VariableWidthHistogramBucket {
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    key_as_string: f_key_as_string,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    max_as_string: f_max_as_string,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    min_as_string: f_min_as_string,
                    aggregations: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for VariableWidthHistogramBucket {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("doc_count", &self.doc_count)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.key_as_string {
            map.serialize_entry("key_as_string", v)?;
        }
        map.serialize_entry("max", &self.max)?;
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        map.serialize_entry("min", &self.min)?;
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        for (k, v) in &self.aggregations {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WeightedAvgAggregate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_as_string: Option<String>,
}
