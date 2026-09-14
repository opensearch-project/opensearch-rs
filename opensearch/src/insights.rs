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
#[doc = "API parts for the Insights Top Queries API"]
pub enum InsightsTopQueriesParts {
    #[doc = "No parts"]
    None,
}
impl InsightsTopQueriesParts {
    #[doc = "Builds a relative URL path to the Insights Top Queries API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            InsightsTopQueriesParts::None => "/_insights/top_queries".into(),
        }
    }
}
#[doc = "Builder for the Insights Top Queries API\n\nRetrieves the top queries based on the given metric type (latency, CPU, or memory)."]
#[derive(Clone, Debug)]
pub struct InsightsTopQueries<'a, 'b> {
    transport: &'a Transport,
    parts: InsightsTopQueriesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    ty: Option<Type>,
}
impl<'a, 'b> InsightsTopQueries<'a, 'b> {
    #[doc = "Creates a new instance of [InsightsTopQueries]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        InsightsTopQueries {
            transport,
            parts: InsightsTopQueriesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            ty: None,
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
    #[doc = "Get top n queries by a specific metric."]
    pub fn ty(mut self, ty: Type) -> Self {
        self.ty = Some(ty);
        self
    }
    #[doc = "Creates an asynchronous call to the Insights Top Queries API that can be awaited"]
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
                #[serde(rename = "type")]
                ty: Option<Type>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                ty: self.ty,
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
#[doc = "Namespace client for Insights APIs"]
pub struct Insights<'a> {
    transport: &'a Transport,
}
impl<'a> Insights<'a> {
    #[doc = "Creates a new instance of [Insights]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Insights Top Queries API\n\nRetrieves the top queries based on the given metric type (latency, CPU, or memory)."]
    pub fn top_queries<'b>(&'a self) -> InsightsTopQueries<'a, 'b> {
        InsightsTopQueries::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Insights APIs"]
    pub fn insights(&self) -> Insights {
        Insights::new(self.transport())
    }
}
