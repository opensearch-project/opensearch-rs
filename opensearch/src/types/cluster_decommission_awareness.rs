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

//! Response types of the `cluster_decommission_awareness` specification namespace

#![allow(missing_docs)]

#[doc = "An enumeration of the states during decommissioning."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DecommissionStatus {
    #[doc = "`INIT`"]
    Init,
    #[doc = "`DRAINING`"]
    Draining,
    #[doc = "`IN_PROGRESS`"]
    InProgress,
    #[doc = "`SUCCESSFUL`"]
    Successful,
    #[doc = "`FAILED`"]
    Failed,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl DecommissionStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            DecommissionStatus::Init => "INIT",
            DecommissionStatus::Draining => "DRAINING",
            DecommissionStatus::InProgress => "IN_PROGRESS",
            DecommissionStatus::Successful => "SUCCESSFUL",
            DecommissionStatus::Failed => "FAILED",
            DecommissionStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for DecommissionStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for DecommissionStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "INIT" => DecommissionStatus::Init,
            "DRAINING" => DecommissionStatus::Draining,
            "IN_PROGRESS" => DecommissionStatus::InProgress,
            "SUCCESSFUL" => DecommissionStatus::Successful,
            "FAILED" => DecommissionStatus::Failed,
            _ => DecommissionStatus::Unknown(s),
        })
    }
}
