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
#[doc = "API parts for the Search Relevance Delete Experiments API"]
pub enum SearchRelevanceDeleteExperimentsParts<'b> {
    #[doc = "ExperimentId"]
    ExperimentId(&'b str),
}
impl<'b> SearchRelevanceDeleteExperimentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Delete Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceDeleteExperimentsParts::ExperimentId(experiment_id) => {
                let encoded_experiment_id: Cow<str> =
                    percent_encode(experiment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_experiment_id.len());
                p.push_str("/_plugins/_search_relevance/experiments/");
                p.push_str(encoded_experiment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Delete Experiments API\n\nDeletes a specified experiment."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceDeleteExperiments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceDeleteExperimentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceDeleteExperiments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceDeleteExperiments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceDeleteExperimentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceDeleteExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Delete Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Delete Judgments API"]
pub enum SearchRelevanceDeleteJudgmentsParts<'b> {
    #[doc = "JudgmentId"]
    JudgmentId(&'b str),
}
impl<'b> SearchRelevanceDeleteJudgmentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Delete Judgments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceDeleteJudgmentsParts::JudgmentId(judgment_id) => {
                let encoded_judgment_id: Cow<str> =
                    percent_encode(judgment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(38usize + encoded_judgment_id.len());
                p.push_str("/_plugins/_search_relevance/judgments/");
                p.push_str(encoded_judgment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Delete Judgments API\n\nDeletes a specified judgment."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceDeleteJudgments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceDeleteJudgmentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceDeleteJudgments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceDeleteJudgments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceDeleteJudgmentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceDeleteJudgments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Delete Judgments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Delete Query Sets API"]
pub enum SearchRelevanceDeleteQuerySetsParts<'b> {
    #[doc = "QuerySetId"]
    QuerySetId(&'b str),
}
impl<'b> SearchRelevanceDeleteQuerySetsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Delete Query Sets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceDeleteQuerySetsParts::QuerySetId(query_set_id) => {
                let encoded_query_set_id: Cow<str> =
                    percent_encode(query_set_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_query_set_id.len());
                p.push_str("/_plugins/_search_relevance/query_sets/");
                p.push_str(encoded_query_set_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Delete Query Sets API\n\nDeletes a query set."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceDeleteQuerySets<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceDeleteQuerySetsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceDeleteQuerySets<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceDeleteQuerySets] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceDeleteQuerySetsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceDeleteQuerySets {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Delete Query Sets API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Delete Scheduled Experiments API"]
pub enum SearchRelevanceDeleteScheduledExperimentsParts<'b> {
    #[doc = "ExperimentId"]
    ExperimentId(&'b str),
}
impl<'b> SearchRelevanceDeleteScheduledExperimentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Delete Scheduled Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceDeleteScheduledExperimentsParts::ExperimentId(experiment_id) => {
                let encoded_experiment_id: Cow<str> =
                    percent_encode(experiment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(49usize + encoded_experiment_id.len());
                p.push_str("/_plugins/_search_relevance/experiments/schedule/");
                p.push_str(encoded_experiment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Delete Scheduled Experiments API\n\nDeletes a specified scheduled experiment."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceDeleteScheduledExperiments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceDeleteScheduledExperimentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceDeleteScheduledExperiments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceDeleteScheduledExperiments] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SearchRelevanceDeleteScheduledExperimentsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceDeleteScheduledExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Delete Scheduled Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Delete Search Configurations API"]
pub enum SearchRelevanceDeleteSearchConfigurationsParts<'b> {
    #[doc = "SearchConfigurationId"]
    SearchConfigurationId(&'b str),
}
impl<'b> SearchRelevanceDeleteSearchConfigurationsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Delete Search Configurations API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceDeleteSearchConfigurationsParts::SearchConfigurationId(
                search_configuration_id,
            ) => {
                let encoded_search_configuration_id: Cow<str> =
                    percent_encode(search_configuration_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(50usize + encoded_search_configuration_id.len());
                p.push_str("/_plugins/_search_relevance/search_configurations/");
                p.push_str(encoded_search_configuration_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Delete Search Configurations API\n\nDeletes a specified search configuration."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceDeleteSearchConfigurations<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceDeleteSearchConfigurationsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceDeleteSearchConfigurations<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceDeleteSearchConfigurations] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SearchRelevanceDeleteSearchConfigurationsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceDeleteSearchConfigurations {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Delete Search Configurations API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Experiments API"]
pub enum SearchRelevanceGetExperimentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ExperimentId"]
    ExperimentId(&'b str),
}
impl<'b> SearchRelevanceGetExperimentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetExperimentsParts::None => {
                "/_plugins/_search_relevance/experiments".into()
            }
            SearchRelevanceGetExperimentsParts::ExperimentId(experiment_id) => {
                let encoded_experiment_id: Cow<str> =
                    percent_encode(experiment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(40usize + encoded_experiment_id.len());
                p.push_str("/_plugins/_search_relevance/experiments/");
                p.push_str(encoded_experiment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Experiments API\n\nGets experiments."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetExperiments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetExperimentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetExperiments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetExperiments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceGetExperimentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Judgments API"]
pub enum SearchRelevanceGetJudgmentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "JudgmentId"]
    JudgmentId(&'b str),
}
impl<'b> SearchRelevanceGetJudgmentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Judgments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetJudgmentsParts::None => {
                "/_plugins/_search_relevance/judgments".into()
            }
            SearchRelevanceGetJudgmentsParts::JudgmentId(judgment_id) => {
                let encoded_judgment_id: Cow<str> =
                    percent_encode(judgment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(38usize + encoded_judgment_id.len());
                p.push_str("/_plugins/_search_relevance/judgments/");
                p.push_str(encoded_judgment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Judgments API\n\nGets judgments."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetJudgments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetJudgmentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetJudgments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetJudgments] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceGetJudgmentsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetJudgments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Judgments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Node Stats API"]
pub enum SearchRelevanceGetNodeStatsParts<'b> {
    #[doc = "NodeId"]
    NodeId(&'b str),
    #[doc = "NodeId and Stat"]
    NodeIdStat(&'b str, &'b str),
}
impl<'b> SearchRelevanceGetNodeStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Node Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetNodeStatsParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(34usize + encoded_node_id.len());
                p.push_str("/_plugins/_search_relevance/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            SearchRelevanceGetNodeStatsParts::NodeIdStat(node_id, stat) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(35usize + encoded_node_id.len() + encoded_stat.len());
                p.push_str("/_plugins/_search_relevance/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Node Stats API\n\nGets stats by node."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetNodeStats<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetNodeStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_stat_paths: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    include_all_nodes: Option<&'b str>,
    include_individual_nodes: Option<&'b str>,
    include_info: Option<&'b str>,
    include_metadata: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetNodeStats<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetNodeStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceGetNodeStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetNodeStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_stat_paths: None,
            human: None,
            include_all_nodes: None,
            include_individual_nodes: None,
            include_info: None,
            include_metadata: None,
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
    #[doc = "Requests flattened stat paths as keys"]
    pub fn flat_stat_paths(mut self, flat_stat_paths: &'b str) -> Self {
        self.flat_stat_paths = Some(flat_stat_paths);
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
    #[doc = "Whether to include all nodes"]
    pub fn include_all_nodes(mut self, include_all_nodes: &'b str) -> Self {
        self.include_all_nodes = Some(include_all_nodes);
        self
    }
    #[doc = "Whether to include individual nodes"]
    pub fn include_individual_nodes(mut self, include_individual_nodes: &'b str) -> Self {
        self.include_individual_nodes = Some(include_individual_nodes);
        self
    }
    #[doc = "Whether to include info"]
    pub fn include_info(mut self, include_info: &'b str) -> Self {
        self.include_info = Some(include_info);
        self
    }
    #[doc = "Whether to include metadata"]
    pub fn include_metadata(mut self, include_metadata: &'b str) -> Self {
        self.include_metadata = Some(include_metadata);
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Node Stats API that can be awaited"]
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
                flat_stat_paths: Option<&'b str>,
                human: Option<bool>,
                include_all_nodes: Option<&'b str>,
                include_individual_nodes: Option<&'b str>,
                include_info: Option<&'b str>,
                include_metadata: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_stat_paths: self.flat_stat_paths,
                human: self.human,
                include_all_nodes: self.include_all_nodes,
                include_individual_nodes: self.include_individual_nodes,
                include_info: self.include_info,
                include_metadata: self.include_metadata,
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
#[doc = "API parts for the Search Relevance Get Query Sets API"]
pub enum SearchRelevanceGetQuerySetsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "QuerySetId"]
    QuerySetId(&'b str),
}
impl<'b> SearchRelevanceGetQuerySetsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Query Sets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetQuerySetsParts::None => {
                "/_plugins/_search_relevance/query_sets".into()
            }
            SearchRelevanceGetQuerySetsParts::QuerySetId(query_set_id) => {
                let encoded_query_set_id: Cow<str> =
                    percent_encode(query_set_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_query_set_id.len());
                p.push_str("/_plugins/_search_relevance/query_sets/");
                p.push_str(encoded_query_set_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Query Sets API\n\nLists the current query sets available."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetQuerySets<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetQuerySetsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetQuerySets<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetQuerySets] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceGetQuerySetsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetQuerySets {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Query Sets API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Scheduled Experiments API"]
pub enum SearchRelevanceGetScheduledExperimentsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ExperimentId"]
    ExperimentId(&'b str),
}
impl<'b> SearchRelevanceGetScheduledExperimentsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Scheduled Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetScheduledExperimentsParts::None => {
                "/_plugins/_search_relevance/experiments/schedule".into()
            }
            SearchRelevanceGetScheduledExperimentsParts::ExperimentId(experiment_id) => {
                let encoded_experiment_id: Cow<str> =
                    percent_encode(experiment_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(49usize + encoded_experiment_id.len());
                p.push_str("/_plugins/_search_relevance/experiments/schedule/");
                p.push_str(encoded_experiment_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Scheduled Experiments API\n\nGets the scheduled experiments."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetScheduledExperiments<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetScheduledExperimentsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetScheduledExperiments<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetScheduledExperiments] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SearchRelevanceGetScheduledExperimentsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetScheduledExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Scheduled Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Search Configurations API"]
pub enum SearchRelevanceGetSearchConfigurationsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "SearchConfigurationId"]
    SearchConfigurationId(&'b str),
}
impl<'b> SearchRelevanceGetSearchConfigurationsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Search Configurations API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetSearchConfigurationsParts::None => {
                "/_plugins/_search_relevance/search_configurations".into()
            }
            SearchRelevanceGetSearchConfigurationsParts::SearchConfigurationId(
                search_configuration_id,
            ) => {
                let encoded_search_configuration_id: Cow<str> =
                    percent_encode(search_configuration_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(50usize + encoded_search_configuration_id.len());
                p.push_str("/_plugins/_search_relevance/search_configurations/");
                p.push_str(encoded_search_configuration_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Search Configurations API\n\nGets the search configurations."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetSearchConfigurations<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetSearchConfigurationsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetSearchConfigurations<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetSearchConfigurations] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: SearchRelevanceGetSearchConfigurationsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetSearchConfigurations {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Search Configurations API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Get Stats API"]
pub enum SearchRelevanceGetStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Stat"]
    Stat(&'b str),
}
impl<'b> SearchRelevanceGetStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Search Relevance Get Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevanceGetStatsParts::None => "/_plugins/_search_relevance/stats".into(),
            SearchRelevanceGetStatsParts::Stat(stat) => {
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(34usize + encoded_stat.len());
                p.push_str("/_plugins/_search_relevance/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Get Stats API\n\nGets stats."]
#[derive(Clone, Debug)]
pub struct SearchRelevanceGetStats<'a, 'b> {
    transport: &'a Transport,
    parts: SearchRelevanceGetStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_stat_paths: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    include_all_nodes: Option<&'b str>,
    include_individual_nodes: Option<&'b str>,
    include_info: Option<&'b str>,
    include_metadata: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> SearchRelevanceGetStats<'a, 'b> {
    #[doc = "Creates a new instance of [SearchRelevanceGetStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: SearchRelevanceGetStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        SearchRelevanceGetStats {
            transport,
            parts,
            headers,
            error_trace: None,
            filter_path: None,
            flat_stat_paths: None,
            human: None,
            include_all_nodes: None,
            include_individual_nodes: None,
            include_info: None,
            include_metadata: None,
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
    #[doc = "Requests flattened stat paths as keys"]
    pub fn flat_stat_paths(mut self, flat_stat_paths: &'b str) -> Self {
        self.flat_stat_paths = Some(flat_stat_paths);
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
    #[doc = "Whether to include all nodes"]
    pub fn include_all_nodes(mut self, include_all_nodes: &'b str) -> Self {
        self.include_all_nodes = Some(include_all_nodes);
        self
    }
    #[doc = "Whether to include individual nodes"]
    pub fn include_individual_nodes(mut self, include_individual_nodes: &'b str) -> Self {
        self.include_individual_nodes = Some(include_individual_nodes);
        self
    }
    #[doc = "Whether to include info"]
    pub fn include_info(mut self, include_info: &'b str) -> Self {
        self.include_info = Some(include_info);
        self
    }
    #[doc = "Whether to include metadata"]
    pub fn include_metadata(mut self, include_metadata: &'b str) -> Self {
        self.include_metadata = Some(include_metadata);
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
    #[doc = "Creates an asynchronous call to the Search Relevance Get Stats API that can be awaited"]
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
                flat_stat_paths: Option<&'b str>,
                human: Option<bool>,
                include_all_nodes: Option<&'b str>,
                include_individual_nodes: Option<&'b str>,
                include_info: Option<&'b str>,
                include_metadata: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                flat_stat_paths: self.flat_stat_paths,
                human: self.human,
                include_all_nodes: self.include_all_nodes,
                include_individual_nodes: self.include_individual_nodes,
                include_info: self.include_info,
                include_metadata: self.include_metadata,
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
#[doc = "API parts for the Search Relevance Post Query Sets API"]
pub enum SearchRelevancePostQuerySetsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePostQuerySetsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Post Query Sets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePostQuerySetsParts::None => {
                "/_plugins/_search_relevance/query_sets".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Post Query Sets API\n\nCreates a new query set by sampling queries from the user behavior data."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePostQuerySets<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePostQuerySetsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePostQuerySets<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePostQuerySets]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePostQuerySets {
            transport,
            parts: SearchRelevancePostQuerySetsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePostQuerySets<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePostQuerySets {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Post Query Sets API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Post Scheduled Experiments API"]
pub enum SearchRelevancePostScheduledExperimentsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePostScheduledExperimentsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Post Scheduled Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePostScheduledExperimentsParts::None => {
                "/_plugins/_search_relevance/experiments/schedule".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Post Scheduled Experiments API\n\nCreates a scheduled experiment."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePostScheduledExperiments<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePostScheduledExperimentsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePostScheduledExperiments<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePostScheduledExperiments]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePostScheduledExperiments {
            transport,
            parts: SearchRelevancePostScheduledExperimentsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePostScheduledExperiments<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePostScheduledExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Post Scheduled Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Put Experiments API"]
pub enum SearchRelevancePutExperimentsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePutExperimentsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Put Experiments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePutExperimentsParts::None => {
                "/_plugins/_search_relevance/experiments".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Put Experiments API\n\nCreates an experiment."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePutExperiments<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePutExperimentsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePutExperiments<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePutExperiments]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePutExperiments {
            transport,
            parts: SearchRelevancePutExperimentsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePutExperiments<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePutExperiments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Put Experiments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Put Judgments API"]
pub enum SearchRelevancePutJudgmentsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePutJudgmentsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Put Judgments API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePutJudgmentsParts::None => {
                "/_plugins/_search_relevance/judgments".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Put Judgments API\n\nCreates a judgment."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePutJudgments<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePutJudgmentsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePutJudgments<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePutJudgments]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePutJudgments {
            transport,
            parts: SearchRelevancePutJudgmentsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePutJudgments<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePutJudgments {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Put Judgments API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Put Query Sets API"]
pub enum SearchRelevancePutQuerySetsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePutQuerySetsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Put Query Sets API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePutQuerySetsParts::None => {
                "/_plugins/_search_relevance/query_sets".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Put Query Sets API\n\nCreates a new query set by uploading manually."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePutQuerySets<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePutQuerySetsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePutQuerySets<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePutQuerySets]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePutQuerySets {
            transport,
            parts: SearchRelevancePutQuerySetsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePutQuerySets<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePutQuerySets {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Put Query Sets API that can be awaited"]
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
#[doc = "API parts for the Search Relevance Put Search Configurations API"]
pub enum SearchRelevancePutSearchConfigurationsParts {
    #[doc = "No parts"]
    None,
}
impl SearchRelevancePutSearchConfigurationsParts {
    #[doc = "Builds a relative URL path to the Search Relevance Put Search Configurations API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SearchRelevancePutSearchConfigurationsParts::None => {
                "/_plugins/_search_relevance/search_configurations".into()
            }
        }
    }
}
#[doc = "Builder for the Search Relevance Put Search Configurations API\n\nCreates a search configuration."]
#[derive(Clone, Debug)]
pub struct SearchRelevancePutSearchConfigurations<'a, 'b, B> {
    transport: &'a Transport,
    parts: SearchRelevancePutSearchConfigurationsParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> SearchRelevancePutSearchConfigurations<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [SearchRelevancePutSearchConfigurations]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SearchRelevancePutSearchConfigurations {
            transport,
            parts: SearchRelevancePutSearchConfigurationsParts::None,
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
    pub fn body<T>(self, body: T) -> SearchRelevancePutSearchConfigurations<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        SearchRelevancePutSearchConfigurations {
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
    #[doc = "Creates an asynchronous call to the Search Relevance Put Search Configurations API that can be awaited"]
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
#[doc = "Namespace client for SearchRelevance APIs"]
pub struct SearchRelevance<'a> {
    transport: &'a Transport,
}
impl<'a> SearchRelevance<'a> {
    #[doc = "Creates a new instance of [SearchRelevance]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Search Relevance Delete Experiments API\n\nDeletes a specified experiment."]
    pub fn delete_experiments<'b>(
        &'a self,
        parts: SearchRelevanceDeleteExperimentsParts<'b>,
    ) -> SearchRelevanceDeleteExperiments<'a, 'b> {
        SearchRelevanceDeleteExperiments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Delete Judgments API\n\nDeletes a specified judgment."]
    pub fn delete_judgments<'b>(
        &'a self,
        parts: SearchRelevanceDeleteJudgmentsParts<'b>,
    ) -> SearchRelevanceDeleteJudgments<'a, 'b> {
        SearchRelevanceDeleteJudgments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Delete Query Sets API\n\nDeletes a query set."]
    pub fn delete_query_sets<'b>(
        &'a self,
        parts: SearchRelevanceDeleteQuerySetsParts<'b>,
    ) -> SearchRelevanceDeleteQuerySets<'a, 'b> {
        SearchRelevanceDeleteQuerySets::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Delete Scheduled Experiments API\n\nDeletes a specified scheduled experiment."]
    pub fn delete_scheduled_experiments<'b>(
        &'a self,
        parts: SearchRelevanceDeleteScheduledExperimentsParts<'b>,
    ) -> SearchRelevanceDeleteScheduledExperiments<'a, 'b> {
        SearchRelevanceDeleteScheduledExperiments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Delete Search Configurations API\n\nDeletes a specified search configuration."]
    pub fn delete_search_configurations<'b>(
        &'a self,
        parts: SearchRelevanceDeleteSearchConfigurationsParts<'b>,
    ) -> SearchRelevanceDeleteSearchConfigurations<'a, 'b> {
        SearchRelevanceDeleteSearchConfigurations::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Experiments API\n\nGets experiments."]
    pub fn get_experiments<'b>(
        &'a self,
        parts: SearchRelevanceGetExperimentsParts<'b>,
    ) -> SearchRelevanceGetExperiments<'a, 'b> {
        SearchRelevanceGetExperiments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Judgments API\n\nGets judgments."]
    pub fn get_judgments<'b>(
        &'a self,
        parts: SearchRelevanceGetJudgmentsParts<'b>,
    ) -> SearchRelevanceGetJudgments<'a, 'b> {
        SearchRelevanceGetJudgments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Node Stats API\n\nGets stats by node."]
    pub fn get_node_stats<'b>(
        &'a self,
        parts: SearchRelevanceGetNodeStatsParts<'b>,
    ) -> SearchRelevanceGetNodeStats<'a, 'b> {
        SearchRelevanceGetNodeStats::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Query Sets API\n\nLists the current query sets available."]
    pub fn get_query_sets<'b>(
        &'a self,
        parts: SearchRelevanceGetQuerySetsParts<'b>,
    ) -> SearchRelevanceGetQuerySets<'a, 'b> {
        SearchRelevanceGetQuerySets::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Scheduled Experiments API\n\nGets the scheduled experiments."]
    pub fn get_scheduled_experiments<'b>(
        &'a self,
        parts: SearchRelevanceGetScheduledExperimentsParts<'b>,
    ) -> SearchRelevanceGetScheduledExperiments<'a, 'b> {
        SearchRelevanceGetScheduledExperiments::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Search Configurations API\n\nGets the search configurations."]
    pub fn get_search_configurations<'b>(
        &'a self,
        parts: SearchRelevanceGetSearchConfigurationsParts<'b>,
    ) -> SearchRelevanceGetSearchConfigurations<'a, 'b> {
        SearchRelevanceGetSearchConfigurations::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Get Stats API\n\nGets stats."]
    pub fn get_stats<'b>(
        &'a self,
        parts: SearchRelevanceGetStatsParts<'b>,
    ) -> SearchRelevanceGetStats<'a, 'b> {
        SearchRelevanceGetStats::new(self.transport(), parts)
    }
    #[doc = "Search Relevance Post Query Sets API\n\nCreates a new query set by sampling queries from the user behavior data."]
    pub fn post_query_sets<'b>(&'a self) -> SearchRelevancePostQuerySets<'a, 'b, ()> {
        SearchRelevancePostQuerySets::new(self.transport())
    }
    #[doc = "Search Relevance Post Scheduled Experiments API\n\nCreates a scheduled experiment."]
    pub fn post_scheduled_experiments<'b>(
        &'a self,
    ) -> SearchRelevancePostScheduledExperiments<'a, 'b, ()> {
        SearchRelevancePostScheduledExperiments::new(self.transport())
    }
    #[doc = "Search Relevance Put Experiments API\n\nCreates an experiment."]
    pub fn put_experiments<'b>(&'a self) -> SearchRelevancePutExperiments<'a, 'b, ()> {
        SearchRelevancePutExperiments::new(self.transport())
    }
    #[doc = "Search Relevance Put Judgments API\n\nCreates a judgment."]
    pub fn put_judgments<'b>(&'a self) -> SearchRelevancePutJudgments<'a, 'b, ()> {
        SearchRelevancePutJudgments::new(self.transport())
    }
    #[doc = "Search Relevance Put Query Sets API\n\nCreates a new query set by uploading manually."]
    pub fn put_query_sets<'b>(&'a self) -> SearchRelevancePutQuerySets<'a, 'b, ()> {
        SearchRelevancePutQuerySets::new(self.transport())
    }
    #[doc = "Search Relevance Put Search Configurations API\n\nCreates a search configuration."]
    pub fn put_search_configurations<'b>(
        &'a self,
    ) -> SearchRelevancePutSearchConfigurations<'a, 'b, ()> {
        SearchRelevancePutSearchConfigurations::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for SearchRelevance APIs"]
    pub fn search_relevance(&self) -> SearchRelevance {
        SearchRelevance::new(self.transport())
    }
}
