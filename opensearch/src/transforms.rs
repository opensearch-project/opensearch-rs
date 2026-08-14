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
#[doc = "API parts for the Transforms Delete API"]
pub enum TransformsDeleteParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsDeleteParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Delete API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#delete-a-transform-job)\n\nDelete an index transform."]
#[derive(Clone, Debug)]
pub struct TransformsDelete<'a, 'b> {
    transport: &'a Transport,
    parts: TransformsDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformsDelete<'a, 'b> {
    #[doc = "Creates a new instance of [TransformsDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsDelete {
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
    #[doc = "Creates an asynchronous call to the Transforms Delete API that can be awaited"]
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
#[doc = "API parts for the Transforms Explain API"]
pub enum TransformsExplainParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsExplainParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Explain API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsExplainParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_explain");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Explain API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-the-status-of-a-transform-job)\n\nReturns the status and metadata of a transform job."]
#[derive(Clone, Debug)]
pub struct TransformsExplain<'a, 'b> {
    transport: &'a Transport,
    parts: TransformsExplainParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformsExplain<'a, 'b> {
    #[doc = "Creates a new instance of [TransformsExplain] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsExplainParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsExplain {
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
    #[doc = "Creates an asynchronous call to the Transforms Explain API that can be awaited"]
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
#[doc = "API parts for the Transforms Get API"]
pub enum TransformsGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsGetParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsGetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Get API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-a-transform-jobs-details)\n\nReturns the status and metadata of a transform job."]
#[derive(Clone, Debug)]
pub struct TransformsGet<'a, 'b> {
    transport: &'a Transport,
    parts: TransformsGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformsGet<'a, 'b> {
    #[doc = "Creates a new instance of [TransformsGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsGet {
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
    #[doc = "Creates an asynchronous call to the Transforms Get API that can be awaited"]
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
#[doc = "API parts for the Transforms Preview API"]
pub enum TransformsPreviewParts {
    #[doc = "No parts"]
    None,
}
impl TransformsPreviewParts {
    #[doc = "Builds a relative URL path to the Transforms Preview API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsPreviewParts::None => "/_plugins/_transform/_preview".into(),
        }
    }
}
#[doc = "Builder for the [Transforms Preview API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#preview-a-transform-jobs-results)\n\nReturns a preview of what a transformed index would look like."]
#[derive(Clone, Debug)]
pub struct TransformsPreview<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformsPreviewParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformsPreview<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformsPreview]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TransformsPreview {
            transport,
            parts: TransformsPreviewParts::None,
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
    pub fn body<T>(self, body: T) -> TransformsPreview<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformsPreview {
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
    #[doc = "Creates an asynchronous call to the Transforms Preview API that can be awaited"]
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
#[doc = "API parts for the Transforms Put API"]
pub enum TransformsPutParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsPutParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsPutParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Put API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#create-a-transform-job)\n\nCreate an index transform, or update a transform if `if_seq_no` and `if_primary_term` are provided."]
#[derive(Clone, Debug)]
pub struct TransformsPut<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformsPutParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformsPut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformsPut] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsPutParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsPut {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TransformsPut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformsPut {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
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
    #[doc = "Creates an asynchronous call to the Transforms Put API that can be awaited"]
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
                if_primary_term: Option<i64>,
                if_seq_no: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
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
#[doc = "API parts for the Transforms Search API"]
pub enum TransformsSearchParts {
    #[doc = "No parts"]
    None,
}
impl TransformsSearchParts {
    #[doc = "Builds a relative URL path to the Transforms Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsSearchParts::None => "/_plugins/_transform".into(),
        }
    }
}
#[doc = "Builder for the [Transforms Search API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-a-transform-jobs-details)\n\nReturns the details of all transform jobs."]
#[derive(Clone, Debug)]
pub struct TransformsSearch<'a, 'b> {
    transport: &'a Transport,
    parts: TransformsSearchParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    search: Option<&'b str>,
    size: Option<i64>,
    sortdirection: Option<&'b str>,
    sortfield: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b> TransformsSearch<'a, 'b> {
    #[doc = "Creates a new instance of [TransformsSearch]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        TransformsSearch {
            transport,
            parts: TransformsSearchParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            request_timeout: None,
            search: None,
            size: None,
            sortdirection: None,
            sortfield: None,
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
    #[doc = "The starting transform to return. Default is `0`."]
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
    #[doc = "The search term to use to filter results."]
    pub fn search(mut self, search: &'b str) -> Self {
        self.search = Some(search);
        self
    }
    #[doc = "Specifies the number of transforms to return. Default is `10`."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "Specifies the direction to sort results in. Can be `ASC` or `DESC`. Default is `ASC`."]
    pub fn sortdirection(mut self, sortdirection: &'b str) -> Self {
        self.sortdirection = Some(sortdirection);
        self
    }
    #[doc = "The field to sort results with."]
    pub fn sortfield(mut self, sortfield: &'b str) -> Self {
        self.sortfield = Some(sortfield);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Transforms Search API that can be awaited"]
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
                from: Option<i64>,
                human: Option<bool>,
                pretty: Option<bool>,
                search: Option<&'b str>,
                size: Option<i64>,
                #[serde(rename = "sortDirection")]
                sortdirection: Option<&'b str>,
                #[serde(rename = "sortField")]
                sortfield: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                search: self.search,
                size: self.size,
                sortdirection: self.sortdirection,
                sortfield: self.sortfield,
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
#[doc = "API parts for the Transforms Start API"]
pub enum TransformsStartParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsStartParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Start API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsStartParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(28usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Start API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#start-a-transform-job)\n\nStart transform."]
#[derive(Clone, Debug)]
pub struct TransformsStart<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformsStartParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformsStart<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformsStart] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsStartParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsStart {
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
    pub fn body<T>(self, body: T) -> TransformsStart<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformsStart {
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
    #[doc = "Creates an asynchronous call to the Transforms Start API that can be awaited"]
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
#[doc = "API parts for the Transforms Stop API"]
pub enum TransformsStopParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> TransformsStopParts<'b> {
    #[doc = "Builds a relative URL path to the Transforms Stop API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            TransformsStopParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_id.len());
                p.push_str("/_plugins/_transform/");
                p.push_str(encoded_id.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Transforms Stop API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#stop-a-transform-job)\n\nStop transform."]
#[derive(Clone, Debug)]
pub struct TransformsStop<'a, 'b, B> {
    transport: &'a Transport,
    parts: TransformsStopParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> TransformsStop<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [TransformsStop] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: TransformsStopParts<'b>) -> Self {
        let headers = HeaderMap::new();
        TransformsStop {
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
    pub fn body<T>(self, body: T) -> TransformsStop<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TransformsStop {
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
    #[doc = "Creates an asynchronous call to the Transforms Stop API that can be awaited"]
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
#[doc = "Namespace client for Transforms APIs"]
pub struct Transforms<'a> {
    transport: &'a Transport,
}
impl<'a> Transforms<'a> {
    #[doc = "Creates a new instance of [Transforms]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Transforms Delete API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#delete-a-transform-job)\n\nDelete an index transform."]
    pub fn delete<'b>(&'a self, parts: TransformsDeleteParts<'b>) -> TransformsDelete<'a, 'b> {
        TransformsDelete::new(self.transport(), parts)
    }
    #[doc = "[Transforms Explain API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-the-status-of-a-transform-job)\n\nReturns the status and metadata of a transform job."]
    pub fn explain<'b>(&'a self, parts: TransformsExplainParts<'b>) -> TransformsExplain<'a, 'b> {
        TransformsExplain::new(self.transport(), parts)
    }
    #[doc = "[Transforms Get API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-a-transform-jobs-details)\n\nReturns the status and metadata of a transform job."]
    pub fn get<'b>(&'a self, parts: TransformsGetParts<'b>) -> TransformsGet<'a, 'b> {
        TransformsGet::new(self.transport(), parts)
    }
    #[doc = "[Transforms Preview API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#preview-a-transform-jobs-results)\n\nReturns a preview of what a transformed index would look like."]
    pub fn preview<'b>(&'a self) -> TransformsPreview<'a, 'b, ()> {
        TransformsPreview::new(self.transport())
    }
    #[doc = "[Transforms Put API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#create-a-transform-job)\n\nCreate an index transform, or update a transform if `if_seq_no` and `if_primary_term` are provided."]
    pub fn put<'b>(&'a self, parts: TransformsPutParts<'b>) -> TransformsPut<'a, 'b, ()> {
        TransformsPut::new(self.transport(), parts)
    }
    #[doc = "[Transforms Search API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#get-a-transform-jobs-details)\n\nReturns the details of all transform jobs."]
    pub fn search<'b>(&'a self) -> TransformsSearch<'a, 'b> {
        TransformsSearch::new(self.transport())
    }
    #[doc = "[Transforms Start API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#start-a-transform-job)\n\nStart transform."]
    pub fn start<'b>(&'a self, parts: TransformsStartParts<'b>) -> TransformsStart<'a, 'b, ()> {
        TransformsStart::new(self.transport(), parts)
    }
    #[doc = "[Transforms Stop API](https://opensearch.org/docs/latest/im-plugin/index-transforms/transforms-apis/#stop-a-transform-job)\n\nStop transform."]
    pub fn stop<'b>(&'a self, parts: TransformsStopParts<'b>) -> TransformsStop<'a, 'b, ()> {
        TransformsStop::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Transforms APIs"]
    pub fn transforms(&self) -> Transforms {
        Transforms::new(self.transport())
    }
}
