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

//! Response types of the `wlm_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponse {
    #[doc = "The ID of the query group, which can be used to associate query requests with the group and enforce the group's resource limits."]
    #[serde(rename = "_id")]
    pub id: String,
    #[doc = "The name of the query group."]
    pub name: String,
    #[doc = "The resiliency mode of the query group."]
    pub resiliency_mode: crate::types::wlm_common::ResiliencyMode,
    #[doc = "The resource limits for query requests in the query group."]
    pub resource_limits: crate::types::wlm_common::QueryGroupResponseResourceLimits,
    #[doc = "The time at which the query group was last updated."]
    pub updated_at: i64,
}

#[doc = "The resource limits for query requests in the query group."]
#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct QueryGroupResponseResourceLimits {
    raw: Box<serde_json::value::RawValue>,
}
impl QueryGroupResponseResourceLimits {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object")
    }
    #[doc = "Parses the value as the `object_object` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_object_object(
        &self,
    ) -> std::result::Result<
        crate::types::wlm_common::QueryGroupResponseResourceLimitsObjectObject,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "object_object")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponseResourceLimitsObject {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cpu: Option<f64>,
    pub memory: f64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct QueryGroupResponseResourceLimitsObjectObject {
    pub cpu: f64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResiliencyMode {
    #[doc = "`enforced`"]
    Enforced,
    #[doc = "`monitor`"]
    Monitor,
    #[doc = "`soft`"]
    Soft,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ResiliencyMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ResiliencyMode::Enforced => "enforced",
            ResiliencyMode::Monitor => "monitor",
            ResiliencyMode::Soft => "soft",
            ResiliencyMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ResiliencyMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ResiliencyMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "enforced" => ResiliencyMode::Enforced,
            "monitor" => ResiliencyMode::Monitor,
            "soft" => ResiliencyMode::Soft,
            _ => ResiliencyMode::Unknown(s),
        })
    }
}
