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
#[doc = "API parts for the Sql Close API"]
pub enum SqlCloseParts {
    #[doc = "No parts"]
    None,
}
impl SqlCloseParts {
    #[doc = "Builds a relative URL path to the Sql Close API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlCloseParts::None => "/_opendistro/_sql/close".into(),
        }
    }
}
#[doc = "Builder for the [Sql Close API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nCloses an open cursor to free server-side resources."]
#[derive(Clone, Debug)]
pub struct SqlClose<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlCloseParts,
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
impl<'a, 'b, B> SqlClose<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlClose]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlClose {
            transport,
            parts: SqlCloseParts::None,
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
    pub fn body<T>(self, body: T) -> SqlClose<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlClose {
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "Creates an asynchronous call to the Sql Close API that can be awaited"]
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
#[doc = "API parts for the Sql Explain API"]
pub enum SqlExplainParts {
    #[doc = "No parts"]
    None,
}
impl SqlExplainParts {
    #[doc = "Builds a relative URL path to the Sql Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlExplainParts::None => "/_opendistro/_sql/_explain".into(),
        }
    }
}
#[doc = "Builder for the [Sql Explain API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nReturns the execution plan for a SQL or PPL query."]
#[derive(Clone, Debug)]
pub struct SqlExplain<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlExplainParts,
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
impl<'a, 'b, B> SqlExplain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlExplain]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlExplain {
            transport,
            parts: SqlExplainParts::None,
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
    pub fn body<T>(self, body: T) -> SqlExplain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlExplain {
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "Creates an asynchronous call to the Sql Explain API that can be awaited"]
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
#[doc = "API parts for the Sql Get Stats API"]
pub enum SqlGetStatsParts {
    #[doc = "No parts"]
    None,
}
impl SqlGetStatsParts {
    #[doc = "Builds a relative URL path to the Sql Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlGetStatsParts::None => "/_opendistro/_sql/stats".into(),
        }
    }
}
#[doc = "Builder for the [Sql Get Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves performance metrics for the SQL plugin."]
#[derive(Clone, Debug)]
pub struct SqlGetStats<'a, 'b> {
    transport: &'a Transport,
    parts: SqlGetStatsParts,
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
impl<'a, 'b> SqlGetStats<'a, 'b> {
    #[doc = "Creates a new instance of [SqlGetStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlGetStats {
            transport,
            parts: SqlGetStatsParts::None,
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "Creates an asynchronous call to the Sql Get Stats API that can be awaited"]
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
#[doc = "API parts for the Sql Post Stats API"]
pub enum SqlPostStatsParts {
    #[doc = "No parts"]
    None,
}
impl SqlPostStatsParts {
    #[doc = "Builds a relative URL path to the Sql Post Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlPostStatsParts::None => "/_opendistro/_sql/stats".into(),
        }
    }
}
#[doc = "Builder for the [Sql Post Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves filtered performance metrics for the SQL plugin."]
#[derive(Clone, Debug)]
pub struct SqlPostStats<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlPostStatsParts,
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
impl<'a, 'b, B> SqlPostStats<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlPostStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlPostStats {
            transport,
            parts: SqlPostStatsParts::None,
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
    pub fn body<T>(self, body: T) -> SqlPostStats<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlPostStats {
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "Creates an asynchronous call to the Sql Post Stats API that can be awaited"]
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
#[doc = "API parts for the Sql Query API"]
pub enum SqlQueryParts {
    #[doc = "No parts"]
    None,
}
impl SqlQueryParts {
    #[doc = "Builds a relative URL path to the Sql Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlQueryParts::None => "/_opendistro/_sql".into(),
        }
    }
}
#[doc = "Builder for the [Sql Query API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nExecutes SQL or PPL queries against OpenSearch indexes."]
#[derive(Clone, Debug)]
pub struct SqlQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlQueryParts,
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
impl<'a, 'b, B> SqlQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlQuery]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlQuery {
            transport,
            parts: SqlQueryParts::None,
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
    pub fn body<T>(self, body: T) -> SqlQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlQuery {
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "Creates an asynchronous call to the Sql Query API that can be awaited"]
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
#[doc = "API parts for the Sql Settings API"]
pub enum SqlSettingsParts {
    #[doc = "No parts"]
    None,
}
impl SqlSettingsParts {
    #[doc = "Builds a relative URL path to the Sql Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SqlSettingsParts::None => "/_opendistro/_sql/settings".into(),
        }
    }
}
#[doc = "Builder for the [Sql Settings API](https://opensearch.org/docs/latest/search-plugins/sql/settings/)\n\nUpdates SQL plugin settings in the OpenSearch cluster configuration."]
#[derive(Clone, Debug)]
pub struct SqlSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: SqlSettingsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    format: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SqlSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SqlSettings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SqlSettings {
            transport,
            parts: SqlSettingsParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            format: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SqlSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SqlSettings {
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
    #[doc = "Specifies the response format (JSON or YAML)."]
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sql Settings API that can be awaited"]
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
                format: Option<&'b str>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                format: self.format,
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
#[doc = "Namespace client for Sql APIs"]
pub struct Sql<'a> {
    transport: &'a Transport,
}
impl<'a> Sql<'a> {
    #[doc = "Creates a new instance of [Sql]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Sql Close API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nCloses an open cursor to free server-side resources."]
    pub fn close<'b>(&'a self) -> SqlClose<'a, 'b, ()> {
        SqlClose::new(self.transport())
    }
    #[doc = "[Sql Explain API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nReturns the execution plan for a SQL or PPL query."]
    pub fn explain<'b>(&'a self) -> SqlExplain<'a, 'b, ()> {
        SqlExplain::new(self.transport())
    }
    #[doc = "[Sql Get Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves performance metrics for the SQL plugin."]
    pub fn get_stats<'b>(&'a self) -> SqlGetStats<'a, 'b> {
        SqlGetStats::new(self.transport())
    }
    #[doc = "[Sql Post Stats API](https://opensearch.org/docs/latest/search-plugins/sql/monitoring/)\n\nRetrieves filtered performance metrics for the SQL plugin."]
    pub fn post_stats<'b>(&'a self) -> SqlPostStats<'a, 'b, ()> {
        SqlPostStats::new(self.transport())
    }
    #[doc = "[Sql Query API](https://opensearch.org/docs/latest/search-plugins/sql/sql-ppl-api/)\n\nExecutes SQL or PPL queries against OpenSearch indexes."]
    pub fn query<'b>(&'a self) -> SqlQuery<'a, 'b, ()> {
        SqlQuery::new(self.transport())
    }
    #[doc = "[Sql Settings API](https://opensearch.org/docs/latest/search-plugins/sql/settings/)\n\nUpdates SQL plugin settings in the OpenSearch cluster configuration."]
    pub fn settings<'b>(&'a self) -> SqlSettings<'a, 'b, ()> {
        SqlSettings::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Sql APIs"]
    pub fn sql(&self) -> Sql {
        Sql::new(self.transport())
    }
}
