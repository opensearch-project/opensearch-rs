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
#[doc = "API parts for the Ppl Explain API"]
pub enum PplExplainParts {
    #[doc = "No parts"]
    None,
}
impl PplExplainParts {
    #[doc = "Builds a relative URL path to the Ppl Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PplExplainParts::None => "/_opendistro/_ppl/_explain".into(),
        }
    }
}
#[doc = "Builder for the [Ppl Explain API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nReturns the execution plan for a PPL query."]
#[derive(Clone, Debug)]
pub struct PplExplain<'a, 'b, B> {
    transport: &'a Transport,
    parts: PplExplainParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    sanitize: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> PplExplain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [PplExplain]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        PplExplain {
            transport,
            parts: PplExplainParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            sanitize: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> PplExplain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        PplExplain {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            format: self.format,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            sanitize: self.sanitize,
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
    #[doc = "Specifies the response format (JSON, YAML)."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
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
    #[doc = "Whether to escape special characters in the results."]
    pub fn sanitize(mut self, sanitize: bool) -> Self {
        self.sanitize = Some(sanitize);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ppl Explain API that can be awaited"]
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
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                sanitize: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                pretty: self.pretty,
                sanitize: self.sanitize,
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
#[doc = "API parts for the Ppl Get Stats API"]
pub enum PplGetStatsParts {
    #[doc = "No parts"]
    None,
}
impl PplGetStatsParts {
    #[doc = "Builds a relative URL path to the Ppl Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PplGetStatsParts::None => "/_opendistro/_ppl/stats".into(),
        }
    }
}
#[doc = "Builder for the [Ppl Get Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves performance metrics for the PPL plugin."]
#[derive(Clone, Debug)]
pub struct PplGetStats<'a, 'b> {
    transport: &'a Transport,
    parts: PplGetStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    sanitize: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> PplGetStats<'a, 'b> {
    #[doc = "Creates a new instance of [PplGetStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        PplGetStats {
            transport,
            parts: PplGetStatsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            sanitize: None,
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
    #[doc = "Specifies the response format (JSON, YAML)."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
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
    #[doc = "Whether to escape special characters in the results."]
    pub fn sanitize(mut self, sanitize: bool) -> Self {
        self.sanitize = Some(sanitize);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ppl Get Stats API that can be awaited"]
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
                human: Option<bool>,
                pretty: Option<bool>,
                sanitize: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                pretty: self.pretty,
                sanitize: self.sanitize,
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
#[doc = "API parts for the Ppl Post Stats API"]
pub enum PplPostStatsParts {
    #[doc = "No parts"]
    None,
}
impl PplPostStatsParts {
    #[doc = "Builds a relative URL path to the Ppl Post Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PplPostStatsParts::None => "/_opendistro/_ppl/stats".into(),
        }
    }
}
#[doc = "Builder for the [Ppl Post Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves filtered performance metrics for the PPL plugin."]
#[derive(Clone, Debug)]
pub struct PplPostStats<'a, 'b, B> {
    transport: &'a Transport,
    parts: PplPostStatsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    sanitize: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> PplPostStats<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [PplPostStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        PplPostStats {
            transport,
            parts: PplPostStatsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            sanitize: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> PplPostStats<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        PplPostStats {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            format: self.format,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            sanitize: self.sanitize,
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
    #[doc = "Specifies the response format (JSON, YAML)."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
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
    #[doc = "Whether to escape special characters in the results."]
    pub fn sanitize(mut self, sanitize: bool) -> Self {
        self.sanitize = Some(sanitize);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ppl Post Stats API that can be awaited"]
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
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                sanitize: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                pretty: self.pretty,
                sanitize: self.sanitize,
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
#[doc = "API parts for the Ppl Query API"]
pub enum PplQueryParts {
    #[doc = "No parts"]
    None,
}
impl PplQueryParts {
    #[doc = "Builds a relative URL path to the Ppl Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PplQueryParts::None => "/_opendistro/_ppl".into(),
        }
    }
}
#[doc = "Builder for the [Ppl Query API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nExecutes a PPL query against OpenSearch indexes."]
#[derive(Clone, Debug)]
pub struct PplQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: PplQueryParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    sanitize: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> PplQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [PplQuery]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        PplQuery {
            transport,
            parts: PplQueryParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            sanitize: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> PplQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        PplQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            format: self.format,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            sanitize: self.sanitize,
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
    #[doc = "Specifies the response format (JSON OR YAML)."]
    pub fn format(mut self, format: &'b str) -> Self {
        self.format = Some(format);
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
    #[doc = "Whether to sanitize special characters in the results."]
    pub fn sanitize(mut self, sanitize: bool) -> Self {
        self.sanitize = Some(sanitize);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ppl Query API that can be awaited"]
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
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                sanitize: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
                human: self.human,
                pretty: self.pretty,
                sanitize: self.sanitize,
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
#[doc = "Namespace client for Ppl APIs"]
pub struct Ppl<'a> {
    transport: &'a Transport,
}
impl<'a> Ppl<'a> {
    #[doc = "Creates a new instance of [Ppl]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ppl Explain API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nReturns the execution plan for a PPL query."]
    pub fn explain<'b>(&'a self) -> PplExplain<'a, 'b, ()> {
        PplExplain::new(self.transport())
    }
    #[doc = "[Ppl Get Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves performance metrics for the PPL plugin."]
    pub fn get_stats<'b>(&'a self) -> PplGetStats<'a, 'b> {
        PplGetStats::new(self.transport())
    }
    #[doc = "[Ppl Post Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves filtered performance metrics for the PPL plugin."]
    pub fn post_stats<'b>(&'a self) -> PplPostStats<'a, 'b, ()> {
        PplPostStats::new(self.transport())
    }
    #[doc = "[Ppl Query API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nExecutes a PPL query against OpenSearch indexes."]
    pub fn query<'b>(&'a self) -> PplQuery<'a, 'b, ()> {
        PplQuery::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Ppl APIs"]
    pub fn ppl(&self) -> Ppl {
        Ppl::new(self.transport())
    }
}
