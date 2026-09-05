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
#[doc = "API parts for the Ism Add Policy API"]
pub enum IsmAddPolicyParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmAddPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Add Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmAddPolicyParts::None => "/_opendistro/_ism/add".into(),
            IsmAddPolicyParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_index.len());
                p.push_str("/_opendistro/_ism/add/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Add Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#add-policy)\n\nAdds a policy to an index."]
#[derive(Clone, Debug)]
pub struct IsmAddPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmAddPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmAddPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmAddPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmAddPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmAddPolicy {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IsmAddPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmAddPolicy {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
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
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Ism Add Policy API that can be awaited"]
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
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "API parts for the Ism Change Policy API"]
pub enum IsmChangePolicyParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmChangePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Change Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmChangePolicyParts::None => "/_opendistro/_ism/change_policy".into(),
            IsmChangePolicyParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_index.len());
                p.push_str("/_opendistro/_ism/change_policy/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Change Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#update-managed-index-policy)\n\nUpdates the managed index policy to a new policy."]
#[derive(Clone, Debug)]
pub struct IsmChangePolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmChangePolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmChangePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmChangePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmChangePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmChangePolicy {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IsmChangePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmChangePolicy {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
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
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Ism Change Policy API that can be awaited"]
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
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "API parts for the Ism Delete Policy API"]
pub enum IsmDeletePolicyParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> IsmDeletePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Delete Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmDeletePolicyParts::PolicyId(policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_policy_id.len());
                p.push_str("/_opendistro/_ism/policies/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Delete Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#delete-policy)\n\nDeletes a policy."]
#[derive(Clone, Debug)]
pub struct IsmDeletePolicy<'a, 'b> {
    transport: &'a Transport,
    parts: IsmDeletePolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IsmDeletePolicy<'a, 'b> {
    #[doc = "Creates a new instance of [IsmDeletePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmDeletePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmDeletePolicy {
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
    #[doc = "Creates an asynchronous call to the Ism Delete Policy API that can be awaited"]
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
#[doc = "API parts for the Ism Exists Policy API"]
pub enum IsmExistsPolicyParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> IsmExistsPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Exists Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmExistsPolicyParts::PolicyId(policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_policy_id.len());
                p.push_str("/_opendistro/_ism/policies/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Exists Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#get-policy)\n\nChecks for the existence of a policy."]
#[derive(Clone, Debug)]
pub struct IsmExistsPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: IsmExistsPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IsmExistsPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [IsmExistsPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmExistsPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmExistsPolicy {
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
    #[doc = "Creates an asynchronous call to the Ism Exists Policy API that can be awaited"]
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
#[doc = "API parts for the Ism Explain Policy API"]
pub enum IsmExplainPolicyParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmExplainPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Explain Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmExplainPolicyParts::None => "/_opendistro/_ism/explain".into(),
            IsmExplainPolicyParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(26usize + encoded_index.len());
                p.push_str("/_opendistro/_ism/explain/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Explain Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#explain-index)\n\nRetrieves the currently applied policy on the specified indexes."]
#[derive(Clone, Debug)]
pub struct IsmExplainPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmExplainPolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmExplainPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmExplainPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmExplainPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmExplainPolicy {
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
    pub fn body<T>(self, body: T) -> IsmExplainPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmExplainPolicy {
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
    #[doc = "Creates an asynchronous call to the Ism Explain Policy API that can be awaited"]
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
#[doc = "API parts for the Ism Get Policies API"]
pub enum IsmGetPoliciesParts {
    #[doc = "No parts"]
    None,
}
impl IsmGetPoliciesParts {
    #[doc = "Builds a relative URL path to the Ism Get Policies API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmGetPoliciesParts::None => "/_opendistro/_ism/policies".into(),
        }
    }
}
#[doc = "Builder for the [Ism Get Policies API](https://opensearch.org/docs/latest/im-plugin/ism/api/#get-policy)\n\nRetrieves the policies."]
#[derive(Clone, Debug)]
pub struct IsmGetPolicies<'a, 'b> {
    transport: &'a Transport,
    parts: IsmGetPoliciesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IsmGetPolicies<'a, 'b> {
    #[doc = "Creates a new instance of [IsmGetPolicies]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IsmGetPolicies {
            transport,
            parts: IsmGetPoliciesParts::None,
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
    #[doc = "Creates an asynchronous call to the Ism Get Policies API that can be awaited"]
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
#[doc = "API parts for the Ism Get Policy API"]
pub enum IsmGetPolicyParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> IsmGetPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Get Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmGetPolicyParts::PolicyId(policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_policy_id.len());
                p.push_str("/_opendistro/_ism/policies/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Get Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#put-policy)\n\nRetrieves a specific policy."]
#[derive(Clone, Debug)]
pub struct IsmGetPolicy<'a, 'b> {
    transport: &'a Transport,
    parts: IsmGetPolicyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> IsmGetPolicy<'a, 'b> {
    #[doc = "Creates a new instance of [IsmGetPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmGetPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmGetPolicy {
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
    #[doc = "Creates an asynchronous call to the Ism Get Policy API that can be awaited"]
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
#[doc = "API parts for the Ism Put Policies API"]
pub enum IsmPutPoliciesParts {
    #[doc = "No parts"]
    None,
}
impl IsmPutPoliciesParts {
    #[doc = "Builds a relative URL path to the Ism Put Policies API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmPutPoliciesParts::None => "/_opendistro/_ism/policies".into(),
        }
    }
}
#[doc = "Builder for the [Ism Put Policies API](https://opensearch.org/docs/latest/im-plugin/ism/api/#create-policy)\n\nCreates or updates policies."]
#[derive(Clone, Debug)]
pub struct IsmPutPolicies<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmPutPoliciesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    if_primary_term: Option<i64>,
    if_seq_no: Option<i64>,
    policyid: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmPutPolicies<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmPutPolicies]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        IsmPutPolicies {
            transport,
            parts: IsmPutPoliciesParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            if_primary_term: None,
            if_seq_no: None,
            policyid: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IsmPutPolicies<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmPutPolicies {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            if_primary_term: self.if_primary_term,
            if_seq_no: self.if_seq_no,
            policyid: self.policyid,
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
    pub fn policyid(mut self, policyid: &'b str) -> Self {
        self.policyid = Some(policyid);
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
    #[doc = "Creates an asynchronous call to the Ism Put Policies API that can be awaited"]
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
                #[serde(rename = "policyID")]
                policyid: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                if_primary_term: self.if_primary_term,
                if_seq_no: self.if_seq_no,
                policyid: self.policyid,
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
#[doc = "API parts for the Ism Put Policy API"]
pub enum IsmPutPolicyParts<'b> {
    #[doc = "PolicyId"]
    PolicyId(&'b str),
}
impl<'b> IsmPutPolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Put Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmPutPolicyParts::PolicyId(policy_id) => {
                let encoded_policy_id: Cow<str> =
                    percent_encode(policy_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(27usize + encoded_policy_id.len());
                p.push_str("/_opendistro/_ism/policies/");
                p.push_str(encoded_policy_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Put Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#create-policy)\n\nCreates or updates a policy."]
#[derive(Clone, Debug)]
pub struct IsmPutPolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmPutPolicyParts<'b>,
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
impl<'a, 'b, B> IsmPutPolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmPutPolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmPutPolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmPutPolicy {
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
    pub fn body<T>(self, body: T) -> IsmPutPolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmPutPolicy {
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
    #[doc = "Creates an asynchronous call to the Ism Put Policy API that can be awaited"]
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
#[doc = "API parts for the Ism Refresh Search Analyzers API"]
pub enum IsmRefreshSearchAnalyzersParts<'b> {
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmRefreshSearchAnalyzersParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Refresh Search Analyzers API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmRefreshSearchAnalyzersParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_index.len());
                p.push_str("/_opendistro/_refresh_search_analyzers/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Refresh Search Analyzers API](https://opensearch.org/docs/latest/im-plugin/refresh-analyzer/)\n\nRefreshes search analyzers in real time."]
#[derive(Clone, Debug)]
pub struct IsmRefreshSearchAnalyzers<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmRefreshSearchAnalyzersParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmRefreshSearchAnalyzers<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmRefreshSearchAnalyzers] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmRefreshSearchAnalyzersParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmRefreshSearchAnalyzers {
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
    pub fn body<T>(self, body: T) -> IsmRefreshSearchAnalyzers<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmRefreshSearchAnalyzers {
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
    #[doc = "Creates an asynchronous call to the Ism Refresh Search Analyzers API that can be awaited"]
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
#[doc = "API parts for the Ism Remove Policy API"]
pub enum IsmRemovePolicyParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmRemovePolicyParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Remove Policy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmRemovePolicyParts::None => "/_opendistro/_ism/remove".into(),
            IsmRemovePolicyParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_index.len());
                p.push_str("/_opendistro/_ism/remove/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Remove Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#remove-policy)\n\nRemoves a policy from an index."]
#[derive(Clone, Debug)]
pub struct IsmRemovePolicy<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmRemovePolicyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmRemovePolicy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmRemovePolicy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmRemovePolicyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmRemovePolicy {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IsmRemovePolicy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmRemovePolicy {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
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
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Ism Remove Policy API that can be awaited"]
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
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "API parts for the Ism Retry Index API"]
pub enum IsmRetryIndexParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Index"]
    Index(&'b str),
}
impl<'b> IsmRetryIndexParts<'b> {
    #[doc = "Builds a relative URL path to the Ism Retry Index API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            IsmRetryIndexParts::None => "/_opendistro/_ism/retry".into(),
            IsmRetryIndexParts::Index(index) => {
                let encoded_index: Cow<str> =
                    percent_encode(index.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_index.len());
                p.push_str("/_opendistro/_ism/retry/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Ism Retry Index API](https://opensearch.org/docs/latest/im-plugin/ism/api/#retry-failed-index)\n\nRetries the failed action for an index."]
#[derive(Clone, Debug)]
pub struct IsmRetryIndex<'a, 'b, B> {
    transport: &'a Transport,
    parts: IsmRetryIndexParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    index: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> IsmRetryIndex<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [IsmRetryIndex] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: IsmRetryIndexParts<'b>) -> Self {
        let headers = HeaderMap::new();
        IsmRetryIndex {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            index: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> IsmRetryIndex<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        IsmRetryIndex {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            index: self.index,
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
    pub fn index(mut self, index: &'b str) -> Self {
        self.index = Some(index);
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
    #[doc = "Creates an asynchronous call to the Ism Retry Index API that can be awaited"]
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
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                index: self.index,
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
#[doc = "Namespace client for Ism APIs"]
pub struct Ism<'a> {
    transport: &'a Transport,
}
impl<'a> Ism<'a> {
    #[doc = "Creates a new instance of [Ism]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Ism Add Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#add-policy)\n\nAdds a policy to an index."]
    pub fn add_policy<'b>(&'a self, parts: IsmAddPolicyParts<'b>) -> IsmAddPolicy<'a, 'b, ()> {
        IsmAddPolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Change Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#update-managed-index-policy)\n\nUpdates the managed index policy to a new policy."]
    pub fn change_policy<'b>(
        &'a self,
        parts: IsmChangePolicyParts<'b>,
    ) -> IsmChangePolicy<'a, 'b, ()> {
        IsmChangePolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Delete Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#delete-policy)\n\nDeletes a policy."]
    pub fn delete_policy<'b>(&'a self, parts: IsmDeletePolicyParts<'b>) -> IsmDeletePolicy<'a, 'b> {
        IsmDeletePolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Exists Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#get-policy)\n\nChecks for the existence of a policy."]
    pub fn exists_policy<'b>(&'a self, parts: IsmExistsPolicyParts<'b>) -> IsmExistsPolicy<'a, 'b> {
        IsmExistsPolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Explain Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#explain-index)\n\nRetrieves the currently applied policy on the specified indexes."]
    pub fn explain_policy<'b>(
        &'a self,
        parts: IsmExplainPolicyParts<'b>,
    ) -> IsmExplainPolicy<'a, 'b, ()> {
        IsmExplainPolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Get Policies API](https://opensearch.org/docs/latest/im-plugin/ism/api/#get-policy)\n\nRetrieves the policies."]
    pub fn get_policies<'b>(&'a self) -> IsmGetPolicies<'a, 'b> {
        IsmGetPolicies::new(self.transport())
    }
    #[doc = "[Ism Get Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#put-policy)\n\nRetrieves a specific policy."]
    pub fn get_policy<'b>(&'a self, parts: IsmGetPolicyParts<'b>) -> IsmGetPolicy<'a, 'b> {
        IsmGetPolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Put Policies API](https://opensearch.org/docs/latest/im-plugin/ism/api/#create-policy)\n\nCreates or updates policies."]
    pub fn put_policies<'b>(&'a self) -> IsmPutPolicies<'a, 'b, ()> {
        IsmPutPolicies::new(self.transport())
    }
    #[doc = "[Ism Put Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#create-policy)\n\nCreates or updates a policy."]
    pub fn put_policy<'b>(&'a self, parts: IsmPutPolicyParts<'b>) -> IsmPutPolicy<'a, 'b, ()> {
        IsmPutPolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Refresh Search Analyzers API](https://opensearch.org/docs/latest/im-plugin/refresh-analyzer/)\n\nRefreshes search analyzers in real time."]
    pub fn refresh_search_analyzers<'b>(
        &'a self,
        parts: IsmRefreshSearchAnalyzersParts<'b>,
    ) -> IsmRefreshSearchAnalyzers<'a, 'b, ()> {
        IsmRefreshSearchAnalyzers::new(self.transport(), parts)
    }
    #[doc = "[Ism Remove Policy API](https://opensearch.org/docs/latest/im-plugin/ism/api/#remove-policy)\n\nRemoves a policy from an index."]
    pub fn remove_policy<'b>(
        &'a self,
        parts: IsmRemovePolicyParts<'b>,
    ) -> IsmRemovePolicy<'a, 'b, ()> {
        IsmRemovePolicy::new(self.transport(), parts)
    }
    #[doc = "[Ism Retry Index API](https://opensearch.org/docs/latest/im-plugin/ism/api/#retry-failed-index)\n\nRetries the failed action for an index."]
    pub fn retry_index<'b>(&'a self, parts: IsmRetryIndexParts<'b>) -> IsmRetryIndex<'a, 'b, ()> {
        IsmRetryIndex::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Ism APIs"]
    pub fn ism(&self) -> Ism {
        Ism::new(self.transport())
    }
}
