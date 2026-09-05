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
#[doc = "API parts for the Remote Store Restore API"]
pub enum RemoteStoreRestoreParts {
    #[doc = "No parts"]
    None,
}
impl RemoteStoreRestoreParts {
    #[doc = "Builds a relative URL path to the Remote Store Restore API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            RemoteStoreRestoreParts::None => "/_remotestore/_restore".into(),
        }
    }
}
#[doc = "Builder for the [Remote Store Restore API](https://opensearch.org/docs/latest/opensearch/remote/#restoring-from-a-backup)\n\nRestores from remote store."]
#[derive(Clone, Debug)]
pub struct RemoteStoreRestore<'a, 'b, B> {
    transport: &'a Transport,
    parts: RemoteStoreRestoreParts,
    body: Option<B>,
    cluster_manager_timeout: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    wait_for_completion: Option<bool>,
}
impl<'a, 'b, B> RemoteStoreRestore<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [RemoteStoreRestore]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        RemoteStoreRestore {
            transport,
            parts: RemoteStoreRestoreParts::None,
            headers,
            body: None,
            cluster_manager_timeout: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            wait_for_completion: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> RemoteStoreRestore<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        RemoteStoreRestore {
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
            wait_for_completion: self.wait_for_completion,
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
    #[doc = "Should this request wait until the operation has completed before returning."]
    pub fn wait_for_completion(mut self, wait_for_completion: bool) -> Self {
        self.wait_for_completion = Some(wait_for_completion);
        self
    }
    #[doc = "Creates an asynchronous call to the Remote Store Restore API that can be awaited"]
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
                wait_for_completion: Option<bool>,
            }
            let query_params = QueryParams {
                cluster_manager_timeout: self.cluster_manager_timeout,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
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
#[doc = "Namespace client for RemoteStore APIs"]
pub struct RemoteStore<'a> {
    transport: &'a Transport,
}
impl<'a> RemoteStore<'a> {
    #[doc = "Creates a new instance of [RemoteStore]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Remote Store Restore API](https://opensearch.org/docs/latest/opensearch/remote/#restoring-from-a-backup)\n\nRestores from remote store."]
    pub fn restore<'b>(&'a self) -> RemoteStoreRestore<'a, 'b, ()> {
        RemoteStoreRestore::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for RemoteStore APIs"]
    pub fn remote_store(&self) -> RemoteStore {
        RemoteStore::new(self.transport())
    }
}
