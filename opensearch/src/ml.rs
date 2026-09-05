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
#[doc = "API parts for the Ml Add Agentic Memory API"]
pub enum MlAddAgenticMemoryParts<'b> {
    #[doc = "MemoryContainerId"]
    MemoryContainerId(&'b str),
}
impl<'b> MlAddAgenticMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Add Agentic Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlAddAgenticMemoryParts::MemoryContainerId(memory_container_id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_memory_container_id.len());
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Add Agentic Memory API\n\nAdd agentic memory to a memory container."]
#[derive(Clone, Debug)]
pub struct MlAddAgenticMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlAddAgenticMemoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlAddAgenticMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlAddAgenticMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlAddAgenticMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlAddAgenticMemory {
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
    pub fn body<T>(self, body: T) -> MlAddAgenticMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlAddAgenticMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Add Agentic Memory API that can be awaited"]
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
#[deprecated = "Use `upload_chunk` instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Chunk Model API"]
pub enum MlChunkModelParts<'b> {
    #[doc = "ModelId and ChunkNumber"]
    ModelIdChunkNumber(&'b str, i64),
}
#[allow(deprecated)]
impl<'b> MlChunkModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Chunk Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlChunkModelParts::ModelIdChunkNumber(model_id, chunk_number) => {
                let chunk_number_str = chunk_number.to_string();
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_chunk_number: Cow<str> =
                    percent_encode(chunk_number_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    28usize + encoded_model_id.len() + encoded_chunk_number.len(),
                );
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/chunk/");
                p.push_str(encoded_chunk_number.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Chunk Model API\n\nUploads model chunk."]
#[deprecated = "Use `upload_chunk` instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct MlChunkModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlChunkModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[allow(deprecated)]
impl<'a, 'b, B> MlChunkModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlChunkModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlChunkModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlChunkModel {
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
    pub fn body<T>(self, body: T) -> MlChunkModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlChunkModel {
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
    #[doc = "Creates an asynchronous call to the Ml Chunk Model API that can be awaited"]
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
#[doc = "API parts for the Ml Create Connector API"]
pub enum MlCreateConnectorParts {
    #[doc = "No parts"]
    None,
}
impl MlCreateConnectorParts {
    #[doc = "Builds a relative URL path to the Ml Create Connector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateConnectorParts::None => "/_plugins/_ml/connectors/_create".into(),
        }
    }
}
#[doc = "Builder for the Ml Create Connector API\n\nCreates a standalone connector."]
#[derive(Clone, Debug)]
pub struct MlCreateConnector<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateConnectorParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateConnector<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateConnector]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlCreateConnector {
            transport,
            parts: MlCreateConnectorParts::None,
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
    pub fn body<T>(self, body: T) -> MlCreateConnector<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateConnector {
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
    #[doc = "Creates an asynchronous call to the Ml Create Connector API that can be awaited"]
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
#[doc = "API parts for the Ml Create Controller API"]
pub enum MlCreateControllerParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlCreateControllerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Create Controller API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateControllerParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/controllers/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Create Controller API\n\nCreates a controller."]
#[derive(Clone, Debug)]
pub struct MlCreateController<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateControllerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateController<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateController] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlCreateControllerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlCreateController {
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
    pub fn body<T>(self, body: T) -> MlCreateController<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateController {
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
    #[doc = "Creates an asynchronous call to the Ml Create Controller API that can be awaited"]
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
#[doc = "API parts for the Ml Create Memory API"]
pub enum MlCreateMemoryParts {
    #[doc = "No parts"]
    None,
}
impl MlCreateMemoryParts {
    #[doc = "Builds a relative URL path to the Ml Create Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateMemoryParts::None => "/_plugins/_ml/memory".into(),
        }
    }
}
#[doc = "Builder for the Ml Create Memory API\n\nCreate a memory."]
#[derive(Clone, Debug)]
pub struct MlCreateMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateMemoryParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateMemory]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlCreateMemory {
            transport,
            parts: MlCreateMemoryParts::None,
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
    pub fn body<T>(self, body: T) -> MlCreateMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Create Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Create Memory Container API"]
pub enum MlCreateMemoryContainerParts {
    #[doc = "No parts"]
    None,
}
impl MlCreateMemoryContainerParts {
    #[doc = "Builds a relative URL path to the Ml Create Memory Container API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateMemoryContainerParts::None => "/_plugins/_ml/memory_containers/_create".into(),
        }
    }
}
#[doc = "Builder for the Ml Create Memory Container API\n\nCreate a memory container."]
#[derive(Clone, Debug)]
pub struct MlCreateMemoryContainer<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateMemoryContainerParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateMemoryContainer<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateMemoryContainer]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlCreateMemoryContainer {
            transport,
            parts: MlCreateMemoryContainerParts::None,
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
    pub fn body<T>(self, body: T) -> MlCreateMemoryContainer<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateMemoryContainer {
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
    #[doc = "Creates an asynchronous call to the Ml Create Memory Container API that can be awaited"]
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
#[doc = "API parts for the Ml Create Memory Container Session API"]
pub enum MlCreateMemoryContainerSessionParts<'b> {
    #[doc = "MemoryContainerId"]
    MemoryContainerId(&'b str),
}
impl<'b> MlCreateMemoryContainerSessionParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Create Memory Container Session API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateMemoryContainerSessionParts::MemoryContainerId(memory_container_id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(50usize + encoded_memory_container_id.len());
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/sessions");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Create Memory Container Session API\n\nCreate session in a memory container."]
#[derive(Clone, Debug)]
pub struct MlCreateMemoryContainerSession<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateMemoryContainerSessionParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateMemoryContainerSession<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateMemoryContainerSession] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlCreateMemoryContainerSessionParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlCreateMemoryContainerSession {
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
    pub fn body<T>(self, body: T) -> MlCreateMemoryContainerSession<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateMemoryContainerSession {
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
    #[doc = "Creates an asynchronous call to the Ml Create Memory Container Session API that can be awaited"]
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
#[doc = "API parts for the Ml Create Message API"]
pub enum MlCreateMessageParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlCreateMessageParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Create Message API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateMessageParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.push_str("/messages");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Create Message API\n\nCreate a message."]
#[derive(Clone, Debug)]
pub struct MlCreateMessage<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateMessageParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlCreateMessage<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateMessage] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlCreateMessageParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlCreateMessage {
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
    pub fn body<T>(self, body: T) -> MlCreateMessage<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateMessage {
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
    #[doc = "Creates an asynchronous call to the Ml Create Message API that can be awaited"]
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
#[deprecated = "Use `_register_meta` instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Create Model Meta API"]
pub enum MlCreateModelMetaParts {
    #[doc = "No parts"]
    None,
}
#[allow(deprecated)]
impl MlCreateModelMetaParts {
    #[doc = "Builds a relative URL path to the Ml Create Model Meta API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlCreateModelMetaParts::None => "/_plugins/_ml/models/meta".into(),
        }
    }
}
#[doc = "Builder for the Ml Create Model Meta API\n\nRegisters model metadata."]
#[deprecated = "Use `_register_meta` instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct MlCreateModelMeta<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlCreateModelMetaParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[allow(deprecated)]
impl<'a, 'b, B> MlCreateModelMeta<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlCreateModelMeta]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlCreateModelMeta {
            transport,
            parts: MlCreateModelMetaParts::None,
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
    pub fn body<T>(self, body: T) -> MlCreateModelMeta<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlCreateModelMeta {
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
    #[doc = "Creates an asynchronous call to the Ml Create Model Meta API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Agent API"]
pub enum MlDeleteAgentParts<'b> {
    #[doc = "AgentId"]
    AgentId(&'b str),
}
impl<'b> MlDeleteAgentParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Agent API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteAgentParts::AgentId(agent_id) => {
                let encoded_agent_id: Cow<str> =
                    percent_encode(agent_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_agent_id.len());
                p.push_str("/_plugins/_ml/agents/");
                p.push_str(encoded_agent_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Agent API\n\nDelete an agent."]
#[derive(Clone, Debug)]
pub struct MlDeleteAgent<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteAgentParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteAgent<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteAgent] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteAgentParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteAgent {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Agent API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Agentic Memory API"]
pub enum MlDeleteAgenticMemoryParts<'b> {
    #[doc = "MemoryContainerId, Type and Id"]
    MemoryContainerIdTypeId(&'b str, &'b str, &'b str),
}
impl<'b> MlDeleteAgenticMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Agentic Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteAgenticMemoryParts::MemoryContainerIdTypeId(memory_container_id, ty, id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    43usize
                        + encoded_memory_container_id.len()
                        + encoded_ty.len()
                        + encoded_id.len(),
                );
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/");
                p.push_str(encoded_ty.as_ref());
                p.push('/');
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Agentic Memory API\n\nDelete a specific memory by its type and ID."]
#[derive(Clone, Debug)]
pub struct MlDeleteAgenticMemory<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteAgenticMemoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteAgenticMemory<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteAgenticMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteAgenticMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteAgenticMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Agentic Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Agentic Memory Query API"]
pub enum MlDeleteAgenticMemoryQueryParts<'b> {
    #[doc = "MemoryContainerId and Type"]
    MemoryContainerIdType(&'b str, &'b str),
}
impl<'b> MlDeleteAgenticMemoryQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Agentic Memory Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteAgenticMemoryQueryParts::MemoryContainerIdType(memory_container_id, ty) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    59usize + encoded_memory_container_id.len() + encoded_ty.len(),
                );
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_delete_by_query");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Agentic Memory Query API\n\nDelete multiple memories using a query to match specific criteria."]
#[derive(Clone, Debug)]
pub struct MlDeleteAgenticMemoryQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlDeleteAgenticMemoryQueryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlDeleteAgenticMemoryQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlDeleteAgenticMemoryQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteAgenticMemoryQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteAgenticMemoryQuery {
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
    pub fn body<T>(self, body: T) -> MlDeleteAgenticMemoryQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlDeleteAgenticMemoryQuery {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Agentic Memory Query API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Connector API"]
pub enum MlDeleteConnectorParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
impl<'b> MlDeleteConnectorParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Connector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteConnectorParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_connector_id.len());
                p.push_str("/_plugins/_ml/connectors/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Connector API\n\nDeletes a standalone connector."]
#[derive(Clone, Debug)]
pub struct MlDeleteConnector<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteConnectorParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteConnector<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteConnector] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteConnectorParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteConnector {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Connector API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Controller API"]
pub enum MlDeleteControllerParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlDeleteControllerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Controller API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteControllerParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/controllers/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Controller API\n\nDeletes a controller."]
#[derive(Clone, Debug)]
pub struct MlDeleteController<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteControllerParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteController<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteController] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteControllerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteController {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Controller API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Memory API"]
pub enum MlDeleteMemoryParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlDeleteMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteMemoryParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Memory API\n\nDelete a memory."]
#[derive(Clone, Debug)]
pub struct MlDeleteMemory<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteMemoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteMemory<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Memory Container API"]
pub enum MlDeleteMemoryContainerParts<'b> {
    #[doc = "MemoryContainerId"]
    MemoryContainerId(&'b str),
}
impl<'b> MlDeleteMemoryContainerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Memory Container API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteMemoryContainerParts::MemoryContainerId(memory_container_id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_memory_container_id.len());
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Memory Container API\n\nDelete a memory container."]
#[derive(Clone, Debug)]
pub struct MlDeleteMemoryContainer<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteMemoryContainerParts<'b>,
    delete_all_memories: Option<bool>,
    delete_memories: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteMemoryContainer<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteMemoryContainer] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteMemoryContainerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteMemoryContainer {
            transport,
            parts,
            headers,
            delete_all_memories: None,
            delete_memories: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    pub fn delete_all_memories(mut self, delete_all_memories: bool) -> Self {
        self.delete_all_memories = Some(delete_all_memories);
        self
    }
    pub fn delete_memories(mut self, delete_memories: &'b [&'b str]) -> Self {
        self.delete_memories = Some(delete_memories);
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
    #[doc = "Creates an asynchronous call to the Ml Delete Memory Container API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                delete_all_memories: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                delete_memories: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                delete_all_memories: self.delete_all_memories,
                delete_memories: self.delete_memories,
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
#[doc = "API parts for the Ml Delete Model API"]
pub enum MlDeleteModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlDeleteModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Model API\n\nDeletes a model."]
#[derive(Clone, Debug)]
pub struct MlDeleteModel<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteModel<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteModel {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Model API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Model Group API"]
pub enum MlDeleteModelGroupParts<'b> {
    #[doc = "ModelGroupId"]
    ModelGroupId(&'b str),
}
impl<'b> MlDeleteModelGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Model Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteModelGroupParts::ModelGroupId(model_group_id) => {
                let encoded_model_group_id: Cow<str> =
                    percent_encode(model_group_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_model_group_id.len());
                p.push_str("/_plugins/_ml/model_groups/");
                p.push_str(encoded_model_group_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Model Group API\n\nDeletes a model group."]
#[derive(Clone, Debug)]
pub struct MlDeleteModelGroup<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteModelGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteModelGroup<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteModelGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteModelGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteModelGroup {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Model Group API that can be awaited"]
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
#[doc = "API parts for the Ml Delete Task API"]
pub enum MlDeleteTaskParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> MlDeleteTaskParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Delete Task API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeleteTaskParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_task_id.len());
                p.push_str("/_plugins/_ml/tasks/");
                p.push_str(encoded_task_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Delete Task API\n\nDeletes a task."]
#[derive(Clone, Debug)]
pub struct MlDeleteTask<'a, 'b> {
    transport: &'a Transport,
    parts: MlDeleteTaskParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlDeleteTask<'a, 'b> {
    #[doc = "Creates a new instance of [MlDeleteTask] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeleteTaskParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeleteTask {
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
    #[doc = "Creates an asynchronous call to the Ml Delete Task API that can be awaited"]
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
#[doc = "API parts for the Ml Deploy Model API"]
pub enum MlDeployModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlDeployModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Deploy Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlDeployModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_deploy");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Deploy Model API\n\nDeploys a model."]
#[derive(Clone, Debug)]
pub struct MlDeployModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlDeployModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlDeployModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlDeployModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlDeployModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlDeployModel {
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
    pub fn body<T>(self, body: T) -> MlDeployModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlDeployModel {
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
    #[doc = "Creates an asynchronous call to the Ml Deploy Model API that can be awaited"]
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
#[doc = "API parts for the Ml Execute Agent API"]
pub enum MlExecuteAgentParts<'b> {
    #[doc = "AgentId"]
    AgentId(&'b str),
}
impl<'b> MlExecuteAgentParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Execute Agent API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlExecuteAgentParts::AgentId(agent_id) => {
                let encoded_agent_id: Cow<str> =
                    percent_encode(agent_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_agent_id.len());
                p.push_str("/_plugins/_ml/agents/");
                p.push_str(encoded_agent_id.as_ref());
                p.push_str("/_execute");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Execute Agent API\n\nExecute an agent."]
#[derive(Clone, Debug)]
pub struct MlExecuteAgent<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlExecuteAgentParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlExecuteAgent<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlExecuteAgent] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlExecuteAgentParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlExecuteAgent {
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
    pub fn body<T>(self, body: T) -> MlExecuteAgent<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlExecuteAgent {
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
    #[doc = "Creates an asynchronous call to the Ml Execute Agent API that can be awaited"]
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
#[doc = "API parts for the Ml Execute Agent Stream API"]
pub enum MlExecuteAgentStreamParts<'b> {
    #[doc = "AgentId"]
    AgentId(&'b str),
}
impl<'b> MlExecuteAgentStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Execute Agent Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlExecuteAgentStreamParts::AgentId(agent_id) => {
                let encoded_agent_id: Cow<str> =
                    percent_encode(agent_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(37usize + encoded_agent_id.len());
                p.push_str("/_plugins/_ml/agents/");
                p.push_str(encoded_agent_id.as_ref());
                p.push_str("/_execute/stream");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Execute Agent Stream API\n\nExecute an agent in streaming mode."]
#[derive(Clone, Debug)]
pub struct MlExecuteAgentStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlExecuteAgentStreamParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlExecuteAgentStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlExecuteAgentStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlExecuteAgentStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlExecuteAgentStream {
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
    pub fn body<T>(self, body: T) -> MlExecuteAgentStream<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlExecuteAgentStream {
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
    #[doc = "Creates an asynchronous call to the Ml Execute Agent Stream API that can be awaited"]
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
#[doc = "API parts for the Ml Execute Algorithm API"]
pub enum MlExecuteAlgorithmParts<'b> {
    #[doc = "AlgorithmName"]
    AlgorithmName(&'b str),
}
impl<'b> MlExecuteAlgorithmParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Execute Algorithm API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlExecuteAlgorithmParts::AlgorithmName(algorithm_name) => {
                let encoded_algorithm_name: Cow<str> =
                    percent_encode(algorithm_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_algorithm_name.len());
                p.push_str("/_plugins/_ml/_execute/");
                p.push_str(encoded_algorithm_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Execute Algorithm API\n\nExecute an algorithm."]
#[derive(Clone, Debug)]
pub struct MlExecuteAlgorithm<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlExecuteAlgorithmParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlExecuteAlgorithm<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlExecuteAlgorithm] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlExecuteAlgorithmParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlExecuteAlgorithm {
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
    pub fn body<T>(self, body: T) -> MlExecuteAlgorithm<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlExecuteAlgorithm {
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
    #[doc = "Creates an asynchronous call to the Ml Execute Algorithm API that can be awaited"]
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
#[doc = "API parts for the Ml Execute Tool API"]
pub enum MlExecuteToolParts<'b> {
    #[doc = "ToolName"]
    ToolName(&'b str),
}
impl<'b> MlExecuteToolParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Execute Tool API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlExecuteToolParts::ToolName(tool_name) => {
                let encoded_tool_name: Cow<str> =
                    percent_encode(tool_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_tool_name.len());
                p.push_str("/_plugins/_ml/tools/_execute/");
                p.push_str(encoded_tool_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Execute Tool API\n\nExecute a tool."]
#[derive(Clone, Debug)]
pub struct MlExecuteTool<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlExecuteToolParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlExecuteTool<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlExecuteTool] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlExecuteToolParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlExecuteTool {
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
    pub fn body<T>(self, body: T) -> MlExecuteTool<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlExecuteTool {
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
    #[doc = "Creates an asynchronous call to the Ml Execute Tool API that can be awaited"]
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
#[doc = "API parts for the Ml Get Agent API"]
pub enum MlGetAgentParts<'b> {
    #[doc = "AgentId"]
    AgentId(&'b str),
}
impl<'b> MlGetAgentParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Agent API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetAgentParts::AgentId(agent_id) => {
                let encoded_agent_id: Cow<str> =
                    percent_encode(agent_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_agent_id.len());
                p.push_str("/_plugins/_ml/agents/");
                p.push_str(encoded_agent_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Agent API\n\nGet an agent."]
#[derive(Clone, Debug)]
pub struct MlGetAgent<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetAgentParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetAgent<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetAgent] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetAgentParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetAgent {
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
    #[doc = "Creates an asynchronous call to the Ml Get Agent API that can be awaited"]
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
#[doc = "API parts for the Ml Get Agentic Memory API"]
pub enum MlGetAgenticMemoryParts<'b> {
    #[doc = "MemoryContainerId, Type and Id"]
    MemoryContainerIdTypeId(&'b str, &'b str, &'b str),
}
impl<'b> MlGetAgenticMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Agentic Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetAgenticMemoryParts::MemoryContainerIdTypeId(memory_container_id, ty, id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    43usize
                        + encoded_memory_container_id.len()
                        + encoded_ty.len()
                        + encoded_id.len(),
                );
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/");
                p.push_str(encoded_ty.as_ref());
                p.push('/');
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Agentic Memory API\n\nGet a specific memory by its type and ID."]
#[derive(Clone, Debug)]
pub struct MlGetAgenticMemory<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetAgenticMemoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetAgenticMemory<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetAgenticMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetAgenticMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetAgenticMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Get Agentic Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Get All Memories API"]
pub enum MlGetAllMemoriesParts {
    #[doc = "No parts"]
    None,
}
impl MlGetAllMemoriesParts {
    #[doc = "Builds a relative URL path to the Ml Get All Memories API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetAllMemoriesParts::None => "/_plugins/_ml/memory".into(),
        }
    }
}
#[doc = "Builder for the Ml Get All Memories API\n\nGet all memories."]
#[derive(Clone, Debug)]
pub struct MlGetAllMemories<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetAllMemoriesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_results: Option<i64>,
    next_token: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetAllMemories<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetAllMemories]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlGetAllMemories {
            transport,
            parts: MlGetAllMemoriesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            max_results: None,
            next_token: None,
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
    pub fn max_results(mut self, max_results: i64) -> Self {
        self.max_results = Some(max_results);
        self
    }
    pub fn next_token(mut self, next_token: i64) -> Self {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get All Memories API that can be awaited"]
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
                max_results: Option<i64>,
                next_token: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_results: self.max_results,
                next_token: self.next_token,
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
#[doc = "API parts for the Ml Get All Messages API"]
pub enum MlGetAllMessagesParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlGetAllMessagesParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get All Messages API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetAllMessagesParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.push_str("/messages");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get All Messages API\n\nGet all messages in a memory."]
#[derive(Clone, Debug)]
pub struct MlGetAllMessages<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetAllMessagesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_results: Option<i64>,
    next_token: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetAllMessages<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetAllMessages] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetAllMessagesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetAllMessages {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            max_results: None,
            next_token: None,
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
    pub fn max_results(mut self, max_results: i64) -> Self {
        self.max_results = Some(max_results);
        self
    }
    pub fn next_token(mut self, next_token: i64) -> Self {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get All Messages API that can be awaited"]
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
                max_results: Option<i64>,
                next_token: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_results: self.max_results,
                next_token: self.next_token,
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
#[doc = "API parts for the Ml Get All Tools API"]
pub enum MlGetAllToolsParts {
    #[doc = "No parts"]
    None,
}
impl MlGetAllToolsParts {
    #[doc = "Builds a relative URL path to the Ml Get All Tools API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetAllToolsParts::None => "/_plugins/_ml/tools".into(),
        }
    }
}
#[doc = "Builder for the Ml Get All Tools API\n\nGet tools."]
#[derive(Clone, Debug)]
pub struct MlGetAllTools<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetAllToolsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetAllTools<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetAllTools]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlGetAllTools {
            transport,
            parts: MlGetAllToolsParts::None,
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
    #[doc = "Creates an asynchronous call to the Ml Get All Tools API that can be awaited"]
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
#[doc = "API parts for the Ml Get Connector API"]
pub enum MlGetConnectorParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
impl<'b> MlGetConnectorParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Connector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetConnectorParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_connector_id.len());
                p.push_str("/_plugins/_ml/connectors/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Connector API\n\nRetrieves a standalone connector."]
#[derive(Clone, Debug)]
pub struct MlGetConnector<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetConnectorParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetConnector<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetConnector] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetConnectorParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetConnector {
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
    #[doc = "Creates an asynchronous call to the Ml Get Connector API that can be awaited"]
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
#[doc = "API parts for the Ml Get Controller API"]
pub enum MlGetControllerParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlGetControllerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Controller API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetControllerParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/controllers/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Controller API\n\nRetrieves a controller."]
#[derive(Clone, Debug)]
pub struct MlGetController<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetControllerParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetController<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetController] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetControllerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetController {
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
    #[doc = "Creates an asynchronous call to the Ml Get Controller API that can be awaited"]
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
#[doc = "API parts for the Ml Get Memory API"]
pub enum MlGetMemoryParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlGetMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetMemoryParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Memory API\n\nGet a memory."]
#[derive(Clone, Debug)]
pub struct MlGetMemory<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetMemoryParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetMemory<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Get Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Get Memory Container API"]
pub enum MlGetMemoryContainerParts<'b> {
    #[doc = "MemoryContainerId"]
    MemoryContainerId(&'b str),
}
impl<'b> MlGetMemoryContainerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Memory Container API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetMemoryContainerParts::MemoryContainerId(memory_container_id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_memory_container_id.len());
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Memory Container API\n\nGet a memory container."]
#[derive(Clone, Debug)]
pub struct MlGetMemoryContainer<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetMemoryContainerParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetMemoryContainer<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetMemoryContainer] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetMemoryContainerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetMemoryContainer {
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
    #[doc = "Creates an asynchronous call to the Ml Get Memory Container API that can be awaited"]
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
#[doc = "API parts for the Ml Get Message API"]
pub enum MlGetMessageParts<'b> {
    #[doc = "MessageId"]
    MessageId(&'b str),
}
impl<'b> MlGetMessageParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Message API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetMessageParts::MessageId(message_id) => {
                let encoded_message_id: Cow<str> =
                    percent_encode(message_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_message_id.len());
                p.push_str("/_plugins/_ml/memory/message/");
                p.push_str(encoded_message_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Message API\n\nGet a message."]
#[derive(Clone, Debug)]
pub struct MlGetMessage<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetMessageParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetMessage<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetMessage] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetMessageParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetMessage {
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
    #[doc = "Creates an asynchronous call to the Ml Get Message API that can be awaited"]
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
#[doc = "API parts for the Ml Get Message Traces API"]
pub enum MlGetMessageTracesParts<'b> {
    #[doc = "MessageId"]
    MessageId(&'b str),
}
impl<'b> MlGetMessageTracesParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Message Traces API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetMessageTracesParts::MessageId(message_id) => {
                let encoded_message_id: Cow<str> =
                    percent_encode(message_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(36usize + encoded_message_id.len());
                p.push_str("/_plugins/_ml/memory/message/");
                p.push_str(encoded_message_id.as_ref());
                p.push_str("/traces");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Message Traces API\n\nGet a message traces."]
#[derive(Clone, Debug)]
pub struct MlGetMessageTraces<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetMessageTracesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    max_results: Option<i64>,
    next_token: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetMessageTraces<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetMessageTraces] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetMessageTracesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetMessageTraces {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            max_results: None,
            next_token: None,
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
    pub fn max_results(mut self, max_results: i64) -> Self {
        self.max_results = Some(max_results);
        self
    }
    pub fn next_token(mut self, next_token: i64) -> Self {
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
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ml Get Message Traces API that can be awaited"]
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
                max_results: Option<i64>,
                next_token: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                max_results: self.max_results,
                next_token: self.next_token,
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
#[doc = "API parts for the Ml Get Model API"]
pub enum MlGetModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlGetModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Model API\n\nRetrieves a model."]
#[derive(Clone, Debug)]
pub struct MlGetModel<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetModel<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetModel {
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
    #[doc = "Creates an asynchronous call to the Ml Get Model API that can be awaited"]
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
#[doc = "API parts for the Ml Get Model Group API"]
pub enum MlGetModelGroupParts<'b> {
    #[doc = "ModelGroupId"]
    ModelGroupId(&'b str),
}
impl<'b> MlGetModelGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Model Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetModelGroupParts::ModelGroupId(model_group_id) => {
                let encoded_model_group_id: Cow<str> =
                    percent_encode(model_group_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_model_group_id.len());
                p.push_str("/_plugins/_ml/model_groups/");
                p.push_str(encoded_model_group_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Model Group API\n\nRetrieves a model group."]
#[derive(Clone, Debug)]
pub struct MlGetModelGroup<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetModelGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetModelGroup<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetModelGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetModelGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetModelGroup {
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
    #[doc = "Creates an asynchronous call to the Ml Get Model Group API that can be awaited"]
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
#[doc = "API parts for the Ml Get Profile API"]
pub enum MlGetProfileParts {
    #[doc = "No parts"]
    None,
}
impl MlGetProfileParts {
    #[doc = "Builds a relative URL path to the Ml Get Profile API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetProfileParts::None => "/_plugins/_ml/profile".into(),
        }
    }
}
#[doc = "Builder for the Ml Get Profile API\n\nGet a profile."]
#[derive(Clone, Debug)]
pub struct MlGetProfile<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetProfileParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlGetProfile<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetProfile]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlGetProfile {
            transport,
            parts: MlGetProfileParts::None,
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
    pub fn body<T>(self, body: T) -> MlGetProfile<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetProfile {
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
    #[doc = "Creates an asynchronous call to the Ml Get Profile API that can be awaited"]
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Get Profile Models API"]
pub enum MlGetProfileModelsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlGetProfileModelsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Profile Models API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetProfileModelsParts::None => "/_plugins/_ml/profile/models".into(),
            MlGetProfileModelsParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/profile/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Profile Models API\n\nGet a profile models."]
#[derive(Clone, Debug)]
pub struct MlGetProfileModels<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetProfileModelsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlGetProfileModels<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetProfileModels] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetProfileModelsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetProfileModels {
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
    pub fn body<T>(self, body: T) -> MlGetProfileModels<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetProfileModels {
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
    #[doc = "Creates an asynchronous call to the Ml Get Profile Models API that can be awaited"]
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Get Profile Tasks API"]
pub enum MlGetProfileTasksParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> MlGetProfileTasksParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Profile Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetProfileTasksParts::None => "/_plugins/_ml/profile/tasks".into(),
            MlGetProfileTasksParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_task_id.len());
                p.push_str("/_plugins/_ml/profile/tasks/");
                p.push_str(encoded_task_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Profile Tasks API\n\nGet a profile tasks."]
#[derive(Clone, Debug)]
pub struct MlGetProfileTasks<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlGetProfileTasksParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlGetProfileTasks<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlGetProfileTasks] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetProfileTasksParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetProfileTasks {
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
    pub fn body<T>(self, body: T) -> MlGetProfileTasks<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlGetProfileTasks {
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
    #[doc = "Creates an asynchronous call to the Ml Get Profile Tasks API that can be awaited"]
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Get Stats API"]
pub enum MlGetStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Stat"]
    Stat(&'b str),
    #[doc = "NodeId"]
    NodeId(&'b str),
    #[doc = "NodeId and Stat"]
    NodeIdStat(&'b str, &'b str),
}
impl<'b> MlGetStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetStatsParts::None => "/_plugins/_ml/stats".into(),
            MlGetStatsParts::Stat(stat) => {
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_stat.len());
                p.push_str("/_plugins/_ml/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
            MlGetStatsParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_node_id.len());
                p.push_str("/_plugins/_ml/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            MlGetStatsParts::NodeIdStat(node_id, stat) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(21usize + encoded_node_id.len() + encoded_stat.len());
                p.push_str("/_plugins/_ml/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Stats API\n\nGet stats."]
#[derive(Clone, Debug)]
pub struct MlGetStats<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetStats<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetStats {
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
    #[doc = "Creates an asynchronous call to the Ml Get Stats API that can be awaited"]
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
#[doc = "API parts for the Ml Get Task API"]
pub enum MlGetTaskParts<'b> {
    #[doc = "TaskId"]
    TaskId(&'b str),
}
impl<'b> MlGetTaskParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Task API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetTaskParts::TaskId(task_id) => {
                let encoded_task_id: Cow<str> =
                    percent_encode(task_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_task_id.len());
                p.push_str("/_plugins/_ml/tasks/");
                p.push_str(encoded_task_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Task API\n\nRetrieves a task."]
#[derive(Clone, Debug)]
pub struct MlGetTask<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetTaskParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetTask<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetTask] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetTaskParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetTask {
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
    #[doc = "Creates an asynchronous call to the Ml Get Task API that can be awaited"]
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
#[doc = "API parts for the Ml Get Tool API"]
pub enum MlGetToolParts<'b> {
    #[doc = "ToolName"]
    ToolName(&'b str),
}
impl<'b> MlGetToolParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Get Tool API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlGetToolParts::ToolName(tool_name) => {
                let encoded_tool_name: Cow<str> =
                    percent_encode(tool_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(20usize + encoded_tool_name.len());
                p.push_str("/_plugins/_ml/tools/");
                p.push_str(encoded_tool_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Get Tool API\n\nGet tools."]
#[derive(Clone, Debug)]
pub struct MlGetTool<'a, 'b> {
    transport: &'a Transport,
    parts: MlGetToolParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> MlGetTool<'a, 'b> {
    #[doc = "Creates a new instance of [MlGetTool] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlGetToolParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlGetTool {
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
    #[doc = "Creates an asynchronous call to the Ml Get Tool API that can be awaited"]
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
#[deprecated = "Use `deploy_model` instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Load Model API"]
pub enum MlLoadModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
#[allow(deprecated)]
impl<'b> MlLoadModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Load Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlLoadModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_load");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Load Model API\n\nDeploys a model."]
#[deprecated = "Use `deploy_model` instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct MlLoadModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlLoadModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[allow(deprecated)]
impl<'a, 'b, B> MlLoadModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlLoadModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlLoadModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlLoadModel {
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
    pub fn body<T>(self, body: T) -> MlLoadModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlLoadModel {
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
    #[doc = "Creates an asynchronous call to the Ml Load Model API that can be awaited"]
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
#[doc = "API parts for the Ml Predict API"]
pub enum MlPredictParts<'b> {
    #[doc = "AlgorithmName and ModelId"]
    AlgorithmNameModelId(&'b str, &'b str),
}
impl<'b> MlPredictParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Predict API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPredictParts::AlgorithmNameModelId(algorithm_name, model_id) => {
                let encoded_algorithm_name: Cow<str> =
                    percent_encode(algorithm_name.as_bytes(), PARTS_ENCODED).into();
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    24usize + encoded_algorithm_name.len() + encoded_model_id.len(),
                );
                p.push_str("/_plugins/_ml/_predict/");
                p.push_str(encoded_algorithm_name.as_ref());
                p.push('/');
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Predict API\n\nPredicts new data with trained model."]
#[derive(Clone, Debug)]
pub struct MlPredict<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPredictParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPredict<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPredict] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPredictParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPredict {
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
    pub fn body<T>(self, body: T) -> MlPredict<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPredict {
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
    #[doc = "Creates an asynchronous call to the Ml Predict API that can be awaited"]
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
#[doc = "API parts for the Ml Predict Model API"]
pub enum MlPredictModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlPredictModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Predict Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPredictModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_predict");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Predict Model API\n\nPredicts a model."]
#[derive(Clone, Debug)]
pub struct MlPredictModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPredictModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPredictModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPredictModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPredictModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPredictModel {
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
    pub fn body<T>(self, body: T) -> MlPredictModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPredictModel {
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
    #[doc = "Creates an asynchronous call to the Ml Predict Model API that can be awaited"]
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
#[doc = "API parts for the Ml Predict Model Stream API"]
pub enum MlPredictModelStreamParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlPredictModelStreamParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Predict Model Stream API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlPredictModelStreamParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(37usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_predict/stream");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Predict Model Stream API\n\nPredicts a model in streaming mode."]
#[derive(Clone, Debug)]
pub struct MlPredictModelStream<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlPredictModelStreamParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlPredictModelStream<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlPredictModelStream] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlPredictModelStreamParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlPredictModelStream {
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
    pub fn body<T>(self, body: T) -> MlPredictModelStream<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlPredictModelStream {
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
    #[doc = "Creates an asynchronous call to the Ml Predict Model Stream API that can be awaited"]
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
#[doc = "API parts for the Ml Register Agents API"]
pub enum MlRegisterAgentsParts {
    #[doc = "No parts"]
    None,
}
impl MlRegisterAgentsParts {
    #[doc = "Builds a relative URL path to the Ml Register Agents API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRegisterAgentsParts::None => "/_plugins/_ml/agents/_register".into(),
        }
    }
}
#[doc = "Builder for the Ml Register Agents API\n\nRegister an agent."]
#[derive(Clone, Debug)]
pub struct MlRegisterAgents<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlRegisterAgentsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlRegisterAgents<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlRegisterAgents]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlRegisterAgents {
            transport,
            parts: MlRegisterAgentsParts::None,
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
    pub fn body<T>(self, body: T) -> MlRegisterAgents<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRegisterAgents {
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
    #[doc = "Creates an asynchronous call to the Ml Register Agents API that can be awaited"]
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
#[doc = "API parts for the Ml Register Model API"]
pub enum MlRegisterModelParts {
    #[doc = "No parts"]
    None,
}
impl MlRegisterModelParts {
    #[doc = "Builds a relative URL path to the Ml Register Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRegisterModelParts::None => "/_plugins/_ml/models/_register".into(),
        }
    }
}
#[doc = "Builder for the Ml Register Model API\n\nRegisters a model."]
#[derive(Clone, Debug)]
pub struct MlRegisterModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlRegisterModelParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlRegisterModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlRegisterModel]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlRegisterModel {
            transport,
            parts: MlRegisterModelParts::None,
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
    pub fn body<T>(self, body: T) -> MlRegisterModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRegisterModel {
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
    #[doc = "Creates an asynchronous call to the Ml Register Model API that can be awaited"]
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
#[doc = "API parts for the Ml Register Model Group API"]
pub enum MlRegisterModelGroupParts {
    #[doc = "No parts"]
    None,
}
impl MlRegisterModelGroupParts {
    #[doc = "Builds a relative URL path to the Ml Register Model Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRegisterModelGroupParts::None => "/_plugins/_ml/model_groups/_register".into(),
        }
    }
}
#[doc = "Builder for the Ml Register Model Group API\n\nRegisters a model group."]
#[derive(Clone, Debug)]
pub struct MlRegisterModelGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlRegisterModelGroupParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlRegisterModelGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlRegisterModelGroup]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlRegisterModelGroup {
            transport,
            parts: MlRegisterModelGroupParts::None,
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
    pub fn body<T>(self, body: T) -> MlRegisterModelGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRegisterModelGroup {
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
    #[doc = "Creates an asynchronous call to the Ml Register Model Group API that can be awaited"]
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
#[doc = "API parts for the Ml Register Model Meta API"]
pub enum MlRegisterModelMetaParts {
    #[doc = "No parts"]
    None,
}
impl MlRegisterModelMetaParts {
    #[doc = "Builds a relative URL path to the Ml Register Model Meta API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlRegisterModelMetaParts::None => "/_plugins/_ml/models/_register_meta".into(),
        }
    }
}
#[doc = "Builder for the Ml Register Model Meta API\n\nRegisters model metadata."]
#[derive(Clone, Debug)]
pub struct MlRegisterModelMeta<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlRegisterModelMetaParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlRegisterModelMeta<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlRegisterModelMeta]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlRegisterModelMeta {
            transport,
            parts: MlRegisterModelMetaParts::None,
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
    pub fn body<T>(self, body: T) -> MlRegisterModelMeta<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlRegisterModelMeta {
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
    #[doc = "Creates an asynchronous call to the Ml Register Model Meta API that can be awaited"]
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
#[doc = "API parts for the Ml Search Agentic Memory API"]
pub enum MlSearchAgenticMemoryParts<'b> {
    #[doc = "MemoryContainerId and Type"]
    MemoryContainerIdType(&'b str, &'b str),
}
impl<'b> MlSearchAgenticMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Search Agentic Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchAgenticMemoryParts::MemoryContainerIdType(memory_container_id, ty) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    50usize + encoded_memory_container_id.len() + encoded_ty.len(),
                );
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/");
                p.push_str(encoded_ty.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Search Agentic Memory API\n\nSearch for memories of a specific type within a memory container."]
#[derive(Clone, Debug)]
pub struct MlSearchAgenticMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchAgenticMemoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchAgenticMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchAgenticMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlSearchAgenticMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlSearchAgenticMemory {
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
    pub fn body<T>(self, body: T) -> MlSearchAgenticMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchAgenticMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Search Agentic Memory API that can be awaited"]
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
        let body = self.body;
        let response = self
            .transport
            .send(method, &path, headers, query_string.as_ref(), body, timeout)
            .await?;
        Ok(response)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Search Agents API"]
pub enum MlSearchAgentsParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchAgentsParts {
    #[doc = "Builds a relative URL path to the Ml Search Agents API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchAgentsParts::None => "/_plugins/_ml/agents/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Agents API\n\nSearch agents."]
#[derive(Clone, Debug)]
pub struct MlSearchAgents<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchAgentsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchAgents<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchAgents]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchAgents {
            transport,
            parts: MlSearchAgentsParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchAgents<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchAgents {
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
    #[doc = "Creates an asynchronous call to the Ml Search Agents API that can be awaited"]
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
#[doc = "API parts for the Ml Search Connectors API"]
pub enum MlSearchConnectorsParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchConnectorsParts {
    #[doc = "Builds a relative URL path to the Ml Search Connectors API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchConnectorsParts::None => "/_plugins/_ml/connectors/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Connectors API\n\nSearches for standalone connectors."]
#[derive(Clone, Debug)]
pub struct MlSearchConnectors<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchConnectorsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchConnectors<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchConnectors]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchConnectors {
            transport,
            parts: MlSearchConnectorsParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchConnectors<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchConnectors {
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
    #[doc = "Creates an asynchronous call to the Ml Search Connectors API that can be awaited"]
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
#[doc = "API parts for the Ml Search Memory API"]
pub enum MlSearchMemoryParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchMemoryParts {
    #[doc = "Builds a relative URL path to the Ml Search Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchMemoryParts::None => "/_plugins/_ml/memory/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Memory API\n\nSearch memory."]
#[derive(Clone, Debug)]
pub struct MlSearchMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchMemoryParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchMemory]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchMemory {
            transport,
            parts: MlSearchMemoryParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Search Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Search Memory Container API"]
pub enum MlSearchMemoryContainerParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchMemoryContainerParts {
    #[doc = "Builds a relative URL path to the Ml Search Memory Container API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchMemoryContainerParts::None => "/_plugins/_ml/memory_containers/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Memory Container API\n\nSearch memory containers."]
#[derive(Clone, Debug)]
pub struct MlSearchMemoryContainer<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchMemoryContainerParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchMemoryContainer<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchMemoryContainer]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchMemoryContainer {
            transport,
            parts: MlSearchMemoryContainerParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchMemoryContainer<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchMemoryContainer {
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
    #[doc = "Creates an asynchronous call to the Ml Search Memory Container API that can be awaited"]
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
#[doc = "API parts for the Ml Search Message API"]
pub enum MlSearchMessageParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlSearchMessageParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Search Message API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchMessageParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.push_str("/_search");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Search Message API\n\nSearch messages."]
#[derive(Clone, Debug)]
pub struct MlSearchMessage<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchMessageParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchMessage<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchMessage] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlSearchMessageParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlSearchMessage {
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
    pub fn body<T>(self, body: T) -> MlSearchMessage<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchMessage {
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
    #[doc = "Creates an asynchronous call to the Ml Search Message API that can be awaited"]
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
#[doc = "API parts for the Ml Search Model Group API"]
pub enum MlSearchModelGroupParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchModelGroupParts {
    #[doc = "Builds a relative URL path to the Ml Search Model Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchModelGroupParts::None => "/_plugins/_ml/model_groups/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Model Group API\n\nSearches for model groups."]
#[derive(Clone, Debug)]
pub struct MlSearchModelGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchModelGroupParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchModelGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchModelGroup]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchModelGroup {
            transport,
            parts: MlSearchModelGroupParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchModelGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchModelGroup {
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
    #[doc = "Creates an asynchronous call to the Ml Search Model Group API that can be awaited"]
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
#[doc = "API parts for the Ml Search Models API"]
pub enum MlSearchModelsParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchModelsParts {
    #[doc = "Builds a relative URL path to the Ml Search Models API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchModelsParts::None => "/_plugins/_ml/models/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Models API\n\nSearches for models."]
#[derive(Clone, Debug)]
pub struct MlSearchModels<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchModelsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchModels<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchModels]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchModels {
            transport,
            parts: MlSearchModelsParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchModels<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchModels {
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
    #[doc = "Creates an asynchronous call to the Ml Search Models API that can be awaited"]
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
#[doc = "API parts for the Ml Search Tasks API"]
pub enum MlSearchTasksParts {
    #[doc = "No parts"]
    None,
}
impl MlSearchTasksParts {
    #[doc = "Builds a relative URL path to the Ml Search Tasks API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlSearchTasksParts::None => "/_plugins/_ml/tasks/_search".into(),
        }
    }
}
#[doc = "Builder for the Ml Search Tasks API\n\nSearches for tasks."]
#[derive(Clone, Debug)]
pub struct MlSearchTasks<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlSearchTasksParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlSearchTasks<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlSearchTasks]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlSearchTasks {
            transport,
            parts: MlSearchTasksParts::None,
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
    pub fn body<T>(self, body: T) -> MlSearchTasks<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlSearchTasks {
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
    #[doc = "Creates an asynchronous call to the Ml Search Tasks API that can be awaited"]
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
#[doc = "API parts for the Ml Train API"]
pub enum MlTrainParts<'b> {
    #[doc = "AlgorithmName"]
    AlgorithmName(&'b str),
}
impl<'b> MlTrainParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Train API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlTrainParts::AlgorithmName(algorithm_name) => {
                let encoded_algorithm_name: Cow<str> =
                    percent_encode(algorithm_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_algorithm_name.len());
                p.push_str("/_plugins/_ml/_train/");
                p.push_str(encoded_algorithm_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Train API\n\nTrains a model synchronously."]
#[derive(Clone, Debug)]
pub struct MlTrain<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlTrainParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlTrain<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlTrain] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlTrainParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlTrain {
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
    pub fn body<T>(self, body: T) -> MlTrain<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlTrain {
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
    #[doc = "Creates an asynchronous call to the Ml Train API that can be awaited"]
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
#[doc = "API parts for the Ml Train Predict API"]
pub enum MlTrainPredictParts<'b> {
    #[doc = "AlgorithmName"]
    AlgorithmName(&'b str),
}
impl<'b> MlTrainPredictParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Train Predict API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlTrainPredictParts::AlgorithmName(algorithm_name) => {
                let encoded_algorithm_name: Cow<str> =
                    percent_encode(algorithm_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_algorithm_name.len());
                p.push_str("/_plugins/_ml/_train_predict/");
                p.push_str(encoded_algorithm_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Train Predict API\n\nTrains a model and predicts against the same training dataset."]
#[derive(Clone, Debug)]
pub struct MlTrainPredict<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlTrainPredictParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlTrainPredict<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlTrainPredict] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlTrainPredictParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlTrainPredict {
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
    pub fn body<T>(self, body: T) -> MlTrainPredict<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlTrainPredict {
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
    #[doc = "Creates an asynchronous call to the Ml Train Predict API that can be awaited"]
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
#[doc = "API parts for the Ml Undeploy Model API"]
pub enum MlUndeployModelParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlUndeployModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Undeploy Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUndeployModelParts::None => "/_plugins/_ml/models/_undeploy".into(),
            MlUndeployModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_undeploy");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Undeploy Model API\n\nUndeploys a model."]
#[derive(Clone, Debug)]
pub struct MlUndeployModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUndeployModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUndeployModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUndeployModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUndeployModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUndeployModel {
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
    pub fn body<T>(self, body: T) -> MlUndeployModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUndeployModel {
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
    #[doc = "Creates an asynchronous call to the Ml Undeploy Model API that can be awaited"]
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
#[deprecated = "Use `undeploy_model` instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Unload Model API"]
pub enum MlUnloadModelParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ModelId"]
    ModelId(&'b str),
}
#[allow(deprecated)]
impl<'b> MlUnloadModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Unload Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUnloadModelParts::None => "/_plugins/_ml/models/_unload".into(),
            MlUnloadModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_unload");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Unload Model API\n\nUnloads a model."]
#[deprecated = "Use `undeploy_model` instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct MlUnloadModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUnloadModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[allow(deprecated)]
impl<'a, 'b, B> MlUnloadModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUnloadModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUnloadModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUnloadModel {
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
    pub fn body<T>(self, body: T) -> MlUnloadModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUnloadModel {
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
    #[doc = "Creates an asynchronous call to the Ml Unload Model API that can be awaited"]
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
#[doc = "API parts for the Ml Update Agentic Memory API"]
pub enum MlUpdateAgenticMemoryParts<'b> {
    #[doc = "MemoryContainerId, Type and Id"]
    MemoryContainerIdTypeId(&'b str, &'b str, &'b str),
}
impl<'b> MlUpdateAgenticMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Agentic Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateAgenticMemoryParts::MemoryContainerIdTypeId(memory_container_id, ty, id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_ty: Cow<str> = percent_encode(ty.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    43usize
                        + encoded_memory_container_id.len()
                        + encoded_ty.len()
                        + encoded_id.len(),
                );
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.push_str("/memories/");
                p.push_str(encoded_ty.as_ref());
                p.push('/');
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Agentic Memory API\n\nUpdate a specific memory by its type and ID."]
#[derive(Clone, Debug)]
pub struct MlUpdateAgenticMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateAgenticMemoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateAgenticMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateAgenticMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateAgenticMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateAgenticMemory {
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
    pub fn body<T>(self, body: T) -> MlUpdateAgenticMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateAgenticMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Update Agentic Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Update Connector API"]
pub enum MlUpdateConnectorParts<'b> {
    #[doc = "ConnectorId"]
    ConnectorId(&'b str),
}
impl<'b> MlUpdateConnectorParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Connector API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateConnectorParts::ConnectorId(connector_id) => {
                let encoded_connector_id: Cow<str> =
                    percent_encode(connector_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_connector_id.len());
                p.push_str("/_plugins/_ml/connectors/");
                p.push_str(encoded_connector_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Connector API\n\nUpdates a standalone connector."]
#[derive(Clone, Debug)]
pub struct MlUpdateConnector<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateConnectorParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateConnector<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateConnector] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateConnectorParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateConnector {
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
    pub fn body<T>(self, body: T) -> MlUpdateConnector<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateConnector {
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
    #[doc = "Creates an asynchronous call to the Ml Update Connector API that can be awaited"]
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
#[doc = "API parts for the Ml Update Controller API"]
pub enum MlUpdateControllerParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlUpdateControllerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Controller API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateControllerParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/controllers/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Controller API\n\nUpdates a controller."]
#[derive(Clone, Debug)]
pub struct MlUpdateController<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateControllerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateController<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateController] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateControllerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateController {
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
    pub fn body<T>(self, body: T) -> MlUpdateController<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateController {
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
    #[doc = "Creates an asynchronous call to the Ml Update Controller API that can be awaited"]
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
#[doc = "API parts for the Ml Update Memory API"]
pub enum MlUpdateMemoryParts<'b> {
    #[doc = "MemoryId"]
    MemoryId(&'b str),
}
impl<'b> MlUpdateMemoryParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Memory API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateMemoryParts::MemoryId(memory_id) => {
                let encoded_memory_id: Cow<str> =
                    percent_encode(memory_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_memory_id.len());
                p.push_str("/_plugins/_ml/memory/");
                p.push_str(encoded_memory_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Memory API\n\nUpdate a memory."]
#[derive(Clone, Debug)]
pub struct MlUpdateMemory<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateMemoryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateMemory<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateMemory] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateMemoryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateMemory {
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
    pub fn body<T>(self, body: T) -> MlUpdateMemory<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateMemory {
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
    #[doc = "Creates an asynchronous call to the Ml Update Memory API that can be awaited"]
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
#[doc = "API parts for the Ml Update Memory Container API"]
pub enum MlUpdateMemoryContainerParts<'b> {
    #[doc = "MemoryContainerId"]
    MemoryContainerId(&'b str),
}
impl<'b> MlUpdateMemoryContainerParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Memory Container API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateMemoryContainerParts::MemoryContainerId(memory_container_id) => {
                let encoded_memory_container_id: Cow<str> =
                    percent_encode(memory_container_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_memory_container_id.len());
                p.push_str("/_plugins/_ml/memory_containers/");
                p.push_str(encoded_memory_container_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Memory Container API\n\nUpdate a memory container."]
#[derive(Clone, Debug)]
pub struct MlUpdateMemoryContainer<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateMemoryContainerParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateMemoryContainer<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateMemoryContainer] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateMemoryContainerParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateMemoryContainer {
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
    pub fn body<T>(self, body: T) -> MlUpdateMemoryContainer<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateMemoryContainer {
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
    #[doc = "Creates an asynchronous call to the Ml Update Memory Container API that can be awaited"]
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
#[doc = "API parts for the Ml Update Message API"]
pub enum MlUpdateMessageParts<'b> {
    #[doc = "MessageId"]
    MessageId(&'b str),
}
impl<'b> MlUpdateMessageParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Message API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateMessageParts::MessageId(message_id) => {
                let encoded_message_id: Cow<str> =
                    percent_encode(message_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_message_id.len());
                p.push_str("/_plugins/_ml/memory/message/");
                p.push_str(encoded_message_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Message API\n\nUpdate a message."]
#[derive(Clone, Debug)]
pub struct MlUpdateMessage<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateMessageParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateMessage<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateMessage] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateMessageParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateMessage {
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
    pub fn body<T>(self, body: T) -> MlUpdateMessage<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateMessage {
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
    #[doc = "Creates an asynchronous call to the Ml Update Message API that can be awaited"]
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
#[doc = "API parts for the Ml Update Model API"]
pub enum MlUpdateModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> MlUpdateModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_model_id.len());
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Model API\n\nUpdates a model."]
#[derive(Clone, Debug)]
pub struct MlUpdateModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateModel {
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
    pub fn body<T>(self, body: T) -> MlUpdateModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateModel {
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
    #[doc = "Creates an asynchronous call to the Ml Update Model API that can be awaited"]
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
#[doc = "API parts for the Ml Update Model Group API"]
pub enum MlUpdateModelGroupParts<'b> {
    #[doc = "ModelGroupId"]
    ModelGroupId(&'b str),
}
impl<'b> MlUpdateModelGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Update Model Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUpdateModelGroupParts::ModelGroupId(model_group_id) => {
                let encoded_model_group_id: Cow<str> =
                    percent_encode(model_group_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_model_group_id.len());
                p.push_str("/_plugins/_ml/model_groups/");
                p.push_str(encoded_model_group_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Update Model Group API\n\nUpdates a model group."]
#[derive(Clone, Debug)]
pub struct MlUpdateModelGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUpdateModelGroupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUpdateModelGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUpdateModelGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUpdateModelGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUpdateModelGroup {
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
    pub fn body<T>(self, body: T) -> MlUpdateModelGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUpdateModelGroup {
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
    #[doc = "Creates an asynchronous call to the Ml Update Model Group API that can be awaited"]
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
#[doc = "API parts for the Ml Upload Chunk API"]
pub enum MlUploadChunkParts<'b> {
    #[doc = "ModelId and ChunkNumber"]
    ModelIdChunkNumber(&'b str, i64),
}
impl<'b> MlUploadChunkParts<'b> {
    #[doc = "Builds a relative URL path to the Ml Upload Chunk API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUploadChunkParts::ModelIdChunkNumber(model_id, chunk_number) => {
                let chunk_number_str = chunk_number.to_string();
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_chunk_number: Cow<str> =
                    percent_encode(chunk_number_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    35usize + encoded_model_id.len() + encoded_chunk_number.len(),
                );
                p.push_str("/_plugins/_ml/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/upload_chunk/");
                p.push_str(encoded_chunk_number.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ml Upload Chunk API\n\nUploads model chunk."]
#[derive(Clone, Debug)]
pub struct MlUploadChunk<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUploadChunkParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> MlUploadChunk<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUploadChunk] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: MlUploadChunkParts<'b>) -> Self {
        let headers = HeaderMap::new();
        MlUploadChunk {
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
    pub fn body<T>(self, body: T) -> MlUploadChunk<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUploadChunk {
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
    #[doc = "Creates an asynchronous call to the Ml Upload Chunk API that can be awaited"]
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
#[deprecated = "Use `register_model` instead."]
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ml Upload Model API"]
pub enum MlUploadModelParts {
    #[doc = "No parts"]
    None,
}
#[allow(deprecated)]
impl MlUploadModelParts {
    #[doc = "Builds a relative URL path to the Ml Upload Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            MlUploadModelParts::None => "/_plugins/_ml/models/_upload".into(),
        }
    }
}
#[doc = "Builder for the Ml Upload Model API\n\nRegisters a model."]
#[deprecated = "Use `register_model` instead."]
#[allow(deprecated)]
#[derive(Clone, Debug)]
pub struct MlUploadModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: MlUploadModelParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
#[allow(deprecated)]
impl<'a, 'b, B> MlUploadModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [MlUploadModel]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        MlUploadModel {
            transport,
            parts: MlUploadModelParts::None,
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
    pub fn body<T>(self, body: T) -> MlUploadModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        MlUploadModel {
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
    #[doc = "Creates an asynchronous call to the Ml Upload Model API that can be awaited"]
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
#[doc = "Namespace client for Machine Learning APIs"]
pub struct Ml<'a> {
    transport: &'a Transport,
}
impl<'a> Ml<'a> {
    #[doc = "Creates a new instance of [Ml]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Ml Add Agentic Memory API\n\nAdd agentic memory to a memory container."]
    pub fn add_agentic_memory<'b>(
        &'a self,
        parts: MlAddAgenticMemoryParts<'b>,
    ) -> MlAddAgenticMemory<'a, 'b, ()> {
        MlAddAgenticMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Chunk Model API\n\nUploads model chunk."]
    #[deprecated = "Use `upload_chunk` instead."]
    #[allow(deprecated)]
    pub fn chunk_model<'b>(&'a self, parts: MlChunkModelParts<'b>) -> MlChunkModel<'a, 'b, ()> {
        MlChunkModel::new(self.transport(), parts)
    }
    #[doc = "Ml Create Connector API\n\nCreates a standalone connector."]
    pub fn create_connector<'b>(&'a self) -> MlCreateConnector<'a, 'b, ()> {
        MlCreateConnector::new(self.transport())
    }
    #[doc = "Ml Create Controller API\n\nCreates a controller."]
    pub fn create_controller<'b>(
        &'a self,
        parts: MlCreateControllerParts<'b>,
    ) -> MlCreateController<'a, 'b, ()> {
        MlCreateController::new(self.transport(), parts)
    }
    #[doc = "Ml Create Memory API\n\nCreate a memory."]
    pub fn create_memory<'b>(&'a self) -> MlCreateMemory<'a, 'b, ()> {
        MlCreateMemory::new(self.transport())
    }
    #[doc = "Ml Create Memory Container API\n\nCreate a memory container."]
    pub fn create_memory_container<'b>(&'a self) -> MlCreateMemoryContainer<'a, 'b, ()> {
        MlCreateMemoryContainer::new(self.transport())
    }
    #[doc = "Ml Create Memory Container Session API\n\nCreate session in a memory container."]
    pub fn create_memory_container_session<'b>(
        &'a self,
        parts: MlCreateMemoryContainerSessionParts<'b>,
    ) -> MlCreateMemoryContainerSession<'a, 'b, ()> {
        MlCreateMemoryContainerSession::new(self.transport(), parts)
    }
    #[doc = "Ml Create Message API\n\nCreate a message."]
    pub fn create_message<'b>(
        &'a self,
        parts: MlCreateMessageParts<'b>,
    ) -> MlCreateMessage<'a, 'b, ()> {
        MlCreateMessage::new(self.transport(), parts)
    }
    #[doc = "Ml Create Model Meta API\n\nRegisters model metadata."]
    #[deprecated = "Use `_register_meta` instead."]
    #[allow(deprecated)]
    pub fn create_model_meta<'b>(&'a self) -> MlCreateModelMeta<'a, 'b, ()> {
        MlCreateModelMeta::new(self.transport())
    }
    #[doc = "Ml Delete Agent API\n\nDelete an agent."]
    pub fn delete_agent<'b>(&'a self, parts: MlDeleteAgentParts<'b>) -> MlDeleteAgent<'a, 'b> {
        MlDeleteAgent::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Agentic Memory API\n\nDelete a specific memory by its type and ID."]
    pub fn delete_agentic_memory<'b>(
        &'a self,
        parts: MlDeleteAgenticMemoryParts<'b>,
    ) -> MlDeleteAgenticMemory<'a, 'b> {
        MlDeleteAgenticMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Agentic Memory Query API\n\nDelete multiple memories using a query to match specific criteria."]
    pub fn delete_agentic_memory_query<'b>(
        &'a self,
        parts: MlDeleteAgenticMemoryQueryParts<'b>,
    ) -> MlDeleteAgenticMemoryQuery<'a, 'b, ()> {
        MlDeleteAgenticMemoryQuery::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Connector API\n\nDeletes a standalone connector."]
    pub fn delete_connector<'b>(
        &'a self,
        parts: MlDeleteConnectorParts<'b>,
    ) -> MlDeleteConnector<'a, 'b> {
        MlDeleteConnector::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Controller API\n\nDeletes a controller."]
    pub fn delete_controller<'b>(
        &'a self,
        parts: MlDeleteControllerParts<'b>,
    ) -> MlDeleteController<'a, 'b> {
        MlDeleteController::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Memory API\n\nDelete a memory."]
    pub fn delete_memory<'b>(&'a self, parts: MlDeleteMemoryParts<'b>) -> MlDeleteMemory<'a, 'b> {
        MlDeleteMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Memory Container API\n\nDelete a memory container."]
    pub fn delete_memory_container<'b>(
        &'a self,
        parts: MlDeleteMemoryContainerParts<'b>,
    ) -> MlDeleteMemoryContainer<'a, 'b> {
        MlDeleteMemoryContainer::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Model API\n\nDeletes a model."]
    pub fn delete_model<'b>(&'a self, parts: MlDeleteModelParts<'b>) -> MlDeleteModel<'a, 'b> {
        MlDeleteModel::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Model Group API\n\nDeletes a model group."]
    pub fn delete_model_group<'b>(
        &'a self,
        parts: MlDeleteModelGroupParts<'b>,
    ) -> MlDeleteModelGroup<'a, 'b> {
        MlDeleteModelGroup::new(self.transport(), parts)
    }
    #[doc = "Ml Delete Task API\n\nDeletes a task."]
    pub fn delete_task<'b>(&'a self, parts: MlDeleteTaskParts<'b>) -> MlDeleteTask<'a, 'b> {
        MlDeleteTask::new(self.transport(), parts)
    }
    #[doc = "Ml Deploy Model API\n\nDeploys a model."]
    pub fn deploy_model<'b>(&'a self, parts: MlDeployModelParts<'b>) -> MlDeployModel<'a, 'b, ()> {
        MlDeployModel::new(self.transport(), parts)
    }
    #[doc = "Ml Execute Agent API\n\nExecute an agent."]
    pub fn execute_agent<'b>(
        &'a self,
        parts: MlExecuteAgentParts<'b>,
    ) -> MlExecuteAgent<'a, 'b, ()> {
        MlExecuteAgent::new(self.transport(), parts)
    }
    #[doc = "Ml Execute Agent Stream API\n\nExecute an agent in streaming mode."]
    pub fn execute_agent_stream<'b>(
        &'a self,
        parts: MlExecuteAgentStreamParts<'b>,
    ) -> MlExecuteAgentStream<'a, 'b, ()> {
        MlExecuteAgentStream::new(self.transport(), parts)
    }
    #[doc = "Ml Execute Algorithm API\n\nExecute an algorithm."]
    pub fn execute_algorithm<'b>(
        &'a self,
        parts: MlExecuteAlgorithmParts<'b>,
    ) -> MlExecuteAlgorithm<'a, 'b, ()> {
        MlExecuteAlgorithm::new(self.transport(), parts)
    }
    #[doc = "Ml Execute Tool API\n\nExecute a tool."]
    pub fn execute_tool<'b>(&'a self, parts: MlExecuteToolParts<'b>) -> MlExecuteTool<'a, 'b, ()> {
        MlExecuteTool::new(self.transport(), parts)
    }
    #[doc = "Ml Get Agent API\n\nGet an agent."]
    pub fn get_agent<'b>(&'a self, parts: MlGetAgentParts<'b>) -> MlGetAgent<'a, 'b> {
        MlGetAgent::new(self.transport(), parts)
    }
    #[doc = "Ml Get Agentic Memory API\n\nGet a specific memory by its type and ID."]
    pub fn get_agentic_memory<'b>(
        &'a self,
        parts: MlGetAgenticMemoryParts<'b>,
    ) -> MlGetAgenticMemory<'a, 'b> {
        MlGetAgenticMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Get All Memories API\n\nGet all memories."]
    pub fn get_all_memories<'b>(&'a self) -> MlGetAllMemories<'a, 'b> {
        MlGetAllMemories::new(self.transport())
    }
    #[doc = "Ml Get All Messages API\n\nGet all messages in a memory."]
    pub fn get_all_messages<'b>(
        &'a self,
        parts: MlGetAllMessagesParts<'b>,
    ) -> MlGetAllMessages<'a, 'b> {
        MlGetAllMessages::new(self.transport(), parts)
    }
    #[doc = "Ml Get All Tools API\n\nGet tools."]
    pub fn get_all_tools<'b>(&'a self) -> MlGetAllTools<'a, 'b> {
        MlGetAllTools::new(self.transport())
    }
    #[doc = "Ml Get Connector API\n\nRetrieves a standalone connector."]
    pub fn get_connector<'b>(&'a self, parts: MlGetConnectorParts<'b>) -> MlGetConnector<'a, 'b> {
        MlGetConnector::new(self.transport(), parts)
    }
    #[doc = "Ml Get Controller API\n\nRetrieves a controller."]
    pub fn get_controller<'b>(
        &'a self,
        parts: MlGetControllerParts<'b>,
    ) -> MlGetController<'a, 'b> {
        MlGetController::new(self.transport(), parts)
    }
    #[doc = "Ml Get Memory API\n\nGet a memory."]
    pub fn get_memory<'b>(&'a self, parts: MlGetMemoryParts<'b>) -> MlGetMemory<'a, 'b> {
        MlGetMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Get Memory Container API\n\nGet a memory container."]
    pub fn get_memory_container<'b>(
        &'a self,
        parts: MlGetMemoryContainerParts<'b>,
    ) -> MlGetMemoryContainer<'a, 'b> {
        MlGetMemoryContainer::new(self.transport(), parts)
    }
    #[doc = "Ml Get Message API\n\nGet a message."]
    pub fn get_message<'b>(&'a self, parts: MlGetMessageParts<'b>) -> MlGetMessage<'a, 'b> {
        MlGetMessage::new(self.transport(), parts)
    }
    #[doc = "Ml Get Message Traces API\n\nGet a message traces."]
    pub fn get_message_traces<'b>(
        &'a self,
        parts: MlGetMessageTracesParts<'b>,
    ) -> MlGetMessageTraces<'a, 'b> {
        MlGetMessageTraces::new(self.transport(), parts)
    }
    #[doc = "Ml Get Model API\n\nRetrieves a model."]
    pub fn get_model<'b>(&'a self, parts: MlGetModelParts<'b>) -> MlGetModel<'a, 'b> {
        MlGetModel::new(self.transport(), parts)
    }
    #[doc = "Ml Get Model Group API\n\nRetrieves a model group."]
    pub fn get_model_group<'b>(
        &'a self,
        parts: MlGetModelGroupParts<'b>,
    ) -> MlGetModelGroup<'a, 'b> {
        MlGetModelGroup::new(self.transport(), parts)
    }
    #[doc = "Ml Get Profile API\n\nGet a profile."]
    pub fn get_profile<'b>(&'a self) -> MlGetProfile<'a, 'b, ()> {
        MlGetProfile::new(self.transport())
    }
    #[doc = "Ml Get Profile Models API\n\nGet a profile models."]
    pub fn get_profile_models<'b>(
        &'a self,
        parts: MlGetProfileModelsParts<'b>,
    ) -> MlGetProfileModels<'a, 'b, ()> {
        MlGetProfileModels::new(self.transport(), parts)
    }
    #[doc = "Ml Get Profile Tasks API\n\nGet a profile tasks."]
    pub fn get_profile_tasks<'b>(
        &'a self,
        parts: MlGetProfileTasksParts<'b>,
    ) -> MlGetProfileTasks<'a, 'b, ()> {
        MlGetProfileTasks::new(self.transport(), parts)
    }
    #[doc = "Ml Get Stats API\n\nGet stats."]
    pub fn get_stats<'b>(&'a self, parts: MlGetStatsParts<'b>) -> MlGetStats<'a, 'b> {
        MlGetStats::new(self.transport(), parts)
    }
    #[doc = "Ml Get Task API\n\nRetrieves a task."]
    pub fn get_task<'b>(&'a self, parts: MlGetTaskParts<'b>) -> MlGetTask<'a, 'b> {
        MlGetTask::new(self.transport(), parts)
    }
    #[doc = "Ml Get Tool API\n\nGet tools."]
    pub fn get_tool<'b>(&'a self, parts: MlGetToolParts<'b>) -> MlGetTool<'a, 'b> {
        MlGetTool::new(self.transport(), parts)
    }
    #[doc = "Ml Load Model API\n\nDeploys a model."]
    #[deprecated = "Use `deploy_model` instead."]
    #[allow(deprecated)]
    pub fn load_model<'b>(&'a self, parts: MlLoadModelParts<'b>) -> MlLoadModel<'a, 'b, ()> {
        MlLoadModel::new(self.transport(), parts)
    }
    #[doc = "Ml Predict API\n\nPredicts new data with trained model."]
    pub fn predict<'b>(&'a self, parts: MlPredictParts<'b>) -> MlPredict<'a, 'b, ()> {
        MlPredict::new(self.transport(), parts)
    }
    #[doc = "Ml Predict Model API\n\nPredicts a model."]
    pub fn predict_model<'b>(
        &'a self,
        parts: MlPredictModelParts<'b>,
    ) -> MlPredictModel<'a, 'b, ()> {
        MlPredictModel::new(self.transport(), parts)
    }
    #[doc = "Ml Predict Model Stream API\n\nPredicts a model in streaming mode."]
    pub fn predict_model_stream<'b>(
        &'a self,
        parts: MlPredictModelStreamParts<'b>,
    ) -> MlPredictModelStream<'a, 'b, ()> {
        MlPredictModelStream::new(self.transport(), parts)
    }
    #[doc = "Ml Register Agents API\n\nRegister an agent."]
    pub fn register_agents<'b>(&'a self) -> MlRegisterAgents<'a, 'b, ()> {
        MlRegisterAgents::new(self.transport())
    }
    #[doc = "Ml Register Model API\n\nRegisters a model."]
    pub fn register_model<'b>(&'a self) -> MlRegisterModel<'a, 'b, ()> {
        MlRegisterModel::new(self.transport())
    }
    #[doc = "Ml Register Model Group API\n\nRegisters a model group."]
    pub fn register_model_group<'b>(&'a self) -> MlRegisterModelGroup<'a, 'b, ()> {
        MlRegisterModelGroup::new(self.transport())
    }
    #[doc = "Ml Register Model Meta API\n\nRegisters model metadata."]
    pub fn register_model_meta<'b>(&'a self) -> MlRegisterModelMeta<'a, 'b, ()> {
        MlRegisterModelMeta::new(self.transport())
    }
    #[doc = "Ml Search Agentic Memory API\n\nSearch for memories of a specific type within a memory container."]
    pub fn search_agentic_memory<'b>(
        &'a self,
        parts: MlSearchAgenticMemoryParts<'b>,
    ) -> MlSearchAgenticMemory<'a, 'b, ()> {
        MlSearchAgenticMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Search Agents API\n\nSearch agents."]
    pub fn search_agents<'b>(&'a self) -> MlSearchAgents<'a, 'b, ()> {
        MlSearchAgents::new(self.transport())
    }
    #[doc = "Ml Search Connectors API\n\nSearches for standalone connectors."]
    pub fn search_connectors<'b>(&'a self) -> MlSearchConnectors<'a, 'b, ()> {
        MlSearchConnectors::new(self.transport())
    }
    #[doc = "Ml Search Memory API\n\nSearch memory."]
    pub fn search_memory<'b>(&'a self) -> MlSearchMemory<'a, 'b, ()> {
        MlSearchMemory::new(self.transport())
    }
    #[doc = "Ml Search Memory Container API\n\nSearch memory containers."]
    pub fn search_memory_container<'b>(&'a self) -> MlSearchMemoryContainer<'a, 'b, ()> {
        MlSearchMemoryContainer::new(self.transport())
    }
    #[doc = "Ml Search Message API\n\nSearch messages."]
    pub fn search_message<'b>(
        &'a self,
        parts: MlSearchMessageParts<'b>,
    ) -> MlSearchMessage<'a, 'b, ()> {
        MlSearchMessage::new(self.transport(), parts)
    }
    #[doc = "Ml Search Model Group API\n\nSearches for model groups."]
    pub fn search_model_group<'b>(&'a self) -> MlSearchModelGroup<'a, 'b, ()> {
        MlSearchModelGroup::new(self.transport())
    }
    #[doc = "Ml Search Models API\n\nSearches for models."]
    pub fn search_models<'b>(&'a self) -> MlSearchModels<'a, 'b, ()> {
        MlSearchModels::new(self.transport())
    }
    #[doc = "Ml Search Tasks API\n\nSearches for tasks."]
    pub fn search_tasks<'b>(&'a self) -> MlSearchTasks<'a, 'b, ()> {
        MlSearchTasks::new(self.transport())
    }
    #[doc = "Ml Train API\n\nTrains a model synchronously."]
    pub fn train<'b>(&'a self, parts: MlTrainParts<'b>) -> MlTrain<'a, 'b, ()> {
        MlTrain::new(self.transport(), parts)
    }
    #[doc = "Ml Train Predict API\n\nTrains a model and predicts against the same training dataset."]
    pub fn train_predict<'b>(
        &'a self,
        parts: MlTrainPredictParts<'b>,
    ) -> MlTrainPredict<'a, 'b, ()> {
        MlTrainPredict::new(self.transport(), parts)
    }
    #[doc = "Ml Undeploy Model API\n\nUndeploys a model."]
    pub fn undeploy_model<'b>(
        &'a self,
        parts: MlUndeployModelParts<'b>,
    ) -> MlUndeployModel<'a, 'b, ()> {
        MlUndeployModel::new(self.transport(), parts)
    }
    #[doc = "Ml Unload Model API\n\nUnloads a model."]
    #[deprecated = "Use `undeploy_model` instead."]
    #[allow(deprecated)]
    pub fn unload_model<'b>(&'a self, parts: MlUnloadModelParts<'b>) -> MlUnloadModel<'a, 'b, ()> {
        MlUnloadModel::new(self.transport(), parts)
    }
    #[doc = "Ml Update Agentic Memory API\n\nUpdate a specific memory by its type and ID."]
    pub fn update_agentic_memory<'b>(
        &'a self,
        parts: MlUpdateAgenticMemoryParts<'b>,
    ) -> MlUpdateAgenticMemory<'a, 'b, ()> {
        MlUpdateAgenticMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Update Connector API\n\nUpdates a standalone connector."]
    pub fn update_connector<'b>(
        &'a self,
        parts: MlUpdateConnectorParts<'b>,
    ) -> MlUpdateConnector<'a, 'b, ()> {
        MlUpdateConnector::new(self.transport(), parts)
    }
    #[doc = "Ml Update Controller API\n\nUpdates a controller."]
    pub fn update_controller<'b>(
        &'a self,
        parts: MlUpdateControllerParts<'b>,
    ) -> MlUpdateController<'a, 'b, ()> {
        MlUpdateController::new(self.transport(), parts)
    }
    #[doc = "Ml Update Memory API\n\nUpdate a memory."]
    pub fn update_memory<'b>(
        &'a self,
        parts: MlUpdateMemoryParts<'b>,
    ) -> MlUpdateMemory<'a, 'b, ()> {
        MlUpdateMemory::new(self.transport(), parts)
    }
    #[doc = "Ml Update Memory Container API\n\nUpdate a memory container."]
    pub fn update_memory_container<'b>(
        &'a self,
        parts: MlUpdateMemoryContainerParts<'b>,
    ) -> MlUpdateMemoryContainer<'a, 'b, ()> {
        MlUpdateMemoryContainer::new(self.transport(), parts)
    }
    #[doc = "Ml Update Message API\n\nUpdate a message."]
    pub fn update_message<'b>(
        &'a self,
        parts: MlUpdateMessageParts<'b>,
    ) -> MlUpdateMessage<'a, 'b, ()> {
        MlUpdateMessage::new(self.transport(), parts)
    }
    #[doc = "Ml Update Model API\n\nUpdates a model."]
    pub fn update_model<'b>(&'a self, parts: MlUpdateModelParts<'b>) -> MlUpdateModel<'a, 'b, ()> {
        MlUpdateModel::new(self.transport(), parts)
    }
    #[doc = "Ml Update Model Group API\n\nUpdates a model group."]
    pub fn update_model_group<'b>(
        &'a self,
        parts: MlUpdateModelGroupParts<'b>,
    ) -> MlUpdateModelGroup<'a, 'b, ()> {
        MlUpdateModelGroup::new(self.transport(), parts)
    }
    #[doc = "Ml Upload Chunk API\n\nUploads model chunk."]
    pub fn upload_chunk<'b>(&'a self, parts: MlUploadChunkParts<'b>) -> MlUploadChunk<'a, 'b, ()> {
        MlUploadChunk::new(self.transport(), parts)
    }
    #[doc = "Ml Upload Model API\n\nRegisters a model."]
    #[deprecated = "Use `register_model` instead."]
    #[allow(deprecated)]
    pub fn upload_model<'b>(&'a self) -> MlUploadModel<'a, 'b, ()> {
        MlUploadModel::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Machine Learning APIs"]
    pub fn ml(&self) -> Ml {
        Ml::new(self.transport())
    }
}
