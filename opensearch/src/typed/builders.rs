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
use super::TypedOpenSearch;
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
#[doc = "Typed builder for the Bulk API: [`send`](Self::send) returns [`types::bulk::BulkResponse`](crate::types::bulk::BulkResponse).\n\nOptions that change the shape of the response are not available here: `filter_path`. Use the generic [`Bulk`](crate::Bulk) builder for them."]
#[derive(Clone, Debug)]
pub struct TypedBulk<'a, 'b, B> {
    inner: crate::Bulk<'a, 'b, B>,
    report_unknown_fields: bool,
}
#[allow(deprecated)]
impl<'a, 'b, B> TypedBulk<'a, 'b, B>
where
    B: Body,
{
    #[doc = "`true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(self, _source: &'b str) -> Self {
        Self {
            inner: self.inner._source(_source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to exclude from the response."]
    pub fn _source_excludes(self, _source_excludes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_excludes(_source_excludes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(self, _source_includes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_includes(_source_includes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: Vec<T>) -> TypedBulk<'a, 'b, NdBody<T>>
    where
        T: Body,
    {
        TypedBulk {
            inner: self.inner.body(body),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(self, error_trace: bool) -> Self {
        Self {
            inner: self.inner.error_trace(error_trace),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(self, key: HeaderName, value: HeaderValue) -> Self {
        Self {
            inner: self.inner.header(key, value),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(self, human: bool) -> Self {
        Self {
            inner: self.inner.human(human),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Name of the data stream, index, or index alias to perform bulk actions on."]
    pub fn index(self, index: &'b str) -> Self {
        Self {
            inner: self.inner.index(index),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(self, pipeline: &'b str) -> Self {
        Self {
            inner: self.inner.pipeline(pipeline),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(self, pretty: bool) -> Self {
        Self {
            inner: self.inner.pretty(pretty),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(self, refresh: Refresh) -> Self {
        Self {
            inner: self.inner.refresh(refresh),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(self, timeout: Duration) -> Self {
        Self {
            inner: self.inner.request_timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, the request's actions must target an index alias."]
    pub fn require_alias(self, require_alias: bool) -> Self {
        Self {
            inner: self.inner.require_alias(require_alias),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(self, routing: &'b str) -> Self {
        Self {
            inner: self.inner.routing(routing),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(self, source: &'b str) -> Self {
        Self {
            inner: self.inner.source(source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Period each action waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(self, timeout: &'b str) -> Self {
        Self {
            inner: self.inner.timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Default document type for items which don't provide one."]
    pub fn ty(self, ty: &'b str) -> Self {
        Self {
            inner: self.inner.ty(ty),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(self, wait_for_active_shards: &'b str) -> Self {
        Self {
            inner: self.inner.wait_for_active_shards(wait_for_active_shards),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sends the request and decodes the response body as [`types::bulk::BulkResponse`](crate::types::bulk::BulkResponse).\n\nThe body is decoded for a success status. Any other status is an error, whose details are available from [`Error::api_error`](crate::Error::api_error). To read the status code or the headers, use the generic client instead."]
    pub async fn send(self) -> Result<crate::types::bulk::BulkResponse, Error> {
        self.inner
            .send()
            .await?
            .typed("types::bulk::BulkResponse", &[], self.report_unknown_fields)
            .await
    }
}
#[doc = "Typed builder for the Get API: [`send`](Self::send) returns [`types::get::GetResponse`](crate::types::get::GetResponse).\n\nOptions that change the shape of the response are not available here: `filter_path`. Use the generic [`Get`](crate::Get) builder for them."]
#[derive(Clone, Debug)]
pub struct TypedGet<'a, 'b> {
    inner: crate::Get<'a, 'b>,
    report_unknown_fields: bool,
}
#[allow(deprecated)]
impl<'a, 'b> TypedGet<'a, 'b> {
    #[doc = "Set to `true` or `false` to return the `_source` field or not, or a list of fields to return."]
    pub fn _source(self, _source: &'b str) -> Self {
        Self {
            inner: self.inner._source(_source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to exclude in the response."]
    pub fn _source_excludes(self, _source_excludes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_excludes(_source_excludes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to include in the response."]
    pub fn _source_includes(self, _source_includes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_includes(_source_includes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(self, error_trace: bool) -> Self {
        Self {
            inner: self.inner.error_trace(error_trace),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(self, key: HeaderName, value: HeaderValue) -> Self {
        Self {
            inner: self.inner.header(key, value),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(self, human: bool) -> Self {
        Self {
            inner: self.inner.human(human),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Specifies the node or shard the operation should be performed on. Random by default."]
    pub fn preference(self, preference: &'b str) -> Self {
        Self {
            inner: self.inner.preference(preference),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(self, pretty: bool) -> Self {
        Self {
            inner: self.inner.pretty(pretty),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, the request is real time as opposed to near real time."]
    pub fn realtime(self, realtime: bool) -> Self {
        Self {
            inner: self.inner.realtime(realtime),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search. If `false`, do nothing with refreshes."]
    pub fn refresh(self, refresh: Refresh) -> Self {
        Self {
            inner: self.inner.refresh(refresh),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(self, timeout: Duration) -> Self {
        Self {
            inner: self.inner.request_timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Target the specified primary shard."]
    pub fn routing(self, routing: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.routing(routing),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(self, source: &'b str) -> Self {
        Self {
            inner: self.inner.source(source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "List of stored fields to return as part of a hit.\nIf no fields are specified, no stored fields are included in the response.\nIf this field is specified, the `_source` parameter defaults to false."]
    pub fn stored_fields(self, stored_fields: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.stored_fields(stored_fields),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Explicit version number for concurrency control. The specified version must match the current version of the document for the request to succeed."]
    pub fn version(self, version: i64) -> Self {
        Self {
            inner: self.inner.version(version),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The specific version type: `internal`, `external`, `external_gte`."]
    pub fn version_type(self, version_type: VersionType) -> Self {
        Self {
            inner: self.inner.version_type(version_type),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sends the request and decodes the response body as [`types::get::GetResponse`](crate::types::get::GetResponse).\n\nThe body is decoded for a success status, and for `404`, for which the API answers with the same body (e.g. a document that is not found). Any other status is an error, whose details are available from [`Error::api_error`](crate::Error::api_error). To read the status code or the headers, use the generic client instead."]
    pub async fn send(self) -> Result<crate::types::get::GetResponse, Error> {
        self.inner
            .send()
            .await?
            .typed(
                "types::get::GetResponse",
                &[404u16],
                self.report_unknown_fields,
            )
            .await
    }
}
#[doc = "Typed builder for the Index API: [`send`](Self::send) returns [`types::index::IndexResponse`](crate::types::index::IndexResponse).\n\nOptions that change the shape of the response are not available here: `filter_path`. Use the generic [`Index`](crate::Index) builder for them."]
#[derive(Clone, Debug)]
pub struct TypedIndex<'a, 'b, B> {
    inner: crate::Index<'a, 'b, B>,
    report_unknown_fields: bool,
}
#[allow(deprecated)]
impl<'a, 'b, B> TypedIndex<'a, 'b, B>
where
    B: Body,
{
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TypedIndex<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TypedIndex {
            inner: self.inner.body(body),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(self, error_trace: bool) -> Self {
        Self {
            inner: self.inner.error_trace(error_trace),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(self, key: HeaderName, value: HeaderValue) -> Self {
        Self {
            inner: self.inner.header(key, value),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(self, human: bool) -> Self {
        Self {
            inner: self.inner.human(human),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Only perform the operation if the document has this primary term."]
    pub fn if_primary_term(self, if_primary_term: i64) -> Self {
        Self {
            inner: self.inner.if_primary_term(if_primary_term),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Only perform the operation if the document has this sequence number."]
    pub fn if_seq_no(self, if_seq_no: i64) -> Self {
        Self {
            inner: self.inner.if_seq_no(if_seq_no),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Set to create to only index the document if it does not already exist (put if absent).\nIf a document with the specified `_id` already exists, the indexing operation will fail.\nSame as using the `&lt;index&gt;/_create` endpoint.\nValid values: `index`, `create`.\nIf document id is specified, it defaults to `index`.\nOtherwise, it defaults to `create`."]
    pub fn op_type(self, op_type: OpType) -> Self {
        Self {
            inner: self.inner.op_type(op_type),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "ID of the pipeline to use to preprocess incoming documents.\nIf the index has a default ingest pipeline specified, then setting the value to `_none` disables the default ingest pipeline for this request.\nIf a final pipeline is configured it will always run, regardless of the value of this parameter."]
    pub fn pipeline(self, pipeline: &'b str) -> Self {
        Self {
            inner: self.inner.pipeline(pipeline),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(self, pretty: bool) -> Self {
        Self {
            inner: self.inner.pretty(pretty),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, OpenSearch refreshes the affected shards to make this operation visible to search, if `wait_for` then wait for a refresh to make this operation visible to search, if `false` do nothing with refreshes.\nValid values: `true`, `false`, `wait_for`."]
    pub fn refresh(self, refresh: Refresh) -> Self {
        Self {
            inner: self.inner.refresh(refresh),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(self, timeout: Duration) -> Self {
        Self {
            inner: self.inner.request_timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, the destination must be an index alias."]
    pub fn require_alias(self, require_alias: bool) -> Self {
        Self {
            inner: self.inner.require_alias(require_alias),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(self, routing: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.routing(routing),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(self, source: &'b str) -> Self {
        Self {
            inner: self.inner.source(source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Period the request waits for the following operations: automatic index creation, dynamic mapping updates, waiting for active shards."]
    pub fn timeout(self, timeout: &'b str) -> Self {
        Self {
            inner: self.inner.timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Explicit version number for concurrency control.\nThe specified version must match the current version of the document for the request to succeed."]
    pub fn version(self, version: i64) -> Self {
        Self {
            inner: self.inner.version(version),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The specific version type: `external`, `external_gte`."]
    pub fn version_type(self, version_type: VersionType) -> Self {
        Self {
            inner: self.inner.version_type(version_type),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The number of shard copies that must be active before proceeding with the operation.\nSet to all or any positive integer up to the total number of shards in the index (`number_of_replicas+1`)."]
    pub fn wait_for_active_shards(self, wait_for_active_shards: &'b str) -> Self {
        Self {
            inner: self.inner.wait_for_active_shards(wait_for_active_shards),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sends the request and decodes the response body as [`types::index::IndexResponse`](crate::types::index::IndexResponse).\n\nThe body is decoded for a success status. Any other status is an error, whose details are available from [`Error::api_error`](crate::Error::api_error). To read the status code or the headers, use the generic client instead."]
    pub async fn send(self) -> Result<crate::types::index::IndexResponse, Error> {
        self.inner
            .send()
            .await?
            .typed(
                "types::index::IndexResponse",
                &[],
                self.report_unknown_fields,
            )
            .await
    }
}
#[doc = "Typed builder for the Search API: [`send`](Self::send) returns [`types::search::SearchResponse`](crate::types::search::SearchResponse).\n\nOptions that change the shape of the response are not available here: `filter_path`. Use the generic [`Search`](crate::Search) builder for them."]
#[derive(Clone, Debug)]
pub struct TypedSearch<'a, 'b, B> {
    inner: crate::Search<'a, 'b, B>,
    report_unknown_fields: bool,
}
#[allow(deprecated)]
impl<'a, 'b, B> TypedSearch<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Indicates which source fields are returned for matching documents.\nThese fields are returned in the `hits._source` property of the search response.\nValid values are:\n`true` to return the entire document source;\n`false` to not return the document source;\n`&lt;string&gt;` to return the source fields that are specified as a comma-separated list (supports wildcard (`*`) patterns)."]
    pub fn _source(self, _source: &'b str) -> Self {
        Self {
            inner: self.inner._source(_source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to exclude from the response.\nYou can also use this parameter to exclude fields from the subset specified in `_source_includes` query parameter.\nIf the `_source` parameter is `false`, this parameter is ignored."]
    pub fn _source_excludes(self, _source_excludes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_excludes(_source_excludes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of source fields to include in the response.\nIf this parameter is specified, only these source fields are returned.\nYou can exclude fields from this subset using the `_source_excludes` query parameter.\nIf the `_source` parameter is `false`, this parameter is ignored."]
    pub fn _source_includes(self, _source_includes: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner._source_includes(_source_includes),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `false`, the request returns an error if any wildcard expression, index alias, or `_all` value targets only missing or closed indexes.\nThis behavior applies even if the request targets other open indexes.\nFor example, a request targeting `foo*,bar*` returns an error if an index starts with `foo` but no index starts with `bar`."]
    pub fn allow_no_indices(self, allow_no_indices: bool) -> Self {
        Self {
            inner: self.inner.allow_no_indices(allow_no_indices),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, returns partial results if there are shard request timeouts or shard failures. If `false`, returns an error with no partial results."]
    pub fn allow_partial_search_results(self, allow_partial_search_results: bool) -> Self {
        Self {
            inner: self
                .inner
                .allow_partial_search_results(allow_partial_search_results),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, wildcard and prefix queries are analyzed.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn analyze_wildcard(self, analyze_wildcard: bool) -> Self {
        Self {
            inner: self.inner.analyze_wildcard(analyze_wildcard),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Analyzer to use for the query string.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn analyzer(self, analyzer: &'b str) -> Self {
        Self {
            inner: self.inner.analyzer(analyzer),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The number of shard results that should be reduced at once on the coordinating node.\nThis value should be used as a protection mechanism to reduce the memory overhead per search request if the potential number of shards in the request can be large."]
    pub fn batched_reduce_size(self, batched_reduce_size: i64) -> Self {
        Self {
            inner: self.inner.batched_reduce_size(batched_reduce_size),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> TypedSearch<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        TypedSearch {
            inner: self.inner.body(body),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The time after which the search request will be canceled.\nRequest-level parameter takes precedence over `cancel_after_time_interval` cluster setting."]
    pub fn cancel_after_time_interval(self, cancel_after_time_interval: &'b str) -> Self {
        Self {
            inner: self
                .inner
                .cancel_after_time_interval(cancel_after_time_interval),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, network round-trips between the coordinating node and the remote clusters are minimized when executing cross-cluster search (CCS) requests."]
    pub fn ccs_minimize_roundtrips(self, ccs_minimize_roundtrips: bool) -> Self {
        Self {
            inner: self.inner.ccs_minimize_roundtrips(ccs_minimize_roundtrips),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The default operator for query string query: AND or OR.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn default_operator(self, default_operator: DefaultOperator) -> Self {
        Self {
            inner: self.inner.default_operator(default_operator),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Field to use as default where no field prefix is given in the query string.\nThis parameter can only be used when the q query string parameter is specified."]
    pub fn df(self, df: &'b str) -> Self {
        Self {
            inner: self.inner.df(df),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of fields to return as the docvalue representation for each hit."]
    pub fn docvalue_fields(self, docvalue_fields: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.docvalue_fields(docvalue_fields),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to include the stack trace of returned errors."]
    pub fn error_trace(self, error_trace: bool) -> Self {
        Self {
            inner: self.inner.error_trace(error_trace),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Type of index that wildcard patterns can match.\nIf the request can target data streams, this argument determines whether wildcard expressions match hidden data streams.\nSupports comma-separated values, such as `open,hidden`."]
    pub fn expand_wildcards(self, expand_wildcards: &'b [ExpandWildcards]) -> Self {
        Self {
            inner: self.inner.expand_wildcards(expand_wildcards),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, returns detailed information about score computation as part of a hit."]
    pub fn explain(self, explain: bool) -> Self {
        Self {
            inner: self.inner.explain(explain),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Starting document offset.\nNeeds to be non-negative.\nBy default, you cannot page through more than 10,000 hits using the `from` and `size` parameters.\nTo page through more hits, use the `search_after` parameter."]
    pub fn from(self, from: i64) -> Self {
        Self {
            inner: self.inner.from(from),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Adds a HTTP header"]
    pub fn header(self, key: HeaderName, value: HeaderValue) -> Self {
        Self {
            inner: self.inner.header(key, value),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to return human-readable values for statistics."]
    pub fn human(self, human: bool) -> Self {
        Self {
            inner: self.inner.human(human),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, concrete, expanded or aliased indexes will be ignored when frozen."]
    pub fn ignore_throttled(self, ignore_throttled: bool) -> Self {
        Self {
            inner: self.inner.ignore_throttled(ignore_throttled),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `false`, the request returns an error if it targets a missing or closed index."]
    pub fn ignore_unavailable(self, ignore_unavailable: bool) -> Self {
        Self {
            inner: self.inner.ignore_unavailable(ignore_unavailable),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Indicates whether `hit.matched_queries` should be rendered as a map that includes the name of the matched query associated with its score (true) or as an array containing the name of the matched queries (false)"]
    pub fn include_named_queries_score(self, include_named_queries_score: bool) -> Self {
        Self {
            inner: self
                .inner
                .include_named_queries_score(include_named_queries_score),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of data streams, indexes, and aliases to search.\nSupports wildcards (`*`).\nTo search all data streams and indexes, omit this parameter or use `*` or `_all`."]
    pub fn index(self, index: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.index(index),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, format-based query failures (such as providing text to a numeric field) in the query string will be ignored.\nThis parameter can only be used when the `q` query string parameter is specified."]
    pub fn lenient(self, lenient: bool) -> Self {
        Self {
            inner: self.inner.lenient(lenient),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Defines the number of concurrent shard requests per node this search executes concurrently.\nThis value should be used to limit the impact of the search on the cluster in order to limit the number of concurrent shard requests."]
    pub fn max_concurrent_shard_requests(self, max_concurrent_shard_requests: i64) -> Self {
        Self {
            inner: self
                .inner
                .max_concurrent_shard_requests(max_concurrent_shard_requests),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Indicates whether to return phase-level `took` time values in the response."]
    pub fn phase_took(self, phase_took: bool) -> Self {
        Self {
            inner: self.inner.phase_took(phase_took),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Defines a threshold that enforces a pre-filter roundtrip to prefilter search shards based on query rewriting if the number of shards the search request expands to exceeds the threshold.\nThis filter roundtrip can limit the number of shards significantly if for instance a shard can not match any documents based on its rewrite method (if date filters are mandatory to match but the shard bounds and the query are disjoint).\nWhen unspecified, the pre-filter phase is executed if any of these conditions is met:\nthe request targets more than 128 shards;\nthe request targets one or more read-only index;\nthe primary sort of the query targets an indexed field."]
    pub fn pre_filter_shard_size(self, pre_filter_shard_size: i64) -> Self {
        Self {
            inner: self.inner.pre_filter_shard_size(pre_filter_shard_size),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Nodes and shards used for the search.\nBy default, OpenSearch selects from eligible nodes and shards using adaptive replica selection, accounting for allocation awareness. Valid values are:\n`_only_local` to run the search only on shards on the local node;\n`_local` to, if possible, run the search on shards on the local node, or if not, select shards using the default method;\n`_only_nodes:&lt;node-id&gt;,&lt;node-id&gt;` to run the search on only the specified nodes IDs, where, if suitable shards exist on more than one selected node, use shards on those nodes using the default method, or if none of the specified nodes are available, select shards from any available node using the default method;\n`_prefer_nodes:&lt;node-id&gt;,&lt;node-id&gt;` to if possible, run the search on the specified nodes IDs, or if not, select shards using the default method;\n`_shards:&lt;shard&gt;,&lt;shard&gt;` to run the search only on the specified shards;\n`&lt;custom-string&gt;` (any string that does not start with `_`) to route searches with the same `&lt;custom-string&gt;` to the same shards in the same order."]
    pub fn preference(self, preference: &'b str) -> Self {
        Self {
            inner: self.inner.preference(preference),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(self, pretty: bool) -> Self {
        Self {
            inner: self.inner.pretty(pretty),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Query in the Lucene query string syntax using query parameter search.\nQuery parameter searches do not support the full OpenSearch Query DSL but are handy for testing."]
    pub fn q(self, q: &'b str) -> Self {
        Self {
            inner: self.inner.q(q),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, the caching of search results is enabled for requests where `size` is `0`.\nDefaults to index level settings."]
    pub fn request_cache(self, request_cache: bool) -> Self {
        Self {
            inner: self.inner.request_cache(request_cache),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(self, timeout: Duration) -> Self {
        Self {
            inner: self.inner.request_timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Indicates whether `hits.total` should be rendered as an integer or an object in the rest search response."]
    pub fn rest_total_hits_as_int(self, rest_total_hits_as_int: bool) -> Self {
        Self {
            inner: self.inner.rest_total_hits_as_int(rest_total_hits_as_int),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A custom value used to route operations to a specific shard."]
    pub fn routing(self, routing: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.routing(routing),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Period to retain the search context for scrolling. See Scroll search results.\nBy default, this value cannot exceed `1d` (24 hours).\nYou can change this limit using the `search.max_keep_alive` cluster-level setting."]
    pub fn scroll(self, scroll: &'b str) -> Self {
        Self {
            inner: self.inner.scroll(scroll),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Customizable sequence of processing stages applied to search queries."]
    pub fn search_pipeline(self, search_pipeline: &'b str) -> Self {
        Self {
            inner: self.inner.search_pipeline(search_pipeline),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "How distributed term frequencies are calculated for relevance scoring."]
    pub fn search_type(self, search_type: SearchType) -> Self {
        Self {
            inner: self.inner.search_type(search_type),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, returns sequence number and primary term of the last modification of each hit."]
    pub fn seq_no_primary_term(self, seq_no_primary_term: bool) -> Self {
        Self {
            inner: self.inner.seq_no_primary_term(seq_no_primary_term),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Defines the number of hits to return.\nBy default, you cannot page through more than 10,000 hits using the `from` and `size` parameters.\nTo page through more hits, use the `search_after` parameter."]
    pub fn size(self, size: i64) -> Self {
        Self {
            inner: self.inner.size(size),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of &lt;field&gt;:&lt;direction&gt; pairs."]
    pub fn sort(self, sort: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.sort(sort),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(self, source: &'b str) -> Self {
        Self {
            inner: self.inner.source(source),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Specific `tag` of the request for logging and statistical purposes."]
    pub fn stats(self, stats: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.stats(stats),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "A comma-separated list of stored fields to return as part of a hit.\nIf no fields are specified, no stored fields are included in the response.\nIf this field is specified, the `_source` parameter defaults to `false`.\nYou can pass `_source: true` to return both source fields and stored fields in the search response."]
    pub fn stored_fields(self, stored_fields: &'b [&'b str]) -> Self {
        Self {
            inner: self.inner.stored_fields(stored_fields),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Specifies which field to use for suggestions."]
    pub fn suggest_field(self, suggest_field: &'b str) -> Self {
        Self {
            inner: self.inner.suggest_field(suggest_field),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Specifies the suggest mode.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_mode(self, suggest_mode: SuggestMode) -> Self {
        Self {
            inner: self.inner.suggest_mode(suggest_mode),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Number of suggestions to return.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_size(self, suggest_size: i64) -> Self {
        Self {
            inner: self.inner.suggest_size(suggest_size),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "The source text for which the suggestions should be returned.\nThis parameter can only be used when the `suggest_field` and `suggest_text` query string parameters are specified."]
    pub fn suggest_text(self, suggest_text: &'b str) -> Self {
        Self {
            inner: self.inner.suggest_text(suggest_text),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Maximum number of documents to collect for each shard.\nIf a query reaches this limit, OpenSearch terminates the query early.\nOpenSearch collects documents before sorting.\nUse with caution.\nOpenSearch applies this parameter to each shard handling the request.\nWhen possible, let OpenSearch perform early termination automatically.\nAvoid specifying this parameter for requests that target data streams with backing indexes across multiple data tiers.\nIf set to `0` (default), the query does not terminate early."]
    pub fn terminate_after(self, terminate_after: i64) -> Self {
        Self {
            inner: self.inner.terminate_after(terminate_after),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Specifies the period of time to wait for a response from each shard.\nIf no response is received before the timeout expires, the request fails and returns an error."]
    pub fn timeout(self, timeout: &'b str) -> Self {
        Self {
            inner: self.inner.timeout(timeout),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, calculate and return document scores, even if the scores are not used for sorting."]
    pub fn track_scores(self, track_scores: bool) -> Self {
        Self {
            inner: self.inner.track_scores(track_scores),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Number of hits matching the query to count accurately.\nIf `true`, the exact number of hits is returned at the cost of some performance.\nIf `false`, the response does not include the total number of hits matching the query."]
    pub fn track_total_hits<T: Into<TrackTotalHits>>(self, track_total_hits: T) -> Self {
        Self {
            inner: self.inner.track_total_hits(track_total_hits),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, aggregation and suggester names are be prefixed by their respective types in the response."]
    pub fn typed_keys(self, typed_keys: bool) -> Self {
        Self {
            inner: self.inner.typed_keys(typed_keys),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Enables or disables verbose mode for the search pipeline.\nWhen verbose mode is enabled, detailed information about each processor\nin the search pipeline is included in the search response. This includes\nthe processor name, execution status, input, output, and time taken for processing.\nThis parameter is primarily intended for debugging purposes, allowing users\nto track how data flows and transforms through the search pipeline."]
    pub fn verbose_pipeline(self, verbose_pipeline: bool) -> Self {
        Self {
            inner: self.inner.verbose_pipeline(verbose_pipeline),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "If `true`, returns document version as part of a hit."]
    pub fn version(self, version: bool) -> Self {
        Self {
            inner: self.inner.version(version),
            report_unknown_fields: self.report_unknown_fields,
        }
    }
    #[doc = "Sends the request and decodes the response body as [`types::search::SearchResponse`](crate::types::search::SearchResponse).\n\nThe body is decoded for a success status. Any other status is an error, whose details are available from [`Error::api_error`](crate::Error::api_error). To read the status code or the headers, use the generic client instead."]
    pub async fn send(self) -> Result<crate::types::search::SearchResponse, Error> {
        self.inner
            .send()
            .await?
            .typed(
                "types::search::SearchResponse",
                &[],
                self.report_unknown_fields,
            )
            .await
    }
}
impl TypedOpenSearch {
    #[doc = "[Bulk API](https://opensearch.org/docs/latest/api-reference/document-apis/bulk/)\n\nAllows to perform multiple index/update/delete operations in a single request."]
    #[allow(deprecated)]
    pub fn bulk<'a, 'b>(&'a self, parts: crate::BulkParts<'b>) -> TypedBulk<'a, 'b, ()> {
        TypedBulk {
            inner: crate::Bulk::new(self.transport(), parts),
            report_unknown_fields: self.reports_unknown_fields(),
        }
    }
    #[doc = "[Get API](https://opensearch.org/docs/latest/api-reference/document-apis/get-documents/)\n\nReturns a document."]
    #[allow(deprecated)]
    pub fn get<'a, 'b>(&'a self, parts: crate::GetParts<'b>) -> TypedGet<'a, 'b> {
        TypedGet {
            inner: crate::Get::new(self.transport(), parts),
            report_unknown_fields: self.reports_unknown_fields(),
        }
    }
    #[doc = "[Index API](https://opensearch.org/docs/latest/api-reference/document-apis/index-document/)\n\nCreates or updates a document in an index."]
    #[allow(deprecated)]
    pub fn index<'a, 'b>(&'a self, parts: crate::IndexParts<'b>) -> TypedIndex<'a, 'b, ()> {
        TypedIndex {
            inner: crate::Index::new(self.transport(), parts),
            report_unknown_fields: self.reports_unknown_fields(),
        }
    }
    #[doc = "[Search API](https://opensearch.org/docs/latest/api-reference/search/)\n\nReturns results matching a query."]
    #[allow(deprecated)]
    pub fn search<'a, 'b>(&'a self, parts: crate::SearchParts<'b>) -> TypedSearch<'a, 'b, ()> {
        TypedSearch {
            inner: crate::Search::new(self.transport(), parts),
            report_unknown_fields: self.reports_unknown_fields(),
        }
    }
}
