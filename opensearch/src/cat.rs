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

//! Cat APIs
//!
//! The [Cat APIs](https://docs.opensearch.org/latest/api-reference/cat/index/) aim to
//! meet the needs of humans when looking at data returned from OpenSearch,
//! formatting it as compact, column aligned text, making it easier on human eyes.
//!
//! # Plain text responses
//!
//! By default, all Cat APIs are configured to send requests with `text/plain` content-type
//! and accept headers, returning plain text responses
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts};
//! # use url::Url;
//! # use opensearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .cat()
//!     .nodes()
//!     .send()
//!     .await?;
//!
//! let response_body = response.text().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # JSON responses
//!
//! JSON responses can be returned from Cat APIs either by using `.format("json")`
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts};
//! # use url::Url;
//! # use opensearch::auth::Credentials;
//! # use serde_json::{json, Value};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .cat()
//!     .nodes()
//!     .format("json")
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<Value>().await?;
//! # Ok(())
//! # }
//! ```
//!
//! Or by setting an accept header using `.headers()`
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts, http::headers::{HeaderValue, DEFAULT_ACCEPT, ACCEPT}};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .cat()
//!     .nodes()
//!     .header(ACCEPT, HeaderValue::from_static(DEFAULT_ACCEPT))
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<Value>().await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Column Headers
//!
//! The column headers to return can be controlled with `.h()`
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, Error, SearchParts};
//! # use url::Url;
//! # use serde_json::{json, Value};
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! let response = client
//!     .cat()
//!     .nodes()
//!     .h(&["ip", "port", "heapPercent", "name"])
//!     .send()
//!     .await?;
//!
//! let response_body = response.json::<String>().await?;
//! # Ok(())
//! # }
//! ```
//!

#![allow(unused_imports)]
use crate::{
    client::OpenSearch,
    error::Error,
    http::{
        headers::{HeaderMap, HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE},
        request::{Body, JsonBody, NdBody, PARTS_ENCODED},
        response::Response,
        transport::Transport,
        Method,
    },
    params::*,
};
use percent_encoding::percent_encode;
use serde::Serialize;
use std::{borrow::Cow, time::Duration};
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Aliases API"]
pub enum CatAliasesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> CatAliasesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Aliases API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatAliasesParts::None => "/_cat/aliases".into(),
            CatAliasesParts::Name(name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_name.len());
                p.push_str("/_cat/aliases/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Aliases API](https://opensearch.org/docs/latest/api-reference/cat/cat-aliases/)\n\nShows information about aliases currently configured to indexes, including filter and routing information."]
