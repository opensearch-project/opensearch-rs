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
#[doc = "API parts for the Knn Delete Model API"]
pub enum KnnDeleteModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> KnnDeleteModelParts<'b> {
    #[doc = "Builds a relative URL path to the Knn Delete Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnDeleteModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_model_id.len());
                p.push_str("/_plugins/_knn/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Knn Delete Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#delete-a-model)\n\nUsed to delete a particular model in the cluster."]
#[derive(Clone, Debug)]
pub struct KnnDeleteModel<'a, 'b> {
    transport: &'a Transport,
    parts: KnnDeleteModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> KnnDeleteModel<'a, 'b> {
    #[doc = "Creates a new instance of [KnnDeleteModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: KnnDeleteModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        KnnDeleteModel {
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
    #[doc = "Creates an asynchronous call to the Knn Delete Model API that can be awaited"]
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
#[doc = "API parts for the Knn Get Model API"]
pub enum KnnGetModelParts<'b> {
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> KnnGetModelParts<'b> {
    #[doc = "Builds a relative URL path to the Knn Get Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnGetModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(22usize + encoded_model_id.len());
                p.push_str("/_plugins/_knn/models/");
                p.push_str(encoded_model_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Knn Get Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#get-a-model)\n\nUsed to retrieve information about models present in the cluster."]
#[derive(Clone, Debug)]
pub struct KnnGetModel<'a, 'b> {
    transport: &'a Transport,
    parts: KnnGetModelParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> KnnGetModel<'a, 'b> {
    #[doc = "Creates a new instance of [KnnGetModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: KnnGetModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        KnnGetModel {
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
    #[doc = "Creates an asynchronous call to the Knn Get Model API that can be awaited"]
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
#[doc = "API parts for the Knn Search Models API"]
pub enum KnnSearchModelsParts {
    #[doc = "No parts"]
    None,
}
impl KnnSearchModelsParts {
    #[doc = "Builds a relative URL path to the Knn Search Models API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnSearchModelsParts::None => "/_plugins/_knn/models/_search".into(),
        }
    }
}
#[doc = "Builder for the [Knn Search Models API](https://docs.opensearch.org/latest/vector-search/api/knn/#search-for-a-model)\n\nUse an OpenSearch query to search for models in the index."]
#[derive(Clone, Debug)]
pub struct KnnSearchModels<'a, 'b, B> {
    transport: &'a Transport,
    parts: KnnSearchModelsParts,
    _source: Option<&'b [&'b str]>,
    _source_excludes: Option<&'b [&'b str]>,
    _source_includes: Option<&'b [&'b str]>,
    allow_no_indices: Option<bool>,
    allow_partial_search_results: Option<bool>,
    analyze_wildcard: Option<bool>,
    analyzer: Option<&'b str>,
    batched_reduce_size: Option<i64>,
    body: Option<B>,
    ccs_minimize_roundtrips: Option<bool>,
    default_operator: Option<DefaultOperator>,
    df: Option<&'b str>,
    docvalue_fields: Option<&'b [&'b str]>,
    error_trace: Option<bool>,
    expand_wildcards: Option<&'b [ExpandWildcards]>,
    explain: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    from: Option<i64>,
    headers: HeaderMap,
    human: Option<bool>,
    ignore_throttled: Option<bool>,
    ignore_unavailable: Option<bool>,
    lenient: Option<bool>,
    max_concurrent_shard_requests: Option<i64>,
    pre_filter_shard_size: Option<i64>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    q: Option<&'b str>,
    request_cache: Option<bool>,
    request_timeout: Option<Duration>,
    rest_total_hits_as_int: Option<bool>,
    routing: Option<&'b [&'b str]>,
    scroll: Option<&'b str>,
    search_type: Option<SearchType>,
    seq_no_primary_term: Option<bool>,
    size: Option<i64>,
    sort: Option<&'b [&'b str]>,
    source: Option<&'b str>,
    stats: Option<&'b [&'b str]>,
    stored_fields: Option<&'b [&'b str]>,
    suggest_field: Option<&'b str>,
    suggest_mode: Option<SuggestMode>,
    suggest_size: Option<i64>,
    suggest_text: Option<&'b str>,
    terminate_after: Option<i64>,
    timeout: Option<&'b str>,
    track_scores: Option<bool>,
    track_total_hits: Option<TrackTotalHits>,
    typed_keys: Option<bool>,
    version: Option<bool>,
}
impl<'a, 'b, B> KnnSearchModels<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [KnnSearchModels]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        KnnSearchModels {
            transport,
            parts: KnnSearchModelsParts::None,
            headers,
            _source: None,
            _source_excludes: None,
            _source_includes: None,
            allow_no_indices: None,
            allow_partial_search_results: None,
            analyze_wildcard: None,
            analyzer: None,
            batched_reduce_size: None,
            body: None,
            ccs_minimize_roundtrips: None,
            default_operator: None,
            df: None,
            docvalue_fields: None,
            error_trace: None,
            expand_wildcards: None,
            explain: None,
            filter_path: None,
            from: None,
            human: None,
            ignore_throttled: None,
            ignore_unavailable: None,
            lenient: None,
            max_concurrent_shard_requests: None,
            pre_filter_shard_size: None,
            preference: None,
            pretty: None,
            q: None,
            request_cache: None,
            request_timeout: None,
            rest_total_hits_as_int: None,
            routing: None,
            scroll: None,
            search_type: None,
            seq_no_primary_term: None,
            size: None,
            sort: None,
            source: None,
            stats: None,
            stored_fields: None,
            suggest_field: None,
            suggest_mode: None,
            suggest_size: None,
            suggest_text: None,
            terminate_after: None,
            timeout: None,
            track_scores: None,
            track_total_hits: None,
            typed_keys: None,
            version: None,
        }
    }
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(mut self, _source: &'b [&'b str]) -> Self {
        self._source = Some(_source);
        self
    }
    #[doc = "List of fields to exclude from the returned `_source` field."]
    pub fn _source_excludes(mut self, _source_excludes: &'b [&'b str]) -> Self {
        self._source_excludes = Some(_source_excludes);
        self
    }
    #[doc = "List of fields to extract and return from the `_source` field."]
    pub fn _source_includes(mut self, _source_includes: &'b [&'b str]) -> Self {
        self._source_includes = Some(_source_includes);
        self
    }
    #[doc = "Whether to ignore if a wildcard indexes expression resolves into no concrete indexes. (This includes `_all` string or when no indexes have been specified)."]
    pub fn allow_no_indices(mut self, allow_no_indices: bool) -> Self {
        self.allow_no_indices = Some(allow_no_indices);
        self
    }
    #[doc = "Indicate if an error should be returned if there is a partial search failure or timeout."]
    pub fn allow_partial_search_results(mut self, allow_partial_search_results: bool) -> Self {
        self.allow_partial_search_results = Some(allow_partial_search_results);
        self
    }
    #[doc = "Specify whether wildcard and prefix queries should be analyzed."]
    pub fn analyze_wildcard(mut self, analyze_wildcard: bool) -> Self {
        self.analyze_wildcard = Some(analyze_wildcard);
        self
    }
    #[doc = "The analyzer to use for the query string."]
    pub fn analyzer(mut self, analyzer: &'b str) -> Self {
        self.analyzer = Some(analyzer);
        self
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node. This value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(mut self, batched_reduce_size: i64) -> Self {
        self.batched_reduce_size = Some(batched_reduce_size);
        self
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> KnnSearchModels<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        KnnSearchModels {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            _source: self._source,
            _source_excludes: self._source_excludes,
            _source_includes: self._source_includes,
            allow_no_indices: self.allow_no_indices,
            allow_partial_search_results: self.allow_partial_search_results,
            analyze_wildcard: self.analyze_wildcard,
            analyzer: self.analyzer,
            batched_reduce_size: self.batched_reduce_size,
            ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
            default_operator: self.default_operator,
            df: self.df,
            docvalue_fields: self.docvalue_fields,
            error_trace: self.error_trace,
            expand_wildcards: self.expand_wildcards,
            explain: self.explain,
            filter_path: self.filter_path,
            from: self.from,
            headers: self.headers,
            human: self.human,
            ignore_throttled: self.ignore_throttled,
            ignore_unavailable: self.ignore_unavailable,
            lenient: self.lenient,
            max_concurrent_shard_requests: self.max_concurrent_shard_requests,
            pre_filter_shard_size: self.pre_filter_shard_size,
            preference: self.preference,
            pretty: self.pretty,
            q: self.q,
            request_cache: self.request_cache,
            request_timeout: self.request_timeout,
            rest_total_hits_as_int: self.rest_total_hits_as_int,
            routing: self.routing,
            scroll: self.scroll,
            search_type: self.search_type,
            seq_no_primary_term: self.seq_no_primary_term,
            size: self.size,
            sort: self.sort,
            source: self.source,
            stats: self.stats,
            stored_fields: self.stored_fields,
            suggest_field: self.suggest_field,
            suggest_mode: self.suggest_mode,
            suggest_size: self.suggest_size,
            suggest_text: self.suggest_text,
            terminate_after: self.terminate_after,
            timeout: self.timeout,
            track_scores: self.track_scores,
            track_total_hits: self.track_total_hits,
            typed_keys: self.typed_keys,
            version: self.version,
        }
    }
    #[doc = "Indicates whether network round-trips should be minimized as part of cross-cluster search requests execution."]
    pub fn ccs_minimize_roundtrips(mut self, ccs_minimize_roundtrips: bool) -> Self {
        self.ccs_minimize_roundtrips = Some(ccs_minimize_roundtrips);
        self
    }
    #[doc = "The default operator for query string query (AND or OR)."]
    pub fn default_operator(mut self, default_operator: DefaultOperator) -> Self {
        self.default_operator = Some(default_operator);
        self
    }
    #[doc = "The field to use as default where no field prefix is given in the query string."]
    pub fn df(mut self, df: &'b str) -> Self {
        self.df = Some(df);
        self
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation of a field for each hit."]
    pub fn docvalue_fields(mut self, docvalue_fields: &'b [&'b str]) -> Self {
        self.docvalue_fields = Some(docvalue_fields);
        self
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(mut self, error_trace: bool) -> Self {
        self.error_trace = Some(error_trace);
        self
    }
    #[doc = "Whether to expand wildcard expression to concrete indexes that are open, closed or both."]
    pub fn expand_wildcards(mut self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        self.expand_wildcards = Some(expand_wildcards);
        self
    }
    #[doc = "Specify whether to return detailed information about score computation as part of a hit."]
    pub fn explain(mut self, explain: bool) -> Self {
        self.explain = Some(explain);
        self
    }
    #[doc = "A comma-separated list of filters used to filter the response. Use wildcards to match any field or part of a field's name. To exclude fields, use `-`."]
    pub fn filter_path(mut self, filter_path: &'b [&'b str]) -> Self {
        self.filter_path = Some(filter_path);
        self
    }
    #[doc = "Starting offset."]
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
    #[doc = "Whether specified concrete, expanded or aliased indexes should be ignored when throttled."]
    pub fn ignore_throttled(mut self, ignore_throttled: bool) -> Self {
        self.ignore_throttled = Some(ignore_throttled);
        self
    }
    #[doc = "Whether specified concrete indexes should be ignored when unavailable (missing or closed)."]
    pub fn ignore_unavailable(mut self, ignore_unavailable: bool) -> Self {
        self.ignore_unavailable = Some(ignore_unavailable);
        self
    }
    #[doc = "Specify whether format-based query failures (such as providing text to a numeric field) should be ignored."]
    pub fn lenient(mut self, lenient: bool) -> Self {
        self.lenient = Some(lenient);
        self
    }
    #[doc = "The number of concurrent shard requests per node this search executes concurrently. This value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests."]
    pub fn max_concurrent_shard_requests(mut self, max_concurrent_shard_requests: i64) -> Self {
        self.max_concurrent_shard_requests = Some(max_concurrent_shard_requests);
        self
    }
    #[doc = "Threshold that enforces a pre-filter round-trip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold. This filter round-trip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method, that is if date filters are mandatory to match but the shard bounds and the query are disjoint."]
    pub fn pre_filter_shard_size(mut self, pre_filter_shard_size: i64) -> Self {
        self.pre_filter_shard_size = Some(pre_filter_shard_size);
        self
    }
    #[doc = "Specify the node or shard the operation should be performed on."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    #[doc = "Query in the Lucene query string syntax."]
    pub fn q(mut self, q: &'b str) -> Self {
        self.q = Some(q);
        self
    }
    #[doc = "Specify if request cache should be used for this request or not, defaults to index level setting."]
    pub fn request_cache(mut self, request_cache: bool) -> Self {
        self.request_cache = Some(request_cache);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    #[doc = "Indicates whether `hits.total` should be rendered as an integer or an object in the rest search response."]
    pub fn rest_total_hits_as_int(mut self, rest_total_hits_as_int: bool) -> Self {
        self.rest_total_hits_as_int = Some(rest_total_hits_as_int);
        self
    }
    #[doc = "A comma-separated list of specific routing values."]
    pub fn routing(mut self, routing: &'b [&'b str]) -> Self {
        self.routing = Some(routing);
        self
    }
    #[doc = "Specify how long a consistent view of the index should be maintained for scrolled search."]
    pub fn scroll(mut self, scroll: &'b str) -> Self {
        self.scroll = Some(scroll);
        self
    }
    #[doc = "Search operation type."]
    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.search_type = Some(search_type);
        self
    }
    #[doc = "Specify whether to return sequence number and primary term of the last modification of each hit."]
    pub fn seq_no_primary_term(mut self, seq_no_primary_term: bool) -> Self {
        self.seq_no_primary_term = Some(seq_no_primary_term);
        self
    }
    #[doc = "Number of hits to return."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "A comma-separated list of &lt;field&gt;:&lt;direction&gt; pairs."]
    pub fn sort(mut self, sort: &'b [&'b str]) -> Self {
        self.sort = Some(sort);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "Specific 'tag' of the request for logging and statistical purposes."]
    pub fn stats(mut self, stats: &'b [&'b str]) -> Self {
        self.stats = Some(stats);
        self
    }
    #[doc = "A comma-separated list of stored fields to return."]
    pub fn stored_fields(mut self, stored_fields: &'b [&'b str]) -> Self {
        self.stored_fields = Some(stored_fields);
        self
    }
    #[doc = "Specify which field to use for suggestions."]
    pub fn suggest_field(mut self, suggest_field: &'b str) -> Self {
        self.suggest_field = Some(suggest_field);
        self
    }
    #[doc = "Specify suggest mode."]
    pub fn suggest_mode(mut self, suggest_mode: SuggestMode) -> Self {
        self.suggest_mode = Some(suggest_mode);
        self
    }
    #[doc = "How many suggestions to return in response."]
    pub fn suggest_size(mut self, suggest_size: i64) -> Self {
        self.suggest_size = Some(suggest_size);
        self
    }
    #[doc = "The source text for which the suggestions should be returned."]
    pub fn suggest_text(mut self, suggest_text: &'b str) -> Self {
        self.suggest_text = Some(suggest_text);
        self
    }
    #[doc = "The maximum number of documents to collect for each shard, upon reaching which the query execution will terminate early."]
    pub fn terminate_after(mut self, terminate_after: i64) -> Self {
        self.terminate_after = Some(terminate_after);
        self
    }
    #[doc = "Operation timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Whether to calculate and return scores even if they are not used for sorting."]
    pub fn track_scores(mut self, track_scores: bool) -> Self {
        self.track_scores = Some(track_scores);
        self
    }
    #[doc = "Indicate if the number of documents that match the query should be tracked."]
    pub fn track_total_hits<T: Into<TrackTotalHits>>(mut self, track_total_hits: T) -> Self {
        self.track_total_hits = Some(track_total_hits.into());
        self
    }
    #[doc = "Specify whether aggregation and suggester names should be prefixed by their respective types in the response."]
    pub fn typed_keys(mut self, typed_keys: bool) -> Self {
        self.typed_keys = Some(typed_keys);
        self
    }
    #[doc = "Whether to return document version as part of a hit."]
    pub fn version(mut self, version: bool) -> Self {
        self.version = Some(version);
        self
    }
    #[doc = "Creates an asynchronous call to the Knn Search Models API that can be awaited"]
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
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_excludes: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                _source_includes: Option<&'b [&'b str]>,
                allow_no_indices: Option<bool>,
                allow_partial_search_results: Option<bool>,
                analyze_wildcard: Option<bool>,
                analyzer: Option<&'b str>,
                batched_reduce_size: Option<i64>,
                ccs_minimize_roundtrips: Option<bool>,
                default_operator: Option<DefaultOperator>,
                df: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                docvalue_fields: Option<&'b [&'b str]>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                expand_wildcards: Option<&'b [ExpandWildcards]>,
                explain: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                from: Option<i64>,
                human: Option<bool>,
                ignore_throttled: Option<bool>,
                ignore_unavailable: Option<bool>,
                lenient: Option<bool>,
                max_concurrent_shard_requests: Option<i64>,
                pre_filter_shard_size: Option<i64>,
                preference: Option<&'b str>,
                pretty: Option<bool>,
                q: Option<&'b str>,
                request_cache: Option<bool>,
                rest_total_hits_as_int: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                routing: Option<&'b [&'b str]>,
                scroll: Option<&'b str>,
                search_type: Option<SearchType>,
                seq_no_primary_term: Option<bool>,
                size: Option<i64>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                sort: Option<&'b [&'b str]>,
                source: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stats: Option<&'b [&'b str]>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                stored_fields: Option<&'b [&'b str]>,
                suggest_field: Option<&'b str>,
                suggest_mode: Option<SuggestMode>,
                suggest_size: Option<i64>,
                suggest_text: Option<&'b str>,
                terminate_after: Option<i64>,
                timeout: Option<&'b str>,
                track_scores: Option<bool>,
                track_total_hits: Option<TrackTotalHits>,
                typed_keys: Option<bool>,
                version: Option<bool>,
            }
            let query_params = QueryParams {
                _source: self._source,
                _source_excludes: self._source_excludes,
                _source_includes: self._source_includes,
                allow_no_indices: self.allow_no_indices,
                allow_partial_search_results: self.allow_partial_search_results,
                analyze_wildcard: self.analyze_wildcard,
                analyzer: self.analyzer,
                batched_reduce_size: self.batched_reduce_size,
                ccs_minimize_roundtrips: self.ccs_minimize_roundtrips,
                default_operator: self.default_operator,
                df: self.df,
                docvalue_fields: self.docvalue_fields,
                error_trace: self.error_trace,
                expand_wildcards: self.expand_wildcards,
                explain: self.explain,
                filter_path: self.filter_path,
                from: self.from,
                human: self.human,
                ignore_throttled: self.ignore_throttled,
                ignore_unavailable: self.ignore_unavailable,
                lenient: self.lenient,
                max_concurrent_shard_requests: self.max_concurrent_shard_requests,
                pre_filter_shard_size: self.pre_filter_shard_size,
                preference: self.preference,
                pretty: self.pretty,
                q: self.q,
                request_cache: self.request_cache,
                rest_total_hits_as_int: self.rest_total_hits_as_int,
                routing: self.routing,
                scroll: self.scroll,
                search_type: self.search_type,
                seq_no_primary_term: self.seq_no_primary_term,
                size: self.size,
                sort: self.sort,
                source: self.source,
                stats: self.stats,
                stored_fields: self.stored_fields,
                suggest_field: self.suggest_field,
                suggest_mode: self.suggest_mode,
                suggest_size: self.suggest_size,
                suggest_text: self.suggest_text,
                terminate_after: self.terminate_after,
                timeout: self.timeout,
                track_scores: self.track_scores,
                track_total_hits: self.track_total_hits,
                typed_keys: self.typed_keys,
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
#[doc = "API parts for the Knn Stats API"]
pub enum KnnStatsParts<'b> {
    #[doc = "Stat"]
    Stat(&'b str),
    #[doc = "NodeId and Stat"]
    NodeIdStat(&'b [&'b str], &'b str),
    #[doc = "No parts"]
    None,
    #[doc = "NodeId"]
    NodeId(&'b [&'b str]),
}
impl<'b> KnnStatsParts<'b> {
    #[doc = "Builds a relative URL path to the Knn Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnStatsParts::Stat(stat) => {
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(24usize + encoded_stat.len());
                p.push_str("/_opendistro/_knn/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
            KnnStatsParts::NodeIdStat(node_id, stat) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let encoded_stat: Cow<str> = percent_encode(stat.as_bytes(), PARTS_ENCODED).into();
                let mut p =
                    String::with_capacity(25usize + encoded_node_id.len() + encoded_stat.len());
                p.push_str("/_opendistro/_knn/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats/");
                p.push_str(encoded_stat.as_ref());
                p.into()
            }
            KnnStatsParts::None => "/_plugins/_knn/stats".into(),
            KnnStatsParts::NodeId(node_id) => {
                let node_id_str = node_id.join(",");
                let encoded_node_id: Cow<str> =
                    percent_encode(node_id_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(21usize + encoded_node_id.len());
                p.push_str("/_plugins/_knn/");
                p.push_str(encoded_node_id.as_ref());
                p.push_str("/stats");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Knn Stats API](https://docs.opensearch.org/latest/vector-search/api/knn/#stats)\n\nProvides information about the current status of the k-NN plugin."]
#[derive(Clone, Debug)]
pub struct KnnStats<'a, 'b> {
    transport: &'a Transport,
    parts: KnnStatsParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    timeout: Option<&'b str>,
}
impl<'a, 'b> KnnStats<'a, 'b> {
    #[doc = "Creates a new instance of [KnnStats] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: KnnStatsParts<'b>) -> Self {
        let headers = HeaderMap::new();
        KnnStats {
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
    #[doc = "Operation timeout."]
    pub fn timeout(mut self, timeout: &'b str) -> Self {
        self.timeout = Some(timeout);
        self
    }
    #[doc = "Creates an asynchronous call to the Knn Stats API that can be awaited"]
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
#[doc = "API parts for the Knn Train Model API"]
pub enum KnnTrainModelParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "ModelId"]
    ModelId(&'b str),
}
impl<'b> KnnTrainModelParts<'b> {
    #[doc = "Builds a relative URL path to the Knn Train Model API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnTrainModelParts::None => "/_plugins/_knn/models/_train".into(),
            KnnTrainModelParts::ModelId(model_id) => {
                let encoded_model_id: Cow<str> =
                    percent_encode(model_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(29usize + encoded_model_id.len());
                p.push_str("/_plugins/_knn/models/");
                p.push_str(encoded_model_id.as_ref());
                p.push_str("/_train");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Knn Train Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#train-a-model)\n\nCreate and train a model that can be used for initializing k-NN native library indexes during indexing."]
#[derive(Clone, Debug)]
pub struct KnnTrainModel<'a, 'b, B> {
    transport: &'a Transport,
    parts: KnnTrainModelParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    preference: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> KnnTrainModel<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [KnnTrainModel] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: KnnTrainModelParts<'b>) -> Self {
        let headers = HeaderMap::new();
        KnnTrainModel {
            transport,
            parts,
            headers,
            body: None,
            error_trace: None,
            filter_path: None,
            human: None,
            preference: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> KnnTrainModel<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        KnnTrainModel {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            preference: self.preference,
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
    #[doc = "Preferred node to execute training."]
    pub fn preference(mut self, preference: &'b str) -> Self {
        self.preference = Some(preference);
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
    #[doc = "Creates an asynchronous call to the Knn Train Model API that can be awaited"]
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
                preference: Option<&'b str>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                preference: self.preference,
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
#[doc = "API parts for the Knn Warmup API"]
pub enum KnnWarmupParts<'b> {
    #[doc = "Index"]
    Index(&'b [&'b str]),
}
impl<'b> KnnWarmupParts<'b> {
    #[doc = "Builds a relative URL path to the Knn Warmup API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            KnnWarmupParts::Index(index) => {
                let index_str = index.join(",");
                let encoded_index: Cow<str> =
                    percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(25usize + encoded_index.len());
                p.push_str("/_opendistro/_knn/warmup/");
                p.push_str(encoded_index.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Knn Warmup API](https://docs.opensearch.org/latest/vector-search/api/knn/#warmup-operation)\n\nPreloads native library files into memory, reducing initial search latency for specified indexes."]
#[derive(Clone, Debug)]
pub struct KnnWarmup<'a, 'b> {
    transport: &'a Transport,
    parts: KnnWarmupParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> KnnWarmup<'a, 'b> {
    #[doc = "Creates a new instance of [KnnWarmup] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: KnnWarmupParts<'b>) -> Self {
        let headers = HeaderMap::new();
        KnnWarmup {
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
    #[doc = "Creates an asynchronous call to the Knn Warmup API that can be awaited"]
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
#[doc = "Namespace client for Knn APIs"]
pub struct Knn<'a> {
    transport: &'a Transport,
}
impl<'a> Knn<'a> {
    #[doc = "Creates a new instance of [Knn]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Knn Delete Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#delete-a-model)\n\nUsed to delete a particular model in the cluster."]
    pub fn delete_model<'b>(&'a self, parts: KnnDeleteModelParts<'b>) -> KnnDeleteModel<'a, 'b> {
        KnnDeleteModel::new(self.transport(), parts)
    }
    #[doc = "[Knn Get Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#get-a-model)\n\nUsed to retrieve information about models present in the cluster."]
    pub fn get_model<'b>(&'a self, parts: KnnGetModelParts<'b>) -> KnnGetModel<'a, 'b> {
        KnnGetModel::new(self.transport(), parts)
    }
    #[doc = "[Knn Search Models API](https://docs.opensearch.org/latest/vector-search/api/knn/#search-for-a-model)\n\nUse an OpenSearch query to search for models in the index."]
    pub fn search_models<'b>(&'a self) -> KnnSearchModels<'a, 'b, ()> {
        KnnSearchModels::new(self.transport())
    }
    #[doc = "[Knn Stats API](https://docs.opensearch.org/latest/vector-search/api/knn/#stats)\n\nProvides information about the current status of the k-NN plugin."]
    pub fn stats<'b>(&'a self, parts: KnnStatsParts<'b>) -> KnnStats<'a, 'b> {
        KnnStats::new(self.transport(), parts)
    }
    #[doc = "[Knn Train Model API](https://docs.opensearch.org/latest/vector-search/api/knn/#train-a-model)\n\nCreate and train a model that can be used for initializing k-NN native library indexes during indexing."]
    pub fn train_model<'b>(&'a self, parts: KnnTrainModelParts<'b>) -> KnnTrainModel<'a, 'b, ()> {
        KnnTrainModel::new(self.transport(), parts)
    }
    #[doc = "[Knn Warmup API](https://docs.opensearch.org/latest/vector-search/api/knn/#warmup-operation)\n\nPreloads native library files into memory, reducing initial search latency for specified indexes."]
    pub fn warmup<'b>(&'a self, parts: KnnWarmupParts<'b>) -> KnnWarmup<'a, 'b> {
        KnnWarmup::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Knn APIs"]
    pub fn knn(&self) -> Knn {
        Knn::new(self.transport())
    }
}
