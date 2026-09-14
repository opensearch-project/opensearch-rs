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
#[doc = "API parts for the Neural Stats API"]
pub enum NeuralStatsParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Stat"]
    Stat(&'b str),
    #[doc = "NodeId"]
    NodeId(&'b str),
    #[doc = "NodeId and Stat"]
    NodeIdStat(&'b str, &'b str),
}
impl<'b> NeuralStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Neural Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NeuralStatsParts::None => "/_plugins/_neural/stats".into(),
            NeuralStatsParts::Stat(stat) => {
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_stat.len());
                p.push_str("/_plugins/_neural/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
            NeuralStatsParts::NodeId(node_id) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_node_id.len());
                p.push_str("/_plugins/_neural/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
            NeuralStatsParts::NodeIdStat(node_id, stat) => {
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id.as_bytes(), PARTS_ENCODED).into();
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(25usize + encoded_node_id.len() + encoded_stat.len());
                p.push_str("/_plugins/_neural/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Neural Stats API\n\nProvides information about the current status of the neural-search plugin."]
#[derive(Clone, Debug)]
pub struct NeuralStats<'a, 'b> {
    transport: &'a Transport,
    parts: NeuralStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    flat_stat_paths: Option<bool>,
    headers: HeaderMap,
    human: Option<bool>,
    include_all_nodes: Option<bool>,
    include_individual_nodes: Option<bool>,
    include_info: Option<bool>,
    include_metadata: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NeuralStats<'a, 'b> {
    #[doc = "Creates a new instance of [NeuralStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NeuralStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NeuralStats {
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
    #[doc = "Whether to return stats in the flat form, which can improve readability, especially for heavily nested stats.\nFor example, the flat form of `\"processors\": { \"ingest\": { \"text_embedding_executions\": 20181212 } }` is \n`\"processors.ingest.text_embedding_executions\": \"20181212\"`."]
    pub fn flat_stat_paths(mut self, flat_stat_paths: bool) -> Self {
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
    #[doc = "When `true` includes aggregated statistics across all nodes in the `all_nodes` category.\nWhen `false`, excludes the `all_nodes` category from the response."]
    pub fn include_all_nodes(mut self, include_all_nodes: bool) -> Self {
        self.include_all_nodes = Some(include_all_nodes);
        self
    }
    #[doc = "When `true` includes statistics for individual nodes in the `nodes` category.\nWhen `false`, excludes the `nodes` category from the response."]
    pub fn include_individual_nodes(mut self, include_individual_nodes: bool) -> Self {
        self.include_individual_nodes = Some(include_individual_nodes);
        self
    }
    #[doc = "When `true` includes cluster-wide information in the `info` category.\nWhen `false`, excludes the `info` category from the response."]
    pub fn include_info(mut self, include_info: bool) -> Self {
        self.include_info = Some(include_info);
        self
    }
    #[doc = "Whether to return stat metadata instead of the raw stat value, includes additional information about the stat.\nThese can include things like type hints, time since last stats being recorded, or recent rolling interval values"]
    pub fn include_metadata(mut self, include_metadata: bool) -> Self {
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
    #[doc = "Creates an asynchronous call to the Neural Stats API that can be awaited"]
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
                flat_stat_paths: Option<bool>,
                human: Option<bool>,
                include_all_nodes: Option<bool>,
                include_individual_nodes: Option<bool>,
                include_info: Option<bool>,
                include_metadata: Option<bool>,
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
#[doc = "Namespace client for Neural APIs"]
pub struct Neural<'a> {
    transport: &'a Transport,
}
impl<'a> Neural<'a> {
    #[doc = "Creates a new instance of [Neural]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "Neural Stats API\n\nProvides information about the current status of the neural-search plugin."]
    pub fn stats<'b>(&'a self, parts: NeuralStatsParts<'b>) -> NeuralStats<'a, 'b> {
        NeuralStats::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Neural APIs"]
    pub fn neural(&self) -> Neural {
        Neural::new(self.transport())
    }
}
