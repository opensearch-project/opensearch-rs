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
#[doc = "API parts for the Notifications Create Config API"]
pub enum NotificationsCreateConfigParts {
    #[doc = "No parts"]
    None,
}
impl NotificationsCreateConfigParts {
    #[doc = "Builds a relative URL path to the Notifications Create Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsCreateConfigParts::None => "/_plugins/_notifications/configs".into(),
        }
    }
}
#[doc = "Builder for the [Notifications Create Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#create-channel-configuration)\n\nCreate channel configuration."]
#[derive(Clone, Debug)]
pub struct NotificationsCreateConfig<'a, 'b, B> {
    transport: &'a Transport,
    parts: NotificationsCreateConfigParts,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> NotificationsCreateConfig<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [NotificationsCreateConfig]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        NotificationsCreateConfig {
            transport,
            parts: NotificationsCreateConfigParts::None,
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
    pub fn body<T>(self, body: T) -> NotificationsCreateConfig<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        NotificationsCreateConfig {
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
    #[doc = "Creates an asynchronous call to the Notifications Create Config API that can be awaited"]
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
#[doc = "API parts for the Notifications Delete Config API"]
pub enum NotificationsDeleteConfigParts<'b> {
    #[doc = "ConfigId"]
    ConfigId(&'b str),
}
impl<'b> NotificationsDeleteConfigParts<'b> {
    #[doc = "Builds a relative URL path to the Notifications Delete Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsDeleteConfigParts::ConfigId(config_id) => {
                let encoded_config_id: Cow<str> =
                    percent_encode(config_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_config_id.len());
                p.push_str("/_plugins/_notifications/configs/");
                p.push_str(encoded_config_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Notifications Delete Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#delete-channel-configuration)\n\nDelete a channel configuration."]
#[derive(Clone, Debug)]
pub struct NotificationsDeleteConfig<'a, 'b> {
    transport: &'a Transport,
    parts: NotificationsDeleteConfigParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NotificationsDeleteConfig<'a, 'b> {
    #[doc = "Creates a new instance of [NotificationsDeleteConfig] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NotificationsDeleteConfigParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NotificationsDeleteConfig {
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
    #[doc = "Creates an asynchronous call to the Notifications Delete Config API that can be awaited"]
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
#[doc = "API parts for the Notifications Delete Configs API"]
pub enum NotificationsDeleteConfigsParts {
    #[doc = "No parts"]
    None,
}
impl NotificationsDeleteConfigsParts {
    #[doc = "Builds a relative URL path to the Notifications Delete Configs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsDeleteConfigsParts::None => "/_plugins/_notifications/configs".into(),
        }
    }
}
#[doc = "Builder for the [Notifications Delete Configs API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#delete-channel-configuration)\n\nDelete multiple channel configurations."]
#[derive(Clone, Debug)]
pub struct NotificationsDeleteConfigs<'a, 'b> {
    transport: &'a Transport,
    parts: NotificationsDeleteConfigsParts,
    config_id: Option<&'b str>,
    config_id_list: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NotificationsDeleteConfigs<'a, 'b> {
    #[doc = "Creates a new instance of [NotificationsDeleteConfigs]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        NotificationsDeleteConfigs {
            transport,
            parts: NotificationsDeleteConfigsParts::None,
            headers,
            config_id: None,
            config_id_list: None,
            error_trace: None,
            filter_path: None,
            human: None,
            pretty: None,
            request_timeout: None,
            source: None,
        }
    }
    #[doc = "The ID of the channel configuration to delete."]
    pub fn config_id(mut self, config_id: &'b str) -> Self {
        self.config_id = Some(config_id);
        self
    }
    #[doc = "A comma-separated list of channel IDs to delete."]
    pub fn config_id_list(mut self, config_id_list: &'b str) -> Self {
        self.config_id_list = Some(config_id_list);
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
    #[doc = "Creates an asynchronous call to the Notifications Delete Configs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Delete;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                config_id: Option<&'b str>,
                config_id_list: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                pretty: Option<bool>,
                source: Option<&'b str>,
            }
            let query_params = QueryParams {
                config_id: self.config_id,
                config_id_list: self.config_id_list,
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
#[doc = "API parts for the Notifications Get Config API"]
pub enum NotificationsGetConfigParts<'b> {
    #[doc = "ConfigId"]
    ConfigId(&'b str),
}
impl<'b> NotificationsGetConfigParts<'b> {
    #[doc = "Builds a relative URL path to the Notifications Get Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsGetConfigParts::ConfigId(config_id) => {
                let encoded_config_id: Cow<str> =
                    percent_encode(config_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_config_id.len());
                p.push_str("/_plugins/_notifications/configs/");
                p.push_str(encoded_config_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the Notifications Get Config API\n\nGet a specific channel configuration."]
#[derive(Clone, Debug)]
pub struct NotificationsGetConfig<'a, 'b> {
    transport: &'a Transport,
    parts: NotificationsGetConfigParts<'b>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NotificationsGetConfig<'a, 'b> {
    #[doc = "Creates a new instance of [NotificationsGetConfig] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NotificationsGetConfigParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NotificationsGetConfig {
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
    #[doc = "Creates an asynchronous call to the Notifications Get Config API that can be awaited"]
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
#[doc = "API parts for the Notifications Get Configs API"]
pub enum NotificationsGetConfigsParts {
    #[doc = "No parts"]
    None,
}
impl NotificationsGetConfigsParts {
    #[doc = "Builds a relative URL path to the Notifications Get Configs API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsGetConfigsParts::None => "/_plugins/_notifications/configs".into(),
        }
    }
}
#[doc = "Builder for the [Notifications Get Configs API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-all-notification-configurations)\n\nGet multiple channel configurations with filtering."]
#[derive(Clone, Debug)]
pub struct NotificationsGetConfigs<'a, 'b, B> {
    transport: &'a Transport,
    parts: NotificationsGetConfigsParts,
    body: Option<B>,
    chime_url: Option<&'b str>,
    chime_url_keyword: Option<&'b str>,
    config_id: Option<&'b str>,
    config_id_list: Option<&'b [&'b str]>,
    config_type: Option<ConfigType>,
    created_time_ms: Option<i64>,
    description: Option<&'b str>,
    description_keyword: Option<&'b str>,
    email_email_account_id: Option<&'b str>,
    email_email_group_id_list: Option<&'b str>,
    email_group_recipient_list_recipient: Option<&'b str>,
    email_group_recipient_list_recipient_keyword: Option<&'b str>,
    email_recipient_list_recipient: Option<&'b str>,
    email_recipient_list_recipient_keyword: Option<&'b str>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    is_enabled: Option<bool>,
    last_updated_time_ms: Option<i64>,
    microsoft_teams_url: Option<&'b str>,
    microsoft_teams_url_keyword: Option<&'b str>,
    name: Option<&'b str>,
    name_keyword: Option<&'b str>,
    pretty: Option<bool>,
    query: Option<&'b str>,
    request_timeout: Option<Duration>,
    ses_account_from_address: Option<&'b str>,
    ses_account_from_address_keyword: Option<&'b str>,
    ses_account_region: Option<&'b str>,
    ses_account_role_arn: Option<&'b str>,
    ses_account_role_arn_keyword: Option<&'b str>,
    slack_url: Option<&'b str>,
    slack_url_keyword: Option<&'b str>,
    smtp_account_from_address: Option<&'b str>,
    smtp_account_from_address_keyword: Option<&'b str>,
    smtp_account_host: Option<&'b str>,
    smtp_account_host_keyword: Option<&'b str>,
    smtp_account_method: Option<&'b str>,
    sns_role_arn: Option<&'b str>,
    sns_role_arn_keyword: Option<&'b str>,
    sns_topic_arn: Option<&'b str>,
    sns_topic_arn_keyword: Option<&'b str>,
    source: Option<&'b str>,
    text_query: Option<&'b str>,
    webhook_url: Option<&'b str>,
    webhook_url_keyword: Option<&'b str>,
}
impl<'a, 'b, B> NotificationsGetConfigs<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [NotificationsGetConfigs]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        NotificationsGetConfigs {
            transport,
            parts: NotificationsGetConfigsParts::None,
            headers,
            body: None,
            chime_url: None,
            chime_url_keyword: None,
            config_id: None,
            config_id_list: None,
            config_type: None,
            created_time_ms: None,
            description: None,
            description_keyword: None,
            email_email_account_id: None,
            email_email_group_id_list: None,
            email_group_recipient_list_recipient: None,
            email_group_recipient_list_recipient_keyword: None,
            email_recipient_list_recipient: None,
            email_recipient_list_recipient_keyword: None,
            error_trace: None,
            filter_path: None,
            human: None,
            is_enabled: None,
            last_updated_time_ms: None,
            microsoft_teams_url: None,
            microsoft_teams_url_keyword: None,
            name: None,
            name_keyword: None,
            pretty: None,
            query: None,
            request_timeout: None,
            ses_account_from_address: None,
            ses_account_from_address_keyword: None,
            ses_account_region: None,
            ses_account_role_arn: None,
            ses_account_role_arn_keyword: None,
            slack_url: None,
            slack_url_keyword: None,
            smtp_account_from_address: None,
            smtp_account_from_address_keyword: None,
            smtp_account_host: None,
            smtp_account_host_keyword: None,
            smtp_account_method: None,
            sns_role_arn: None,
            sns_role_arn_keyword: None,
            sns_topic_arn: None,
            sns_topic_arn_keyword: None,
            source: None,
            text_query: None,
            webhook_url: None,
            webhook_url_keyword: None,
        }
    }
    #[doc = "The body for the API call"]
    pub fn body<T>(self, body: T) -> NotificationsGetConfigs<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        NotificationsGetConfigs {
            transport: self.transport,
            parts: self.parts,
            body: Some(body.into()),
            chime_url: self.chime_url,
            chime_url_keyword: self.chime_url_keyword,
            config_id: self.config_id,
            config_id_list: self.config_id_list,
            config_type: self.config_type,
            created_time_ms: self.created_time_ms,
            description: self.description,
            description_keyword: self.description_keyword,
            email_email_account_id: self.email_email_account_id,
            email_email_group_id_list: self.email_email_group_id_list,
            email_group_recipient_list_recipient: self.email_group_recipient_list_recipient,
            email_group_recipient_list_recipient_keyword: self
                .email_group_recipient_list_recipient_keyword,
            email_recipient_list_recipient: self.email_recipient_list_recipient,
            email_recipient_list_recipient_keyword: self.email_recipient_list_recipient_keyword,
            error_trace: self.error_trace,
            filter_path: self.filter_path,
            headers: self.headers,
            human: self.human,
            is_enabled: self.is_enabled,
            last_updated_time_ms: self.last_updated_time_ms,
            microsoft_teams_url: self.microsoft_teams_url,
            microsoft_teams_url_keyword: self.microsoft_teams_url_keyword,
            name: self.name,
            name_keyword: self.name_keyword,
            pretty: self.pretty,
            query: self.query,
            request_timeout: self.request_timeout,
            ses_account_from_address: self.ses_account_from_address,
            ses_account_from_address_keyword: self.ses_account_from_address_keyword,
            ses_account_region: self.ses_account_region,
            ses_account_role_arn: self.ses_account_role_arn,
            ses_account_role_arn_keyword: self.ses_account_role_arn_keyword,
            slack_url: self.slack_url,
            slack_url_keyword: self.slack_url_keyword,
            smtp_account_from_address: self.smtp_account_from_address,
            smtp_account_from_address_keyword: self.smtp_account_from_address_keyword,
            smtp_account_host: self.smtp_account_host,
            smtp_account_host_keyword: self.smtp_account_host_keyword,
            smtp_account_method: self.smtp_account_method,
            sns_role_arn: self.sns_role_arn,
            sns_role_arn_keyword: self.sns_role_arn_keyword,
            sns_topic_arn: self.sns_topic_arn,
            sns_topic_arn_keyword: self.sns_topic_arn_keyword,
            source: self.source,
            text_query: self.text_query,
            webhook_url: self.webhook_url,
            webhook_url_keyword: self.webhook_url_keyword,
        }
    }
    pub fn chime_url(mut self, chime_url: &'b str) -> Self {
        self.chime_url = Some(chime_url);
        self
    }
    pub fn chime_url_keyword(mut self, chime_url_keyword: &'b str) -> Self {
        self.chime_url_keyword = Some(chime_url_keyword);
        self
    }
    #[doc = "Notification configuration ID."]
    pub fn config_id(mut self, config_id: &'b str) -> Self {
        self.config_id = Some(config_id);
        self
    }
    #[doc = "Notification configuration IDs."]
    pub fn config_id_list(mut self, config_id_list: &'b [&'b str]) -> Self {
        self.config_id_list = Some(config_id_list);
        self
    }
    #[doc = "Type of notification configuration."]
    pub fn config_type(mut self, config_type: ConfigType) -> Self {
        self.config_type = Some(config_type);
        self
    }
    pub fn created_time_ms(mut self, created_time_ms: i64) -> Self {
        self.created_time_ms = Some(created_time_ms);
        self
    }
    pub fn description(mut self, description: &'b str) -> Self {
        self.description = Some(description);
        self
    }
    pub fn description_keyword(mut self, description_keyword: &'b str) -> Self {
        self.description_keyword = Some(description_keyword);
        self
    }
    pub fn email_email_account_id(mut self, email_email_account_id: &'b str) -> Self {
        self.email_email_account_id = Some(email_email_account_id);
        self
    }
    pub fn email_email_group_id_list(mut self, email_email_group_id_list: &'b str) -> Self {
        self.email_email_group_id_list = Some(email_email_group_id_list);
        self
    }
    pub fn email_group_recipient_list_recipient(
        mut self,
        email_group_recipient_list_recipient: &'b str,
    ) -> Self {
        self.email_group_recipient_list_recipient = Some(email_group_recipient_list_recipient);
        self
    }
    pub fn email_group_recipient_list_recipient_keyword(
        mut self,
        email_group_recipient_list_recipient_keyword: &'b str,
    ) -> Self {
        self.email_group_recipient_list_recipient_keyword =
            Some(email_group_recipient_list_recipient_keyword);
        self
    }
    pub fn email_recipient_list_recipient(
        mut self,
        email_recipient_list_recipient: &'b str,
    ) -> Self {
        self.email_recipient_list_recipient = Some(email_recipient_list_recipient);
        self
    }
    pub fn email_recipient_list_recipient_keyword(
        mut self,
        email_recipient_list_recipient_keyword: &'b str,
    ) -> Self {
        self.email_recipient_list_recipient_keyword = Some(email_recipient_list_recipient_keyword);
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
    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = Some(is_enabled);
        self
    }
    pub fn last_updated_time_ms(mut self, last_updated_time_ms: i64) -> Self {
        self.last_updated_time_ms = Some(last_updated_time_ms);
        self
    }
    pub fn microsoft_teams_url(mut self, microsoft_teams_url: &'b str) -> Self {
        self.microsoft_teams_url = Some(microsoft_teams_url);
        self
    }
    pub fn microsoft_teams_url_keyword(mut self, microsoft_teams_url_keyword: &'b str) -> Self {
        self.microsoft_teams_url_keyword = Some(microsoft_teams_url_keyword);
        self
    }
    pub fn name(mut self, name: &'b str) -> Self {
        self.name = Some(name);
        self
    }
    pub fn name_keyword(mut self, name_keyword: &'b str) -> Self {
        self.name_keyword = Some(name_keyword);
        self
    }
    #[doc = "Whether to pretty-format the returned JSON response."]
    pub fn pretty(mut self, pretty: bool) -> Self {
        self.pretty = Some(pretty);
        self
    }
    pub fn query(mut self, query: &'b str) -> Self {
        self.query = Some(query);
        self
    }
    #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }
    pub fn ses_account_from_address(mut self, ses_account_from_address: &'b str) -> Self {
        self.ses_account_from_address = Some(ses_account_from_address);
        self
    }
    pub fn ses_account_from_address_keyword(
        mut self,
        ses_account_from_address_keyword: &'b str,
    ) -> Self {
        self.ses_account_from_address_keyword = Some(ses_account_from_address_keyword);
        self
    }
    pub fn ses_account_region(mut self, ses_account_region: &'b str) -> Self {
        self.ses_account_region = Some(ses_account_region);
        self
    }
    pub fn ses_account_role_arn(mut self, ses_account_role_arn: &'b str) -> Self {
        self.ses_account_role_arn = Some(ses_account_role_arn);
        self
    }
    pub fn ses_account_role_arn_keyword(mut self, ses_account_role_arn_keyword: &'b str) -> Self {
        self.ses_account_role_arn_keyword = Some(ses_account_role_arn_keyword);
        self
    }
    pub fn slack_url(mut self, slack_url: &'b str) -> Self {
        self.slack_url = Some(slack_url);
        self
    }
    pub fn slack_url_keyword(mut self, slack_url_keyword: &'b str) -> Self {
        self.slack_url_keyword = Some(slack_url_keyword);
        self
    }
    pub fn smtp_account_from_address(mut self, smtp_account_from_address: &'b str) -> Self {
        self.smtp_account_from_address = Some(smtp_account_from_address);
        self
    }
    pub fn smtp_account_from_address_keyword(
        mut self,
        smtp_account_from_address_keyword: &'b str,
    ) -> Self {
        self.smtp_account_from_address_keyword = Some(smtp_account_from_address_keyword);
        self
    }
    pub fn smtp_account_host(mut self, smtp_account_host: &'b str) -> Self {
        self.smtp_account_host = Some(smtp_account_host);
        self
    }
    pub fn smtp_account_host_keyword(mut self, smtp_account_host_keyword: &'b str) -> Self {
        self.smtp_account_host_keyword = Some(smtp_account_host_keyword);
        self
    }
    pub fn smtp_account_method(mut self, smtp_account_method: &'b str) -> Self {
        self.smtp_account_method = Some(smtp_account_method);
        self
    }
    pub fn sns_role_arn(mut self, sns_role_arn: &'b str) -> Self {
        self.sns_role_arn = Some(sns_role_arn);
        self
    }
    pub fn sns_role_arn_keyword(mut self, sns_role_arn_keyword: &'b str) -> Self {
        self.sns_role_arn_keyword = Some(sns_role_arn_keyword);
        self
    }
    pub fn sns_topic_arn(mut self, sns_topic_arn: &'b str) -> Self {
        self.sns_topic_arn = Some(sns_topic_arn);
        self
    }
    pub fn sns_topic_arn_keyword(mut self, sns_topic_arn_keyword: &'b str) -> Self {
        self.sns_topic_arn_keyword = Some(sns_topic_arn_keyword);
        self
    }
    #[doc = "The URL-encoded request definition. Useful for libraries that do not accept a request body for non-POST requests."]
    pub fn source(mut self, source: &'b str) -> Self {
        self.source = Some(source);
        self
    }
    pub fn text_query(mut self, text_query: &'b str) -> Self {
        self.text_query = Some(text_query);
        self
    }
    pub fn webhook_url(mut self, webhook_url: &'b str) -> Self {
        self.webhook_url = Some(webhook_url);
        self
    }
    pub fn webhook_url_keyword(mut self, webhook_url_keyword: &'b str) -> Self {
        self.webhook_url_keyword = Some(webhook_url_keyword);
        self
    }
    #[doc = "Creates an asynchronous call to the Notifications Get Configs API that can be awaited"]
    pub async fn send(self) -> Result<Response, Error> {
        let path = self.parts.url();
        let method = Method::Get;
        let headers = self.headers;
        let timeout = self.request_timeout;
        let query_string = {
            #[serde_with::skip_serializing_none]
            #[derive(Serialize)]
            struct QueryParams<'b> {
                #[serde(rename = "chime.url")]
                chime_url: Option<&'b str>,
                #[serde(rename = "chime.url.keyword")]
                chime_url_keyword: Option<&'b str>,
                config_id: Option<&'b str>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                config_id_list: Option<&'b [&'b str]>,
                config_type: Option<ConfigType>,
                created_time_ms: Option<i64>,
                description: Option<&'b str>,
                #[serde(rename = "description.keyword")]
                description_keyword: Option<&'b str>,
                #[serde(rename = "email.email_account_id")]
                email_email_account_id: Option<&'b str>,
                #[serde(rename = "email.email_group_id_list")]
                email_email_group_id_list: Option<&'b str>,
                #[serde(rename = "email.recipient_list.recipient")]
                email_recipient_list_recipient: Option<&'b str>,
                #[serde(rename = "email.recipient_list.recipient.keyword")]
                email_recipient_list_recipient_keyword: Option<&'b str>,
                #[serde(rename = "email_group.recipient_list.recipient")]
                email_group_recipient_list_recipient: Option<&'b str>,
                #[serde(rename = "email_group.recipient_list.recipient.keyword")]
                email_group_recipient_list_recipient_keyword: Option<&'b str>,
                error_trace: Option<bool>,
                #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                filter_path: Option<&'b [&'b str]>,
                human: Option<bool>,
                is_enabled: Option<bool>,
                last_updated_time_ms: Option<i64>,
                #[serde(rename = "microsoft_teams.url")]
                microsoft_teams_url: Option<&'b str>,
                #[serde(rename = "microsoft_teams.url.keyword")]
                microsoft_teams_url_keyword: Option<&'b str>,
                name: Option<&'b str>,
                #[serde(rename = "name.keyword")]
                name_keyword: Option<&'b str>,
                pretty: Option<bool>,
                query: Option<&'b str>,
                #[serde(rename = "ses_account.from_address")]
                ses_account_from_address: Option<&'b str>,
                #[serde(rename = "ses_account.from_address.keyword")]
                ses_account_from_address_keyword: Option<&'b str>,
                #[serde(rename = "ses_account.region")]
                ses_account_region: Option<&'b str>,
                #[serde(rename = "ses_account.role_arn")]
                ses_account_role_arn: Option<&'b str>,
                #[serde(rename = "ses_account.role_arn.keyword")]
                ses_account_role_arn_keyword: Option<&'b str>,
                #[serde(rename = "slack.url")]
                slack_url: Option<&'b str>,
                #[serde(rename = "slack.url.keyword")]
                slack_url_keyword: Option<&'b str>,
                #[serde(rename = "smtp_account.from_address")]
                smtp_account_from_address: Option<&'b str>,
                #[serde(rename = "smtp_account.from_address.keyword")]
                smtp_account_from_address_keyword: Option<&'b str>,
                #[serde(rename = "smtp_account.host")]
                smtp_account_host: Option<&'b str>,
                #[serde(rename = "smtp_account.host.keyword")]
                smtp_account_host_keyword: Option<&'b str>,
                #[serde(rename = "smtp_account.method")]
                smtp_account_method: Option<&'b str>,
                #[serde(rename = "sns.role_arn")]
                sns_role_arn: Option<&'b str>,
                #[serde(rename = "sns.role_arn.keyword")]
                sns_role_arn_keyword: Option<&'b str>,
                #[serde(rename = "sns.topic_arn")]
                sns_topic_arn: Option<&'b str>,
                #[serde(rename = "sns.topic_arn.keyword")]
                sns_topic_arn_keyword: Option<&'b str>,
                source: Option<&'b str>,
                text_query: Option<&'b str>,
                #[serde(rename = "webhook.url")]
                webhook_url: Option<&'b str>,
                #[serde(rename = "webhook.url.keyword")]
                webhook_url_keyword: Option<&'b str>,
            }
            let query_params = QueryParams {
                chime_url: self.chime_url,
                chime_url_keyword: self.chime_url_keyword,
                config_id: self.config_id,
                config_id_list: self.config_id_list,
                config_type: self.config_type,
                created_time_ms: self.created_time_ms,
                description: self.description,
                description_keyword: self.description_keyword,
                email_email_account_id: self.email_email_account_id,
                email_email_group_id_list: self.email_email_group_id_list,
                email_recipient_list_recipient: self.email_recipient_list_recipient,
                email_recipient_list_recipient_keyword: self.email_recipient_list_recipient_keyword,
                email_group_recipient_list_recipient: self.email_group_recipient_list_recipient,
                email_group_recipient_list_recipient_keyword: self
                    .email_group_recipient_list_recipient_keyword,
                error_trace: self.error_trace,
                filter_path: self.filter_path,
                human: self.human,
                is_enabled: self.is_enabled,
                last_updated_time_ms: self.last_updated_time_ms,
                microsoft_teams_url: self.microsoft_teams_url,
                microsoft_teams_url_keyword: self.microsoft_teams_url_keyword,
                name: self.name,
                name_keyword: self.name_keyword,
                pretty: self.pretty,
                query: self.query,
                ses_account_from_address: self.ses_account_from_address,
                ses_account_from_address_keyword: self.ses_account_from_address_keyword,
                ses_account_region: self.ses_account_region,
                ses_account_role_arn: self.ses_account_role_arn,
                ses_account_role_arn_keyword: self.ses_account_role_arn_keyword,
                slack_url: self.slack_url,
                slack_url_keyword: self.slack_url_keyword,
                smtp_account_from_address: self.smtp_account_from_address,
                smtp_account_from_address_keyword: self.smtp_account_from_address_keyword,
                smtp_account_host: self.smtp_account_host,
                smtp_account_host_keyword: self.smtp_account_host_keyword,
                smtp_account_method: self.smtp_account_method,
                sns_role_arn: self.sns_role_arn,
                sns_role_arn_keyword: self.sns_role_arn_keyword,
                sns_topic_arn: self.sns_topic_arn,
                sns_topic_arn_keyword: self.sns_topic_arn_keyword,
                source: self.source,
                text_query: self.text_query,
                webhook_url: self.webhook_url,
                webhook_url_keyword: self.webhook_url_keyword,
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
#[doc = "API parts for the Notifications List Channels API"]
pub enum NotificationsListChannelsParts {
    #[doc = "No parts"]
    None,
}
impl NotificationsListChannelsParts {
    #[doc = "Builds a relative URL path to the Notifications List Channels API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsListChannelsParts::None => "/_plugins/_notifications/channels".into(),
        }
    }
}
#[doc = "Builder for the [Notifications List Channels API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-all-notification-channels)\n\nList created notification channels."]
#[derive(Clone, Debug)]
pub struct NotificationsListChannels<'a, 'b> {
    transport: &'a Transport,
    parts: NotificationsListChannelsParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NotificationsListChannels<'a, 'b> {
    #[doc = "Creates a new instance of [NotificationsListChannels]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        NotificationsListChannels {
            transport,
            parts: NotificationsListChannelsParts::None,
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
    #[doc = "Creates an asynchronous call to the Notifications List Channels API that can be awaited"]
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
#[doc = "API parts for the Notifications List Features API"]
pub enum NotificationsListFeaturesParts {
    #[doc = "No parts"]
    None,
}
impl NotificationsListFeaturesParts {
    #[doc = "Builds a relative URL path to the Notifications List Features API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsListFeaturesParts::None => "/_plugins/_notifications/features".into(),
        }
    }
}
#[doc = "Builder for the [Notifications List Features API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-supported-channel-configurations)\n\nList supported channel configurations."]
#[derive(Clone, Debug)]
pub struct NotificationsListFeatures<'a, 'b> {
    transport: &'a Transport,
    parts: NotificationsListFeaturesParts,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b> NotificationsListFeatures<'a, 'b> {
    #[doc = "Creates a new instance of [NotificationsListFeatures]"]
    pub fn new(transport: &'a Transport) -> Self {
        let headers = HeaderMap::new();
        NotificationsListFeatures {
            transport,
            parts: NotificationsListFeaturesParts::None,
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
    #[doc = "Creates an asynchronous call to the Notifications List Features API that can be awaited"]
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
#[doc = "API parts for the Notifications Send Test API"]
pub enum NotificationsSendTestParts<'b> {
    #[doc = "ConfigId"]
    ConfigId(&'b str),
}
impl<'b> NotificationsSendTestParts<'b> {
    #[doc = "Builds a relative URL path to the Notifications Send Test API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsSendTestParts::ConfigId(config_id) => {
                let encoded_config_id: Cow<str> =
                    percent_encode(config_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(38usize + encoded_config_id.len());
                p.push_str("/_plugins/_notifications/feature/test/");
                p.push_str(encoded_config_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Notifications Send Test API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#send-test-notification)\n\nSend a test notification."]
#[derive(Clone, Debug)]
pub struct NotificationsSendTest<'a, 'b, B> {
    transport: &'a Transport,
    parts: NotificationsSendTestParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> NotificationsSendTest<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [NotificationsSendTest] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NotificationsSendTestParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NotificationsSendTest {
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
    pub fn body<T>(self, body: T) -> NotificationsSendTest<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        NotificationsSendTest {
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
    #[doc = "Creates an asynchronous call to the Notifications Send Test API that can be awaited"]
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
#[doc = "API parts for the Notifications Update Config API"]
pub enum NotificationsUpdateConfigParts<'b> {
    #[doc = "ConfigId"]
    ConfigId(&'b str),
}
impl<'b> NotificationsUpdateConfigParts<'b> {
    #[doc = "Builds a relative URL path to the Notifications Update Config API"]
    pub fn url(self) -> Cow<'static, str> {
        match self {
            NotificationsUpdateConfigParts::ConfigId(config_id) => {
                let encoded_config_id: Cow<str> =
                    percent_encode(config_id.as_bytes(), PARTS_ENCODED).into();
                let mut p = String::with_capacity(33usize + encoded_config_id.len());
                p.push_str("/_plugins/_notifications/configs/");
                p.push_str(encoded_config_id.as_ref());
                p.into()
            }
        }
    }
}
#[doc = "Builder for the [Notifications Update Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#update-channel-configuration)\n\nUpdate channel configuration."]
#[derive(Clone, Debug)]
pub struct NotificationsUpdateConfig<'a, 'b, B> {
    transport: &'a Transport,
    parts: NotificationsUpdateConfigParts<'b>,
    body: Option<B>,
    error_trace: Option<bool>,
    filter_path: Option<&'b [&'b str]>,
    headers: HeaderMap,
    human: Option<bool>,
    pretty: Option<bool>,
    request_timeout: Option<Duration>,
    source: Option<&'b str>,
}
impl<'a, 'b, B> NotificationsUpdateConfig<'a, 'b, B>
where
    B: Body,
{
    #[doc = "Creates a new instance of [NotificationsUpdateConfig] with the specified API parts"]
    pub fn new(transport: &'a Transport, parts: NotificationsUpdateConfigParts<'b>) -> Self {
        let headers = HeaderMap::new();
        NotificationsUpdateConfig {
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
    pub fn body<T>(self, body: T) -> NotificationsUpdateConfig<'a, 'b, JsonBody<T>>
    where
        T: Serialize,
    {
        NotificationsUpdateConfig {
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
    #[doc = "Creates an asynchronous call to the Notifications Update Config API that can be awaited"]
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
#[doc = "Namespace client for Notifications APIs"]
pub struct Notifications<'a> {
    transport: &'a Transport,
}
impl<'a> Notifications<'a> {
    #[doc = "Creates a new instance of [Notifications]"]
    pub fn new(transport: &'a Transport) -> Self {
        Self { transport }
    }
    pub fn transport(&self) -> &Transport {
        self.transport
    }
    #[doc = "[Notifications Create Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#create-channel-configuration)\n\nCreate channel configuration."]
    pub fn create_config<'b>(&'a self) -> NotificationsCreateConfig<'a, 'b, ()> {
        NotificationsCreateConfig::new(self.transport())
    }
    #[doc = "[Notifications Delete Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#delete-channel-configuration)\n\nDelete a channel configuration."]
    pub fn delete_config<'b>(
        &'a self,
        parts: NotificationsDeleteConfigParts<'b>,
    ) -> NotificationsDeleteConfig<'a, 'b> {
        NotificationsDeleteConfig::new(self.transport(), parts)
    }
    #[doc = "[Notifications Delete Configs API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#delete-channel-configuration)\n\nDelete multiple channel configurations."]
    pub fn delete_configs<'b>(&'a self) -> NotificationsDeleteConfigs<'a, 'b> {
        NotificationsDeleteConfigs::new(self.transport())
    }
    #[doc = "Notifications Get Config API\n\nGet a specific channel configuration."]
    pub fn get_config<'b>(
        &'a self,
        parts: NotificationsGetConfigParts<'b>,
    ) -> NotificationsGetConfig<'a, 'b> {
        NotificationsGetConfig::new(self.transport(), parts)
    }
    #[doc = "[Notifications Get Configs API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-all-notification-configurations)\n\nGet multiple channel configurations with filtering."]
    pub fn get_configs<'b>(&'a self) -> NotificationsGetConfigs<'a, 'b, ()> {
        NotificationsGetConfigs::new(self.transport())
    }
    #[doc = "[Notifications List Channels API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-all-notification-channels)\n\nList created notification channels."]
    pub fn list_channels<'b>(&'a self) -> NotificationsListChannels<'a, 'b> {
        NotificationsListChannels::new(self.transport())
    }
    #[doc = "[Notifications List Features API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#list-supported-channel-configurations)\n\nList supported channel configurations."]
    pub fn list_features<'b>(&'a self) -> NotificationsListFeatures<'a, 'b> {
        NotificationsListFeatures::new(self.transport())
    }
    #[doc = "[Notifications Send Test API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#send-test-notification)\n\nSend a test notification."]
    pub fn send_test<'b>(
        &'a self,
        parts: NotificationsSendTestParts<'b>,
    ) -> NotificationsSendTest<'a, 'b, ()> {
        NotificationsSendTest::new(self.transport(), parts)
    }
    #[doc = "[Notifications Update Config API](https://opensearch.org/docs/latest/observing-your-data/notifications/api/#update-channel-configuration)\n\nUpdate channel configuration."]
    pub fn update_config<'b>(
        &'a self,
        parts: NotificationsUpdateConfigParts<'b>,
    ) -> NotificationsUpdateConfig<'a, 'b, ()> {
        NotificationsUpdateConfig::new(self.transport(), parts)
    }
}
impl OpenSearch {
    #[doc = "Creates a namespace client for Notifications APIs"]
    pub fn notifications(&self) -> Notifications {
        Notifications::new(self.transport())
    }
}