#[derive(Clone, Debug)]
pub struct CatAliases<'a, 'b> {
    transport: &'a Transport,
    parts: CatAliasesParts<'b>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatAliases<'a, 'b> {
    #[doc = "Creates a new instance of [CatAliases] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatAliasesParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatAliases {
            transport,
            parts,
            headers,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to return information from the local node only instead of from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Aliases API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat All Pit Segments API"]
pub enum CatAllPitSegmentsParts {
    #[doc = "No parts"]
    None,
}
impl CatAllPitSegmentsParts {
    #[doc = "Builds a relative URL path to the Cat All Pit Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatAllPitSegmentsParts::None => "/_cat/pit_segments/_all".into(),
        }
    }
}
#[doc = "Builder for the [Cat All Pit Segments API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/)\n\nLists all active CAT point-in-time segments."]
#[derive(Clone, Debug)]
pub struct CatAllPitSegments<'a, 'b> {
    transport: &'a Transport,
    parts: CatAllPitSegmentsParts,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatAllPitSegments<'a, 'b> {
    #[doc = "Creates a new instance of [CatAllPitSegments]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatAllPitSegments {
            transport,
            parts: CatAllPitSegmentsParts::None,
            headers,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat All Pit Segments API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Allocation API"]
pub enum CatAllocationParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> CatAllocationParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Allocation API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatAllocationParts::None => "/_cat/allocation".into(),
            CatAllocationParts::NodeId(node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_node_id.len());
                p.push_str("/_cat/allocation/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Allocation API](https://opensearch.org/docs/latest/api-reference/cat/cat-allocation/)\n\nProvides a snapshot of how many shards are allocated to each data node and how much disk space they are using."]
#[derive(Clone, Debug)]
pub struct CatAllocation<'a, 'b> {
    transport: &'a Transport,
    parts: CatAllocationParts<'b>,
    bytes: Option<Bytes>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatAllocation<'a, 'b> {
    #[doc = "Creates a new instance of [CatAllocation] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatAllocationParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatAllocation {
            transport,
            parts,
            headers,
            bytes: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "A timeout for connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the HTTP `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from cluster-manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "A timeout for connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Allocation API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Cluster Manager API"]
pub enum CatClusterManagerParts {
    #[doc = "No parts"]
    None,
}
impl CatClusterManagerParts {
    #[doc = "Builds a relative URL path to the Cat Cluster Manager API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatClusterManagerParts::None => "/_cat/cluster_manager".into(),
        }
    }
}
#[doc = "Builder for the [Cat Cluster Manager API](https://opensearch.org/docs/latest/api-reference/cat/cat-cluster_manager/)\n\nReturns information about the cluster-manager node."]
#[derive(Clone, Debug)]
pub struct CatClusterManager<'a, 'b> {
    transport: &'a Transport,
    parts: CatClusterManagerParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatClusterManager<'a, 'b> {
    #[doc = "Creates a new instance of [CatClusterManager]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatClusterManager {
            transport,
            parts: CatClusterManagerParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "A timeout for connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the HTTP `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "A timeout for connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Cluster Manager API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Count API"]
pub enum CatCountParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatCountParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Count API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatCountParts::None => "/_cat/count".into(),
            CatCountParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_index.len());
                p.push_str("/_cat/count/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Count API](https://opensearch.org/docs/latest/api-reference/cat/cat-count/)\n\nProvides quick access to the document count of the entire cluster or of an individual index."]
#[derive(Clone, Debug)]
pub struct CatCount<'a, 'b> {
    transport: &'a Transport,
    parts: CatCountParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatCount<'a, 'b> {
    #[doc = "Creates a new instance of [CatCount] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatCountParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatCount {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Count API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Fielddata API"]
pub enum CatFielddataParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Fields"]
    Fields(&'b [&'b str]),
}
impl<'b> CatFielddataParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Fielddata API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatFielddataParts::None => "/_cat/fielddata".into(),
            CatFielddataParts::Fields(fields) => {
                let fields_str = fields.join(",");
                let encoded_fields: Cow<str> =
                    percent_encode(fields_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_fields.len());
                p.push_str("/_cat/fielddata/");
                p.push_str(encoded_fields.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Fielddata API](https://opensearch.org/docs/latest/api-reference/cat/cat-field-data/)\n\nShows how much heap memory is currently being used by field data on every data node in the cluster."]
#[derive(Clone, Debug)]
pub struct CatFielddata<'a, 'b> {
    transport: &'a Transport,
    parts: CatFielddataParts<'b>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatFielddata<'a, 'b> {
    #[doc = "Creates a new instance of [CatFielddata] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatFielddataParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatFielddata {
            transport,
            parts,
            headers,
            bytes: None,
            error_trace: None,
            fields: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of fields used to limit the amount of returned information."]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Fielddata API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                fields: self.fields,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Health API"]
pub enum CatHealthParts {
    #[doc = "No parts"]
    None,
}
impl CatHealthParts {
    #[doc = "Builds a relative URL path to the Cat Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatHealthParts::None => "/_cat/health".into(),
        }
    }
}
#[doc = "Builder for the [Cat Health API](https://opensearch.org/docs/latest/api-reference/cat/cat-health/)\n\nReturns a concise representation of the cluster health."]
#[derive(Clone, Debug)]
pub struct CatHealth<'a, 'b> {
    transport: &'a Transport,
    parts: CatHealthParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    ts: Option<bool>,
    v: Option<bool>,
}
impl<'a, 'b> CatHealth<'a, 'b> {
    #[doc = "Creates a new instance of [CatHealth]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatHealth {
            transport,
            parts: CatHealthParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            ts: None,
            v: None,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The unit used to display time values."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "When `true`, returns `HH:MM:SS` and Unix epoch timestamps."]
    pub fn ts(mut self, ts: bool) -> Self {
        self.ts = Some(ts);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Health API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                ts: Option<bool>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                ts: self.ts,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Help API"]
pub enum CatHelpParts {
    #[doc = "No parts"]
    None,
}
impl CatHelpParts {
    #[doc = "Builds a relative URL path to the Cat Help API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatHelpParts::None => "/_cat".into(),
        }
    }
}
#[doc = "Builder for the [Cat Help API](https://opensearch.org/docs/latest/api-reference/cat/index/)\n\nReturns help for the Cat APIs."]
#[derive(Clone, Debug)]
pub struct CatHelp<'a, 'b> {
    transport: &'a Transport,
    parts: CatHelpParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> CatHelp<'a, 'b> {
    #[doc = "Creates a new instance of [CatHelp]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatHelp {
            transport,
            parts: CatHelpParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Help API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Indices API"]
pub enum CatIndicesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatIndicesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Indices API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatIndicesParts::None => "/_cat/indices".into(),
            CatIndicesParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push_str("/_cat/indices/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Indices API](https://opensearch.org/docs/latest/api-reference/cat/cat-indices/)\n\nLists information related to indexes, that is, how much disk space they are using, how many shards they have, their health status, and so on."]
#[derive(Clone, Debug)]
pub struct CatIndices<'a, 'b> {
    transport: &'a Transport,
    parts: CatIndicesParts<'b>,
    bytes: Option<Bytes>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    health: Option<Health>,
    help: Option<bool>,
    human: Option<bool>,
    include_unloaded_segments: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    pri: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatIndices<'a, 'b> {
    #[doc = "Creates a new instance of [CatIndices] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatIndicesParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatIndices {
            transport,
            parts,
            headers,
            bytes: None,
            cluster_manager_timeout: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            format: None,
            h: None,
            health: None,
            help: None,
            human: None,
            include_unloaded_segments: None,
            local: None,
            master_timeout: None,
            pretty: None,
            pri: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Limits indexes based on their health status. Supported values are `green`, `yellow`, and `red`."]
    pub fn health(mut self, health: Health) -> Self {
        self.health = Some(health);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to include information from segments not loaded into memory."]
    pub fn include_unloaded_segments(mut self, include_unloaded_segments: bool) -> Self {
        self.include_unloaded_segments = Some(include_unloaded_segments);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "When `true`, returns information only from the primary shards."]
    pub fn pri(mut self, pri: bool) -> Self {
        self.pri = Some(pri);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Indices API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                health: Option<Health>,
                help: Option<bool>,
                human: Option<bool>,
                include_unloaded_segments: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                pri: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                health: self.health,
                help: self.help,
                human: self.human,
                include_unloaded_segments: self.include_unloaded_segments,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                pri: self.pri,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[deprecated = "To promote inclusive language, use '/_cat/cluster_manager' instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Master API"]
pub enum CatMasterParts {
    #[doc = "No parts"]
    None,
}
#[allow(deprecated)]
impl CatMasterParts {
    #[doc = "Builds a relative URL path to the Cat Master API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatMasterParts::None => "/_cat/master".into(),
        }
    }
}
#[doc = "Builder for the [Cat Master API](https://opensearch.org/docs/latest/api-reference/cat/cat-cluster_manager/)\n\nReturns information about the cluster-manager node."]
#[deprecated = "To promote inclusive language, use '/_cat/cluster_manager' instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct CatMaster<'a, 'b> {
    transport: &'a Transport,
    parts: CatMasterParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
#[allow(deprecated)]
impl<'a, 'b> CatMaster<'a, 'b> {
    #[doc = "Creates a new instance of [CatMaster]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatMaster {
            transport,
            parts: CatMasterParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Master API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Nodeattrs API"]
pub enum CatNodeattrsParts {
    #[doc = "No parts"]
    None,
}
impl CatNodeattrsParts {
    #[doc = "Builds a relative URL path to the Cat Nodeattrs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatNodeattrsParts::None => "/_cat/nodeattrs".into(),
        }
    }
}
#[doc = "Builder for the [Cat Nodeattrs API](https://opensearch.org/docs/latest/api-reference/cat/cat-nodeattrs/)\n\nReturns information about custom node attributes."]
#[derive(Clone, Debug)]
pub struct CatNodeattrs<'a, 'b> {
    transport: &'a Transport,
    parts: CatNodeattrsParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatNodeattrs<'a, 'b> {
    #[doc = "Creates a new instance of [CatNodeattrs]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatNodeattrs {
            transport,
            parts: CatNodeattrsParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Nodeattrs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Nodes API"]
pub enum CatNodesParts {
    #[doc = "No parts"]
    None,
}
impl CatNodesParts {
    #[doc = "Builds a relative URL path to the Cat Nodes API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatNodesParts::None => "/_cat/nodes".into(),
        }
    }
}
#[doc = "Builder for the [Cat Nodes API](https://opensearch.org/docs/latest/api-reference/cat/cat-nodes/)\n\nReturns basic statistics about the performance of cluster nodes."]
#[derive(Clone, Debug)]
pub struct CatNodes<'a, 'b> {
    transport: &'a Transport,
    parts: CatNodesParts,
    bytes: Option<Bytes>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    full_id: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatNodes<'a, 'b> {
    #[doc = "Creates a new instance of [CatNodes]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatNodes {
            transport,
            parts: CatNodesParts::None,
            headers,
            bytes: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            full_id: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "When `true`, returns the full node ID. When `false`, returns the shortened node ID."]
    pub fn full_id(mut self, full_id: &'b str) -> Self {
        self.full_id = Some(full_id);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    #[deprecated = "This parameter does not cause this API to act locally."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Nodes API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                full_id: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                full_id: self.full_id,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Pending Tasks API"]
pub enum CatPendingTasksParts {
    #[doc = "No parts"]
    None,
}
impl CatPendingTasksParts {
    #[doc = "Builds a relative URL path to the Cat Pending Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatPendingTasksParts::None => "/_cat/pending_tasks".into(),
        }
    }
}
#[doc = "Builder for the [Cat Pending Tasks API](https://opensearch.org/docs/latest/api-reference/cat/cat-pending-tasks/)\n\nReturns a concise representation of the cluster's pending tasks."]
#[derive(Clone, Debug)]
pub struct CatPendingTasks<'a, 'b> {
    transport: &'a Transport,
    parts: CatPendingTasksParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatPendingTasks<'a, 'b> {
    #[doc = "Creates a new instance of [CatPendingTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatPendingTasks {
            transport,
            parts: CatPendingTasksParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Pending Tasks API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Pit Segments API"]
pub enum CatPitSegmentsParts {
    #[doc = "No parts"]
    None,
}
impl CatPitSegmentsParts {
    #[doc = "Builds a relative URL path to the Cat Pit Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatPitSegmentsParts::None => "/_cat/pit_segments".into(),
        }
    }
}
#[doc = "Builder for the [Cat Pit Segments API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/)\n\nLists one or several CAT point-in-time segments."]
#[derive(Clone, Debug)]
pub struct CatPitSegments<'a, 'b, B> {
    transport: &'a Transport,
    parts: CatPitSegmentsParts,
    body: Option<B>,
    bytes: Option<Bytes>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b, B> CatPitSegments<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CatPitSegments]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatPitSegments {
            transport,
            parts: CatPitSegmentsParts::None,
            headers,
            body: None,
            bytes: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CatPitSegments<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CatPitSegments {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            bytes: self.bytes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            format: self.format,
            h: self.h,
            headers: self.headers,
            help: self.help,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            s: self.s,
            source: self.source,
            v: self.v,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Pit Segments API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Plugins API"]
pub enum CatPluginsParts {
    #[doc = "No parts"]
    None,
}
impl CatPluginsParts {
    #[doc = "Builds a relative URL path to the Cat Plugins API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatPluginsParts::None => "/_cat/plugins".into(),
        }
    }
}
#[doc = "Builder for the [Cat Plugins API](https://opensearch.org/docs/latest/api-reference/cat/cat-plugins/)\n\nReturns information about the names, components, and versions of the installed plugins."]
#[derive(Clone, Debug)]
pub struct CatPlugins<'a, 'b> {
    transport: &'a Transport,
    parts: CatPluginsParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatPlugins<'a, 'b> {
    #[doc = "Creates a new instance of [CatPlugins]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatPlugins {
            transport,
            parts: CatPluginsParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Plugins API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Recovery API"]
pub enum CatRecoveryParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatRecoveryParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Recovery API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatRecoveryParts::None => "/_cat/recovery".into(),
            CatRecoveryParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/_cat/recovery/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Recovery API](https://opensearch.org/docs/latest/api-reference/cat/cat-plugins/)\n\nReturns all completed and ongoing index and shard recoveries."]
#[derive(Clone, Debug)]
pub struct CatRecovery<'a, 'b> {
    transport: &'a Transport,
    parts: CatRecoveryParts<'b>,
    active_only: Option<bool>,
    bytes: Option<Bytes>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatRecovery<'a, 'b> {
    #[doc = "Creates a new instance of [CatRecovery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatRecoveryParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatRecovery {
            transport,
            parts,
            headers,
            active_only: None,
            bytes: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "If `true`, the response only includes ongoing shard recoveries."]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "When `true`, includes detailed information about shard recoveries."]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "A comma-separated list of data streams, indexes, and aliases used to limit the request.\nSupports wildcards (`*`). To target all data streams and indexes, omit this parameter or use `*` or `_all`."]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Recovery API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                active_only: Option<bool>,
                bytes: Option<Bytes>,
                detailed: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
                bytes: self.bytes,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                index: self.index,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Repositories API"]
pub enum CatRepositoriesParts {
    #[doc = "No parts"]
    None,
}
impl CatRepositoriesParts {
    #[doc = "Builds a relative URL path to the Cat Repositories API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatRepositoriesParts::None => "/_cat/repositories".into(),
        }
    }
}
#[doc = "Builder for the [Cat Repositories API](https://opensearch.org/docs/latest/api-reference/cat/cat-repositories/)\n\nReturns information about all snapshot repositories for a cluster."]
#[derive(Clone, Debug)]
pub struct CatRepositories<'a, 'b> {
    transport: &'a Transport,
    parts: CatRepositoriesParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatRepositories<'a, 'b> {
    #[doc = "Creates a new instance of [CatRepositories]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatRepositories {
            transport,
            parts: CatRepositoriesParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Repositories API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Segment Replication API"]
pub enum CatSegmentReplicationParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatSegmentReplicationParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Segment Replication API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatSegmentReplicationParts::None => "/_cat/segment_replication".into(),
            CatSegmentReplicationParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_index.len());
                p.push_str("/_cat/segment_replication/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Segment Replication API](https://opensearch.org/docs/latest/api-reference/cat/cat-segment-replication/)\n\nReturns information about active and last-completed segment replication events on each replica shard, including related shard-level metrics. \nThese metrics provide information about how far behind the primary shard the replicas are lagging."]
#[derive(Clone, Debug)]
pub struct CatSegmentReplication<'a, 'b> {
    transport: &'a Transport,
    parts: CatSegmentReplicationParts<'b>,
    active_only: Option<bool>,
    allow_no_indices: Option<bool>,
    bytes: Option<Bytes>,
    completed_only: Option<bool>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    index: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    shards: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    timeout: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatSegmentReplication<'a, 'b> {
    #[doc = "Creates a new instance of [CatSegmentReplication] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatSegmentReplicationParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatSegmentReplication {
            transport,
            parts,
            headers,
            active_only: None,
            allow_no_indices: None,
            bytes: None,
            completed_only: None,
            detailed: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            index: None,
            pretty: None,
            request_timeout: None,
            s: None,
            shards: None,
            source: None,
            time: None,
            timeout: None,
            v: None,
        }
    }
    #[doc = "When `true`, the response only includes ongoing segment replication events."]
    pub fn active_only(mut self, active_only: bool) -> Self {
        self.active_only = Some(active_only);
        self
    }
    #[doc = "Whether to ignore the index if a wildcard index expression resolves to no concrete indexes. This includes the `_all` string or when no indexes have been specified."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "When `true`, the response only includes the last-completed segment replication events."]
    pub fn completed_only(mut self, completed_only: bool) -> Self {
        self.completed_only = Some(completed_only);
        self
    }
    #[doc = "When `true`, the response includes additional metrics for each stage of a segment replication event."]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Whether specified concrete, expanded, or aliased indexes should be ignored when throttled."]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether the specified concrete indexes should be ignored when missing or closed."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "A comma-separated list of data streams, indexes, and aliases used to limit the request.\nSupports wildcards (`*`). To target all data streams and indexes, omit this parameter or use `*` or `_all`."]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "A comma-separated list of shards to display."]
    pub fn shards(mut self, shards: &'b [&'b str]) -> Self {
        self.shards = Some(shards);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "The operation timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Segment Replication API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                active_only: Option<bool>,
                allow_no_indices: Option<bool>,
                bytes: Option<Bytes>,
                completed_only: Option<bool>,
                detailed: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                shards: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                timeout: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                active_only: self.active_only,
                allow_no_indices: self.allow_no_indices,
                bytes: self.bytes,
                completed_only: self.completed_only,
                detailed: self.detailed,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                index: self.index,
                pretty: self.pretty,
                s: self.s,
                shards: self.shards,
                source: self.source,
                time: self.time,
                timeout: self.timeout,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Segments API"]
pub enum CatSegmentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatSegmentsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Segments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatSegmentsParts::None => "/_cat/segments".into(),
            CatSegmentsParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push_str("/_cat/segments/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Segments API](https://opensearch.org/docs/latest/api-reference/cat/cat-segments/)\n\nProvides low-level information about the segments in the shards of an index."]
#[derive(Clone, Debug)]
pub struct CatSegments<'a, 'b> {
    transport: &'a Transport,
    parts: CatSegmentsParts<'b>,
    bytes: Option<Bytes>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatSegments<'a, 'b> {
    #[doc = "Creates a new instance of [CatSegments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatSegmentsParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatSegments {
            transport,
            parts,
            headers,
            bytes: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Segments API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Shards API"]
pub enum CatShardsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CatShardsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Shards API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatShardsParts::None => "/_cat/shards".into(),
            CatShardsParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push_str("/_cat/shards/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Shards API](https://opensearch.org/docs/latest/api-reference/cat/cat-shards/)\n\nLists the states of all primary and replica shards and how they are distributed."]
#[derive(Clone, Debug)]
pub struct CatShards<'a, 'b> {
    transport: &'a Transport,
    parts: CatShardsParts<'b>,
    bytes: Option<Bytes>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatShards<'a, 'b> {
    #[doc = "Creates a new instance of [CatShards] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatShardsParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatShards {
            transport,
            parts,
            headers,
            bytes: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The units used to display byte values."]
    pub fn bytes(mut self, bytes: Bytes) -> Self {
        self.bytes = Some(bytes);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Shards API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                bytes: Option<Bytes>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                bytes: self.bytes,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Snapshots API"]
pub enum CatSnapshotsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Repository"]
    Repository(&'b [&'b str]),
}
impl<'b> CatSnapshotsParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Snapshots API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatSnapshotsParts::None => "/_cat/snapshots".into(),
            CatSnapshotsParts::Repository(repository) => {
                let repository_str = repository.join(",");
                let encoded_repository: Cow<str> =
                    percent_encode(repository_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_repository.len());
                p.push_str("/_cat/snapshots/");
                p.push_str(encoded_repository.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Snapshots API](https://opensearch.org/docs/latest/api-reference/cat/cat-snapshots/)\n\nLists all of the snapshots stored in a specific repository."]
#[derive(Clone, Debug)]
pub struct CatSnapshots<'a, 'b> {
    transport: &'a Transport,
    parts: CatSnapshotsParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    repository: Option<&'b [&'b str]>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatSnapshots<'a, 'b> {
    #[doc = "Creates a new instance of [CatSnapshots] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatSnapshotsParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatSnapshots {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            ignore_unavailable: None,
            master_timeout: None,
            pretty: None,
            repository: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "When `true`, the response does not include information from unavailable snapshots."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "A comma-separated list of snapshot repositories used to limit the request.\nAccepts wildcard expressions.\n`_all` returns all repositories.\nIf any repository fails during the request, OpenSearch returns an error."]
    pub fn repository(mut self, repository: &'b [&'b str]) -> Self {
        self.repository = Some(repository);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Snapshots API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                repository: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                repository: self.repository,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Tasks API"]
pub enum CatTasksParts {
    #[doc = "No parts"]
    None,
}
impl CatTasksParts {
    #[doc = "Builds a relative URL path to the Cat Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatTasksParts::None => "/_cat/tasks".into(),
        }
    }
}
#[doc = "Builder for the [Cat Tasks API](https://opensearch.org/docs/latest/api-reference/cat/cat-tasks/)\n\nLists the progress of all tasks currently running on the cluster."]
#[derive(Clone, Debug)]
pub struct CatTasks<'a, 'b> {
    transport: &'a Transport,
    parts: CatTasksParts,
    actions: Option<&'b [&'b str]>,
    detailed: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    nodes: Option<&'b [&'b str]>,
    parent_task_id: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    time: Option<Time>,
    v: Option<bool>,
}
impl<'a, 'b> CatTasks<'a, 'b> {
    #[doc = "Creates a new instance of [CatTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatTasks {
            transport,
            parts: CatTasksParts::None,
            headers,
            actions: None,
            detailed: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            nodes: None,
            parent_task_id: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            time: None,
            v: None,
        }
    }
    #[doc = "The task action names used to limit the response."]
    pub fn actions(mut self, actions: &'b [&'b str]) -> Self {
        self.actions = Some(actions);
        self
    }
    #[doc = "If `true`, the response includes detailed information about shard recoveries."]
    pub fn detailed(mut self, detailed: bool) -> Self {
        self.detailed = Some(detailed);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "A comma-separated list of node IDs or names used to limit the returned information. \nUse `_local` to return information from the node to which you're connecting, specify a specific node from which to get information, or keep the parameter empty to get information from all nodes."]
    pub fn nodes(mut self, nodes: &'b [&'b str]) -> Self {
        self.nodes = Some(nodes);
        self
    }
    #[doc = "The parent task identifier, which is used to limit the response."]
    pub fn parent_task_id(mut self, parent_task_id: &'b str) -> Self {
        self.parent_task_id = Some(parent_task_id);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies the time units, for example, `5d` or `7h`. For more information, see [Supported units](https://opensearch.org/docs/latest/api-reference/units/)."]
    pub fn time(mut self, time: Time) -> Self {
        self.time = Some(time);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Tasks API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                actions: Option<&'b [&'b str]>,
                detailed: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                nodes: Option<&'b [&'b str]>,
                parent_task_id: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                time: Option<Time>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                actions: self.actions,
                detailed: self.detailed,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                nodes: self.nodes,
                parent_task_id: self.parent_task_id,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                time: self.time,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Templates API"]
pub enum CatTemplatesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> CatTemplatesParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Templates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatTemplatesParts::None => "/_cat/templates".into(),
            CatTemplatesParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_name.len());
                p.push_str("/_cat/templates/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Templates API](https://opensearch.org/docs/latest/api-reference/cat/cat-templates/)\n\nLists the names, patterns, order numbers, and version numbers of index templates."]
#[derive(Clone, Debug)]
pub struct CatTemplates<'a, 'b> {
    transport: &'a Transport,
    parts: CatTemplatesParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatTemplates<'a, 'b> {
    #[doc = "Creates a new instance of [CatTemplates] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatTemplatesParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatTemplates {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            source: None,
            v: None,
        }
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Templates API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cat Thread Pool API"]
pub enum CatThreadPoolParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ThreadPoolPatterns"]
    ThreadPoolPatterns(&'b [&'b str]),
}
impl<'b> CatThreadPoolParts<'b> {
    #[doc = "Builds a relative URL path to the Cat Thread Pool API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CatThreadPoolParts::None => "/_cat/thread_pool".into(),
            CatThreadPoolParts::ThreadPoolPatterns(thread_pool_patterns) => {
                let thread_pool_patterns_str = thread_pool_patterns.join(",");
                let encoded_thread_pool_patterns: Cow<str> =
                    percent_encode(thread_pool_patterns_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_thread_pool_patterns.len());
                p.push_str("/_cat/thread_pool/");
                p.push_str(encoded_thread_pool_patterns.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cat Thread Pool API](https://opensearch.org/docs/latest/api-reference/cat/cat-thread-pool/)\n\nReturns cluster-wide thread pool statistics per node.\nBy default the active, queued, and rejected statistics are returned for all thread pools."]
#[derive(Clone, Debug)]
pub struct CatThreadPool<'a, 'b> {
    transport: &'a Transport,
    parts: CatThreadPoolParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    h: Option<&'b [&'b str]>,
    headers: HeaderMap,
    help: Option<bool>,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    s: Option<&'b [&'b str]>,
    size: Option<i64>,
    source: Option<&'b str>,
    v: Option<bool>,
}
impl<'a, 'b> CatThreadPool<'a, 'b> {
    #[doc = "Creates a new instance of [CatThreadPool] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CatThreadPoolParts<'b>) -> Self {
        let mut headers = HeaderMap::with_capacity(2);
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
        headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
        CatThreadPool {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            format: None,
            h: None,
            help: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            s: None,
            size: None,
            source: None,
            v: None,
        }
    }
    #[doc = "A timeout for connection to the cluster manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "A short version of the `Accept` header, such as `json` or `yaml`."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
        self
    }
    #[doc = "A comma-separated list of column names to display."]
    pub fn h(mut self, h: &'b [&'b str]) -> Self {
        self.h = Some(h);
        self
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
        self.headers.insert(key, value);
        self
    }
    #[doc = "Returns help information."]
    pub fn help(mut self, help: bool) -> Self {
        self.help = Some(help);
        self
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(mut self, human: bool) -> Self {
        self.human = Some(human);
        self
    }
    #[doc = "Returns local information but does not retrieve the state from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "The amount of time allowed to establish a connection to the cluster manager node."]
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A comma-separated list of column names or column aliases to sort by."]
    pub fn s(mut self, s: &'b [&'b str]) -> Self {
        self.s = Some(s);
        self
    }
    #[doc = "The multiplier in which to display values."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Enables verbose mode, which displays column headers."]
    pub fn v(mut self, v: bool) -> Self {
        self.v = Some(v);
        self
    }
    #[doc = "Creates an asynchronous call to the Cat Thread Pool API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                format: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                h: Option<&'b [&'b str]>,
                help: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                s: Option<&'b [&'b str]>,
                size: Option<i64>,
                source: Option<&'b str>,
                v: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                h: self.h,
                help: self.help,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                s: self.s,
                size: self.size,
                source: self.source,
                v: self.v,
            };
            Some(query_params)
        };
        let body = Option::<()>::None;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[doc = "Namespace client for Cat APIs"]
