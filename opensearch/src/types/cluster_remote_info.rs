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

//! Response types of the `cluster_remote_info` specification namespace

#![allow(missing_docs)]

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ClusterRemoteInfo {
    raw: Box<serde_json::value::RawValue>,
}
impl ClusterRemoteInfo {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `cluster_remote_sniff_info` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cluster_remote_sniff_info(
        &self,
    ) -> std::result::Result<
        crate::types::cluster_remote_info::ClusterRemoteSniffInfo,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cluster_remote_sniff_info")
    }
    #[doc = "Parses the value as the `cluster_remote_proxy_info` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_cluster_remote_proxy_info(
        &self,
    ) -> std::result::Result<
        crate::types::cluster_remote_info::ClusterRemoteProxyInfo,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "cluster_remote_proxy_info")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterRemoteProxyInfo {
    pub connected: bool,
    pub initial_connect_timeout: crate::types::common::Duration,
    pub max_proxy_socket_connections: i32,
    pub mode: crate::types::cluster_remote_info::ClusterRemoteProxyInfoMode,
    pub num_proxy_sockets_connected: i32,
    pub proxy_address: String,
    pub server_name: String,
    pub skip_unavailable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClusterRemoteProxyInfoMode {
    #[doc = "`proxy`"]
    Proxy,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ClusterRemoteProxyInfoMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ClusterRemoteProxyInfoMode::Proxy => "proxy",
            ClusterRemoteProxyInfoMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ClusterRemoteProxyInfoMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteProxyInfoMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "proxy" => ClusterRemoteProxyInfoMode::Proxy,
            _ => ClusterRemoteProxyInfoMode::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ClusterRemoteSniffInfo {
    pub connected: bool,
    pub initial_connect_timeout: crate::types::common::Duration,
    pub max_connections_per_cluster: i32,
    pub mode: crate::types::cluster_remote_info::ClusterRemoteSniffInfoMode,
    pub num_nodes_connected: i64,
    pub seeds: Vec<String>,
    pub skip_unavailable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ClusterRemoteSniffInfoMode {
    #[doc = "`sniff`"]
    Sniff,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ClusterRemoteSniffInfoMode {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ClusterRemoteSniffInfoMode::Sniff => "sniff",
            ClusterRemoteSniffInfoMode::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ClusterRemoteSniffInfoMode {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ClusterRemoteSniffInfoMode {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "sniff" => ClusterRemoteSniffInfoMode::Sniff,
            _ => ClusterRemoteSniffInfoMode::Unknown(s),
        })
    }
}

#[doc = "The response of the `cluster.remote_info` API"]
pub type RemoteInfoResponse = crate::types::cluster::RemoteInfoResponse;
