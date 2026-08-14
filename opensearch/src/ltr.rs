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
#[doc = "API parts for the Ltr Add Features To Set API"]
pub enum LtrAddFeaturesToSetParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
    #[doc = "Store and Name"]
    StoreName(&'b str, &'b str),
}
impl<'b> LtrAddFeaturesToSetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Add Features To Set API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrAddFeaturesToSetParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_name.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_addfeatures");
                p.into()
            }
            LtrAddFeaturesToSetParts::StoreName(store, name) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(32usize + encoded_store.len() + encoded_name.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_addfeatures");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Add Features To Set API\n\nAdd features to an existing feature set in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrAddFeaturesToSet<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrAddFeaturesToSetParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    merge: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    version: Option<i64>,
}
impl<'a, 'b, B> LtrAddFeaturesToSet<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrAddFeaturesToSet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrAddFeaturesToSetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrAddFeaturesToSet {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            merge: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
            version: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrAddFeaturesToSet<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrAddFeaturesToSet {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            merge: self.merge,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            version: self.version,
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
    #[doc = "Whether to merge the feature list or append only."]
    pub fn merge(mut self, merge: bool) -> Self {
        self.merge = Some(merge);
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Version check to ensure feature set is modified with expected version."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Add Features To Set API that can be awaited"]
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
                merge: Option<bool>,
                pretty: Option<bool>,
                routing: Option<&'b str>,
                source: Option<&'b str>,
                version: Option<i64>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                merge: self.merge,
                pretty: self.pretty,
                routing: self.routing,
                source: self.source,
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
#[doc = "API parts for the Ltr Add Features To Set By Query API"]
pub enum LtrAddFeaturesToSetByQueryParts<'b> {
    #[doc = "Name and Query"]
    NameQuery(&'b str, &'b str),
    #[doc = "Store, Name and Query"]
    StoreNameQuery(&'b str, &'b str, &'b str),
}
impl<'b> LtrAddFeaturesToSetByQueryParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Add Features To Set By Query API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrAddFeaturesToSetByQueryParts::NameQuery(name, query) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let encoded_query: Cow<str> =
                    percent_encode(query.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(32usize + encoded_name.len() + encoded_query.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_addfeatures/");
                p.push_str(encoded_query.as_ref());
                p.into()
            }
            LtrAddFeaturesToSetByQueryParts::StoreNameQuery(store, name, query) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let encoded_query: Cow<str> =
                    percent_encode(query.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(
                    33usize + encoded_store.len() + encoded_name.len() + encoded_query.len(),
                );
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_addfeatures/");
                p.push_str(encoded_query.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Add Features To Set By Query API\n\nAdd features to an existing feature set in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrAddFeaturesToSetByQuery<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrAddFeaturesToSetByQueryParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    merge: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
    version: Option<i64>,
}
impl<'a, 'b, B> LtrAddFeaturesToSetByQuery<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrAddFeaturesToSetByQuery] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrAddFeaturesToSetByQueryParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrAddFeaturesToSetByQuery {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            merge: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
            version: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrAddFeaturesToSetByQuery<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrAddFeaturesToSetByQuery {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            merge: self.merge,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            routing: self.routing,
            source: self.source,
            version: self.version,
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
    #[doc = "Whether to merge the feature list or append only."]
    pub fn merge(mut self, merge: bool) -> Self {
        self.merge = Some(merge);
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Version check to ensure feature set is modified with expected version."]
    pub fn version(mut self, version: i64) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Add Features To Set By Query API that can be awaited"]
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
                merge: Option<bool>,
                pretty: Option<bool>,
                routing: Option<&'b str>,
                source: Option<&'b str>,
                version: Option<i64>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                merge: self.merge,
                pretty: self.pretty,
                routing: self.routing,
                source: self.source,
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
#[doc = "API parts for the Ltr Cache Stats API"]
pub enum LtrCacheStatsParts {
    #[doc = "No parts"]
    None,
}
impl LtrCacheStatsParts {
    #[doc = "Builds a relative URL path to the Ltr Cache Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCacheStatsParts::None => "/_ltr/_cachestats".into(),
        }
    }
}
#[doc = "Builder for the Ltr Cache Stats API\n\nRetrieves cache statistics for all feature stores."]
#[derive(Clone, Debug)]
pub struct LtrCacheStats<'a, 'b> {
    transport: &'a Transport,
    parts: LtrCacheStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrCacheStats<'a, 'b> {
    #[doc = "Creates a new instance of [LtrCacheStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        LtrCacheStats {
            transport,
            parts: LtrCacheStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Ltr Cache Stats API that can be awaited"]
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
#[doc = "API parts for the Ltr Clear Cache API"]
pub enum LtrClearCacheParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrClearCacheParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Clear Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrClearCacheParts::None => "/_ltr/_clearcache".into(),
            LtrClearCacheParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_clearcache");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Clear Cache API\n\nClears the store caches."]
#[derive(Clone, Debug)]
pub struct LtrClearCache<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrClearCacheParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrClearCache<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrClearCache] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrClearCacheParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrClearCache {
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
    pub fn body<T>(self, body: T) -> LtrClearCache<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrClearCache {
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
    #[doc = "Creates an asynchronous call to the Ltr Clear Cache API that can be awaited"]
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
#[doc = "API parts for the Ltr Create Default Store API"]
pub enum LtrCreateDefaultStoreParts {
    #[doc = "No parts"]
    None,
}
impl LtrCreateDefaultStoreParts {
    #[doc = "Builds a relative URL path to the Ltr Create Default Store API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateDefaultStoreParts::None => "/_ltr".into(),
        }
    }
}
#[doc = "Builder for the Ltr Create Default Store API\n\nCreates the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrCreateDefaultStore<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateDefaultStoreParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateDefaultStore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateDefaultStore]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        LtrCreateDefaultStore {
            transport,
            parts: LtrCreateDefaultStoreParts::None,
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
    pub fn body<T>(self, body: T) -> LtrCreateDefaultStore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateDefaultStore {
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
    #[doc = "Creates an asynchronous call to the Ltr Create Default Store API that can be awaited"]
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
#[doc = "API parts for the Ltr Create Feature API"]
pub enum LtrCreateFeatureParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrCreateFeatureParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Create Feature API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateFeatureParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_ltr/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrCreateFeatureParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Create Feature API\n\nCreate or update a feature in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrCreateFeature<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateFeatureParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateFeature<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateFeature] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrCreateFeatureParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrCreateFeature {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrCreateFeature<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateFeature {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Create Feature API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ltr Create Featureset API"]
pub enum LtrCreateFeaturesetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrCreateFeaturesetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Create Featureset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateFeaturesetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrCreateFeaturesetParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Create Featureset API\n\nCreate or update a feature set in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrCreateFeatureset<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateFeaturesetParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateFeatureset<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateFeatureset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrCreateFeaturesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrCreateFeatureset {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrCreateFeatureset<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateFeatureset {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Create Featureset API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ltr Create Model API"]
pub enum LtrCreateModelParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrCreateModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Create Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateModelParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_ltr/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrCreateModelParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Create Model API\n\nCreate or update a model in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrCreateModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrCreateModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrCreateModel {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrCreateModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateModel {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Create Model API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ltr Create Model From Set API"]
pub enum LtrCreateModelFromSetParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
    #[doc = "Store and Name"]
    StoreName(&'b str, &'b str),
}
impl<'b> LtrCreateModelFromSetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Create Model From Set API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateModelFromSetParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(31usize + encoded_name.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_createmodel");
                p.into()
            }
            LtrCreateModelFromSetParts::StoreName(store, name) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(32usize + encoded_store.len() + encoded_name.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_createmodel");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Create Model From Set API\n\nCreate a model from an existing feature set in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrCreateModelFromSet<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateModelFromSetParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateModelFromSet<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateModelFromSet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrCreateModelFromSetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrCreateModelFromSet {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrCreateModelFromSet<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateModelFromSet {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Create Model From Set API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ltr Create Store API"]
pub enum LtrCreateStoreParts<'b> {
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrCreateStoreParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Create Store API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrCreateStoreParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(6usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Create Store API\n\nCreates a new feature store with the specified name."]
#[derive(Clone, Debug)]
pub struct LtrCreateStore<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrCreateStoreParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrCreateStore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrCreateStore] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrCreateStoreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrCreateStore {
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
    pub fn body<T>(self, body: T) -> LtrCreateStore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrCreateStore {
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
    #[doc = "Creates an asynchronous call to the Ltr Create Store API that can be awaited"]
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
#[doc = "API parts for the Ltr Delete Default Store API"]
pub enum LtrDeleteDefaultStoreParts {
    #[doc = "No parts"]
    None,
}
impl LtrDeleteDefaultStoreParts {
    #[doc = "Builds a relative URL path to the Ltr Delete Default Store API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrDeleteDefaultStoreParts::None => "/_ltr".into(),
        }
    }
}
#[doc = "Builder for the Ltr Delete Default Store API\n\nDeletes the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrDeleteDefaultStore<'a, 'b> {
    transport: &'a Transport,
    parts: LtrDeleteDefaultStoreParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrDeleteDefaultStore<'a, 'b> {
    #[doc = "Creates a new instance of [LtrDeleteDefaultStore]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        LtrDeleteDefaultStore {
            transport,
            parts: LtrDeleteDefaultStoreParts::None,
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
    #[doc = "Creates an asynchronous call to the Ltr Delete Default Store API that can be awaited"]
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
#[doc = "API parts for the Ltr Delete Feature API"]
pub enum LtrDeleteFeatureParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrDeleteFeatureParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Delete Feature API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrDeleteFeatureParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_ltr/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrDeleteFeatureParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Delete Feature API\n\nDelete a feature from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrDeleteFeature<'a, 'b> {
    transport: &'a Transport,
    parts: LtrDeleteFeatureParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrDeleteFeature<'a, 'b> {
    #[doc = "Creates a new instance of [LtrDeleteFeature] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrDeleteFeatureParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrDeleteFeature {
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
    #[doc = "Creates an asynchronous call to the Ltr Delete Feature API that can be awaited"]
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
#[doc = "API parts for the Ltr Delete Featureset API"]
pub enum LtrDeleteFeaturesetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrDeleteFeaturesetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Delete Featureset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrDeleteFeaturesetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrDeleteFeaturesetParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Delete Featureset API\n\nDelete a feature set from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrDeleteFeatureset<'a, 'b> {
    transport: &'a Transport,
    parts: LtrDeleteFeaturesetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrDeleteFeatureset<'a, 'b> {
    #[doc = "Creates a new instance of [LtrDeleteFeatureset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrDeleteFeaturesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrDeleteFeatureset {
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
    #[doc = "Creates an asynchronous call to the Ltr Delete Featureset API that can be awaited"]
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
#[doc = "API parts for the Ltr Delete Model API"]
pub enum LtrDeleteModelParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrDeleteModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Delete Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrDeleteModelParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_ltr/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrDeleteModelParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Delete Model API\n\nDelete a model from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrDeleteModel<'a, 'b> {
    transport: &'a Transport,
    parts: LtrDeleteModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrDeleteModel<'a, 'b> {
    #[doc = "Creates a new instance of [LtrDeleteModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrDeleteModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrDeleteModel {
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
    #[doc = "Creates an asynchronous call to the Ltr Delete Model API that can be awaited"]
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
#[doc = "API parts for the Ltr Delete Store API"]
pub enum LtrDeleteStoreParts<'b> {
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrDeleteStoreParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Delete Store API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrDeleteStoreParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(6usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Delete Store API\n\nDeletes a feature store with the specified name."]
#[derive(Clone, Debug)]
pub struct LtrDeleteStore<'a, 'b> {
    transport: &'a Transport,
    parts: LtrDeleteStoreParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrDeleteStore<'a, 'b> {
    #[doc = "Creates a new instance of [LtrDeleteStore] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrDeleteStoreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrDeleteStore {
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
    #[doc = "Creates an asynchronous call to the Ltr Delete Store API that can be awaited"]
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
#[doc = "API parts for the Ltr Get Feature API"]
pub enum LtrGetFeatureParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrGetFeatureParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Get Feature API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrGetFeatureParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_ltr/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrGetFeatureParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Get Feature API\n\nGet a feature from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrGetFeature<'a, 'b> {
    transport: &'a Transport,
    parts: LtrGetFeatureParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrGetFeature<'a, 'b> {
    #[doc = "Creates a new instance of [LtrGetFeature] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrGetFeatureParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrGetFeature {
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
    #[doc = "Creates an asynchronous call to the Ltr Get Feature API that can be awaited"]
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
#[doc = "API parts for the Ltr Get Featureset API"]
pub enum LtrGetFeaturesetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrGetFeaturesetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Get Featureset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrGetFeaturesetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrGetFeaturesetParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Get Featureset API\n\nGet a feature set from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrGetFeatureset<'a, 'b> {
    transport: &'a Transport,
    parts: LtrGetFeaturesetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrGetFeatureset<'a, 'b> {
    #[doc = "Creates a new instance of [LtrGetFeatureset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrGetFeaturesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrGetFeatureset {
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
    #[doc = "Creates an asynchronous call to the Ltr Get Featureset API that can be awaited"]
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
#[doc = "API parts for the Ltr Get Model API"]
pub enum LtrGetModelParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrGetModelParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Get Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrGetModelParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_id.len());
                p.push_str("/_ltr/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrGetModelParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(14usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_model/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Get Model API\n\nGet a model from the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrGetModel<'a, 'b> {
    transport: &'a Transport,
    parts: LtrGetModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrGetModel<'a, 'b> {
    #[doc = "Creates a new instance of [LtrGetModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrGetModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrGetModel {
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
    #[doc = "Creates an asynchronous call to the Ltr Get Model API that can be awaited"]
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
#[doc = "API parts for the Ltr Get Store API"]
pub enum LtrGetStoreParts<'b> {
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrGetStoreParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Get Store API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrGetStoreParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(6usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Get Store API\n\nChecks if a store exists."]
#[derive(Clone, Debug)]
pub struct LtrGetStore<'a, 'b> {
    transport: &'a Transport,
    parts: LtrGetStoreParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrGetStore<'a, 'b> {
    #[doc = "Creates a new instance of [LtrGetStore] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrGetStoreParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrGetStore {
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
    #[doc = "Creates an asynchronous call to the Ltr Get Store API that can be awaited"]
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
#[doc = "API parts for the Ltr List Stores API"]
pub enum LtrListStoresParts {
    #[doc = "No parts"]
    None,
}
impl LtrListStoresParts {
    #[doc = "Builds a relative URL path to the Ltr List Stores API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrListStoresParts::None => "/_ltr".into(),
        }
    }
}
#[doc = "Builder for the Ltr List Stores API\n\nLists all available feature stores."]
#[derive(Clone, Debug)]
pub struct LtrListStores<'a, 'b> {
    transport: &'a Transport,
    parts: LtrListStoresParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrListStores<'a, 'b> {
    #[doc = "Creates a new instance of [LtrListStores]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        LtrListStores {
            transport,
            parts: LtrListStoresParts::None,
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
    #[doc = "Creates an asynchronous call to the Ltr List Stores API that can be awaited"]
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
#[doc = "API parts for the Ltr Search Features API"]
pub enum LtrSearchFeaturesParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrSearchFeaturesParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Search Features API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrSearchFeaturesParts::None => "/_ltr/_feature".into(),
            LtrSearchFeaturesParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_feature");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Search Features API\n\nSearch for features in a feature store."]
#[derive(Clone, Debug)]
pub struct LtrSearchFeatures<'a, 'b> {
    transport: &'a Transport,
    parts: LtrSearchFeaturesParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    prefix: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrSearchFeatures<'a, 'b> {
    #[doc = "Creates a new instance of [LtrSearchFeatures] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrSearchFeaturesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrSearchFeatures {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            prefix: None,
            pretty: None,
            request_timeout: None,
            size: None,
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
    #[doc = "The offset from the first result (for pagination)."]
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
    #[doc = "A name prefix to filter features by."]
    pub fn prefix(mut self, prefix: &'b str) -> Self {
        self.prefix = Some(prefix);
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
    #[doc = "The number of features to return."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Search Features API that can be awaited"]
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
                prefix: Option<&'b str>,
                pretty: Option<bool>,
                size: Option<i64>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                prefix: self.prefix,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Ltr Search Featuresets API"]
pub enum LtrSearchFeaturesetsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrSearchFeaturesetsParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Search Featuresets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrSearchFeaturesetsParts::None => "/_ltr/_featureset".into(),
            LtrSearchFeaturesetsParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Search Featuresets API\n\nSearch for feature sets in a feature store."]
#[derive(Clone, Debug)]
pub struct LtrSearchFeaturesets<'a, 'b> {
    transport: &'a Transport,
    parts: LtrSearchFeaturesetsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    prefix: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrSearchFeaturesets<'a, 'b> {
    #[doc = "Creates a new instance of [LtrSearchFeaturesets] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrSearchFeaturesetsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrSearchFeaturesets {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            prefix: None,
            pretty: None,
            request_timeout: None,
            size: None,
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
    #[doc = "The offset from the first result (for pagination)."]
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
    #[doc = "A name prefix to filter feature sets by."]
    pub fn prefix(mut self, prefix: &'b str) -> Self {
        self.prefix = Some(prefix);
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
    #[doc = "The number of feature sets to return."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Search Featuresets API that can be awaited"]
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
                prefix: Option<&'b str>,
                pretty: Option<bool>,
                size: Option<i64>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                prefix: self.prefix,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Ltr Search Models API"]
pub enum LtrSearchModelsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Store"]
    Store(&'b str),
}
impl<'b> LtrSearchModelsParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Search Models API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrSearchModelsParts::None => "/_ltr/_model".into(),
            LtrSearchModelsParts::Store(store) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(13usize + encoded_store.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_model");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Search Models API\n\nSearch for models in a feature store."]
#[derive(Clone, Debug)]
pub struct LtrSearchModels<'a, 'b> {
    transport: &'a Transport,
    parts: LtrSearchModelsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    prefix: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    source: Option<&'b str>,
}
impl<'a, 'b> LtrSearchModels<'a, 'b> {
    #[doc = "Creates a new instance of [LtrSearchModels] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrSearchModelsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrSearchModels {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            prefix: None,
            pretty: None,
            request_timeout: None,
            size: None,
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
    #[doc = "The offset from the first result (for pagination)."]
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
    #[doc = "A name prefix to filter models by."]
    pub fn prefix(mut self, prefix: &'b str) -> Self {
        self.prefix = Some(prefix);
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
    #[doc = "The number of models to return."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Search Models API that can be awaited"]
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
                prefix: Option<&'b str>,
                pretty: Option<bool>,
                size: Option<i64>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                prefix: self.prefix,
                pretty: self.pretty,
                size: self.size,
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
#[doc = "API parts for the Ltr Stats API"]
pub enum LtrStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Stat"]
    Stat(&'b str),
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
    #[doc = "NodeId and Stat"]
    NodeIdStat(&'b [&'b str], &'b str),
}
impl<'b> LtrStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrStatsParts::None => "/_plugins/_ltr/stats".into(),
            LtrStatsParts::Stat(stat) => {
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_stat.len());
                p.push_str("/_plugins/_ltr/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
            LtrStatsParts::NodeId(node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_node_id.len());
                p.push_str("/_plugins/_ltr/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            LtrStatsParts::NodeIdStat(node_id, stat) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(22usize + encoded_node_id.len() + encoded_stat.len());
                p.push_str("/_plugins/_ltr/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Stats API\n\nProvides information about the current status of the LTR plugin."]
#[derive(Clone, Debug)]
pub struct LtrStats<'a, 'b> {
    transport: &'a Transport,
    parts: LtrStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> LtrStats<'a, 'b> {
    #[doc = "Creates a new instance of [LtrStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
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
    #[doc = "The time in milliseconds to wait for a response."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Stats API that can be awaited"]
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
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc = "API parts for the Ltr Update Feature API"]
pub enum LtrUpdateFeatureParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrUpdateFeatureParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Update Feature API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrUpdateFeatureParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(15usize + encoded_id.len());
                p.push_str("/_ltr/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrUpdateFeatureParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(16usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_feature/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Update Feature API\n\nUpdate a feature in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrUpdateFeature<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrUpdateFeatureParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrUpdateFeature<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrUpdateFeature] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrUpdateFeatureParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrUpdateFeature {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrUpdateFeature<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrUpdateFeature {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Update Feature API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "API parts for the Ltr Update Featureset API"]
pub enum LtrUpdateFeaturesetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
    #[doc = "Store and Id"]
    StoreId(&'b str, &'b str),
}
impl<'b> LtrUpdateFeaturesetParts<'b> {
    #[doc = "Builds a relative URL path to the Ltr Update Featureset API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            LtrUpdateFeaturesetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(18usize + encoded_id.len());
                p.push_str("/_ltr/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
            LtrUpdateFeaturesetParts::StoreId(store, id) => {
                let encoded_store: Cow<str> =
                    percent_encode(store.as_bytes(), PARTS_ENCODED).into();
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(19usize + encoded_store.len() + encoded_id.len());
                p.push_str("/_ltr/");
                p.push_str(encoded_store.as_ref());
                p.push_str("/_featureset/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Ltr Update Featureset API\n\nUpdate a feature set in the default feature store."]
#[derive(Clone, Debug)]
pub struct LtrUpdateFeatureset<'a, 'b, B> {
    transport: &'a Transport,
    parts: LtrUpdateFeaturesetParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    routing: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> LtrUpdateFeatureset<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [LtrUpdateFeatureset] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: LtrUpdateFeaturesetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        LtrUpdateFeatureset {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            routing: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> LtrUpdateFeatureset<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        LtrUpdateFeatureset {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
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
    #[doc = "Specific routing value."]
    pub fn routing(mut self, routing: &'b str) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Ltr Update Featureset API that can be awaited"]
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
                routing: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
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
#[doc = "Namespace client for Ltr APIs"]
pub struct Ltr<'a> {
    transport: &'a Transport,
}
impl<'a> Ltr<'a> {
    #[doc = "Creates a new instance of [Ltr]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Ltr Add Features To Set API\n\nAdd features to an existing feature set in the default feature store."]
    pub fn add_features_to_set<'b>(
        &'a self,
        parts: LtrAddFeaturesToSetParts<'b>,
    ) -> LtrAddFeaturesToSet<'a, 'b, ()> {
        LtrAddFeaturesToSet::new(self.transport(), parts)
    }
    #[doc = "Ltr Add Features To Set By Query API\n\nAdd features to an existing feature set in the default feature store."]
    pub fn add_features_to_set_by_query<'b>(
        &'a self,
        parts: LtrAddFeaturesToSetByQueryParts<'b>,
    ) -> LtrAddFeaturesToSetByQuery<'a, 'b, ()> {
        LtrAddFeaturesToSetByQuery::new(self.transport(), parts)
    }
    #[doc = "Ltr Cache Stats API\n\nRetrieves cache statistics for all feature stores."]
    pub fn cache_stats<'b>(&'a self) -> LtrCacheStats<'a, 'b> {
        LtrCacheStats::new(self.transport())
    }
    #[doc = "Ltr Clear Cache API\n\nClears the store caches."]
    pub fn clear_cache<'b>(&'a self, parts: LtrClearCacheParts<'b>) -> LtrClearCache<'a, 'b, ()> {
        LtrClearCache::new(self.transport(), parts)
    }
    #[doc = "Ltr Create Default Store API\n\nCreates the default feature store."]
    pub fn create_default_store<'b>(&'a self) -> LtrCreateDefaultStore<'a, 'b, ()> {
        LtrCreateDefaultStore::new(self.transport())
    }
    #[doc = "Ltr Create Feature API\n\nCreate or update a feature in the default feature store."]
    pub fn create_feature<'b>(
        &'a self,
        parts: LtrCreateFeatureParts<'b>,
    ) -> LtrCreateFeature<'a, 'b, ()> {
        LtrCreateFeature::new(self.transport(), parts)
    }
    #[doc = "Ltr Create Featureset API\n\nCreate or update a feature set in the default feature store."]
    pub fn create_featureset<'b>(
        &'a self,
        parts: LtrCreateFeaturesetParts<'b>,
    ) -> LtrCreateFeatureset<'a, 'b, ()> {
        LtrCreateFeatureset::new(self.transport(), parts)
    }
    #[doc = "Ltr Create Model API\n\nCreate or update a model in the default feature store."]
    pub fn create_model<'b>(
        &'a self,
        parts: LtrCreateModelParts<'b>,
    ) -> LtrCreateModel<'a, 'b, ()> {
        LtrCreateModel::new(self.transport(), parts)
    }
    #[doc = "Ltr Create Model From Set API\n\nCreate a model from an existing feature set in the default feature store."]
    pub fn create_model_from_set<'b>(
        &'a self,
        parts: LtrCreateModelFromSetParts<'b>,
    ) -> LtrCreateModelFromSet<'a, 'b, ()> {
        LtrCreateModelFromSet::new(self.transport(), parts)
    }
    #[doc = "Ltr Create Store API\n\nCreates a new feature store with the specified name."]
    pub fn create_store<'b>(
        &'a self,
        parts: LtrCreateStoreParts<'b>,
    ) -> LtrCreateStore<'a, 'b, ()> {
        LtrCreateStore::new(self.transport(), parts)
    }
    #[doc = "Ltr Delete Default Store API\n\nDeletes the default feature store."]
    pub fn delete_default_store<'b>(&'a self) -> LtrDeleteDefaultStore<'a, 'b> {
        LtrDeleteDefaultStore::new(self.transport())
    }
    #[doc = "Ltr Delete Feature API\n\nDelete a feature from the default feature store."]
    pub fn delete_feature<'b>(
        &'a self,
        parts: LtrDeleteFeatureParts<'b>,
    ) -> LtrDeleteFeature<'a, 'b> {
        LtrDeleteFeature::new(self.transport(), parts)
    }
    #[doc = "Ltr Delete Featureset API\n\nDelete a feature set from the default feature store."]
    pub fn delete_featureset<'b>(
        &'a self,
        parts: LtrDeleteFeaturesetParts<'b>,
    ) -> LtrDeleteFeatureset<'a, 'b> {
        LtrDeleteFeatureset::new(self.transport(), parts)
    }
    #[doc = "Ltr Delete Model API\n\nDelete a model from the default feature store."]
    pub fn delete_model<'b>(&'a self, parts: LtrDeleteModelParts<'b>) -> LtrDeleteModel<'a, 'b> {
        LtrDeleteModel::new(self.transport(), parts)
    }
    #[doc = "Ltr Delete Store API\n\nDeletes a feature store with the specified name."]
    pub fn delete_store<'b>(&'a self, parts: LtrDeleteStoreParts<'b>) -> LtrDeleteStore<'a, 'b> {
        LtrDeleteStore::new(self.transport(), parts)
    }
    #[doc = "Ltr Get Feature API\n\nGet a feature from the default feature store."]
    pub fn get_feature<'b>(&'a self, parts: LtrGetFeatureParts<'b>) -> LtrGetFeature<'a, 'b> {
        LtrGetFeature::new(self.transport(), parts)
    }
    #[doc = "Ltr Get Featureset API\n\nGet a feature set from the default feature store."]
    pub fn get_featureset<'b>(
        &'a self,
        parts: LtrGetFeaturesetParts<'b>,
    ) -> LtrGetFeatureset<'a, 'b> {
        LtrGetFeatureset::new(self.transport(), parts)
    }
    #[doc = "Ltr Get Model API\n\nGet a model from the default feature store."]
    pub fn get_model<'b>(&'a self, parts: LtrGetModelParts<'b>) -> LtrGetModel<'a, 'b> {
        LtrGetModel::new(self.transport(), parts)
    }
    #[doc = "Ltr Get Store API\n\nChecks if a store exists."]
    pub fn get_store<'b>(&'a self, parts: LtrGetStoreParts<'b>) -> LtrGetStore<'a, 'b> {
        LtrGetStore::new(self.transport(), parts)
    }
    #[doc = "Ltr List Stores API\n\nLists all available feature stores."]
    pub fn list_stores<'b>(&'a self) -> LtrListStores<'a, 'b> {
        LtrListStores::new(self.transport())
    }
    #[doc = "Ltr Search Features API\n\nSearch for features in a feature store."]
    pub fn search_features<'b>(
        &'a self,
        parts: LtrSearchFeaturesParts<'b>,
    ) -> LtrSearchFeatures<'a, 'b> {
        LtrSearchFeatures::new(self.transport(), parts)
    }
    #[doc = "Ltr Search Featuresets API\n\nSearch for feature sets in a feature store."]
    pub fn search_featuresets<'b>(
        &'a self,
        parts: LtrSearchFeaturesetsParts<'b>,
    ) -> LtrSearchFeaturesets<'a, 'b> {
        LtrSearchFeaturesets::new(self.transport(), parts)
    }
    #[doc = "Ltr Search Models API\n\nSearch for models in a feature store."]
    pub fn search_models<'b>(&'a self, parts: LtrSearchModelsParts<'b>) -> LtrSearchModels<'a, 'b> {
        LtrSearchModels::new(self.transport(), parts)
    }
    #[doc = "Ltr Stats API\n\nProvides information about the current status of the LTR plugin."]
    pub fn stats<'b>(&'a self, parts: LtrStatsParts<'b>) -> LtrStats<'a, 'b> {
        LtrStats::new(self.transport(), parts)
    }
    #[doc = "Ltr Update Feature API\n\nUpdate a feature in the default feature store."]
    pub fn update_feature<'b>(
        &'a self,
        parts: LtrUpdateFeatureParts<'b>,
    ) -> LtrUpdateFeature<'a, 'b, ()> {
        LtrUpdateFeature::new(self.transport(), parts)
    }
    #[doc = "Ltr Update Featureset API\n\nUpdate a feature set in the default feature store."]
    pub fn update_featureset<'b>(
        &'a self,
        parts: LtrUpdateFeaturesetParts<'b>,
    ) -> LtrUpdateFeatureset<'a, 'b, ()> {
        LtrUpdateFeatureset::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Ltr APIs"]
    pub fn ltr(&self) -> Ltr {
        Ltr::new(self.transport())
    }
}
