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

//! Response types of the `search_pipeline_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgenticContextResponseProcessor {
    #[doc = "Whether to include the agent's execution step summary in the response. Available for conversational agents only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_steps_summary: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Whether to include the generated DSL query in the response. Available for both conversational and flow agents."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dsl_query: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgenticQueryTranslatorRequestProcessor {
    #[doc = "The ID of the agent to use for query translation."]
    pub agent_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CollapseResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FilterQueryRequestProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MLOpenSearchReranker {
    pub model_id: String,
}

pub type NeuralFieldMap = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NeuralQueryEnricherRequestProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_model_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub neural_field_default_id: Option<crate::types::search_pipeline_common::NeuralFieldMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NormalizationPhaseResultsProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub combination: Option<crate::types::search_pipeline_common::ScoreCombination>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub normalization: Option<crate::types::search_pipeline_common::ScoreNormalization>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct OversampleRequestProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    pub sample_factor: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersonalizeSearchRankingResponseProcessor {
    pub campaign_arn: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iam_role_arn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id_field: Option<String>,
    pub recipe: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub weight: f64,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PhaseResultsProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl PhaseResultsProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `normalization_processor` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_normalization_processor(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::PhaseResultsProcessorNormalizationProcessor,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "normalization_processor")
    }
    #[doc = "Parses the value as the `score_ranker_processor` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_score_ranker_processor(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::PhaseResultsProcessorScoreRankerProcessor,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "score_ranker_processor")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhaseResultsProcessorNormalizationProcessor {
    #[serde(rename = "normalization-processor")]
    pub normalization_processor:
        crate::types::search_pipeline_common::NormalizationPhaseResultsProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhaseResultsProcessorScoreRankerProcessor {
    #[serde(rename = "score-ranker-processor")]
    pub score_ranker_processor:
        crate::types::search_pipeline_common::ScoreRankerPhaseResultsProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RenameFieldResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    pub target_field: String,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct RequestProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl RequestProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `agentic_query_translator` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_agentic_query_translator(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorAgenticQueryTranslator,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "agentic_query_translator")
    }
    #[doc = "Parses the value as the `filter_query` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_filter_query(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorFilterQuery,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "filter_query")
    }
    #[doc = "Parses the value as the `neural_query_enricher` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_neural_query_enricher(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorNeuralQueryEnricher,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "neural_query_enricher")
    }
    #[doc = "Parses the value as the `script` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_script(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorScript,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "script")
    }
    #[doc = "Parses the value as the `oversample` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_oversample(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::RequestProcessorOversample,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "oversample")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestProcessorAgenticQueryTranslator {
    pub agentic_query_translator:
        crate::types::search_pipeline_common::AgenticQueryTranslatorRequestProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestProcessorFilterQuery {
    pub filter_query: crate::types::search_pipeline_common::FilterQueryRequestProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestProcessorNeuralQueryEnricher {
    pub neural_query_enricher:
        crate::types::search_pipeline_common::NeuralQueryEnricherRequestProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestProcessorOversample {
    pub oversample: crate::types::search_pipeline_common::OversampleRequestProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RequestProcessorScript {
    pub script: crate::types::search_pipeline_common::SearchScriptRequestProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerankContext {
    pub document_fields: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RerankResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<crate::types::search_pipeline_common::RerankContext>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ml_opensearch: Option<crate::types::search_pipeline_common::MLOpenSearchReranker>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ResponseProcessor {
    raw: Box<serde_json::value::RawValue>,
}
impl ResponseProcessor {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `agentic_context` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_agentic_context(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorAgenticContext,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "agentic_context")
    }
    #[doc = "Parses the value as the `personalize_search_ranking` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_personalize_search_ranking(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorPersonalizeSearchRanking,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "personalize_search_ranking")
    }
    #[doc = "Parses the value as the `retrieval_augmented_generation` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_retrieval_augmented_generation(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRetrievalAugmentedGeneration,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "retrieval_augmented_generation")
    }
    #[doc = "Parses the value as the `rename_field` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rename_field(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRenameField,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rename_field")
    }
    #[doc = "Parses the value as the `rerank` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_rerank(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorRerank,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "rerank")
    }
    #[doc = "Parses the value as the `collapse` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_collapse(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorCollapse,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "collapse")
    }
    #[doc = "Parses the value as the `truncate_hits` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_truncate_hits(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorTruncateHits,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "truncate_hits")
    }
    #[doc = "Parses the value as the `sort` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_sort(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorSort,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "sort")
    }
    #[doc = "Parses the value as the `split` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_split(
        &self,
    ) -> std::result::Result<
        crate::types::search_pipeline_common::ResponseProcessorSplit,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "split")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorAgenticContext {
    pub agentic_context: crate::types::search_pipeline_common::AgenticContextResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorCollapse {
    pub collapse: crate::types::search_pipeline_common::CollapseResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorPersonalizeSearchRanking {
    pub personalize_search_ranking:
        crate::types::search_pipeline_common::PersonalizeSearchRankingResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorRenameField {
    pub rename_field: crate::types::search_pipeline_common::RenameFieldResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorRerank {
    pub rerank: crate::types::search_pipeline_common::RerankResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorRetrievalAugmentedGeneration {
    pub retrieval_augmented_generation:
        crate::types::search_pipeline_common::RetrievalAugmentedGenerationResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorSort {
    pub sort: crate::types::search_pipeline_common::SortResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorSplit {
    pub split: crate::types::search_pipeline_common::SplitResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResponseProcessorTruncateHits {
    pub truncate_hits: crate::types::search_pipeline_common::TruncateHitsResponseProcessor,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RetrievalAugmentedGenerationResponseProcessor {
    pub context_field_list: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub model_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_instructions: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreCombination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameters: Option<crate::types::search_pipeline_common::ScoreCombinationParameters>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technique: Option<crate::types::search_pipeline_common::ScoreCombinationTechnique>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreCombinationParameters {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreCombinationTechnique {
    #[doc = "`arithmetic_mean`"]
    ArithmeticMean,
    #[doc = "`geometric_mean`"]
    GeometricMean,
    #[doc = "`harmonic_mean`"]
    HarmonicMean,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreCombinationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreCombinationTechnique::ArithmeticMean => "arithmetic_mean",
            ScoreCombinationTechnique::GeometricMean => "geometric_mean",
            ScoreCombinationTechnique::HarmonicMean => "harmonic_mean",
            ScoreCombinationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreCombinationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreCombinationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "arithmetic_mean" => ScoreCombinationTechnique::ArithmeticMean,
            "geometric_mean" => ScoreCombinationTechnique::GeometricMean,
            "harmonic_mean" => ScoreCombinationTechnique::HarmonicMean,
            _ => ScoreCombinationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreNormalization {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub technique: Option<crate::types::search_pipeline_common::ScoreNormalizationTechnique>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreNormalizationTechnique {
    #[doc = "`l2`"]
    L2,
    #[doc = "`min_max`"]
    MinMax,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreNormalizationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreNormalizationTechnique::L2 => "l2",
            ScoreNormalizationTechnique::MinMax => "min_max",
            ScoreNormalizationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreNormalizationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreNormalizationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "l2" => ScoreNormalizationTechnique::L2,
            "min_max" => ScoreNormalizationTechnique::MinMax,
            _ => ScoreNormalizationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreRankerCombination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_constant: Option<i32>,
    pub technique: crate::types::search_pipeline_common::ScoreRankerCombinationTechnique,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ScoreRankerCombinationTechnique {
    #[doc = "`rrf`"]
    Rrf,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ScoreRankerCombinationTechnique {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ScoreRankerCombinationTechnique::Rrf => "rrf",
            ScoreRankerCombinationTechnique::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ScoreRankerCombinationTechnique {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ScoreRankerCombinationTechnique {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "rrf" => ScoreRankerCombinationTechnique::Rrf,
            _ => ScoreRankerCombinationTechnique::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ScoreRankerPhaseResultsProcessor {
    pub combination: crate::types::search_pipeline_common::ScoreRankerCombination,
}

pub type SearchPipelineMap = std::collections::BTreeMap<
    String,
    crate::types::search_pipeline_common::SearchPipelineStructure,
>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchPipelineStructure {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase_results_processors:
        Option<Vec<crate::types::search_pipeline_common::PhaseResultsProcessor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_processors: Option<Vec<crate::types::search_pipeline_common::RequestProcessor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_processors: Option<Vec<crate::types::search_pipeline_common::ResponseProcessor>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<i32>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchScriptRequestProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    pub source: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SortResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SplitResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub field: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_trailing: Option<bool>,
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_field: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TruncateHitsResponseProcessor {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context_prefix: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_failure: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_size: Option<i32>,
}
