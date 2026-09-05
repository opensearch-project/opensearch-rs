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
#[doc = "API parts for the Ingestion Get State API"]
pub enum IngestionGetStateParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IngestionGetStateParts<'b> {
    #[doc = "Builds a relative URL path to the Ingestion Get State API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestionGetStateParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/ingestion/_state");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingestion Get State API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to retrieve the ingestion state for a given index."]
#[derive(Clone, Debug)]
pub struct IngestionGetState<'a, 'b> {
    transport: &'a Transport,
    parts: IngestionGetStateParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    next_token: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> IngestionGetState<'a, 'b> {
    #[doc = "Creates a new instance of [IngestionGetState] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestionGetStateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestionGetState {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            next_token: None,
            pretty: None,
            request_timeout: None,
            size: None,
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
    #[doc = "Token to retrieve the next page of results."]
    pub fn next_token(mut self, next_token: &'b str) -> Self {
        self.next_token = Some(next_token);
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
    #[doc = "Number of results to return per page."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Timeout for the request."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingestion Get State API that can be awaited"]
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
                next_token: Option<&'b str>,
                pretty: Option<bool>,
                size: Option<i64>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                next_token: self.next_token,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Ingestion Pause API"]
pub enum IngestionPauseParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IngestionPauseParts<'b> {
    #[doc = "Builds a relative URL path to the Ingestion Pause API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestionPauseParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/ingestion/_pause");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingestion Pause API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to pause ingestion for a given index."]
#[derive(Clone, Debug)]
pub struct IngestionPause<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestionPauseParts<'b>,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestionPause<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestionPause] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestionPauseParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestionPause {
            transport,
            parts,
            headers,
            body: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestionPause<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestionPause {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Time to wait for cluster manager connection."]
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
    #[doc = "Timeout for the request."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingestion Pause API that can be awaited"]
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
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ingestion Resume API"]
pub enum IngestionResumeParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IngestionResumeParts<'b> {
    #[doc = "Builds a relative URL path to the Ingestion Resume API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IngestionResumeParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/ingestion/_resume");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ingestion Resume API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to resume ingestion for the given index."]
#[derive(Clone, Debug)]
pub struct IngestionResume<'a, 'b, B> {
    transport: &'a Transport,
    parts: IngestionResumeParts<'b>,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b, B> IngestionResume<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IngestionResume] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IngestionResumeParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IngestionResume {
            transport,
            parts,
            headers,
            body: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IngestionResume<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IngestionResume {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            timeout: self.timeout,
        }
    }
    #[doc = "Time to wait for cluster manager connection."]
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
    #[doc = "Timeout for the request."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ingestion Resume API that can be awaited"]
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
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "Namespace client for Ingestion APIs"]
pub struct Ingestion<'a> {
    transport: &'a Transport,
}
impl<'a> Ingestion<'a> {
    #[doc = "Creates a new instance of [Ingestion]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ingestion Get State API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to retrieve the ingestion state for a given index."]
    pub fn get_state<'b>(&'a self, parts: IngestionGetStateParts<'b>) -> IngestionGetState<'a, 'b> {
        IngestionGetState::new(self.transport(), parts)
    }
    #[doc = "[Ingestion Pause API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to pause ingestion for a given index."]
    pub fn pause<'b>(&'a self, parts: IngestionPauseParts<'b>) -> IngestionPause<'a, 'b, ()> {
        IngestionPause::new(self.transport(), parts)
    }
    #[doc = "[Ingestion Resume API](https://docs.opensearch.org/docs/latest/api-reference/document-apis/pull-based-ingestion-management/)\n\nUse this API to resume ingestion for the given index."]
    pub fn resume<'b>(&'a self, parts: IngestionResumeParts<'b>) -> IngestionResume<'a, 'b, ()> {
        IngestionResume::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Ingestion APIs"]
    pub fn ingestion(&self) -> Ingestion {
        Ingestion::new(self.transport())
    }
}
