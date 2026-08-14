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
#[doc = "API parts for the Bulk API"]
pub enum BulkParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> BulkParts<'b> {
    #[doc = "Builds a relative URL path to the Bulk API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            BulkParts::None => "/_bulk".into(),
            BulkParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_bulk");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Bulk API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk/)\n\nAllows to perform multiple index/update/delete operations in a single request."]
#[derive(Clone, Debug)]
pub struct Bulk<'a, 'b, B> {
    transport: &'a Transport,
    parts: BulkParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    require_alias: Option<bool>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    ty: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Bulk<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Bulk] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: BulkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Bulk {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            require_alias: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "`true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude from the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> Bulk<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        Bulk {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            require_alias: self.require_alias,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            ty: self.ty,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "Name of the data stream, index, or index alias to perform bulk actions on."]
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, the request's actions must target an index alias."]
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        self.require_alias = Some(require_alias);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period each action waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Default document type for items which don't provide one."]
    pub fn ty(mut self, ty: &'b str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Bulk API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                index: Option<&'b str>,
                pipeline: Option<&'b str>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                require_alias: Option<bool>,
                routing: Option<&'b str>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                #[serde(rename = "type")]
                ty: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                require_alias: self.require_alias,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                ty: self.ty,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Bulk Stream API"]
pub enum BulkStreamParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> BulkStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Bulk Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            BulkStreamParts::None => "/_bulk/stream".into(),
            BulkStreamParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_bulk/stream");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Bulk Stream API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk-streaming/)\n\nAllows to perform multiple index/update/delete operations using request response streaming."]
#[derive(Clone, Debug)]
pub struct BulkStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: BulkStreamParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    batch_interval: Option<&'b str>,
    batch_size: Option<i64>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    require_alias: Option<bool>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    ty: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> BulkStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [BulkStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: BulkStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        BulkStream {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            batch_interval: None,
            batch_size: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            require_alias: None,
            routing: None,
            source: None,
            timeout: None,
            ty: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "`true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude from the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Specifies for how long bulk operations should be accumulated into a batch before sending the batch to data nodes."]
    pub fn batch_interval(mut self, batch_interval: &'b str) -> Self {
        self.batch_interval = Some(batch_interval);
        self
    }
    #[doc = "Specifies how many bulk operations should be accumulated into a batch before sending the batch to data nodes."]
    pub fn batch_size(mut self, batch_size: i64) -> Self {
        self.batch_size = Some(batch_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> BulkStream<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        BulkStream {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            batch_interval: self.batch_interval,
            batch_size: self.batch_size,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            require_alias: self.require_alias,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            ty: self.ty,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, the request's actions must target an index alias."]
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        self.require_alias = Some(require_alias);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period each action waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Default document type for items which don't provide one."]
    pub fn ty(mut self, ty: &'b str) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Bulk Stream API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                batch_interval: Option<&'b str>,
                batch_size: Option<i64>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pipeline: Option<&'b str>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                require_alias: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                #[serde(rename = "type")]
                ty: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                batch_interval: self.batch_interval,
                batch_size: self.batch_size,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                require_alias: self.require_alias,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                ty: self.ty,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Clear Scroll API"]
pub enum ClearScrollParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ScrollId"]
    ScrollId(&'b [&'b str]),
}
impl<'b> ClearScrollParts<'b> {
    #[doc = "Builds a relative URL path to the Clear Scroll API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ClearScrollParts::None => "/_search/scroll".into(),
            ClearScrollParts::ScrollId(scroll_id) => {
                let scroll_id_str = scroll_id.join(",");
                let encoded_scroll_id: Cow<str> =
                    percent_encode(scroll_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_scroll_id.len());
                p.push_str("/_search/scroll/");
                p.push_str(encoded_scroll_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Clear Scroll API](https://opensearch.org/docs/latest/api-reference/scroll/)\n\nExplicitly clears the search context for a scroll."]
#[derive(Clone, Debug)]
pub struct ClearScroll<'a, 'b, B> {
    transport: &'a Transport,
    parts: ClearScrollParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ClearScroll<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ClearScroll] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ClearScrollParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ClearScroll {
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
    pub fn body<T>(self, body: T) -> ClearScroll<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ClearScroll {
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
    #[doc = "Creates an asynchronous call to the Clear Scroll API that can be awaited"]
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
#[doc = "API parts for the Count API"]
pub enum CountParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CountParts<'b> {
    #[doc = "Builds a relative URL path to the Count API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CountParts::None => "/_count".into(),
            CountParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(8usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_count");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Count API](https://opensearch.org/docs/latest/api-reference/count/)\n\nReturns number of documents matching a query."]
#[derive(Clone, Debug)]
pub struct Count<'a, 'b, B> {
    transport: &'a Transport,
    parts: CountParts<'b>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    min_score: Option<f32>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    terminate_after: Option<i64>,
}
impl<'a, 'b, B> Count<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Count] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CountParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Count {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            min_score: None,
            preference: None,
            pretty: None,
            q: None,
            request_timeout: None,
            routing: None,
            source: None,
            terminate_after: None,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "Analyzer to use for the query string.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Count<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Count {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            min_score: self.min_score,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            terminate_after: self.terminate_after,
        }
    }
    #[doc = "The default operator for query string query: `AND` or `OR`.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "Field to use as default where no field prefix is given in the query string.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
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
    #[doc = "If `true`, concrete, expanded or aliased indexes are ignored when frozen."]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Sets the minimum `_score` value that documents must have to be included in the result."]
    pub fn min_score(mut self, min_score: f32) -> Self {
        self.min_score = Some(min_score);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Maximum number of documents to collect for each shard.\nIf a query reaches this limit, OpenSearch terminates the query early.\nOpenSearch collects documents before sorting."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Creates an asynchronous call to the Count API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                lenient: Option<bool>,
                min_score: Option<f32>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                terminate_after: Option<i64>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                min_score: self.min_score,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                terminate_after: self.terminate_after,
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
#[doc = "API parts for the Create API"]
pub enum CreateParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> CreateParts<'b> {
    #[doc = "Builds a relative URL path to the Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CreateParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_create/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Create API](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)\n\nCreates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
#[derive(Clone, Debug)]
pub struct Create<'a, 'b, B> {
    transport: &'a Transport,
    parts: CreateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Create<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Create] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CreateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Create {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Create<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Create {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period the request waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to `all` or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Create API that can be awaited"]
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
                pipeline: Option<&'b str>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                version: Option<i64>,
                version_type: Option<VersionType>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Create Pit API"]
pub enum CreatePitParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> CreatePitParts<'b> {
    #[doc = "Builds a relative URL path to the Create Pit API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            CreatePitParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_search/point_in_time");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Create Pit API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#create-a-pit)\n\nCreates point in time context."]
#[derive(Clone, Debug)]
pub struct CreatePit<'a, 'b, B> {
    transport: &'a Transport,
    parts: CreatePitParts<'b>,
    allow_partial_pit_creation: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    keep_alive: Option<&'b str>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> CreatePit<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [CreatePit] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: CreatePitParts<'b>) -> Self {
        let headers = HeaderMap::new();
        CreatePit {
            transport,
            parts,
            headers,
            allow_partial_pit_creation: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            keep_alive: None,
            preference: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "Allow if point in time can be created with partial failures."]
    pub fn allow_partial_pit_creation(mut self, allow_partial_pit_creation: bool) -> Self {
        self.allow_partial_pit_creation = Some(allow_partial_pit_creation);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> CreatePit<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        CreatePit {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_partial_pit_creation: self.allow_partial_pit_creation,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            keep_alive: self.keep_alive,
            preference: self.preference,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indexes that are open, closed or both."]
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
    #[doc = "Specify the keep alive for point in time."]
    pub fn keep_alive(mut self, keep_alive: &'b str) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "A comma-separated list of specific routing values."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Create Pit API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_partial_pit_creation: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                keep_alive: Option<&'b str>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_partial_pit_creation: self.allow_partial_pit_creation,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                keep_alive: self.keep_alive,
                preference: self.preference,
                pretty: self.pretty,
                routing: self.routing,
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
#[doc = "API parts for the Delete API"]
pub enum DeleteParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> DeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_doc/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Delete API](https://opensearch.org/docs/latest/api-reference/document-apis/delete-document/)\n\nRemoves a document from the index."]
#[derive(Clone, Debug)]
pub struct Delete<'a, 'b> {
    transport: &'a Transport,
    parts: DeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b> Delete<'a, 'b> {
    #[doc = "Creates a new instance of [Delete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: DeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Delete {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
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
    #[doc = "Only perform the operation if the document has this primary term."]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "Only perform the operation if the document has this sequence number."]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period to wait for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to `all` or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete API that can be awaited"]
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
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                version: Option<i64>,
                version_type: Option<VersionType>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                pretty: self.pretty,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Delete All Pits API"]
pub enum DeleteAllPitsParts {
    #[doc = "No parts"]
    None,
}
impl DeleteAllPitsParts {
    #[doc = "Builds a relative URL path to the Delete All Pits API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteAllPitsParts::None => "/_search/point_in_time/_all".into(),
        }
    }
}
#[doc = "Builder for the [Delete All Pits API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#delete-pits)\n\nDeletes all active point in time searches."]
#[derive(Clone, Debug)]
pub struct DeleteAllPits<'a, 'b> {
    transport: &'a Transport,
    parts: DeleteAllPitsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> DeleteAllPits<'a, 'b> {
    #[doc = "Creates a new instance of [DeleteAllPits]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        DeleteAllPits {
            transport,
            parts: DeleteAllPitsParts::None,
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
    #[doc = "Creates an asynchronous call to the Delete All Pits API that can be awaited"]
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
#[doc = "API parts for the Delete By Query API"]
pub enum DeleteByQueryParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> DeleteByQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Delete By Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Delete By Query API](https://opensearch.org/docs/latest/api-reference/document-apis/delete-by-query/)\n\nDeletes documents matching the provided query."]
#[derive(Clone, Debug)]
pub struct DeleteByQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: DeleteByQueryParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    refresh: Option<Refresh>,
    request_cache: Option<bool>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'b str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<&'b str>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    version: Option<bool>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> DeleteByQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DeleteByQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: DeleteByQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteByQuery {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            request_timeout: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "List of fields to exclude from the returned `_source` field."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "List of fields to extract and return from the `_source` field."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "Analyzer to use for the query string."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DeleteByQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DeleteByQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            conflicts: self.conflicts,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_docs: self.max_docs,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            refresh: self.refresh,
            request_cache: self.request_cache,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
            routing: self.routing,
            scroll: self.scroll,
            scroll_size: self.scroll_size,
            search_timeout: self.search_timeout,
            search_type: self.search_type,
            size: self.size,
            slices: self.slices,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            version: self.version,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "What to do if delete by query hits version conflicts: `abort` or `proceed`."]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query: `AND` or `OR`."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "Field to use as default where no field prefix is given in the query string."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`. Valid values are: `all`, `open`, `closed`, `hidden`, `none`."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset."]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
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
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process.\nDefaults to all documents."]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes all shards involved in the delete by query after the request completes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "If `true`, the request cache is used for this request.\nDefaults to the index-level setting."]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The throttle for this request in sub-requests per second."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Period to retain the search context for scrolling."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size of the scroll request that powers the operation."]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request.\nDefaults to no timeout."]
    pub fn search_timeout(mut self, search_timeout: &'b str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "The type of the search operation.\nAvailable options: `query_then_fetch`, `dfs_query_then_fetch`."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, use `max_docs` instead."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into."]
    pub fn slices(mut self, slices: &'b str) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of &lt;field&gt;:&lt;direction&gt; pairs."]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific `tag` of the request for logging and statistical purposes."]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "Maximum number of documents to collect for each shard.\nIf a query reaches this limit, OpenSearch terminates the query early.\nOpenSearch collects documents before sorting.\nUse with caution.\nOpenSearch applies this parameter to each shard handling the request.\nWhen possible, let OpenSearch perform early termination automatically.\nAvoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Period each deletion request waits for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, returns the document version as part of a hit."]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "If `true`, the request blocks until the operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete By Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                allow_no_indices: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                conflicts: Option<Conflicts>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                lenient: Option<bool>,
                max_docs: Option<i64>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                refresh: Option<Refresh>,
                request_cache: Option<bool>,
                requests_per_second: Option<f32>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                scroll: Option<&'b str>,
                scroll_size: Option<i64>,
                search_timeout: Option<&'b str>,
                search_type: Option<SearchType>,
                size: Option<i64>,
                slices: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                terminate_after: Option<i64>,
                timeout: Option<&'b str>,
                version: Option<bool>,
                wait_for_active_shards: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Delete By Query Rethrottle API"]
pub enum DeleteByQueryRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> DeleteByQueryRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Delete By Query Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteByQueryRethrottleParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_task_id.len());
                p.push_str("/_delete_by_query/");
                p.push_str(encoded_task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Delete By Query Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular Delete By Query operation."]
#[derive(Clone, Debug)]
pub struct DeleteByQueryRethrottle<'a, 'b, B> {
    transport: &'a Transport,
    parts: DeleteByQueryRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> DeleteByQueryRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DeleteByQueryRethrottle] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: DeleteByQueryRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteByQueryRethrottle {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> DeleteByQueryRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DeleteByQueryRethrottle {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle for this request in sub-requests per second."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete By Query Rethrottle API that can be awaited"]
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
                requests_per_second: Option<f32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
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
#[doc = "API parts for the Delete Pit API"]
pub enum DeletePitParts {
    #[doc = "No parts"]
    None,
}
impl DeletePitParts {
    #[doc = "Builds a relative URL path to the Delete Pit API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeletePitParts::None => "/_search/point_in_time".into(),
        }
    }
}
#[doc = "Builder for the [Delete Pit API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#delete-pits)\n\nDeletes one or more point in time searches based on the IDs passed."]
#[derive(Clone, Debug)]
pub struct DeletePit<'a, 'b, B> {
    transport: &'a Transport,
    parts: DeletePitParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> DeletePit<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [DeletePit]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        DeletePit {
            transport,
            parts: DeletePitParts::None,
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
    pub fn body<T>(self, body: T) -> DeletePit<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        DeletePit {
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
    #[doc = "Creates an asynchronous call to the Delete Pit API that can be awaited"]
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
#[doc = "API parts for the Delete Script API"]
pub enum DeleteScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> DeleteScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Delete Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            DeleteScriptParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_id.len());
                p.push_str("/_scripts/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Delete Script API](https://opensearch.org/docs/latest/api-reference/script-apis/delete-script/)\n\nDeletes a script."]
#[derive(Clone, Debug)]
pub struct DeleteScript<'a, 'b> {
    transport: &'a Transport,
    parts: DeleteScriptParts<'b>,
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
impl<'a, 'b> DeleteScript<'a, 'b> {
    #[doc = "Creates a new instance of [DeleteScript] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: DeleteScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        DeleteScript {
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
    #[doc = "Operation timeout for connection to cluster-manager node."]
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
    #[doc = "Period to wait for a connection to the cluster-manager node.\nIf no response is received before the timeout expires, the request fails and returns an error."]
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
    #[doc = "Period to wait for a response.\nIf no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Delete Script API that can be awaited"]
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
#[doc = "API parts for the Exists API"]
pub enum ExistsParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> ExistsParts<'b> {
    #[doc = "Builds a relative URL path to the Exists API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExistsParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_doc/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Exists API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns information about whether a document exists in an index."]
#[derive(Clone, Debug)]
pub struct Exists<'a, 'b> {
    transport: &'a Transport,
    parts: ExistsParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> Exists<'a, 'b> {
    #[doc = "Creates a new instance of [Exists] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ExistsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Exists {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "`true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude in the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes all shards involved in the delete by query after the request completes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Target the specified primary shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "List of stored fields to return as part of a hit.\nIf no fields are specified, no stored fields are included in the response.\nIf this field is specified, the `_source` parameter defaults to false."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Exists API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Exists Source API"]
pub enum ExistsSourceParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> ExistsSourceParts<'b> {
    #[doc = "Builds a relative URL path to the Exists Source API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExistsSourceParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_source/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Exists Source API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns information about whether a document source exists in an index."]
#[derive(Clone, Debug)]
pub struct ExistsSource<'a, 'b> {
    transport: &'a Transport,
    parts: ExistsSourceParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> ExistsSource<'a, 'b> {
    #[doc = "Creates a new instance of [ExistsSource] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ExistsSourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ExistsSource {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "`true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude in the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes all shards involved in the delete by query after the request completes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Target the specified primary shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Exists Source API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Explain API"]
pub enum ExplainParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> ExplainParts<'b> {
    #[doc = "Builds a relative URL path to the Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ExplainParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(11usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_explain/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Explain API](https://opensearch.org/docs/latest/api-reference/explain/)\n\nReturns information about why a specific document matches (or doesn't match) a query."]
#[derive(Clone, Debug)]
pub struct Explain<'a, 'b, B> {
    transport: &'a Transport,
    parts: ExplainParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    lenient: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> Explain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Explain] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ExplainParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Explain {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            default_operator: None,
            df: None,
            error_trace: None,
            filter_path: None,
            human: None,
            lenient: None,
            preference: None,
            pretty: None,
            q: None,
            request_timeout: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude from the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "Analyzer to use for the query string.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Explain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Explain {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            lenient: self.lenient,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
        }
    }
    #[doc = "The default operator for query string query: `AND` or `OR`."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "Field to use as default where no field prefix is given in the query string."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
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
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "A comma-separated list of stored fields to return in the response."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous call to the Explain API that can be awaited"]
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
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                lenient: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                lenient: self.lenient,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
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
#[doc = "API parts for the Field Caps API"]
pub enum FieldCapsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> FieldCapsParts<'b> {
    #[doc = "Builds a relative URL path to the Field Caps API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FieldCapsParts::None => "/_field_caps".into(),
            FieldCapsParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_field_caps");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Field Caps API](https://opensearch.org/docs/latest/field-types/supported-field-types/alias/#using-aliases-in-field-capabilities-api-operations)\n\nReturns the information about the capabilities of fields among multiple indexes."]
#[derive(Clone, Debug)]
pub struct FieldCaps<'a, 'b, B> {
    transport: &'a Transport,
    parts: FieldCapsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_unmapped: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FieldCaps<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FieldCaps] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FieldCapsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FieldCaps {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            fields: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            include_unmapped: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias,\nor `_all` value targets only missing or closed indexes. This behavior applies even if the request targets other open indexes. For example, a request\ntargeting `foo*,bar*` returns an error if an index starts with foo but no index starts with bar."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FieldCaps<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FieldCaps {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            include_unmapped: self.include_unmapped,
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
    #[doc = "The type of index that wildcard patterns can match. If the request can target data streams, this argument determines whether wildcard expressions match hidden data streams. Supports comma-separated values, such as `open,hidden`."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of fields to retrieve capabilities for. Wildcard (`*`) expressions are supported."]
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "If `true`, missing or closed indexes are not included in the response."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If `true`, unmapped fields are included in the response."]
    pub fn include_unmapped(mut self, include_unmapped: bool) -> Self {
        self.include_unmapped = Some(include_unmapped);
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
    #[doc = "Creates an asynchronous call to the Field Caps API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_unmapped: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                include_unmapped: self.include_unmapped,
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
#[doc = "API parts for the Get API"]
pub enum GetParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> GetParts<'b> {
    #[doc = "Builds a relative URL path to the Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_doc/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Get API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns a document."]
#[derive(Clone, Debug)]
pub struct Get<'a, 'b> {
    transport: &'a Transport,
    parts: GetParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> Get<'a, 'b> {
    #[doc = "Creates a new instance of [Get] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Get {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            stored_fields: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude in the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specifies the node or shard the operation should be performed on. Random by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search. If `false`, do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Target the specified primary shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "List of stored fields to return as part of a hit.\nIf no fields are specified, no stored fields are included in the response.\nIf this field is specified, the `_source` parameter defaults to false."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Explicit version number for concurrency control. The specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `internal`, `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Get API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Get All Pits API"]
pub enum GetAllPitsParts {
    #[doc = "No parts"]
    None,
}
impl GetAllPitsParts {
    #[doc = "Builds a relative URL path to the Get All Pits API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetAllPitsParts::None => "/_search/point_in_time/_all".into(),
        }
    }
}
#[doc = "Builder for the [Get All Pits API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#list-all-pits)\n\nLists all active point in time searches."]
#[derive(Clone, Debug)]
pub struct GetAllPits<'a, 'b> {
    transport: &'a Transport,
    parts: GetAllPitsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GetAllPits<'a, 'b> {
    #[doc = "Creates a new instance of [GetAllPits]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GetAllPits {
            transport,
            parts: GetAllPitsParts::None,
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
    #[doc = "Creates an asynchronous call to the Get All Pits API that can be awaited"]
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
#[doc = "API parts for the Get Script API"]
pub enum GetScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> GetScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Get Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetScriptParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_id.len());
                p.push_str("/_scripts/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Get Script API](https://opensearch.org/docs/latest/api-reference/script-apis/get-stored-script/)\n\nReturns a script."]
#[derive(Clone, Debug)]
pub struct GetScript<'a, 'b> {
    transport: &'a Transport,
    parts: GetScriptParts<'b>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    master_timeout: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GetScript<'a, 'b> {
    #[doc = "Creates a new instance of [GetScript] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GetScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GetScript {
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
        }
    }
    #[doc = "Operation timeout for connection to cluster-manager node."]
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
    #[doc = "Specify timeout for connection to master"]
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
    #[doc = "Creates an asynchronous call to the Get Script API that can be awaited"]
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
                master_timeout: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Get Script Context API"]
pub enum GetScriptContextParts {
    #[doc = "No parts"]
    None,
}
impl GetScriptContextParts {
    #[doc = "Builds a relative URL path to the Get Script Context API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetScriptContextParts::None => "/_script_context".into(),
        }
    }
}
#[doc = "Builder for the [Get Script Context API](https://opensearch.org/docs/latest/api-reference/script-apis/get-script-contexts/)\n\nReturns all script contexts."]
#[derive(Clone, Debug)]
pub struct GetScriptContext<'a, 'b> {
    transport: &'a Transport,
    parts: GetScriptContextParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GetScriptContext<'a, 'b> {
    #[doc = "Creates a new instance of [GetScriptContext]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GetScriptContext {
            transport,
            parts: GetScriptContextParts::None,
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
    #[doc = "Creates an asynchronous call to the Get Script Context API that can be awaited"]
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
#[doc = "API parts for the Get Script Languages API"]
pub enum GetScriptLanguagesParts {
    #[doc = "No parts"]
    None,
}
impl GetScriptLanguagesParts {
    #[doc = "Builds a relative URL path to the Get Script Languages API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetScriptLanguagesParts::None => "/_script_language".into(),
        }
    }
}
#[doc = "Builder for the [Get Script Languages API](https://opensearch.org/docs/latest/api-reference/script-apis/get-script-language/)\n\nReturns available script types, languages and contexts."]
#[derive(Clone, Debug)]
pub struct GetScriptLanguages<'a, 'b> {
    transport: &'a Transport,
    parts: GetScriptLanguagesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GetScriptLanguages<'a, 'b> {
    #[doc = "Creates a new instance of [GetScriptLanguages]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GetScriptLanguages {
            transport,
            parts: GetScriptLanguagesParts::None,
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
    #[doc = "Creates an asynchronous call to the Get Script Languages API that can be awaited"]
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
#[doc = "API parts for the Get Source API"]
pub enum GetSourceParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> GetSourceParts<'b> {
    #[doc = "Builds a relative URL path to the Get Source API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GetSourceParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_source/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Get Source API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns the source of a document."]
#[derive(Clone, Debug)]
pub struct GetSource<'a, 'b> {
    transport: &'a Transport,
    parts: GetSourceParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b> GetSource<'a, 'b> {
    #[doc = "Creates a new instance of [GetSource] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GetSourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GetSource {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude in the response."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
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
    #[doc = "Specifies the node or shard the operation should be performed on. Random by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Boolean) If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search. If `false`, do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Target the specified primary shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Explicit version number for concurrency control. The specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type. One of `internal`, `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Get Source API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Index API"]
pub enum IndexParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> IndexParts<'b> {
    #[doc = "Builds a relative URL path to the Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IndexParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(6usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_doc");
                p.into()
            }
            IndexParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_doc/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Index API](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)\n\nCreates or updates a document in an index."]
#[derive(Clone, Debug)]
pub struct Index<'a, 'b, B> {
    transport: &'a Transport,
    parts: IndexParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    op_type: Option<OpType>,
    pipeline: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    require_alias: Option<bool>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    version: Option<i64>,
    version_type: Option<VersionType>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Index<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Index] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IndexParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Index {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            op_type: None,
            pipeline: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            require_alias: None,
            routing: None,
            source: None,
            timeout: None,
            version: None,
            version_type: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Index<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Index {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            op_type: self.op_type,
            pipeline: self.pipeline,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            require_alias: self.require_alias,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            version: self.version,
            version_type: self.version_type,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "Only perform the operation if the document has this primary term."]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "Only perform the operation if the document has this sequence number."]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "Set to create to only index the document if it does not already exist (put if absent).\nIf a document with the specified `_id` already exists, the indexing operation will fail.\nSame as using the `&lt;index&gt;/_create` endpoint.\nValid values: `index`, `create`.\nIf document id is specified, it defaults to `index`.\nOtherwise, it defaults to `create`."]
    pub fn op_type(mut self, op_type: OpType) -> Self {
        self.op_type = Some(op_type);
        self
    }
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, the destination must be an index alias."]
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        self.require_alias = Some(require_alias);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period the request waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Index API that can be awaited"]
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
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                op_type: Option<OpType>,
                pipeline: Option<&'b str>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                require_alias: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                version: Option<i64>,
                version_type: Option<VersionType>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                op_type: self.op_type,
                pipeline: self.pipeline,
                pretty: self.pretty,
                refresh: self.refresh,
                require_alias: self.require_alias,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                version: self.version,
                version_type: self.version_type,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Info API"]
pub enum InfoParts {
    #[doc = "No parts"]
    None,
}
impl InfoParts {
    #[doc = "Builds a relative URL path to the Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InfoParts::None => "/".into(),
        }
    }
}
#[doc = "Builder for the [Info API](https://opensearch.org/docs/latest)\n\nReturns basic information about the cluster."]
#[derive(Clone, Debug)]
pub struct Info<'a, 'b> {
    transport: &'a Transport,
    parts: InfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> Info<'a, 'b> {
    #[doc = "Creates a new instance of [Info]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        Info {
            transport,
            parts: InfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Info API that can be awaited"]
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
#[doc = "API parts for the Mget API"]
pub enum MgetParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> MgetParts<'b> {
    #[doc = "Builds a relative URL path to the Mget API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MgetParts::None => "/_mget".into(),
            MgetParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(7usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mget");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Mget API](https://opensearch.org/docs/latest/api-reference/document-apis/multi-get/)\n\nAllows to get multiple documents in one request."]
#[derive(Clone, Debug)]
pub struct Mget<'a, 'b, B> {
    transport: &'a Transport,
    parts: MgetParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stored_fields: Option<&'b [&'b str]>,
}
impl<'a, 'b, B> Mget<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Mget] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MgetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Mget {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            realtime: None,
            refresh: None,
            request_timeout: None,
            routing: None,
            source: None,
            stored_fields: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude from the response.\nYou can also use this parameter to exclude fields from the subset specified in `_source_includes` query parameter."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response.\nIf this parameter is specified, only these source fields are returned. You can exclude fields from this subset using the `_source_excludes` query parameter.\nIf the `_source` parameter is `false`, this parameter is ignored."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Mget<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Mget {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            stored_fields: self.stored_fields,
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
    #[doc = "Specifies the node or shard the operation should be performed on. Random by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "If `true`, the request refreshes relevant shards before retrieving documents."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "If `true`, retrieves the document fields stored in the index rather than the document `_source`."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Creates an asynchronous call to the Mget API that can be awaited"]
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
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                refresh: Option<Refresh>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                refresh: self.refresh,
                routing: self.routing,
                source: self.source,
                stored_fields: self.stored_fields,
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
#[doc = "API parts for the Msearch API"]
pub enum MsearchParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> MsearchParts<'b> {
    #[doc = "Builds a relative URL path to the Msearch API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MsearchParts::None => "/_msearch".into(),
            MsearchParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_msearch");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Msearch API](https://opensearch.org/docs/latest/api-reference/multi-search/)\n\nAllows to execute several search operations in one request."]
#[derive(Clone, Debug)]
pub struct Msearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: MsearchParts<'b>,
    allow_partial_results: Option<bool>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> Msearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Msearch] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MsearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Msearch {
            transport,
            parts,
            headers,
            allow_partial_results: None,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            pretty: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "Specifies whether to return partial results if there are shard request timeouts or shard failures"]
    pub fn allow_partial_results(mut self, allow_partial_results: bool) -> Self {
        self.allow_partial_results = Some(allow_partial_results);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> Msearch<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        Msearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            allow_partial_results: self.allow_partial_results,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "If `true`, network round-trips between the coordinating node and remote clusters are minimized for cross-cluster search requests."]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
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
    #[doc = "Maximum number of concurrent searches the multi search API can execute."]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
        self
    }
    #[doc = "Maximum number of concurrent shard requests that each sub-search request executes per node."]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "Defines a threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method i.e., if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
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
    #[doc = "If `true`, `hits.total` are returned as an integer in the response. Defaults to false, which returns an object."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Indicates whether global term and document frequencies should be used when scoring returned documents."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specifies whether aggregation and suggester names should be prefixed by their respective types in the response."]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous call to the Msearch API that can be awaited"]
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
                allow_partial_results: Option<bool>,
                ccs_minimize_roundtrips: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                max_concurrent_searches: Option<i64>,
                max_concurrent_shard_requests: Option<i64>,
                pre_filter_shard_size: Option<i64>,
                pretty: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                search_type: Option<SearchType>,
                source: Option<&'b str>,
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                allow_partial_results: self.allow_partial_results,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
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
#[doc = "API parts for the Msearch Template API"]
pub enum MsearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> MsearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Msearch Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MsearchTemplateParts::None => "/_msearch/template".into(),
            MsearchTemplateParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_msearch/template");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Msearch Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to execute several search template operations in one request."]
#[derive(Clone, Debug)]
pub struct MsearchTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: MsearchTemplateParts<'b>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_concurrent_searches: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> MsearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MsearchTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MsearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MsearchTemplate {
            transport,
            parts,
            headers,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_concurrent_searches: None,
            pretty: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> MsearchTemplate<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        MsearchTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(NdBody(body)),
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_concurrent_searches: self.max_concurrent_searches,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "If `true`, network round-trips are minimized for cross-cluster search requests."]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
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
    #[doc = "Maximum number of concurrent searches the API can run."]
    pub fn max_concurrent_searches(mut self, max_concurrent_searches: i64) -> Self {
        self.max_concurrent_searches = Some(max_concurrent_searches);
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
    #[doc = "If `true`, the response returns `hits.total` as an integer.\nIf `false`, it returns `hits.total` as an object."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "The type of the search operation.\nAvailable options: `query_then_fetch`, `dfs_query_then_fetch`."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "If `true`, the response prefixes aggregation and suggester names with their respective types."]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous call to the Msearch Template API that can be awaited"]
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
                ccs_minimize_roundtrips: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                max_concurrent_searches: Option<i64>,
                pretty: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                search_type: Option<SearchType>,
                source: Option<&'b str>,
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_concurrent_searches: self.max_concurrent_searches,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
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
#[doc = "API parts for the Mtermvectors API"]
pub enum MtermvectorsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> MtermvectorsParts<'b> {
    #[doc = "Builds a relative URL path to the Mtermvectors API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MtermvectorsParts::None => "/_mtermvectors".into(),
            MtermvectorsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_mtermvectors");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Mtermvectors API](https://opensearch.org/docs/latest)\n\nReturns multiple termvectors in one request."]
#[derive(Clone, Debug)]
pub struct Mtermvectors<'a, 'b, B> {
    transport: &'a Transport,
    parts: MtermvectorsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ids: Option<&'b [&'b str]>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b, B> Mtermvectors<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Mtermvectors] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MtermvectorsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Mtermvectors {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            ids: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            request_timeout: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Mtermvectors<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Mtermvectors {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ids: self.ids,
            offsets: self.offsets,
            payloads: self.payloads,
            positions: self.positions,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            term_statistics: self.term_statistics,
            version: self.version,
            version_type: self.version_type,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "If `true`, the response includes the document count, sum of document frequencies, and sum of total term frequencies."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "A comma-separated list of documents IDs. You must provide either the `docs` field in the request body or specify `ids` as a query parameter or in the request body."]
    pub fn ids(mut self, ids: &'b [&'b str]) -> Self {
        self.ids = Some(ids);
        self
    }
    #[doc = "If `true`, the response includes term offsets."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "If `true`, the response includes term payloads."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "If `true`, the response includes term positions."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specifies the node or shard on which the operation should be performed.\nSee [preference query parameter]({{site.url}}{{site.baseurl}}/api-reference/search-apis/search/#the-preference-query-parameter) for a list of available options.\nBy default the requests are routed randomly to available shard copies (primary or replica), with no guarantee of consistency across repeated queries."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "If `true`, the response includes term frequency and document frequency."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "If `true`, returns the document version as part of a hit."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Mtermvectors API that can be awaited"]
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
                field_statistics: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                ids: Option<&'b [&'b str]>,
                offsets: Option<bool>,
                payloads: Option<bool>,
                positions: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                term_statistics: Option<bool>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                ids: self.ids,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Ping API"]
pub enum PingParts {
    #[doc = "No parts"]
    None,
}
impl PingParts {
    #[doc = "Builds a relative URL path to the Ping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PingParts::None => "/".into(),
        }
    }
}
#[doc = "Builder for the [Ping API](https://opensearch.org/docs/latest)\n\nReturns whether the cluster is running."]
#[derive(Clone, Debug)]
pub struct Ping<'a, 'b> {
    transport: &'a Transport,
    parts: PingParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> Ping<'a, 'b> {
    #[doc = "Creates a new instance of [Ping]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        Ping {
            transport,
            parts: PingParts::None,
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
    #[doc = "Creates an asynchronous call to the Ping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Head;
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
#[doc = "API parts for the Put Script API"]
pub enum PutScriptParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Id and Context"]
    IdContext(&'b str, &'b str),
}
impl<'b> PutScriptParts<'b> {
    #[doc = "Builds a relative URL path to the Put Script API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            PutScriptParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_id.len());
                p.push_str("/_scripts/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            PutScriptParts::IdContext(id, context) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let encoded_context: Cow<str> =
                    percent_encode(context.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(11usize + encoded_id.len() + encoded_context.len());
                p.push_str("/_scripts/");
                p.push_str(encoded_id.as_ref());
                p.push('/');
                p.push_str(encoded_context.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Put Script API](https://opensearch.org/docs/latest/api-reference/script-apis/create-stored-script/)\n\nCreates or updates a script."]
#[derive(Clone, Debug)]
pub struct PutScript<'a, 'b, B> {
    transport: &'a Transport,
    parts: PutScriptParts<'b>,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    context: Option<&'b str>,
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
impl<'a, 'b, B> PutScript<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [PutScript] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: PutScriptParts<'b>) -> Self {
        let headers = HeaderMap::new();
        PutScript {
            transport,
            parts,
            headers,
            body: None,
            cluster_manager_timeout: None,
            context: None,
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
    pub fn body<T>(self, body: T) -> PutScript<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        PutScript {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            cluster_manager_timeout: self.cluster_manager_timeout,
            context: self.context,
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
    #[doc = "Operation timeout for connection to cluster-manager node."]
    pub fn cluster_manager_timeout(mut self, cluster_manager_timeout: &'b str) -> Self {
        self.cluster_manager_timeout = Some(cluster_manager_timeout);
        self
    }
    #[doc = "Context in which the script or search template should run.\nTo prevent errors, the API immediately compiles the script or template in this context."]
    pub fn context(mut self, context: &'b str) -> Self {
        self.context = Some(context);
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
    #[doc = "Period to wait for a connection to the cluster-manager node.\nIf no response is received before the timeout expires, the request fails and returns an error."]
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
    #[doc = "Period to wait for a response.\nIf no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Put Script API that can be awaited"]
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
                context: Option<&'b str>,
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
                context: self.context,
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
#[doc = "API parts for the Rank Eval API"]
pub enum RankEvalParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> RankEvalParts<'b> {
    #[doc = "Builds a relative URL path to the Rank Eval API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RankEvalParts::None => "/_rank_eval".into(),
            RankEvalParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_rank_eval");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Rank Eval API](https://opensearch.org/docs/latest/api-reference/rank-eval/)\n\nAllows to evaluate the quality of ranked search results over a set of typical search queries."]
#[derive(Clone, Debug)]
pub struct RankEval<'a, 'b, B> {
    transport: &'a Transport,
    parts: RankEvalParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> RankEval<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RankEval] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RankEvalParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RankEval {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            pretty: None,
            request_timeout: None,
            search_type: None,
            source: None,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes. This behavior applies even if the request targets other open indexes. For example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RankEval<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RankEval {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            search_type: self.search_type,
            source: self.source,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indexes that are open, closed or both."]
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
    #[doc = "If `true`, missing or closed indexes are not included in the response."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
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
    #[doc = "Search operation type"]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Rank Eval API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                pretty: Option<bool>,
                search_type: Option<SearchType>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                pretty: self.pretty,
                search_type: self.search_type,
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
#[doc = "API parts for the Reindex API"]
pub enum ReindexParts {
    #[doc = "No parts"]
    None,
}
impl ReindexParts {
    #[doc = "Builds a relative URL path to the Reindex API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReindexParts::None => "/_reindex".into(),
        }
    }
}
#[doc = "Builder for the [Reindex API](https://opensearch.org/docs/latest/im-plugin/reindex-data/)\n\nAllows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
#[derive(Clone, Debug)]
pub struct Reindex<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReindexParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_docs: Option<i64>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    require_alias: Option<bool>,
    scroll: Option<&'b str>,
    slices: Option<&'b str>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> Reindex<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Reindex]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        Reindex {
            transport,
            parts: ReindexParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            max_docs: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            requests_per_second: None,
            require_alias: None,
            scroll: None,
            slices: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Reindex<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Reindex {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            max_docs: self.max_docs,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
            require_alias: self.require_alias,
            scroll: self.scroll,
            slices: self.slices,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
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
    #[doc = "Maximum number of documents to process. By default, all documents."]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request refreshes affected shards to make this operation visible to search."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The throttle for this request in sub-requests per second.\nDefaults to no throttle."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        self.require_alias = Some(require_alias);
        self
    }
    #[doc = "Specifies how long a consistent view of the index should be maintained for scrolled search."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The number of slices this task should be divided into.\nDefaults to 1 slice, meaning the task isn't sliced into subtasks."]
    pub fn slices(mut self, slices: &'b str) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period each indexing waits for automatic index creation, dynamic mapping updates, and waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to `all` or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "If `true`, the request blocks until the operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Reindex API that can be awaited"]
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
                max_docs: Option<i64>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                requests_per_second: Option<f32>,
                require_alias: Option<bool>,
                scroll: Option<&'b str>,
                slices: Option<&'b str>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_docs: self.max_docs,
                pretty: self.pretty,
                refresh: self.refresh,
                requests_per_second: self.requests_per_second,
                require_alias: self.require_alias,
                scroll: self.scroll,
                slices: self.slices,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Reindex Rethrottle API"]
pub enum ReindexRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> ReindexRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Reindex Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ReindexRethrottleParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_task_id.len());
                p.push_str("/_reindex/");
                p.push_str(encoded_task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Reindex Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular reindex operation."]
#[derive(Clone, Debug)]
pub struct ReindexRethrottle<'a, 'b, B> {
    transport: &'a Transport,
    parts: ReindexRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ReindexRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ReindexRethrottle] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ReindexRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ReindexRethrottle {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> ReindexRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ReindexRethrottle {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle for this request in sub-requests per second."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Reindex Rethrottle API that can be awaited"]
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
                requests_per_second: Option<f32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
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
#[doc = "API parts for the Render Search Template API"]
pub enum RenderSearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> RenderSearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Render Search Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RenderSearchTemplateParts::None => "/_render/template".into(),
            RenderSearchTemplateParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_render/template/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Render Search Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to use the Mustache language to pre-render a search definition."]
#[derive(Clone, Debug)]
pub struct RenderSearchTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: RenderSearchTemplateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> RenderSearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RenderSearchTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: RenderSearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        RenderSearchTemplate {
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
    pub fn body<T>(self, body: T) -> RenderSearchTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RenderSearchTemplate {
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
    #[doc = "Creates an asynchronous call to the Render Search Template API that can be awaited"]
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
#[doc = "API parts for the Scripts Painless Execute API"]
pub enum ScriptsPainlessExecuteParts {
    #[doc = "No parts"]
    None,
}
impl ScriptsPainlessExecuteParts {
    #[doc = "Builds a relative URL path to the Scripts Painless Execute API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ScriptsPainlessExecuteParts::None => "/_scripts/painless/_execute".into(),
        }
    }
}
#[doc = "Builder for the [Scripts Painless Execute API](https://opensearch.org/docs/latest/api-reference/script-apis/exec-script/)\n\nAllows an arbitrary script to be executed and a result to be returned."]
#[derive(Clone, Debug)]
pub struct ScriptsPainlessExecute<'a, 'b, B> {
    transport: &'a Transport,
    parts: ScriptsPainlessExecuteParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ScriptsPainlessExecute<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ScriptsPainlessExecute]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ScriptsPainlessExecute {
            transport,
            parts: ScriptsPainlessExecuteParts::None,
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
    pub fn body<T>(self, body: T) -> ScriptsPainlessExecute<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ScriptsPainlessExecute {
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
    #[doc = "Creates an asynchronous call to the Scripts Painless Execute API that can be awaited"]
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
#[doc = "API parts for the Scroll API"]
pub enum ScrollParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ScrollId"]
    ScrollId(&'b str),
}
impl<'b> ScrollParts<'b> {
    #[doc = "Builds a relative URL path to the Scroll API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ScrollParts::None => "/_search/scroll".into(),
            ScrollParts::ScrollId(scroll_id) => {
                let encoded_scroll_id: Cow<str> =
                    percent_encode(scroll_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_scroll_id.len());
                p.push_str("/_search/scroll/");
                p.push_str(encoded_scroll_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Scroll API](https://opensearch.org/docs/latest/api-reference/scroll/#path-and-http-methods)\n\nAllows to retrieve a large numbers of results from a single search request."]
#[derive(Clone, Debug)]
pub struct Scroll<'a, 'b, B> {
    transport: &'a Transport,
    parts: ScrollParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    scroll: Option<&'b str>,
    scroll_id: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> Scroll<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Scroll] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ScrollParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Scroll {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            scroll: None,
            scroll_id: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Scroll<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Scroll {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            scroll: self.scroll,
            scroll_id: self.scroll_id,
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
    #[doc = "If `true`, the API response's `hit.total` property is returned as an integer. If `false`, the API response's `hit.total` property is returned as an object."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "Period to retain the search context for scrolling."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "The scroll ID for scrolled search"]
    #[deprecated = "Deprecated"]
    pub fn scroll_id(mut self, scroll_id: &'b str) -> Self {
        self.scroll_id = Some(scroll_id);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Scroll API that can be awaited"]
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
                pretty: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                scroll: Option<&'b str>,
                scroll_id: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                scroll: self.scroll,
                scroll_id: self.scroll_id,
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
#[doc = "API parts for the Search API"]
pub enum SearchParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> SearchParts<'b> {
    #[doc = "Builds a relative URL path to the Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchParts::None => "/_search".into(),
            SearchParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(9usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search API](https://opensearch.org/docs/latest/api-reference/search/)\n\nReturns results matching a query."]
#[derive(Clone, Debug)]
pub struct Search<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    cancel_after_time_interval: Option<&'b str>,
    ccs_minimize_roundtrips: Option<bool>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    docvalue_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    include_named_queries_score: Option<bool>,
    index: Option<&'b [&'b str]>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    phase_took: Option<bool>,
    pre_filter_shard_size: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_cache: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    search_pipeline: Option<&'b str>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    stored_fields: Option<&'b [&'b str]>,
    suggest_field: Option<&'b str>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<&'b str>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    track_scores: Option<bool>,
    track_total_hits: Option<TrackTotalHits>,
    typed_keys: Option<bool>,
    verbose_pipeline: Option<bool>,
    version: Option<bool>,
}
impl<'a, 'b, B> Search<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Search] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Search {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            allow_partial_search_results: None,
            analyze_wildcard: None,
            analyzer: None,
            batched_reduce_size: None,
            body: None,
            cancel_after_time_interval: None,
            ccs_minimize_roundtrips: None,
            default_operator: None,
            df: None,
            docvalue_fields: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            include_named_queries_score: None,
            index: None,
            lenient: None,
            max_concurrent_shard_requests: None,
            phase_took: None,
            pre_filter_shard_size: None,
            preference: None,
            pretty: None,
            q: None,
            request_cache: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_pipeline: None,
            search_type: None,
            seq_no_primary_term: None,
            size: None,
            sort: None,
            source: None,
            stats: None,
            stored_fields: None,
            suggest_field: None,
            suggest_mode: None,
            suggest_size: None,
            suggest_text: None,
            terminate_after: None,
            timeout: None,
            track_scores: None,
            track_total_hits: None,
            typed_keys: None,
            verbose_pipeline: None,
            version: None,
        }
    }
    #[doc = "Indicates which source fields are returned for matching documents.\nThese fields are returned in the `hits._source` property of the search response.\nValid values are:\n`true` to return the entire document source;\n`false` to not return the document source;\n`&lt;string&gt;` to return the source fields that are specified as a comma-separated list (supports wildcard (`*`) patterns)."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "A comma-separated list of source fields to exclude from the response.\nYou can also use this parameter to exclude fields from the subset specified in `_source_includes` query parameter.\nIf the `_source` parameter is `false`, this parameter is ignored."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "A comma-separated list of source fields to include in the response.\nIf this parameter is specified, only these source fields are returned.\nYou can exclude fields from this subset using the `_source_excludes` query parameter.\nIf the `_source` parameter is `false`, this parameter is ignored."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "If `true`, returns partial results if there are shard request timeouts or shard failures. If `false`, returns an error with no partial results."]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
        self
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "Analyzer to use for the query string.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node.\nThis value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: i64) -> Self {
        self.batched_reduce_size = Some(batched_reduce_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Search<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Search {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            allow_partial_search_results: self.allow_partial_search_results,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            batched_reduce_size: self.batched_reduce_size,
            cancel_after_time_interval: self.cancel_after_time_interval,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            default_operator: self.default_operator,
            df: self.df,
            docvalue_fields: self.docvalue_fields,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            include_named_queries_score: self.include_named_queries_score,
            index: self.index,
            lenient: self.lenient,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            phase_took: self.phase_took,
            pre_filter_shard_size: self.pre_filter_shard_size,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_cache: self.request_cache,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            routing: self.routing,
            scroll: self.scroll,
            search_pipeline: self.search_pipeline,
            search_type: self.search_type,
            seq_no_primary_term: self.seq_no_primary_term,
            size: self.size,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            stored_fields: self.stored_fields,
            suggest_field: self.suggest_field,
            suggest_mode: self.suggest_mode,
            suggest_size: self.suggest_size,
            suggest_text: self.suggest_text,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            track_scores: self.track_scores,
            track_total_hits: self.track_total_hits,
            typed_keys: self.typed_keys,
            verbose_pipeline: self.verbose_pipeline,
            version: self.version,
        }
    }
    #[doc = "The time after which the search request will be canceled.\nRequest-level parameter takes precedence over `cancel_after_time_interval` cluster setting."]
    pub fn cancel_after_time_interval(mut self, cancel_after_time_interval: &'b str) -> Self {
        self.cancel_after_time_interval = Some(cancel_after_time_interval);
        self
    }
    #[doc = "If `true`, network round-trips between the coordinating node and the remote clusters are minimized when executing cross-cluster search (CCS) requests."]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "The default operator for query string query: AND or OR.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "Field to use as default where no field prefix is given in the query string.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation for each hit."]
    pub fn docvalue_fields(mut self, docvalue_fields: &'b [&'b str]) -> Self {
        self.docvalue_fields = Some(docvalue_fields);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "If `true`, returns detailed information about score computation as part of a hit."]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting document offset.\nNeeds to be non-negative.\nBy default, you cannot page through more than 10,000 hits using the `from` and `size` parameters.\nTo page through more hits, use the `search_after` parameter."]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
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
    #[doc = "If `true`, concrete, expanded or aliased indexes will be ignored when frozen."]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Indicates whether `hit.matched_queries` should be rendered as a map that includes the name of the matched query associated with its score (true) or as an array containing the name of the matched queries (false)"]
    pub fn include_named_queries_score(mut self, include_named_queries_score: bool) -> Self {
        self.include_named_queries_score = Some(include_named_queries_score);
        self
    }
    #[doc = "A comma-separated list of data streams, indexes, and aliases to search.\nSupports wildcards (`*`).\nTo search all data streams and indexes, omit this parameter or use `*` or `_all`."]
    pub fn index(mut self, index: &'b [&'b str]) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Defines the number of concurrent shard requests per node this search executes concurrently.\nThis value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests."]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "Indicates whether to return phase-level `took` time values in the response."]
    pub fn phase_took(mut self, phase_took: bool) -> Self {
        self.phase_took = Some(phase_took);
        self
    }
    #[doc = "Defines a threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold.\nThis filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method (if date filters are mandatory to match but the shard bounds and the query are disjoint).\nWhen unspecified, the pre-filter phase is executed if any of these conditions is met:\nthe request targets more than 128 shards;\nthe request targets one or more read-only index;\nthe primary sort of the query targets an indexed field."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Nodes and shards used for the search.\nBy default, OpenSearch selects from eligible nodes and shards using adaptive replica selection, accounting for allocation awareness. Valid values are:\n`_only_local` to run the search only on shards on the local node;\n`_local` to, if possible, run the search on shards on the local node, or if not, select shards using the default method;\n`_only_nodes:&lt;node-id&gt;,&lt;node-id&gt;` to run the search on only the specified nodes IDs, where, if suitable shards exist on more than one selected node, use shards on those nodes using the default method, or if none of the specified nodes are available, select shards from any available node using the default method;\n`_prefer_nodes:&lt;node-id&gt;,&lt;node-id&gt;` to if possible, run the search on the specified nodes IDs, or if not, select shards using the default method;\n`_shards:&lt;shard&gt;,&lt;shard&gt;` to run the search only on the specified shards;\n`&lt;custom-string&gt;` (any string that does not start with `_`) to route searches with the same `&lt;custom-string&gt;` to the same shards in the same order."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax using query parameter search.\nQuery parameter searches do not support the full OpenSearch Query DSL but are handy for testing."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "If `true`, the caching of search results is enabled for requests where `size` is `0`.\nDefaults to index level settings."]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Indicates whether `hits.total` should be rendered as an integer or an object in the rest search response."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Period to retain the search context for scrolling. See Scroll search results.\nBy default, this value cannot exceed `1d` (24 hours).\nYou can change this limit using the `search.max_keep_alive` cluster-level setting."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Customizable sequence of processing stages applied to search queries."]
    pub fn search_pipeline(mut self, search_pipeline: &'b str) -> Self {
        self.search_pipeline = Some(search_pipeline);
        self
    }
    #[doc = "How distributed term frequencies are calculated for relevance scoring."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "If `true`, returns sequence number and primary term of the last modification of each hit."]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: bool) -> Self {
        self.seq_no_primary_term = Some(seq_no_primary_term);
        self
    }
    #[doc = "Defines the number of hits to return.\nBy default, you cannot page through more than 10,000 hits using the `from` and `size` parameters.\nTo page through more hits, use the `search_after` parameter."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "A comma-separated list of &lt;field&gt;:&lt;direction&gt; pairs."]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific `tag` of the request for logging and statistical purposes."]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit.\nIf no fields are specified, no stored fields are included in the response.\nIf this field is specified, the `_source` parameter defaults to `false`.\nYou can pass `_source: true` to return both source fields and stored fields in the search response."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Specifies which field to use for suggestions."]
    pub fn suggest_field(mut self, suggest_field: &'b str) -> Self {
        self.suggest_field = Some(suggest_field);
        self
    }
    #[doc = "Specifies the suggest mode.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_mode(mut self, suggest_mode: SuggestMode) -> Self {
        self.suggest_mode = Some(suggest_mode);
        self
    }
    #[doc = "Number of suggestions to return.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_size(mut self, suggest_size: i64) -> Self {
        self.suggest_size = Some(suggest_size);
        self
    }
    #[doc = "The source text for which the suggestions should be returned.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_text(mut self, suggest_text: &'b str) -> Self {
        self.suggest_text = Some(suggest_text);
        self
    }
    #[doc = "Maximum number of documents to collect for each shard.\nIf a query reaches this limit, OpenSearch terminates the query early.\nOpenSearch collects documents before sorting.\nUse with caution.\nOpenSearch applies this parameter to each shard handling the request.\nWhen possible, let OpenSearch perform early termination automatically.\nAvoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers.\nIf set to `0` (default), the query does not terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Specifies the period of time to wait for a response from each shard.\nIf no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, calculate and return document scores, even if the scores are not used for sorting."]
    pub fn track_scores(mut self, track_scores: bool) -> Self {
        self.track_scores = Some(track_scores);
        self
    }
    #[doc = "Number of hits matching the query to count accurately.\nIf `true`, the exact number of hits is returned at the cost of some performance.\nIf `false`, the response does not include the total number of hits matching the query."]
    pub fn track_total_hits<T: Into<TrackTotalHits>>(mut self, track_total_hits: T) -> Self {
        self.track_total_hits = Some(track_total_hits.into());
        self
    }
    #[doc = "If `true`, aggregation and suggester names are be prefixed by their respective types in the response."]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Enables or disables verbose mode for the search pipeline.\nWhen verbose mode is enabled, detailed information about each processor\nin the search pipeline is included in the search response. This includes\nthe processor name, execution status, input, output, and time taken for processing.\nThis parameter is primarily intended for debugging purposes, allowing users\nto track how data flows and transforms through the search pipeline."]
    pub fn verbose_pipeline(mut self, verbose_pipeline: bool) -> Self {
        self.verbose_pipeline = Some(verbose_pipeline);
        self
    }
    #[doc = "If `true`, returns document version as part of a hit."]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Search API that can be awaited"]
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
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                allow_no_indices: Option<bool>,
                allow_partial_search_results: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                batched_reduce_size: Option<i64>,
                cancel_after_time_interval: Option<&'b str>,
                ccs_minimize_roundtrips: Option<bool>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                docvalue_fields: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                include_named_queries_score: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                index: Option<&'b [&'b str]>,
                lenient: Option<bool>,
                max_concurrent_shard_requests: Option<i64>,
                phase_took: Option<bool>,
                pre_filter_shard_size: Option<i64>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                request_cache: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                scroll: Option<&'b str>,
                search_pipeline: Option<&'b str>,
                search_type: Option<SearchType>,
                seq_no_primary_term: Option<bool>,
                size: Option<i64>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
                suggest_field: Option<&'b str>,
                suggest_mode: Option<SuggestMode>,
                suggest_size: Option<i64>,
                suggest_text: Option<&'b str>,
                terminate_after: Option<i64>,
                timeout: Option<&'b str>,
                track_scores: Option<bool>,
                track_total_hits: Option<TrackTotalHits>,
                typed_keys: Option<bool>,
                verbose_pipeline: Option<bool>,
                version: Option<bool>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                allow_partial_search_results: self.allow_partial_search_results,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                batched_reduce_size: self.batched_reduce_size,
                cancel_after_time_interval: self.cancel_after_time_interval,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                default_operator: self.default_operator,
                df: self.df,
                docvalue_fields: self.docvalue_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                include_named_queries_score: self.include_named_queries_score,
                index: self.index,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                phase_took: self.phase_took,
                pre_filter_shard_size: self.pre_filter_shard_size,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                request_cache: self.request_cache,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_pipeline: self.search_pipeline,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                stored_fields: self.stored_fields,
                suggest_field: self.suggest_field,
                suggest_mode: self.suggest_mode,
                suggest_size: self.suggest_size,
                suggest_text: self.suggest_text,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                track_scores: self.track_scores,
                track_total_hits: self.track_total_hits,
                typed_keys: self.typed_keys,
                verbose_pipeline: self.verbose_pipeline,
                version: self.version,
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
#[doc = "API parts for the Search Shards API"]
pub enum SearchShardsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> SearchShardsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Shards API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchShardsParts::None => "/_search_shards".into(),
            SearchShardsParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_search_shards");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Shards API](https://opensearch.org/docs/latest)\n\nReturns information about the indexes and shards that a search request would be executed against."]
#[derive(Clone, Debug)]
pub struct SearchShards<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchShardsParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    local: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchShards<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchShards] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchShardsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchShards {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            human: None,
            ignore_unavailable: None,
            local: None,
            preference: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchShards<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchShards {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            local: self.local,
            preference: self.preference,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`.\nValid values are: `all`, `open`, `closed`, `hidden`, `none`."]
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
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If `true`, the request retrieves information from the local node only."]
    pub fn local(mut self, local: bool) -> Self {
        self.local = Some(local);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Search Shards API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                local: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                local: self.local,
                preference: self.preference,
                pretty: self.pretty,
                routing: self.routing,
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
#[doc = "API parts for the Search Template API"]
pub enum SearchTemplateParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> SearchTemplateParts<'b> {
    #[doc = "Builds a relative URL path to the Search Template API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchTemplateParts::None => "/_search/template".into(),
            SearchTemplateParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_search/template");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Search Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to use the Mustache language to pre-render a search definition."]
#[derive(Clone, Debug)]
pub struct SearchTemplate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchTemplateParts<'b>,
    allow_no_indices: Option<bool>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    phase_took: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    profile: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    search_pipeline: Option<&'b str>,
    search_type: Option<SearchType>,
    source: Option<&'b str>,
    typed_keys: Option<bool>,
}
impl<'a, 'b, B> SearchTemplate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchTemplate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchTemplateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchTemplate {
            transport,
            parts,
            headers,
            allow_no_indices: None,
            body: None,
            ccs_minimize_roundtrips: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            phase_took: None,
            preference: None,
            pretty: None,
            profile: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_pipeline: None,
            search_type: None,
            source: None,
            typed_keys: None,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SearchTemplate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchTemplate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_no_indices: self.allow_no_indices,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            phase_took: self.phase_took,
            preference: self.preference,
            pretty: self.pretty,
            profile: self.profile,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            routing: self.routing,
            scroll: self.scroll,
            search_pipeline: self.search_pipeline,
            search_type: self.search_type,
            source: self.source,
            typed_keys: self.typed_keys,
        }
    }
    #[doc = "If `true`, network round-trips are minimized for cross-cluster search requests."]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`.\nValid values are: `all`, `open`, `closed`, `hidden`, `none`."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "If `true`, the response includes additional details about score computation as part of a hit."]
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
    #[doc = "If `true`, specified concrete, expanded, or aliased indexes are not included in the response when throttled."]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Indicates whether to return phase-level `took` time values in the response."]
    pub fn phase_took(mut self, phase_took: bool) -> Self {
        self.phase_took = Some(phase_took);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the query execution is profiled."]
    pub fn profile(mut self, profile: bool) -> Self {
        self.profile = Some(profile);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, `hits.total` are rendered as an integer in the response."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specifies how long a consistent view of the index\nshould be maintained for scrolled search."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Customizable sequence of processing stages applied to search queries."]
    pub fn search_pipeline(mut self, search_pipeline: &'b str) -> Self {
        self.search_pipeline = Some(search_pipeline);
        self
    }
    #[doc = "The type of the search operation."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "If `true`, the response prefixes aggregation and suggester names with their respective types."]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Creates an asynchronous call to the Search Template API that can be awaited"]
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
                allow_no_indices: Option<bool>,
                ccs_minimize_roundtrips: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                phase_took: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                profile: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                scroll: Option<&'b str>,
                search_pipeline: Option<&'b str>,
                search_type: Option<SearchType>,
                source: Option<&'b str>,
                typed_keys: Option<bool>,
            }
            let query_params = QueryParams {
                allow_no_indices: self.allow_no_indices,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                phase_took: self.phase_took,
                preference: self.preference,
                pretty: self.pretty,
                profile: self.profile,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_pipeline: self.search_pipeline,
                search_type: self.search_type,
                source: self.source,
                typed_keys: self.typed_keys,
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
#[doc = "API parts for the Termvectors API"]
pub enum TermvectorsParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> TermvectorsParts<'b> {
    #[doc = "Builds a relative URL path to the Termvectors API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TermvectorsParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_termvectors");
                p.into()
            }
            TermvectorsParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_termvectors/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Termvectors API](https://opensearch.org/docs/latest)\n\nReturns information and statistics about terms in the fields of a particular document."]
#[derive(Clone, Debug)]
pub struct Termvectors<'a, 'b, B> {
    transport: &'a Transport,
    parts: TermvectorsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    field_statistics: Option<bool>,
    fields: Option<&'b [&'b str]>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    offsets: Option<bool>,
    payloads: Option<bool>,
    positions: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    realtime: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    term_statistics: Option<bool>,
    version: Option<i64>,
    version_type: Option<VersionType>,
}
impl<'a, 'b, B> Termvectors<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Termvectors] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TermvectorsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Termvectors {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            field_statistics: None,
            fields: None,
            filter_path: None,
            human: None,
            offsets: None,
            payloads: None,
            positions: None,
            preference: None,
            pretty: None,
            realtime: None,
            request_timeout: None,
            routing: None,
            source: None,
            term_statistics: None,
            version: None,
            version_type: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Termvectors<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Termvectors {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            field_statistics: self.field_statistics,
            fields: self.fields,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            offsets: self.offsets,
            payloads: self.payloads,
            positions: self.positions,
            preference: self.preference,
            pretty: self.pretty,
            realtime: self.realtime,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            term_statistics: self.term_statistics,
            version: self.version,
            version_type: self.version_type,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "If `true`, the response includes the document count, sum of document frequencies, and sum of total term frequencies."]
    pub fn field_statistics(mut self, field_statistics: bool) -> Self {
        self.field_statistics = Some(field_statistics);
        self
    }
    pub fn fields(mut self, fields: &'b [&'b str]) -> Self {
        self.fields = Some(fields);
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
    #[doc = "If `true`, the response includes term offsets."]
    pub fn offsets(mut self, offsets: bool) -> Self {
        self.offsets = Some(offsets);
        self
    }
    #[doc = "If `true`, the response includes term payloads."]
    pub fn payloads(mut self, payloads: bool) -> Self {
        self.payloads = Some(payloads);
        self
    }
    #[doc = "If `true`, the response includes term positions."]
    pub fn positions(mut self, positions: bool) -> Self {
        self.positions = Some(positions);
        self
    }
    #[doc = "Specifies the node or shard on which the operation should be performed.\nSee [preference query parameter]({{site.url}}{{site.baseurl}}/api-reference/search-apis/search/#the-preference-query-parameter) for a list of available options.\nBy default the requests are routed randomly to available shard copies (primary or replica), with no guarantee of consistency across repeated queries."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(mut self, realtime: bool) -> Self {
        self.realtime = Some(realtime);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "If `true`, the response includes term frequency and document frequency."]
    pub fn term_statistics(mut self, term_statistics: bool) -> Self {
        self.term_statistics = Some(term_statistics);
        self
    }
    #[doc = "If `true`, returns the document version as part of a hit."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The specific version type."]
    pub fn version_type(mut self, version_type: VersionType) -> Self {
        self.version_type = Some(version_type);
        self
    }
    #[doc = "Creates an asynchronous call to the Termvectors API that can be awaited"]
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
                field_statistics: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                fields: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                offsets: Option<bool>,
                payloads: Option<bool>,
                positions: Option<bool>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                realtime: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                term_statistics: Option<bool>,
                version: Option<i64>,
                version_type: Option<VersionType>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                field_statistics: self.field_statistics,
                fields: self.fields,
                filter_path: self.filter_path,
                human: self.human,
                offsets: self.offsets,
                payloads: self.payloads,
                positions: self.positions,
                preference: self.preference,
                pretty: self.pretty,
                realtime: self.realtime,
                routing: self.routing,
                source: self.source,
                term_statistics: self.term_statistics,
                version: self.version,
                version_type: self.version_type,
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
#[doc = "API parts for the Update API"]
pub enum UpdateParts<'b> {
    #[doc = "Index and Id"]
    IndexId(&'b str, &'b str),
}
impl<'b> UpdateParts<'b> {
    #[doc = "Builds a relative URL path to the Update API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateParts::IndexId(index, id) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(10usize + encoded_index.len() + encoded_id.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_update/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Update API](https://opensearch.org/docs/latest/api-reference/document-apis/update-document/)\n\nUpdates a document with a script or partial document."]
#[derive(Clone, Debug)]
pub struct Update<'a, 'b, B> {
    transport: &'a Transport,
    parts: UpdateParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    lang: Option<&'b str>,
    pretty: Option<bool>,
    refresh: Option<Refresh>,
    request_timeout: Option<Duration>,
    require_alias: Option<bool>,
    retry_on_conflict: Option<i64>,
    routing: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
    wait_for_active_shards: Option<&'b str>,
}
impl<'a, 'b, B> Update<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [Update] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: UpdateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        Update {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            lang: None,
            pretty: None,
            refresh: None,
            request_timeout: None,
            require_alias: None,
            retry_on_conflict: None,
            routing: None,
            source: None,
            timeout: None,
            wait_for_active_shards: None,
        }
    }
    #[doc = "Set to `false` to disable source retrieval. You can also specify a comma-separated\nlist of the fields you want to retrieve."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "Specify the source fields you want to exclude."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "Specify the source fields you want to retrieve."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> Update<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        Update {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            lang: self.lang,
            pretty: self.pretty,
            refresh: self.refresh,
            request_timeout: self.request_timeout,
            require_alias: self.require_alias,
            retry_on_conflict: self.retry_on_conflict,
            routing: self.routing,
            source: self.source,
            timeout: self.timeout,
            wait_for_active_shards: self.wait_for_active_shards,
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
    #[doc = "Only perform the operation if the document has this primary term."]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "Only perform the operation if the document has this sequence number."]
    pub fn if_seq_no(mut self, if_seq_no: i64) -> Self {
        self.if_seq_no = Some(if_seq_no);
        self
    }
    #[doc = "The script language."]
    pub fn lang(mut self, lang: &'b str) -> Self {
        self.lang = Some(lang);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "If 'true', OpenSearch refreshes the affected shards to make this operation\nvisible to search, if `wait_for` then wait for a refresh to make this operation\nvisible to search, if `false` do nothing with refreshes."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, the destination must be an index alias."]
    pub fn require_alias(mut self, require_alias: bool) -> Self {
        self.require_alias = Some(require_alias);
        self
    }
    #[doc = "Specify how many times should the operation be retried when a conflict occurs."]
    pub fn retry_on_conflict(mut self, retry_on_conflict: i64) -> Self {
        self.retry_on_conflict = Some(retry_on_conflict);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Period to wait for dynamic mapping updates and active shards.\nThis guarantees OpenSearch waits for at least the timeout before failing.\nThe actual wait time could be longer, particularly when multiple waits occur."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operations.\nSet to 'all' or any positive integer up to the total number of shards in the index\n(number_of_replicas+1). Defaults to 1 meaning the primary shard."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "Creates an asynchronous call to the Update API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                lang: Option<&'b str>,
                pretty: Option<bool>,
                refresh: Option<Refresh>,
                require_alias: Option<bool>,
                retry_on_conflict: Option<i64>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
                wait_for_active_shards: Option<&'b str>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                lang: self.lang,
                pretty: self.pretty,
                refresh: self.refresh,
                require_alias: self.require_alias,
                retry_on_conflict: self.retry_on_conflict,
                routing: self.routing,
                source: self.source,
                timeout: self.timeout,
                wait_for_active_shards: self.wait_for_active_shards,
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
#[doc = "API parts for the Update By Query API"]
pub enum UpdateByQueryParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> UpdateByQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Update By Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_index.len());
                p.push('/');
                p.push_str(encoded_index.as_ref());
                p.push_str("/_update_by_query");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Update By Query API](https://opensearch.org/docs/latest/api-reference/document-apis/update-by-query/)\n\nPerforms an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
#[derive(Clone, Debug)]
pub struct UpdateByQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: UpdateByQueryParts<'b>,
    _source: Option<&'b str>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    body: Option<B>,
    conflicts: Option<Conflicts>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_docs: Option<i64>,
    pipeline: Option<&'b str>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    refresh: Option<Refresh>,
    request_cache: Option<bool>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    scroll_size: Option<i64>,
    search_timeout: Option<&'b str>,
    search_type: Option<SearchType>,
    size: Option<i64>,
    slices: Option<&'b str>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    version: Option<bool>,
    wait_for_active_shards: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> UpdateByQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [UpdateByQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: UpdateByQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        UpdateByQuery {
            transport,
            parts,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            analyze_wildcard: None,
            analyzer: None,
            body: None,
            conflicts: None,
            default_operator: None,
            df: None,
            error_trace: None,
            expand_wildcards: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_unavailable: None,
            lenient: None,
            max_docs: None,
            pipeline: None,
            preference: None,
            pretty: None,
            q: None,
            refresh: None,
            request_cache: None,
            request_timeout: None,
            requests_per_second: None,
            routing: None,
            scroll: None,
            scroll_size: None,
            search_timeout: None,
            search_type: None,
            size: None,
            slices: None,
            sort: None,
            source: None,
            stats: None,
            terminate_after: None,
            timeout: None,
            version: None,
            wait_for_active_shards: None,
            wait_for_completion: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b str) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "List of fields to exclude from the returned `_source` field."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "List of fields to extract and return from the `_source` field."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "Analyzer to use for the query string."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> UpdateByQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        UpdateByQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            conflicts: self.conflicts,
            default_operator: self.default_operator,
            df: self.df,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_docs: self.max_docs,
            pipeline: self.pipeline,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            refresh: self.refresh,
            request_cache: self.request_cache,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
            routing: self.routing,
            scroll: self.scroll,
            scroll_size: self.scroll_size,
            search_timeout: self.search_timeout,
            search_type: self.search_type,
            size: self.size,
            slices: self.slices,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            version: self.version,
            wait_for_active_shards: self.wait_for_active_shards,
            wait_for_completion: self.wait_for_completion,
        }
    }
    #[doc = "What to do if update by query hits version conflicts: `abort` or `proceed`."]
    pub fn conflicts(mut self, conflicts: Conflicts) -> Self {
        self.conflicts = Some(conflicts);
        self
    }
    #[doc = "The default operator for query string query: `AND` or `OR`."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "Field to use as default where no field prefix is given in the query string."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`.\nValid values are: `all`, `open`, `closed`, `hidden`, `none`."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset."]
    pub fn from(mut self, from: i64) -> Self {
        self.from = Some(from);
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
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "Maximum number of documents to process.\nDefaults to all documents."]
    pub fn max_docs(mut self, max_docs: i64) -> Self {
        self.max_docs = Some(max_docs);
        self
    }
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(mut self, pipeline: &'b str) -> Self {
        self.pipeline = Some(pipeline);
        self
    }
    #[doc = "Specifies the node or shard the operation should be performed on.\nRandom by default."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "If `true`, OpenSearch refreshes affected shards to make the operation visible to search."]
    pub fn refresh(mut self, refresh: Refresh) -> Self {
        self.refresh = Some(refresh);
        self
    }
    #[doc = "If `true`, the request cache is used for this request."]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The throttle for this request in sub-requests per second."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Period to retain the search context for scrolling."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Size of the scroll request that powers the operation."]
    pub fn scroll_size(mut self, scroll_size: i64) -> Self {
        self.scroll_size = Some(scroll_size);
        self
    }
    #[doc = "Explicit timeout for each search request."]
    pub fn search_timeout(mut self, search_timeout: &'b str) -> Self {
        self.search_timeout = Some(search_timeout);
        self
    }
    #[doc = "The type of the search operation. Available options: `query_then_fetch`, `dfs_query_then_fetch`."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Deprecated, use `max_docs` instead."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The number of slices this task should be divided into."]
    pub fn slices(mut self, slices: &'b str) -> Self {
        self.slices = Some(slices);
        self
    }
    #[doc = "A comma-separated list of &lt;field&gt;:&lt;direction&gt; pairs."]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific `tag` of the request for logging and statistical purposes."]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "Maximum number of documents to collect for each shard.\nIf a query reaches this limit, OpenSearch terminates the query early.\nOpenSearch collects documents before sorting.\nUse with caution.\nOpenSearch applies this parameter to each shard handling the request.\nWhen possible, let OpenSearch perform early termination automatically.\nAvoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Period each update request waits for the following operations: dynamic mapping updates, waiting for active shards."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "If `true`, returns the document version as part of a hit."]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to `all` or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(mut self, wait_for_active_shards: &'b str) -> Self {
        self.wait_for_active_shards = Some(wait_for_active_shards);
        self
    }
    #[doc = "If `true`, the request blocks until the operation is complete."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Update By Query API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                _source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                allow_no_indices: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                conflicts: Option<Conflicts>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                ignore_unavailable: Option<bool>,
                lenient: Option<bool>,
                max_docs: Option<i64>,
                pipeline: Option<&'b str>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                refresh: Option<Refresh>,
                request_cache: Option<bool>,
                requests_per_second: Option<f32>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                scroll: Option<&'b str>,
                scroll_size: Option<i64>,
                search_timeout: Option<&'b str>,
                search_type: Option<SearchType>,
                size: Option<i64>,
                slices: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                terminate_after: Option<i64>,
                timeout: Option<&'b str>,
                version: Option<bool>,
                wait_for_active_shards: Option<&'b str>,
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                conflicts: self.conflicts,
                default_operator: self.default_operator,
                df: self.df,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_docs: self.max_docs,
                pipeline: self.pipeline,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                refresh: self.refresh,
                request_cache: self.request_cache,
                requests_per_second: self.requests_per_second,
                routing: self.routing,
                scroll: self.scroll,
                scroll_size: self.scroll_size,
                search_timeout: self.search_timeout,
                search_type: self.search_type,
                size: self.size,
                slices: self.slices,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                version: self.version,
                wait_for_active_shards: self.wait_for_active_shards,
                wait_for_completion: self.wait_for_completion,
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
#[doc = "API parts for the Update By Query Rethrottle API"]
pub enum UpdateByQueryRethrottleParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> UpdateByQueryRethrottleParts<'b> {
    #[doc = "Builds a relative URL path to the Update By Query Rethrottle API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            UpdateByQueryRethrottleParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_task_id.len());
                p.push_str("/_update_by_query/");
                p.push_str(encoded_task_id.as_ref());
                p.push_str("/_rethrottle");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Update By Query Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular Update By Query operation."]
#[derive(Clone, Debug)]
pub struct UpdateByQueryRethrottle<'a, 'b, B> {
    transport: &'a Transport,
    parts: UpdateByQueryRethrottleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    requests_per_second: Option<f32>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> UpdateByQueryRethrottle<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [UpdateByQueryRethrottle] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: UpdateByQueryRethrottleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        UpdateByQueryRethrottle {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            requests_per_second: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> UpdateByQueryRethrottle<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        UpdateByQueryRethrottle {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            requests_per_second: self.requests_per_second,
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
    #[doc = "The throttle for this request in sub-requests per second."]
    pub fn requests_per_second(mut self, requests_per_second: f32) -> Self {
        self.requests_per_second = Some(requests_per_second);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Update By Query Rethrottle API that can be awaited"]
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
                requests_per_second: Option<f32>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                requests_per_second: self.requests_per_second,
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
impl OpenSearch {
    #[doc = "[Bulk API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk/)\n\nAllows to perform multiple index/update/delete operations in a single request."]
    pub fn bulk<'a, 'b>(&'a self, parts: BulkParts<'b>) -> Bulk<'a, 'b, ()> {
        Bulk::new(self.transport(), parts)
    }
    #[doc = "[Bulk Stream API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk-streaming/)\n\nAllows to perform multiple index/update/delete operations using request response streaming."]
    pub fn bulk_stream<'a, 'b>(&'a self, parts: BulkStreamParts<'b>) -> BulkStream<'a, 'b, ()> {
        BulkStream::new(self.transport(), parts)
    }
    #[doc = "[Clear Scroll API](https://opensearch.org/docs/latest/api-reference/scroll/)\n\nExplicitly clears the search context for a scroll."]
    pub fn clear_scroll<'a, 'b>(&'a self, parts: ClearScrollParts<'b>) -> ClearScroll<'a, 'b, ()> {
        ClearScroll::new(self.transport(), parts)
    }
    #[doc = "[Count API](https://opensearch.org/docs/latest/api-reference/count/)\n\nReturns number of documents matching a query."]
    pub fn count<'a, 'b>(&'a self, parts: CountParts<'b>) -> Count<'a, 'b, ()> {
        Count::new(self.transport(), parts)
    }
    #[doc = "[Create API](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)\n\nCreates a new document in the index.\n\nReturns a 409 response when a document with a same ID already exists in the index."]
    pub fn create<'a, 'b>(&'a self, parts: CreateParts<'b>) -> Create<'a, 'b, ()> {
        Create::new(self.transport(), parts)
    }
    #[doc = "[Create Pit API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#create-a-pit)\n\nCreates point in time context."]
    pub fn create_pit<'a, 'b>(&'a self, parts: CreatePitParts<'b>) -> CreatePit<'a, 'b, ()> {
        CreatePit::new(self.transport(), parts)
    }
    #[doc = "[Delete API](https://opensearch.org/docs/latest/api-reference/document-apis/delete-document/)\n\nRemoves a document from the index."]
    pub fn delete<'a, 'b>(&'a self, parts: DeleteParts<'b>) -> Delete<'a, 'b> {
        Delete::new(self.transport(), parts)
    }
    #[doc = "[Delete All Pits API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#delete-pits)\n\nDeletes all active point in time searches."]
    pub fn delete_all_pits<'a, 'b>(&'a self) -> DeleteAllPits<'a, 'b> {
        DeleteAllPits::new(self.transport())
    }
    #[doc = "[Delete By Query API](https://opensearch.org/docs/latest/api-reference/document-apis/delete-by-query/)\n\nDeletes documents matching the provided query."]
    pub fn delete_by_query<'a, 'b>(
        &'a self,
        parts: DeleteByQueryParts<'b>,
    ) -> DeleteByQuery<'a, 'b, ()> {
        DeleteByQuery::new(self.transport(), parts)
    }
    #[doc = "[Delete By Query Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular Delete By Query operation."]
    pub fn delete_by_query_rethrottle<'a, 'b>(
        &'a self,
        parts: DeleteByQueryRethrottleParts<'b>,
    ) -> DeleteByQueryRethrottle<'a, 'b, ()> {
        DeleteByQueryRethrottle::new(self.transport(), parts)
    }
    #[doc = "[Delete Pit API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#delete-pits)\n\nDeletes one or more point in time searches based on the IDs passed."]
    pub fn delete_pit<'a, 'b>(&'a self) -> DeletePit<'a, 'b, ()> {
        DeletePit::new(self.transport())
    }
    #[doc = "[Delete Script API](https://opensearch.org/docs/latest/api-reference/script-apis/delete-script/)\n\nDeletes a script."]
    pub fn delete_script<'a, 'b>(&'a self, parts: DeleteScriptParts<'b>) -> DeleteScript<'a, 'b> {
        DeleteScript::new(self.transport(), parts)
    }
    #[doc = "[Exists API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns information about whether a document exists in an index."]
    pub fn exists<'a, 'b>(&'a self, parts: ExistsParts<'b>) -> Exists<'a, 'b> {
        Exists::new(self.transport(), parts)
    }
    #[doc = "[Exists Source API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns information about whether a document source exists in an index."]
    pub fn exists_source<'a, 'b>(&'a self, parts: ExistsSourceParts<'b>) -> ExistsSource<'a, 'b> {
        ExistsSource::new(self.transport(), parts)
    }
    #[doc = "[Explain API](https://opensearch.org/docs/latest/api-reference/explain/)\n\nReturns information about why a specific document matches (or doesn't match) a query."]
    pub fn explain<'a, 'b>(&'a self, parts: ExplainParts<'b>) -> Explain<'a, 'b, ()> {
        Explain::new(self.transport(), parts)
    }
    #[doc = "[Field Caps API](https://opensearch.org/docs/latest/field-types/supported-field-types/alias/#using-aliases-in-field-capabilities-api-operations)\n\nReturns the information about the capabilities of fields among multiple indexes."]
    pub fn field_caps<'a, 'b>(&'a self, parts: FieldCapsParts<'b>) -> FieldCaps<'a, 'b, ()> {
        FieldCaps::new(self.transport(), parts)
    }
    #[doc = "[Get API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns a document."]
    pub fn get<'a, 'b>(&'a self, parts: GetParts<'b>) -> Get<'a, 'b> {
        Get::new(self.transport(), parts)
    }
    #[doc = "[Get All Pits API](https://opensearch.org/docs/latest/search-plugins/point-in-time-api/#list-all-pits)\n\nLists all active point in time searches."]
    pub fn get_all_pits<'a, 'b>(&'a self) -> GetAllPits<'a, 'b> {
        GetAllPits::new(self.transport())
    }
    #[doc = "[Get Script API](https://opensearch.org/docs/latest/api-reference/script-apis/get-stored-script/)\n\nReturns a script."]
    pub fn get_script<'a, 'b>(&'a self, parts: GetScriptParts<'b>) -> GetScript<'a, 'b> {
        GetScript::new(self.transport(), parts)
    }
    #[doc = "[Get Script Context API](https://opensearch.org/docs/latest/api-reference/script-apis/get-script-contexts/)\n\nReturns all script contexts."]
    pub fn get_script_context<'a, 'b>(&'a self) -> GetScriptContext<'a, 'b> {
        GetScriptContext::new(self.transport())
    }
    #[doc = "[Get Script Languages API](https://opensearch.org/docs/latest/api-reference/script-apis/get-script-language/)\n\nReturns available script types, languages and contexts."]
    pub fn get_script_languages<'a, 'b>(&'a self) -> GetScriptLanguages<'a, 'b> {
        GetScriptLanguages::new(self.transport())
    }
    #[doc = "[Get Source API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns the source of a document."]
    pub fn get_source<'a, 'b>(&'a self, parts: GetSourceParts<'b>) -> GetSource<'a, 'b> {
        GetSource::new(self.transport(), parts)
    }
    #[doc = "[Index API](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)\n\nCreates or updates a document in an index."]
    pub fn index<'a, 'b>(&'a self, parts: IndexParts<'b>) -> Index<'a, 'b, ()> {
        Index::new(self.transport(), parts)
    }
    #[doc = "[Info API](https://opensearch.org/docs/latest)\n\nReturns basic information about the cluster."]
    pub fn info<'a, 'b>(&'a self) -> Info<'a, 'b> {
        Info::new(self.transport())
    }
    #[doc = "[Mget API](https://opensearch.org/docs/latest/api-reference/document-apis/multi-get/)\n\nAllows to get multiple documents in one request."]
    pub fn mget<'a, 'b>(&'a self, parts: MgetParts<'b>) -> Mget<'a, 'b, ()> {
        Mget::new(self.transport(), parts)
    }
    #[doc = "[Msearch API](https://opensearch.org/docs/latest/api-reference/multi-search/)\n\nAllows to execute several search operations in one request."]
    pub fn msearch<'a, 'b>(&'a self, parts: MsearchParts<'b>) -> Msearch<'a, 'b, ()> {
        Msearch::new(self.transport(), parts)
    }
    #[doc = "[Msearch Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to execute several search template operations in one request."]
    pub fn msearch_template<'a, 'b>(
        &'a self,
        parts: MsearchTemplateParts<'b>,
    ) -> MsearchTemplate<'a, 'b, ()> {
        MsearchTemplate::new(self.transport(), parts)
    }
    #[doc = "[Mtermvectors API](https://opensearch.org/docs/latest)\n\nReturns multiple termvectors in one request."]
    pub fn mtermvectors<'a, 'b>(
        &'a self,
        parts: MtermvectorsParts<'b>,
    ) -> Mtermvectors<'a, 'b, ()> {
        Mtermvectors::new(self.transport(), parts)
    }
    #[doc = "[Ping API](https://opensearch.org/docs/latest)\n\nReturns whether the cluster is running."]
    pub fn ping<'a, 'b>(&'a self) -> Ping<'a, 'b> {
        Ping::new(self.transport())
    }
    #[doc = "[Put Script API](https://opensearch.org/docs/latest/api-reference/script-apis/create-stored-script/)\n\nCreates or updates a script."]
    pub fn put_script<'a, 'b>(&'a self, parts: PutScriptParts<'b>) -> PutScript<'a, 'b, ()> {
        PutScript::new(self.transport(), parts)
    }
    #[doc = "[Rank Eval API](https://opensearch.org/docs/latest/api-reference/rank-eval/)\n\nAllows to evaluate the quality of ranked search results over a set of typical search queries."]
    pub fn rank_eval<'a, 'b>(&'a self, parts: RankEvalParts<'b>) -> RankEval<'a, 'b, ()> {
        RankEval::new(self.transport(), parts)
    }
    #[doc = "[Reindex API](https://opensearch.org/docs/latest/im-plugin/reindex-data/)\n\nAllows to copy documents from one index to another, optionally filtering the source\ndocuments by a query, changing the destination index settings, or fetching the\ndocuments from a remote cluster."]
    pub fn reindex<'a, 'b>(&'a self) -> Reindex<'a, 'b, ()> {
        Reindex::new(self.transport())
    }
    #[doc = "[Reindex Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular reindex operation."]
    pub fn reindex_rethrottle<'a, 'b>(
        &'a self,
        parts: ReindexRethrottleParts<'b>,
    ) -> ReindexRethrottle<'a, 'b, ()> {
        ReindexRethrottle::new(self.transport(), parts)
    }
    #[doc = "[Render Search Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to use the Mustache language to pre-render a search definition."]
    pub fn render_search_template<'a, 'b>(
        &'a self,
        parts: RenderSearchTemplateParts<'b>,
    ) -> RenderSearchTemplate<'a, 'b, ()> {
        RenderSearchTemplate::new(self.transport(), parts)
    }
    #[doc = "[Scripts Painless Execute API](https://opensearch.org/docs/latest/api-reference/script-apis/exec-script/)\n\nAllows an arbitrary script to be executed and a result to be returned."]
    pub fn scripts_painless_execute<'a, 'b>(&'a self) -> ScriptsPainlessExecute<'a, 'b, ()> {
        ScriptsPainlessExecute::new(self.transport())
    }
    #[doc = "[Scroll API](https://opensearch.org/docs/latest/api-reference/scroll/#path-and-http-methods)\n\nAllows to retrieve a large numbers of results from a single search request.\n\n# Examples\n\nTo initiate a scroll, make search API call with a specified `scroll` timeout,\nthen fetch the next set of hits using the `_scroll_id` returned in\nthe response. Once no more hits are returned, clear the scroll.\n\n```rust,no_run\n# use opensearch::{OpenSearch, Error, SearchParts, ScrollParts, ClearScrollParts};\n# use serde_json::{json, Value};\n# async fn doc() -> Result<(), Box<dyn std::error::Error>> {\nlet client = OpenSearch::default();\n\nfn print_hits(hits: &[Value]) {\n    for hit in hits {\n        println!(\n            \"id: '{}', source: '{}', score: '{}'\",\n            hit[\"_id\"].as_str().unwrap(),\n            hit[\"_source\"],\n            hit[\"_score\"].as_f64().unwrap()\n        );\n    }\n}\n\nlet scroll = \"1m\";\nlet mut response = client\n    .search(SearchParts::Index(&[\"tweets\"]))\n    .scroll(scroll)\n    .body(json!({\n        \"query\": {\n            \"match\": {\n                \"body\": {\n                    \"query\": \"OpenSearch rust\",\n                    \"operator\": \"AND\"\n                }\n            }\n        }\n    }))\n    .send()\n    .await?;\n\nlet mut response_body = response.json::<Value>().await?;\nlet mut scroll_id = response_body[\"_scroll_id\"].as_str().unwrap();\nlet mut hits = response_body[\"hits\"][\"hits\"].as_array().unwrap();\n\nprint_hits(hits);\n\nwhile hits.len() > 0 {\n    response = client\n        .scroll(ScrollParts::None)\n        .body(json!({\n            \"scroll\": scroll,\n            \"scroll_id\": scroll_id\n        }))\n        .send()\n        .await?;\n\n    response_body = response.json::<Value>().await?;\n    scroll_id = response_body[\"_scroll_id\"].as_str().unwrap();\n    hits = response_body[\"hits\"][\"hits\"].as_array().unwrap();\n    print_hits(hits);\n}\n\nresponse = client\n    .clear_scroll(ClearScrollParts::None)\n    .body(json!({\n        \"scroll_id\": scroll_id\n    }))\n    .send()\n    .await?;\n    \n# Ok(())\n# }\n```"]
    pub fn scroll<'a, 'b>(&'a self, parts: ScrollParts<'b>) -> Scroll<'a, 'b, ()> {
        Scroll::new(self.transport(), parts)
    }
    #[doc = "[Search API](https://opensearch.org/docs/latest/api-reference/search/)\n\nReturns results matching a query."]
    pub fn search<'a, 'b>(&'a self, parts: SearchParts<'b>) -> Search<'a, 'b, ()> {
        Search::new(self.transport(), parts)
    }
    #[doc = "[Search Shards API](https://opensearch.org/docs/latest)\n\nReturns information about the indexes and shards that a search request would be executed against."]
    pub fn search_shards<'a, 'b>(
        &'a self,
        parts: SearchShardsParts<'b>,
    ) -> SearchShards<'a, 'b, ()> {
        SearchShards::new(self.transport(), parts)
    }
    #[doc = "[Search Template API](https://opensearch.org/docs/latest/search-plugins/search-template/)\n\nAllows to use the Mustache language to pre-render a search definition."]
    pub fn search_template<'a, 'b>(
        &'a self,
        parts: SearchTemplateParts<'b>,
    ) -> SearchTemplate<'a, 'b, ()> {
        SearchTemplate::new(self.transport(), parts)
    }
    #[doc = "[Termvectors API](https://opensearch.org/docs/latest)\n\nReturns information and statistics about terms in the fields of a particular document."]
    pub fn termvectors<'a, 'b>(&'a self, parts: TermvectorsParts<'b>) -> Termvectors<'a, 'b, ()> {
        Termvectors::new(self.transport(), parts)
    }
    #[doc = "[Update API](https://opensearch.org/docs/latest/api-reference/document-apis/update-document/)\n\nUpdates a document with a script or partial document."]
    pub fn update<'a, 'b>(&'a self, parts: UpdateParts<'b>) -> Update<'a, 'b, ()> {
        Update::new(self.transport(), parts)
    }
    #[doc = "[Update By Query API](https://opensearch.org/docs/latest/api-reference/document-apis/update-by-query/)\n\nPerforms an update on every document in the index without changing the source,\nfor example to pick up a mapping change."]
    pub fn update_by_query<'a, 'b>(
        &'a self,
        parts: UpdateByQueryParts<'b>,
    ) -> UpdateByQuery<'a, 'b, ()> {
        UpdateByQuery::new(self.transport(), parts)
    }
    #[doc = "[Update By Query Rethrottle API](https://opensearch.org/docs/latest)\n\nChanges the number of requests per second for a particular Update By Query operation."]
    pub fn update_by_query_rethrottle<'a, 'b>(
        &'a self,
        parts: UpdateByQueryRethrottleParts<'b>,
    ) -> UpdateByQueryRethrottle<'a, 'b, ()> {
        UpdateByQueryRethrottle::new(self.transport(), parts)
    }
}

mod bulk;
pub use bulk::*;
