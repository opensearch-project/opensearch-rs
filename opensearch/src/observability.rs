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
#[doc = "API parts for the Observability Create Object API"]
pub enum ObservabilityCreateObjectParts {
    #[doc = "No parts"]
    None,
}
impl ObservabilityCreateObjectParts {
    #[doc = "Builds a relative URL path to the Observability Create Object API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityCreateObjectParts::None => "/_plugins/_observability/object".into(),
        }
    }
}
#[doc = "Builder for the Observability Create Object API\n\nCreates a new observability object."]
#[derive(Clone, Debug)]
pub struct ObservabilityCreateObject<'a, 'b, B> {
    transport: &'a Transport,
    parts: ObservabilityCreateObjectParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ObservabilityCreateObject<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ObservabilityCreateObject]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ObservabilityCreateObject {
            transport,
            parts: ObservabilityCreateObjectParts::None,
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
    pub fn body<T>(self, body: T) -> ObservabilityCreateObject<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ObservabilityCreateObject {
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
    #[doc = "Creates an asynchronous call to the Observability Create Object API that can be awaited"]
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
#[doc = "API parts for the Observability Delete Object API"]
pub enum ObservabilityDeleteObjectParts<'b> {
    #[doc = "ObjectId"]
    ObjectId(&'b str),
}
impl<'b> ObservabilityDeleteObjectParts<'b> {
    #[doc = "Builds a relative URL path to the Observability Delete Object API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityDeleteObjectParts::ObjectId(object_id) => {
                let encoded_object_id: Cow<str> =
                    percent_encode(object_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_object_id.len());
                p.push_str("/_plugins/_observability/object/");
                p.push_str(encoded_object_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Observability Delete Object API\n\nDeletes specific observability object specified by ID."]
#[derive(Clone, Debug)]
pub struct ObservabilityDeleteObject<'a, 'b> {
    transport: &'a Transport,
    parts: ObservabilityDeleteObjectParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ObservabilityDeleteObject<'a, 'b> {
    #[doc = "Creates a new instance of [ObservabilityDeleteObject] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ObservabilityDeleteObjectParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ObservabilityDeleteObject {
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
    #[doc = "Creates an asynchronous call to the Observability Delete Object API that can be awaited"]
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
#[doc = "API parts for the Observability Delete Objects API"]
pub enum ObservabilityDeleteObjectsParts {
    #[doc = "No parts"]
    None,
}
impl ObservabilityDeleteObjectsParts {
    #[doc = "Builds a relative URL path to the Observability Delete Objects API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityDeleteObjectsParts::None => "/_plugins/_observability/object".into(),
        }
    }
}
#[doc = "Builder for the Observability Delete Objects API\n\nDeletes specific observability objects specified by ID or a list of IDs."]
#[derive(Clone, Debug)]
pub struct ObservabilityDeleteObjects<'a, 'b> {
    transport: &'a Transport,
    parts: ObservabilityDeleteObjectsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    objectid: Option<&'b str>,
    objectidlist: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ObservabilityDeleteObjects<'a, 'b> {
    #[doc = "Creates a new instance of [ObservabilityDeleteObjects]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ObservabilityDeleteObjects {
            transport,
            parts: ObservabilityDeleteObjectsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            objectid: None,
            objectidlist: None,
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
    #[doc = "The ID of a single observability object to delete."]
    pub fn objectid(mut self, objectid: &'b str) -> Self {
        self.objectid = Some(objectid);
        self
    }
    #[doc = "A comma-separated list of observability object IDs to delete."]
    pub fn objectidlist(mut self, objectidlist: &'b str) -> Self {
        self.objectidlist = Some(objectidlist);
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
    #[doc = "Creates an asynchronous call to the Observability Delete Objects API that can be awaited"]
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
                #[serde(rename = "objectId")]
                objectid: Option<&'b str>,
                #[serde(rename = "objectIdList")]
                objectidlist: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                objectid: self.objectid,
                objectidlist: self.objectidlist,
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
#[doc = "API parts for the Observability Get Localstats API"]
pub enum ObservabilityGetLocalstatsParts {
    #[doc = "No parts"]
    None,
}
impl ObservabilityGetLocalstatsParts {
    #[doc = "Builds a relative URL path to the Observability Get Localstats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityGetLocalstatsParts::None => "/_plugins/_observability/_local/stats".into(),
        }
    }
}
#[doc = "Builder for the Observability Get Localstats API\n\nRetrieves local stats of all observability objects."]
#[derive(Clone, Debug)]
pub struct ObservabilityGetLocalstats<'a, 'b> {
    transport: &'a Transport,
    parts: ObservabilityGetLocalstatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ObservabilityGetLocalstats<'a, 'b> {
    #[doc = "Creates a new instance of [ObservabilityGetLocalstats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ObservabilityGetLocalstats {
            transport,
            parts: ObservabilityGetLocalstatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Observability Get Localstats API that can be awaited"]
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
#[doc = "API parts for the Observability Get Object API"]
pub enum ObservabilityGetObjectParts<'b> {
    #[doc = "ObjectId"]
    ObjectId(&'b str),
}
impl<'b> ObservabilityGetObjectParts<'b> {
    #[doc = "Builds a relative URL path to the Observability Get Object API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityGetObjectParts::ObjectId(object_id) => {
                let encoded_object_id: Cow<str> =
                    percent_encode(object_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_object_id.len());
                p.push_str("/_plugins/_observability/object/");
                p.push_str(encoded_object_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Observability Get Object API\n\nRetrieves specific observability object specified by ID."]
#[derive(Clone, Debug)]
pub struct ObservabilityGetObject<'a, 'b> {
    transport: &'a Transport,
    parts: ObservabilityGetObjectParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ObservabilityGetObject<'a, 'b> {
    #[doc = "Creates a new instance of [ObservabilityGetObject] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ObservabilityGetObjectParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ObservabilityGetObject {
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
    #[doc = "Creates an asynchronous call to the Observability Get Object API that can be awaited"]
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
#[doc = "API parts for the Observability List Objects API"]
pub enum ObservabilityListObjectsParts {
    #[doc = "No parts"]
    None,
}
impl ObservabilityListObjectsParts {
    #[doc = "Builds a relative URL path to the Observability List Objects API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityListObjectsParts::None => "/_plugins/_observability/object".into(),
        }
    }
}
#[doc = "Builder for the Observability List Objects API\n\nRetrieves list of all observability objects."]
#[derive(Clone, Debug)]
pub struct ObservabilityListObjects<'a, 'b> {
    transport: &'a Transport,
    parts: ObservabilityListObjectsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> ObservabilityListObjects<'a, 'b> {
    #[doc = "Creates a new instance of [ObservabilityListObjects]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        ObservabilityListObjects {
            transport,
            parts: ObservabilityListObjectsParts::None,
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
    #[doc = "Creates an asynchronous call to the Observability List Objects API that can be awaited"]
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
#[doc = "API parts for the Observability Update Object API"]
pub enum ObservabilityUpdateObjectParts<'b> {
    #[doc = "ObjectId"]
    ObjectId(&'b str),
}
impl<'b> ObservabilityUpdateObjectParts<'b> {
    #[doc = "Builds a relative URL path to the Observability Update Object API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            ObservabilityUpdateObjectParts::ObjectId(object_id) => {
                let encoded_object_id: Cow<str> =
                    percent_encode(object_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_object_id.len());
                p.push_str("/_plugins/_observability/object/");
                p.push_str(encoded_object_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Observability Update Object API\n\nUpdates an existing observability object."]
#[derive(Clone, Debug)]
pub struct ObservabilityUpdateObject<'a, 'b, B> {
    transport: &'a Transport,
    parts: ObservabilityUpdateObjectParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> ObservabilityUpdateObject<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [ObservabilityUpdateObject] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: ObservabilityUpdateObjectParts<'b>) -> Self {
        let headers = HeaderMap::new();
        ObservabilityUpdateObject {
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
    pub fn body<T>(self, body: T) -> ObservabilityUpdateObject<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        ObservabilityUpdateObject {
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
    #[doc = "Creates an asynchronous call to the Observability Update Object API that can be awaited"]
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
#[doc = "Namespace client for Observability APIs"]
pub struct Observability<'a> {
    transport: &'a Transport,
}
impl<'a> Observability<'a> {
    #[doc = "Creates a new instance of [Observability]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Observability Create Object API\n\nCreates a new observability object."]
    pub fn create_object<'b>(&'a self) -> ObservabilityCreateObject<'a, 'b, ()> {
        ObservabilityCreateObject::new(self.transport())
    }
    #[doc = "Observability Delete Object API\n\nDeletes specific observability object specified by ID."]
    pub fn delete_object<'b>(
        &'a self,
        parts: ObservabilityDeleteObjectParts<'b>,
    ) -> ObservabilityDeleteObject<'a, 'b> {
        ObservabilityDeleteObject::new(self.transport(), parts)
    }
    #[doc = "Observability Delete Objects API\n\nDeletes specific observability objects specified by ID or a list of IDs."]
    pub fn delete_objects<'b>(&'a self) -> ObservabilityDeleteObjects<'a, 'b> {
        ObservabilityDeleteObjects::new(self.transport())
    }
    #[doc = "Observability Get Localstats API\n\nRetrieves local stats of all observability objects."]
    pub fn get_localstats<'b>(&'a self) -> ObservabilityGetLocalstats<'a, 'b> {
        ObservabilityGetLocalstats::new(self.transport())
    }
    #[doc = "Observability Get Object API\n\nRetrieves specific observability object specified by ID."]
    pub fn get_object<'b>(
        &'a self,
        parts: ObservabilityGetObjectParts<'b>,
    ) -> ObservabilityGetObject<'a, 'b> {
        ObservabilityGetObject::new(self.transport(), parts)
    }
    #[doc = "Observability List Objects API\n\nRetrieves list of all observability objects."]
    pub fn list_objects<'b>(&'a self) -> ObservabilityListObjects<'a, 'b> {
        ObservabilityListObjects::new(self.transport())
    }
    #[doc = "Observability Update Object API\n\nUpdates an existing observability object."]
    pub fn update_object<'b>(
        &'a self,
        parts: ObservabilityUpdateObjectParts<'b>,
    ) -> ObservabilityUpdateObject<'a, 'b, ()> {
        ObservabilityUpdateObject::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Observability APIs"]
    pub fn observability(&self) -> Observability {
        Observability::new(self.transport())
    }
}
