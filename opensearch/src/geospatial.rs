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
#[doc = "API parts for the Geospatial Delete Ip2 Geo Datasource API"]
pub enum GeospatialDeleteIp2GeoDatasourceParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> GeospatialDeleteIp2GeoDatasourceParts<'b> {
    #[doc = "Builds a relative URL path to the Geospatial Delete Ip2 Geo Datasource API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialDeleteIp2GeoDatasourceParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_name.len());
                p.push_str("/_plugins/geospatial/ip2geo/datasource/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Geospatial Delete Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#deleting-the-ip2geo-data-source)\n\nDelete a specific IP2Geo data source."]
#[derive(Clone, Debug)]
pub struct GeospatialDeleteIp2GeoDatasource<'a, 'b> {
    transport: &'a Transport,
    parts: GeospatialDeleteIp2GeoDatasourceParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GeospatialDeleteIp2GeoDatasource<'a, 'b> {
    #[doc = "Creates a new instance of [GeospatialDeleteIp2GeoDatasource] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GeospatialDeleteIp2GeoDatasourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GeospatialDeleteIp2GeoDatasource {
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
    #[doc = "Creates an asynchronous call to the Geospatial Delete Ip2 Geo Datasource API that can be awaited"]
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
#[doc = "API parts for the Geospatial Geojson Upload Post API"]
pub enum GeospatialGeojsonUploadPostParts {
    #[doc = "No parts"]
    None,
}
impl GeospatialGeojsonUploadPostParts {
    #[doc = "Builds a relative URL path to the Geospatial Geojson Upload Post API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialGeojsonUploadPostParts::None => "/_plugins/geospatial/geojson/_upload".into(),
        }
    }
}
#[doc = "Builder for the Geospatial Geojson Upload Post API\n\nUse an OpenSearch query to upload `GeoJSON`, operation will fail if index exists.\n- When type is `geo_point`, only Point geometry is allowed\n- When type is `geo_shape`, all geometry types are allowed (Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection, Envelope)."]
#[derive(Clone, Debug)]
pub struct GeospatialGeojsonUploadPost<'a, 'b, B> {
    transport: &'a Transport,
    parts: GeospatialGeojsonUploadPostParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> GeospatialGeojsonUploadPost<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [GeospatialGeojsonUploadPost]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GeospatialGeojsonUploadPost {
            transport,
            parts: GeospatialGeojsonUploadPostParts::None,
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
    pub fn body<T>(self, body: T) -> GeospatialGeojsonUploadPost<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        GeospatialGeojsonUploadPost {
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
    #[doc = "Creates an asynchronous call to the Geospatial Geojson Upload Post API that can be awaited"]
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
#[doc = "API parts for the Geospatial Geojson Upload Put API"]
pub enum GeospatialGeojsonUploadPutParts {
    #[doc = "No parts"]
    None,
}
impl GeospatialGeojsonUploadPutParts {
    #[doc = "Builds a relative URL path to the Geospatial Geojson Upload Put API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialGeojsonUploadPutParts::None => "/_plugins/geospatial/geojson/_upload".into(),
        }
    }
}
#[doc = "Builder for the Geospatial Geojson Upload Put API\n\nUse an OpenSearch query to upload `GeoJSON` regardless if index exists.\n- When type is `geo_point`, only Point geometry is allowed\n- When type is `geo_shape`, all geometry types are allowed (Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection, Envelope)."]
#[derive(Clone, Debug)]
pub struct GeospatialGeojsonUploadPut<'a, 'b, B> {
    transport: &'a Transport,
    parts: GeospatialGeojsonUploadPutParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> GeospatialGeojsonUploadPut<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [GeospatialGeojsonUploadPut]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GeospatialGeojsonUploadPut {
            transport,
            parts: GeospatialGeojsonUploadPutParts::None,
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
    pub fn body<T>(self, body: T) -> GeospatialGeojsonUploadPut<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        GeospatialGeojsonUploadPut {
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
    #[doc = "Creates an asynchronous call to the Geospatial Geojson Upload Put API that can be awaited"]
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
#[doc = "API parts for the Geospatial Get Ip2 Geo Datasource API"]
pub enum GeospatialGetIp2GeoDatasourceParts<'b> {
    #[doc = "No parts"]
    None,
    #[doc = "Name"]
    Name(&'b [&'b str]),
}
impl<'b> GeospatialGetIp2GeoDatasourceParts<'b> {
    #[doc = "Builds a relative URL path to the Geospatial Get Ip2 Geo Datasource API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialGetIp2GeoDatasourceParts::None => {
                "/_plugins/geospatial/ip2geo/datasource".into()
            }
            GeospatialGetIp2GeoDatasourceParts::Name(name) => {
                let name_str = name.join(",");
                let encoded_name: Cow<str> =
                    percent_encode(name_str.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_name.len());
                p.push_str("/_plugins/geospatial/ip2geo/datasource/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Geospatial Get Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#sending-a-get-request)\n\nGet one or more IP2Geo data sources, defaulting to returning all if no names specified."]
#[derive(Clone, Debug)]
pub struct GeospatialGetIp2GeoDatasource<'a, 'b> {
    transport: &'a Transport,
    parts: GeospatialGetIp2GeoDatasourceParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GeospatialGetIp2GeoDatasource<'a, 'b> {
    #[doc = "Creates a new instance of [GeospatialGetIp2GeoDatasource] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GeospatialGetIp2GeoDatasourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GeospatialGetIp2GeoDatasource {
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
    #[doc = "Creates an asynchronous call to the Geospatial Get Ip2 Geo Datasource API that can be awaited"]
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
#[doc = "API parts for the Geospatial Get Upload Stats API"]
pub enum GeospatialGetUploadStatsParts {
    #[doc = "No parts"]
    None,
}
impl GeospatialGetUploadStatsParts {
    #[doc = "Builds a relative URL path to the Geospatial Get Upload Stats API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialGetUploadStatsParts::None => "/_plugins/geospatial/_upload/stats".into(),
        }
    }
}
#[doc = "Builder for the Geospatial Get Upload Stats API\n\nRetrieves statistics for all geospatial uploads."]
#[derive(Clone, Debug)]
pub struct GeospatialGetUploadStats<'a, 'b> {
    transport: &'a Transport,
    parts: GeospatialGetUploadStatsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> GeospatialGetUploadStats<'a, 'b> {
    #[doc = "Creates a new instance of [GeospatialGetUploadStats]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        GeospatialGetUploadStats {
            transport,
            parts: GeospatialGetUploadStatsParts::None,
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
    #[doc = "Creates an asynchronous call to the Geospatial Get Upload Stats API that can be awaited"]
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
#[doc = "API parts for the Geospatial Put Ip2 Geo Datasource API"]
pub enum GeospatialPutIp2GeoDatasourceParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> GeospatialPutIp2GeoDatasourceParts<'b> {
    #[doc = "Builds a relative URL path to the Geospatial Put Ip2 Geo Datasource API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialPutIp2GeoDatasourceParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(39usize + encoded_name.len());
                p.push_str("/_plugins/geospatial/ip2geo/datasource/");
                p.push_str(encoded_name.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Geospatial Put Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#data-source-options)\n\nCreate a specific IP2Geo data source.\nDefault values:\n  - `endpoint`: `\"https://geoip.maps.opensearch.org/v1/geolite2-city/manifest.json\"`\n  - `update_interval_in_days`: 3."]
#[derive(Clone, Debug)]
pub struct GeospatialPutIp2GeoDatasource<'a, 'b, B> {
    transport: &'a Transport,
    parts: GeospatialPutIp2GeoDatasourceParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> GeospatialPutIp2GeoDatasource<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [GeospatialPutIp2GeoDatasource] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: GeospatialPutIp2GeoDatasourceParts<'b>) -> Self {
        let headers = HeaderMap::new();
        GeospatialPutIp2GeoDatasource {
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
    pub fn body<T>(self, body: T) -> GeospatialPutIp2GeoDatasource<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        GeospatialPutIp2GeoDatasource {
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
    #[doc = "Creates an asynchronous call to the Geospatial Put Ip2 Geo Datasource API that can be awaited"]
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
#[doc = "API parts for the Geospatial Put Ip2 Geo Datasource Settings API"]
pub enum GeospatialPutIp2GeoDatasourceSettingsParts<'b> {
    #[doc = "Name"]
    Name(&'b str),
}
impl<'b> GeospatialPutIp2GeoDatasourceSettingsParts<'b> {
    #[doc = "Builds a relative URL path to the Geospatial Put Ip2 Geo Datasource Settings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            GeospatialPutIp2GeoDatasourceSettingsParts::Name(name) => {
                let encoded_name: Cow<str> = percent_encode(name.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(49usize + encoded_name.len());
                p.push_str("/_plugins/geospatial/ip2geo/datasource/");
                p.push_str(encoded_name.as_ref());
                p.push_str("/_settings");
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Geospatial Put Ip2 Geo Datasource Settings API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#updating-an-ip2geo-data-source)\n\nUpdate a specific IP2Geo data source."]
#[derive(Clone, Debug)]
pub struct GeospatialPutIp2GeoDatasourceSettings<'a, 'b, B> {
    transport: &'a Transport,
    parts: GeospatialPutIp2GeoDatasourceSettingsParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> GeospatialPutIp2GeoDatasourceSettings<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [GeospatialPutIp2GeoDatasourceSettings] with the specified API parts"]
    pub fn new(
        transport: &'a Transport,
        parts: GeospatialPutIp2GeoDatasourceSettingsParts<'b>,
    ) -> Self {
        let headers = HeaderMap::new();
        GeospatialPutIp2GeoDatasourceSettings {
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
    pub fn body<T>(self, body: T) -> GeospatialPutIp2GeoDatasourceSettings<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        GeospatialPutIp2GeoDatasourceSettings {
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
    #[doc = "Creates an asynchronous call to the Geospatial Put Ip2 Geo Datasource Settings API that can be awaited"]
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
#[doc = "Namespace client for Geospatial APIs"]
pub struct Geospatial<'a> {
    transport: &'a Transport,
}
impl<'a> Geospatial<'a> {
    #[doc = "Creates a new instance of [Geospatial]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Geospatial Delete Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#deleting-the-ip2geo-data-source)\n\nDelete a specific IP2Geo data source."]
    pub fn delete_ip2geo_datasource<'b>(
        &'a self,
        parts: GeospatialDeleteIp2GeoDatasourceParts<'b>,
    ) -> GeospatialDeleteIp2GeoDatasource<'a, 'b> {
        GeospatialDeleteIp2GeoDatasource::new(self.transport(), parts)
    }
    #[doc = "Geospatial Geojson Upload Post API\n\nUse an OpenSearch query to upload `GeoJSON`, operation will fail if index exists.\n- When type is `geo_point`, only Point geometry is allowed\n- When type is `geo_shape`, all geometry types are allowed (Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection, Envelope)."]
    pub fn geojson_upload_post<'b>(&'a self) -> GeospatialGeojsonUploadPost<'a, 'b, ()> {
        GeospatialGeojsonUploadPost::new(self.transport())
    }
    #[doc = "Geospatial Geojson Upload Put API\n\nUse an OpenSearch query to upload `GeoJSON` regardless if index exists.\n- When type is `geo_point`, only Point geometry is allowed\n- When type is `geo_shape`, all geometry types are allowed (Point, MultiPoint, LineString, MultiLineString, Polygon, MultiPolygon, GeometryCollection, Envelope)."]
    pub fn geojson_upload_put<'b>(&'a self) -> GeospatialGeojsonUploadPut<'a, 'b, ()> {
        GeospatialGeojsonUploadPut::new(self.transport())
    }
    #[doc = "[Geospatial Get Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#sending-a-get-request)\n\nGet one or more IP2Geo data sources, defaulting to returning all if no names specified."]
    pub fn get_ip2geo_datasource<'b>(
        &'a self,
        parts: GeospatialGetIp2GeoDatasourceParts<'b>,
    ) -> GeospatialGetIp2GeoDatasource<'a, 'b> {
        GeospatialGetIp2GeoDatasource::new(self.transport(), parts)
    }
    #[doc = "Geospatial Get Upload Stats API\n\nRetrieves statistics for all geospatial uploads."]
    pub fn get_upload_stats<'b>(&'a self) -> GeospatialGetUploadStats<'a, 'b> {
        GeospatialGetUploadStats::new(self.transport())
    }
    #[doc = "[Geospatial Put Ip2 Geo Datasource API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#data-source-options)\n\nCreate a specific IP2Geo data source.\nDefault values:\n  - `endpoint`: `\"https://geoip.maps.opensearch.org/v1/geolite2-city/manifest.json\"`\n  - `update_interval_in_days`: 3."]
    pub fn put_ip2geo_datasource<'b>(
        &'a self,
        parts: GeospatialPutIp2GeoDatasourceParts<'b>,
    ) -> GeospatialPutIp2GeoDatasource<'a, 'b, ()> {
        GeospatialPutIp2GeoDatasource::new(self.transport(), parts)
    }
    #[doc = "[Geospatial Put Ip2 Geo Datasource Settings API](https://docs.opensearch.org/docs/latest/ingest-pipelines/processors/ip2geo/#updating-an-ip2geo-data-source)\n\nUpdate a specific IP2Geo data source."]
    pub fn put_ip2geo_datasource_settings<'b>(
        &'a self,
        parts: GeospatialPutIp2GeoDatasourceSettingsParts<'b>,
    ) -> GeospatialPutIp2GeoDatasourceSettings<'a, 'b, ()> {
        GeospatialPutIp2GeoDatasourceSettings::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Geospatial APIs"]
    pub fn geospatial(&self) -> Geospatial {
        Geospatial::new(self.transport())
    }
}
