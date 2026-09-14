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
#[doc = "API parts for the Sm Create Policy API"]
pub enum SmCreatePolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmCreatePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Create Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmCreatePolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Create Policy API\n\nCreates a snapshot management policy."]
#[derive(Clone, Debug)]
pub struct SmCreatePolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: SmCreatePolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SmCreatePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SmCreatePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmCreatePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmCreatePolicy {
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
    pub fn body<T>(self, body: T) -> SmCreatePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SmCreatePolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Create Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Delete Policy API"]
pub enum SmDeletePolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmDeletePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Delete Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmDeletePolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Delete Policy API\n\nDeletes a snapshot management policy."]
#[derive(Clone, Debug)]
pub struct SmDeletePolicy<'a, 'b> {
    transport: &'a Transport,
    parts: SmDeletePolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SmDeletePolicy<'a, 'b> {
    #[doc = "Creates a new instance of [SmDeletePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmDeletePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmDeletePolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Delete Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Explain Policy API"]
pub enum SmExplainPolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmExplainPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Explain Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmExplainPolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.push_str("/_explain");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Explain Policy API\n\nExplains the state of the snapshot management policy."]
#[derive(Clone, Debug)]
pub struct SmExplainPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: SmExplainPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SmExplainPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [SmExplainPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmExplainPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmExplainPolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Explain Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Get Policies API"]
pub enum SmGetPoliciesParts {
    #[doc = "No parts"]
    None,
}
impl SmGetPoliciesParts {
    #[doc = "Builds a relative URL path to the Sm Get Policies API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmGetPoliciesParts::None => "/_plugins/_sm/policies".into(),
        }
    }
}
#[doc = "Builder for the Sm Get Policies API\n\nRetrieves all snapshot management policies with optional pagination and filtering."]
#[derive(Clone, Debug)]
pub struct SmGetPolicies<'a, 'b> {
    transport: &'a Transport,
    parts: SmGetPoliciesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    querystring: Option<&'b str>,
    request_timeout: Option<Duration>,
    size: Option<i64>,
    sortfield: Option<&'b str>,
    sortorder: Option<SortOrder>,
    source: Option<&'b str>,
}
impl<'a, 'b> SmGetPolicies<'a, 'b> {
    #[doc = "Creates a new instance of [SmGetPolicies]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SmGetPolicies {
            transport,
            parts: SmGetPoliciesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            from: None,
            human: None,
            pretty: None,
            querystring: None,
            request_timeout: None,
            size: None,
            sortfield: None,
            sortorder: None,
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
    #[doc = "The starting index from which to retrieve snapshot management policies."]
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
    #[doc = "The query string to filter the returned snapshot management policies."]
    pub fn querystring(mut self, querystring: &'b str) -> Self {
        self.querystring = Some(querystring);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "The number of snapshot management policies to return."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The name of the field to sort the snapshot management policies by."]
    pub fn sortfield(mut self, sortfield: &'b str) -> Self {
        self.sortfield = Some(sortfield);
        self
    }
    #[doc = "The order to sort the snapshot management policies."]
    pub fn sortorder(mut self, sortorder: SortOrder) -> Self {
        self.sortorder = Some(sortorder);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Sm Get Policies API that can be awaited"]
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
                #[serde(rename = "queryString")]
                querystring: Option<&'b str>,
                size: Option<i64>,
                #[serde(rename = "sortField")]
                sortfield: Option<&'b str>,
                #[serde(rename = "sortOrder")]
                sortorder: Option<SortOrder>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                pretty: self.pretty,
                querystring: self.querystring,
                size: self.size,
                sortfield: self.sortfield,
                sortorder: self.sortorder,
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
#[doc = "API parts for the Sm Get Policy API"]
pub enum SmGetPolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmGetPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Get Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmGetPolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Get Policy API\n\nRetrieves a specific snapshot management policy by name."]
#[derive(Clone, Debug)]
pub struct SmGetPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: SmGetPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SmGetPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [SmGetPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmGetPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmGetPolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Get Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Start Policy API"]
pub enum SmStartPolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmStartPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Start Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmStartPolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(30usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.push_str("/_start");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Start Policy API\n\nStarts a snapshot management policy."]
#[derive(Clone, Debug)]
pub struct SmStartPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: SmStartPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SmStartPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SmStartPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmStartPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmStartPolicy {
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
    pub fn body<T>(self, body: T) -> SmStartPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SmStartPolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Start Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Stop Policy API"]
pub enum SmStopPolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmStopPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Stop Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmStopPolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.push_str("/_stop");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Stop Policy API\n\nStops a snapshot management policy."]
#[derive(Clone, Debug)]
pub struct SmStopPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: SmStopPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SmStopPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SmStopPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmStopPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmStopPolicy {
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
    pub fn body<T>(self, body: T) -> SmStopPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SmStopPolicy {
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
    #[doc = "Creates an asynchronous call to the Sm Stop Policy API that can be awaited"]
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
#[doc = "API parts for the Sm Update Policy API"]
pub enum SmUpdatePolicyParts<'b> {
    #[doc = "PolicyName"]
    PolicyName(&'b str),
}
impl<'b> SmUpdatePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Sm Update Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SmUpdatePolicyParts::PolicyName(policy_name) => {
                let encoded_policy_name: Cow<str> =
                    percent_encode(policy_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(23usize + encoded_policy_name.len());
                p.push_str("/_plugins/_sm/policies/");
                p.push_str(encoded_policy_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Sm Update Policy API\n\nUpdates an existing snapshot management policy. Requires `if_seq_no` and `if_primary_term`."]
#[derive(Clone, Debug)]
pub struct SmUpdatePolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: SmUpdatePolicyParts<'b>,
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
impl<'a, 'b, B> SmUpdatePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SmUpdatePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SmUpdatePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SmUpdatePolicy {
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
    pub fn body<T>(self, body: T) -> SmUpdatePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SmUpdatePolicy {
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
    #[doc = "The primary term of the policy to update."]
    pub fn if_primary_term(mut self, if_primary_term: i64) -> Self {
        self.if_primary_term = Some(if_primary_term);
        self
    }
    #[doc = "The sequence number of the policy to update."]
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
    #[doc = "Creates an asynchronous call to the Sm Update Policy API that can be awaited"]
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
#[doc = "Namespace client for Sm APIs"]
pub struct Sm<'a> {
    transport: &'a Transport,
}
impl<'a> Sm<'a> {
    #[doc = "Creates a new instance of [Sm]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Sm Create Policy API\n\nCreates a snapshot management policy."]
    pub fn create_policy<'b>(
        &'a self,
        parts: SmCreatePolicyParts<'b>,
    ) -> SmCreatePolicy<'a, 'b, ()> {
        SmCreatePolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Delete Policy API\n\nDeletes a snapshot management policy."]
    pub fn delete_policy<'b>(&'a self, parts: SmDeletePolicyParts<'b>) -> SmDeletePolicy<'a, 'b> {
        SmDeletePolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Explain Policy API\n\nExplains the state of the snapshot management policy."]
    pub fn explain_policy<'b>(
        &'a self,
        parts: SmExplainPolicyParts<'b>,
    ) -> SmExplainPolicy<'a, 'b> {
        SmExplainPolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Get Policies API\n\nRetrieves all snapshot management policies with optional pagination and filtering."]
    pub fn get_policies<'b>(&'a self) -> SmGetPolicies<'a, 'b> {
        SmGetPolicies::new(self.transport())
    }
    #[doc = "Sm Get Policy API\n\nRetrieves a specific snapshot management policy by name."]
    pub fn get_policy<'b>(&'a self, parts: SmGetPolicyParts<'b>) -> SmGetPolicy<'a, 'b> {
        SmGetPolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Start Policy API\n\nStarts a snapshot management policy."]
    pub fn start_policy<'b>(&'a self, parts: SmStartPolicyParts<'b>) -> SmStartPolicy<'a, 'b, ()> {
        SmStartPolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Stop Policy API\n\nStops a snapshot management policy."]
    pub fn stop_policy<'b>(&'a self, parts: SmStopPolicyParts<'b>) -> SmStopPolicy<'a, 'b, ()> {
        SmStopPolicy::new(self.transport(), parts)
    }
    #[doc = "Sm Update Policy API\n\nUpdates an existing snapshot management policy. Requires `if_seq_no` and `if_primary_term`."]
    pub fn update_policy<'b>(
        &'a self,
        parts: SmUpdatePolicyParts<'b>,
    ) -> SmUpdatePolicy<'a, 'b, ()> {
        SmUpdatePolicy::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Sm APIs"]
    pub fn sm(&self) -> Sm {
        Sm::new(self.transport())
    }
}
