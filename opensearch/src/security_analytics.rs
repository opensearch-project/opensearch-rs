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
#[doc = "API parts for the Security Analytics Get Alerts API"]
pub enum SecurityAnalyticsGetAlertsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAnalyticsGetAlertsParts {
    #[doc = "Builds a relative URL path to the Security Analytics Get Alerts API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAnalyticsGetAlertsParts::None => "/_plugins/_security_analytics/alerts".into(),
        }
    }
}
#[doc = "Builder for the [Security Analytics Get Alerts API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/alert-finding-api/#get-alerts)\n\nRetrieve alerts related to a specific detector type or detector ID."]
#[derive(Clone, Debug)]
pub struct SecurityAnalyticsGetAlerts<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityAnalyticsGetAlertsParts,
    alertstate: Option<AlertState>,
    detector_id: Option<&'b str>,
    detectortype: Option<&'b str>,
    endtime: Option<i64>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    missing: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    searchstring: Option<&'b str>,
    severitylevel: Option<&'b str>,
    size: Option<i64>,
    sortorder: Option<SortOrder>,
    sortstring: Option<&'b str>,
    source: Option<&'b str>,
    startindex: Option<i64>,
    starttime: Option<i64>,
}
impl<'a, 'b> SecurityAnalyticsGetAlerts<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityAnalyticsGetAlerts]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAnalyticsGetAlerts {
            transport,
            parts: SecurityAnalyticsGetAlertsParts::None,
            headers,
            alertstate: None,
            detector_id: None,
            detectortype: None,
            endtime: None,
            error_trace: None,
            filter_path: None,
            human: None,
            missing: None,
            pretty: None,
            request_timeout: None,
            searchstring: None,
            severitylevel: None,
            size: None,
            sortorder: None,
            sortstring: None,
            source: None,
            startindex: None,
            starttime: None,
        }
    }
    #[doc = "Used to filter by alert state. Optional."]
    pub fn alertstate(mut self, alertstate: AlertState) -> Self {
        self.alertstate = Some(alertstate);
        self
    }
    #[doc = "The ID of the detector used to fetch alerts. Optional when `detectorType` is specified. Otherwise required."]
    pub fn detector_id(mut self, detector_id: &'b str) -> Self {
        self.detector_id = Some(detector_id);
        self
    }
    #[doc = "The type of detector used to fetch alerts. Optional when `detector_id` is specified. Otherwise required."]
    pub fn detectortype(mut self, detectortype: &'b str) -> Self {
        self.detectortype = Some(detectortype);
        self
    }
    #[doc = "The end timestamp (in ms) of the time window in which you want to retrieve alerts. Optional."]
    pub fn endtime(mut self, endtime: i64) -> Self {
        self.endtime = Some(endtime);
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
    #[doc = "Used to sort by whether the field `missing` exists or not in the documents associated with the alert. Optional."]
    pub fn missing(mut self, missing: &'b str) -> Self {
        self.missing = Some(missing);
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
    #[doc = "The alert attribute you want returned in the search. Optional."]
    pub fn searchstring(mut self, searchstring: &'b str) -> Self {
        self.searchstring = Some(searchstring);
        self
    }
    #[doc = "Used to filter by alert severity level. Optional."]
    pub fn severitylevel(mut self, severitylevel: &'b str) -> Self {
        self.severitylevel = Some(severitylevel);
        self
    }
    #[doc = "The maximum number of results returned in the response. Optional."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The order used to sort the list of findings. Possible values are `asc` or `desc`. Optional."]
    pub fn sortorder(mut self, sortorder: SortOrder) -> Self {
        self.sortorder = Some(sortorder);
        self
    }
    #[doc = "The string used by Security Analytics to sort the alerts. Optional."]
    pub fn sortstring(mut self, sortstring: &'b str) -> Self {
        self.sortstring = Some(sortstring);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The pagination index. Optional."]
    pub fn startindex(mut self, startindex: i64) -> Self {
        self.startindex = Some(startindex);
        self
    }
    #[doc = "The beginning timestamp (in ms) of the time window in which you want to retrieve alerts. Optional."]
    pub fn starttime(mut self, starttime: i64) -> Self {
        self.starttime = Some(starttime);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Analytics Get Alerts API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "alertState")]
                alertstate: Option<AlertState>,
                #[serde(rename = "detectorType")]
                detectortype: Option<&'b str>,
                detector_id: Option<&'b str>,
                #[serde(rename = "endTime")]
                endtime: Option<i64>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                missing: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(rename = "searchString")]
                searchstring: Option<&'b str>,
                #[serde(rename = "severityLevel")]
                severitylevel: Option<&'b str>,
                size: Option<i64>,
                #[serde(rename = "sortOrder")]
                sortorder: Option<SortOrder>,
                #[serde(rename = "sortString")]
                sortstring: Option<&'b str>,
                source: Option<&'b str>,
                #[serde(rename = "startIndex")]
                startindex: Option<i64>,
                #[serde(rename = "startTime")]
                starttime: Option<i64>,
            }
            let query_params = QueryParams {
                alertstate: self.alertstate,
                detectortype: self.detectortype,
                detector_id: self.detector_id,
                endtime: self.endtime,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                missing: self.missing,
                pretty: self.pretty,
                searchstring: self.searchstring,
                severitylevel: self.severitylevel,
                size: self.size,
                sortorder: self.sortorder,
                sortstring: self.sortstring,
                source: self.source,
                startindex: self.startindex,
                starttime: self.starttime,
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
#[doc = "API parts for the Security Analytics Get Findings API"]
pub enum SecurityAnalyticsGetFindingsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAnalyticsGetFindingsParts {
    #[doc = "Builds a relative URL path to the Security Analytics Get Findings API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAnalyticsGetFindingsParts::None => {
                "/_plugins/_security_analytics/findings/_search".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Analytics Get Findings API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/alert-finding-api/#get-findings)\n\nRetrieve findings related to a specific detector type or detector ID."]
#[derive(Clone, Debug)]
pub struct SecurityAnalyticsGetFindings<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityAnalyticsGetFindingsParts,
    detectiontype: Option<DetectionType>,
    detector_id: Option<&'b str>,
    detectortype: Option<&'b str>,
    endtime: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    findingids: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    missing: Option<&'b str>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    searchstring: Option<&'b str>,
    severity: Option<Severity>,
    size: Option<i64>,
    sortorder: Option<SortOrder>,
    sortstring: Option<&'b str>,
    source: Option<&'b str>,
    startindex: Option<i64>,
    starttime: Option<i64>,
}
impl<'a, 'b> SecurityAnalyticsGetFindings<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityAnalyticsGetFindings]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAnalyticsGetFindings {
            transport,
            parts: SecurityAnalyticsGetFindingsParts::None,
            headers,
            detectiontype: None,
            detector_id: None,
            detectortype: None,
            endtime: None,
            error_trace: None,
            filter_path: None,
            findingids: None,
            human: None,
            missing: None,
            pretty: None,
            request_timeout: None,
            searchstring: None,
            severity: None,
            size: None,
            sortorder: None,
            sortstring: None,
            source: None,
            startindex: None,
            starttime: None,
        }
    }
    #[doc = "The detection type that dictates the retrieval type for the findings. When the detection type is `threat`, it fetches threat intelligence feeds. When the detection type is `rule`, findings are fetched based on the detector’s rule. Optional."]
    pub fn detectiontype(mut self, detectiontype: DetectionType) -> Self {
        self.detectiontype = Some(detectiontype);
        self
    }
    #[doc = "The ID of the detector used to fetch alerts. Optional when the `detectorType` is specified. Otherwise required."]
    pub fn detector_id(mut self, detector_id: &'b str) -> Self {
        self.detector_id = Some(detector_id);
        self
    }
    #[doc = "The type of detector used to fetch alerts. Optional when the `detector_id` is specified. Otherwise required."]
    pub fn detectortype(mut self, detectortype: &'b str) -> Self {
        self.detectortype = Some(detectortype);
        self
    }
    #[doc = "The end timestamp (in ms) of the time window in which you want to retrieve findings. Optional."]
    pub fn endtime(mut self, endtime: &'b str) -> Self {
        self.endtime = Some(endtime);
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
    #[doc = "The comma-separated id list of findings for which you want retrieve details. Optional."]
    pub fn findingids(mut self, findingids: &'b str) -> Self {
        self.findingids = Some(findingids);
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
    #[doc = "Used to sort by whether the field `missing` exists or not in the documents associated with the finding. Optional."]
    pub fn missing(mut self, missing: &'b str) -> Self {
        self.missing = Some(missing);
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
    #[doc = "The finding attribute you want returned in the search. To search in a specific index, specify the index name in the request path. For example, to search findings in the indexABC index, use `searchString=indexABC’. Optional."]
    pub fn searchstring(mut self, searchstring: &'b str) -> Self {
        self.searchstring = Some(searchstring);
        self
    }
    #[doc = "The rule severity for which retrieve findings. Severity can be `critical`, `high`, `medium`, or `low`. Optional."]
    pub fn severity(mut self, severity: Severity) -> Self {
        self.severity = Some(severity);
        self
    }
    #[doc = "The maximum number of results returned in the response. Optional."]
    pub fn size(mut self, size: i64) -> Self {
        self.size = Some(size);
        self
    }
    #[doc = "The order used to sort the list of findings. Possible values are `asc` or `desc`. Optional."]
    pub fn sortorder(mut self, sortorder: SortOrder) -> Self {
        self.sortorder = Some(sortorder);
        self
    }
    #[doc = "The string used by the Alerting plugin to sort the findings. Optional."]
    pub fn sortstring(mut self, sortstring: &'b str) -> Self {
        self.sortstring = Some(sortstring);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    #[doc = "The pagination index. Optional."]
    pub fn startindex(mut self, startindex: i64) -> Self {
        self.startindex = Some(startindex);
        self
    }
    #[doc = "The beginning timestamp (in ms) of the time window in which you want to retrieve findings. Optional."]
    pub fn starttime(mut self, starttime: i64) -> Self {
        self.starttime = Some(starttime);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Analytics Get Findings API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "detectionType")]
                detectiontype: Option<DetectionType>,
                #[serde(rename = "detectorType")]
                detectortype: Option<&'b str>,
                detector_id: Option<&'b str>,
                #[serde(rename = "endTime")]
                endtime: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                #[serde(rename = "findingIds")]
                findingids: Option<&'b str>,
                human: Option<bool>,
                missing: Option<&'b str>,
                pretty: Option<bool>,
                #[serde(rename = "searchString")]
                searchstring: Option<&'b str>,
                severity: Option<Severity>,
                size: Option<i64>,
                #[serde(rename = "sortOrder")]
                sortorder: Option<SortOrder>,
                #[serde(rename = "sortString")]
                sortstring: Option<&'b str>,
                source: Option<&'b str>,
                #[serde(rename = "startIndex")]
                startindex: Option<i64>,
                #[serde(rename = "startTime")]
                starttime: Option<i64>,
            }
            let query_params = QueryParams {
                detectiontype: self.detectiontype,
                detectortype: self.detectortype,
                detector_id: self.detector_id,
                endtime: self.endtime,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                findingids: self.findingids,
                human: self.human,
                missing: self.missing,
                pretty: self.pretty,
                searchstring: self.searchstring,
                severity: self.severity,
                size: self.size,
                sortorder: self.sortorder,
                sortstring: self.sortstring,
                source: self.source,
                startindex: self.startindex,
                starttime: self.starttime,
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
#[doc = "API parts for the Security Analytics Search Finding Correlations API"]
pub enum SecurityAnalyticsSearchFindingCorrelationsParts {
    #[doc = "No parts"]
    None,
}
impl SecurityAnalyticsSearchFindingCorrelationsParts {
    #[doc = "Builds a relative URL path to the Security Analytics Search Finding Correlations API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            SecurityAnalyticsSearchFindingCorrelationsParts::None => {
                "/_plugins/_security_analytics/findings/correlate".into()
            }
        }
    }
}
#[doc = "Builder for the [Security Analytics Search Finding Correlations API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/correlation-eng/#list-correlations-for-a-finding-belonging-to-a-log-type)\n\nList correlations for a finding."]
#[derive(Clone, Debug)]
pub struct SecurityAnalyticsSearchFindingCorrelations<'a, 'b> {
    transport: &'a Transport,
    parts: SecurityAnalyticsSearchFindingCorrelationsParts,
    detector_type: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    finding: Option<&'b str>,
    headers: HeaderMap,
    human: Option<bool>,
    nearby_findings: Option<i64>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
    time_window: Option<i64>,
}
impl<'a, 'b> SecurityAnalyticsSearchFindingCorrelations<'a, 'b> {
    #[doc = "Creates a new instance of [SecurityAnalyticsSearchFindingCorrelations]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        SecurityAnalyticsSearchFindingCorrelations {
            transport,
            parts: SecurityAnalyticsSearchFindingCorrelationsParts::None,
            headers,
            detector_type: None,
            error_trace: None,
            filter_path: None,
            finding: None,
            human: None,
            nearby_findings: None,
            pretty: None,
            request_timeout: None,
            source: None,
            time_window: None,
        }
    }
    #[doc = "The log type of findings you want to correlate with the specified finding. Required."]
    pub fn detector_type(mut self, detector_type: &'b str) -> Self {
        self.detector_type = Some(detector_type);
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
    #[doc = "The finding ID for which you want to find other findings that are correlated. Required."]
    pub fn finding(mut self, finding: &'b str) -> Self {
        self.finding = Some(finding);
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
    #[doc = "The number of nearby findings you want to return. Optional."]
    pub fn nearby_findings(mut self, nearby_findings: i64) -> Self {
        self.nearby_findings = Some(nearby_findings);
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
    #[doc = "The time window (in ms) in which all of the correlations must have occurred together. Optional."]
    pub fn time_window(mut self, time_window: i64) -> Self {
        self.time_window = Some(time_window);
        self
    }
    #[doc = "Creates an asynchronous call to the Security Analytics Search Finding Correlations API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                detector_type: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                finding: Option<&'b str>,
                human: Option<bool>,
                nearby_findings: Option<i64>,
                pretty: Option<bool>,
                source: Option<&'b str>,
                time_window: Option<i64>,
            }
            let query_params = QueryParams {
                detector_type: self.detector_type,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                finding: self.finding,
                human: self.human,
                nearby_findings: self.nearby_findings,
                pretty: self.pretty,
                source: self.source,
                time_window: self.time_window,
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
#[doc = "Namespace client for SecurityAnalytics APIs"]
pub struct SecurityAnalytics<'a> {
    transport: &'a Transport,
}
impl<'a> SecurityAnalytics<'a> {
    #[doc = "Creates a new instance of [SecurityAnalytics]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Security Analytics Get Alerts API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/alert-finding-api/#get-alerts)\n\nRetrieve alerts related to a specific detector type or detector ID."]
    pub fn get_alerts<'b>(&'a self) -> SecurityAnalyticsGetAlerts<'a, 'b> {
        SecurityAnalyticsGetAlerts::new(self.transport())
    }
    #[doc = "[Security Analytics Get Findings API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/alert-finding-api/#get-findings)\n\nRetrieve findings related to a specific detector type or detector ID."]
    pub fn get_findings<'b>(&'a self) -> SecurityAnalyticsGetFindings<'a, 'b> {
        SecurityAnalyticsGetFindings::new(self.transport())
    }
    #[doc = "[Security Analytics Search Finding Correlations API](https://docs.opensearch.org/docs/latest/security-analytics/api-tools/correlation-eng/#list-correlations-for-a-finding-belonging-to-a-log-type)\n\nList correlations for a finding."]
    pub fn search_finding_correlations<'b>(
        &'a self,
    ) -> SecurityAnalyticsSearchFindingCorrelations<'a, 'b> {
        SecurityAnalyticsSearchFindingCorrelations::new(self.transport())
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for SecurityAnalytics APIs"]
    pub fn security_analytics(&self) -> SecurityAnalytics {
        SecurityAnalytics::new(self.transport())
    }
}
