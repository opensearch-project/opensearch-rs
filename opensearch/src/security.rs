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
#[doc = "API parts for the Security Authinfo API"]
pub enum SecurityAuthinfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAuthinfoParts {
    #[doc = "Builds a relative URL path to the Security Authinfo API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAuthinfoParts::None => "/_opendistro/_security/authinfo".into(),
        }
    }
}
#[doc = "Builder for the Security Authinfo API\n\nReturns or updates authentication information for the currently authenticated user."]
#[derive(Clone, Debug)]
pub struct SecurityAuthinfo<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityAuthinfoParts,
    auth_type: Option<&'b str>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    verbose: Option<bool>,
}
impl<'a, 'b, B> SecurityAuthinfo<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityAuthinfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAuthinfo {
            transport,
            parts: SecurityAuthinfoParts::None,
            headers,
            auth_type: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            verbose: None,
        }
    }
    #[doc = "The type of the current authentication request."]
    pub fn auth_type(mut self, auth_type: &'b str) -> Self {
        self.auth_type = Some(auth_type);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityAuthinfo<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityAuthinfo {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            auth_type: self.auth_type,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            request_timeout: self.request_timeout,
            source: self.source,
            verbose: self.verbose,
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
    #[doc = "Whether to return a verbose response."]
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = Some(verbose);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Authinfo API that can be awaited"]
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
                auth_type: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                verbose: Option<bool>,
            }
            let query_params = QueryParams {
                auth_type: self.auth_type,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                verbose: self.verbose,
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
#[doc = "API parts for the Security Authtoken API"]
pub enum SecurityAuthtokenParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAuthtokenParts {
    #[doc = "Builds a relative URL path to the Security Authtoken API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAuthtokenParts::None => "/_opendistro/_security/api/authtoken".into(),
        }
    }
}
#[doc = "Builder for the Security Authtoken API\n\nReturns the authorization token for the current user."]
#[derive(Clone, Debug)]
pub struct SecurityAuthtoken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityAuthtokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityAuthtoken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityAuthtoken]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAuthtoken {
            transport,
            parts: SecurityAuthtokenParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityAuthtoken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityAuthtoken {
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
    #[doc = "Creates an asynchronous call to the Security Authtoken API that can be awaited"]
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
#[doc = "API parts for the Security Change Password API"]
pub enum SecurityChangePasswordParts {
    #[doc = "No parts"]
    None,
}
impl SecurityChangePasswordParts {
    #[doc = "Builds a relative URL path to the Security Change Password API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityChangePasswordParts::None => "/_opendistro/_security/api/account".into(),
        }
    }
}
#[doc = "Builder for the [Security Change Password API](https://opensearch.org/docs/latest/security/access-control/api/#change-password)\n\nChanges the password for the current user."]
#[derive(Clone, Debug)]
pub struct SecurityChangePassword<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityChangePasswordParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityChangePassword<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityChangePassword]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityChangePassword {
            transport,
            parts: SecurityChangePasswordParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityChangePassword<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityChangePassword {
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
    #[doc = "Creates an asynchronous call to the Security Change Password API that can be awaited"]
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
#[doc = "API parts for the Security Config Upgrade Check API"]
pub enum SecurityConfigUpgradeCheckParts {
    #[doc = "No parts"]
    None,
}
impl SecurityConfigUpgradeCheckParts {
    #[doc = "Builds a relative URL path to the Security Config Upgrade Check API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityConfigUpgradeCheckParts::None => {
                "/_plugins/_security/api/_upgrade_check".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Config Upgrade Check API](https://opensearch.org/docs/latest/security/access-control/api/#configuration-upgrade-check)\n\nChecks whether or not an upgrade can be performed and which security resources can be updated."]
#[derive(Clone, Debug)]
pub struct SecurityConfigUpgradeCheck<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityConfigUpgradeCheckParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityConfigUpgradeCheck<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityConfigUpgradeCheck]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityConfigUpgradeCheck {
            transport,
            parts: SecurityConfigUpgradeCheckParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Config Upgrade Check API that can be awaited"]
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
#[doc = "API parts for the Security Config Upgrade Perform API"]
pub enum SecurityConfigUpgradePerformParts {
    #[doc = "No parts"]
    None,
}
impl SecurityConfigUpgradePerformParts {
    #[doc = "Builds a relative URL path to the Security Config Upgrade Perform API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityConfigUpgradePerformParts::None => {
                "/_plugins/_security/api/_upgrade_perform".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Config Upgrade Perform API](https://opensearch.org/docs/latest/security/access-control/api/#configuration-upgrade)\n\nAssists the cluster operator with upgrading missing default values and stale default definitions."]
#[derive(Clone, Debug)]
pub struct SecurityConfigUpgradePerform<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityConfigUpgradePerformParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityConfigUpgradePerform<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityConfigUpgradePerform]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityConfigUpgradePerform {
            transport,
            parts: SecurityConfigUpgradePerformParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityConfigUpgradePerform<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityConfigUpgradePerform {
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
    #[doc = "Creates an asynchronous call to the Security Config Upgrade Perform API that can be awaited"]
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
#[doc = "API parts for the Security Create Action Group API"]
pub enum SecurityCreateActionGroupParts<'b> {
    #[doc = "ActionGroup"]
    ActionGroup(&'b str),
}
impl<'b> SecurityCreateActionGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create Action Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateActionGroupParts::ActionGroup(action_group) => {
                let encoded_action_group: Cow<str> =
                    percent_encode(action_group.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_action_group.len());
                p.push_str("/_opendistro/_security/api/actiongroups/");
                p.push_str(encoded_action_group.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#create-action-group)\n\nCreates or replaces the specified action group."]
#[derive(Clone, Debug)]
pub struct SecurityCreateActionGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateActionGroupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateActionGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateActionGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateActionGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateActionGroup {
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
    pub fn body<T>(self, body: T) -> SecurityCreateActionGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateActionGroup {
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
    #[doc = "Creates an asynchronous call to the Security Create Action Group API that can be awaited"]
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
#[doc = "API parts for the Security Create Allowlist API"]
pub enum SecurityCreateAllowlistParts {
    #[doc = "No parts"]
    None,
}
impl SecurityCreateAllowlistParts {
    #[doc = "Builds a relative URL path to the Security Create Allowlist API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateAllowlistParts::None => "/_opendistro/_security/api/allowlist".into(),
        }
    }
}
#[doc = "Builder for the [Security Create Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nCreates or replaces APIs permitted for users on the allow list. Requires a super admin certificate or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityCreateAllowlist<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateAllowlistParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateAllowlist<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateAllowlist]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateAllowlist {
            transport,
            parts: SecurityCreateAllowlistParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityCreateAllowlist<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateAllowlist {
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
    #[doc = "Creates an asynchronous call to the Security Create Allowlist API that can be awaited"]
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
#[doc = "API parts for the Security Create Role API"]
pub enum SecurityCreateRoleParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityCreateRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateRoleParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/roles/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Role API](https://opensearch.org/docs/latest/security/access-control/api/#create-role)\n\nCreates or replaces the specified role."]
#[derive(Clone, Debug)]
pub struct SecurityCreateRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateRoleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateRole {
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
    pub fn body<T>(self, body: T) -> SecurityCreateRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateRole {
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
    #[doc = "Creates an asynchronous call to the Security Create Role API that can be awaited"]
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
#[doc = "API parts for the Security Create Role Mapping API"]
pub enum SecurityCreateRoleMappingParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityCreateRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateRoleMappingParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/rolesmapping/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#create-role-mapping)\n\nCreates or replaces the specified role mapping."]
#[derive(Clone, Debug)]
pub struct SecurityCreateRoleMapping<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateRoleMappingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateRoleMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateRoleMapping {
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
    pub fn body<T>(self, body: T) -> SecurityCreateRoleMapping<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateRoleMapping {
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
    #[doc = "Creates an asynchronous call to the Security Create Role Mapping API that can be awaited"]
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
#[doc = "API parts for the Security Create Tenant API"]
pub enum SecurityCreateTenantParts<'b> {
    #[doc = "Tenant"]
    Tenant(&'b str),
}
impl<'b> SecurityCreateTenantParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create Tenant API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateTenantParts::Tenant(tenant) => {
                let encoded_tenant: Cow<str> =
                    percent_encode(tenant.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_tenant.len());
                p.push_str("/_opendistro/_security/api/tenants/");
                p.push_str(encoded_tenant.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#create-tenant)\n\nCreates or replaces the specified tenant."]
#[derive(Clone, Debug)]
pub struct SecurityCreateTenant<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateTenantParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateTenant<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateTenant] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateTenantParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateTenant {
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
    pub fn body<T>(self, body: T) -> SecurityCreateTenant<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateTenant {
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
    #[doc = "Creates an asynchronous call to the Security Create Tenant API that can be awaited"]
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
#[doc = "API parts for the Security Create Update Tenancy Config API"]
pub enum SecurityCreateUpdateTenancyConfigParts {
    #[doc = "No parts"]
    None,
}
impl SecurityCreateUpdateTenancyConfigParts {
    #[doc = "Builds a relative URL path to the Security Create Update Tenancy Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateUpdateTenancyConfigParts::None => {
                "/_opendistro/_security/api/tenancy/config".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create Update Tenancy Config API](https://opensearch.org/docs/latest/security/multi-tenancy/dynamic-config/#configuring-multi-tenancy-with-the-rest-api)\n\nCreates or replaces the multi-tenancy configuration. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityCreateUpdateTenancyConfig<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateUpdateTenancyConfigParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateUpdateTenancyConfig<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateUpdateTenancyConfig]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateUpdateTenancyConfig {
            transport,
            parts: SecurityCreateUpdateTenancyConfigParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityCreateUpdateTenancyConfig<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateUpdateTenancyConfig {
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
    #[doc = "Creates an asynchronous call to the Security Create Update Tenancy Config API that can be awaited"]
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
#[doc = "API parts for the Security Create User API"]
pub enum SecurityCreateUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityCreateUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateUserParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/internalusers/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Create User API](https://opensearch.org/docs/latest/security/access-control/api/#create-user)\n\nCreates or replaces the specified user."]
#[derive(Clone, Debug)]
pub struct SecurityCreateUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateUser {
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
    pub fn body<T>(self, body: T) -> SecurityCreateUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateUser {
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
    #[doc = "Creates an asynchronous call to the Security Create User API that can be awaited"]
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
#[doc = "API parts for the Security Create User Legacy API"]
pub enum SecurityCreateUserLegacyParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityCreateUserLegacyParts<'b> {
    #[doc = "Builds a relative URL path to the Security Create User Legacy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityCreateUserLegacyParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Create User Legacy API\n\nCreates or replaces the specified user. Legacy API."]
#[derive(Clone, Debug)]
pub struct SecurityCreateUserLegacy<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityCreateUserLegacyParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityCreateUserLegacy<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityCreateUserLegacy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityCreateUserLegacyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityCreateUserLegacy {
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
    pub fn body<T>(self, body: T) -> SecurityCreateUserLegacy<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityCreateUserLegacy {
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
    #[doc = "Creates an asynchronous call to the Security Create User Legacy API that can be awaited"]
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
#[doc = "API parts for the Security Delete Action Group API"]
pub enum SecurityDeleteActionGroupParts<'b> {
    #[doc = "ActionGroup"]
    ActionGroup(&'b str),
}
impl<'b> SecurityDeleteActionGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Action Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteActionGroupParts::ActionGroup(action_group) => {
                let encoded_action_group: Cow<str> =
                    percent_encode(action_group.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_action_group.len());
                p.push_str("/_opendistro/_security/api/actiongroups/");
                p.push_str(encoded_action_group.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#delete-action-group)\n\nDeletes the specified action group."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteActionGroup<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteActionGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteActionGroup<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteActionGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteActionGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteActionGroup {
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
    #[doc = "Creates an asynchronous call to the Security Delete Action Group API that can be awaited"]
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
#[doc = "API parts for the Security Delete Distinguished Name API"]
pub enum SecurityDeleteDistinguishedNameParts<'b> {
    #[doc = "ClusterName"]
    ClusterName(&'b str),
}
impl<'b> SecurityDeleteDistinguishedNameParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Distinguished Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteDistinguishedNameParts::ClusterName(cluster_name) => {
                let encoded_cluster_name: Cow<str> =
                    percent_encode(cluster_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_cluster_name.len());
                p.push_str("/_opendistro/_security/api/nodesdn/");
                p.push_str(encoded_cluster_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#delete-distinguished-names)\n\nDeletes all distinguished names in the specified cluster or node allowlist. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteDistinguishedName<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteDistinguishedNameParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteDistinguishedName<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteDistinguishedName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteDistinguishedNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteDistinguishedName {
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
    #[doc = "Creates an asynchronous call to the Security Delete Distinguished Name API that can be awaited"]
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
#[doc = "API parts for the Security Delete Role API"]
pub enum SecurityDeleteRoleParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityDeleteRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/roles/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role API](https://opensearch.org/docs/latest/security/access-control/api/#delete-role)\n\nDeletes the specified role."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteRole<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRole {
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
    #[doc = "Creates an asynchronous call to the Security Delete Role API that can be awaited"]
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
#[doc = "API parts for the Security Delete Role Mapping API"]
pub enum SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityDeleteRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteRoleMappingParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/rolesmapping/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#delete-role-mapping)\n\nDeletes the specified role mapping."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteRoleMapping<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteRoleMapping {
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
    #[doc = "Creates an asynchronous call to the Security Delete Role Mapping API that can be awaited"]
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
#[doc = "API parts for the Security Delete Tenant API"]
pub enum SecurityDeleteTenantParts<'b> {
    #[doc = "Tenant"]
    Tenant(&'b str),
}
impl<'b> SecurityDeleteTenantParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete Tenant API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteTenantParts::Tenant(tenant) => {
                let encoded_tenant: Cow<str> =
                    percent_encode(tenant.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_tenant.len());
                p.push_str("/_opendistro/_security/api/tenants/");
                p.push_str(encoded_tenant.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#delete-action-group)\n\nDeletes the specified tenant."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteTenant<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteTenantParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteTenant<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteTenant] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteTenantParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteTenant {
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
    #[doc = "Creates an asynchronous call to the Security Delete Tenant API that can be awaited"]
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
#[doc = "API parts for the Security Delete User API"]
pub enum SecurityDeleteUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDeleteUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteUserParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/internalusers/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Delete User API](https://opensearch.org/docs/latest/security/access-control/api/#delete-user)\n\nDeletes the specified internal user."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteUser<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteUser {
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
    #[doc = "Creates an asynchronous call to the Security Delete User API that can be awaited"]
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
#[doc = "API parts for the Security Delete User Legacy API"]
pub enum SecurityDeleteUserLegacyParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityDeleteUserLegacyParts<'b> {
    #[doc = "Builds a relative URL path to the Security Delete User Legacy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityDeleteUserLegacyParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Delete User Legacy API\n\nDelete the specified user. Legacy API."]
#[derive(Clone, Debug)]
pub struct SecurityDeleteUserLegacy<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityDeleteUserLegacyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityDeleteUserLegacy<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityDeleteUserLegacy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityDeleteUserLegacyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityDeleteUserLegacy {
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
    #[doc = "Creates an asynchronous call to the Security Delete User Legacy API that can be awaited"]
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
#[doc = "API parts for the Security Flush Cache API"]
pub enum SecurityFlushCacheParts {
    #[doc = "No parts"]
    None,
}
impl SecurityFlushCacheParts {
    #[doc = "Builds a relative URL path to the Security Flush Cache API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityFlushCacheParts::None => "/_opendistro/_security/api/cache".into(),
        }
    }
}
#[doc = "Builder for the [Security Flush Cache API](https://opensearch.org/docs/latest/security/access-control/api/#flush-cache)\n\nFlushes the Security plugin's user, authentication, and authorization cache."]
#[derive(Clone, Debug)]
pub struct SecurityFlushCache<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityFlushCacheParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityFlushCache<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityFlushCache]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityFlushCache {
            transport,
            parts: SecurityFlushCacheParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Flush Cache API that can be awaited"]
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
#[doc = "API parts for the Security Generate Obo Token API"]
pub enum SecurityGenerateOboTokenParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGenerateOboTokenParts {
    #[doc = "Builds a relative URL path to the Security Generate Obo Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGenerateOboTokenParts::None => {
                "/_plugins/_security/api/generateonbehalfoftoken".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Generate Obo Token API](https://opensearch.org/docs/latest/security/access-control/authentication-tokens/#api-endpoint)\n\nGenerates a `On-Behalf-Of` token for the current user."]
#[derive(Clone, Debug)]
pub struct SecurityGenerateOboToken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityGenerateOboTokenParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityGenerateOboToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityGenerateOboToken]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGenerateOboToken {
            transport,
            parts: SecurityGenerateOboTokenParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityGenerateOboToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGenerateOboToken {
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
    #[doc = "Creates an asynchronous call to the Security Generate Obo Token API that can be awaited"]
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
#[doc = "API parts for the Security Generate User Token API"]
pub enum SecurityGenerateUserTokenParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityGenerateUserTokenParts<'b> {
    #[doc = "Builds a relative URL path to the Security Generate User Token API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGenerateUserTokenParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(51usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/internalusers/");
                p.push_str(encoded_username.as_ref());
                p.push_str("/authtoken");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Generate User Token API\n\nGenerates an authorization token for the specified user."]
#[derive(Clone, Debug)]
pub struct SecurityGenerateUserToken<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityGenerateUserTokenParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityGenerateUserToken<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityGenerateUserToken] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGenerateUserTokenParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGenerateUserToken {
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
    pub fn body<T>(self, body: T) -> SecurityGenerateUserToken<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityGenerateUserToken {
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
    #[doc = "Creates an asynchronous call to the Security Generate User Token API that can be awaited"]
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
#[doc = "API parts for the Security Get Account Details API"]
pub enum SecurityGetAccountDetailsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetAccountDetailsParts {
    #[doc = "Builds a relative URL path to the Security Get Account Details API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetAccountDetailsParts::None => "/_opendistro/_security/api/account".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Account Details API](https://opensearch.org/docs/latest/security/access-control/api/#get-account-details)\n\nReturns account information for the current user."]
#[derive(Clone, Debug)]
pub struct SecurityGetAccountDetails<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetAccountDetailsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetAccountDetails<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetAccountDetails]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetAccountDetails {
            transport,
            parts: SecurityGetAccountDetailsParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Account Details API that can be awaited"]
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
#[doc = "API parts for the Security Get Action Group API"]
pub enum SecurityGetActionGroupParts<'b> {
    #[doc = "ActionGroup"]
    ActionGroup(&'b str),
}
impl<'b> SecurityGetActionGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Action Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetActionGroupParts::ActionGroup(action_group) => {
                let encoded_action_group: Cow<str> =
                    percent_encode(action_group.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_action_group.len());
                p.push_str("/_opendistro/_security/api/actiongroups/");
                p.push_str(encoded_action_group.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#get-action-group)\n\nRetrieves one action group."]
#[derive(Clone, Debug)]
pub struct SecurityGetActionGroup<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetActionGroupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetActionGroup<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetActionGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetActionGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetActionGroup {
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
    #[doc = "Creates an asynchronous call to the Security Get Action Group API that can be awaited"]
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
#[doc = "API parts for the Security Get Action Groups API"]
pub enum SecurityGetActionGroupsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetActionGroupsParts {
    #[doc = "Builds a relative URL path to the Security Get Action Groups API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetActionGroupsParts::None => "/_plugins/_security/api/actiongroups".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Action Groups API](https://opensearch.org/docs/latest/security/access-control/api/#get-action-groups)\n\nRetrieves all action groups."]
#[derive(Clone, Debug)]
pub struct SecurityGetActionGroups<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetActionGroupsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetActionGroups<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetActionGroups]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetActionGroups {
            transport,
            parts: SecurityGetActionGroupsParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Action Groups API that can be awaited"]
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
#[doc = "API parts for the Security Get All Certificates API"]
pub enum SecurityGetAllCertificatesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetAllCertificatesParts {
    #[doc = "Builds a relative URL path to the Security Get All Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetAllCertificatesParts::None => "/_plugins/_security/api/certificates".into(),
        }
    }
}
#[doc = "Builder for the Security Get All Certificates API\n\nRetrieves the cluster security certificates."]
#[derive(Clone, Debug)]
pub struct SecurityGetAllCertificates<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetAllCertificatesParts,
    cert_type: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> SecurityGetAllCertificates<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetAllCertificates]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetAllCertificates {
            transport,
            parts: SecurityGetAllCertificatesParts::None,
            headers,
            cert_type: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The type of certificates (`HTTP`, `TRANSPORT`, or `ALL`) to retrieve from all nodes."]
    pub fn cert_type(mut self, cert_type: &'b str) -> Self {
        self.cert_type = Some(cert_type);
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
    #[doc = "The maximum duration, in seconds, to spend retrieving certificates from all nodes before a timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get All Certificates API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cert_type: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cert_type: self.cert_type,
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
#[doc = "API parts for the Security Get Allowlist API"]
pub enum SecurityGetAllowlistParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetAllowlistParts {
    #[doc = "Builds a relative URL path to the Security Get Allowlist API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetAllowlistParts::None => "/_opendistro/_security/api/allowlist".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nRetrieves the current list of allowed APIs accessible to a normal user."]
#[derive(Clone, Debug)]
pub struct SecurityGetAllowlist<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetAllowlistParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetAllowlist<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetAllowlist]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetAllowlist {
            transport,
            parts: SecurityGetAllowlistParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Allowlist API that can be awaited"]
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
#[doc = "API parts for the Security Get Audit Configuration API"]
pub enum SecurityGetAuditConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetAuditConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Get Audit Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetAuditConfigurationParts::None => "/_plugins/_security/api/audit".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nRetrieves the audit configuration."]
#[derive(Clone, Debug)]
pub struct SecurityGetAuditConfiguration<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetAuditConfigurationParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetAuditConfiguration<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetAuditConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetAuditConfiguration {
            transport,
            parts: SecurityGetAuditConfigurationParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Audit Configuration API that can be awaited"]
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
#[doc = "API parts for the Security Get Certificates API"]
pub enum SecurityGetCertificatesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetCertificatesParts {
    #[doc = "Builds a relative URL path to the Security Get Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetCertificatesParts::None => "/_opendistro/_security/api/ssl/certs".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#get-certificates)\n\nRetrieves the cluster security certificates."]
#[derive(Clone, Debug)]
pub struct SecurityGetCertificates<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetCertificatesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetCertificates<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetCertificates]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetCertificates {
            transport,
            parts: SecurityGetCertificatesParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Certificates API that can be awaited"]
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
#[doc = "API parts for the Security Get Configuration API"]
pub enum SecurityGetConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Get Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetConfigurationParts::None => {
                "/_opendistro/_security/api/securityconfig".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#get-configuration)\n\nReturns the current Security plugin configuration in a JSON format."]
#[derive(Clone, Debug)]
pub struct SecurityGetConfiguration<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetConfigurationParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetConfiguration<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetConfiguration {
            transport,
            parts: SecurityGetConfigurationParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Configuration API that can be awaited"]
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
#[doc = "API parts for the Security Get Dashboards Info API"]
pub enum SecurityGetDashboardsInfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetDashboardsInfoParts {
    #[doc = "Builds a relative URL path to the Security Get Dashboards Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetDashboardsInfoParts::None => "/_opendistro/_security/kibanainfo".into(),
        }
    }
}
#[doc = "Builder for the Security Get Dashboards Info API\n\nRetrieves the current values for dynamic security settings for OpenSearch Dashboards."]
#[derive(Clone, Debug)]
pub struct SecurityGetDashboardsInfo<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetDashboardsInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetDashboardsInfo<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetDashboardsInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetDashboardsInfo {
            transport,
            parts: SecurityGetDashboardsInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Dashboards Info API that can be awaited"]
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
#[doc = "API parts for the Security Get Distinguished Name API"]
pub enum SecurityGetDistinguishedNameParts<'b> {
    #[doc = "ClusterName"]
    ClusterName(&'b str),
}
impl<'b> SecurityGetDistinguishedNameParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Distinguished Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetDistinguishedNameParts::ClusterName(cluster_name) => {
                let encoded_cluster_name: Cow<str> =
                    percent_encode(cluster_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_cluster_name.len());
                p.push_str("/_opendistro/_security/api/nodesdn/");
                p.push_str(encoded_cluster_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#get-distinguished-names)\n\nRetrieves all node distinguished names. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityGetDistinguishedName<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetDistinguishedNameParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    show_all: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetDistinguishedName<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetDistinguishedName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetDistinguishedNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetDistinguishedName {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            show_all: None,
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
    #[doc = "Whether to include or exclude any static node's DN settings from the final result."]
    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = Some(show_all);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Distinguished Name API that can be awaited"]
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
                show_all: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                show_all: self.show_all,
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
#[doc = "API parts for the Security Get Distinguished Names API"]
pub enum SecurityGetDistinguishedNamesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetDistinguishedNamesParts {
    #[doc = "Builds a relative URL path to the Security Get Distinguished Names API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetDistinguishedNamesParts::None => "/_plugins/_security/api/nodesdn".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Distinguished Names API](https://opensearch.org/docs/latest/security/access-control/api/#get-distinguished-names)\n\nRetrieves all node distinguished names. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityGetDistinguishedNames<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetDistinguishedNamesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    show_all: Option<bool>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetDistinguishedNames<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetDistinguishedNames]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetDistinguishedNames {
            transport,
            parts: SecurityGetDistinguishedNamesParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            show_all: None,
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
    #[doc = "Whether to include or exclude any static node's DN settings from the final result."]
    pub fn show_all(mut self, show_all: bool) -> Self {
        self.show_all = Some(show_all);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Distinguished Names API that can be awaited"]
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
                show_all: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                show_all: self.show_all,
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
#[doc = "API parts for the Security Get Node Certificates API"]
pub enum SecurityGetNodeCertificatesParts<'b> {
    #[doc = "NodeId"]
    NodeId(&'b str),
}
impl<'b> SecurityGetNodeCertificatesParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Node Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetNodeCertificatesParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(37usize + encoded_node_id.len());
                p.push_str("/_plugins/_security/api/certificates/");
                p.push_str(encoded_node_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Get Node Certificates API\n\nRetrieves the specified node's security certificates."]
#[derive(Clone, Debug)]
pub struct SecurityGetNodeCertificates<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetNodeCertificatesParts<'b>,
    cert_type: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> SecurityGetNodeCertificates<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetNodeCertificates] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetNodeCertificatesParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetNodeCertificates {
            transport,
            parts,
            headers,
            cert_type: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            timeout: None,
        }
    }
    #[doc = "The type of certificates (`HTTP`, `TRANSPORT`, or `ALL`) to retrieve from a node."]
    pub fn cert_type(mut self, cert_type: &'b str) -> Self {
        self.cert_type = Some(cert_type);
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
    #[doc = "The maximum duration, in seconds, to spend retrieving certificates from all nodes before a timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Node Certificates API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                cert_type: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                timeout: Option<&'b str>,
            }
            let query_params = QueryParams {
                cert_type: self.cert_type,
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
#[doc = "API parts for the Security Get Permissions Info API"]
pub enum SecurityGetPermissionsInfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetPermissionsInfoParts {
    #[doc = "Builds a relative URL path to the Security Get Permissions Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetPermissionsInfoParts::None => {
                "/_opendistro/_security/api/permissionsinfo".into()
            }
        }
    }
}
#[doc = "Builder for the Security Get Permissions Info API\n\nRetrieves the evaluated REST API permissions for the currently logged in user."]
#[derive(Clone, Debug)]
pub struct SecurityGetPermissionsInfo<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetPermissionsInfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetPermissionsInfo<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetPermissionsInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetPermissionsInfo {
            transport,
            parts: SecurityGetPermissionsInfoParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Permissions Info API that can be awaited"]
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
#[doc = "API parts for the Security Get Role API"]
pub enum SecurityGetRoleParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityGetRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/roles/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Role API](https://opensearch.org/docs/latest/security/access-control/api/#get-role)\n\nRetrieves one role."]
#[derive(Clone, Debug)]
pub struct SecurityGetRole<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRoleParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRole<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRole {
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
    #[doc = "Creates an asynchronous call to the Security Get Role API that can be awaited"]
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
#[doc = "API parts for the Security Get Role Mapping API"]
pub enum SecurityGetRoleMappingParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityGetRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleMappingParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/rolesmapping/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#get-role-mapping)\n\nRetrieves the specified role mapping."]
#[derive(Clone, Debug)]
pub struct SecurityGetRoleMapping<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRoleMappingParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRoleMapping<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRoleMapping {
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
    #[doc = "Creates an asynchronous call to the Security Get Role Mapping API that can be awaited"]
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
#[doc = "API parts for the Security Get Role Mappings API"]
pub enum SecurityGetRoleMappingsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetRoleMappingsParts {
    #[doc = "Builds a relative URL path to the Security Get Role Mappings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRoleMappingsParts::None => "/_plugins/_security/api/rolesmapping".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Role Mappings API](https://opensearch.org/docs/latest/security/access-control/api/#get-role-mappings)\n\nRetrieves all role mappings."]
#[derive(Clone, Debug)]
pub struct SecurityGetRoleMappings<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRoleMappingsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRoleMappings<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRoleMappings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRoleMappings {
            transport,
            parts: SecurityGetRoleMappingsParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Role Mappings API that can be awaited"]
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
#[doc = "API parts for the Security Get Roles API"]
pub enum SecurityGetRolesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetRolesParts {
    #[doc = "Builds a relative URL path to the Security Get Roles API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetRolesParts::None => "/_plugins/_security/api/roles".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Roles API](https://opensearch.org/docs/latest/security/access-control/api/#get-roles)\n\nRetrieves all roles."]
#[derive(Clone, Debug)]
pub struct SecurityGetRoles<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetRolesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetRoles<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetRoles]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetRoles {
            transport,
            parts: SecurityGetRolesParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Roles API that can be awaited"]
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
#[doc = "API parts for the Security Get Sslinfo API"]
pub enum SecurityGetSslinfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetSslinfoParts {
    #[doc = "Builds a relative URL path to the Security Get Sslinfo API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetSslinfoParts::None => "/_opendistro/_security/sslinfo".into(),
        }
    }
}
#[doc = "Builder for the Security Get Sslinfo API\n\nRetrieves information about the SSL configuration."]
#[derive(Clone, Debug)]
pub struct SecurityGetSslinfo<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetSslinfoParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    show_dn: Option<&'b str>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetSslinfo<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetSslinfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetSslinfo {
            transport,
            parts: SecurityGetSslinfoParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            show_dn: None,
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
    #[doc = "Whether to include all domain names in the response."]
    pub fn show_dn(mut self, show_dn: &'b str) -> Self {
        self.show_dn = Some(show_dn);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Get Sslinfo API that can be awaited"]
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
                show_dn: Option<&'b str>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                show_dn: self.show_dn,
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
#[doc = "API parts for the Security Get Tenancy Config API"]
pub enum SecurityGetTenancyConfigParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetTenancyConfigParts {
    #[doc = "Builds a relative URL path to the Security Get Tenancy Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetTenancyConfigParts::None => {
                "/_opendistro/_security/api/tenancy/config".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Tenancy Config API](https://opensearch.org/docs/latest/security/multi-tenancy/dynamic-config/#configuring-multi-tenancy-with-the-rest-api)\n\nRetrieves the multi-tenancy configuration. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityGetTenancyConfig<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetTenancyConfigParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetTenancyConfig<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetTenancyConfig]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetTenancyConfig {
            transport,
            parts: SecurityGetTenancyConfigParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Tenancy Config API that can be awaited"]
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
#[doc = "API parts for the Security Get Tenant API"]
pub enum SecurityGetTenantParts<'b> {
    #[doc = "Tenant"]
    Tenant(&'b str),
}
impl<'b> SecurityGetTenantParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get Tenant API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetTenantParts::Tenant(tenant) => {
                let encoded_tenant: Cow<str> =
                    percent_encode(tenant.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_tenant.len());
                p.push_str("/_opendistro/_security/api/tenants/");
                p.push_str(encoded_tenant.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#get-tenant)\n\nRetrieves the specified tenant."]
#[derive(Clone, Debug)]
pub struct SecurityGetTenant<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetTenantParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetTenant<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetTenant] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetTenantParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetTenant {
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
    #[doc = "Creates an asynchronous call to the Security Get Tenant API that can be awaited"]
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
#[doc = "API parts for the Security Get Tenants API"]
pub enum SecurityGetTenantsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetTenantsParts {
    #[doc = "Builds a relative URL path to the Security Get Tenants API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetTenantsParts::None => "/_opendistro/_security/api/tenants".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Tenants API](https://opensearch.org/docs/latest/security/access-control/api/#get-tenants)\n\nRetrieves all tenants."]
#[derive(Clone, Debug)]
pub struct SecurityGetTenants<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetTenantsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetTenants<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetTenants]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetTenants {
            transport,
            parts: SecurityGetTenantsParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Tenants API that can be awaited"]
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
#[doc = "API parts for the Security Get User API"]
pub enum SecurityGetUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityGetUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/internalusers/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Get User API](https://opensearch.org/docs/latest/security/access-control/api/#get-user)\n\nRetrieve information about the specified internal user."]
#[derive(Clone, Debug)]
pub struct SecurityGetUser<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUserParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUser<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUser {
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
    #[doc = "Creates an asynchronous call to the Security Get User API that can be awaited"]
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
#[doc = "API parts for the Security Get User Legacy API"]
pub enum SecurityGetUserLegacyParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityGetUserLegacyParts<'b> {
    #[doc = "Builds a relative URL path to the Security Get User Legacy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUserLegacyParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(32usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/user/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Get User Legacy API\n\nRetrieve one user. Legacy API."]
#[derive(Clone, Debug)]
pub struct SecurityGetUserLegacy<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUserLegacyParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUserLegacy<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUserLegacy] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityGetUserLegacyParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUserLegacy {
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
    #[doc = "Creates an asynchronous call to the Security Get User Legacy API that can be awaited"]
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
#[doc = "API parts for the Security Get Users API"]
pub enum SecurityGetUsersParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetUsersParts {
    #[doc = "Builds a relative URL path to the Security Get Users API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUsersParts::None => "/_plugins/_security/api/internalusers".into(),
        }
    }
}
#[doc = "Builder for the [Security Get Users API](https://opensearch.org/docs/latest/security/access-control/api/#get-users)\n\nRetrieve all internal users."]
#[derive(Clone, Debug)]
pub struct SecurityGetUsers<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUsersParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUsers<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUsers]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUsers {
            transport,
            parts: SecurityGetUsersParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Users API that can be awaited"]
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
#[doc = "API parts for the Security Get Users Legacy API"]
pub enum SecurityGetUsersLegacyParts {
    #[doc = "No parts"]
    None,
}
impl SecurityGetUsersLegacyParts {
    #[doc = "Builds a relative URL path to the Security Get Users Legacy API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityGetUsersLegacyParts::None => "/_opendistro/_security/api/user".into(),
        }
    }
}
#[doc = "Builder for the Security Get Users Legacy API\n\nRetrieve all internal users. Legacy API."]
#[derive(Clone, Debug)]
pub struct SecurityGetUsersLegacy<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityGetUsersLegacyParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityGetUsersLegacy<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityGetUsersLegacy]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityGetUsersLegacy {
            transport,
            parts: SecurityGetUsersLegacyParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Get Users Legacy API that can be awaited"]
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
#[doc = "API parts for the Security Health API"]
pub enum SecurityHealthParts {
    #[doc = "No parts"]
    None,
}
impl SecurityHealthParts {
    #[doc = "Builds a relative URL path to the Security Health API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityHealthParts::None => "/_opendistro/_security/health".into(),
        }
    }
}
#[doc = "Builder for the [Security Health API](https://opensearch.org/docs/latest/security/access-control/api/#health-check)\n\nChecks to see if the Security plugin is running."]
#[derive(Clone, Debug)]
pub struct SecurityHealth<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityHealthParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    mode: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityHealth<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityHealth]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityHealth {
            transport,
            parts: SecurityHealthParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            mode: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> SecurityHealth<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityHealth {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            mode: self.mode,
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
    #[doc = "A flag that determines whether to consider the security status before returning a response for a health query response. For example, `strict` mode indicates service should check the Security plugin status."]
    pub fn mode(mut self, mode: &'b str) -> Self {
        self.mode = Some(mode);
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
    #[doc = "Creates an asynchronous call to the Security Health API that can be awaited"]
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
                mode: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                mode: self.mode,
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
#[doc = "API parts for the Security Migrate API"]
pub enum SecurityMigrateParts {
    #[doc = "No parts"]
    None,
}
impl SecurityMigrateParts {
    #[doc = "Builds a relative URL path to the Security Migrate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityMigrateParts::None => "/_opendistro/_security/api/migrate".into(),
        }
    }
}
#[doc = "Builder for the Security Migrate API\n\nMigrates the security configuration from v6 to v7."]
#[derive(Clone, Debug)]
pub struct SecurityMigrate<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityMigrateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityMigrate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityMigrate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityMigrate {
            transport,
            parts: SecurityMigrateParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityMigrate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityMigrate {
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
    #[doc = "Creates an asynchronous call to the Security Migrate API that can be awaited"]
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
#[doc = "API parts for the Security Patch Action Group API"]
pub enum SecurityPatchActionGroupParts<'b> {
    #[doc = "ActionGroup"]
    ActionGroup(&'b str),
}
impl<'b> SecurityPatchActionGroupParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch Action Group API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchActionGroupParts::ActionGroup(action_group) => {
                let encoded_action_group: Cow<str> =
                    percent_encode(action_group.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_action_group.len());
                p.push_str("/_opendistro/_security/api/actiongroups/");
                p.push_str(encoded_action_group.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#patch-action-group)\n\nUpdates the individual attributes of an action group."]
#[derive(Clone, Debug)]
pub struct SecurityPatchActionGroup<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchActionGroupParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchActionGroup<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchActionGroup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchActionGroupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchActionGroup {
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
    pub fn body<T>(self, body: T) -> SecurityPatchActionGroup<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchActionGroup {
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
    #[doc = "Creates an asynchronous call to the Security Patch Action Group API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Action Groups API"]
pub enum SecurityPatchActionGroupsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchActionGroupsParts {
    #[doc = "Builds a relative URL path to the Security Patch Action Groups API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchActionGroupsParts::None => "/_plugins/_security/api/actiongroups".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Action Groups API](https://opensearch.org/docs/latest/security/access-control/api/#patch-action-groups)\n\nCreates, updates, or deletes multiple action groups in a single request."]
#[derive(Clone, Debug)]
pub struct SecurityPatchActionGroups<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchActionGroupsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchActionGroups<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchActionGroups]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchActionGroups {
            transport,
            parts: SecurityPatchActionGroupsParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchActionGroups<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchActionGroups {
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
    #[doc = "Creates an asynchronous call to the Security Patch Action Groups API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Allowlist API"]
pub enum SecurityPatchAllowlistParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchAllowlistParts {
    #[doc = "Builds a relative URL path to the Security Patch Allowlist API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchAllowlistParts::None => "/_opendistro/_security/api/allowlist".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nUpdates the current list of APIs accessible for users on the allow list."]
#[derive(Clone, Debug)]
pub struct SecurityPatchAllowlist<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchAllowlistParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchAllowlist<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchAllowlist]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchAllowlist {
            transport,
            parts: SecurityPatchAllowlistParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchAllowlist<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchAllowlist {
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
    #[doc = "Creates an asynchronous call to the Security Patch Allowlist API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Audit Configuration API"]
pub enum SecurityPatchAuditConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchAuditConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Patch Audit Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchAuditConfigurationParts::None => "/_plugins/_security/api/audit".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nUpdates the specified fields in the audit configuration."]
#[derive(Clone, Debug)]
pub struct SecurityPatchAuditConfiguration<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchAuditConfigurationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchAuditConfiguration<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchAuditConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchAuditConfiguration {
            transport,
            parts: SecurityPatchAuditConfigurationParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchAuditConfiguration<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchAuditConfiguration {
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
    #[doc = "Creates an asynchronous call to the Security Patch Audit Configuration API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Configuration API"]
pub enum SecurityPatchConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Patch Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchConfigurationParts::None => {
                "/_opendistro/_security/api/securityconfig".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#patch-configuration)\n\nUpdates the existing security configuration using the REST API. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityPatchConfiguration<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchConfigurationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchConfiguration<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchConfiguration {
            transport,
            parts: SecurityPatchConfigurationParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchConfiguration<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchConfiguration {
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
    #[doc = "Creates an asynchronous call to the Security Patch Configuration API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Distinguished Name API"]
pub enum SecurityPatchDistinguishedNameParts<'b> {
    #[doc = "ClusterName"]
    ClusterName(&'b str),
}
impl<'b> SecurityPatchDistinguishedNameParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch Distinguished Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchDistinguishedNameParts::ClusterName(cluster_name) => {
                let encoded_cluster_name: Cow<str> =
                    percent_encode(cluster_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_cluster_name.len());
                p.push_str("/_opendistro/_security/api/nodesdn/");
                p.push_str(encoded_cluster_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Security Patch Distinguished Name API\n\nUpdates the distinguished cluster name for the specified cluster. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityPatchDistinguishedName<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchDistinguishedNameParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchDistinguishedName<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchDistinguishedName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchDistinguishedNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchDistinguishedName {
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
    pub fn body<T>(self, body: T) -> SecurityPatchDistinguishedName<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchDistinguishedName {
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
    #[doc = "Creates an asynchronous call to the Security Patch Distinguished Name API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Distinguished Names API"]
pub enum SecurityPatchDistinguishedNamesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchDistinguishedNamesParts {
    #[doc = "Builds a relative URL path to the Security Patch Distinguished Names API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchDistinguishedNamesParts::None => "/_plugins/_security/api/nodesdn".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Distinguished Names API](https://opensearch.org/docs/latest/security/access-control/api/#update-all-distinguished-names)\n\nBulk updates specified node distinguished names. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityPatchDistinguishedNames<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchDistinguishedNamesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchDistinguishedNames<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchDistinguishedNames]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchDistinguishedNames {
            transport,
            parts: SecurityPatchDistinguishedNamesParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchDistinguishedNames<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchDistinguishedNames {
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
    #[doc = "Creates an asynchronous call to the Security Patch Distinguished Names API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Role API"]
pub enum SecurityPatchRoleParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityPatchRoleParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch Role API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchRoleParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/roles/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch Role API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role)\n\nUpdates the individual attributes of a role."]
#[derive(Clone, Debug)]
pub struct SecurityPatchRole<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchRoleParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchRole<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchRole] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchRoleParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchRole {
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
    pub fn body<T>(self, body: T) -> SecurityPatchRole<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchRole {
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
    #[doc = "Creates an asynchronous call to the Security Patch Role API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Role Mapping API"]
pub enum SecurityPatchRoleMappingParts<'b> {
    #[doc = "Role"]
    Role(&'b str),
}
impl<'b> SecurityPatchRoleMappingParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch Role Mapping API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchRoleMappingParts::Role(role) => {
                let encoded_role: Cow<str> = percent_encode(role.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_role.len());
                p.push_str("/_opendistro/_security/api/rolesmapping/");
                p.push_str(encoded_role.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role-mapping)\n\nUpdates the individual attributes of a role mapping."]
#[derive(Clone, Debug)]
pub struct SecurityPatchRoleMapping<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchRoleMappingParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchRoleMapping<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchRoleMapping] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchRoleMappingParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchRoleMapping {
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
    pub fn body<T>(self, body: T) -> SecurityPatchRoleMapping<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchRoleMapping {
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
    #[doc = "Creates an asynchronous call to the Security Patch Role Mapping API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Role Mappings API"]
pub enum SecurityPatchRoleMappingsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchRoleMappingsParts {
    #[doc = "Builds a relative URL path to the Security Patch Role Mappings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchRoleMappingsParts::None => "/_plugins/_security/api/rolesmapping".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Role Mappings API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role-mappings)\n\nCreates or updates multiple role mappings in a single request."]
#[derive(Clone, Debug)]
pub struct SecurityPatchRoleMappings<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchRoleMappingsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchRoleMappings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchRoleMappings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchRoleMappings {
            transport,
            parts: SecurityPatchRoleMappingsParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchRoleMappings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchRoleMappings {
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
    #[doc = "Creates an asynchronous call to the Security Patch Role Mappings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Roles API"]
pub enum SecurityPatchRolesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchRolesParts {
    #[doc = "Builds a relative URL path to the Security Patch Roles API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchRolesParts::None => "/_plugins/_security/api/roles".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Roles API](https://opensearch.org/docs/latest/security/access-control/api/#patch-roles)\n\nCreates, updates, or deletes multiple roles in a single call."]
#[derive(Clone, Debug)]
pub struct SecurityPatchRoles<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchRolesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchRoles<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchRoles]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchRoles {
            transport,
            parts: SecurityPatchRolesParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchRoles<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchRoles {
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
    #[doc = "Creates an asynchronous call to the Security Patch Roles API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Tenant API"]
pub enum SecurityPatchTenantParts<'b> {
    #[doc = "Tenant"]
    Tenant(&'b str),
}
impl<'b> SecurityPatchTenantParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch Tenant API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchTenantParts::Tenant(tenant) => {
                let encoded_tenant: Cow<str> =
                    percent_encode(tenant.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_tenant.len());
                p.push_str("/_opendistro/_security/api/tenants/");
                p.push_str(encoded_tenant.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#patch-tenant)\n\nAdds, deletes, or modifies a single tenant."]
#[derive(Clone, Debug)]
pub struct SecurityPatchTenant<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchTenantParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchTenant<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchTenant] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchTenantParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchTenant {
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
    pub fn body<T>(self, body: T) -> SecurityPatchTenant<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchTenant {
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
    #[doc = "Creates an asynchronous call to the Security Patch Tenant API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Tenants API"]
pub enum SecurityPatchTenantsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchTenantsParts {
    #[doc = "Builds a relative URL path to the Security Patch Tenants API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchTenantsParts::None => "/_opendistro/_security/api/tenants".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Tenants API](https://opensearch.org/docs/latest/security/access-control/api/#patch-tenants)\n\nAdds, deletes, or modifies multiple tenants in a single request."]
#[derive(Clone, Debug)]
pub struct SecurityPatchTenants<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchTenantsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchTenants<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchTenants]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchTenants {
            transport,
            parts: SecurityPatchTenantsParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchTenants<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchTenants {
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
    #[doc = "Creates an asynchronous call to the Security Patch Tenants API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch User API"]
pub enum SecurityPatchUserParts<'b> {
    #[doc = "Username"]
    Username(&'b str),
}
impl<'b> SecurityPatchUserParts<'b> {
    #[doc = "Builds a relative URL path to the Security Patch User API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchUserParts::Username(username) => {
                let encoded_username: Cow<str> =
                    percent_encode(username.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(41usize + encoded_username.len());
                p.push_str("/_opendistro/_security/api/internalusers/");
                p.push_str(encoded_username.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Patch User API](https://opensearch.org/docs/latest/security/access-control/api/#patch-user)\n\nUpdates individual attributes for an internal user."]
#[derive(Clone, Debug)]
pub struct SecurityPatchUser<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchUserParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchUser<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchUser] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityPatchUserParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchUser {
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
    pub fn body<T>(self, body: T) -> SecurityPatchUser<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchUser {
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
    #[doc = "Creates an asynchronous call to the Security Patch User API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Patch Users API"]
pub enum SecurityPatchUsersParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPatchUsersParts {
    #[doc = "Builds a relative URL path to the Security Patch Users API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPatchUsersParts::None => "/_plugins/_security/api/internalusers".into(),
        }
    }
}
#[doc = "Builder for the [Security Patch Users API](https://opensearch.org/docs/latest/security/access-control/api/#patch-users)\n\nCreates, updates, or deletes multiple internal users in a single request."]
#[derive(Clone, Debug)]
pub struct SecurityPatchUsers<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPatchUsersParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPatchUsers<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPatchUsers]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPatchUsers {
            transport,
            parts: SecurityPatchUsersParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPatchUsers<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPatchUsers {
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
    #[doc = "Creates an asynchronous call to the Security Patch Users API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Patch;
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
#[doc = "API parts for the Security Post Dashboards Info API"]
pub enum SecurityPostDashboardsInfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityPostDashboardsInfoParts {
    #[doc = "Builds a relative URL path to the Security Post Dashboards Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityPostDashboardsInfoParts::None => "/_opendistro/_security/kibanainfo".into(),
        }
    }
}
#[doc = "Builder for the Security Post Dashboards Info API\n\nRetrieves the current values for dynamic security settings for OpenSearch Dashboards."]
#[derive(Clone, Debug)]
pub struct SecurityPostDashboardsInfo<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityPostDashboardsInfoParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityPostDashboardsInfo<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityPostDashboardsInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityPostDashboardsInfo {
            transport,
            parts: SecurityPostDashboardsInfoParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityPostDashboardsInfo<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityPostDashboardsInfo {
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
    #[doc = "Creates an asynchronous call to the Security Post Dashboards Info API that can be awaited"]
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
#[doc = "API parts for the Security Reload Http Certificates API"]
pub enum SecurityReloadHttpCertificatesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityReloadHttpCertificatesParts {
    #[doc = "Builds a relative URL path to the Security Reload Http Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityReloadHttpCertificatesParts::None => {
                "/_opendistro/_security/api/ssl/http/reloadcerts".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Reload Http Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#reload-http-certificates)\n\nReloads the HTTP communication certificates."]
#[derive(Clone, Debug)]
pub struct SecurityReloadHttpCertificates<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityReloadHttpCertificatesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityReloadHttpCertificates<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityReloadHttpCertificates]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityReloadHttpCertificates {
            transport,
            parts: SecurityReloadHttpCertificatesParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityReloadHttpCertificates<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityReloadHttpCertificates {
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
    #[doc = "Creates an asynchronous call to the Security Reload Http Certificates API that can be awaited"]
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
#[doc = "API parts for the Security Reload Transport Certificates API"]
pub enum SecurityReloadTransportCertificatesParts {
    #[doc = "No parts"]
    None,
}
impl SecurityReloadTransportCertificatesParts {
    #[doc = "Builds a relative URL path to the Security Reload Transport Certificates API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityReloadTransportCertificatesParts::None => {
                "/_opendistro/_security/api/ssl/transport/reloadcerts".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Reload Transport Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#reload-transport-certificates)\n\nReloads the transport communication certificates."]
#[derive(Clone, Debug)]
pub struct SecurityReloadTransportCertificates<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityReloadTransportCertificatesParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityReloadTransportCertificates<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityReloadTransportCertificates]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityReloadTransportCertificates {
            transport,
            parts: SecurityReloadTransportCertificatesParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityReloadTransportCertificates<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityReloadTransportCertificates {
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
    #[doc = "Creates an asynchronous call to the Security Reload Transport Certificates API that can be awaited"]
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
#[doc = "API parts for the Security Tenant Info API"]
pub enum SecurityTenantInfoParts {
    #[doc = "No parts"]
    None,
}
impl SecurityTenantInfoParts {
    #[doc = "Builds a relative URL path to the Security Tenant Info API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityTenantInfoParts::None => "/_opendistro/_security/tenantinfo".into(),
        }
    }
}
#[doc = "Builder for the Security Tenant Info API\n\nRetrieves the names of current tenants. Requires super admin or `kibanaserver` permissions."]
#[derive(Clone, Debug)]
pub struct SecurityTenantInfo<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityTenantInfoParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityTenantInfo<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityTenantInfo]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityTenantInfo {
            transport,
            parts: SecurityTenantInfoParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityTenantInfo<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityTenantInfo {
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
    #[doc = "Creates an asynchronous call to the Security Tenant Info API that can be awaited"]
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
#[doc = "API parts for the Security Update Audit Configuration API"]
pub enum SecurityUpdateAuditConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityUpdateAuditConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Update Audit Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateAuditConfigurationParts::None => {
                "/_opendistro/_security/api/audit/config".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nUpdates the audit configuration."]
#[derive(Clone, Debug)]
pub struct SecurityUpdateAuditConfiguration<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateAuditConfigurationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateAuditConfiguration<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateAuditConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateAuditConfiguration {
            transport,
            parts: SecurityUpdateAuditConfigurationParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityUpdateAuditConfiguration<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateAuditConfiguration {
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
    #[doc = "Creates an asynchronous call to the Security Update Audit Configuration API that can be awaited"]
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
#[doc = "API parts for the Security Update Configuration API"]
pub enum SecurityUpdateConfigurationParts {
    #[doc = "No parts"]
    None,
}
impl SecurityUpdateConfigurationParts {
    #[doc = "Builds a relative URL path to the Security Update Configuration API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateConfigurationParts::None => {
                "/_opendistro/_security/api/securityconfig/config".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#update-configuration)\n\nUpdates the settings for an existing security configuration. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityUpdateConfiguration<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateConfigurationParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateConfiguration<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateConfiguration]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateConfiguration {
            transport,
            parts: SecurityUpdateConfigurationParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityUpdateConfiguration<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateConfiguration {
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
    #[doc = "Creates an asynchronous call to the Security Update Configuration API that can be awaited"]
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
#[doc = "API parts for the Security Update Distinguished Name API"]
pub enum SecurityUpdateDistinguishedNameParts<'b> {
    #[doc = "ClusterName"]
    ClusterName(&'b str),
}
impl<'b> SecurityUpdateDistinguishedNameParts<'b> {
    #[doc = "Builds a relative URL path to the Security Update Distinguished Name API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityUpdateDistinguishedNameParts::ClusterName(cluster_name) => {
                let encoded_cluster_name: Cow<str> =
                    percent_encode(cluster_name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_cluster_name.len());
                p.push_str("/_opendistro/_security/api/nodesdn/");
                p.push_str(encoded_cluster_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Security Update Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#update-distinguished-names)\n\nAdds or updates the specified distinguished names in the cluster or node allowlist. Requires super admin or REST API permissions."]
#[derive(Clone, Debug)]
pub struct SecurityUpdateDistinguishedName<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityUpdateDistinguishedNameParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityUpdateDistinguishedName<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityUpdateDistinguishedName] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SecurityUpdateDistinguishedNameParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SecurityUpdateDistinguishedName {
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
    pub fn body<T>(self, body: T) -> SecurityUpdateDistinguishedName<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityUpdateDistinguishedName {
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
    #[doc = "Creates an asynchronous call to the Security Update Distinguished Name API that can be awaited"]
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
#[doc = "API parts for the Security Validate API"]
pub enum SecurityValidateParts {
    #[doc = "No parts"]
    None,
}
impl SecurityValidateParts {
    #[doc = "Builds a relative URL path to the Security Validate API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityValidateParts::None => "/_opendistro/_security/api/validate".into(),
        }
    }
}
#[doc = "Builder for the Security Validate API\n\nChecks whether the v6 security configuration is valid and ready to be migrated to v7."]
#[derive(Clone, Debug)]
pub struct SecurityValidate<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityValidateParts,
    accept_invalid: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityValidate<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityValidate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityValidate {
            transport,
            parts: SecurityValidateParts::None,
            headers,
            accept_invalid: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether an invalid v6 configuration should be allowed."]
    pub fn accept_invalid(mut self, accept_invalid: bool) -> Self {
        self.accept_invalid = Some(accept_invalid);
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
    #[doc = "Creates an asynchronous call to the Security Validate API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                accept_invalid: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                accept_invalid: self.accept_invalid,
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
#[doc = "API parts for the Security Who Am I API"]
pub enum SecurityWhoAmIParts {
    #[doc = "No parts"]
    None,
}
impl SecurityWhoAmIParts {
    #[doc = "Builds a relative URL path to the Security Who Am I API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityWhoAmIParts::None => "/_plugins/_security/whoami".into(),
        }
    }
}
#[doc = "Builder for the Security Who Am I API\n\nGets the identity information for the user currently logged in."]
#[derive(Clone, Debug)]
pub struct SecurityWhoAmI<'a, 'b, B> {
    transport: &'a Transport,
    parts: SecurityWhoAmIParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SecurityWhoAmI<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SecurityWhoAmI]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityWhoAmI {
            transport,
            parts: SecurityWhoAmIParts::None,
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
    pub fn body<T>(self, body: T) -> SecurityWhoAmI<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SecurityWhoAmI {
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
    #[doc = "Creates an asynchronous call to the Security Who Am I API that can be awaited"]
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
#[doc = "API parts for the Security Who Am I Protected API"]
pub enum SecurityWhoAmIProtectedParts {
    #[doc = "No parts"]
    None,
}
impl SecurityWhoAmIProtectedParts {
    #[doc = "Builds a relative URL path to the Security Who Am I Protected API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityWhoAmIProtectedParts::None => "/_plugins/_security/whoamiprotected".into(),
        }
    }
}
#[doc = "Builder for the Security Who Am I Protected API\n\nGets the identity information for the user currently logged in. To use this operation, you must have access to this endpoint when authorization at REST layer is enabled."]
#[derive(Clone, Debug)]
pub struct SecurityWhoAmIProtected<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityWhoAmIProtectedParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SecurityWhoAmIProtected<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityWhoAmIProtected]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityWhoAmIProtected {
            transport,
            parts: SecurityWhoAmIProtectedParts::None,
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
    #[doc = "Creates an asynchronous call to the Security Who Am I Protected API that can be awaited"]
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
#[doc = "Namespace client for Security APIs"]
pub struct Security<'a> {
    transport: &'a Transport,
}
impl<'a> Security<'a> {
    #[doc = "Creates a new instance of [Security]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Security Authinfo API\n\nReturns or updates authentication information for the currently authenticated user."]
    pub fn authinfo<'b>(&'a self) -> SecurityAuthinfo<'a, 'b, ()> {
        SecurityAuthinfo::new(self.transport())
    }
    #[doc = "Security Authtoken API\n\nReturns the authorization token for the current user."]
    pub fn authtoken<'b>(&'a self) -> SecurityAuthtoken<'a, 'b, ()> {
        SecurityAuthtoken::new(self.transport())
    }
    #[doc = "[Security Change Password API](https://opensearch.org/docs/latest/security/access-control/api/#change-password)\n\nChanges the password for the current user."]
    pub fn change_password<'b>(&'a self) -> SecurityChangePassword<'a, 'b, ()> {
        SecurityChangePassword::new(self.transport())
    }
    #[doc = "[Security Config Upgrade Check API](https://opensearch.org/docs/latest/security/access-control/api/#configuration-upgrade-check)\n\nChecks whether or not an upgrade can be performed and which security resources can be updated."]
    pub fn config_upgrade_check<'b>(&'a self) -> SecurityConfigUpgradeCheck<'a, 'b> {
        SecurityConfigUpgradeCheck::new(self.transport())
    }
    #[doc = "[Security Config Upgrade Perform API](https://opensearch.org/docs/latest/security/access-control/api/#configuration-upgrade)\n\nAssists the cluster operator with upgrading missing default values and stale default definitions."]
    pub fn config_upgrade_perform<'b>(&'a self) -> SecurityConfigUpgradePerform<'a, 'b, ()> {
        SecurityConfigUpgradePerform::new(self.transport())
    }
    #[doc = "[Security Create Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#create-action-group)\n\nCreates or replaces the specified action group."]
    pub fn create_action_group<'b>(
        &'a self,
        parts: SecurityCreateActionGroupParts<'b>,
    ) -> SecurityCreateActionGroup<'a, 'b, ()> {
        SecurityCreateActionGroup::new(self.transport(), parts)
    }
    #[doc = "[Security Create Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nCreates or replaces APIs permitted for users on the allow list. Requires a super admin certificate or REST API permissions."]
    pub fn create_allowlist<'b>(&'a self) -> SecurityCreateAllowlist<'a, 'b, ()> {
        SecurityCreateAllowlist::new(self.transport())
    }
    #[doc = "[Security Create Role API](https://opensearch.org/docs/latest/security/access-control/api/#create-role)\n\nCreates or replaces the specified role."]
    pub fn create_role<'b>(
        &'a self,
        parts: SecurityCreateRoleParts<'b>,
    ) -> SecurityCreateRole<'a, 'b, ()> {
        SecurityCreateRole::new(self.transport(), parts)
    }
    #[doc = "[Security Create Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#create-role-mapping)\n\nCreates or replaces the specified role mapping."]
    pub fn create_role_mapping<'b>(
        &'a self,
        parts: SecurityCreateRoleMappingParts<'b>,
    ) -> SecurityCreateRoleMapping<'a, 'b, ()> {
        SecurityCreateRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Create Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#create-tenant)\n\nCreates or replaces the specified tenant."]
    pub fn create_tenant<'b>(
        &'a self,
        parts: SecurityCreateTenantParts<'b>,
    ) -> SecurityCreateTenant<'a, 'b, ()> {
        SecurityCreateTenant::new(self.transport(), parts)
    }
    #[doc = "[Security Create Update Tenancy Config API](https://opensearch.org/docs/latest/security/multi-tenancy/dynamic-config/#configuring-multi-tenancy-with-the-rest-api)\n\nCreates or replaces the multi-tenancy configuration. Requires super admin or REST API permissions."]
    pub fn create_update_tenancy_config<'b>(
        &'a self,
    ) -> SecurityCreateUpdateTenancyConfig<'a, 'b, ()> {
        SecurityCreateUpdateTenancyConfig::new(self.transport())
    }
    #[doc = "[Security Create User API](https://opensearch.org/docs/latest/security/access-control/api/#create-user)\n\nCreates or replaces the specified user."]
    pub fn create_user<'b>(
        &'a self,
        parts: SecurityCreateUserParts<'b>,
    ) -> SecurityCreateUser<'a, 'b, ()> {
        SecurityCreateUser::new(self.transport(), parts)
    }
    #[doc = "Security Create User Legacy API\n\nCreates or replaces the specified user. Legacy API."]
    pub fn create_user_legacy<'b>(
        &'a self,
        parts: SecurityCreateUserLegacyParts<'b>,
    ) -> SecurityCreateUserLegacy<'a, 'b, ()> {
        SecurityCreateUserLegacy::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#delete-action-group)\n\nDeletes the specified action group."]
    pub fn delete_action_group<'b>(
        &'a self,
        parts: SecurityDeleteActionGroupParts<'b>,
    ) -> SecurityDeleteActionGroup<'a, 'b> {
        SecurityDeleteActionGroup::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#delete-distinguished-names)\n\nDeletes all distinguished names in the specified cluster or node allowlist. Requires super admin or REST API permissions."]
    pub fn delete_distinguished_name<'b>(
        &'a self,
        parts: SecurityDeleteDistinguishedNameParts<'b>,
    ) -> SecurityDeleteDistinguishedName<'a, 'b> {
        SecurityDeleteDistinguishedName::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role API](https://opensearch.org/docs/latest/security/access-control/api/#delete-role)\n\nDeletes the specified role."]
    pub fn delete_role<'b>(
        &'a self,
        parts: SecurityDeleteRoleParts<'b>,
    ) -> SecurityDeleteRole<'a, 'b> {
        SecurityDeleteRole::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#delete-role-mapping)\n\nDeletes the specified role mapping."]
    pub fn delete_role_mapping<'b>(
        &'a self,
        parts: SecurityDeleteRoleMappingParts<'b>,
    ) -> SecurityDeleteRoleMapping<'a, 'b> {
        SecurityDeleteRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Delete Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#delete-action-group)\n\nDeletes the specified tenant."]
    pub fn delete_tenant<'b>(
        &'a self,
        parts: SecurityDeleteTenantParts<'b>,
    ) -> SecurityDeleteTenant<'a, 'b> {
        SecurityDeleteTenant::new(self.transport(), parts)
    }
    #[doc = "[Security Delete User API](https://opensearch.org/docs/latest/security/access-control/api/#delete-user)\n\nDeletes the specified internal user."]
    pub fn delete_user<'b>(
        &'a self,
        parts: SecurityDeleteUserParts<'b>,
    ) -> SecurityDeleteUser<'a, 'b> {
        SecurityDeleteUser::new(self.transport(), parts)
    }
    #[doc = "Security Delete User Legacy API\n\nDelete the specified user. Legacy API."]
    pub fn delete_user_legacy<'b>(
        &'a self,
        parts: SecurityDeleteUserLegacyParts<'b>,
    ) -> SecurityDeleteUserLegacy<'a, 'b> {
        SecurityDeleteUserLegacy::new(self.transport(), parts)
    }
    #[doc = "[Security Flush Cache API](https://opensearch.org/docs/latest/security/access-control/api/#flush-cache)\n\nFlushes the Security plugin's user, authentication, and authorization cache."]
    pub fn flush_cache<'b>(&'a self) -> SecurityFlushCache<'a, 'b> {
        SecurityFlushCache::new(self.transport())
    }
    #[doc = "[Security Generate Obo Token API](https://opensearch.org/docs/latest/security/access-control/authentication-tokens/#api-endpoint)\n\nGenerates a `On-Behalf-Of` token for the current user."]
    pub fn generate_obo_token<'b>(&'a self) -> SecurityGenerateOboToken<'a, 'b, ()> {
        SecurityGenerateOboToken::new(self.transport())
    }
    #[doc = "Security Generate User Token API\n\nGenerates an authorization token for the specified user."]
    pub fn generate_user_token<'b>(
        &'a self,
        parts: SecurityGenerateUserTokenParts<'b>,
    ) -> SecurityGenerateUserToken<'a, 'b, ()> {
        SecurityGenerateUserToken::new(self.transport(), parts)
    }
    #[doc = "[Security Get Account Details API](https://opensearch.org/docs/latest/security/access-control/api/#get-account-details)\n\nReturns account information for the current user."]
    pub fn get_account_details<'b>(&'a self) -> SecurityGetAccountDetails<'a, 'b> {
        SecurityGetAccountDetails::new(self.transport())
    }
    #[doc = "[Security Get Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#get-action-group)\n\nRetrieves one action group."]
    pub fn get_action_group<'b>(
        &'a self,
        parts: SecurityGetActionGroupParts<'b>,
    ) -> SecurityGetActionGroup<'a, 'b> {
        SecurityGetActionGroup::new(self.transport(), parts)
    }
    #[doc = "[Security Get Action Groups API](https://opensearch.org/docs/latest/security/access-control/api/#get-action-groups)\n\nRetrieves all action groups."]
    pub fn get_action_groups<'b>(&'a self) -> SecurityGetActionGroups<'a, 'b> {
        SecurityGetActionGroups::new(self.transport())
    }
    #[doc = "Security Get All Certificates API\n\nRetrieves the cluster security certificates."]
    pub fn get_all_certificates<'b>(&'a self) -> SecurityGetAllCertificates<'a, 'b> {
        SecurityGetAllCertificates::new(self.transport())
    }
    #[doc = "[Security Get Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nRetrieves the current list of allowed APIs accessible to a normal user."]
    pub fn get_allowlist<'b>(&'a self) -> SecurityGetAllowlist<'a, 'b> {
        SecurityGetAllowlist::new(self.transport())
    }
    #[doc = "[Security Get Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nRetrieves the audit configuration."]
    pub fn get_audit_configuration<'b>(&'a self) -> SecurityGetAuditConfiguration<'a, 'b> {
        SecurityGetAuditConfiguration::new(self.transport())
    }
    #[doc = "[Security Get Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#get-certificates)\n\nRetrieves the cluster security certificates."]
    pub fn get_certificates<'b>(&'a self) -> SecurityGetCertificates<'a, 'b> {
        SecurityGetCertificates::new(self.transport())
    }
    #[doc = "[Security Get Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#get-configuration)\n\nReturns the current Security plugin configuration in a JSON format."]
    pub fn get_configuration<'b>(&'a self) -> SecurityGetConfiguration<'a, 'b> {
        SecurityGetConfiguration::new(self.transport())
    }
    #[doc = "Security Get Dashboards Info API\n\nRetrieves the current values for dynamic security settings for OpenSearch Dashboards."]
    pub fn get_dashboards_info<'b>(&'a self) -> SecurityGetDashboardsInfo<'a, 'b> {
        SecurityGetDashboardsInfo::new(self.transport())
    }
    #[doc = "[Security Get Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#get-distinguished-names)\n\nRetrieves all node distinguished names. Requires super admin or REST API permissions."]
    pub fn get_distinguished_name<'b>(
        &'a self,
        parts: SecurityGetDistinguishedNameParts<'b>,
    ) -> SecurityGetDistinguishedName<'a, 'b> {
        SecurityGetDistinguishedName::new(self.transport(), parts)
    }
    #[doc = "[Security Get Distinguished Names API](https://opensearch.org/docs/latest/security/access-control/api/#get-distinguished-names)\n\nRetrieves all node distinguished names. Requires super admin or REST API permissions."]
    pub fn get_distinguished_names<'b>(&'a self) -> SecurityGetDistinguishedNames<'a, 'b> {
        SecurityGetDistinguishedNames::new(self.transport())
    }
    #[doc = "Security Get Node Certificates API\n\nRetrieves the specified node's security certificates."]
    pub fn get_node_certificates<'b>(
        &'a self,
        parts: SecurityGetNodeCertificatesParts<'b>,
    ) -> SecurityGetNodeCertificates<'a, 'b> {
        SecurityGetNodeCertificates::new(self.transport(), parts)
    }
    #[doc = "Security Get Permissions Info API\n\nRetrieves the evaluated REST API permissions for the currently logged in user."]
    pub fn get_permissions_info<'b>(&'a self) -> SecurityGetPermissionsInfo<'a, 'b> {
        SecurityGetPermissionsInfo::new(self.transport())
    }
    #[doc = "[Security Get Role API](https://opensearch.org/docs/latest/security/access-control/api/#get-role)\n\nRetrieves one role."]
    pub fn get_role<'b>(&'a self, parts: SecurityGetRoleParts<'b>) -> SecurityGetRole<'a, 'b> {
        SecurityGetRole::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#get-role-mapping)\n\nRetrieves the specified role mapping."]
    pub fn get_role_mapping<'b>(
        &'a self,
        parts: SecurityGetRoleMappingParts<'b>,
    ) -> SecurityGetRoleMapping<'a, 'b> {
        SecurityGetRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Get Role Mappings API](https://opensearch.org/docs/latest/security/access-control/api/#get-role-mappings)\n\nRetrieves all role mappings."]
    pub fn get_role_mappings<'b>(&'a self) -> SecurityGetRoleMappings<'a, 'b> {
        SecurityGetRoleMappings::new(self.transport())
    }
    #[doc = "[Security Get Roles API](https://opensearch.org/docs/latest/security/access-control/api/#get-roles)\n\nRetrieves all roles."]
    pub fn get_roles<'b>(&'a self) -> SecurityGetRoles<'a, 'b> {
        SecurityGetRoles::new(self.transport())
    }
    #[doc = "Security Get Sslinfo API\n\nRetrieves information about the SSL configuration."]
    pub fn get_sslinfo<'b>(&'a self) -> SecurityGetSslinfo<'a, 'b> {
        SecurityGetSslinfo::new(self.transport())
    }
    #[doc = "[Security Get Tenancy Config API](https://opensearch.org/docs/latest/security/multi-tenancy/dynamic-config/#configuring-multi-tenancy-with-the-rest-api)\n\nRetrieves the multi-tenancy configuration. Requires super admin or REST API permissions."]
    pub fn get_tenancy_config<'b>(&'a self) -> SecurityGetTenancyConfig<'a, 'b> {
        SecurityGetTenancyConfig::new(self.transport())
    }
    #[doc = "[Security Get Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#get-tenant)\n\nRetrieves the specified tenant."]
    pub fn get_tenant<'b>(
        &'a self,
        parts: SecurityGetTenantParts<'b>,
    ) -> SecurityGetTenant<'a, 'b> {
        SecurityGetTenant::new(self.transport(), parts)
    }
    #[doc = "[Security Get Tenants API](https://opensearch.org/docs/latest/security/access-control/api/#get-tenants)\n\nRetrieves all tenants."]
    pub fn get_tenants<'b>(&'a self) -> SecurityGetTenants<'a, 'b> {
        SecurityGetTenants::new(self.transport())
    }
    #[doc = "[Security Get User API](https://opensearch.org/docs/latest/security/access-control/api/#get-user)\n\nRetrieve information about the specified internal user."]
    pub fn get_user<'b>(&'a self, parts: SecurityGetUserParts<'b>) -> SecurityGetUser<'a, 'b> {
        SecurityGetUser::new(self.transport(), parts)
    }
    #[doc = "Security Get User Legacy API\n\nRetrieve one user. Legacy API."]
    pub fn get_user_legacy<'b>(
        &'a self,
        parts: SecurityGetUserLegacyParts<'b>,
    ) -> SecurityGetUserLegacy<'a, 'b> {
        SecurityGetUserLegacy::new(self.transport(), parts)
    }
    #[doc = "[Security Get Users API](https://opensearch.org/docs/latest/security/access-control/api/#get-users)\n\nRetrieve all internal users."]
    pub fn get_users<'b>(&'a self) -> SecurityGetUsers<'a, 'b> {
        SecurityGetUsers::new(self.transport())
    }
    #[doc = "Security Get Users Legacy API\n\nRetrieve all internal users. Legacy API."]
    pub fn get_users_legacy<'b>(&'a self) -> SecurityGetUsersLegacy<'a, 'b> {
        SecurityGetUsersLegacy::new(self.transport())
    }
    #[doc = "[Security Health API](https://opensearch.org/docs/latest/security/access-control/api/#health-check)\n\nChecks to see if the Security plugin is running."]
    pub fn health<'b>(&'a self) -> SecurityHealth<'a, 'b, ()> {
        SecurityHealth::new(self.transport())
    }
    #[doc = "Security Migrate API\n\nMigrates the security configuration from v6 to v7."]
    pub fn migrate<'b>(&'a self) -> SecurityMigrate<'a, 'b, ()> {
        SecurityMigrate::new(self.transport())
    }
    #[doc = "[Security Patch Action Group API](https://opensearch.org/docs/latest/security/access-control/api/#patch-action-group)\n\nUpdates the individual attributes of an action group."]
    pub fn patch_action_group<'b>(
        &'a self,
        parts: SecurityPatchActionGroupParts<'b>,
    ) -> SecurityPatchActionGroup<'a, 'b, ()> {
        SecurityPatchActionGroup::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Action Groups API](https://opensearch.org/docs/latest/security/access-control/api/#patch-action-groups)\n\nCreates, updates, or deletes multiple action groups in a single request."]
    pub fn patch_action_groups<'b>(&'a self) -> SecurityPatchActionGroups<'a, 'b, ()> {
        SecurityPatchActionGroups::new(self.transport())
    }
    #[doc = "[Security Patch Allowlist API](https://opensearch.org/docs/latest/security/access-control/api/#access-control-for-the-api)\n\nUpdates the current list of APIs accessible for users on the allow list."]
    pub fn patch_allowlist<'b>(&'a self) -> SecurityPatchAllowlist<'a, 'b, ()> {
        SecurityPatchAllowlist::new(self.transport())
    }
    #[doc = "[Security Patch Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nUpdates the specified fields in the audit configuration."]
    pub fn patch_audit_configuration<'b>(&'a self) -> SecurityPatchAuditConfiguration<'a, 'b, ()> {
        SecurityPatchAuditConfiguration::new(self.transport())
    }
    #[doc = "[Security Patch Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#patch-configuration)\n\nUpdates the existing security configuration using the REST API. Requires super admin or REST API permissions."]
    pub fn patch_configuration<'b>(&'a self) -> SecurityPatchConfiguration<'a, 'b, ()> {
        SecurityPatchConfiguration::new(self.transport())
    }
    #[doc = "Security Patch Distinguished Name API\n\nUpdates the distinguished cluster name for the specified cluster. Requires super admin or REST API permissions."]
    pub fn patch_distinguished_name<'b>(
        &'a self,
        parts: SecurityPatchDistinguishedNameParts<'b>,
    ) -> SecurityPatchDistinguishedName<'a, 'b, ()> {
        SecurityPatchDistinguishedName::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Distinguished Names API](https://opensearch.org/docs/latest/security/access-control/api/#update-all-distinguished-names)\n\nBulk updates specified node distinguished names. Requires super admin or REST API permissions."]
    pub fn patch_distinguished_names<'b>(&'a self) -> SecurityPatchDistinguishedNames<'a, 'b, ()> {
        SecurityPatchDistinguishedNames::new(self.transport())
    }
    #[doc = "[Security Patch Role API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role)\n\nUpdates the individual attributes of a role."]
    pub fn patch_role<'b>(
        &'a self,
        parts: SecurityPatchRoleParts<'b>,
    ) -> SecurityPatchRole<'a, 'b, ()> {
        SecurityPatchRole::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Role Mapping API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role-mapping)\n\nUpdates the individual attributes of a role mapping."]
    pub fn patch_role_mapping<'b>(
        &'a self,
        parts: SecurityPatchRoleMappingParts<'b>,
    ) -> SecurityPatchRoleMapping<'a, 'b, ()> {
        SecurityPatchRoleMapping::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Role Mappings API](https://opensearch.org/docs/latest/security/access-control/api/#patch-role-mappings)\n\nCreates or updates multiple role mappings in a single request."]
    pub fn patch_role_mappings<'b>(&'a self) -> SecurityPatchRoleMappings<'a, 'b, ()> {
        SecurityPatchRoleMappings::new(self.transport())
    }
    #[doc = "[Security Patch Roles API](https://opensearch.org/docs/latest/security/access-control/api/#patch-roles)\n\nCreates, updates, or deletes multiple roles in a single call."]
    pub fn patch_roles<'b>(&'a self) -> SecurityPatchRoles<'a, 'b, ()> {
        SecurityPatchRoles::new(self.transport())
    }
    #[doc = "[Security Patch Tenant API](https://opensearch.org/docs/latest/security/access-control/api/#patch-tenant)\n\nAdds, deletes, or modifies a single tenant."]
    pub fn patch_tenant<'b>(
        &'a self,
        parts: SecurityPatchTenantParts<'b>,
    ) -> SecurityPatchTenant<'a, 'b, ()> {
        SecurityPatchTenant::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Tenants API](https://opensearch.org/docs/latest/security/access-control/api/#patch-tenants)\n\nAdds, deletes, or modifies multiple tenants in a single request."]
    pub fn patch_tenants<'b>(&'a self) -> SecurityPatchTenants<'a, 'b, ()> {
        SecurityPatchTenants::new(self.transport())
    }
    #[doc = "[Security Patch User API](https://opensearch.org/docs/latest/security/access-control/api/#patch-user)\n\nUpdates individual attributes for an internal user."]
    pub fn patch_user<'b>(
        &'a self,
        parts: SecurityPatchUserParts<'b>,
    ) -> SecurityPatchUser<'a, 'b, ()> {
        SecurityPatchUser::new(self.transport(), parts)
    }
    #[doc = "[Security Patch Users API](https://opensearch.org/docs/latest/security/access-control/api/#patch-users)\n\nCreates, updates, or deletes multiple internal users in a single request."]
    pub fn patch_users<'b>(&'a self) -> SecurityPatchUsers<'a, 'b, ()> {
        SecurityPatchUsers::new(self.transport())
    }
    #[doc = "Security Post Dashboards Info API\n\nRetrieves the current values for dynamic security settings for OpenSearch Dashboards."]
    pub fn post_dashboards_info<'b>(&'a self) -> SecurityPostDashboardsInfo<'a, 'b, ()> {
        SecurityPostDashboardsInfo::new(self.transport())
    }
    #[doc = "[Security Reload Http Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#reload-http-certificates)\n\nReloads the HTTP communication certificates."]
    pub fn reload_http_certificates<'b>(&'a self) -> SecurityReloadHttpCertificates<'a, 'b, ()> {
        SecurityReloadHttpCertificates::new(self.transport())
    }
    #[doc = "[Security Reload Transport Certificates API](https://opensearch.org/docs/latest/security/access-control/api/#reload-transport-certificates)\n\nReloads the transport communication certificates."]
    pub fn reload_transport_certificates<'b>(
        &'a self,
    ) -> SecurityReloadTransportCertificates<'a, 'b, ()> {
        SecurityReloadTransportCertificates::new(self.transport())
    }
    #[doc = "Security Tenant Info API\n\nRetrieves the names of current tenants. Requires super admin or `kibanaserver` permissions."]
    pub fn tenant_info<'b>(&'a self) -> SecurityTenantInfo<'a, 'b, ()> {
        SecurityTenantInfo::new(self.transport())
    }
    #[doc = "[Security Update Audit Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#audit-logs)\n\nUpdates the audit configuration."]
    pub fn update_audit_configuration<'b>(
        &'a self,
    ) -> SecurityUpdateAuditConfiguration<'a, 'b, ()> {
        SecurityUpdateAuditConfiguration::new(self.transport())
    }
    #[doc = "[Security Update Configuration API](https://opensearch.org/docs/latest/security/access-control/api/#update-configuration)\n\nUpdates the settings for an existing security configuration. Requires super admin or REST API permissions."]
    pub fn update_configuration<'b>(&'a self) -> SecurityUpdateConfiguration<'a, 'b, ()> {
        SecurityUpdateConfiguration::new(self.transport())
    }
    #[doc = "[Security Update Distinguished Name API](https://opensearch.org/docs/latest/security/access-control/api/#update-distinguished-names)\n\nAdds or updates the specified distinguished names in the cluster or node allowlist. Requires super admin or REST API permissions."]
    pub fn update_distinguished_name<'b>(
        &'a self,
        parts: SecurityUpdateDistinguishedNameParts<'b>,
    ) -> SecurityUpdateDistinguishedName<'a, 'b, ()> {
        SecurityUpdateDistinguishedName::new(self.transport(), parts)
    }
    #[doc = "Security Validate API\n\nChecks whether the v6 security configuration is valid and ready to be migrated to v7."]
    pub fn validate<'b>(&'a self) -> SecurityValidate<'a, 'b> {
        SecurityValidate::new(self.transport())
    }
    #[doc = "Security Who Am I API\n\nGets the identity information for the user currently logged in."]
    pub fn who_am_i<'b>(&'a self) -> SecurityWhoAmI<'a, 'b, ()> {
        SecurityWhoAmI::new(self.transport())
    }
    #[doc = "Security Who Am I Protected API\n\nGets the identity information for the user currently logged in. To use this operation, you must have access to this endpoint when authorization at REST layer is enabled."]
    pub fn who_am_i_protected<'b>(&'a self) -> SecurityWhoAmIProtected<'a, 'b> {
        SecurityWhoAmIProtected::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Security APIs"]
    pub fn security(&self) -> Security {
        Security::new(self.transport())
    }
}
