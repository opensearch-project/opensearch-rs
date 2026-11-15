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

//! Response types of the `notifications_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chime {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeleteConfigsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_response_list: Option<crate::types::notifications_common::DeleteResponseList>,
}

pub type DeleteResponseList =
    std::collections::BTreeMap<String, crate::types::notifications_common::RestStatus>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeliveryStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub email_account_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_list: Option<Vec<crate::types::notifications_common::RecipientListItem>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EmailEncryptionMethod {
    #[doc = "`none`"]
    None,
    #[doc = "`ssl`"]
    Ssl,
    #[doc = "`start_tls`"]
    StartTls,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl EmailEncryptionMethod {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            EmailEncryptionMethod::None => "none",
            EmailEncryptionMethod::Ssl => "ssl",
            EmailEncryptionMethod::StartTls => "start_tls",
            EmailEncryptionMethod::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for EmailEncryptionMethod {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for EmailEncryptionMethod {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "none" => EmailEncryptionMethod::None,
            "ssl" => EmailEncryptionMethod::Ssl,
            "start_tls" => EmailEncryptionMethod::StartTls,
            _ => EmailEncryptionMethod::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_group_id_list: Option<Vec<String>>,
    pub recipient_list: Vec<crate::types::notifications_common::RecipientListItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EmailRecipientStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity: Option<crate::types::notifications_common::SeverityType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delivery_status: Option<crate::types::notifications_common::DeliveryStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_recipient_status:
        Option<Vec<crate::types::notifications_common::EmailRecipientStatus>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetConfigsResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_list:
        Option<Vec<crate::types::notifications_common::NotificationsConfigsOutputItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hit_relation: Option<crate::types::notifications_common::TotalHitRelation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

pub type HeaderParamsMap = std::collections::BTreeMap<String, i32>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum HttpMethodType {
    #[doc = "`PATCH`"]
    Patch,
    #[doc = "`POST`"]
    Post,
    #[doc = "`PUT`"]
    Put,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl HttpMethodType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            HttpMethodType::Patch => "PATCH",
            HttpMethodType::Post => "POST",
            HttpMethodType::Put => "PUT",
            HttpMethodType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for HttpMethodType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpMethodType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "PATCH" => HttpMethodType::Patch,
            "POST" => HttpMethodType::Post,
            "PUT" => HttpMethodType::Put,
            _ => HttpMethodType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MicrosoftTeamsItem {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_type: Option<crate::types::notifications_common::NotificationConfigType>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[doc = "Type of notification configuration."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationConfigType {
    #[doc = "`chime`"]
    Chime,
    #[doc = "`email`"]
    Email,
    #[doc = "`email_group`"]
    EmailGroup,
    #[doc = "`microsoft_teams`"]
    MicrosoftTeams,
    #[doc = "`ses_account`"]
    SesAccount,
    #[doc = "`slack`"]
    Slack,
    #[doc = "`smtp_account`"]
    SmtpAccount,
    #[doc = "`sns`"]
    Sns,
    #[doc = "`webhook`"]
    Webhook,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl NotificationConfigType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            NotificationConfigType::Chime => "chime",
            NotificationConfigType::Email => "email",
            NotificationConfigType::EmailGroup => "email_group",
            NotificationConfigType::MicrosoftTeams => "microsoft_teams",
            NotificationConfigType::SesAccount => "ses_account",
            NotificationConfigType::Slack => "slack",
            NotificationConfigType::SmtpAccount => "smtp_account",
            NotificationConfigType::Sns => "sns",
            NotificationConfigType::Webhook => "webhook",
            NotificationConfigType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for NotificationConfigType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for NotificationConfigType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "chime" => NotificationConfigType::Chime,
            "email" => NotificationConfigType::Email,
            "email_group" => NotificationConfigType::EmailGroup,
            "microsoft_teams" => NotificationConfigType::MicrosoftTeams,
            "ses_account" => NotificationConfigType::SesAccount,
            "slack" => NotificationConfigType::Slack,
            "smtp_account" => NotificationConfigType::SmtpAccount,
            "sns" => NotificationConfigType::Sns,
            "webhook" => NotificationConfigType::Webhook,
            _ => NotificationConfigType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationsConfigItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime: Option<crate::types::notifications_common::Chime>,
    pub config_type: crate::types::notifications_common::NotificationConfigType,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<crate::types::notifications_common::Email>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email_group: Option<crate::types::notifications_common::EmailGroup>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub microsoft_teams: Option<crate::types::notifications_common::MicrosoftTeamsItem>,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ses_account: Option<crate::types::notifications_common::SesAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slack: Option<crate::types::notifications_common::SlackItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub smtp_account: Option<crate::types::notifications_common::SmtpAccount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sns: Option<crate::types::notifications_common::SnsItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<crate::types::notifications_common::Webhook>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NotificationsConfigsOutputItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::types::notifications_common::NotificationsConfigItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time_ms: Option<i64>,
}

pub type NotificationsPluginFeaturesMap = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RecipientListItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RestStatus {
    #[doc = "`ACCEPTED`"]
    Accepted,
    #[doc = "`CREATED`"]
    Created,
    #[doc = "`MULTI_STATUS`"]
    MultiStatus,
    #[doc = "`NON_AUTHORITATIVE_INFORMATION`"]
    NonAuthoritativeInformation,
    #[doc = "`NO_CONTENT`"]
    NoContent,
    #[doc = "`OK`"]
    Ok,
    #[doc = "`PARTIAL_CONTENT`"]
    PartialContent,
    #[doc = "`RESET_CONTENT`"]
    ResetContent,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl RestStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            RestStatus::Accepted => "ACCEPTED",
            RestStatus::Created => "CREATED",
            RestStatus::MultiStatus => "MULTI_STATUS",
            RestStatus::NonAuthoritativeInformation => "NON_AUTHORITATIVE_INFORMATION",
            RestStatus::NoContent => "NO_CONTENT",
            RestStatus::Ok => "OK",
            RestStatus::PartialContent => "PARTIAL_CONTENT",
            RestStatus::ResetContent => "RESET_CONTENT",
            RestStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for RestStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RestStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "ACCEPTED" => RestStatus::Accepted,
            "CREATED" => RestStatus::Created,
            "MULTI_STATUS" => RestStatus::MultiStatus,
            "NON_AUTHORITATIVE_INFORMATION" => RestStatus::NonAuthoritativeInformation,
            "NO_CONTENT" => RestStatus::NoContent,
            "OK" => RestStatus::Ok,
            "PARTIAL_CONTENT" => RestStatus::PartialContent,
            "RESET_CONTENT" => RestStatus::ResetContent,
            _ => RestStatus::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SesAccount {
    pub from_address: String,
    pub region: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SeverityType {
    #[doc = "`critical`"]
    Critical,
    #[doc = "`high`"]
    High,
    #[doc = "`info`"]
    Info,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl SeverityType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            SeverityType::Critical => "critical",
            SeverityType::High => "high",
            SeverityType::Info => "info",
            SeverityType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for SeverityType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SeverityType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "critical" => SeverityType::Critical,
            "high" => SeverityType::High,
            "info" => SeverityType::Info,
            _ => SeverityType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlackItem {
    pub url: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SmtpAccount {
    pub from_address: String,
    pub host: String,
    pub method: crate::types::notifications_common::EmailEncryptionMethod,
    pub port: i32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SnsItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_arn: Option<String>,
    pub topic_arn: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TotalHitRelation {
    #[doc = "`eq`"]
    Eq,
    #[doc = "`gte`"]
    Gte,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl TotalHitRelation {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            TotalHitRelation::Eq => "eq",
            TotalHitRelation::Gte => "gte",
            TotalHitRelation::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for TotalHitRelation {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TotalHitRelation {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "eq" => TotalHitRelation::Eq,
            "gte" => TotalHitRelation::Gte,
            _ => TotalHitRelation::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Webhook {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_params: Option<crate::types::notifications_common::HeaderParamsMap>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::types::notifications_common::HttpMethodType>,
    pub url: String,
}
