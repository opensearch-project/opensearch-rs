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
#[doc = "API parts for the Flow Framework Create API"]
pub enum FlowFrameworkCreateParts {
    #[doc = "No parts"]
    None,
}
impl FlowFrameworkCreateParts {
    #[doc = "Builds a relative URL path to the Flow Framework Create API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkCreateParts::None => "/_plugins/_flow_framework/workflow".into(),
        }
    }
}
#[doc = "Builder for the [Flow Framework Create API](https://opensearch.org/docs/latest/automating-configurations/api/create-workflow/)\n\nCreates a new workflow template."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkCreate<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkCreateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    provision: Option<bool>,
    reprovision: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    update_fields: Option<bool>,
    use_case: Option<&'b str>,
    validation: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkCreate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkCreate]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkCreate {
            transport,
            parts: FlowFrameworkCreateParts::None,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            provision: None,
            reprovision: None,
            request_timeout: None,
            source: None,
            update_fields: None,
            use_case: None,
            validation: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FlowFrameworkCreate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkCreate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            provision: self.provision,
            reprovision: self.reprovision,
            request_timeout: self.request_timeout,
            source: self.source,
            update_fields: self.update_fields,
            use_case: self.use_case,
            validation: self.validation,
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
    pub fn provision(mut self, provision: bool) -> Self {
        self.provision = Some(provision);
        self
    }
    pub fn reprovision(mut self, reprovision: bool) -> Self {
        self.reprovision = Some(reprovision);
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
    pub fn update_fields(mut self, update_fields: bool) -> Self {
        self.update_fields = Some(update_fields);
        self
    }
    #[doc = "Specifies the workflow template to use."]
    pub fn use_case(mut self, use_case: &'b str) -> Self {
        self.use_case = Some(use_case);
        self
    }
    pub fn validation(mut self, validation: &'b str) -> Self {
        self.validation = Some(validation);
        self
    }
    #[doc = "Creates an asynchronous call to the Flow Framework Create API that can be awaited"]
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
                provision: Option<bool>,
                reprovision: Option<bool>,
                source: Option<&'b str>,
                update_fields: Option<bool>,
                use_case: Option<&'b str>,
                validation: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                provision: self.provision,
                reprovision: self.reprovision,
                source: self.source,
                update_fields: self.update_fields,
                use_case: self.use_case,
                validation: self.validation,
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
#[doc = "API parts for the Flow Framework Delete API"]
pub enum FlowFrameworkDeleteParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkDeleteParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Delete API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkDeleteParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Delete API](https://opensearch.org/docs/latest/automating-configurations/api/delete-workflow/)\n\nDeletes a workflow template."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkDelete<'a, 'b> {
    transport: &'a Transport,
    parts: FlowFrameworkDeleteParts<'b>,
    clear_status: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> FlowFrameworkDelete<'a, 'b> {
    #[doc = "Creates a new instance of [FlowFrameworkDelete] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkDeleteParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkDelete {
            transport,
            parts,
            headers,
            clear_status: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    pub fn clear_status(mut self, clear_status: bool) -> Self {
        self.clear_status = Some(clear_status);
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
    #[doc = "Creates an asynchronous call to the Flow Framework Delete API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                clear_status: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                clear_status: self.clear_status,
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
#[doc = "API parts for the Flow Framework Deprovision API"]
pub enum FlowFrameworkDeprovisionParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkDeprovisionParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Deprovision API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkDeprovisionParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(48usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.push_str("/_deprovision");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Deprovision API](https://opensearch.org/docs/latest/automating-configurations/api/deprovision-workflow/)\n\nDeprovision workflow's resources when you no longer need them."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkDeprovision<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkDeprovisionParts<'b>,
    allow_delete: Option<&'b str>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkDeprovision<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkDeprovision] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkDeprovisionParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkDeprovision {
            transport,
            parts,
            headers,
            allow_delete: None,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    pub fn allow_delete(mut self, allow_delete: &'b str) -> Self {
        self.allow_delete = Some(allow_delete);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FlowFrameworkDeprovision<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkDeprovision {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            allow_delete: self.allow_delete,
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
    #[doc = "Creates an asynchronous call to the Flow Framework Deprovision API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Post;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                allow_delete: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                allow_delete: self.allow_delete,
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
#[doc = "API parts for the Flow Framework Get API"]
pub enum FlowFrameworkGetParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkGetParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Get API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkGetParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Get API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow/)\n\nRetrieves a workflow template."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkGet<'a, 'b> {
    transport: &'a Transport,
    parts: FlowFrameworkGetParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> FlowFrameworkGet<'a, 'b> {
    #[doc = "Creates a new instance of [FlowFrameworkGet] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkGetParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkGet {
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
    #[doc = "Creates an asynchronous call to the Flow Framework Get API that can be awaited"]
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
#[doc = "API parts for the Flow Framework Get Status API"]
pub enum FlowFrameworkGetStatusParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkGetStatusParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Get Status API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkGetStatusParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(43usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.push_str("/_status");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Get Status API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow-status/)\n\nRetrieves the current workflow provisioning status."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkGetStatus<'a, 'b> {
    transport: &'a Transport,
    parts: FlowFrameworkGetStatusParts<'b>,
    all: Option<bool>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> FlowFrameworkGetStatus<'a, 'b> {
    #[doc = "Creates a new instance of [FlowFrameworkGetStatus] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkGetStatusParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkGetStatus {
            transport,
            parts,
            headers,
            all: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "Whether to return all fields in the response."]
    pub fn all(mut self, all: bool) -> Self {
        self.all = Some(all);
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
    #[doc = "Creates an asynchronous call to the Flow Framework Get Status API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                all: Option<bool>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                all: self.all,
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
#[doc = "API parts for the Flow Framework Get Steps API"]
pub enum FlowFrameworkGetStepsParts {
    #[doc = "No parts"]
    None,
}
impl FlowFrameworkGetStepsParts {
    #[doc = "Builds a relative URL path to the Flow Framework Get Steps API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkGetStepsParts::None => "/_plugins/_flow_framework/workflow/_steps".into(),
        }
    }
}
#[doc = "Builder for the [Flow Framework Get Steps API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow-steps/)\n\nRetrieves available workflow steps."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkGetSteps<'a, 'b> {
    transport: &'a Transport,
    parts: FlowFrameworkGetStepsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    workflow_step: Option<&'b str>,
}
impl<'a, 'b> FlowFrameworkGetSteps<'a, 'b> {
    #[doc = "Creates a new instance of [FlowFrameworkGetSteps]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkGetSteps {
            transport,
            parts: FlowFrameworkGetStepsParts::None,
            headers,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
            workflow_step: None,
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
    pub fn workflow_step(mut self, workflow_step: &'b str) -> Self {
        self.workflow_step = Some(workflow_step);
        self
    }
    #[doc = "Creates an asynchronous call to the Flow Framework Get Steps API that can be awaited"]
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
                workflow_step: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                source: self.source,
                workflow_step: self.workflow_step,
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
#[doc = "API parts for the Flow Framework Provision API"]
pub enum FlowFrameworkProvisionParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkProvisionParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Provision API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkProvisionParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(46usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.push_str("/_provision");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Provision API](https://opensearch.org/docs/latest/automating-configurations/api/provision-workflow/)\n\nProvisioning a workflow. This API is also executed when the Create or Update Workflow API is called with the provision parameter set to true."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkProvision<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkProvisionParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkProvision<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkProvision] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkProvisionParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkProvision {
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
    pub fn body<T>(self, body: T) -> FlowFrameworkProvision<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkProvision {
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
    #[doc = "Creates an asynchronous call to the Flow Framework Provision API that can be awaited"]
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
#[doc = "API parts for the Flow Framework Search API"]
pub enum FlowFrameworkSearchParts {
    #[doc = "No parts"]
    None,
}
impl FlowFrameworkSearchParts {
    #[doc = "Builds a relative URL path to the Flow Framework Search API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkSearchParts::None => "/_plugins/_flow_framework/workflow/_search".into(),
        }
    }
}
#[doc = "Builder for the [Flow Framework Search API](https://opensearch.org/docs/latest/automating-configurations/api/provision-workflow/)\n\nSearch for workflows by using a query matching a field."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkSearch<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkSearchParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkSearch]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkSearch {
            transport,
            parts: FlowFrameworkSearchParts::None,
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
    pub fn body<T>(self, body: T) -> FlowFrameworkSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkSearch {
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
    #[doc = "Creates an asynchronous call to the Flow Framework Search API that can be awaited"]
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
#[doc = "API parts for the Flow Framework Search State API"]
pub enum FlowFrameworkSearchStateParts {
    #[doc = "No parts"]
    None,
}
impl FlowFrameworkSearchStateParts {
    #[doc = "Builds a relative URL path to the Flow Framework Search State API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkSearchStateParts::None => {
                "/_plugins/_flow_framework/workflow/state/_search".into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Search State API](https://opensearch.org/docs/latest/automating-configurations/api/search-workflow-state/)\n\nSearch for workflows by using a query matching a field."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkSearchState<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkSearchStateParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkSearchState<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkSearchState]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkSearchState {
            transport,
            parts: FlowFrameworkSearchStateParts::None,
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
    pub fn body<T>(self, body: T) -> FlowFrameworkSearchState<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkSearchState {
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
    #[doc = "Creates an asynchronous call to the Flow Framework Search State API that can be awaited"]
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
#[doc = "API parts for the Flow Framework Update API"]
pub enum FlowFrameworkUpdateParts<'b> {
    #[doc = "WorkflowId"]
    WorkflowId(&'b str),
}
impl<'b> FlowFrameworkUpdateParts<'b> {
    #[doc = "Builds a relative URL path to the Flow Framework Update API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            FlowFrameworkUpdateParts::WorkflowId(workflow_id) => {
                let encoded_workflow_id: Cow<str> =
                    percent_encode(workflow_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(35usize + encoded_workflow_id.len());
                p.push_str("/_plugins/_flow_framework/workflow/");
                p.push_str(encoded_workflow_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Flow Framework Update API](https://opensearch.org/docs/latest/automating-configurations/api/create-workflow/)\n\nUpdates a workflow template that has not been provisioned."]
#[derive(Clone, Debug)]
pub struct FlowFrameworkUpdate<'a, 'b, B> {
    transport: &'a Transport,
    parts: FlowFrameworkUpdateParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    provision: Option<bool>,
    reprovision: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    update_fields: Option<bool>,
    use_case: Option<&'b str>,
    validation: Option<&'b str>,
}
impl<'a, 'b, B> FlowFrameworkUpdate<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [FlowFrameworkUpdate] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: FlowFrameworkUpdateParts<'b>) -> Self {
        let headers = HeaderMap::new();
        FlowFrameworkUpdate {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            provision: None,
            reprovision: None,
            request_timeout: None,
            source: None,
            update_fields: None,
            use_case: None,
            validation: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> FlowFrameworkUpdate<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        FlowFrameworkUpdate {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            pretty: self.pretty,
            provision: self.provision,
            reprovision: self.reprovision,
            request_timeout: self.request_timeout,
            source: self.source,
            update_fields: self.update_fields,
            use_case: self.use_case,
            validation: self.validation,
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
    pub fn provision(mut self, provision: bool) -> Self {
        self.provision = Some(provision);
        self
    }
    pub fn reprovision(mut self, reprovision: bool) -> Self {
        self.reprovision = Some(reprovision);
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
    pub fn update_fields(mut self, update_fields: bool) -> Self {
        self.update_fields = Some(update_fields);
        self
    }
    #[doc = "Specifies the workflow template to use."]
    pub fn use_case(mut self, use_case: &'b str) -> Self {
        self.use_case = Some(use_case);
        self
    }
    pub fn validation(mut self, validation: &'b str) -> Self {
        self.validation = Some(validation);
        self
    }
    #[doc = "Creates an asynchronous call to the Flow Framework Update API that can be awaited"]
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
                provision: Option<bool>,
                reprovision: Option<bool>,
                source: Option<&'b str>,
                update_fields: Option<bool>,
                use_case: Option<&'b str>,
                validation: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                pretty: self.pretty,
                provision: self.provision,
                reprovision: self.reprovision,
                source: self.source,
                update_fields: self.update_fields,
                use_case: self.use_case,
                validation: self.validation,
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
#[doc = "Namespace client for FlowFramework APIs"]
pub struct FlowFramework<'a> {
    transport: &'a Transport,
}
impl<'a> FlowFramework<'a> {
    #[doc = "Creates a new instance of [FlowFramework]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Flow Framework Create API](https://opensearch.org/docs/latest/automating-configurations/api/create-workflow/)\n\nCreates a new workflow template."]
    pub fn create<'b>(&'a self) -> FlowFrameworkCreate<'a, 'b, ()> {
        FlowFrameworkCreate::new(self.transport())
    }
    #[doc = "[Flow Framework Delete API](https://opensearch.org/docs/latest/automating-configurations/api/delete-workflow/)\n\nDeletes a workflow template."]
    pub fn delete<'b>(
        &'a self,
        parts: FlowFrameworkDeleteParts<'b>,
    ) -> FlowFrameworkDelete<'a, 'b> {
        FlowFrameworkDelete::new(self.transport(), parts)
    }
    #[doc = "[Flow Framework Deprovision API](https://opensearch.org/docs/latest/automating-configurations/api/deprovision-workflow/)\n\nDeprovision workflow's resources when you no longer need them."]
    pub fn deprovision<'b>(
        &'a self,
        parts: FlowFrameworkDeprovisionParts<'b>,
    ) -> FlowFrameworkDeprovision<'a, 'b, ()> {
        FlowFrameworkDeprovision::new(self.transport(), parts)
    }
    #[doc = "[Flow Framework Get API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow/)\n\nRetrieves a workflow template."]
    pub fn get<'b>(&'a self, parts: FlowFrameworkGetParts<'b>) -> FlowFrameworkGet<'a, 'b> {
        FlowFrameworkGet::new(self.transport(), parts)
    }
    #[doc = "[Flow Framework Get Status API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow-status/)\n\nRetrieves the current workflow provisioning status."]
    pub fn get_status<'b>(
        &'a self,
        parts: FlowFrameworkGetStatusParts<'b>,
    ) -> FlowFrameworkGetStatus<'a, 'b> {
        FlowFrameworkGetStatus::new(self.transport(), parts)
    }
    #[doc = "[Flow Framework Get Steps API](https://opensearch.org/docs/latest/automating-configurations/api/get-workflow-steps/)\n\nRetrieves available workflow steps."]
    pub fn get_steps<'b>(&'a self) -> FlowFrameworkGetSteps<'a, 'b> {
        FlowFrameworkGetSteps::new(self.transport())
    }
    #[doc = "[Flow Framework Provision API](https://opensearch.org/docs/latest/automating-configurations/api/provision-workflow/)\n\nProvisioning a workflow. This API is also executed when the Create or Update Workflow API is called with the provision parameter set to true."]
    pub fn provision<'b>(
        &'a self,
        parts: FlowFrameworkProvisionParts<'b>,
    ) -> FlowFrameworkProvision<'a, 'b, ()> {
        FlowFrameworkProvision::new(self.transport(), parts)
    }
    #[doc = "[Flow Framework Search API](https://opensearch.org/docs/latest/automating-configurations/api/provision-workflow/)\n\nSearch for workflows by using a query matching a field."]
    pub fn search<'b>(&'a self) -> FlowFrameworkSearch<'a, 'b, ()> {
        FlowFrameworkSearch::new(self.transport())
    }
    #[doc = "[Flow Framework Search State API](https://opensearch.org/docs/latest/automating-configurations/api/search-workflow-state/)\n\nSearch for workflows by using a query matching a field."]
    pub fn search_state<'b>(&'a self) -> FlowFrameworkSearchState<'a, 'b, ()> {
        FlowFrameworkSearchState::new(self.transport())
    }
    #[doc = "[Flow Framework Update API](https://opensearch.org/docs/latest/automating-configurations/api/create-workflow/)\n\nUpdates a workflow template that has not been provisioned."]
    pub fn update<'b>(
        &'a self,
        parts: FlowFrameworkUpdateParts<'b>,
    ) -> FlowFrameworkUpdate<'a, 'b, ()> {
        FlowFrameworkUpdate::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for FlowFramework APIs"]
    pub fn flow_framework(&self) -> FlowFramework {
        FlowFramework::new(self.transport())
    }
}
