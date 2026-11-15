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

//! Response types of the `common_analysis` specification namespace

#![allow(missing_docs)]

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Analyzer {
    raw: Box<serde_json::value::RawValue>,
}
impl Analyzer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `custom_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CustomAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "custom_analyzer")
    }
    #[doc = "Parses the value as the `fingerprint_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fingerprint_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::FingerprintAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "fingerprint_analyzer")
    }
    #[doc = "Parses the value as the `keyword_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_analyzer")
    }
    #[doc = "Parses the value as the `language_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_language_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LanguageAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "language_analyzer")
    }
    #[doc = "Parses the value as the `nori_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_analyzer")
    }
    #[doc = "Parses the value as the `pattern_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_analyzer")
    }
    #[doc = "Parses the value as the `simple_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimpleAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_analyzer")
    }
    #[doc = "Parses the value as the `standard_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_standard_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StandardAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "standard_analyzer")
    }
    #[doc = "Parses the value as the `stop_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stop_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StopAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stop_analyzer")
    }
    #[doc = "Parses the value as the `whitespace_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_whitespace_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WhitespaceAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "whitespace_analyzer")
    }
    #[doc = "Parses the value as the `icu_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_analyzer")
    }
    #[doc = "Parses the value as the `kuromoji_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_analyzer")
    }
    #[doc = "Parses the value as the `snowball_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_snowball_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SnowballAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "snowball_analyzer")
    }
    #[doc = "Parses the value as the `dutch_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_dutch_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DutchAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "dutch_analyzer")
    }
    #[doc = "Parses the value as the `smartcn_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_analyzer")
    }
    #[doc = "Parses the value as the `cjk_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cjk_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CjkAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cjk_analyzer")
    }
    #[doc = "Parses the value as the `phone_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phone_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phone_analyzer")
    }
    #[doc = "Parses the value as the `phone_search_analyzer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phone_search_analyzer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneSearchAnalyzer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phone_search_analyzer")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AsciiFoldingTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::AsciiFoldingTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AsciiFoldingTokenFilterType {
    #[doc = "`asciifolding`"]
    Asciifolding,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl AsciiFoldingTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            AsciiFoldingTokenFilterType::Asciifolding => "asciifolding",
            AsciiFoldingTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for AsciiFoldingTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AsciiFoldingTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "asciifolding" => AsciiFoldingTokenFilterType::Asciifolding,
            _ => AsciiFoldingTokenFilterType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CharFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl CharFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CharFilterDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CharFilterDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl CharFilterDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `html_strip_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_html_strip_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HtmlStripCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "html_strip_char_filter")
    }
    #[doc = "Parses the value as the `mapping_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_mapping_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::MappingCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "mapping_char_filter")
    }
    #[doc = "Parses the value as the `pattern_replace_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_replace_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternReplaceCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_replace_char_filter")
    }
    #[doc = "Parses the value as the `icu_normalization_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_normalization_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuNormalizationCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_normalization_char_filter")
    }
    #[doc = "Parses the value as the `kuromoji_iteration_mark_char_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_iteration_mark_char_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiIterationMarkCharFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_iteration_mark_char_filter")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CharGroupTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i64>,
    pub tokenize_on_chars: Vec<String>,
    pub r#type: crate::types::common_analysis::CharGroupTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CharGroupTokenizerType {
    #[doc = "`char_group`"]
    CharGroup,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CharGroupTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CharGroupTokenizerType::CharGroup => "char_group",
            CharGroupTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CharGroupTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CharGroupTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "char_group" => CharGroupTokenizerType::CharGroup,
            _ => CharGroupTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CjkAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_analysis::CjkAnalyzerType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CjkAnalyzerType {
    #[doc = "`cjk`"]
    Cjk,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CjkAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CjkAnalyzerType::Cjk => "cjk",
            CjkAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CjkAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CjkAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "cjk" => CjkAnalyzerType::Cjk,
            _ => CjkAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommonGramsTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_mode: Option<bool>,
    pub r#type: crate::types::common_analysis::CommonGramsTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommonGramsTokenFilterType {
    #[doc = "`common_grams`"]
    CommonGrams,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CommonGramsTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CommonGramsTokenFilterType::CommonGrams => "common_grams",
            CommonGramsTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CommonGramsTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CommonGramsTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "common_grams" => CommonGramsTokenFilterType::CommonGrams,
            _ => CommonGramsTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConditionTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub filter: Vec<String>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::ConditionTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConditionTokenFilterType {
    #[doc = "`condition`"]
    Condition,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ConditionTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ConditionTokenFilterType::Condition => "condition",
            ConditionTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ConditionTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ConditionTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "condition" => ConditionTokenFilterType::Condition,
            _ => ConditionTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_increment_gap: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_offset_gap: Option<i64>,
    pub tokenizer: String,
    pub r#type: crate::types::common_analysis::CustomAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomAnalyzerType {
    #[doc = "`custom`"]
    Custom,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CustomAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CustomAnalyzerType::Custom => "custom",
            CustomAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CustomAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => CustomAnalyzerType::Custom,
            _ => CustomAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CustomNormalizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub char_filter: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::CustomNormalizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CustomNormalizerType {
    #[doc = "`custom`"]
    Custom,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl CustomNormalizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            CustomNormalizerType::Custom => "custom",
            CustomNormalizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for CustomNormalizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CustomNormalizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => CustomNormalizerType::Custom,
            _ => CustomNormalizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelimitedPayloadEncoding {
    #[doc = "`float`"]
    Float,
    #[doc = "`identity`"]
    Identity,
    #[doc = "`int`"]
    Int,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DelimitedPayloadEncoding {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DelimitedPayloadEncoding::Float => "float",
            DelimitedPayloadEncoding::Identity => "identity",
            DelimitedPayloadEncoding::Int => "int",
            DelimitedPayloadEncoding::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DelimitedPayloadEncoding {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DelimitedPayloadEncoding {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "float" => DelimitedPayloadEncoding::Float,
            "identity" => DelimitedPayloadEncoding::Identity,
            "int" => DelimitedPayloadEncoding::Int,
            _ => DelimitedPayloadEncoding::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DelimitedPayloadTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delimiter: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<crate::types::common_analysis::DelimitedPayloadEncoding>,
    pub r#type: crate::types::common_analysis::DelimitedPayloadTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DelimitedPayloadTokenFilterType {
    #[doc = "`delimited_payload`"]
    DelimitedPayload,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DelimitedPayloadTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DelimitedPayloadTokenFilterType::DelimitedPayload => "delimited_payload",
            DelimitedPayloadTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DelimitedPayloadTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DelimitedPayloadTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "delimited_payload" => DelimitedPayloadTokenFilterType::DelimitedPayload,
            _ => DelimitedPayloadTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DictionaryDecompounderTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyphenation_patterns_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_subword_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_subword_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_longest_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::DictionaryDecompounderTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DictionaryDecompounderTokenFilterType {
    #[doc = "`dictionary_decompounder`"]
    DictionaryDecompounder,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DictionaryDecompounderTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DictionaryDecompounderTokenFilterType::DictionaryDecompounder => {
                "dictionary_decompounder"
            }
            DictionaryDecompounderTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DictionaryDecompounderTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DictionaryDecompounderTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dictionary_decompounder" => {
                DictionaryDecompounderTokenFilterType::DictionaryDecompounder
            }
            _ => DictionaryDecompounderTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DutchAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::DutchAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DutchAnalyzerType {
    #[doc = "`dutch`"]
    Dutch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DutchAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DutchAnalyzerType::Dutch => "dutch",
            DutchAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DutchAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DutchAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "dutch" => DutchAnalyzerType::Dutch,
            _ => DutchAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramSide {
    #[doc = "`back`"]
    Back,
    #[doc = "`front`"]
    Front,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramSide {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramSide::Back => "back",
            EdgeNGramSide::Front => "front",
            EdgeNGramSide::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramSide {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramSide {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "back" => EdgeNGramSide::Back,
            "front" => EdgeNGramSide::Front,
            _ => EdgeNGramSide::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EdgeNGramTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<crate::types::common_analysis::EdgeNGramSide>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramTokenFilterType {
    #[doc = "`edge_ngram`"]
    EdgeNgram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramTokenFilterType::EdgeNgram => "edge_ngram",
            EdgeNGramTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "edge_ngram" => EdgeNGramTokenFilterType::EdgeNgram,
            _ => EdgeNGramTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EdgeNGramTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_token_chars: Option<String>,
    pub max_gram: i64,
    pub min_gram: i64,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::EdgeNGramTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeNGramTokenizerType {
    #[doc = "`edge_ngram`"]
    EdgeNgram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EdgeNGramTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EdgeNGramTokenizerType::EdgeNgram => "edge_ngram",
            EdgeNGramTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EdgeNGramTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EdgeNGramTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "edge_ngram" => EdgeNGramTokenizerType::EdgeNgram,
            _ => EdgeNGramTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ElisionTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles_case: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub articles_path: Option<String>,
    pub r#type: crate::types::common_analysis::ElisionTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ElisionTokenFilterType {
    #[doc = "`elision`"]
    Elision,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ElisionTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ElisionTokenFilterType::Elision => "elision",
            ElisionTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ElisionTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ElisionTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "elision" => ElisionTokenFilterType::Elision,
            _ => ElisionTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FingerprintAnalyzer {
    pub max_output_size: i64,
    pub preserve_original: bool,
    pub separator: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FingerprintAnalyzerType {
    #[doc = "`fingerprint`"]
    Fingerprint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FingerprintAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FingerprintAnalyzerType::Fingerprint => "fingerprint",
            FingerprintAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FingerprintAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FingerprintAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fingerprint" => FingerprintAnalyzerType::Fingerprint,
            _ => FingerprintAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FingerprintTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_output_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub separator: Option<String>,
    pub r#type: crate::types::common_analysis::FingerprintTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FingerprintTokenFilterType {
    #[doc = "`fingerprint`"]
    Fingerprint,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FingerprintTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FingerprintTokenFilterType::Fingerprint => "fingerprint",
            FingerprintTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FingerprintTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FingerprintTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "fingerprint" => FingerprintTokenFilterType::Fingerprint,
            _ => FingerprintTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HtmlStripCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::HtmlStripCharFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HtmlStripCharFilterType {
    #[doc = "`html_strip`"]
    HtmlStrip,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HtmlStripCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HtmlStripCharFilterType::HtmlStrip => "html_strip",
            HtmlStripCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HtmlStripCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HtmlStripCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "html_strip" => HtmlStripCharFilterType::HtmlStrip,
            _ => HtmlStripCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HunspellTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dedup: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dictionary: Option<String>,
    pub locale: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub longest_only: Option<bool>,
    pub r#type: crate::types::common_analysis::HunspellTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HunspellTokenFilterType {
    #[doc = "`hunspell`"]
    Hunspell,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HunspellTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HunspellTokenFilterType::Hunspell => "hunspell",
            HunspellTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HunspellTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HunspellTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hunspell" => HunspellTokenFilterType::Hunspell,
            _ => HunspellTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HyphenationDecompounderTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hyphenation_patterns_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_subword_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_subword_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_word_size: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_longest_match: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub word_list_path: Option<String>,
    pub r#type: crate::types::common_analysis::HyphenationDecompounderTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HyphenationDecompounderTokenFilterType {
    #[doc = "`hyphenation_decompounder`"]
    HyphenationDecompounder,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HyphenationDecompounderTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HyphenationDecompounderTokenFilterType::HyphenationDecompounder => {
                "hyphenation_decompounder"
            }
            HyphenationDecompounderTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HyphenationDecompounderTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HyphenationDecompounderTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "hyphenation_decompounder" => {
                HyphenationDecompounderTokenFilterType::HyphenationDecompounder
            }
            _ => HyphenationDecompounderTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuAnalyzer {
    pub method: crate::types::common_analysis::IcuNormalizationType,
    pub mode: crate::types::common_analysis::IcuNormalizationMode,
    pub r#type: crate::types::common_analysis::IcuAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuAnalyzerType {
    #[doc = "`icu_analyzer`"]
    IcuAnalyzer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuAnalyzerType::IcuAnalyzer => "icu_analyzer",
            IcuAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_analyzer" => IcuAnalyzerType::IcuAnalyzer,
            _ => IcuAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationAlternate {
    #[doc = "`non-ignorable`"]
    NonIgnorable,
    #[doc = "`shifted`"]
    Shifted,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationAlternate {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationAlternate::NonIgnorable => "non-ignorable",
            IcuCollationAlternate::Shifted => "shifted",
            IcuCollationAlternate::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationAlternate {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationAlternate {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "non-ignorable" => IcuCollationAlternate::NonIgnorable,
            "shifted" => IcuCollationAlternate::Shifted,
            _ => IcuCollationAlternate::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationCaseFirst {
    #[doc = "`lower`"]
    Lower,
    #[doc = "`upper`"]
    Upper,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationCaseFirst {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationCaseFirst::Lower => "lower",
            IcuCollationCaseFirst::Upper => "upper",
            IcuCollationCaseFirst::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationCaseFirst {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationCaseFirst {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lower" => IcuCollationCaseFirst::Lower,
            "upper" => IcuCollationCaseFirst::Upper,
            _ => IcuCollationCaseFirst::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationDecomposition {
    #[doc = "`canonical`"]
    Canonical,
    #[doc = "`no`"]
    No,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationDecomposition {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationDecomposition::Canonical => "canonical",
            IcuCollationDecomposition::No => "no",
            IcuCollationDecomposition::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationDecomposition {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationDecomposition {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "canonical" => IcuCollationDecomposition::Canonical,
            "no" => IcuCollationDecomposition::No,
            _ => IcuCollationDecomposition::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationStrength {
    #[doc = "`identical`"]
    Identical,
    #[doc = "`primary`"]
    Primary,
    #[doc = "`quaternary`"]
    Quaternary,
    #[doc = "`secondary`"]
    Secondary,
    #[doc = "`tertiary`"]
    Tertiary,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationStrength {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationStrength::Identical => "identical",
            IcuCollationStrength::Primary => "primary",
            IcuCollationStrength::Quaternary => "quaternary",
            IcuCollationStrength::Secondary => "secondary",
            IcuCollationStrength::Tertiary => "tertiary",
            IcuCollationStrength::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationStrength {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationStrength {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "identical" => IcuCollationStrength::Identical,
            "primary" => IcuCollationStrength::Primary,
            "quaternary" => IcuCollationStrength::Quaternary,
            "secondary" => IcuCollationStrength::Secondary,
            "tertiary" => IcuCollationStrength::Tertiary,
            _ => IcuCollationStrength::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuCollationTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alternate: Option<crate::types::common_analysis::IcuCollationAlternate>,
    #[serde(rename = "caseFirst")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_first: Option<crate::types::common_analysis::IcuCollationCaseFirst>,
    #[serde(rename = "caseLevel")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub case_level: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decomposition: Option<crate::types::common_analysis::IcuCollationDecomposition>,
    #[serde(rename = "hiraganaQuaternaryMode")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hiragana_quaternary_mode: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strength: Option<crate::types::common_analysis::IcuCollationStrength>,
    pub r#type: crate::types::common_analysis::IcuCollationTokenFilterType,
    #[serde(rename = "variableTop")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variable_top: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuCollationTokenFilterType {
    #[doc = "`icu_collation`"]
    IcuCollation,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuCollationTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuCollationTokenFilterType::IcuCollation => "icu_collation",
            IcuCollationTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuCollationTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuCollationTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_collation" => IcuCollationTokenFilterType::IcuCollation,
            _ => IcuCollationTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuFoldingTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::IcuFoldingTokenFilterType,
    pub unicode_set_filter: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuFoldingTokenFilterType {
    #[doc = "`icu_folding`"]
    IcuFolding,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuFoldingTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuFoldingTokenFilterType::IcuFolding => "icu_folding",
            IcuFoldingTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuFoldingTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuFoldingTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_folding" => IcuFoldingTokenFilterType::IcuFolding,
            _ => IcuFoldingTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuNormalizationCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common_analysis::IcuNormalizationMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::types::common_analysis::IcuNormalizationType>,
    pub r#type: crate::types::common_analysis::IcuNormalizationCharFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationCharFilterType {
    #[doc = "`icu_normalizer`"]
    IcuNormalizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationCharFilterType::IcuNormalizer => "icu_normalizer",
            IcuNormalizationCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_normalizer" => IcuNormalizationCharFilterType::IcuNormalizer,
            _ => IcuNormalizationCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationMode {
    #[doc = "`compose`"]
    Compose,
    #[doc = "`decompose`"]
    Decompose,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationMode::Compose => "compose",
            IcuNormalizationMode::Decompose => "decompose",
            IcuNormalizationMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "compose" => IcuNormalizationMode::Compose,
            "decompose" => IcuNormalizationMode::Decompose,
            _ => IcuNormalizationMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuNormalizationTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub name: crate::types::common_analysis::IcuNormalizationType,
    pub r#type: crate::types::common_analysis::IcuNormalizationTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationTokenFilterType {
    #[doc = "`icu_normalizer`"]
    IcuNormalizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationTokenFilterType::IcuNormalizer => "icu_normalizer",
            IcuNormalizationTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_normalizer" => IcuNormalizationTokenFilterType::IcuNormalizer,
            _ => IcuNormalizationTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuNormalizationType {
    #[doc = "`nfc`"]
    Nfc,
    #[doc = "`nfkc`"]
    Nfkc,
    #[doc = "`nfkc_cf`"]
    NfkcCf,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuNormalizationType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuNormalizationType::Nfc => "nfc",
            IcuNormalizationType::Nfkc => "nfkc",
            IcuNormalizationType::NfkcCf => "nfkc_cf",
            IcuNormalizationType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuNormalizationType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuNormalizationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nfc" => IcuNormalizationType::Nfc,
            "nfkc" => IcuNormalizationType::Nfkc,
            "nfkc_cf" => IcuNormalizationType::NfkcCf,
            _ => IcuNormalizationType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub rule_files: String,
    pub r#type: crate::types::common_analysis::IcuTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTokenizerType {
    #[doc = "`icu_tokenizer`"]
    IcuTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTokenizerType::IcuTokenizer => "icu_tokenizer",
            IcuTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_tokenizer" => IcuTokenizerType::IcuTokenizer,
            _ => IcuTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTransformDirection {
    #[doc = "`forward`"]
    Forward,
    #[doc = "`reverse`"]
    Reverse,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTransformDirection {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTransformDirection::Forward => "forward",
            IcuTransformDirection::Reverse => "reverse",
            IcuTransformDirection::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTransformDirection {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTransformDirection {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "forward" => IcuTransformDirection::Forward,
            "reverse" => IcuTransformDirection::Reverse,
            _ => IcuTransformDirection::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IcuTransformTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dir: Option<crate::types::common_analysis::IcuTransformDirection>,
    pub id: String,
    pub r#type: crate::types::common_analysis::IcuTransformTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IcuTransformTokenFilterType {
    #[doc = "`icu_transform`"]
    IcuTransform,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl IcuTransformTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            IcuTransformTokenFilterType::IcuTransform => "icu_transform",
            IcuTransformTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for IcuTransformTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for IcuTransformTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "icu_transform" => IcuTransformTokenFilterType::IcuTransform,
            _ => IcuTransformTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KStemTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KStemTokenFilterType {
    #[doc = "`kstem`"]
    Kstem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KStemTokenFilterType::Kstem => "kstem",
            KStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kstem" => KStemTokenFilterType::Kstem,
            _ => KStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepTypesMode {
    #[doc = "`exclude`"]
    Exclude,
    #[doc = "`include`"]
    Include,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepTypesMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepTypesMode::Exclude => "exclude",
            KeepTypesMode::Include => "include",
            KeepTypesMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepTypesMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepTypesMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "exclude" => KeepTypesMode::Exclude,
            "include" => KeepTypesMode::Include,
            _ => KeepTypesMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeepTypesTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<crate::types::common_analysis::KeepTypesMode>,
    pub r#type: crate::types::common_analysis::KeepTypesTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepTypesTokenFilterType {
    #[doc = "`keep_types`"]
    KeepTypes,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepTypesTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepTypesTokenFilterType::KeepTypes => "keep_types",
            KeepTypesTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepTypesTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepTypesTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keep_types" => KeepTypesTokenFilterType::KeepTypes,
            _ => KeepTypesTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeepWordsTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keep_words_path: Option<String>,
    pub r#type: crate::types::common_analysis::KeepWordsTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeepWordsTokenFilterType {
    #[doc = "`keep`"]
    Keep,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeepWordsTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeepWordsTokenFilterType::Keep => "keep",
            KeepWordsTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeepWordsTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeepWordsTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keep" => KeepWordsTokenFilterType::Keep,
            _ => KeepWordsTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordAnalyzer {
    pub r#type: crate::types::common_analysis::KeywordAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordAnalyzerType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordAnalyzerType::Keyword => "keyword",
            KeywordAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordAnalyzerType::Keyword,
            _ => KeywordAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordMarkerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub keywords_pattern: Option<String>,
    pub r#type: crate::types::common_analysis::KeywordMarkerTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordMarkerTokenFilterType {
    #[doc = "`keyword_marker`"]
    KeywordMarker,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordMarkerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordMarkerTokenFilterType::KeywordMarker => "keyword_marker",
            KeywordMarkerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordMarkerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordMarkerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword_marker" => KeywordMarkerTokenFilterType::KeywordMarker,
            _ => KeywordMarkerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KeywordTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: i64,
    pub r#type: crate::types::common_analysis::KeywordTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeywordTokenizerType {
    #[doc = "`keyword`"]
    Keyword,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KeywordTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KeywordTokenizerType::Keyword => "keyword",
            KeywordTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KeywordTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KeywordTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "keyword" => KeywordTokenizerType::Keyword,
            _ => KeywordTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiAnalyzer {
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    pub r#type: crate::types::common_analysis::KuromojiAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiAnalyzerType {
    #[doc = "`kuromoji`"]
    Kuromoji,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiAnalyzerType::Kuromoji => "kuromoji",
            KuromojiAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji" => KuromojiAnalyzerType::Kuromoji,
            _ => KuromojiAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiIterationMarkCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub normalize_kana: bool,
    pub normalize_kanji: bool,
    pub r#type: crate::types::common_analysis::KuromojiIterationMarkCharFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiIterationMarkCharFilterType {
    #[doc = "`kuromoji_iteration_mark`"]
    KuromojiIterationMark,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiIterationMarkCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiIterationMarkCharFilterType::KuromojiIterationMark => "kuromoji_iteration_mark",
            KuromojiIterationMarkCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiIterationMarkCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiIterationMarkCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_iteration_mark" => KuromojiIterationMarkCharFilterType::KuromojiIterationMark,
            _ => KuromojiIterationMarkCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiPartOfSpeechTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub stoptags: Vec<String>,
    pub r#type: crate::types::common_analysis::KuromojiPartOfSpeechTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiPartOfSpeechTokenFilterType {
    #[doc = "`kuromoji_part_of_speech`"]
    KuromojiPartOfSpeech,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiPartOfSpeechTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiPartOfSpeechTokenFilterType::KuromojiPartOfSpeech => "kuromoji_part_of_speech",
            KuromojiPartOfSpeechTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiPartOfSpeechTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiPartOfSpeechTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_part_of_speech" => KuromojiPartOfSpeechTokenFilterType::KuromojiPartOfSpeech,
            _ => KuromojiPartOfSpeechTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiReadingFormTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::KuromojiReadingFormTokenFilterType,
    pub use_romaji: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiReadingFormTokenFilterType {
    #[doc = "`kuromoji_readingform`"]
    KuromojiReadingform,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiReadingFormTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiReadingFormTokenFilterType::KuromojiReadingform => "kuromoji_readingform",
            KuromojiReadingFormTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiReadingFormTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiReadingFormTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_readingform" => KuromojiReadingFormTokenFilterType::KuromojiReadingform,
            _ => KuromojiReadingFormTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiStemmerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub minimum_length: i64,
    pub r#type: crate::types::common_analysis::KuromojiStemmerTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiStemmerTokenFilterType {
    #[doc = "`kuromoji_stemmer`"]
    KuromojiStemmer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiStemmerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiStemmerTokenFilterType::KuromojiStemmer => "kuromoji_stemmer",
            KuromojiStemmerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiStemmerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiStemmerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_stemmer" => KuromojiStemmerTokenFilterType::KuromojiStemmer,
            _ => KuromojiStemmerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiTokenizationMode {
    #[doc = "`extended`"]
    Extended,
    #[doc = "`normal`"]
    Normal,
    #[doc = "`search`"]
    Search,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiTokenizationMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiTokenizationMode::Extended => "extended",
            KuromojiTokenizationMode::Normal => "normal",
            KuromojiTokenizationMode::Search => "search",
            KuromojiTokenizationMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiTokenizationMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiTokenizationMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "extended" => KuromojiTokenizationMode::Extended,
            "normal" => KuromojiTokenizationMode::Normal,
            "search" => KuromojiTokenizationMode::Search,
            _ => KuromojiTokenizationMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct KuromojiTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_compound_token: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_punctuation: Option<bool>,
    pub mode: crate::types::common_analysis::KuromojiTokenizationMode,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbest_cost: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nbest_examples: Option<String>,
    pub r#type: crate::types::common_analysis::KuromojiTokenizerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary_rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KuromojiTokenizerType {
    #[doc = "`kuromoji_tokenizer`"]
    KuromojiTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl KuromojiTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            KuromojiTokenizerType::KuromojiTokenizer => "kuromoji_tokenizer",
            KuromojiTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for KuromojiTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for KuromojiTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "kuromoji_tokenizer" => KuromojiTokenizerType::KuromojiTokenizer,
            _ => KuromojiTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Language {
    #[doc = "`Arabic`"]
    Arabic,
    #[doc = "`Armenian`"]
    Armenian,
    #[doc = "`Basque`"]
    Basque,
    #[doc = "`Brazilian`"]
    Brazilian,
    #[doc = "`Bulgarian`"]
    Bulgarian,
    #[doc = "`Catalan`"]
    Catalan,
    #[doc = "`Chinese`"]
    Chinese,
    #[doc = "`Cjk`"]
    Cjk,
    #[doc = "`Czech`"]
    Czech,
    #[doc = "`Danish`"]
    Danish,
    #[doc = "`Dutch`"]
    Dutch,
    #[doc = "`English`"]
    English,
    #[doc = "`Estonian`"]
    Estonian,
    #[doc = "`Finnish`"]
    Finnish,
    #[doc = "`French`"]
    French,
    #[doc = "`Galician`"]
    Galician,
    #[doc = "`German`"]
    German,
    #[doc = "`Greek`"]
    Greek,
    #[doc = "`Hindi`"]
    Hindi,
    #[doc = "`Hungarian`"]
    Hungarian,
    #[doc = "`Indonesian`"]
    Indonesian,
    #[doc = "`Irish`"]
    Irish,
    #[doc = "`Italian`"]
    Italian,
    #[doc = "`Latvian`"]
    Latvian,
    #[doc = "`Norwegian`"]
    Norwegian,
    #[doc = "`Persian`"]
    Persian,
    #[doc = "`Portuguese`"]
    Portuguese,
    #[doc = "`Romanian`"]
    Romanian,
    #[doc = "`Russian`"]
    Russian,
    #[doc = "`Sorani`"]
    Sorani,
    #[doc = "`Spanish`"]
    Spanish,
    #[doc = "`Swedish`"]
    Swedish,
    #[doc = "`Thai`"]
    Thai,
    #[doc = "`Turkish`"]
    Turkish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl Language {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            Language::Arabic => "Arabic",
            Language::Armenian => "Armenian",
            Language::Basque => "Basque",
            Language::Brazilian => "Brazilian",
            Language::Bulgarian => "Bulgarian",
            Language::Catalan => "Catalan",
            Language::Chinese => "Chinese",
            Language::Cjk => "Cjk",
            Language::Czech => "Czech",
            Language::Danish => "Danish",
            Language::Dutch => "Dutch",
            Language::English => "English",
            Language::Estonian => "Estonian",
            Language::Finnish => "Finnish",
            Language::French => "French",
            Language::Galician => "Galician",
            Language::German => "German",
            Language::Greek => "Greek",
            Language::Hindi => "Hindi",
            Language::Hungarian => "Hungarian",
            Language::Indonesian => "Indonesian",
            Language::Irish => "Irish",
            Language::Italian => "Italian",
            Language::Latvian => "Latvian",
            Language::Norwegian => "Norwegian",
            Language::Persian => "Persian",
            Language::Portuguese => "Portuguese",
            Language::Romanian => "Romanian",
            Language::Russian => "Russian",
            Language::Sorani => "Sorani",
            Language::Spanish => "Spanish",
            Language::Swedish => "Swedish",
            Language::Thai => "Thai",
            Language::Turkish => "Turkish",
            Language::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Arabic" => Language::Arabic,
            "Armenian" => Language::Armenian,
            "Basque" => Language::Basque,
            "Brazilian" => Language::Brazilian,
            "Bulgarian" => Language::Bulgarian,
            "Catalan" => Language::Catalan,
            "Chinese" => Language::Chinese,
            "Cjk" => Language::Cjk,
            "Czech" => Language::Czech,
            "Danish" => Language::Danish,
            "Dutch" => Language::Dutch,
            "English" => Language::English,
            "Estonian" => Language::Estonian,
            "Finnish" => Language::Finnish,
            "French" => Language::French,
            "Galician" => Language::Galician,
            "German" => Language::German,
            "Greek" => Language::Greek,
            "Hindi" => Language::Hindi,
            "Hungarian" => Language::Hungarian,
            "Indonesian" => Language::Indonesian,
            "Irish" => Language::Irish,
            "Italian" => Language::Italian,
            "Latvian" => Language::Latvian,
            "Norwegian" => Language::Norwegian,
            "Persian" => Language::Persian,
            "Portuguese" => Language::Portuguese,
            "Romanian" => Language::Romanian,
            "Russian" => Language::Russian,
            "Sorani" => Language::Sorani,
            "Spanish" => Language::Spanish,
            "Swedish" => Language::Swedish,
            "Thai" => Language::Thai,
            "Turkish" => Language::Turkish,
            _ => Language::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LanguageAnalyzer {
    pub language: crate::types::common_analysis::Language,
    pub stem_exclusion: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::LanguageAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LanguageAnalyzerType {
    #[doc = "`language`"]
    Language,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LanguageAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LanguageAnalyzerType::Language => "language",
            LanguageAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LanguageAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LanguageAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "language" => LanguageAnalyzerType::Language,
            _ => LanguageAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LengthTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min: Option<i64>,
    pub r#type: crate::types::common_analysis::LengthTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LengthTokenFilterType {
    #[doc = "`length`"]
    Length,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LengthTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LengthTokenFilterType::Length => "length",
            LengthTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LengthTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LengthTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "length" => LengthTokenFilterType::Length,
            _ => LengthTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LetterTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LetterTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LetterTokenizerType {
    #[doc = "`letter`"]
    Letter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LetterTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LetterTokenizerType::Letter => "letter",
            LetterTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LetterTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LetterTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "letter" => LetterTokenizerType::Letter,
            _ => LetterTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LimitTokenCountTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consume_all_tokens: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_count: Option<crate::types::common::StringifiedInteger>,
    pub r#type: crate::types::common_analysis::LimitTokenCountTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LimitTokenCountTokenFilterType {
    #[doc = "`limit`"]
    Limit,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LimitTokenCountTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LimitTokenCountTokenFilterType::Limit => "limit",
            LimitTokenCountTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LimitTokenCountTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LimitTokenCountTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "limit" => LimitTokenCountTokenFilterType::Limit,
            _ => LimitTokenCountTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseNormalizer {
    pub r#type: crate::types::common_analysis::LowercaseNormalizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseNormalizerType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseNormalizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseNormalizerType::Lowercase => "lowercase",
            LowercaseNormalizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseNormalizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseNormalizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseNormalizerType::Lowercase,
            _ => LowercaseNormalizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::LowercaseTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseTokenFilterType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseTokenFilterType::Lowercase => "lowercase",
            LowercaseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseTokenFilterType::Lowercase,
            _ => LowercaseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LowercaseTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::LowercaseTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LowercaseTokenizerType {
    #[doc = "`lowercase`"]
    Lowercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl LowercaseTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            LowercaseTokenizerType::Lowercase => "lowercase",
            LowercaseTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for LowercaseTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for LowercaseTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "lowercase" => LowercaseTokenizerType::Lowercase,
            _ => LowercaseTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MappingCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mappings_path: Option<String>,
    pub r#type: crate::types::common_analysis::MappingCharFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MappingCharFilterType {
    #[doc = "`mapping`"]
    Mapping,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MappingCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MappingCharFilterType::Mapping => "mapping",
            MappingCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MappingCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MappingCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "mapping" => MappingCharFilterType::Mapping,
            _ => MappingCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiplexerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub filters: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::MultiplexerTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MultiplexerTokenFilterType {
    #[doc = "`multiplexer`"]
    Multiplexer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl MultiplexerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            MultiplexerTokenFilterType::Multiplexer => "multiplexer",
            MultiplexerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for MultiplexerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MultiplexerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "multiplexer" => MultiplexerTokenFilterType::Multiplexer,
            _ => MultiplexerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NGramTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_gram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_gram: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::NGramTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NGramTokenFilterType {
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NGramTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NGramTokenFilterType::Ngram => "ngram",
            NGramTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NGramTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NGramTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ngram" => NGramTokenFilterType::Ngram,
            _ => NGramTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NGramTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_token_chars: Option<String>,
    pub max_gram: i64,
    pub min_gram: i64,
    pub token_chars: Vec<crate::types::common_analysis::TokenChar>,
    pub r#type: crate::types::common_analysis::NGramTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NGramTokenizerType {
    #[doc = "`ngram`"]
    Ngram,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NGramTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NGramTokenizerType::Ngram => "ngram",
            NGramTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NGramTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NGramTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ngram" => NGramTokenizerType::Ngram,
            _ => NGramTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriAnalyzerType {
    #[doc = "`nori`"]
    Nori,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriAnalyzerType::Nori => "nori",
            NoriAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori" => NoriAnalyzerType::Nori,
            _ => NoriAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriDecompoundMode {
    #[doc = "`discard`"]
    Discard,
    #[doc = "`mixed`"]
    Mixed,
    #[doc = "`none`"]
    None,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriDecompoundMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriDecompoundMode::Discard => "discard",
            NoriDecompoundMode::Mixed => "mixed",
            NoriDecompoundMode::None => "none",
            NoriDecompoundMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriDecompoundMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriDecompoundMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "discard" => NoriDecompoundMode::Discard,
            "mixed" => NoriDecompoundMode::Mixed,
            "none" => NoriDecompoundMode::None,
            _ => NoriDecompoundMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriPartOfSpeechTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stoptags: Option<Vec<String>>,
    pub r#type: crate::types::common_analysis::NoriPartOfSpeechTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriPartOfSpeechTokenFilterType {
    #[doc = "`nori_part_of_speech`"]
    NoriPartOfSpeech,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriPartOfSpeechTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriPartOfSpeechTokenFilterType::NoriPartOfSpeech => "nori_part_of_speech",
            NoriPartOfSpeechTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriPartOfSpeechTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriPartOfSpeechTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori_part_of_speech" => NoriPartOfSpeechTokenFilterType::NoriPartOfSpeech,
            _ => NoriPartOfSpeechTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NoriTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decompound_mode: Option<crate::types::common_analysis::NoriDecompoundMode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discard_punctuation: Option<bool>,
    pub r#type: crate::types::common_analysis::NoriTokenizerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_dictionary_rules: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoriTokenizerType {
    #[doc = "`nori_tokenizer`"]
    NoriTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NoriTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NoriTokenizerType::NoriTokenizer => "nori_tokenizer",
            NoriTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NoriTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NoriTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "nori_tokenizer" => NoriTokenizerType::NoriTokenizer,
            _ => NoriTokenizerType::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Normalizer {
    raw: Box<serde_json::value::RawValue>,
}
impl Normalizer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `lowercase_normalizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_normalizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseNormalizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_normalizer")
    }
    #[doc = "Parses the value as the `custom_normalizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_custom_normalizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CustomNormalizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "custom_normalizer")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PathHierarchyTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub buffer_size: crate::types::common::StringifiedInteger,
    pub delimiter: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub reverse: crate::types::common::StringifiedBoolean,
    pub skip: crate::types::common::StringifiedInteger,
    pub r#type: crate::types::common_analysis::PathHierarchyTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PathHierarchyTokenizerType {
    #[doc = "`path_hierarchy`"]
    PathHierarchy,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PathHierarchyTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PathHierarchyTokenizerType::PathHierarchy => "path_hierarchy",
            PathHierarchyTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PathHierarchyTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PathHierarchyTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "path_hierarchy" => PathHierarchyTokenizerType::PathHierarchy,
            _ => PathHierarchyTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lowercase: Option<bool>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::PatternAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternAnalyzerType {
    #[doc = "`pattern`"]
    Pattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternAnalyzerType::Pattern => "pattern",
            PatternAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern" => PatternAnalyzerType::Pattern,
            _ => PatternAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternCaptureTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub patterns: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    pub r#type: crate::types::common_analysis::PatternCaptureTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternCaptureTokenFilterType {
    #[doc = "`pattern_capture`"]
    PatternCapture,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternCaptureTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternCaptureTokenFilterType::PatternCapture => "pattern_capture",
            PatternCaptureTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternCaptureTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternCaptureTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_capture" => PatternCaptureTokenFilterType::PatternCapture,
            _ => PatternCaptureTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternReplaceCharFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceCharFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternReplaceCharFilterType {
    #[doc = "`pattern_replace`"]
    PatternReplace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternReplaceCharFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternReplaceCharFilterType::PatternReplace => "pattern_replace",
            PatternReplaceCharFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternReplaceCharFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternReplaceCharFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_replace" => PatternReplaceCharFilterType::PatternReplace,
            _ => PatternReplaceCharFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternReplaceTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    pub pattern: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement: Option<String>,
    pub r#type: crate::types::common_analysis::PatternReplaceTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternReplaceTokenFilterType {
    #[doc = "`pattern_replace`"]
    PatternReplace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternReplaceTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternReplaceTokenFilterType::PatternReplace => "pattern_replace",
            PatternReplaceTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternReplaceTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternReplaceTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern_replace" => PatternReplaceTokenFilterType::PatternReplace,
            _ => PatternReplaceTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PatternTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::PatternTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternTokenizerType {
    #[doc = "`pattern`"]
    Pattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PatternTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PatternTokenizerType::Pattern => "pattern",
            PatternTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PatternTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PatternTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "pattern" => PatternTokenizerType::Pattern,
            _ => PatternTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersianStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PersianStemTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PersianStemTokenFilterType {
    #[doc = "`persian_stem`"]
    PersianStem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PersianStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PersianStemTokenFilterType::PersianStem => "persian_stem",
            PersianStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PersianStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PersianStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "persian_stem" => PersianStemTokenFilterType::PersianStem,
            _ => PersianStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    #[serde(rename = "phone-region")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneAnalyzerType {
    #[doc = "`phone`"]
    Phone,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneAnalyzerType::Phone => "phone",
            PhoneAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phone" => PhoneAnalyzerType::Phone,
            _ => PhoneAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneSearchAnalyzer {
    #[doc = "Optional ISO 3166 country code, defaults to \"ZZ\" (unknown region)."]
    #[serde(rename = "phone-region")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phone_region: Option<String>,
    pub r#type: crate::types::common_analysis::PhoneSearchAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneSearchAnalyzerType {
    #[doc = "`phone-search`"]
    PhoneSearch,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneSearchAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneSearchAnalyzerType::PhoneSearch => "phone-search",
            PhoneSearchAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneSearchAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneSearchAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phone-search" => PhoneSearchAnalyzerType::PhoneSearch,
            _ => PhoneSearchAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticEncoder {
    #[doc = "`beider_morse`"]
    BeiderMorse,
    #[doc = "`caverphone1`"]
    Caverphone1,
    #[doc = "`caverphone2`"]
    Caverphone2,
    #[doc = "`cologne`"]
    Cologne,
    #[doc = "`daitch_mokotoff`"]
    DaitchMokotoff,
    #[doc = "`double_metaphone`"]
    DoubleMetaphone,
    #[doc = "`haasephonetik`"]
    Haasephonetik,
    #[doc = "`koelnerphonetik`"]
    Koelnerphonetik,
    #[doc = "`metaphone`"]
    Metaphone,
    #[doc = "`nysiis`"]
    Nysiis,
    #[doc = "`refined_soundex`"]
    RefinedSoundex,
    #[doc = "`soundex`"]
    Soundex,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticEncoder {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticEncoder::BeiderMorse => "beider_morse",
            PhoneticEncoder::Caverphone1 => "caverphone1",
            PhoneticEncoder::Caverphone2 => "caverphone2",
            PhoneticEncoder::Cologne => "cologne",
            PhoneticEncoder::DaitchMokotoff => "daitch_mokotoff",
            PhoneticEncoder::DoubleMetaphone => "double_metaphone",
            PhoneticEncoder::Haasephonetik => "haasephonetik",
            PhoneticEncoder::Koelnerphonetik => "koelnerphonetik",
            PhoneticEncoder::Metaphone => "metaphone",
            PhoneticEncoder::Nysiis => "nysiis",
            PhoneticEncoder::RefinedSoundex => "refined_soundex",
            PhoneticEncoder::Soundex => "soundex",
            PhoneticEncoder::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticEncoder {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticEncoder {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "beider_morse" => PhoneticEncoder::BeiderMorse,
            "caverphone1" => PhoneticEncoder::Caverphone1,
            "caverphone2" => PhoneticEncoder::Caverphone2,
            "cologne" => PhoneticEncoder::Cologne,
            "daitch_mokotoff" => PhoneticEncoder::DaitchMokotoff,
            "double_metaphone" => PhoneticEncoder::DoubleMetaphone,
            "haasephonetik" => PhoneticEncoder::Haasephonetik,
            "koelnerphonetik" => PhoneticEncoder::Koelnerphonetik,
            "metaphone" => PhoneticEncoder::Metaphone,
            "nysiis" => PhoneticEncoder::Nysiis,
            "refined_soundex" => PhoneticEncoder::RefinedSoundex,
            "soundex" => PhoneticEncoder::Soundex,
            _ => PhoneticEncoder::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticLanguage {
    #[doc = "`any`"]
    Any,
    #[doc = "`common`"]
    Common,
    #[doc = "`cyrillic`"]
    Cyrillic,
    #[doc = "`english`"]
    English,
    #[doc = "`french`"]
    French,
    #[doc = "`german`"]
    German,
    #[doc = "`hebrew`"]
    Hebrew,
    #[doc = "`hungarian`"]
    Hungarian,
    #[doc = "`polish`"]
    Polish,
    #[doc = "`romanian`"]
    Romanian,
    #[doc = "`russian`"]
    Russian,
    #[doc = "`spanish`"]
    Spanish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticLanguage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticLanguage::Any => "any",
            PhoneticLanguage::Common => "common",
            PhoneticLanguage::Cyrillic => "cyrillic",
            PhoneticLanguage::English => "english",
            PhoneticLanguage::French => "french",
            PhoneticLanguage::German => "german",
            PhoneticLanguage::Hebrew => "hebrew",
            PhoneticLanguage::Hungarian => "hungarian",
            PhoneticLanguage::Polish => "polish",
            PhoneticLanguage::Romanian => "romanian",
            PhoneticLanguage::Russian => "russian",
            PhoneticLanguage::Spanish => "spanish",
            PhoneticLanguage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "any" => PhoneticLanguage::Any,
            "common" => PhoneticLanguage::Common,
            "cyrillic" => PhoneticLanguage::Cyrillic,
            "english" => PhoneticLanguage::English,
            "french" => PhoneticLanguage::French,
            "german" => PhoneticLanguage::German,
            "hebrew" => PhoneticLanguage::Hebrew,
            "hungarian" => PhoneticLanguage::Hungarian,
            "polish" => PhoneticLanguage::Polish,
            "romanian" => PhoneticLanguage::Romanian,
            "russian" => PhoneticLanguage::Russian,
            "spanish" => PhoneticLanguage::Spanish,
            _ => PhoneticLanguage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticNameType {
    #[doc = "`ashkenazi`"]
    Ashkenazi,
    #[doc = "`generic`"]
    Generic,
    #[doc = "`sephardic`"]
    Sephardic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticNameType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticNameType::Ashkenazi => "ashkenazi",
            PhoneticNameType::Generic => "generic",
            PhoneticNameType::Sephardic => "sephardic",
            PhoneticNameType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticNameType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticNameType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ashkenazi" => PhoneticNameType::Ashkenazi,
            "generic" => PhoneticNameType::Generic,
            "sephardic" => PhoneticNameType::Sephardic,
            _ => PhoneticNameType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticRuleType {
    #[doc = "`approx`"]
    Approx,
    #[doc = "`exact`"]
    Exact,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticRuleType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticRuleType::Approx => "approx",
            PhoneticRuleType::Exact => "exact",
            PhoneticRuleType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticRuleType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticRuleType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "approx" => PhoneticRuleType::Approx,
            "exact" => PhoneticRuleType::Exact,
            _ => PhoneticRuleType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoneticTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub encoder: crate::types::common_analysis::PhoneticEncoder,
    pub languageset: Vec<crate::types::common_analysis::PhoneticLanguage>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_code_len: Option<i64>,
    pub name_type: crate::types::common_analysis::PhoneticNameType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace: Option<bool>,
    pub rule_type: crate::types::common_analysis::PhoneticRuleType,
    pub r#type: crate::types::common_analysis::PhoneticTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhoneticTokenFilterType {
    #[doc = "`phonetic`"]
    Phonetic,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PhoneticTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PhoneticTokenFilterType::Phonetic => "phonetic",
            PhoneticTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PhoneticTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PhoneticTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "phonetic" => PhoneticTokenFilterType::Phonetic,
            _ => PhoneticTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PorterStemTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::PorterStemTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PorterStemTokenFilterType {
    #[doc = "`porter_stem`"]
    PorterStem,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PorterStemTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PorterStemTokenFilterType::PorterStem => "porter_stem",
            PorterStemTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PorterStemTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PorterStemTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "porter_stem" => PorterStemTokenFilterType::PorterStem,
            _ => PorterStemTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PredicateTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub script: crate::types::common::Script,
    pub r#type: crate::types::common_analysis::PredicateTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PredicateTokenFilterType {
    #[doc = "`predicate_token_filter`"]
    PredicateTokenFilter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl PredicateTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            PredicateTokenFilterType::PredicateTokenFilter => "predicate_token_filter",
            PredicateTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for PredicateTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PredicateTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "predicate_token_filter" => PredicateTokenFilterType::PredicateTokenFilter,
            _ => PredicateTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RemoveDuplicatesTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::RemoveDuplicatesTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RemoveDuplicatesTokenFilterType {
    #[doc = "`remove_duplicates`"]
    RemoveDuplicates,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RemoveDuplicatesTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RemoveDuplicatesTokenFilterType::RemoveDuplicates => "remove_duplicates",
            RemoveDuplicatesTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RemoveDuplicatesTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RemoveDuplicatesTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "remove_duplicates" => RemoveDuplicatesTokenFilterType::RemoveDuplicates,
            _ => RemoveDuplicatesTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ReverseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::ReverseTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ReverseTokenFilterType {
    #[doc = "`reverse`"]
    Reverse,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ReverseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ReverseTokenFilterType::Reverse => "reverse",
            ReverseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ReverseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ReverseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "reverse" => ReverseTokenFilterType::Reverse,
            _ => ReverseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ShingleTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filler_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_shingle_size: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_shingle_size: Option<crate::types::common::StringifiedInteger>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_unigrams_if_no_shingles: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token_separator: Option<String>,
    pub r#type: crate::types::common_analysis::ShingleTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShingleTokenFilterType {
    #[doc = "`shingle`"]
    Shingle,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ShingleTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ShingleTokenFilterType::Shingle => "shingle",
            ShingleTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ShingleTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ShingleTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "shingle" => ShingleTokenFilterType::Shingle,
            _ => ShingleTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimpleAnalyzer {
    pub r#type: crate::types::common_analysis::SimpleAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimpleAnalyzerType {
    #[doc = "`simple`"]
    Simple,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimpleAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimpleAnalyzerType::Simple => "simple",
            SimpleAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimpleAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimpleAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple" => SimpleAnalyzerType::Simple,
            _ => SimpleAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplePatternSplitTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternSplitTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimplePatternSplitTokenizerType {
    #[doc = "`simple_pattern_split`"]
    SimplePatternSplit,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimplePatternSplitTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimplePatternSplitTokenizerType::SimplePatternSplit => "simple_pattern_split",
            SimplePatternSplitTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimplePatternSplitTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimplePatternSplitTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple_pattern_split" => SimplePatternSplitTokenizerType::SimplePatternSplit,
            _ => SimplePatternSplitTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimplePatternTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    pub r#type: crate::types::common_analysis::SimplePatternTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SimplePatternTokenizerType {
    #[doc = "`simple_pattern`"]
    SimplePattern,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SimplePatternTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SimplePatternTokenizerType::SimplePattern => "simple_pattern",
            SimplePatternTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SimplePatternTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SimplePatternTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "simple_pattern" => SimplePatternTokenizerType::SimplePattern,
            _ => SimplePatternTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<crate::types::common_analysis::SmartcnAnalyzerType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnAnalyzerType {
    #[doc = "`smartcn`"]
    Smartcn,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnAnalyzerType::Smartcn => "smartcn",
            SmartcnAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn" => SmartcnAnalyzerType::Smartcn,
            _ => SmartcnAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnStopTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnStopTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnStopTokenFilterType {
    #[doc = "`smartcn_stop`"]
    SmartcnStop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnStopTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnStopTokenFilterType::SmartcnStop => "smartcn_stop",
            SmartcnStopTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnStopTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnStopTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn_stop" => SmartcnStopTokenFilterType::SmartcnStop,
            _ => SmartcnStopTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmartcnTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::SmartcnTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SmartcnTokenizerType {
    #[doc = "`smartcn_tokenizer`"]
    SmartcnTokenizer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SmartcnTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SmartcnTokenizerType::SmartcnTokenizer => "smartcn_tokenizer",
            SmartcnTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SmartcnTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SmartcnTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "smartcn_tokenizer" => SmartcnTokenizerType::SmartcnTokenizer,
            _ => SmartcnTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnowballAnalyzer {
    pub language: crate::types::common_analysis::SnowballLanguage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::SnowballAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballAnalyzerType {
    #[doc = "`snowball`"]
    Snowball,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballAnalyzerType::Snowball => "snowball",
            SnowballAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "snowball" => SnowballAnalyzerType::Snowball,
            _ => SnowballAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballLanguage {
    #[doc = "`Armenian`"]
    Armenian,
    #[doc = "`Basque`"]
    Basque,
    #[doc = "`Catalan`"]
    Catalan,
    #[doc = "`Danish`"]
    Danish,
    #[doc = "`Dutch`"]
    Dutch,
    #[doc = "`English`"]
    English,
    #[doc = "`Finnish`"]
    Finnish,
    #[doc = "`French`"]
    French,
    #[doc = "`German`"]
    German,
    #[doc = "`German2`"]
    German2,
    #[doc = "`Hungarian`"]
    Hungarian,
    #[doc = "`Italian`"]
    Italian,
    #[doc = "`Kp`"]
    Kp,
    #[doc = "`Lovins`"]
    Lovins,
    #[doc = "`Norwegian`"]
    Norwegian,
    #[doc = "`Porter`"]
    Porter,
    #[doc = "`Portuguese`"]
    Portuguese,
    #[doc = "`Romanian`"]
    Romanian,
    #[doc = "`Russian`"]
    Russian,
    #[doc = "`Spanish`"]
    Spanish,
    #[doc = "`Swedish`"]
    Swedish,
    #[doc = "`Turkish`"]
    Turkish,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballLanguage {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballLanguage::Armenian => "Armenian",
            SnowballLanguage::Basque => "Basque",
            SnowballLanguage::Catalan => "Catalan",
            SnowballLanguage::Danish => "Danish",
            SnowballLanguage::Dutch => "Dutch",
            SnowballLanguage::English => "English",
            SnowballLanguage::Finnish => "Finnish",
            SnowballLanguage::French => "French",
            SnowballLanguage::German => "German",
            SnowballLanguage::German2 => "German2",
            SnowballLanguage::Hungarian => "Hungarian",
            SnowballLanguage::Italian => "Italian",
            SnowballLanguage::Kp => "Kp",
            SnowballLanguage::Lovins => "Lovins",
            SnowballLanguage::Norwegian => "Norwegian",
            SnowballLanguage::Porter => "Porter",
            SnowballLanguage::Portuguese => "Portuguese",
            SnowballLanguage::Romanian => "Romanian",
            SnowballLanguage::Russian => "Russian",
            SnowballLanguage::Spanish => "Spanish",
            SnowballLanguage::Swedish => "Swedish",
            SnowballLanguage::Turkish => "Turkish",
            SnowballLanguage::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballLanguage {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballLanguage {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "Armenian" => SnowballLanguage::Armenian,
            "Basque" => SnowballLanguage::Basque,
            "Catalan" => SnowballLanguage::Catalan,
            "Danish" => SnowballLanguage::Danish,
            "Dutch" => SnowballLanguage::Dutch,
            "English" => SnowballLanguage::English,
            "Finnish" => SnowballLanguage::Finnish,
            "French" => SnowballLanguage::French,
            "German" => SnowballLanguage::German,
            "German2" => SnowballLanguage::German2,
            "Hungarian" => SnowballLanguage::Hungarian,
            "Italian" => SnowballLanguage::Italian,
            "Kp" => SnowballLanguage::Kp,
            "Lovins" => SnowballLanguage::Lovins,
            "Norwegian" => SnowballLanguage::Norwegian,
            "Porter" => SnowballLanguage::Porter,
            "Portuguese" => SnowballLanguage::Portuguese,
            "Romanian" => SnowballLanguage::Romanian,
            "Russian" => SnowballLanguage::Russian,
            "Spanish" => SnowballLanguage::Spanish,
            "Swedish" => SnowballLanguage::Swedish,
            "Turkish" => SnowballLanguage::Turkish,
            _ => SnowballLanguage::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnowballTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub language: crate::types::common_analysis::SnowballLanguage,
    pub r#type: crate::types::common_analysis::SnowballTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SnowballTokenFilterType {
    #[doc = "`snowball`"]
    Snowball,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SnowballTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SnowballTokenFilterType::Snowball => "snowball",
            SnowballTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SnowballTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SnowballTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "snowball" => SnowballTokenFilterType::Snowball,
            _ => SnowballTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    pub r#type: crate::types::common_analysis::StandardAnalyzerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardAnalyzerType {
    #[doc = "`standard`"]
    Standard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StandardAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StandardAnalyzerType::Standard => "standard",
            StandardAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StandardAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StandardAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "standard" => StandardAnalyzerType::Standard,
            _ => StandardAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StandardTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::StandardTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StandardTokenizerType {
    #[doc = "`standard`"]
    Standard,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StandardTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StandardTokenizerType::Standard => "standard",
            StandardTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StandardTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StandardTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "standard" => StandardTokenizerType::Standard,
            _ => StandardTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StemmerOverrideTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules_path: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerOverrideTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StemmerOverrideTokenFilterType {
    #[doc = "`stemmer_override`"]
    StemmerOverride,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StemmerOverrideTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StemmerOverrideTokenFilterType::StemmerOverride => "stemmer_override",
            StemmerOverrideTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StemmerOverrideTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StemmerOverrideTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stemmer_override" => StemmerOverrideTokenFilterType::StemmerOverride,
            _ => StemmerOverrideTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StemmerTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    pub r#type: crate::types::common_analysis::StemmerTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StemmerTokenFilterType {
    #[doc = "`stemmer`"]
    Stemmer,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StemmerTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StemmerTokenFilterType::Stemmer => "stemmer",
            StemmerTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StemmerTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StemmerTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stemmer" => StemmerTokenFilterType::Stemmer,
            _ => StemmerTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StopAnalyzer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StopAnalyzerType {
    #[doc = "`stop`"]
    Stop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StopAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StopAnalyzerType::Stop => "stop",
            StopAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StopAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StopAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stop" => StopAnalyzerType::Stop,
            _ => StopAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StopTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_case: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove_trailing: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords: Option<crate::types::common_analysis::StopWords>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stopwords_path: Option<String>,
    pub r#type: crate::types::common_analysis::StopTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StopTokenFilterType {
    #[doc = "`stop`"]
    Stop,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StopTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StopTokenFilterType::Stop => "stop",
            StopTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StopTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StopTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "stop" => StopTokenFilterType::Stop,
            _ => StopTokenFilterType::Unknown(s),
        })
    }
}

#[doc = "Language value, such as `arabic` or `thai`. Defaults to `english`. Each language value corresponds to a predefined list of stop words in Lucene. See Stop words by language for supported language values and their stop words. Also accepts an array of stop words."]
pub type StopWords = crate::types::common::StringOrStringArray;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymFormat {
    #[doc = "`solr`"]
    Solr,
    #[doc = "`wordnet`"]
    Wordnet,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymFormat {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymFormat::Solr => "solr",
            SynonymFormat::Wordnet => "wordnet",
            SynonymFormat::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymFormat {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymFormat {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "solr" => SynonymFormat::Solr,
            "wordnet" => SynonymFormat::Wordnet,
            _ => SynonymFormat::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynonymGraphTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymGraphTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updateable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymGraphTokenFilterType {
    #[doc = "`synonym_graph`"]
    SynonymGraph,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymGraphTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymGraphTokenFilterType::SynonymGraph => "synonym_graph",
            SynonymGraphTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymGraphTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymGraphTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "synonym_graph" => SynonymGraphTokenFilterType::SynonymGraph,
            _ => SynonymGraphTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SynonymTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expand: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<crate::types::common_analysis::SynonymFormat>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lenient: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub synonyms_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tokenizer: Option<String>,
    pub r#type: crate::types::common_analysis::SynonymTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updateable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SynonymTokenFilterType {
    #[doc = "`synonym`"]
    Synonym,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SynonymTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SynonymTokenFilterType::Synonym => "synonym",
            SynonymTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SynonymTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SynonymTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "synonym" => SynonymTokenFilterType::Synonym,
            _ => SynonymTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenChar {
    #[doc = "`custom`"]
    Custom,
    #[doc = "`digit`"]
    Digit,
    #[doc = "`letter`"]
    Letter,
    #[doc = "`punctuation`"]
    Punctuation,
    #[doc = "`symbol`"]
    Symbol,
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TokenChar {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TokenChar::Custom => "custom",
            TokenChar::Digit => "digit",
            TokenChar::Letter => "letter",
            TokenChar::Punctuation => "punctuation",
            TokenChar::Symbol => "symbol",
            TokenChar::Whitespace => "whitespace",
            TokenChar::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TokenChar {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TokenChar {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "custom" => TokenChar::Custom,
            "digit" => TokenChar::Digit,
            "letter" => TokenChar::Letter,
            "punctuation" => TokenChar::Punctuation,
            "symbol" => TokenChar::Symbol,
            "whitespace" => TokenChar::Whitespace,
            _ => TokenChar::Unknown(s),
        })
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenFilter {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenFilter {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TokenFilterDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenFilterDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenFilterDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `ascii_folding_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_ascii_folding_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::AsciiFoldingTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "ascii_folding_token_filter")
    }
    #[doc = "Parses the value as the `common_grams_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_common_grams_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CommonGramsTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "common_grams_token_filter")
    }
    #[doc = "Parses the value as the `condition_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_condition_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ConditionTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "condition_token_filter")
    }
    #[doc = "Parses the value as the `delimited_payload_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_delimited_payload_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DelimitedPayloadTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "delimited_payload_token_filter")
    }
    #[doc = "Parses the value as the `edge_n_gram_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_edge_n_gram_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::EdgeNGramTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "edge_n_gram_token_filter")
    }
    #[doc = "Parses the value as the `elision_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_elision_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ElisionTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "elision_token_filter")
    }
    #[doc = "Parses the value as the `fingerprint_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_fingerprint_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::FingerprintTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "fingerprint_token_filter")
    }
    #[doc = "Parses the value as the `hunspell_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hunspell_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HunspellTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hunspell_token_filter")
    }
    #[doc = "Parses the value as the `hyphenation_decompounder_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_hyphenation_decompounder_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::HyphenationDecompounderTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "hyphenation_decompounder_token_filter")
    }
    #[doc = "Parses the value as the `keep_types_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keep_types_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeepTypesTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keep_types_token_filter")
    }
    #[doc = "Parses the value as the `keep_words_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keep_words_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeepWordsTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keep_words_token_filter")
    }
    #[doc = "Parses the value as the `keyword_marker_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_marker_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordMarkerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_marker_token_filter")
    }
    #[doc = "Parses the value as the `k_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_k_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "k_stem_token_filter")
    }
    #[doc = "Parses the value as the `length_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_length_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LengthTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "length_token_filter")
    }
    #[doc = "Parses the value as the `limit_token_count_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_limit_token_count_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LimitTokenCountTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "limit_token_count_token_filter")
    }
    #[doc = "Parses the value as the `lowercase_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_token_filter")
    }
    #[doc = "Parses the value as the `multiplexer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_multiplexer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::MultiplexerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "multiplexer_token_filter")
    }
    #[doc = "Parses the value as the `n_gram_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_n_gram_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NGramTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "n_gram_token_filter")
    }
    #[doc = "Parses the value as the `nori_part_of_speech_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_part_of_speech_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriPartOfSpeechTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_part_of_speech_token_filter")
    }
    #[doc = "Parses the value as the `pattern_capture_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_capture_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternCaptureTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_capture_token_filter")
    }
    #[doc = "Parses the value as the `pattern_replace_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_replace_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternReplaceTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_replace_token_filter")
    }
    #[doc = "Parses the value as the `persian_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_persian_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PersianStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "persian_stem_token_filter")
    }
    #[doc = "Parses the value as the `porter_stem_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_porter_stem_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PorterStemTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "porter_stem_token_filter")
    }
    #[doc = "Parses the value as the `predicate_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_predicate_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PredicateTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "predicate_token_filter")
    }
    #[doc = "Parses the value as the `remove_duplicates_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_remove_duplicates_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::RemoveDuplicatesTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "remove_duplicates_token_filter")
    }
    #[doc = "Parses the value as the `reverse_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_reverse_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ReverseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "reverse_token_filter")
    }
    #[doc = "Parses the value as the `shingle_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_shingle_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::ShingleTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "shingle_token_filter")
    }
    #[doc = "Parses the value as the `snowball_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_snowball_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SnowballTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "snowball_token_filter")
    }
    #[doc = "Parses the value as the `stemmer_override_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stemmer_override_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StemmerOverrideTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stemmer_override_token_filter")
    }
    #[doc = "Parses the value as the `stemmer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stemmer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StemmerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stemmer_token_filter")
    }
    #[doc = "Parses the value as the `stop_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_stop_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StopTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "stop_token_filter")
    }
    #[doc = "Parses the value as the `synonym_graph_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_synonym_graph_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SynonymGraphTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "synonym_graph_token_filter")
    }
    #[doc = "Parses the value as the `synonym_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_synonym_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SynonymTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "synonym_token_filter")
    }
    #[doc = "Parses the value as the `trim_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_trim_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TrimTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "trim_token_filter")
    }
    #[doc = "Parses the value as the `truncate_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_truncate_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TruncateTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "truncate_token_filter")
    }
    #[doc = "Parses the value as the `unique_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_unique_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UniqueTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "unique_token_filter")
    }
    #[doc = "Parses the value as the `uppercase_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_uppercase_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UppercaseTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "uppercase_token_filter")
    }
    #[doc = "Parses the value as the `word_delimiter_graph_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_word_delimiter_graph_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WordDelimiterGraphTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "word_delimiter_graph_token_filter")
    }
    #[doc = "Parses the value as the `word_delimiter_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_word_delimiter_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WordDelimiterTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "word_delimiter_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_stemmer_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_stemmer_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiStemmerTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_stemmer_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_reading_form_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_reading_form_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiReadingFormTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_reading_form_token_filter")
    }
    #[doc = "Parses the value as the `kuromoji_part_of_speech_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_part_of_speech_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiPartOfSpeechTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_part_of_speech_token_filter")
    }
    #[doc = "Parses the value as the `icu_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_tokenizer")
    }
    #[doc = "Parses the value as the `icu_collation_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_collation_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuCollationTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_collation_token_filter")
    }
    #[doc = "Parses the value as the `icu_folding_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_folding_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuFoldingTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_folding_token_filter")
    }
    #[doc = "Parses the value as the `icu_normalization_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_normalization_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuNormalizationTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_normalization_token_filter")
    }
    #[doc = "Parses the value as the `icu_transform_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_transform_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTransformTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_transform_token_filter")
    }
    #[doc = "Parses the value as the `phonetic_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_phonetic_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PhoneticTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "phonetic_token_filter")
    }
    #[doc = "Parses the value as the `dictionary_decompounder_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_dictionary_decompounder_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::DictionaryDecompounderTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "dictionary_decompounder_token_filter")
    }
    #[doc = "Parses the value as the `smartcn_stop_token_filter` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_stop_token_filter(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnStopTokenFilter,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_stop_token_filter")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Tokenizer {
    raw: Box<serde_json::value::RawValue>,
}
impl Tokenizer {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `name` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_name(&self) -> std::result::Result<String, crate::types::UnionBranchError> {
        crate::types::union::parse_branch(&self.raw, "name")
    }
    #[doc = "Parses the value as the `definition` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_definition(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::TokenizerDefinition,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "definition")
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TokenizerDefinition {
    raw: Box<serde_json::value::RawValue>,
}
impl TokenizerDefinition {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `char_group_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_char_group_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::CharGroupTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "char_group_tokenizer")
    }
    #[doc = "Parses the value as the `edge_n_gram_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_edge_n_gram_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::EdgeNGramTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "edge_n_gram_tokenizer")
    }
    #[doc = "Parses the value as the `keyword_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_keyword_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KeywordTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "keyword_tokenizer")
    }
    #[doc = "Parses the value as the `letter_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_letter_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LetterTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "letter_tokenizer")
    }
    #[doc = "Parses the value as the `lowercase_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_lowercase_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::LowercaseTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "lowercase_tokenizer")
    }
    #[doc = "Parses the value as the `n_gram_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_n_gram_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NGramTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "n_gram_tokenizer")
    }
    #[doc = "Parses the value as the `nori_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_nori_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::NoriTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "nori_tokenizer")
    }
    #[doc = "Parses the value as the `path_hierarchy_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_path_hierarchy_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PathHierarchyTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "path_hierarchy_tokenizer")
    }
    #[doc = "Parses the value as the `standard_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_standard_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::StandardTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "standard_tokenizer")
    }
    #[doc = "Parses the value as the `uax_email_url_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_uax_email_url_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::UaxEmailUrlTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "uax_email_url_tokenizer")
    }
    #[doc = "Parses the value as the `whitespace_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_whitespace_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::WhitespaceTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "whitespace_tokenizer")
    }
    #[doc = "Parses the value as the `kuromoji_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_kuromoji_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::KuromojiTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "kuromoji_tokenizer")
    }
    #[doc = "Parses the value as the `pattern_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_pattern_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::PatternTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "pattern_tokenizer")
    }
    #[doc = "Parses the value as the `simple_pattern_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_pattern_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimplePatternTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_pattern_tokenizer")
    }
    #[doc = "Parses the value as the `simple_pattern_split_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_simple_pattern_split_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SimplePatternSplitTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "simple_pattern_split_tokenizer")
    }
    #[doc = "Parses the value as the `icu_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_icu_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::IcuTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "icu_tokenizer")
    }
    #[doc = "Parses the value as the `smartcn_tokenizer` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_smartcn_tokenizer(
        &self,
    ) -> std::result::Result<
        crate::types::common_analysis::SmartcnTokenizer,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "smartcn_tokenizer")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TrimTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::TrimTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TrimTokenFilterType {
    #[doc = "`trim`"]
    Trim,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TrimTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TrimTokenFilterType::Trim => "trim",
            TrimTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TrimTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TrimTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "trim" => TrimTokenFilterType::Trim,
            _ => TrimTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TruncateTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i64>,
    pub r#type: crate::types::common_analysis::TruncateTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TruncateTokenFilterType {
    #[doc = "`truncate`"]
    Truncate,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TruncateTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TruncateTokenFilterType::Truncate => "truncate",
            TruncateTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TruncateTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TruncateTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "truncate" => TruncateTokenFilterType::Truncate,
            _ => TruncateTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UaxEmailUrlTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::UaxEmailUrlTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UaxEmailUrlTokenizerType {
    #[doc = "`uax_url_email`"]
    UaxUrlEmail,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UaxEmailUrlTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UaxEmailUrlTokenizerType::UaxUrlEmail => "uax_url_email",
            UaxEmailUrlTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UaxEmailUrlTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UaxEmailUrlTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "uax_url_email" => UaxEmailUrlTokenizerType::UaxUrlEmail,
            _ => UaxEmailUrlTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UniqueTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub only_on_same_position: Option<bool>,
    pub r#type: crate::types::common_analysis::UniqueTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UniqueTokenFilterType {
    #[doc = "`unique`"]
    Unique,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UniqueTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UniqueTokenFilterType::Unique => "unique",
            UniqueTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UniqueTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UniqueTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "unique" => UniqueTokenFilterType::Unique,
            _ => UniqueTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UppercaseTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    pub r#type: crate::types::common_analysis::UppercaseTokenFilterType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UppercaseTokenFilterType {
    #[doc = "`uppercase`"]
    Uppercase,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl UppercaseTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            UppercaseTokenFilterType::Uppercase => "uppercase",
            UppercaseTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for UppercaseTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for UppercaseTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "uppercase" => UppercaseTokenFilterType::Uppercase,
            _ => UppercaseTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhitespaceAnalyzer {
    pub r#type: crate::types::common_analysis::WhitespaceAnalyzerType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhitespaceAnalyzerType {
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WhitespaceAnalyzerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WhitespaceAnalyzerType::Whitespace => "whitespace",
            WhitespaceAnalyzerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WhitespaceAnalyzerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WhitespaceAnalyzerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "whitespace" => WhitespaceAnalyzerType::Whitespace,
            _ => WhitespaceAnalyzerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhitespaceTokenizer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_token_length: Option<i64>,
    pub r#type: crate::types::common_analysis::WhitespaceTokenizerType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WhitespaceTokenizerType {
    #[doc = "`whitespace`"]
    Whitespace,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WhitespaceTokenizerType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WhitespaceTokenizerType::Whitespace => "whitespace",
            WhitespaceTokenizerType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WhitespaceTokenizerType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WhitespaceTokenizerType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "whitespace" => WhitespaceTokenizerType::Whitespace,
            _ => WhitespaceTokenizerType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WordDelimiterGraphTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adjust_offsets: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_words: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_number_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_word_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_keywords: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_case_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_numerics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterGraphTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordDelimiterGraphTokenFilterType {
    #[doc = "`word_delimiter_graph`"]
    WordDelimiterGraph,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WordDelimiterGraphTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WordDelimiterGraphTokenFilterType::WordDelimiterGraph => "word_delimiter_graph",
            WordDelimiterGraphTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WordDelimiterGraphTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WordDelimiterGraphTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "word_delimiter_graph" => WordDelimiterGraphTokenFilterType::WordDelimiterGraph,
            _ => WordDelimiterGraphTokenFilterType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WordDelimiterTokenFilter {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionString>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_all: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_numbers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catenate_words: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_number_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generate_word_parts: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preserve_original: Option<crate::types::common::StringifiedBoolean>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub protected_words_path: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_case_change: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_on_numerics: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stem_english_possessive: Option<bool>,
    pub r#type: crate::types::common_analysis::WordDelimiterTokenFilterType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_table_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WordDelimiterTokenFilterType {
    #[doc = "`word_delimiter`"]
    WordDelimiter,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl WordDelimiterTokenFilterType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            WordDelimiterTokenFilterType::WordDelimiter => "word_delimiter",
            WordDelimiterTokenFilterType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for WordDelimiterTokenFilterType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WordDelimiterTokenFilterType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "word_delimiter" => WordDelimiterTokenFilterType::WordDelimiter,
            _ => WordDelimiterTokenFilterType::Unknown(s),
        })
    }
}
