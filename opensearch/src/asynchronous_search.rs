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
#[doc = "API parts for the Asynchronous Search Delete API"]
pub enum AsynchronousSearchDeleteParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> AsynchronousSearchDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Asynchronous Search Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsynchronousSearchDeleteParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(34usize + encoded_id.len());
                p.push_str("/_opendistro/_asynchronous_search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Asynchronous Search Delete API](https://opensearch.org/docs/latest/search-plugins/async/index/#delete-searches-and-results)\n\nDeletes any responses from an asynchronous search."]
#[derive(Clone, Debug)]
pub struct AsynchronousSearchDelete<'a, 'b> {
    transport: &'a Transport,
    parts: AsynchronousSearchDeleteParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AsynchronousSearchDelete<'a, 'b> {
    #[doc = "Creates a new instance of [AsynchronousSearchDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsynchronousSearchDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsynchronousSearchDelete {
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
    #[doc = "Creates an asynchronous call to the Asynchronous Search Delete API that can be awaited"]
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
#[doc = "API parts for the Asynchronous Search Get API"]
pub enum AsynchronousSearchGetParts<'b> {
    #[doc = "Id"]
    Id(&'b str),
}
impl<'b> AsynchronousSearchGetParts<'b> {
    #[doc = "Builds a relative URL path to the Asynchronous Search Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsynchronousSearchGetParts::Id(id) => {
                let encoded_id: Cow<str> = percent_encode(id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(34usize + encoded_id.len());
                p.push_str("/_opendistro/_asynchronous_search/");
                p.push_str(encoded_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Asynchronous Search Get API](https://opensearch.org/docs/latest/search-plugins/async/index/#get-partial-results)\n\nGets partial responses from an asynchronous search."]
#[derive(Clone, Debug)]
pub struct AsynchronousSearchGet<'a, 'b> {
    transport: &'a Transport,
    parts: AsynchronousSearchGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AsynchronousSearchGet<'a, 'b> {
    #[doc = "Creates a new instance of [AsynchronousSearchGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: AsynchronousSearchGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        AsynchronousSearchGet {
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
    #[doc = "Creates an asynchronous call to the Asynchronous Search Get API that can be awaited"]
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
#[doc = "API parts for the Asynchronous Search Search API"]
pub enum AsynchronousSearchSearchParts {
    #[doc = "No parts"]
    None,
}
impl AsynchronousSearchSearchParts {
    #[doc = "Builds a relative URL path to the Asynchronous Search Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsynchronousSearchSearchParts::None => "/_opendistro/_asynchronous_search".into(),
        }
    }
}
#[doc = "Builder for the [Asynchronous Search Search API](https://opensearch.org/docs/latest/search-plugins/async/index/#rest-api)\n\nPerforms an asynchronous search."]
#[derive(Clone, Debug)]
pub struct AsynchronousSearchSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: AsynchronousSearchSearchParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    keep_alive: Option<&'b str>,
    keep_on_completion: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion_timeout: Option<&'b str>,
}
impl<'a, 'b, B> AsynchronousSearchSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [AsynchronousSearchSearch]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        AsynchronousSearchSearch {
            transport,
            parts: AsynchronousSearchSearchParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            keep_alive: None,
            keep_on_completion: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion_timeout: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> AsynchronousSearchSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        AsynchronousSearchSearch {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
            keep_alive: self.keep_alive,
            keep_on_completion: self.keep_on_completion,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            wait_for_completion_timeout: self.wait_for_completion_timeout,
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
    #[doc = "The name of the index to be searched. Can be an individual name, a comma-separated list of indexes, or a wildcard expression of index names."]
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
        self
    }
    #[doc = "The amount of time that the result is saved in the cluster. For example, `2d` means that the results are stored in the cluster for 48 hours. \nThe saved search results are deleted after this period or if the search is canceled. Note that this includes the query execution time. \nIf the query exceeds this amount of time, the process cancels this query automatically."]
    pub fn keep_alive(mut self, keep_alive: &'b str) -> Self {
        self.keep_alive = Some(keep_alive);
        self
    }
    #[doc = "Whether to save the results in the cluster after the search is complete. You can examine the stored results at a later time."]
    pub fn keep_on_completion(mut self, keep_on_completion: bool) -> Self {
        self.keep_on_completion = Some(keep_on_completion);
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
    #[doc = "The amount of time to wait for the results. You can poll the remaining results based on an ID. The maximum value is 300 seconds. Default is `1s`."]
    pub fn wait_for_completion_timeout(mut self, wait_for_completion_timeout: &'b str) -> Self {
        self.wait_for_completion_timeout = Some(wait_for_completion_timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Asynchronous Search Search API that can be awaited"]
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
                index: Option<&'b str>,
                keep_alive: Option<&'b str>,
                keep_on_completion: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                wait_for_completion_timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
                keep_alive: self.keep_alive,
                keep_on_completion: self.keep_on_completion,
                pretty: self.pretty,
                source: self.source,
                wait_for_completion_timeout: self.wait_for_completion_timeout,
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
#[doc = "API parts for the Asynchronous Search Stats API"]
pub enum AsynchronousSearchStatsParts {
    #[doc = "No parts"]
    None,
}
impl AsynchronousSearchStatsParts {
    #[doc = "Builds a relative URL path to the Asynchronous Search Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            AsynchronousSearchStatsParts::None => "/_opendistro/_asynchronous_search/stats".into(),
        }
    }
}
#[doc = "Builder for the [Asynchronous Search Stats API](https://opensearch.org/docs/latest/search-plugins/async/index/#monitor-stats)\n\nMonitors any asynchronous searches that are `running`, `completed`, or `persisted`."]
#[derive(Clone, Debug)]
pub struct AsynchronousSearchStats<'a, 'b> {
    transport: &'a Transport,
    parts: AsynchronousSearchStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> AsynchronousSearchStats<'a, 'b> {
    #[doc = "Creates a new instance of [AsynchronousSearchStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        AsynchronousSearchStats {
            transport,
            parts: AsynchronousSearchStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Asynchronous Search Stats API that can be awaited"]
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
#[doc = "Namespace client for AsynchronousSearch APIs"]
pub struct AsynchronousSearch<'a> {
    transport: &'a Transport,
}
impl<'a> AsynchronousSearch<'a> {
    #[doc = "Creates a new instance of [AsynchronousSearch]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Asynchronous Search Delete API](https://opensearch.org/docs/latest/search-plugins/async/index/#delete-searches-and-results)\n\nDeletes any responses from an asynchronous search."]
    pub fn delete<'b>(
        &'a self,
        parts: AsynchronousSearchDeleteParts<'b>,
    ) -> AsynchronousSearchDelete<'a, 'b> {
        AsynchronousSearchDelete::new(self.transport(), parts)
    }
    #[doc = "[Asynchronous Search Get API](https://opensearch.org/docs/latest/search-plugins/async/index/#get-partial-results)\n\nGets partial responses from an asynchronous search."]
    pub fn get<'b>(
        &'a self,
        parts: AsynchronousSearchGetParts<'b>,
    ) -> AsynchronousSearchGet<'a, 'b> {
        AsynchronousSearchGet::new(self.transport(), parts)
    }
    #[doc = "[Asynchronous Search Search API](https://opensearch.org/docs/latest/search-plugins/async/index/#rest-api)\n\nPerforms an asynchronous search."]
    pub fn search<'b>(&'a self) -> AsynchronousSearchSearch<'a, 'b, ()> {
        AsynchronousSearchSearch::new(self.transport())
    }
    #[doc = "[Asynchronous Search Stats API](https://opensearch.org/docs/latest/search-plugins/async/index/#monitor-stats)\n\nMonitors any asynchronous searches that are `running`, `completed`, or `persisted`."]
    pub fn stats<'b>(&'a self) -> AsynchronousSearchStats<'a, 'b> {
        AsynchronousSearchStats::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for AsynchronousSearch APIs"]
    pub fn asynchronous_search(&self) -> AsynchronousSearch {
        AsynchronousSearch::new(self.transport())
    }
}