pub struct Cat<'a> {
    transport: &'a Transport,
}
impl<'a> Cat<'a> {
    #[doc = "Creates a new instance of [Cat]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Cat Aliases API](https://opensearch.org/docs/latest/api-reference/cat/cat-aliases/)\n\nShows information about aliases currently configured to indexes, including filter and routing information."]
    pub fn aliases<'b>(&'a self, parts: CatAliasesParts<'b>) -> CatAliases<'a, 'b> {
        CatAliases::new(self.transport(), parts)
    }
    #[doc = "[Cat All Pit Segments API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/)\n\nLists all active CAT point-in-time segments."]
    pub fn all_pit_segments<'b>(&'a self) -> CatAllPitSegments<'a, 'b> {
        CatAllPitSegments::new(self.transport())
    }
    #[doc = "[Cat Allocation API](https://opensearch.org/docs/latest/api-reference/cat/cat-allocation/)\n\nProvides a snapshot of how many shards are allocated to each data node and how much disk space they are using."]
    pub fn allocation<'b>(&'a self, parts: CatAllocationParts<'b>) -> CatAllocation<'a, 'b> {
        CatAllocation::new(self.transport(), parts)
    }
    #[doc = "[Cat Cluster Manager API](https://opensearch.org/docs/latest/api-reference/cat/cat-cluster_manager/)\n\nReturns information about the cluster-manager node."]
    pub fn cluster_manager<'b>(&'a self) -> CatClusterManager<'a, 'b> {
        CatClusterManager::new(self.transport())
    }
    #[doc = "[Cat Count API](https://opensearch.org/docs/latest/api-reference/cat/cat-count/)\n\nProvides quick access to the document count of the entire cluster or of an individual index."]
    pub fn count<'b>(&'a self, parts: CatCountParts<'b>) -> CatCount<'a, 'b> {
        CatCount::new(self.transport(), parts)
    }
    #[doc = "[Cat Fielddata API](https://opensearch.org/docs/latest/api-reference/cat/cat-field-data/)\n\nShows how much heap memory is currently being used by field data on every data node in the cluster."]
    pub fn fielddata<'b>(&'a self, parts: CatFielddataParts<'b>) -> CatFielddata<'a, 'b> {
        CatFielddata::new(self.transport(), parts)
    }
    #[doc = "[Cat Health API](https://opensearch.org/docs/latest/api-reference/cat/cat-health/)\n\nReturns a concise representation of the cluster health."]
    pub fn health<'b>(&'a self) -> CatHealth<'a, 'b> {
        CatHealth::new(self.transport())
    }
    #[doc = "[Cat Help API](https://opensearch.org/docs/latest/api-reference/cat/index/)\n\nReturns help for the Cat APIs."]
    pub fn help<'b>(&'a self) -> CatHelp<'a, 'b> {
        CatHelp::new(self.transport())
    }
    #[doc = "[Cat Indices API](https://opensearch.org/docs/latest/api-reference/cat/cat-indices/)\n\nLists information related to indexes, that is, how much disk space they are using, how many shards they have, their health status, and so on."]
    pub fn indices<'b>(&'a self, parts: CatIndicesParts<'b>) -> CatIndices<'a, 'b> {
        CatIndices::new(self.transport(), parts)
    }
    #[doc = "[Cat Master API](https://opensearch.org/docs/latest/api-reference/cat/cat-cluster_manager/)\n\nReturns information about the cluster-manager node."]
    #[deprecated = "To promote inclusive language, use '/_cat/cluster_manager' instead."]
    #[allow(deprecated)]
    pub fn master<'b>(&'a self) -> CatMaster<'a, 'b> {
        CatMaster::new(self.transport())
    }
    #[doc = "[Cat Nodeattrs API](https://opensearch.org/docs/latest/api-reference/cat/cat-nodeattrs/)\n\nReturns information about custom node attributes."]
    pub fn nodeattrs<'b>(&'a self) -> CatNodeattrs<'a, 'b> {
        CatNodeattrs::new(self.transport())
    }
    #[doc = "[Cat Nodes API](https://opensearch.org/docs/latest/api-reference/cat/cat-nodes/)\n\nReturns basic statistics about the performance of cluster nodes."]
    pub fn nodes<'b>(&'a self) -> CatNodes<'a, 'b> {
        CatNodes::new(self.transport())
    }
    #[doc = "[Cat Pending Tasks API](https://opensearch.org/docs/latest/api-reference/cat/cat-pending-tasks/)\n\nReturns a concise representation of the cluster's pending tasks."]
    pub fn pending_tasks<'b>(&'a self) -> CatPendingTasks<'a, 'b> {
        CatPendingTasks::new(self.transport())
    }
    #[doc = "[Cat Pit Segments API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/)\n\nLists one or several CAT point-in-time segments."]
    pub fn pit_segments<'b>(&'a self) -> CatPitSegments<'a, 'b, ()> {
        CatPitSegments::new(self.transport())
    }
    #[doc = "[Cat Plugins API](https://opensearch.org/docs/latest/api-reference/cat/cat-plugins/)\n\nReturns information about the names, components, and versions of the installed plugins."]
    pub fn plugins<'b>(&'a self) -> CatPlugins<'a, 'b> {
        CatPlugins::new(self.transport())
    }
    #[doc = "[Cat Recovery API](https://opensearch.org/docs/latest/api-reference/cat/cat-plugins/)\n\nReturns all completed and ongoing index and shard recoveries."]
    pub fn recovery<'b>(&'a self, parts: CatRecoveryParts<'b>) -> CatRecovery<'a, 'b> {
        CatRecovery::new(self.transport(), parts)
    }
    #[doc = "[Cat Repositories API](https://opensearch.org/docs/latest/api-reference/cat/cat-repositories/)\n\nReturns information about all snapshot repositories for a cluster."]
    pub fn repositories<'b>(&'a self) -> CatRepositories<'a, 'b> {
        CatRepositories::new(self.transport())
    }
    #[doc = "[Cat Segment Replication API](https://opensearch.org/docs/latest/api-reference/cat/cat-segment-replication/)\n\nReturns information about active and last-completed segment replication events on each replica shard, including related shard-level metrics. \nThese metrics provide information about how far behind the primary shard the replicas are lagging."]
    pub fn segment_replication<'b>(
        &'a self,
        parts: CatSegmentReplicationParts<'b>,
    ) -> CatSegmentReplication<'a, 'b> {
        CatSegmentReplication::new(self.transport(), parts)
    }
    #[doc = "[Cat Segments API](https://opensearch.org/docs/latest/api-reference/cat/cat-segments/)\n\nProvides low-level information about the segments in the shards of an index."]
    pub fn segments<'b>(&'a self, parts: CatSegmentsParts<'b>) -> CatSegments<'a, 'b> {
        CatSegments::new(self.transport(), parts)
    }
    #[doc = "[Cat Shards API](https://opensearch.org/docs/latest/api-reference/cat/cat-shards/)\n\nLists the states of all primary and replica shards and how they are distributed."]
    pub fn shards<'b>(&'a self, parts: CatShardsParts<'b>) -> CatShards<'a, 'b> {
        CatShards::new(self.transport(), parts)
    }
    #[doc = "[Cat Snapshots API](https://opensearch.org/docs/latest/api-reference/cat/cat-snapshots/)\n\nLists all of the snapshots stored in a specific repository."]
    pub fn snapshots<'b>(&'a self, parts: CatSnapshotsParts<'b>) -> CatSnapshots<'a, 'b> {
        CatSnapshots::new(self.transport(), parts)
    }
    #[doc = "[Cat Tasks API](https://opensearch.org/docs/latest/api-reference/cat/cat-tasks/)\n\nLists the progress of all tasks currently running on the cluster."]
    pub fn tasks<'b>(&'a self) -> CatTasks<'a, 'b> {
        CatTasks::new(self.transport())
    }
    #[doc = "[Cat Templates API](https://opensearch.org/docs/latest/api-reference/cat/cat-templates/)\n\nLists the names, patterns, order numbers, and version numbers of index templates."]
    pub fn templates<'b>(&'a self, parts: CatTemplatesParts<'b>) -> CatTemplates<'a, 'b> {
        CatTemplates::new(self.transport(), parts)
    }
    #[doc = "[Cat Thread Pool API](https://opensearch.org/docs/latest/api-reference/cat/cat-thread-pool/)\n\nReturns cluster-wide thread pool statistics per node.\nBy default the active, queued, and rejected statistics are returned for all thread pools."]
    pub fn thread_pool<'b>(&'a self, parts: CatThreadPoolParts<'b>) -> CatThreadPool<'a, 'b> {
        CatThreadPool::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Cat APIs"]
    pub fn cat(&self) -> Cat {
        Cat::new(self.transport())
    }
}
