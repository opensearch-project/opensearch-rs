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

#[derive(Debug, Clone)]
pub struct AdjacencyMatrixAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::AdjacencyMatrixAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AdjacencyMatrixAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AdjacencyMatrixAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AdjacencyMatrixAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::AdjacencyMatrixAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::AdjacencyMatrixAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AdjacencyMatrixAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AdjacencyMatrixAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct ArrayPercentilesItem {
    pub key: String,
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ArrayPercentilesItem {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ArrayPercentilesItem;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ArrayPercentilesItem object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_key: Option<String> = None;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "key" => {
                            if f_key.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            f_key = Some(map.next_value::<String>()?);
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ArrayPercentilesItem {
                    key: f_key.ok_or_else(|| serde::de::Error::missing_field("key"))?,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ArrayPercentilesItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("key", &self.key)?;
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct AutoDateHistogramAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::AutoDateHistogramAggregateBuckets,
    pub interval: crate::types::common::DurationLarge,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AutoDateHistogramAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AutoDateHistogramAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AutoDateHistogramAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::AutoDateHistogramAggregateBuckets,
                > = None;
                let mut f_interval: Option<crate::types::common::DurationLarge> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::AutoDateHistogramAggregateBuckets>()?);
                        }
                        "interval" => {
                            if f_interval.is_some() {
                                return Err(serde::de::Error::duplicate_field("interval"));
                            }
                            f_interval =
                                Some(map.next_value::<crate::types::common::DurationLarge>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AutoDateHistogramAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    interval: f_interval
                        .ok_or_else(|| serde::de::Error::missing_field("interval"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AutoDateHistogramAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        map.serialize_entry("interval", &self.interval)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct AvgAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for AvgAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = AvgAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a AvgAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(AvgAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for AvgAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct BoxPlotAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub lower: f64,
    pub lower_as_string: Option<String>,
    pub max: f64,
    pub max_as_string: Option<String>,
    pub min: f64,
    pub min_as_string: Option<String>,
    pub q_1: f64,
    pub q_1_as_string: Option<String>,
    pub q_2: f64,
    pub q_2_as_string: Option<String>,
    pub q_3: f64,
    pub q_3_as_string: Option<String>,
    pub upper: f64,
    pub upper_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BoxPlotAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BoxPlotAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BoxPlotAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_lower: Option<f64> = None;
                let mut f_lower_as_string: Option<String> = None;
                let mut seen_lower_as_string = false;
                let mut f_max: Option<f64> = None;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut f_q_1: Option<f64> = None;
                let mut f_q_1_as_string: Option<String> = None;
                let mut seen_q_1_as_string = false;
                let mut f_q_2: Option<f64> = None;
                let mut f_q_2_as_string: Option<String> = None;
                let mut seen_q_2_as_string = false;
                let mut f_q_3: Option<f64> = None;
                let mut f_q_3_as_string: Option<String> = None;
                let mut seen_q_3_as_string = false;
                let mut f_upper: Option<f64> = None;
                let mut f_upper_as_string: Option<String> = None;
                let mut seen_upper_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "lower" => {
                            if f_lower.is_some() {
                                return Err(serde::de::Error::duplicate_field("lower"));
                            }
                            f_lower = Some(map.next_value::<f64>()?);
                        }
                        "lower_as_string" => {
                            if seen_lower_as_string {
                                return Err(serde::de::Error::duplicate_field("lower_as_string"));
                            }
                            seen_lower_as_string = true;
                            f_lower_as_string = map.next_value::<Option<String>>()?;
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
                        "q1" => {
                            if f_q_1.is_some() {
                                return Err(serde::de::Error::duplicate_field("q1"));
                            }
                            f_q_1 = Some(map.next_value::<f64>()?);
                        }
                        "q1_as_string" => {
                            if seen_q_1_as_string {
                                return Err(serde::de::Error::duplicate_field("q1_as_string"));
                            }
                            seen_q_1_as_string = true;
                            f_q_1_as_string = map.next_value::<Option<String>>()?;
                        }
                        "q2" => {
                            if f_q_2.is_some() {
                                return Err(serde::de::Error::duplicate_field("q2"));
                            }
                            f_q_2 = Some(map.next_value::<f64>()?);
                        }
                        "q2_as_string" => {
                            if seen_q_2_as_string {
                                return Err(serde::de::Error::duplicate_field("q2_as_string"));
                            }
                            seen_q_2_as_string = true;
                            f_q_2_as_string = map.next_value::<Option<String>>()?;
                        }
                        "q3" => {
                            if f_q_3.is_some() {
                                return Err(serde::de::Error::duplicate_field("q3"));
                            }
                            f_q_3 = Some(map.next_value::<f64>()?);
                        }
                        "q3_as_string" => {
                            if seen_q_3_as_string {
                                return Err(serde::de::Error::duplicate_field("q3_as_string"));
                            }
                            seen_q_3_as_string = true;
                            f_q_3_as_string = map.next_value::<Option<String>>()?;
                        }
                        "upper" => {
                            if f_upper.is_some() {
                                return Err(serde::de::Error::duplicate_field("upper"));
                            }
                            f_upper = Some(map.next_value::<f64>()?);
                        }
                        "upper_as_string" => {
                            if seen_upper_as_string {
                                return Err(serde::de::Error::duplicate_field("upper_as_string"));
                            }
                            seen_upper_as_string = true;
                            f_upper_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BoxPlotAggregate {
                    meta: f_meta,
                    lower: f_lower.ok_or_else(|| serde::de::Error::missing_field("lower"))?,
                    lower_as_string: f_lower_as_string,
                    max: f_max.ok_or_else(|| serde::de::Error::missing_field("max"))?,
                    max_as_string: f_max_as_string,
                    min: f_min.ok_or_else(|| serde::de::Error::missing_field("min"))?,
                    min_as_string: f_min_as_string,
                    q_1: f_q_1.ok_or_else(|| serde::de::Error::missing_field("q1"))?,
                    q_1_as_string: f_q_1_as_string,
                    q_2: f_q_2.ok_or_else(|| serde::de::Error::missing_field("q2"))?,
                    q_2_as_string: f_q_2_as_string,
                    q_3: f_q_3.ok_or_else(|| serde::de::Error::missing_field("q3"))?,
                    q_3_as_string: f_q_3_as_string,
                    upper: f_upper.ok_or_else(|| serde::de::Error::missing_field("upper"))?,
                    upper_as_string: f_upper_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BoxPlotAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("lower", &self.lower)?;
        if let Some(v) = &self.lower_as_string {
            map.serialize_entry("lower_as_string", v)?;
        }
        map.serialize_entry("max", &self.max)?;
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        map.serialize_entry("min", &self.min)?;
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        map.serialize_entry("q1", &self.q_1)?;
        if let Some(v) = &self.q_1_as_string {
            map.serialize_entry("q1_as_string", v)?;
        }
        map.serialize_entry("q2", &self.q_2)?;
        if let Some(v) = &self.q_2_as_string {
            map.serialize_entry("q2_as_string", v)?;
        }
        map.serialize_entry("q3", &self.q_3)?;
        if let Some(v) = &self.q_3_as_string {
            map.serialize_entry("q3_as_string", v)?;
        }
        map.serialize_entry("upper", &self.upper)?;
        if let Some(v) = &self.upper_as_string {
            map.serialize_entry("upper_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct BucketMetricValueAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    pub keys: Vec<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for BucketMetricValueAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = BucketMetricValueAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a BucketMetricValueAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut f_keys: Option<Vec<String>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        "keys" => {
                            if f_keys.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            f_keys = Some(map.next_value::<Vec<String>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(BucketMetricValueAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    keys: f_keys.ok_or_else(|| serde::de::Error::missing_field("keys"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for BucketMetricValueAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        map.serialize_entry("keys", &self.keys)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CardinalityAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub value: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CardinalityAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CardinalityAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CardinalityAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<i64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CardinalityAggregate {
                    meta: f_meta,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CardinalityAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ChildrenAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ChildrenAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ChildrenAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ChildrenAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(ChildrenAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ChildrenAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct CompositeAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::CompositeAggregateBuckets,
    pub after_key: Option<crate::types::common_aggregations::CompositeAggregateKey>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CompositeAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CompositeAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CompositeAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::CompositeAggregateBuckets,
                > = None;
                let mut f_after_key: Option<
                    crate::types::common_aggregations::CompositeAggregateKey,
                > = None;
                let mut seen_after_key = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::CompositeAggregateBuckets>()?);
                        }
                        "after_key" => {
                            if seen_after_key {
                                return Err(serde::de::Error::duplicate_field("after_key"));
                            }
                            seen_after_key = true;
                            f_after_key =
                                map.next_value::<Option<
                                    crate::types::common_aggregations::CompositeAggregateKey,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CompositeAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    after_key: f_after_key,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CompositeAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.after_key {
            map.serialize_entry("after_key", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct CumulativeCardinalityAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub value: i64,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for CumulativeCardinalityAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = CumulativeCardinalityAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a CumulativeCardinalityAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<i64> = None;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<i64>()?);
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(CumulativeCardinalityAggregate {
                    meta: f_meta,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for CumulativeCardinalityAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct DateHistogramAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::DateHistogramAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateHistogramAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateHistogramAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateHistogramAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::DateHistogramAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::DateHistogramAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateHistogramAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateHistogramAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct DateRangeAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::DateRangeAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DateRangeAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DateRangeAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DateRangeAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::DateRangeAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::DateRangeAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DateRangeAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DateRangeAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct DerivativeAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    pub normalized_value: Option<f64>,
    pub normalized_value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DerivativeAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DerivativeAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DerivativeAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut f_normalized_value: Option<f64> = None;
                let mut seen_normalized_value = false;
                let mut f_normalized_value_as_string: Option<String> = None;
                let mut seen_normalized_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        "normalized_value" => {
                            if seen_normalized_value {
                                return Err(serde::de::Error::duplicate_field("normalized_value"));
                            }
                            seen_normalized_value = true;
                            f_normalized_value = map.next_value::<Option<f64>>()?;
                        }
                        "normalized_value_as_string" => {
                            if seen_normalized_value_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "normalized_value_as_string",
                                ));
                            }
                            seen_normalized_value_as_string = true;
                            f_normalized_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DerivativeAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    normalized_value: f_normalized_value,
                    normalized_value_as_string: f_normalized_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DerivativeAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        if let Some(v) = &self.normalized_value {
            map.serialize_entry("normalized_value", v)?;
        }
        if let Some(v) = &self.normalized_value_as_string {
            map.serialize_entry("normalized_value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of a `terms` aggregation when the field is some kind of decimal number like a float, double, or distance."]
#[derive(Debug, Clone)]
pub struct DoubleTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::DoubleTermsBucket>,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for DoubleTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = DoubleTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a DoubleTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    Vec<crate::types::common_aggregations::DoubleTermsBucket>,
                > = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<Vec<crate::types::common_aggregations::DoubleTermsBucket>>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(DoubleTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for DoubleTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct ExtendedStatsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub avg: Option<f64>,
    pub avg_as_string: Option<String>,
    pub count: i64,
    pub max: Option<f64>,
    pub max_as_string: Option<String>,
    pub min: Option<f64>,
    pub min_as_string: Option<String>,
    pub sum: f64,
    pub sum_as_string: Option<String>,
    pub std_deviation: Option<f64>,
    pub std_deviation_as_string: Option<String>,
    pub std_deviation_bounds: Option<crate::types::common_aggregations::StandardDeviationBounds>,
    pub std_deviation_bounds_as_string:
        Option<crate::types::common_aggregations::StandardDeviationBoundsAsString>,
    pub std_deviation_population: Option<f64>,
    pub std_deviation_sampling: Option<f64>,
    pub sum_of_squares: Option<f64>,
    pub sum_of_squares_as_string: Option<String>,
    pub variance: Option<f64>,
    pub variance_as_string: Option<String>,
    pub variance_population: Option<f64>,
    pub variance_population_as_string: Option<String>,
    pub variance_sampling: Option<f64>,
    pub variance_sampling_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExtendedStatsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExtendedStatsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExtendedStatsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_avg: Option<f64> = None;
                let mut seen_avg = false;
                let mut f_avg_as_string: Option<String> = None;
                let mut seen_avg_as_string = false;
                let mut f_count: Option<i64> = None;
                let mut f_max: Option<f64> = None;
                let mut seen_max = false;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut seen_min = false;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut f_sum: Option<f64> = None;
                let mut f_sum_as_string: Option<String> = None;
                let mut seen_sum_as_string = false;
                let mut f_std_deviation: Option<f64> = None;
                let mut seen_std_deviation = false;
                let mut f_std_deviation_as_string: Option<String> = None;
                let mut seen_std_deviation_as_string = false;
                let mut f_std_deviation_bounds: Option<
                    crate::types::common_aggregations::StandardDeviationBounds,
                > = None;
                let mut seen_std_deviation_bounds = false;
                let mut f_std_deviation_bounds_as_string: Option<
                    crate::types::common_aggregations::StandardDeviationBoundsAsString,
                > = None;
                let mut seen_std_deviation_bounds_as_string = false;
                let mut f_std_deviation_population: Option<f64> = None;
                let mut seen_std_deviation_population = false;
                let mut f_std_deviation_sampling: Option<f64> = None;
                let mut seen_std_deviation_sampling = false;
                let mut f_sum_of_squares: Option<f64> = None;
                let mut seen_sum_of_squares = false;
                let mut f_sum_of_squares_as_string: Option<String> = None;
                let mut seen_sum_of_squares_as_string = false;
                let mut f_variance: Option<f64> = None;
                let mut seen_variance = false;
                let mut f_variance_as_string: Option<String> = None;
                let mut seen_variance_as_string = false;
                let mut f_variance_population: Option<f64> = None;
                let mut seen_variance_population = false;
                let mut f_variance_population_as_string: Option<String> = None;
                let mut seen_variance_population_as_string = false;
                let mut f_variance_sampling: Option<f64> = None;
                let mut seen_variance_sampling = false;
                let mut f_variance_sampling_as_string: Option<String> = None;
                let mut seen_variance_sampling_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "avg" => {
                            if seen_avg {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            seen_avg = true;
                            f_avg = map.next_value::<Option<f64>>()?;
                        }
                        "avg_as_string" => {
                            if seen_avg_as_string {
                                return Err(serde::de::Error::duplicate_field("avg_as_string"));
                            }
                            seen_avg_as_string = true;
                            f_avg_as_string = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<f64>>()?;
                        }
                        "max_as_string" => {
                            if seen_max_as_string {
                                return Err(serde::de::Error::duplicate_field("max_as_string"));
                            }
                            seen_max_as_string = true;
                            f_max_as_string = map.next_value::<Option<String>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<f64>>()?;
                        }
                        "min_as_string" => {
                            if seen_min_as_string {
                                return Err(serde::de::Error::duplicate_field("min_as_string"));
                            }
                            seen_min_as_string = true;
                            f_min_as_string = map.next_value::<Option<String>>()?;
                        }
                        "sum" => {
                            if f_sum.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            f_sum = Some(map.next_value::<f64>()?);
                        }
                        "sum_as_string" => {
                            if seen_sum_as_string {
                                return Err(serde::de::Error::duplicate_field("sum_as_string"));
                            }
                            seen_sum_as_string = true;
                            f_sum_as_string = map.next_value::<Option<String>>()?;
                        }
                        "std_deviation" => {
                            if seen_std_deviation {
                                return Err(serde::de::Error::duplicate_field("std_deviation"));
                            }
                            seen_std_deviation = true;
                            f_std_deviation = map.next_value::<Option<f64>>()?;
                        }
                        "std_deviation_as_string" => {
                            if seen_std_deviation_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_as_string",
                                ));
                            }
                            seen_std_deviation_as_string = true;
                            f_std_deviation_as_string = map.next_value::<Option<String>>()?;
                        }
                        "std_deviation_bounds" => {
                            if seen_std_deviation_bounds {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_bounds",
                                ));
                            }
                            seen_std_deviation_bounds = true;
                            f_std_deviation_bounds = map.next_value::<Option<
                                crate::types::common_aggregations::StandardDeviationBounds,
                            >>()?;
                        }
                        "std_deviation_bounds_as_string" => {
                            if seen_std_deviation_bounds_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_bounds_as_string",
                                ));
                            }
                            seen_std_deviation_bounds_as_string = true;
                            f_std_deviation_bounds_as_string = map.next_value::<Option<
                                crate::types::common_aggregations::StandardDeviationBoundsAsString,
                            >>(
                            )?;
                        }
                        "std_deviation_population" => {
                            if seen_std_deviation_population {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_population",
                                ));
                            }
                            seen_std_deviation_population = true;
                            f_std_deviation_population = map.next_value::<Option<f64>>()?;
                        }
                        "std_deviation_sampling" => {
                            if seen_std_deviation_sampling {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_sampling",
                                ));
                            }
                            seen_std_deviation_sampling = true;
                            f_std_deviation_sampling = map.next_value::<Option<f64>>()?;
                        }
                        "sum_of_squares" => {
                            if seen_sum_of_squares {
                                return Err(serde::de::Error::duplicate_field("sum_of_squares"));
                            }
                            seen_sum_of_squares = true;
                            f_sum_of_squares = map.next_value::<Option<f64>>()?;
                        }
                        "sum_of_squares_as_string" => {
                            if seen_sum_of_squares_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_of_squares_as_string",
                                ));
                            }
                            seen_sum_of_squares_as_string = true;
                            f_sum_of_squares_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance" => {
                            if seen_variance {
                                return Err(serde::de::Error::duplicate_field("variance"));
                            }
                            seen_variance = true;
                            f_variance = map.next_value::<Option<f64>>()?;
                        }
                        "variance_as_string" => {
                            if seen_variance_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_as_string",
                                ));
                            }
                            seen_variance_as_string = true;
                            f_variance_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance_population" => {
                            if seen_variance_population {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_population",
                                ));
                            }
                            seen_variance_population = true;
                            f_variance_population = map.next_value::<Option<f64>>()?;
                        }
                        "variance_population_as_string" => {
                            if seen_variance_population_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_population_as_string",
                                ));
                            }
                            seen_variance_population_as_string = true;
                            f_variance_population_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance_sampling" => {
                            if seen_variance_sampling {
                                return Err(serde::de::Error::duplicate_field("variance_sampling"));
                            }
                            seen_variance_sampling = true;
                            f_variance_sampling = map.next_value::<Option<f64>>()?;
                        }
                        "variance_sampling_as_string" => {
                            if seen_variance_sampling_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_sampling_as_string",
                                ));
                            }
                            seen_variance_sampling_as_string = true;
                            f_variance_sampling_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExtendedStatsAggregate {
                    meta: f_meta,
                    avg: f_avg,
                    avg_as_string: f_avg_as_string,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    max: f_max,
                    max_as_string: f_max_as_string,
                    min: f_min,
                    min_as_string: f_min_as_string,
                    sum: f_sum.ok_or_else(|| serde::de::Error::missing_field("sum"))?,
                    sum_as_string: f_sum_as_string,
                    std_deviation: f_std_deviation,
                    std_deviation_as_string: f_std_deviation_as_string,
                    std_deviation_bounds: f_std_deviation_bounds,
                    std_deviation_bounds_as_string: f_std_deviation_bounds_as_string,
                    std_deviation_population: f_std_deviation_population,
                    std_deviation_sampling: f_std_deviation_sampling,
                    sum_of_squares: f_sum_of_squares,
                    sum_of_squares_as_string: f_sum_of_squares_as_string,
                    variance: f_variance,
                    variance_as_string: f_variance_as_string,
                    variance_population: f_variance_population,
                    variance_population_as_string: f_variance_population_as_string,
                    variance_sampling: f_variance_sampling,
                    variance_sampling_as_string: f_variance_sampling_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExtendedStatsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.avg {
            map.serialize_entry("avg", v)?;
        }
        if let Some(v) = &self.avg_as_string {
            map.serialize_entry("avg_as_string", v)?;
        }
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        map.serialize_entry("sum", &self.sum)?;
        if let Some(v) = &self.sum_as_string {
            map.serialize_entry("sum_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation {
            map.serialize_entry("std_deviation", v)?;
        }
        if let Some(v) = &self.std_deviation_as_string {
            map.serialize_entry("std_deviation_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation_bounds {
            map.serialize_entry("std_deviation_bounds", v)?;
        }
        if let Some(v) = &self.std_deviation_bounds_as_string {
            map.serialize_entry("std_deviation_bounds_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation_population {
            map.serialize_entry("std_deviation_population", v)?;
        }
        if let Some(v) = &self.std_deviation_sampling {
            map.serialize_entry("std_deviation_sampling", v)?;
        }
        if let Some(v) = &self.sum_of_squares {
            map.serialize_entry("sum_of_squares", v)?;
        }
        if let Some(v) = &self.sum_of_squares_as_string {
            map.serialize_entry("sum_of_squares_as_string", v)?;
        }
        if let Some(v) = &self.variance {
            map.serialize_entry("variance", v)?;
        }
        if let Some(v) = &self.variance_as_string {
            map.serialize_entry("variance_as_string", v)?;
        }
        if let Some(v) = &self.variance_population {
            map.serialize_entry("variance_population", v)?;
        }
        if let Some(v) = &self.variance_population_as_string {
            map.serialize_entry("variance_population_as_string", v)?;
        }
        if let Some(v) = &self.variance_sampling {
            map.serialize_entry("variance_sampling", v)?;
        }
        if let Some(v) = &self.variance_sampling_as_string {
            map.serialize_entry("variance_sampling_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ExtendedStatsBucketAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub avg: Option<f64>,
    pub avg_as_string: Option<String>,
    pub count: i64,
    pub max: Option<f64>,
    pub max_as_string: Option<String>,
    pub min: Option<f64>,
    pub min_as_string: Option<String>,
    pub sum: f64,
    pub sum_as_string: Option<String>,
    pub std_deviation: Option<f64>,
    pub std_deviation_as_string: Option<String>,
    pub std_deviation_bounds: Option<crate::types::common_aggregations::StandardDeviationBounds>,
    pub std_deviation_bounds_as_string:
        Option<crate::types::common_aggregations::StandardDeviationBoundsAsString>,
    pub std_deviation_population: Option<f64>,
    pub std_deviation_sampling: Option<f64>,
    pub sum_of_squares: Option<f64>,
    pub sum_of_squares_as_string: Option<String>,
    pub variance: Option<f64>,
    pub variance_as_string: Option<String>,
    pub variance_population: Option<f64>,
    pub variance_population_as_string: Option<String>,
    pub variance_sampling: Option<f64>,
    pub variance_sampling_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ExtendedStatsBucketAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExtendedStatsBucketAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExtendedStatsBucketAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_avg: Option<f64> = None;
                let mut seen_avg = false;
                let mut f_avg_as_string: Option<String> = None;
                let mut seen_avg_as_string = false;
                let mut f_count: Option<i64> = None;
                let mut f_max: Option<f64> = None;
                let mut seen_max = false;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut seen_min = false;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut f_sum: Option<f64> = None;
                let mut f_sum_as_string: Option<String> = None;
                let mut seen_sum_as_string = false;
                let mut f_std_deviation: Option<f64> = None;
                let mut seen_std_deviation = false;
                let mut f_std_deviation_as_string: Option<String> = None;
                let mut seen_std_deviation_as_string = false;
                let mut f_std_deviation_bounds: Option<
                    crate::types::common_aggregations::StandardDeviationBounds,
                > = None;
                let mut seen_std_deviation_bounds = false;
                let mut f_std_deviation_bounds_as_string: Option<
                    crate::types::common_aggregations::StandardDeviationBoundsAsString,
                > = None;
                let mut seen_std_deviation_bounds_as_string = false;
                let mut f_std_deviation_population: Option<f64> = None;
                let mut seen_std_deviation_population = false;
                let mut f_std_deviation_sampling: Option<f64> = None;
                let mut seen_std_deviation_sampling = false;
                let mut f_sum_of_squares: Option<f64> = None;
                let mut seen_sum_of_squares = false;
                let mut f_sum_of_squares_as_string: Option<String> = None;
                let mut seen_sum_of_squares_as_string = false;
                let mut f_variance: Option<f64> = None;
                let mut seen_variance = false;
                let mut f_variance_as_string: Option<String> = None;
                let mut seen_variance_as_string = false;
                let mut f_variance_population: Option<f64> = None;
                let mut seen_variance_population = false;
                let mut f_variance_population_as_string: Option<String> = None;
                let mut seen_variance_population_as_string = false;
                let mut f_variance_sampling: Option<f64> = None;
                let mut seen_variance_sampling = false;
                let mut f_variance_sampling_as_string: Option<String> = None;
                let mut seen_variance_sampling_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "avg" => {
                            if seen_avg {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            seen_avg = true;
                            f_avg = map.next_value::<Option<f64>>()?;
                        }
                        "avg_as_string" => {
                            if seen_avg_as_string {
                                return Err(serde::de::Error::duplicate_field("avg_as_string"));
                            }
                            seen_avg_as_string = true;
                            f_avg_as_string = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<f64>>()?;
                        }
                        "max_as_string" => {
                            if seen_max_as_string {
                                return Err(serde::de::Error::duplicate_field("max_as_string"));
                            }
                            seen_max_as_string = true;
                            f_max_as_string = map.next_value::<Option<String>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<f64>>()?;
                        }
                        "min_as_string" => {
                            if seen_min_as_string {
                                return Err(serde::de::Error::duplicate_field("min_as_string"));
                            }
                            seen_min_as_string = true;
                            f_min_as_string = map.next_value::<Option<String>>()?;
                        }
                        "sum" => {
                            if f_sum.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            f_sum = Some(map.next_value::<f64>()?);
                        }
                        "sum_as_string" => {
                            if seen_sum_as_string {
                                return Err(serde::de::Error::duplicate_field("sum_as_string"));
                            }
                            seen_sum_as_string = true;
                            f_sum_as_string = map.next_value::<Option<String>>()?;
                        }
                        "std_deviation" => {
                            if seen_std_deviation {
                                return Err(serde::de::Error::duplicate_field("std_deviation"));
                            }
                            seen_std_deviation = true;
                            f_std_deviation = map.next_value::<Option<f64>>()?;
                        }
                        "std_deviation_as_string" => {
                            if seen_std_deviation_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_as_string",
                                ));
                            }
                            seen_std_deviation_as_string = true;
                            f_std_deviation_as_string = map.next_value::<Option<String>>()?;
                        }
                        "std_deviation_bounds" => {
                            if seen_std_deviation_bounds {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_bounds",
                                ));
                            }
                            seen_std_deviation_bounds = true;
                            f_std_deviation_bounds = map.next_value::<Option<
                                crate::types::common_aggregations::StandardDeviationBounds,
                            >>()?;
                        }
                        "std_deviation_bounds_as_string" => {
                            if seen_std_deviation_bounds_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_bounds_as_string",
                                ));
                            }
                            seen_std_deviation_bounds_as_string = true;
                            f_std_deviation_bounds_as_string = map.next_value::<Option<
                                crate::types::common_aggregations::StandardDeviationBoundsAsString,
                            >>(
                            )?;
                        }
                        "std_deviation_population" => {
                            if seen_std_deviation_population {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_population",
                                ));
                            }
                            seen_std_deviation_population = true;
                            f_std_deviation_population = map.next_value::<Option<f64>>()?;
                        }
                        "std_deviation_sampling" => {
                            if seen_std_deviation_sampling {
                                return Err(serde::de::Error::duplicate_field(
                                    "std_deviation_sampling",
                                ));
                            }
                            seen_std_deviation_sampling = true;
                            f_std_deviation_sampling = map.next_value::<Option<f64>>()?;
                        }
                        "sum_of_squares" => {
                            if seen_sum_of_squares {
                                return Err(serde::de::Error::duplicate_field("sum_of_squares"));
                            }
                            seen_sum_of_squares = true;
                            f_sum_of_squares = map.next_value::<Option<f64>>()?;
                        }
                        "sum_of_squares_as_string" => {
                            if seen_sum_of_squares_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_of_squares_as_string",
                                ));
                            }
                            seen_sum_of_squares_as_string = true;
                            f_sum_of_squares_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance" => {
                            if seen_variance {
                                return Err(serde::de::Error::duplicate_field("variance"));
                            }
                            seen_variance = true;
                            f_variance = map.next_value::<Option<f64>>()?;
                        }
                        "variance_as_string" => {
                            if seen_variance_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_as_string",
                                ));
                            }
                            seen_variance_as_string = true;
                            f_variance_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance_population" => {
                            if seen_variance_population {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_population",
                                ));
                            }
                            seen_variance_population = true;
                            f_variance_population = map.next_value::<Option<f64>>()?;
                        }
                        "variance_population_as_string" => {
                            if seen_variance_population_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_population_as_string",
                                ));
                            }
                            seen_variance_population_as_string = true;
                            f_variance_population_as_string = map.next_value::<Option<String>>()?;
                        }
                        "variance_sampling" => {
                            if seen_variance_sampling {
                                return Err(serde::de::Error::duplicate_field("variance_sampling"));
                            }
                            seen_variance_sampling = true;
                            f_variance_sampling = map.next_value::<Option<f64>>()?;
                        }
                        "variance_sampling_as_string" => {
                            if seen_variance_sampling_as_string {
                                return Err(serde::de::Error::duplicate_field(
                                    "variance_sampling_as_string",
                                ));
                            }
                            seen_variance_sampling_as_string = true;
                            f_variance_sampling_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExtendedStatsBucketAggregate {
                    meta: f_meta,
                    avg: f_avg,
                    avg_as_string: f_avg_as_string,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    max: f_max,
                    max_as_string: f_max_as_string,
                    min: f_min,
                    min_as_string: f_min_as_string,
                    sum: f_sum.ok_or_else(|| serde::de::Error::missing_field("sum"))?,
                    sum_as_string: f_sum_as_string,
                    std_deviation: f_std_deviation,
                    std_deviation_as_string: f_std_deviation_as_string,
                    std_deviation_bounds: f_std_deviation_bounds,
                    std_deviation_bounds_as_string: f_std_deviation_bounds_as_string,
                    std_deviation_population: f_std_deviation_population,
                    std_deviation_sampling: f_std_deviation_sampling,
                    sum_of_squares: f_sum_of_squares,
                    sum_of_squares_as_string: f_sum_of_squares_as_string,
                    variance: f_variance,
                    variance_as_string: f_variance_as_string,
                    variance_population: f_variance_population,
                    variance_population_as_string: f_variance_population_as_string,
                    variance_sampling: f_variance_sampling,
                    variance_sampling_as_string: f_variance_sampling_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExtendedStatsBucketAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.avg {
            map.serialize_entry("avg", v)?;
        }
        if let Some(v) = &self.avg_as_string {
            map.serialize_entry("avg_as_string", v)?;
        }
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        map.serialize_entry("sum", &self.sum)?;
        if let Some(v) = &self.sum_as_string {
            map.serialize_entry("sum_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation {
            map.serialize_entry("std_deviation", v)?;
        }
        if let Some(v) = &self.std_deviation_as_string {
            map.serialize_entry("std_deviation_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation_bounds {
            map.serialize_entry("std_deviation_bounds", v)?;
        }
        if let Some(v) = &self.std_deviation_bounds_as_string {
            map.serialize_entry("std_deviation_bounds_as_string", v)?;
        }
        if let Some(v) = &self.std_deviation_population {
            map.serialize_entry("std_deviation_population", v)?;
        }
        if let Some(v) = &self.std_deviation_sampling {
            map.serialize_entry("std_deviation_sampling", v)?;
        }
        if let Some(v) = &self.sum_of_squares {
            map.serialize_entry("sum_of_squares", v)?;
        }
        if let Some(v) = &self.sum_of_squares_as_string {
            map.serialize_entry("sum_of_squares_as_string", v)?;
        }
        if let Some(v) = &self.variance {
            map.serialize_entry("variance", v)?;
        }
        if let Some(v) = &self.variance_as_string {
            map.serialize_entry("variance_as_string", v)?;
        }
        if let Some(v) = &self.variance_population {
            map.serialize_entry("variance_population", v)?;
        }
        if let Some(v) = &self.variance_population_as_string {
            map.serialize_entry("variance_population_as_string", v)?;
        }
        if let Some(v) = &self.variance_sampling {
            map.serialize_entry("variance_sampling", v)?;
        }
        if let Some(v) = &self.variance_sampling_as_string {
            map.serialize_entry("variance_sampling_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FilterAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FilterAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FilterAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FilterAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(FilterAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FilterAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct FiltersAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::FiltersAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for FiltersAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = FiltersAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a FiltersAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::FiltersAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::FiltersAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(FiltersAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for FiltersAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct GeoBoundsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub bounds: Option<crate::types::common::GeoBounds>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoBoundsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoBoundsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoBoundsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_bounds: Option<crate::types::common::GeoBounds> = None;
                let mut seen_bounds = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "bounds" => {
                            if seen_bounds {
                                return Err(serde::de::Error::duplicate_field("bounds"));
                            }
                            seen_bounds = true;
                            f_bounds =
                                map.next_value::<Option<crate::types::common::GeoBounds>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoBoundsAggregate {
                    meta: f_meta,
                    bounds: f_bounds,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoBoundsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.bounds {
            map.serialize_entry("bounds", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GeoCentroidAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub count: i64,
    pub location: Option<crate::types::common::GeoLocation>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoCentroidAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoCentroidAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoCentroidAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_count: Option<i64> = None;
                let mut f_location: Option<crate::types::common::GeoLocation> = None;
                let mut seen_location = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "location" => {
                            if seen_location {
                                return Err(serde::de::Error::duplicate_field("location"));
                            }
                            seen_location = true;
                            f_location =
                                map.next_value::<Option<crate::types::common::GeoLocation>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoCentroidAggregate {
                    meta: f_meta,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    location: f_location,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoCentroidAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.location {
            map.serialize_entry("location", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct GeoDistanceAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoDistanceAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoDistanceAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoDistanceAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoDistanceAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::GeoDistanceAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::GeoDistanceAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoDistanceAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoDistanceAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct GeoHashGridAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoHashGridAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoHashGridAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoHashGridAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoHashGridAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::GeoHashGridAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::GeoHashGridAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoHashGridAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoHashGridAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct GeoTileGridAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::GeoTileGridAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GeoTileGridAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GeoTileGridAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GeoTileGridAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::GeoTileGridAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::GeoTileGridAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(GeoTileGridAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GeoTileGridAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct GlobalAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for GlobalAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = GlobalAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a GlobalAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(GlobalAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for GlobalAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HdrPercentileRanksAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HdrPercentileRanksAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HdrPercentileRanksAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HdrPercentileRanksAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_values: Option<crate::types::common_aggregations::Percentiles> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "values" => {
                            if f_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            f_values = Some(
                                map.next_value::<crate::types::common_aggregations::Percentiles>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HdrPercentileRanksAggregate {
                    meta: f_meta,
                    values: f_values.ok_or_else(|| serde::de::Error::missing_field("values"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HdrPercentileRanksAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("values", &self.values)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HdrPercentilesAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HdrPercentilesAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HdrPercentilesAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HdrPercentilesAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_values: Option<crate::types::common_aggregations::Percentiles> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "values" => {
                            if f_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            f_values = Some(
                                map.next_value::<crate::types::common_aggregations::Percentiles>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HdrPercentilesAggregate {
                    meta: f_meta,
                    values: f_values.ok_or_else(|| serde::de::Error::missing_field("values"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HdrPercentilesAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("values", &self.values)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct HistogramAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::HistogramAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for HistogramAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = HistogramAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a HistogramAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::HistogramAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::HistogramAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(HistogramAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for HistogramAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct IpRangeAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::IpRangeAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for IpRangeAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = IpRangeAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a IpRangeAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::IpRangeAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::IpRangeAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(IpRangeAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for IpRangeAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct LongRareTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::LongRareTermsAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LongRareTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongRareTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongRareTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::LongRareTermsAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::LongRareTermsAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongRareTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongRareTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct LongTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::LongTermsBucket>,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for LongTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = LongTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a LongTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<Vec<crate::types::common_aggregations::LongTermsBucket>> =
                    None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<Vec<crate::types::common_aggregations::LongTermsBucket>>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(LongTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for LongTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct MatrixStatsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    pub fields: Option<Vec<crate::types::common_aggregations::MatrixStatsFields>>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatrixStatsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatrixStatsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatrixStatsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut f_fields: Option<
                    Vec<crate::types::common_aggregations::MatrixStatsFields>,
                > = None;
                let mut seen_fields = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "doc_count" => {
                            if f_doc_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            f_doc_count = Some(map.next_value::<i64>()?);
                        }
                        "fields" => {
                            if seen_fields {
                                return Err(serde::de::Error::duplicate_field("fields"));
                            }
                            seen_fields = true;
                            f_fields =
                                map.next_value::<Option<
                                    Vec<crate::types::common_aggregations::MatrixStatsFields>,
                                >>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatrixStatsAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    fields: f_fields,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatrixStatsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        if let Some(v) = &self.fields {
            map.serialize_entry("fields", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MatrixStatsFields {
    pub correlation: std::collections::BTreeMap<String, f64>,
    pub count: i64,
    pub covariance: std::collections::BTreeMap<String, f64>,
    pub kurtosis: f64,
    pub mean: f64,
    pub name: crate::types::common::Field,
    pub skewness: f64,
    pub variance: f64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MatrixStatsFields {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MatrixStatsFields;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MatrixStatsFields object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_correlation: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut f_count: Option<i64> = None;
                let mut f_covariance: Option<std::collections::BTreeMap<String, f64>> = None;
                let mut f_kurtosis: Option<f64> = None;
                let mut f_mean: Option<f64> = None;
                let mut f_name: Option<crate::types::common::Field> = None;
                let mut f_skewness: Option<f64> = None;
                let mut f_variance: Option<f64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "correlation" => {
                            if f_correlation.is_some() {
                                return Err(serde::de::Error::duplicate_field("correlation"));
                            }
                            f_correlation =
                                Some(map.next_value::<std::collections::BTreeMap<String, f64>>()?);
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "covariance" => {
                            if f_covariance.is_some() {
                                return Err(serde::de::Error::duplicate_field("covariance"));
                            }
                            f_covariance =
                                Some(map.next_value::<std::collections::BTreeMap<String, f64>>()?);
                        }
                        "kurtosis" => {
                            if f_kurtosis.is_some() {
                                return Err(serde::de::Error::duplicate_field("kurtosis"));
                            }
                            f_kurtosis = Some(map.next_value::<f64>()?);
                        }
                        "mean" => {
                            if f_mean.is_some() {
                                return Err(serde::de::Error::duplicate_field("mean"));
                            }
                            f_mean = Some(map.next_value::<f64>()?);
                        }
                        "name" => {
                            if f_name.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            f_name = Some(map.next_value::<crate::types::common::Field>()?);
                        }
                        "skewness" => {
                            if f_skewness.is_some() {
                                return Err(serde::de::Error::duplicate_field("skewness"));
                            }
                            f_skewness = Some(map.next_value::<f64>()?);
                        }
                        "variance" => {
                            if f_variance.is_some() {
                                return Err(serde::de::Error::duplicate_field("variance"));
                            }
                            f_variance = Some(map.next_value::<f64>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MatrixStatsFields {
                    correlation: f_correlation
                        .ok_or_else(|| serde::de::Error::missing_field("correlation"))?,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    covariance: f_covariance
                        .ok_or_else(|| serde::de::Error::missing_field("covariance"))?,
                    kurtosis: f_kurtosis
                        .ok_or_else(|| serde::de::Error::missing_field("kurtosis"))?,
                    mean: f_mean.ok_or_else(|| serde::de::Error::missing_field("mean"))?,
                    name: f_name.ok_or_else(|| serde::de::Error::missing_field("name"))?,
                    skewness: f_skewness
                        .ok_or_else(|| serde::de::Error::missing_field("skewness"))?,
                    variance: f_variance
                        .ok_or_else(|| serde::de::Error::missing_field("variance"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MatrixStatsFields {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("correlation", &self.correlation)?;
        map.serialize_entry("count", &self.count)?;
        map.serialize_entry("covariance", &self.covariance)?;
        map.serialize_entry("kurtosis", &self.kurtosis)?;
        map.serialize_entry("mean", &self.mean)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("skewness", &self.skewness)?;
        map.serialize_entry("variance", &self.variance)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MaxAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MaxAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MaxAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MaxAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MaxAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MaxAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MedianAbsoluteDeviationAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MedianAbsoluteDeviationAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MedianAbsoluteDeviationAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MedianAbsoluteDeviationAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MedianAbsoluteDeviationAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MedianAbsoluteDeviationAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MinAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MinAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MinAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MinAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MinAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MinAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MissingAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MissingAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MissingAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MissingAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(MissingAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MissingAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct MultiTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::MultiTermsAggregateBuckets,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for MultiTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = MultiTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a MultiTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::MultiTermsAggregateBuckets,
                > = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::MultiTermsAggregateBuckets>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(MultiTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for MultiTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct NestedAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for NestedAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = NestedAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a NestedAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(NestedAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for NestedAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ParentAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ParentAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ParentAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ParentAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(ParentAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ParentAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct PercentilesBucketAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for PercentilesBucketAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = PercentilesBucketAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a PercentilesBucketAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_values: Option<crate::types::common_aggregations::Percentiles> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "values" => {
                            if f_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            f_values = Some(
                                map.next_value::<crate::types::common_aggregations::Percentiles>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(PercentilesBucketAggregate {
                    meta: f_meta,
                    values: f_values.ok_or_else(|| serde::de::Error::missing_field("values"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for PercentilesBucketAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("values", &self.values)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct RangeAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::RangeAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RangeAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RangeAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RangeAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::RangeAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::RangeAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RangeAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RangeAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct RateAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub value: f64,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for RateAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = RateAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a RateAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<f64>()?);
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(RateAggregate {
                    meta: f_meta,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for RateAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ReverseNestedAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ReverseNestedAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ReverseNestedAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ReverseNestedAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(ReverseNestedAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ReverseNestedAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SamplerAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub doc_count: i64,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SamplerAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SamplerAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SamplerAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_doc_count: Option<i64> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
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
                Ok(SamplerAggregate {
                    meta: f_meta,
                    doc_count: f_doc_count
                        .ok_or_else(|| serde::de::Error::missing_field("doc_count"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SamplerAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("doc_count", &self.doc_count)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct ScriptedMetricAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub value: Box<serde_json::value::RawValue>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ScriptedMetricAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ScriptedMetricAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ScriptedMetricAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<Box<serde_json::value::RawValue>> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if f_value.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            f_value = Some(map.next_value::<Box<serde_json::value::RawValue>>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ScriptedMetricAggregate {
                    meta: f_meta,
                    value: f_value.ok_or_else(|| serde::de::Error::missing_field("value"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ScriptedMetricAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("value", &self.value)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct SignificantLongTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::SignificantLongTermsAggregateBuckets,
    pub bg_count: Option<i64>,
    pub doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SignificantLongTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SignificantLongTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SignificantLongTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::SignificantLongTermsAggregateBuckets,
                > = None;
                let mut f_bg_count: Option<i64> = None;
                let mut seen_bg_count = false;
                let mut f_doc_count: Option<i64> = None;
                let mut seen_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::SignificantLongTermsAggregateBuckets>()?);
                        }
                        "bg_count" => {
                            if seen_bg_count {
                                return Err(serde::de::Error::duplicate_field("bg_count"));
                            }
                            seen_bg_count = true;
                            f_bg_count = map.next_value::<Option<i64>>()?;
                        }
                        "doc_count" => {
                            if seen_doc_count {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            seen_doc_count = true;
                            f_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SignificantLongTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    bg_count: f_bg_count,
                    doc_count: f_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SignificantLongTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.bg_count {
            map.serialize_entry("bg_count", v)?;
        }
        if let Some(v) = &self.doc_count {
            map.serialize_entry("doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct SignificantStringTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::SignificantStringTermsAggregateBuckets,
    pub bg_count: Option<i64>,
    pub doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SignificantStringTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SignificantStringTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SignificantStringTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::SignificantStringTermsAggregateBuckets,
                > = None;
                let mut f_bg_count: Option<i64> = None;
                let mut seen_bg_count = false;
                let mut f_doc_count: Option<i64> = None;
                let mut seen_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::SignificantStringTermsAggregateBuckets>()?);
                        }
                        "bg_count" => {
                            if seen_bg_count {
                                return Err(serde::de::Error::duplicate_field("bg_count"));
                            }
                            seen_bg_count = true;
                            f_bg_count = map.next_value::<Option<i64>>()?;
                        }
                        "doc_count" => {
                            if seen_doc_count {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            seen_doc_count = true;
                            f_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SignificantStringTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    bg_count: f_bg_count,
                    doc_count: f_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SignificantStringTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.bg_count {
            map.serialize_entry("bg_count", v)?;
        }
        if let Some(v) = &self.doc_count {
            map.serialize_entry("doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct SimpleValueAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SimpleValueAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SimpleValueAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SimpleValueAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SimpleValueAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SimpleValueAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StandardDeviationBounds {
    pub lower: Option<f64>,
    pub lower_population: Option<f64>,
    pub lower_sampling: Option<f64>,
    pub upper: Option<f64>,
    pub upper_population: Option<f64>,
    pub upper_sampling: Option<f64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StandardDeviationBounds {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StandardDeviationBounds;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StandardDeviationBounds object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lower: Option<f64> = None;
                let mut seen_lower = false;
                let mut f_lower_population: Option<f64> = None;
                let mut seen_lower_population = false;
                let mut f_lower_sampling: Option<f64> = None;
                let mut seen_lower_sampling = false;
                let mut f_upper: Option<f64> = None;
                let mut seen_upper = false;
                let mut f_upper_population: Option<f64> = None;
                let mut seen_upper_population = false;
                let mut f_upper_sampling: Option<f64> = None;
                let mut seen_upper_sampling = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lower" => {
                            if seen_lower {
                                return Err(serde::de::Error::duplicate_field("lower"));
                            }
                            seen_lower = true;
                            f_lower = map.next_value::<Option<f64>>()?;
                        }
                        "lower_population" => {
                            if seen_lower_population {
                                return Err(serde::de::Error::duplicate_field("lower_population"));
                            }
                            seen_lower_population = true;
                            f_lower_population = map.next_value::<Option<f64>>()?;
                        }
                        "lower_sampling" => {
                            if seen_lower_sampling {
                                return Err(serde::de::Error::duplicate_field("lower_sampling"));
                            }
                            seen_lower_sampling = true;
                            f_lower_sampling = map.next_value::<Option<f64>>()?;
                        }
                        "upper" => {
                            if seen_upper {
                                return Err(serde::de::Error::duplicate_field("upper"));
                            }
                            seen_upper = true;
                            f_upper = map.next_value::<Option<f64>>()?;
                        }
                        "upper_population" => {
                            if seen_upper_population {
                                return Err(serde::de::Error::duplicate_field("upper_population"));
                            }
                            seen_upper_population = true;
                            f_upper_population = map.next_value::<Option<f64>>()?;
                        }
                        "upper_sampling" => {
                            if seen_upper_sampling {
                                return Err(serde::de::Error::duplicate_field("upper_sampling"));
                            }
                            seen_upper_sampling = true;
                            f_upper_sampling = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StandardDeviationBounds {
                    lower: f_lower,
                    lower_population: f_lower_population,
                    lower_sampling: f_lower_sampling,
                    upper: f_upper,
                    upper_population: f_upper_population,
                    upper_sampling: f_upper_sampling,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StandardDeviationBounds {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.lower {
            map.serialize_entry("lower", v)?;
        }
        if let Some(v) = &self.lower_population {
            map.serialize_entry("lower_population", v)?;
        }
        if let Some(v) = &self.lower_sampling {
            map.serialize_entry("lower_sampling", v)?;
        }
        if let Some(v) = &self.upper {
            map.serialize_entry("upper", v)?;
        }
        if let Some(v) = &self.upper_population {
            map.serialize_entry("upper_population", v)?;
        }
        if let Some(v) = &self.upper_sampling {
            map.serialize_entry("upper_sampling", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StandardDeviationBoundsAsString {
    pub lower: String,
    pub lower_population: String,
    pub lower_sampling: String,
    pub upper: String,
    pub upper_population: String,
    pub upper_sampling: String,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StandardDeviationBoundsAsString {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StandardDeviationBoundsAsString;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StandardDeviationBoundsAsString object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_lower: Option<String> = None;
                let mut f_lower_population: Option<String> = None;
                let mut f_lower_sampling: Option<String> = None;
                let mut f_upper: Option<String> = None;
                let mut f_upper_population: Option<String> = None;
                let mut f_upper_sampling: Option<String> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "lower" => {
                            if f_lower.is_some() {
                                return Err(serde::de::Error::duplicate_field("lower"));
                            }
                            f_lower = Some(map.next_value::<String>()?);
                        }
                        "lower_population" => {
                            if f_lower_population.is_some() {
                                return Err(serde::de::Error::duplicate_field("lower_population"));
                            }
                            f_lower_population = Some(map.next_value::<String>()?);
                        }
                        "lower_sampling" => {
                            if f_lower_sampling.is_some() {
                                return Err(serde::de::Error::duplicate_field("lower_sampling"));
                            }
                            f_lower_sampling = Some(map.next_value::<String>()?);
                        }
                        "upper" => {
                            if f_upper.is_some() {
                                return Err(serde::de::Error::duplicate_field("upper"));
                            }
                            f_upper = Some(map.next_value::<String>()?);
                        }
                        "upper_population" => {
                            if f_upper_population.is_some() {
                                return Err(serde::de::Error::duplicate_field("upper_population"));
                            }
                            f_upper_population = Some(map.next_value::<String>()?);
                        }
                        "upper_sampling" => {
                            if f_upper_sampling.is_some() {
                                return Err(serde::de::Error::duplicate_field("upper_sampling"));
                            }
                            f_upper_sampling = Some(map.next_value::<String>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StandardDeviationBoundsAsString {
                    lower: f_lower.ok_or_else(|| serde::de::Error::missing_field("lower"))?,
                    lower_population: f_lower_population
                        .ok_or_else(|| serde::de::Error::missing_field("lower_population"))?,
                    lower_sampling: f_lower_sampling
                        .ok_or_else(|| serde::de::Error::missing_field("lower_sampling"))?,
                    upper: f_upper.ok_or_else(|| serde::de::Error::missing_field("upper"))?,
                    upper_population: f_upper_population
                        .ok_or_else(|| serde::de::Error::missing_field("upper_population"))?,
                    upper_sampling: f_upper_sampling
                        .ok_or_else(|| serde::de::Error::missing_field("upper_sampling"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StandardDeviationBoundsAsString {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("lower", &self.lower)?;
        map.serialize_entry("lower_population", &self.lower_population)?;
        map.serialize_entry("lower_sampling", &self.lower_sampling)?;
        map.serialize_entry("upper", &self.upper)?;
        map.serialize_entry("upper_population", &self.upper_population)?;
        map.serialize_entry("upper_sampling", &self.upper_sampling)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StatsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub avg: Option<f64>,
    pub avg_as_string: Option<String>,
    pub count: i64,
    pub max: Option<f64>,
    pub max_as_string: Option<String>,
    pub min: Option<f64>,
    pub min_as_string: Option<String>,
    pub sum: f64,
    pub sum_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StatsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StatsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StatsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_avg: Option<f64> = None;
                let mut seen_avg = false;
                let mut f_avg_as_string: Option<String> = None;
                let mut seen_avg_as_string = false;
                let mut f_count: Option<i64> = None;
                let mut f_max: Option<f64> = None;
                let mut seen_max = false;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut seen_min = false;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut f_sum: Option<f64> = None;
                let mut f_sum_as_string: Option<String> = None;
                let mut seen_sum_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "avg" => {
                            if seen_avg {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            seen_avg = true;
                            f_avg = map.next_value::<Option<f64>>()?;
                        }
                        "avg_as_string" => {
                            if seen_avg_as_string {
                                return Err(serde::de::Error::duplicate_field("avg_as_string"));
                            }
                            seen_avg_as_string = true;
                            f_avg_as_string = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<f64>>()?;
                        }
                        "max_as_string" => {
                            if seen_max_as_string {
                                return Err(serde::de::Error::duplicate_field("max_as_string"));
                            }
                            seen_max_as_string = true;
                            f_max_as_string = map.next_value::<Option<String>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<f64>>()?;
                        }
                        "min_as_string" => {
                            if seen_min_as_string {
                                return Err(serde::de::Error::duplicate_field("min_as_string"));
                            }
                            seen_min_as_string = true;
                            f_min_as_string = map.next_value::<Option<String>>()?;
                        }
                        "sum" => {
                            if f_sum.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            f_sum = Some(map.next_value::<f64>()?);
                        }
                        "sum_as_string" => {
                            if seen_sum_as_string {
                                return Err(serde::de::Error::duplicate_field("sum_as_string"));
                            }
                            seen_sum_as_string = true;
                            f_sum_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsAggregate {
                    meta: f_meta,
                    avg: f_avg,
                    avg_as_string: f_avg_as_string,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    max: f_max,
                    max_as_string: f_max_as_string,
                    min: f_min,
                    min_as_string: f_min_as_string,
                    sum: f_sum.ok_or_else(|| serde::de::Error::missing_field("sum"))?,
                    sum_as_string: f_sum_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StatsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.avg {
            map.serialize_entry("avg", v)?;
        }
        if let Some(v) = &self.avg_as_string {
            map.serialize_entry("avg_as_string", v)?;
        }
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        map.serialize_entry("sum", &self.sum)?;
        if let Some(v) = &self.sum_as_string {
            map.serialize_entry("sum_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct StatsBucketAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub avg: Option<f64>,
    pub avg_as_string: Option<String>,
    pub count: i64,
    pub max: Option<f64>,
    pub max_as_string: Option<String>,
    pub min: Option<f64>,
    pub min_as_string: Option<String>,
    pub sum: f64,
    pub sum_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StatsBucketAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StatsBucketAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StatsBucketAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_avg: Option<f64> = None;
                let mut seen_avg = false;
                let mut f_avg_as_string: Option<String> = None;
                let mut seen_avg_as_string = false;
                let mut f_count: Option<i64> = None;
                let mut f_max: Option<f64> = None;
                let mut seen_max = false;
                let mut f_max_as_string: Option<String> = None;
                let mut seen_max_as_string = false;
                let mut f_min: Option<f64> = None;
                let mut seen_min = false;
                let mut f_min_as_string: Option<String> = None;
                let mut seen_min_as_string = false;
                let mut f_sum: Option<f64> = None;
                let mut f_sum_as_string: Option<String> = None;
                let mut seen_sum_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "avg" => {
                            if seen_avg {
                                return Err(serde::de::Error::duplicate_field("avg"));
                            }
                            seen_avg = true;
                            f_avg = map.next_value::<Option<f64>>()?;
                        }
                        "avg_as_string" => {
                            if seen_avg_as_string {
                                return Err(serde::de::Error::duplicate_field("avg_as_string"));
                            }
                            seen_avg_as_string = true;
                            f_avg_as_string = map.next_value::<Option<String>>()?;
                        }
                        "count" => {
                            if f_count.is_some() {
                                return Err(serde::de::Error::duplicate_field("count"));
                            }
                            f_count = Some(map.next_value::<i64>()?);
                        }
                        "max" => {
                            if seen_max {
                                return Err(serde::de::Error::duplicate_field("max"));
                            }
                            seen_max = true;
                            f_max = map.next_value::<Option<f64>>()?;
                        }
                        "max_as_string" => {
                            if seen_max_as_string {
                                return Err(serde::de::Error::duplicate_field("max_as_string"));
                            }
                            seen_max_as_string = true;
                            f_max_as_string = map.next_value::<Option<String>>()?;
                        }
                        "min" => {
                            if seen_min {
                                return Err(serde::de::Error::duplicate_field("min"));
                            }
                            seen_min = true;
                            f_min = map.next_value::<Option<f64>>()?;
                        }
                        "min_as_string" => {
                            if seen_min_as_string {
                                return Err(serde::de::Error::duplicate_field("min_as_string"));
                            }
                            seen_min_as_string = true;
                            f_min_as_string = map.next_value::<Option<String>>()?;
                        }
                        "sum" => {
                            if f_sum.is_some() {
                                return Err(serde::de::Error::duplicate_field("sum"));
                            }
                            f_sum = Some(map.next_value::<f64>()?);
                        }
                        "sum_as_string" => {
                            if seen_sum_as_string {
                                return Err(serde::de::Error::duplicate_field("sum_as_string"));
                            }
                            seen_sum_as_string = true;
                            f_sum_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StatsBucketAggregate {
                    meta: f_meta,
                    avg: f_avg,
                    avg_as_string: f_avg_as_string,
                    count: f_count.ok_or_else(|| serde::de::Error::missing_field("count"))?,
                    max: f_max,
                    max_as_string: f_max_as_string,
                    min: f_min,
                    min_as_string: f_min_as_string,
                    sum: f_sum.ok_or_else(|| serde::de::Error::missing_field("sum"))?,
                    sum_as_string: f_sum_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StatsBucketAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.avg {
            map.serialize_entry("avg", v)?;
        }
        if let Some(v) = &self.avg_as_string {
            map.serialize_entry("avg_as_string", v)?;
        }
        map.serialize_entry("count", &self.count)?;
        if let Some(v) = &self.max {
            map.serialize_entry("max", v)?;
        }
        if let Some(v) = &self.max_as_string {
            map.serialize_entry("max_as_string", v)?;
        }
        if let Some(v) = &self.min {
            map.serialize_entry("min", v)?;
        }
        if let Some(v) = &self.min_as_string {
            map.serialize_entry("min_as_string", v)?;
        }
        map.serialize_entry("sum", &self.sum)?;
        if let Some(v) = &self.sum_as_string {
            map.serialize_entry("sum_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of the `rare_terms` aggregation when the field is a string."]
#[derive(Debug, Clone)]
pub struct StringRareTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::StringRareTermsAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StringRareTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StringRareTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringRareTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::StringRareTermsAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::StringRareTermsAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StringRareTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StringRareTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct StringTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::StringTermsBucket>,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for StringTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = StringTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a StringTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    Vec<crate::types::common_aggregations::StringTermsBucket>,
                > = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<Vec<crate::types::common_aggregations::StringTermsBucket>>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(StringTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for StringTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct SumAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for SumAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = SumAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a SumAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(SumAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for SumAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TDigestPercentileRanksAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TDigestPercentileRanksAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TDigestPercentileRanksAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TDigestPercentileRanksAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_values: Option<crate::types::common_aggregations::Percentiles> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "values" => {
                            if f_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            f_values = Some(
                                map.next_value::<crate::types::common_aggregations::Percentiles>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TDigestPercentileRanksAggregate {
                    meta: f_meta,
                    values: f_values.ok_or_else(|| serde::de::Error::missing_field("values"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TDigestPercentileRanksAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("values", &self.values)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TDigestPercentilesAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub values: crate::types::common_aggregations::Percentiles,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TDigestPercentilesAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TDigestPercentilesAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TDigestPercentilesAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_values: Option<crate::types::common_aggregations::Percentiles> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "values" => {
                            if f_values.is_some() {
                                return Err(serde::de::Error::duplicate_field("values"));
                            }
                            f_values = Some(
                                map.next_value::<crate::types::common_aggregations::Percentiles>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TDigestPercentilesAggregate {
                    meta: f_meta,
                    values: f_values.ok_or_else(|| serde::de::Error::missing_field("values"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TDigestPercentilesAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("values", &self.values)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TTestAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TTestAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TTestAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TTestAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TTestAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TTestAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct TopHitsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub hits: crate::types::search::HitsMetadataJsonValue,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for TopHitsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = TopHitsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a TopHitsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_hits: Option<crate::types::search::HitsMetadataJsonValue> = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "hits" => {
                            if f_hits.is_some() {
                                return Err(serde::de::Error::duplicate_field("hits"));
                            }
                            f_hits = Some(
                                map.next_value::<crate::types::search::HitsMetadataJsonValue>()?,
                            );
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(TopHitsAggregate {
                    meta: f_meta,
                    hits: f_hits.ok_or_else(|| serde::de::Error::missing_field("hits"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for TopHitsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("hits", &self.hits)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of a `rare_terms` aggregation when the field is unmapped. `buckets` is always empty."]
#[derive(Debug, Clone)]
pub struct UnmappedRareTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::UnmappedRareTermsAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnmappedRareTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnmappedRareTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnmappedRareTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::UnmappedRareTermsAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::UnmappedRareTermsAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnmappedRareTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnmappedRareTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct UnmappedSignificantTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::UnmappedSignificantTermsAggregateBuckets,
    pub bg_count: Option<i64>,
    pub doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnmappedSignificantTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnmappedSignificantTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnmappedSignificantTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::UnmappedSignificantTermsAggregateBuckets,
                > = None;
                let mut f_bg_count: Option<i64> = None;
                let mut seen_bg_count = false;
                let mut f_doc_count: Option<i64> = None;
                let mut seen_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::UnmappedSignificantTermsAggregateBuckets>()?);
                        }
                        "bg_count" => {
                            if seen_bg_count {
                                return Err(serde::de::Error::duplicate_field("bg_count"));
                            }
                            seen_bg_count = true;
                            f_bg_count = map.next_value::<Option<i64>>()?;
                        }
                        "doc_count" => {
                            if seen_doc_count {
                                return Err(serde::de::Error::duplicate_field("doc_count"));
                            }
                            seen_doc_count = true;
                            f_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnmappedSignificantTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    bg_count: f_bg_count,
                    doc_count: f_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnmappedSignificantTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.bg_count {
            map.serialize_entry("bg_count", v)?;
        }
        if let Some(v) = &self.doc_count {
            map.serialize_entry("doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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
#[derive(Debug, Clone)]
pub struct UnmappedTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common::Void>,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnmappedTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnmappedTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnmappedTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<Vec<crate::types::common::Void>> = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<Vec<crate::types::common::Void>>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnmappedTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnmappedTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "Result of a `terms` aggregation when the field is an `unsigned_long` (64-bit unsigned integer). OpenSearch stores `unsigned_long` values as Long or BigInteger internally. In JSON responses, bucket keys are represented as numbers, which may lose precision for values larger than 2^53--1 due to IEEE 754 double precision limitations. In protocol buffer representations, keys are manually defined as uint64 to preserve the full 64-bit unsigned range. Use `key_as_string` in buckets for exact string representation of large unsigned long values."]
#[derive(Debug, Clone)]
pub struct UnsignedLongTermsAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: Vec<crate::types::common_aggregations::UnsignedLongTermsBucket>,
    pub doc_count_error_upper_bound: Option<i64>,
    pub sum_other_doc_count: Option<i64>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for UnsignedLongTermsAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = UnsignedLongTermsAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a UnsignedLongTermsAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    Vec<crate::types::common_aggregations::UnsignedLongTermsBucket>,
                > = None;
                let mut f_doc_count_error_upper_bound: Option<i64> = None;
                let mut seen_doc_count_error_upper_bound = false;
                let mut f_sum_other_doc_count: Option<i64> = None;
                let mut seen_sum_other_doc_count = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets =
                                Some(map.next_value::<Vec<
                                    crate::types::common_aggregations::UnsignedLongTermsBucket,
                                >>()?);
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
                        "sum_other_doc_count" => {
                            if seen_sum_other_doc_count {
                                return Err(serde::de::Error::duplicate_field(
                                    "sum_other_doc_count",
                                ));
                            }
                            seen_sum_other_doc_count = true;
                            f_sum_other_doc_count = map.next_value::<Option<i64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(UnsignedLongTermsAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    doc_count_error_upper_bound: f_doc_count_error_upper_bound,
                    sum_other_doc_count: f_sum_other_doc_count,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for UnsignedLongTermsAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        if let Some(v) = &self.doc_count_error_upper_bound {
            map.serialize_entry("doc_count_error_upper_bound", v)?;
        }
        if let Some(v) = &self.sum_other_doc_count {
            map.serialize_entry("sum_other_doc_count", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct ValueCountAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for ValueCountAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ValueCountAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ValueCountAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ValueCountAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ValueCountAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone)]
pub struct VariableWidthHistogramAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    pub buckets: crate::types::common_aggregations::VariableWidthHistogramAggregateBuckets,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for VariableWidthHistogramAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = VariableWidthHistogramAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a VariableWidthHistogramAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_buckets: Option<
                    crate::types::common_aggregations::VariableWidthHistogramAggregateBuckets,
                > = None;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "buckets" => {
                            if f_buckets.is_some() {
                                return Err(serde::de::Error::duplicate_field("buckets"));
                            }
                            f_buckets = Some(map.next_value::<crate::types::common_aggregations::VariableWidthHistogramAggregateBuckets>()?);
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(VariableWidthHistogramAggregate {
                    meta: f_meta,
                    buckets: f_buckets.ok_or_else(|| serde::de::Error::missing_field("buckets"))?,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for VariableWidthHistogramAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        map.serialize_entry("buckets", &self.buckets)?;
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
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

#[derive(Debug, Clone)]
pub struct WeightedAvgAggregate {
    pub meta: Option<crate::types::common::Metadata>,
    #[doc = "The metric value. A missing value generally means that there was no data to aggregate, unless specified otherwise."]
    pub value: Option<f64>,
    pub value_as_string: Option<String>,
    #[doc = "Keys of the JSON object that the specification does not define, kept as raw JSON so that nothing the server sends is lost: fields added by plugins or newer OpenSearch versions, or missing from the specification. Serialized back unchanged."]
    pub unknown_fields: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}
impl<'de> serde::Deserialize<'de> for WeightedAvgAggregate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = WeightedAvgAggregate;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a WeightedAvgAggregate object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_meta: Option<crate::types::common::Metadata> = None;
                let mut seen_meta = false;
                let mut f_value: Option<f64> = None;
                let mut seen_value = false;
                let mut f_value_as_string: Option<String> = None;
                let mut seen_value_as_string = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    Box<serde_json::value::RawValue>,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "meta" => {
                            if seen_meta {
                                return Err(serde::de::Error::duplicate_field("meta"));
                            }
                            seen_meta = true;
                            f_meta = map.next_value::<Option<crate::types::common::Metadata>>()?;
                        }
                        "value" => {
                            if seen_value {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            seen_value = true;
                            f_value = map.next_value::<Option<f64>>()?;
                        }
                        "value_as_string" => {
                            if seen_value_as_string {
                                return Err(serde::de::Error::duplicate_field("value_as_string"));
                            }
                            seen_value_as_string = true;
                            f_value_as_string = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(WeightedAvgAggregate {
                    meta: f_meta,
                    value: f_value,
                    value_as_string: f_value_as_string,
                    unknown_fields: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for WeightedAvgAggregate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.meta {
            map.serialize_entry("meta", v)?;
        }
        if let Some(v) = &self.value {
            map.serialize_entry("value", v)?;
        }
        if let Some(v) = &self.value_as_string {
            map.serialize_entry("value_as_string", v)?;
        }
        for (k, v) in &self.unknown_fields {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}
