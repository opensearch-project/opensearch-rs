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
#[doc = "API parts for the Replication Autofollow Stats API"]
pub enum ReplicationAutofollowStatsParts {
    #[doc = "No parts"]
    None,
}
impl ReplicationAutofollowStatsParts {
    #[doc = "Builds a relative URL path to the Replication Autofollow Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationAutofollowStatsParts::None => {
                "/_plugins/_replication/autofollow_stats".into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Autofollow Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-auto-follow-stats)\n\nRetrieves information about any auto-follow activity and any replication rules configured on the specified cluster."]
#[derive(Clone, Debug)]
pub struct ReplicationAutofollowStats<'a, 'b> {
    transport: &'a Transport,
    parts: ReplicationAutofollowStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ReplicationAutofollowStats<'a, 'b> {
    #[doc = "Creates a new instance of [ReplicationAutofollowStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ReplicationAutofollowStats {
            transport,
            parts: ReplicationAutofollowStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Replication Autofollow Stats API that can be awaited"]
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
#[doc = "API parts for the Replication Create Replication Rule API"]
pub enum ReplicationCreateReplicationRuleParts {
    #[doc = "No parts"]
    None,
}
impl ReplicationCreateReplicationRuleParts {
    #[doc = "Builds a relative URL path to the Replication Create Replication Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationCreateReplicationRuleParts::None => {
                "/_plugins/_replication/_autofollow".into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Create Replication Rule API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#create-replication-rule)\n\nAutomatically starts the replication on indexes matching a specified pattern."]
#[derive(Clone, Debug)]
pub struct ReplicationCreateReplicationRule<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationCreateReplicationRuleParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationCreateReplicationRule<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationCreateReplicationRule]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ReplicationCreateReplicationRule {
            transport,
            parts: ReplicationCreateReplicationRuleParts::None,
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
    pub fn body<T>(self, body: T) -> ReplicationCreateReplicationRule<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationCreateReplicationRule {
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
    #[doc = "Creates an asynchronous call to the Replication Create Replication Rule API that can be awaited"]
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
#[doc = "API parts for the Replication Delete Replication Rule API"]
pub enum ReplicationDeleteReplicationRuleParts {
    #[doc = "No parts"]
    None,
}
impl ReplicationDeleteReplicationRuleParts {
    #[doc = "Builds a relative URL path to the Replication Delete Replication Rule API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationDeleteReplicationRuleParts::None => {
                "/_plugins/_replication/_autofollow".into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Delete Replication Rule API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#delete-replication-rule)\n\nDeletes the specified replication rule."]
#[derive(Clone, Debug)]
pub struct ReplicationDeleteReplicationRule<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationDeleteReplicationRuleParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationDeleteReplicationRule<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationDeleteReplicationRule]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ReplicationDeleteReplicationRule {
            transport,
            parts: ReplicationDeleteReplicationRuleParts::None,
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
    pub fn body<T>(self, body: T) -> ReplicationDeleteReplicationRule<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationDeleteReplicationRule {
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
    #[doc = "Creates an asynchronous call to the Replication Delete Replication Rule API that can be awaited"]
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
#[doc = "API parts for the Replication Follower Stats API"]
pub enum ReplicationFollowerStatsParts {
    #[doc = "No parts"]
    None,
}
impl ReplicationFollowerStatsParts {
    #[doc = "Builds a relative URL path to the Replication Follower Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationFollowerStatsParts::None => "/_plugins/_replication/follower_stats".into(),
        }
    }
}
#[doc = "Builder for the [Replication Follower Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-follower-cluster-stats)\n\nRetrieves information about any follower (syncing) indexes on a specified cluster."]
#[derive(Clone, Debug)]
pub struct ReplicationFollowerStats<'a, 'b> {
    transport: &'a Transport,
    parts: ReplicationFollowerStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ReplicationFollowerStats<'a, 'b> {
    #[doc = "Creates a new instance of [ReplicationFollowerStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ReplicationFollowerStats {
            transport,
            parts: ReplicationFollowerStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Replication Follower Stats API that can be awaited"]
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
#[doc = "API parts for the Replication Leader Stats API"]
pub enum ReplicationLeaderStatsParts {
    #[doc = "No parts"]
    None,
}
impl ReplicationLeaderStatsParts {
    #[doc = "Builds a relative URL path to the Replication Leader Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationLeaderStatsParts::None => "/_plugins/_replication/leader_stats".into(),
        }
    }
}
#[doc = "Builder for the [Replication Leader Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-leader-cluster-stats)\n\nRetrieves information about any replicated leader indexes on a specified cluster."]
#[derive(Clone, Debug)]
pub struct ReplicationLeaderStats<'a, 'b> {
    transport: &'a Transport,
    parts: ReplicationLeaderStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ReplicationLeaderStats<'a, 'b> {
    #[doc = "Creates a new instance of [ReplicationLeaderStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ReplicationLeaderStats {
            transport,
            parts: ReplicationLeaderStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Replication Leader Stats API that can be awaited"]
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
#[doc = "API parts for the Replication Pause API"]
pub enum ReplicationPauseParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationPauseParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Pause API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationPauseParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_pause");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Pause API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#pause-replication)\n\nPauses the replication of the leader index."]
#[derive(Clone, Debug)]
pub struct ReplicationPause<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationPauseParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationPause<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationPause] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationPauseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationPause {
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
    pub fn body<T>(self, body: T) -> ReplicationPause<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationPause {
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
    #[doc = "Creates an asynchronous call to the Replication Pause API that can be awaited"]
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
#[doc = "API parts for the Replication Resume API"]
pub enum ReplicationResumeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationResumeParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Resume API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationResumeParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_resume");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Resume API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#resume-replication)\n\nResumes replication of the leader index."]
#[derive(Clone, Debug)]
pub struct ReplicationResume<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationResumeParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationResume<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationResume] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationResumeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationResume {
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
    pub fn body<T>(self, body: T) -> ReplicationResume<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationResume {
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
    #[doc = "Creates an asynchronous call to the Replication Resume API that can be awaited"]
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
#[doc = "API parts for the Replication Start API"]
pub enum ReplicationStartParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationStartParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationStartParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Start API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#start-replication)\n\nInitiates the replication of an index from the leader cluster to the follower cluster."]
#[derive(Clone, Debug)]
pub struct ReplicationStart<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationStartParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationStart] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationStartParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationStart {
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
    pub fn body<T>(self, body: T) -> ReplicationStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationStart {
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
    #[doc = "Creates an asynchronous call to the Replication Start API that can be awaited"]
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
#[doc = "API parts for the Replication Status API"]
pub enum ReplicationStatusParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationStatusParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Status API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-replication-status)\n\nRetrieves the the status of an index replication."]
#[derive(Clone, Debug)]
pub struct ReplicationStatus<'a, 'b> {
    transport: &'a Transport,
    parts: ReplicationStatusParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ReplicationStatus<'a, 'b> {
    #[doc = "Creates a new instance of [ReplicationStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationStatus {
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
    #[doc = "Creates an asynchronous call to the Replication Status API that can be awaited"]
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
#[doc = "API parts for the Replication Stop API"]
pub enum ReplicationStopParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationStopParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationStopParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Stop API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#stop-replication)\n\nTerminates the replication and converts the follower index to a standard index."]
#[derive(Clone, Debug)]
pub struct ReplicationStop<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationStopParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationStop] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationStopParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationStop {
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
    pub fn body<T>(self, body: T) -> ReplicationStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationStop {
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
    #[doc = "Creates an asynchronous call to the Replication Stop API that can be awaited"]
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
#[doc = "API parts for the Replication Update Settings API"]
pub enum ReplicationUpdateSettingsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> ReplicationUpdateSettingsParts<'b> {
    #[doc = "Builds a relative URL path to the Replication Update Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReplicationUpdateSettingsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_index.len());
                p.push_str("/_plugins/_replication/");
                p.push_str(encoded_index.as_ref());
                p.push_str("/_update");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Replication Update Settings API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#update-settings)\n\nUpdates any settings on the follower index."]
#[derive(Clone, Debug)]
pub struct ReplicationUpdateSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReplicationUpdateSettingsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReplicationUpdateSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReplicationUpdateSettings] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReplicationUpdateSettingsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReplicationUpdateSettings {
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
    pub fn body<T>(self, body: T) -> ReplicationUpdateSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReplicationUpdateSettings {
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
    #[doc = "Creates an asynchronous call to the Replication Update Settings API that can be awaited"]
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
#[doc = "Namespace client for Replication APIs"]
pub struct Replication<'a> {
    transport: &'a Transport,
}
impl<'a> Replication<'a> {
    #[doc = "Creates a new instance of [Replication]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Replication Autofollow Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-auto-follow-stats)\n\nRetrieves information about any auto-follow activity and any replication rules configured on the specified cluster."]
    pub fn autofollow_stats<'b>(&'a self) -> ReplicationAutofollowStats<'a, 'b> {
        ReplicationAutofollowStats::new(self.transport())
    }
    #[doc = "[Replication Create Replication Rule API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#create-replication-rule)\n\nAutomatically starts the replication on indexes matching a specified pattern."]
    pub fn create_replication_rule<'b>(&'a self) -> ReplicationCreateReplicationRule<'a, 'b, ()> {
        ReplicationCreateReplicationRule::new(self.transport())
    }
    #[doc = "[Replication Delete Replication Rule API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#delete-replication-rule)\n\nDeletes the specified replication rule."]
    pub fn delete_replication_rule<'b>(&'a self) -> ReplicationDeleteReplicationRule<'a, 'b, ()> {
        ReplicationDeleteReplicationRule::new(self.transport())
    }
    #[doc = "[Replication Follower Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-follower-cluster-stats)\n\nRetrieves information about any follower (syncing) indexes on a specified cluster."]
    pub fn follower_stats<'b>(&'a self) -> ReplicationFollowerStats<'a, 'b> {
        ReplicationFollowerStats::new(self.transport())
    }
    #[doc = "[Replication Leader Stats API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-leader-cluster-stats)\n\nRetrieves information about any replicated leader indexes on a specified cluster."]
    pub fn leader_stats<'b>(&'a self) -> ReplicationLeaderStats<'a, 'b> {
        ReplicationLeaderStats::new(self.transport())
    }
    #[doc = "[Replication Pause API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#pause-replication)\n\nPauses the replication of the leader index."]
    pub fn pause<'b>(&'a self, parts: ReplicationPauseParts<'b>) -> ReplicationPause<'a, 'b, ()> {
        ReplicationPause::new(self.transport(), parts)
    }
    #[doc = "[Replication Resume API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#resume-replication)\n\nResumes replication of the leader index."]
    pub fn resume<'b>(
        &'a self,
        parts: ReplicationResumeParts<'b>,
    ) -> ReplicationResume<'a, 'b, ()> {
        ReplicationResume::new(self.transport(), parts)
    }
    #[doc = "[Replication Start API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#start-replication)\n\nInitiates the replication of an index from the leader cluster to the follower cluster."]
    pub fn start<'b>(&'a self, parts: ReplicationStartParts<'b>) -> ReplicationStart<'a, 'b, ()> {
        ReplicationStart::new(self.transport(), parts)
    }
    #[doc = "[Replication Status API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#get-replication-status)\n\nRetrieves the the status of an index replication."]
    pub fn status<'b>(&'a self, parts: ReplicationStatusParts<'b>) -> ReplicationStatus<'a, 'b> {
        ReplicationStatus::new(self.transport(), parts)
    }
    #[doc = "[Replication Stop API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#stop-replication)\n\nTerminates the replication and converts the follower index to a standard index."]
    pub fn stop<'b>(&'a self, parts: ReplicationStopParts<'b>) -> ReplicationStop<'a, 'b, ()> {
        ReplicationStop::new(self.transport(), parts)
    }
    #[doc = "[Replication Update Settings API](https://opensearch.org/docs/latest/tuning-your-cluster/replication-plugin/api/#update-settings)\n\nUpdates any settings on the follower index."]
    pub fn update_settings<'b>(
        &'a self,
        parts: ReplicationUpdateSettingsParts<'b>,
    ) -> ReplicationUpdateSettings<'a, 'b, ()> {
        ReplicationUpdateSettings::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Replication APIs"]
    pub fn replication(&self) -> Replication {
        Replication::new(self.transport())
    }
}
