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

//! Cluster APIs
//!
//! [Manage settings](https://docs.opensearch.org/latest/api-reference/cluster-settings/),
//! perform operations, and retrieve information about an OpenSearch cluster.

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
#[doc = "API parts for the Cluster Allocation Explain API"]
pub enum ClusterAllocationExplainParts {
    #[doc = "No parts"]
    None,
}
impl ClusterAllocationExplainParts {
    #[doc = "Builds a relative URL path to the Cluster Allocation Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterAllocationExplainParts::None => "/_cluster/allocation/explain".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Allocation Explain API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-allocation/)\n\nExplains how shards are allocated in the current cluster and provides an explanation for why unassigned shards can't be allocated to a node."]
#[derive(Clone, Debug)]
pub struct ClusterAllocationExplain<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterAllocationExplainParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    include_disk_info: Option<bool>,
    include_yes_decisions: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClusterAllocationExplain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterAllocationExplain]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterAllocationExplain {
            transport,
            parts: ClusterAllocationExplainParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            include_disk_info: None,
            include_yes_decisions: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterAllocationExplain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterAllocationExplain {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            include_disk_info: self.include_disk_info,
            include_yes_decisions: self.include_yes_decisions,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
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
    #[doc = "When `true`, returns information about disk usage and shard sizes."]
    pub fn include_disk_info(mut self, include_disk_info: bool) -> Self {
        self.include_disk_info = Some(include_disk_info);
        self
    }
    #[doc = "When `true`, returns any `YES` decisions in the allocation explanation. `YES` decisions indicate when a particular shard allocation attempt was successful for the given node."]
    pub fn include_yes_decisions(mut self, include_yes_decisions: bool) -> Self {
        self.include_yes_decisions = Some(include_yes_decisions);
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
    #[doc = "Creates an asynchronous call to the Cluster Allocation Explain API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = match self.body {
            Some(_) => Method::Post,
            None => Method::Get,
        };
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
                include_disk_info: Option<bool>,
                include_yes_decisions: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                include_disk_info: self.include_disk_info,
                include_yes_decisions: self.include_yes_decisions,
                pretty: self.pretty,
                source: self.source,
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
#[doc = "API parts for the Cluster Delete Component Template API"]
pub enum ClusterDeleteComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterDeleteComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Delete Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteComponentTemplateParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Delete Component Template API](https://opensearch.org/docs/latest)\n\nDeletes a component template."]
#[derive(Clone, Debug)]
pub struct ClusterDeleteComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterDeleteComponentTemplateParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterDeleteComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterDeleteComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterDeleteComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteComponentTemplate {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Delete Component Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster Delete Decommission Awareness API"]
pub enum ClusterDeleteDecommissionAwarenessParts {
    #[doc = "No parts"]
    None,
}
impl ClusterDeleteDecommissionAwarenessParts {
    #[doc = "Builds a relative URL path to the Cluster Delete Decommission Awareness API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteDecommissionAwarenessParts::None => {
                "/_cluster/decommission/awareness".into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Delete Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-decommissioning-and-recommissioning-a-zone)\n\nRecommissions a decommissioned zone."]
#[derive(Clone, Debug)]
pub struct ClusterDeleteDecommissionAwareness<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterDeleteDecommissionAwarenessParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterDeleteDecommissionAwareness<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterDeleteDecommissionAwareness]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteDecommissionAwareness {
            transport,
            parts: ClusterDeleteDecommissionAwarenessParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Delete Decommission Awareness API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
#[doc = "API parts for the Cluster Delete Voting Config Exclusions API"]
pub enum ClusterDeleteVotingConfigExclusionsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterDeleteVotingConfigExclusionsParts {
    #[doc = "Builds a relative URL path to the Cluster Delete Voting Config Exclusions API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteVotingConfigExclusionsParts::None => {
                "/_cluster/voting_config_exclusions".into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Delete Voting Config Exclusions API](https://opensearch.org/docs/latest)\n\nClears any cluster voting configuration exclusions."]
#[derive(Clone, Debug)]
pub struct ClusterDeleteVotingConfigExclusions<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterDeleteVotingConfigExclusionsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_removal: Option<bool>,
}
impl<'a, 'b> ClusterDeleteVotingConfigExclusions<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterDeleteVotingConfigExclusions]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteVotingConfigExclusions {
            transport,
            parts: ClusterDeleteVotingConfigExclusionsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_removal: None,
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
    #[doc = "Specifies whether to wait for all excluded nodes to be removed from the\ncluster before clearing the voting configuration exclusions list.\nWhen `true`, all excluded nodes are removed from\nthe cluster before this API takes any action. When `false`, the\nvoting configuration exclusions list is cleared even if some excluded\nnodes are still in the cluster."]
    pub fn wait_for_removal(mut self, wait_for_removal: bool) -> Self {
        self.wait_for_removal = Some(wait_for_removal);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Delete Voting Config Exclusions API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
                wait_for_removal: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                wait_for_removal: self.wait_for_removal,
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
#[doc = "API parts for the Cluster Delete Weighted Routing API"]
pub enum ClusterDeleteWeightedRoutingParts {
    #[doc = "No parts"]
    None,
}
impl ClusterDeleteWeightedRoutingParts {
    #[doc = "Builds a relative URL path to the Cluster Delete Weighted Routing API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterDeleteWeightedRoutingParts::None => "/_cluster/routing/awareness/weights".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Delete Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-deleting-weights)\n\nDelete weighted shard routing weights."]
#[derive(Clone, Debug)]
pub struct ClusterDeleteWeightedRouting<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterDeleteWeightedRoutingParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClusterDeleteWeightedRouting<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterDeleteWeightedRouting]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterDeleteWeightedRouting {
            transport,
            parts: ClusterDeleteWeightedRoutingParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterDeleteWeightedRouting<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterDeleteWeightedRouting {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
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
    #[doc = "Creates an asynchronous call to the Cluster Delete Weighted Routing API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cluster Exists Component Template API"]
pub enum ClusterExistsComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterExistsComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Exists Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterExistsComponentTemplateParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Exists Component Template API](https://opensearch.org/docs/latest)\n\nReturns information about whether a particular component template exist."]
#[derive(Clone, Debug)]
pub struct ClusterExistsComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterExistsComponentTemplateParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterExistsComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterExistsComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterExistsComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterExistsComponentTemplate {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "When `true`, the request retrieves information from the local node only.\nWhen `false`, information is retrieved from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Exists Component Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Cluster Get Component Template API"]
pub enum ClusterGetComponentTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterGetComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Get Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetComponentTemplateParts::None => "/_component_template".into(),
            ClusterGetComponentTemplateParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Get Component Template API](https://opensearch.org/docs/latest)\n\nReturns one or more component templates."]
#[derive(Clone, Debug)]
pub struct ClusterGetComponentTemplate<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetComponentTemplateParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterGetComponentTemplate<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterGetComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterGetComponentTemplate {
            transport,
            parts,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "Whether to return settings in the flat form, which can improve readability, especially for heavily nested settings.\nFor example, the flat form of `\"cluster\": { \"max_shards_per_node\": 500 }` is `\"cluster.max_shards_per_node\": \"500\"`."]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "When `true`, the request retrieves information from the local node only.\nWhen `false`, information is retrieved from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Get Component Template API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Cluster Get Decommission Awareness API"]
pub enum ClusterGetDecommissionAwarenessParts<'b> {
    #[doc = "AwarenessAttributeName"]
    AwarenessAttributeName(&'b str),
}
impl<'b> ClusterGetDecommissionAwarenessParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Get Decommission Awareness API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetDecommissionAwarenessParts::AwarenessAttributeName(
                awareness_attribute_name,
            ) => {
                let encoded_awareness_attribute_name: Cow<str> =
                    percent_encode(awareness_attribute_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_awareness_attribute_name.len());
                p.push_str("/_cluster/decommission/awareness/");
                p.push_str(encoded_awareness_attribute_name.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Get Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-getting-zone-decommission-status)\n\nRetrieves the decommission status for all zones."]
#[derive(Clone, Debug)]
pub struct ClusterGetDecommissionAwareness<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetDecommissionAwarenessParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterGetDecommissionAwareness<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetDecommissionAwareness] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterGetDecommissionAwarenessParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterGetDecommissionAwareness {
            transport,
            parts,
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
    #[doc = "Creates an asynchronous call to the Cluster Get Decommission Awareness API that can be awaited"]
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
#[doc = "API parts for the Cluster Get Settings API"]
pub enum ClusterGetSettingsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterGetSettingsParts {
    #[doc = "Builds a relative URL path to the Cluster Get Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetSettingsParts::None => "/_cluster/settings".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Get Settings API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-settings/)\n\nReturns cluster settings."]
#[derive(Clone, Debug)]
pub struct ClusterGetSettings<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetSettingsParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_defaults: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterGetSettings<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterGetSettings {
            transport,
            parts: ClusterGetSettingsParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            include_defaults: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "Whether to return settings in the flat form, which can improve readability, especially for heavily nested settings.\nFor example, the flat form of `\"cluster\": { \"max_shards_per_node\": 500 }` is `\"cluster.max_shards_per_node\": \"500\"`."]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "When `true`, returns default cluster settings from the local node."]
    pub fn include_defaults(mut self, include_defaults: bool) -> Self {
        self.include_defaults = Some(include_defaults);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Get Settings API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                include_defaults: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                include_defaults: self.include_defaults,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster Get Weighted Routing API"]
pub enum ClusterGetWeightedRoutingParts<'b> {
    #[doc = "Attribute"]
    Attribute(&'b str),
}
impl<'b> ClusterGetWeightedRoutingParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Get Weighted Routing API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterGetWeightedRoutingParts::Attribute(attribute) => {
                let encoded_attribute: Cow<str> =
                    percent_encode(attribute.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(36usize + encoded_attribute.len());
                p.push_str("/_cluster/routing/awareness/");
                p.push_str(encoded_attribute.as_ref());
                p.push_str("/weights");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Get Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-getting-weights-for-all-zones)\n\nFetches weighted shard routing weights."]
#[derive(Clone, Debug)]
pub struct ClusterGetWeightedRouting<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterGetWeightedRoutingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterGetWeightedRouting<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterGetWeightedRouting] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterGetWeightedRoutingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterGetWeightedRouting {
            transport,
            parts,
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
    #[doc = "Creates an asynchronous call to the Cluster Get Weighted Routing API that can be awaited"]
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
#[doc = "API parts for the Cluster Health API"]
pub enum ClusterHealthParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> ClusterHealthParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterHealthParts::None => "/_cluster/health".into(),
            ClusterHealthParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(17usize + encoded_index.len());
                p.push_str("/_cluster/health/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Health API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-health/)\n\nReturns basic information about the health of the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterHealth<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterHealthParts<'b>,
    awareness_attribute: Option<&'b str>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    level: Option<Level>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_events: Option<WaitForEvents>,
    wait_for_no_initializing_shards: Option<bool>,
    wait_for_no_relocating_shards: Option<bool>,
    wait_for_nodes: Option<&'b str>,
    wait_for_status: Option<WaitForStatus>,
}
impl<'a, 'b> ClusterHealth<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterHealth] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterHealthParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterHealth {
            transport,
            parts,
            headers,
            awareness_attribute: None,
            cluster_manager_timeout: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            level: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
            wait_for_events: None,
            wait_for_no_initializing_shards: None,
            wait_for_no_relocating_shards: None,
            wait_for_nodes: None,
            wait_for_status: None,
        }
    }
    #[doc = "The name of the awareness attribute for which to return the cluster health status (for example, `zone`). Applicable only if `level` is set to `awareness_attributes`."]
    pub fn awareness_attribute(mut self, awareness_attribute: &'b str) -> Self {
        self.awareness_attribute = Some(awareness_attribute);
        self
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    pub fn level(mut self, level: Level) -> Self {
        self.level = Some(level);
        self
    }
    #[doc = "Whether to return information from the local node only instead of from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    pub fn wait_for_events(mut self, wait_for_events: WaitForEvents) -> Self {
        self.wait_for_events = Some(wait_for_events);
        self
    }
    #[doc = "Whether to wait until there are no initializing shards in the cluster."]
    pub fn wait_for_no_initializing_shards(
        mut self,
        wait_for_no_initializing_shards: bool,
    ) -> Self {
        self.wait_for_no_initializing_shards = Some(wait_for_no_initializing_shards);
        self
    }
    #[doc = "Whether to wait until there are no relocating shards in the cluster."]
    pub fn wait_for_no_relocating_shards(mut self, wait_for_no_relocating_shards: bool) -> Self {
        self.wait_for_no_relocating_shards = Some(wait_for_no_relocating_shards);
        self
    }
    #[doc = "Waits until the specified number of nodes (`N`) is available. Accepts `&gt;=N`, `&lt;=N`, `&gt;N`, and `&lt;N`. You can also use `ge(N)`, `le(N)`, `gt(N)`, and `lt(N)` notation."]
    pub fn wait_for_nodes(mut self, wait_for_nodes: &'b str) -> Self {
        self.wait_for_nodes = Some(wait_for_nodes);
        self
    }
    #[doc = "Waits until the cluster health reaches the specified status or better."]
    pub fn wait_for_status(mut self, wait_for_status: WaitForStatus) -> Self {
        self.wait_for_status = Some(wait_for_status);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Health API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                awareness_attribute: Option<&'b str>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                level: Option<Level>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
                wait_for_events: Option<WaitForEvents>,
                wait_for_no_initializing_shards: Option<bool>,
                wait_for_no_relocating_shards: Option<bool>,
                wait_for_nodes: Option<&'b str>,
                wait_for_status: Option<WaitForStatus>,
            }
            let query_params = QueryParams {
                awareness_attribute: self.awareness_attribute,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                level: self.level,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_events: self.wait_for_events,
                wait_for_no_initializing_shards: self.wait_for_no_initializing_shards,
                wait_for_no_relocating_shards: self.wait_for_no_relocating_shards,
                wait_for_nodes: self.wait_for_nodes,
                wait_for_status: self.wait_for_status,
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
#[doc = "API parts for the Cluster Pending Tasks API"]
pub enum ClusterPendingTasksParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPendingTasksParts {
    #[doc = "Builds a relative URL path to the Cluster Pending Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPendingTasksParts::None => "/_cluster/pending_tasks".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Pending Tasks API](https://opensearch.org/docs/latest)\n\nReturns a list of pending cluster-level tasks, such as index creation, mapping updates,\nor new allocations."]
#[derive(Clone, Debug)]
pub struct ClusterPendingTasks<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterPendingTasksParts,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterPendingTasks<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterPendingTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPendingTasks {
            transport,
            parts: ClusterPendingTasksParts::None,
            headers,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "When `true`, the request retrieves information from the local node only.\nWhen `false`, information is retrieved from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Pending Tasks API that can be awaited"]
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
                human: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                local: self.local,
                master_timeout: self.master_timeout,
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
#[doc = "API parts for the Cluster Post Voting Config Exclusions API"]
pub enum ClusterPostVotingConfigExclusionsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPostVotingConfigExclusionsParts {
    #[doc = "Builds a relative URL path to the Cluster Post Voting Config Exclusions API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPostVotingConfigExclusionsParts::None => {
                "/_cluster/voting_config_exclusions".into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Post Voting Config Exclusions API](https://opensearch.org/docs/latest)\n\nUpdates the cluster voting configuration by excluding certain node IDs or names."]
#[derive(Clone, Debug)]
pub struct ClusterPostVotingConfigExclusions<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPostVotingConfigExclusionsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    node_ids: Option<&'b [&'b str]>,
    node_names: Option<&'b [&'b str]>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPostVotingConfigExclusions<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPostVotingConfigExclusions]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPostVotingConfigExclusions {
            transport,
            parts: ClusterPostVotingConfigExclusionsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            node_ids: None,
            node_names: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPostVotingConfigExclusions<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPostVotingConfigExclusions {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            node_ids: self.node_ids,
            node_names: self.node_names,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
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
    #[doc = "A comma-separated list of node IDs to exclude\nfrom the voting configuration. When using this setting, you cannot also specify `node_names`. Either `node_ids` or `node_names` are required to receive a valid response."]
    pub fn node_ids(mut self, node_ids: &'b [&'b str]) -> Self {
        self.node_ids = Some(node_ids);
        self
    }
    #[doc = "A comma-separated list of node names to exclude from the\nvoting configuration. When using this setting, you cannot also specify `node_ids`. Either `node_ids` or `node_names` are required to receive a valid response."]
    pub fn node_names(mut self, node_names: &'b [&'b str]) -> Self {
        self.node_names = Some(node_names);
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
    #[doc = "When adding a voting configuration exclusion, the API waits for the\nspecified nodes to be excluded from the voting configuration before\nreturning a response. If the timeout expires before the appropriate condition\nis satisfied, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Post Voting Config Exclusions API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
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
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                node_ids: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                node_names: Option<&'b [&'b str]>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                node_ids: self.node_ids,
                node_names: self.node_names,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster Put Component Template API"]
pub enum ClusterPutComponentTemplateParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> ClusterPutComponentTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Put Component Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutComponentTemplateParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_component_template/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Put Component Template API](https://opensearch.org/docs/latest/im-plugin/index-templates/#use-component-templates-to-create-an-index-template)\n\nCreates or updates a component template."]
#[derive(Clone, Debug)]
pub struct ClusterPutComponentTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutComponentTemplateParts<'b>,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    create: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPutComponentTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutComponentTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterPutComponentTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterPutComponentTemplate {
            transport,
            parts,
            headers,
            body: None,
            cluster_manager_timeout: None,
            create: None,
            error_trace: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPutComponentTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutComponentTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            create: self.create,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "When `true`, this request cannot replace or update existing component templates."]
    pub fn create(mut self, create: bool) -> Self {
        self.create = Some(create);
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Put Component Template API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                create: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                create: self.create,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster Put Decommission Awareness API"]
pub enum ClusterPutDecommissionAwarenessParts<'b> {
    #[doc = "AwarenessAttributeName and AwarenessAttributeValue"]
    AwarenessAttributeNameAwarenessAttributeValue(&'b str, &'b str),
}
impl<'b> ClusterPutDecommissionAwarenessParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Put Decommission Awareness API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutDecommissionAwarenessParts::AwarenessAttributeNameAwarenessAttributeValue(
                awareness_attribute_name,
                awareness_attribute_value,
            ) => {
                let encoded_awareness_attribute_name: Cow<str> =
                    percent_encode(awareness_attribute_name.as_bytes(), PARTS_ENCODED).into();
                let encoded_awareness_attribute_value: Cow<str> =
                    percent_encode(awareness_attribute_value.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    34usize
                        + encoded_awareness_attribute_name.len()
                        + encoded_awareness_attribute_value.len(),
                );
                p.push_str("/_cluster/decommission/awareness/");
                p.push_str(encoded_awareness_attribute_name.as_ref());
                p.push('/');
                p.push_str(encoded_awareness_attribute_value.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Put Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-decommissioning-and-recommissioning-a-zone)\n\nDecommissions a cluster zone based on awareness. This can greatly benefit multi-zone deployments, where awareness attributes can aid in applying new upgrades to a cluster in a controlled fashion."]
#[derive(Clone, Debug)]
pub struct ClusterPutDecommissionAwareness<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutDecommissionAwarenessParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPutDecommissionAwareness<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutDecommissionAwareness] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterPutDecommissionAwarenessParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterPutDecommissionAwareness {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPutDecommissionAwareness<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutDecommissionAwareness {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
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
    #[doc = "Creates an asynchronous call to the Cluster Put Decommission Awareness API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cluster Put Settings API"]
pub enum ClusterPutSettingsParts {
    #[doc = "No parts"]
    None,
}
impl ClusterPutSettingsParts {
    #[doc = "Builds a relative URL path to the Cluster Put Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutSettingsParts::None => "/_cluster/settings".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Put Settings API](https://opensearch.org/docs/latest/api-reference/cluster-settings/)\n\nUpdates the cluster settings."]
#[derive(Clone, Debug)]
pub struct ClusterPutSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutSettingsParts,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPutSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterPutSettings {
            transport,
            parts: ClusterPutSettingsParts::None,
            headers,
            body: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPutSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutSettings {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            flat_settings: self.flat_settings,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "Whether to return settings in the flat form, which can improve readability, especially for heavily nested settings.\nFor example, the flat form of `\"cluster\": { \"max_shards_per_node\": 500 }` is `\"cluster.max_shards_per_node\": \"500\"`."]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Put Settings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster Put Weighted Routing API"]
pub enum ClusterPutWeightedRoutingParts<'b> {
    #[doc = "Attribute"]
    Attribute(&'b str),
}
impl<'b> ClusterPutWeightedRoutingParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Put Weighted Routing API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterPutWeightedRoutingParts::Attribute(attribute) => {
                let encoded_attribute: Cow<str> =
                    percent_encode(attribute.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(36usize + encoded_attribute.len());
                p.push_str("/_cluster/routing/awareness/");
                p.push_str(encoded_attribute.as_ref());
                p.push_str("/weights");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Put Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-weighted-round-robin-search)\n\nUpdates weighted shard routing weights."]
#[derive(Clone, Debug)]
pub struct ClusterPutWeightedRouting<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterPutWeightedRoutingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClusterPutWeightedRouting<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterPutWeightedRouting] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterPutWeightedRoutingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterPutWeightedRouting {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterPutWeightedRouting<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterPutWeightedRouting {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
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
    #[doc = "Creates an asynchronous call to the Cluster Put Weighted Routing API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Put;
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Cluster Remote Info API"]
pub enum ClusterRemoteInfoParts {
    #[doc = "No parts"]
    None,
}
impl ClusterRemoteInfoParts {
    #[doc = "Builds a relative URL path to the Cluster Remote Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterRemoteInfoParts::None => "/_remote/info".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Remote Info API](https://opensearch.org/docs/latest/api-reference/remote-info/)\n\nReturns the information about configured remote clusters."]
#[derive(Clone, Debug)]
pub struct ClusterRemoteInfo<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterRemoteInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ClusterRemoteInfo<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterRemoteInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterRemoteInfo {
            transport,
            parts: ClusterRemoteInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Cluster Remote Info API that can be awaited"]
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
#[doc = "API parts for the Cluster Reroute API"]
pub enum ClusterRerouteParts {
    #[doc = "No parts"]
    None,
}
impl ClusterRerouteParts {
    #[doc = "Builds a relative URL path to the Cluster Reroute API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterRerouteParts::None => "/_cluster/reroute".into(),
        }
    }
}
#[doc = "Builder for the [Cluster Reroute API](https://opensearch.org/docs/latest)\n\nAllows to manually change the allocation of individual shards in the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterReroute<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClusterRerouteParts,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    dry_run: Option<bool>,
    error_trace: Option<bool>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    metric: Option<Metric>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    retry_failed: Option<bool>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> ClusterReroute<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClusterReroute]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ClusterReroute {
            transport,
            parts: ClusterRerouteParts::None,
            headers,
            body: None,
            cluster_manager_timeout: None,
            dry_run: None,
            error_trace: None,
            explain: None,
            filter_path: None,
            human: None,
            master_timeout: None,
            metric: None,
            pretty: None,
            request_timeout: None,
            retry_failed: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ClusterReroute<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClusterReroute {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            dry_run: self.dry_run,
            error_trace: self.error_trace,
            explain: self.explain,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            master_timeout: self.master_timeout,
            metric: self.metric,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            retry_failed: self.retry_failed,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "When `true`, the request simulates the operation and returns the resulting state."]
    pub fn dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = Some(dry_run);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "When `true`, the response contains an explanation of why reroute certain commands can or cannot be executed."]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
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
    #[deprecated = "To promote inclusive language, use `cluster_manager_timeout` instead."]
    pub fn master_timeout(mut self, master_timeout: &'b str) -> Self {
        self.master_timeout = Some(master_timeout);
        self
    }
    #[doc = "Limits the information returned to the specified metrics."]
    pub fn metric(mut self, metric: Metric) -> Self {
        self.metric = Some(metric);
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
    #[doc = "When `true`, retries shard allocation if it was blocked because of too many subsequent failures."]
    pub fn retry_failed(mut self, retry_failed: bool) -> Self {
        self.retry_failed = Some(retry_failed);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Reroute API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cluster_manager_timeout: Option<&'b str>,
                dry_run: Option<bool>,
                error_trace: Option<bool>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                master_timeout: Option<&'b str>,
                metric: Option<Metric>,
                pretty: Option<bool>,
                retry_failed: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                dry_run: self.dry_run,
                error_trace: self.error_trace,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                master_timeout: self.master_timeout,
                metric: self.metric,
                pretty: self.pretty,
                retry_failed: self.retry_failed,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "API parts for the Cluster State API"]
pub enum ClusterStateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Metric"]
    Metric(&'b [&'b str]),
    #[doc = "Metric and Index"]
    MetricIndex(&'b [&'b str], &'b [&'b str]),
}
impl<'b> ClusterStateParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster State API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStateParts::None => "/_cluster/state".into(),
            ClusterStateParts::Metric(metric) => {
                let metric_str = metric.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_metric.len());
                p.push_str("/_cluster/state/");
                p.push_str(encoded_metric.as_ref());
                p.into()
            }
            ClusterStateParts::MetricIndex(metric, index) => {
                let metric_str = metric.join(",");
                let index_str = index.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(17usize + encoded_metric.len() + encoded_index.len());
                p.push_str("/_cluster/state/");
                p.push_str(encoded_metric.as_ref());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster State API](https://opensearch.org/docs/latest)\n\nReturns comprehensive information about the state of the cluster."]
#[derive(Clone, Debug)]
pub struct ClusterState<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterStateParts<'b>,
    allow_no_indices: Option<bool>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_metadata_version: Option<i64>,
    wait_for_timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterState<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterState] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterStateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterState {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            cluster_manager_timeout: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            master_timeout: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_metadata_version: None,
            wait_for_timeout: None,
        }
    }
    #[doc = "Whether to ignore a wildcard index expression that resolves into no concrete indexes. This includes the `_all` string or when no indexes have been specified."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The amount of time to wait for a response from the cluster manager node. For more information about supported time units, see [Common parameters](https://opensearch.org/docs/latest/api-reference/common-parameters/#time-units)."]
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
    #[doc = "Whether to return settings in the flat form, which can improve readability, especially for heavily nested settings.\nFor example, the flat form of `\"cluster\": { \"max_shards_per_node\": 500 }` is `\"cluster.max_shards_per_node\": \"500\"`."]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "Whether the specified concrete indexes should be ignored when unavailable (missing or closed)."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Whether to return information from the local node only instead of from the cluster manager node."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Wait for the metadata version to be equal or greater than the specified metadata version."]
    pub fn wait_for_metadata_version(mut self, wait_for_metadata_version: i64) -> Self {
        self.wait_for_metadata_version = Some(wait_for_metadata_version);
        self
    }
    #[doc = "The maximum time to wait for `wait_for_metadata_version` before timing out."]
    pub fn wait_for_timeout(mut self, wait_for_timeout: &'b str) -> Self {
        self.wait_for_timeout = Some(wait_for_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster State API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_no_indices: Option<bool>,
                cluster_manager_timeout: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                flat_settings: Option<bool>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                local: Option<bool>,
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_metadata_version: Option<i64>,
                wait_for_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                master_timeout: self.master_timeout,
                pretty: self.pretty,
                source: self.source,
                wait_for_metadata_version: self.wait_for_metadata_version,
                wait_for_timeout: self.wait_for_timeout,
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
#[doc = "API parts for the Cluster Stats API"]
pub enum ClusterStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "Metric and NodeId"]
    MetricNodeId(&'b [&'b str], &'b [&'b str]),
    #[doc = "Metric, IndexMetric and NodeId"]
    MetricIndexMetricNodeId(&'b [&'b str], &'b [&'b str], &'b [&'b str]),
}
impl<'b> ClusterStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Cluster Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClusterStatsParts::None => "/_cluster/stats".into(),
            ClusterStatsParts::NodeId(node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_node_id.len());
                p.push_str("/_cluster/stats/nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
            ClusterStatsParts::MetricNodeId(metric, node_id) => {
                let metric_str = metric.join(",");
                let node_id_str = node_id.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(23usize + encoded_metric.len() + encoded_node_id.len());
                p.push_str("/_cluster/stats/");
                p.push_str(encoded_metric.as_ref());
                p.push_str("/nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
            ClusterStatsParts::MetricIndexMetricNodeId(metric, index_metric, node_id) => {
                let metric_str = metric.join(",");
                let index_metric_str = index_metric.join(",");
                let node_id_str = node_id.join(",");
                let encoded_metric: Cow<str> =
                    percent_encode(metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_index_metric: Cow<str> =
                    percent_encode(index_metric_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    24usize
                        + encoded_metric.len()
                        + encoded_index_metric.len()
                        + encoded_node_id.len(),
                );
                p.push_str("/_cluster/stats/");
                p.push_str(encoded_metric.as_ref());
                p.push('/');
                p.push_str(encoded_index_metric.as_ref());
                p.push_str("/nodes/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Cluster Stats API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-stats/)\n\nReturns a high-level overview of cluster statistics."]
#[derive(Clone, Debug)]
pub struct ClusterStats<'a, 'b> {
    transport: &'a Transport,
    parts: ClusterStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_settings: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> ClusterStats<'a, 'b> {
    #[doc = "Creates a new instance of [ClusterStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClusterStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClusterStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_settings: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
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
    #[doc = "Whether to return settings in the flat form, which can improve readability, especially for heavily nested settings.\nFor example, the flat form of `\"cluster\": { \"max_shards_per_node\": 500 }` is `\"cluster.max_shards_per_node\": \"500\"`."]
    pub fn flat_settings(mut self, flat_settings: bool) -> Self {
        self.flat_settings = Some(flat_settings);
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
    #[doc = "The amount of time to wait for each node to respond.\nIf a node does not respond before its timeout expires, the response does not include its stats.\nHowever, timed out nodes are included in the response's `_nodes.failed` property. Defaults to no timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Cluster Stats API that can be awaited"]
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
                flat_settings: Option<bool>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_settings: self.flat_settings,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                timeout: self.timeout,
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
#[doc = "Namespace client for Cluster APIs"]
pub struct Cluster<'a> {
    transport: &'a Transport,
}
impl<'a> Cluster<'a> {
    #[doc = "Creates a new instance of [Cluster]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Cluster Allocation Explain API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-allocation/)\n\nExplains how shards are allocated in the current cluster and provides an explanation for why unassigned shards can't be allocated to a node."]
    pub fn allocation_explain<'b>(&'a self) -> ClusterAllocationExplain<'a, 'b, ()> {
        ClusterAllocationExplain::new(self.transport())
    }
    #[doc = "[Cluster Delete Component Template API](https://opensearch.org/docs/latest)\n\nDeletes a component template."]
    pub fn delete_component_template<'b>(
        &'a self,
        parts: ClusterDeleteComponentTemplateParts<'b>,
    ) -> ClusterDeleteComponentTemplate<'a, 'b> {
        ClusterDeleteComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Delete Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-decommissioning-and-recommissioning-a-zone)\n\nRecommissions a decommissioned zone."]
    pub fn delete_decommission_awareness<'b>(
        &'a self,
    ) -> ClusterDeleteDecommissionAwareness<'a, 'b> {
        ClusterDeleteDecommissionAwareness::new(self.transport())
    }
    #[doc = "[Cluster Delete Voting Config Exclusions API](https://opensearch.org/docs/latest)\n\nClears any cluster voting configuration exclusions."]
    pub fn delete_voting_config_exclusions<'b>(
        &'a self,
    ) -> ClusterDeleteVotingConfigExclusions<'a, 'b> {
        ClusterDeleteVotingConfigExclusions::new(self.transport())
    }
    #[doc = "[Cluster Delete Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-deleting-weights)\n\nDelete weighted shard routing weights."]
    pub fn delete_weighted_routing<'b>(&'a self) -> ClusterDeleteWeightedRouting<'a, 'b, ()> {
        ClusterDeleteWeightedRouting::new(self.transport())
    }
    #[doc = "[Cluster Exists Component Template API](https://opensearch.org/docs/latest)\n\nReturns information about whether a particular component template exist."]
    pub fn exists_component_template<'b>(
        &'a self,
        parts: ClusterExistsComponentTemplateParts<'b>,
    ) -> ClusterExistsComponentTemplate<'a, 'b> {
        ClusterExistsComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Get Component Template API](https://opensearch.org/docs/latest)\n\nReturns one or more component templates."]
    pub fn get_component_template<'b>(
        &'a self,
        parts: ClusterGetComponentTemplateParts<'b>,
    ) -> ClusterGetComponentTemplate<'a, 'b> {
        ClusterGetComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Get Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-getting-zone-decommission-status)\n\nRetrieves the decommission status for all zones."]
    pub fn get_decommission_awareness<'b>(
        &'a self,
        parts: ClusterGetDecommissionAwarenessParts<'b>,
    ) -> ClusterGetDecommissionAwareness<'a, 'b> {
        ClusterGetDecommissionAwareness::new(self.transport(), parts)
    }
    #[doc = "[Cluster Get Settings API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-settings/)\n\nReturns cluster settings."]
    pub fn get_settings<'b>(&'a self) -> ClusterGetSettings<'a, 'b> {
        ClusterGetSettings::new(self.transport())
    }
    #[doc = "[Cluster Get Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-getting-weights-for-all-zones)\n\nFetches weighted shard routing weights."]
    pub fn get_weighted_routing<'b>(
        &'a self,
        parts: ClusterGetWeightedRoutingParts<'b>,
    ) -> ClusterGetWeightedRouting<'a, 'b> {
        ClusterGetWeightedRouting::new(self.transport(), parts)
    }
    #[doc = "[Cluster Health API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-health/)\n\nReturns basic information about the health of the cluster."]
    pub fn health<'b>(&'a self, parts: ClusterHealthParts<'b>) -> ClusterHealth<'a, 'b> {
        ClusterHealth::new(self.transport(), parts)
    }
    #[doc = "[Cluster Pending Tasks API](https://opensearch.org/docs/latest)\n\nReturns a list of pending cluster-level tasks, such as index creation, mapping updates,\nor new allocations."]
    pub fn pending_tasks<'b>(&'a self) -> ClusterPendingTasks<'a, 'b> {
        ClusterPendingTasks::new(self.transport())
    }
    #[doc = "[Cluster Post Voting Config Exclusions API](https://opensearch.org/docs/latest)\n\nUpdates the cluster voting configuration by excluding certain node IDs or names."]
    pub fn post_voting_config_exclusions<'b>(
        &'a self,
    ) -> ClusterPostVotingConfigExclusions<'a, 'b, ()> {
        ClusterPostVotingConfigExclusions::new(self.transport())
    }
    #[doc = "[Cluster Put Component Template API](https://opensearch.org/docs/latest/im-plugin/index-templates/#use-component-templates-to-create-an-index-template)\n\nCreates or updates a component template."]
    pub fn put_component_template<'b>(
        &'a self,
        parts: ClusterPutComponentTemplateParts<'b>,
    ) -> ClusterPutComponentTemplate<'a, 'b, ()> {
        ClusterPutComponentTemplate::new(self.transport(), parts)
    }
    #[doc = "[Cluster Put Decommission Awareness API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-decommission/#example-decommissioning-and-recommissioning-a-zone)\n\nDecommissions a cluster zone based on awareness. This can greatly benefit multi-zone deployments, where awareness attributes can aid in applying new upgrades to a cluster in a controlled fashion."]
    pub fn put_decommission_awareness<'b>(
        &'a self,
        parts: ClusterPutDecommissionAwarenessParts<'b>,
    ) -> ClusterPutDecommissionAwareness<'a, 'b, ()> {
        ClusterPutDecommissionAwareness::new(self.transport(), parts)
    }
    #[doc = "[Cluster Put Settings API](https://opensearch.org/docs/latest/api-reference/cluster-settings/)\n\nUpdates the cluster settings."]
    pub fn put_settings<'b>(&'a self) -> ClusterPutSettings<'a, 'b, ()> {
        ClusterPutSettings::new(self.transport())
    }
    #[doc = "[Cluster Put Weighted Routing API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-awareness/#example-weighted-round-robin-search)\n\nUpdates weighted shard routing weights."]
    pub fn put_weighted_routing<'b>(
        &'a self,
        parts: ClusterPutWeightedRoutingParts<'b>,
    ) -> ClusterPutWeightedRouting<'a, 'b, ()> {
        ClusterPutWeightedRouting::new(self.transport(), parts)
    }
    #[doc = "[Cluster Remote Info API](https://opensearch.org/docs/latest/api-reference/remote-info/)\n\nReturns the information about configured remote clusters."]
    pub fn remote_info<'b>(&'a self) -> ClusterRemoteInfo<'a, 'b> {
        ClusterRemoteInfo::new(self.transport())
    }
    #[doc = "[Cluster Reroute API](https://opensearch.org/docs/latest)\n\nAllows to manually change the allocation of individual shards in the cluster."]
    pub fn reroute<'b>(&'a self) -> ClusterReroute<'a, 'b, ()> {
        ClusterReroute::new(self.transport())
    }
    #[doc = "[Cluster State API](https://opensearch.org/docs/latest)\n\nReturns comprehensive information about the state of the cluster."]
    pub fn state<'b>(&'a self, parts: ClusterStateParts<'b>) -> ClusterState<'a, 'b> {
        ClusterState::new(self.transport(), parts)
    }
    #[doc = "[Cluster Stats API](https://opensearch.org/docs/latest/api-reference/cluster-api/cluster-stats/)\n\nReturns a high-level overview of cluster statistics."]
    pub fn stats<'b>(&'a self, parts: ClusterStatsParts<'b>) -> ClusterStats<'a, 'b> {
        ClusterStats::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Cluster APIs"]
    pub fn cluster(&self) -> Cluster {
        Cluster::new(self.transport())
    }
}
