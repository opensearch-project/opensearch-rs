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
#[doc = "API parts for the Wlm Create Query Group API"]
pub enum WlmCreateQueryGroupParts {
    #[doc = "No parts"]
    None,
}
impl WlmCreateQueryGroupParts {
    #[doc = "Builds a relative URL path to the Wlm Create Query Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmCreateQueryGroupParts::None => "/_wlm/query_group".into(),
        }
    }
}
#[doc = "Builder for the Wlm Create Query Group API\n\nCreates a new query group and sets the resource limits for the new query group."]
#[derive(Clone, Debug)]
pub struct WlmCreateQueryGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: WlmCreateQueryGroupParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WlmCreateQueryGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WlmCreateQueryGroup]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        WlmCreateQueryGroup {
            transport,
            parts: WlmCreateQueryGroupParts::None,
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
    pub fn body<T>(self, body: T) -> WlmCreateQueryGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WlmCreateQueryGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Create Query Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Create Workload Group API"]
pub enum WlmCreateWorkloadGroupParts {
    #[doc = "No parts"]
    None,
}
impl WlmCreateWorkloadGroupParts {
    #[doc = "Builds a relative URL path to the Wlm Create Workload Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmCreateWorkloadGroupParts::None => "/_wlm/workload_group".into(),
        }
    }
}
#[doc = "Builder for the Wlm Create Workload Group API\n\nCreates a new workload group and sets the resource limits for the new workload group."]
#[derive(Clone, Debug)]
pub struct WlmCreateWorkloadGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: WlmCreateWorkloadGroupParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WlmCreateWorkloadGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WlmCreateWorkloadGroup]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        WlmCreateWorkloadGroup {
            transport,
            parts: WlmCreateWorkloadGroupParts::None,
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
    pub fn body<T>(self, body: T) -> WlmCreateWorkloadGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WlmCreateWorkloadGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Create Workload Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Delete Query Group API"]
pub enum WlmDeleteQueryGroupParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmDeleteQueryGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Delete Query Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmDeleteQueryGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_wlm/query_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Delete Query Group API\n\nDeletes the specified query group."]
#[derive(Clone, Debug)]
pub struct WlmDeleteQueryGroup<'a, 'b> {
    transport: &'a Transport,
    parts: WlmDeleteQueryGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WlmDeleteQueryGroup<'a, 'b> {
    #[doc = "Creates a new instance of [WlmDeleteQueryGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmDeleteQueryGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmDeleteQueryGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Delete Query Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Delete Workload Group API"]
pub enum WlmDeleteWorkloadGroupParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmDeleteWorkloadGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Delete Workload Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmDeleteWorkloadGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_wlm/workload_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Delete Workload Group API\n\nDeletes the specified workload group."]
#[derive(Clone, Debug)]
pub struct WlmDeleteWorkloadGroup<'a, 'b> {
    transport: &'a Transport,
    parts: WlmDeleteWorkloadGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WlmDeleteWorkloadGroup<'a, 'b> {
    #[doc = "Creates a new instance of [WlmDeleteWorkloadGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmDeleteWorkloadGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmDeleteWorkloadGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Delete Workload Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Get Query Group API"]
pub enum WlmGetQueryGroupParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmGetQueryGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Get Query Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmGetQueryGroupParts::None => "/_wlm/query_group".into(),
            WlmGetQueryGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_wlm/query_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Get Query Group API\n\nRetrieves the specified query group. If no query group is specified, all query groups in the cluster are retrieved."]
#[derive(Clone, Debug)]
pub struct WlmGetQueryGroup<'a, 'b> {
    transport: &'a Transport,
    parts: WlmGetQueryGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WlmGetQueryGroup<'a, 'b> {
    #[doc = "Creates a new instance of [WlmGetQueryGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmGetQueryGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmGetQueryGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Get Query Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Get Workload Group API"]
pub enum WlmGetWorkloadGroupParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmGetWorkloadGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Get Workload Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmGetWorkloadGroupParts::None => "/_wlm/workload_group".into(),
            WlmGetWorkloadGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_wlm/workload_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Get Workload Group API\n\nRetrieves the specified workload group. If no workload group is specified, all workload groups in the cluster are retrieved."]
#[derive(Clone, Debug)]
pub struct WlmGetWorkloadGroup<'a, 'b> {
    transport: &'a Transport,
    parts: WlmGetWorkloadGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WlmGetWorkloadGroup<'a, 'b> {
    #[doc = "Creates a new instance of [WlmGetWorkloadGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmGetWorkloadGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmGetWorkloadGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Get Workload Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Stats API"]
pub enum WlmStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "WorkloadGroupId"]
    WorkloadGroupId(&'b [&'b str]),
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "NodeId and WorkloadGroupId"]
    NodeIdWorkloadGroupId(&'b [&'b str], &'b [&'b str]),
}
impl<'b> WlmStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmStatsParts::None => "/_wlm/stats".into(),
            WlmStatsParts::WorkloadGroupId(workload_group_id) => {
                let workload_group_id_str = workload_group_id.join(",");
                let encoded_workload_group_id: Cow<str> =
                    percent_encode(workload_group_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_workload_group_id.len());
                p.push_str("/_wlm/stats/");
                p.push_str(encoded_workload_group_id.as_ref());
                p.into()
            }
            WlmStatsParts::NodeId(node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(12usize + encoded_node_id.len());
                p.push_str("/_wlm/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            WlmStatsParts::NodeIdWorkloadGroupId(node_id, workload_group_id) => {
                let node_id_str = node_id.join(",");
                let workload_group_id_str = workload_group_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_workload_group_id: Cow<str> =
                    percent_encode(workload_group_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    13usize + encoded_node_id.len() + encoded_workload_group_id.len(),
                );
                p.push_str("/_wlm/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_workload_group_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Wlm Stats API](https://docs.opensearch.org/latest/tuning-your-cluster/availability-and-recovery/workload-management/wlm-feature-overview/)\n\nRetrieves workload management statistics for workload groups on one or more nodes."]
#[derive(Clone, Debug)]
pub struct WlmStats<'a, 'b> {
    transport: &'a Transport,
    parts: WlmStatsParts<'b>,
    breach: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> WlmStats<'a, 'b> {
    #[doc = "Creates a new instance of [WlmStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmStats {
            transport,
            parts,
            headers,
            breach: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "When `true`, returns only workload groups that have breached their resource limits."]
    pub fn breach(mut self, breach: bool) -> Self {
        self.breach = Some(breach);
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
    #[doc = "Creates an asynchronous call to the Wlm Stats API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                breach: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                breach: self.breach,
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
#[doc = "API parts for the Wlm Update Query Group API"]
pub enum WlmUpdateQueryGroupParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmUpdateQueryGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Update Query Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmUpdateQueryGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_name.len());
                p.push_str("/_wlm/query_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Update Query Group API\n\nUpdates the specified query group."]
#[derive(Clone, Debug)]
pub struct WlmUpdateQueryGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: WlmUpdateQueryGroupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WlmUpdateQueryGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WlmUpdateQueryGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmUpdateQueryGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmUpdateQueryGroup {
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
    pub fn body<T>(self, body: T) -> WlmUpdateQueryGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WlmUpdateQueryGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Update Query Group API that can be awaited"]
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
#[doc = "API parts for the Wlm Update Workload Group API"]
pub enum WlmUpdateWorkloadGroupParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> WlmUpdateWorkloadGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Wlm Update Workload Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            WlmUpdateWorkloadGroupParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_name.len());
                p.push_str("/_wlm/workload_group/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Wlm Update Workload Group API\n\nUpdates the specified workload group."]
#[derive(Clone, Debug)]
pub struct WlmUpdateWorkloadGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: WlmUpdateWorkloadGroupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> WlmUpdateWorkloadGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [WlmUpdateWorkloadGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: WlmUpdateWorkloadGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        WlmUpdateWorkloadGroup {
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
    pub fn body<T>(self, body: T) -> WlmUpdateWorkloadGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        WlmUpdateWorkloadGroup {
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
    #[doc = "Creates an asynchronous call to the Wlm Update Workload Group API that can be awaited"]
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
#[doc = "Namespace client for Wlm APIs"]
pub struct Wlm<'a> {
    transport: &'a Transport,
}
impl<'a> Wlm<'a> {
    #[doc = "Creates a new instance of [Wlm]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Wlm Create Query Group API\n\nCreates a new query group and sets the resource limits for the new query group."]
    pub fn create_query_group<'b>(&'a self) -> WlmCreateQueryGroup<'a, 'b, ()> {
        WlmCreateQueryGroup::new(self.transport())
    }
    #[doc = "Wlm Create Workload Group API\n\nCreates a new workload group and sets the resource limits for the new workload group."]
    pub fn create_workload_group<'b>(&'a self) -> WlmCreateWorkloadGroup<'a, 'b, ()> {
        WlmCreateWorkloadGroup::new(self.transport())
    }
    #[doc = "Wlm Delete Query Group API\n\nDeletes the specified query group."]
    pub fn delete_query_group<'b>(
        &'a self,
        parts: WlmDeleteQueryGroupParts<'b>,
    ) -> WlmDeleteQueryGroup<'a, 'b> {
        WlmDeleteQueryGroup::new(self.transport(), parts)
    }
    #[doc = "Wlm Delete Workload Group API\n\nDeletes the specified workload group."]
    pub fn delete_workload_group<'b>(
        &'a self,
        parts: WlmDeleteWorkloadGroupParts<'b>,
    ) -> WlmDeleteWorkloadGroup<'a, 'b> {
        WlmDeleteWorkloadGroup::new(self.transport(), parts)
    }
    #[doc = "Wlm Get Query Group API\n\nRetrieves the specified query group. If no query group is specified, all query groups in the cluster are retrieved."]
    pub fn get_query_group<'b>(
        &'a self,
        parts: WlmGetQueryGroupParts<'b>,
    ) -> WlmGetQueryGroup<'a, 'b> {
        WlmGetQueryGroup::new(self.transport(), parts)
    }
    #[doc = "Wlm Get Workload Group API\n\nRetrieves the specified workload group. If no workload group is specified, all workload groups in the cluster are retrieved."]
    pub fn get_workload_group<'b>(
        &'a self,
        parts: WlmGetWorkloadGroupParts<'b>,
    ) -> WlmGetWorkloadGroup<'a, 'b> {
        WlmGetWorkloadGroup::new(self.transport(), parts)
    }
    #[doc = "[Wlm Stats API](https://docs.opensearch.org/latest/tuning-your-cluster/availability-and-recovery/workload-management/wlm-feature-overview/)\n\nRetrieves workload management statistics for workload groups on one or more nodes."]
    pub fn stats<'b>(&'a self, parts: WlmStatsParts<'b>) -> WlmStats<'a, 'b> {
        WlmStats::new(self.transport(), parts)
    }
    #[doc = "Wlm Update Query Group API\n\nUpdates the specified query group."]
    pub fn update_query_group<'b>(
        &'a self,
        parts: WlmUpdateQueryGroupParts<'b>,
    ) -> WlmUpdateQueryGroup<'a, 'b, ()> {
        WlmUpdateQueryGroup::new(self.transport(), parts)
    }
    #[doc = "Wlm Update Workload Group API\n\nUpdates the specified workload group."]
    pub fn update_workload_group<'b>(
        &'a self,
        parts: WlmUpdateWorkloadGroupParts<'b>,
    ) -> WlmUpdateWorkloadGroup<'a, 'b, ()> {
        WlmUpdateWorkloadGroup::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Wlm APIs"]
    pub fn wlm(&self) -> Wlm {
        Wlm::new(self.transport())
    }
}
