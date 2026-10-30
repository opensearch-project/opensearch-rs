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

//! Response types of the `security_common` specification namespace

#![allow(missing_docs)]

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccountDetails {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_names: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_internal_user: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenants: Option<crate::types::security_common::UserTenants>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_requested_tenant: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionGroup {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

pub type ActionGroupsMap =
    std::collections::BTreeMap<String, crate::types::security_common::ActionGroup>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AllowListConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "An object with APIs as key and array of HTTP methods as values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requests: Option<std::collections::BTreeMap<String, Vec<String>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audit: Option<crate::types::security_common::AuditLogsConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compliance: Option<crate::types::security_common::ComplianceConfig>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditConfigWithReadOnly {
    #[serde(rename = "_readonly")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub readonly: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::types::security_common::AuditConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuditLogsConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_rest_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_transport_categories: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_rest: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_transport: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude_sensitive_headers: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_headers: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_requests: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_url_params: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_users: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_request_body: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolve_bulk_requests: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolve_indices: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AuthInfo {
    #[doc = "The backend roles associated with the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[doc = "The name of the attributes associated with the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_attribute_names: Option<Vec<String>>,
    #[doc = "The number of peer certificates related to the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_certificates: Option<crate::types::security_common::AuthInfoPeerCertificates>,
    #[doc = "The user's principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[doc = "The IP address of remote user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_address: Option<String>,
    #[doc = "The roles associated with the user."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[doc = "The size of the user's backend roles in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_of_backendroles: Option<String>,
    #[doc = "The size of the user's custom attributes in bytes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_of_custom_attributes: Option<String>,
    #[doc = "The size of user contained in memory."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_of_user: Option<String>,
    #[doc = "The logout URL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sso_logout_url: Option<String>,
    #[doc = "The tenants the user has access to with `read-write` or `read-only` access indicators."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenants: Option<std::collections::BTreeMap<String, bool>>,
    #[doc = "A user object as a string."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[doc = "The username."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[doc = "The name of the tenant the user would like to switch to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_requested_tenant: Option<String>,
}

#[doc = "The number of peer certificates related to the user."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum AuthInfoPeerCertificates {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for AuthInfoPeerCertificates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(AuthInfoPeerCertificates::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(AuthInfoPeerCertificates::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "AuthInfoPeerCertificates"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CertificateTypes {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<
        Vec<std::collections::BTreeMap<String, crate::types::security_common::CertificatesDetail>>,
    >,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport: Option<
        Vec<std::collections::BTreeMap<String, crate::types::security_common::CertificatesDetail>>,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CertificatesDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issuer_dn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_after: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub san: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CertificatesPerNode {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub certificates:
        Option<std::collections::BTreeMap<String, crate::types::security_common::CertificateTypes>>,
    #[doc = "The name of the node."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ComplianceConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_config: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub internal_config: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_ignore_users: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_metadata_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_watched_fields: Option<Box<serde_json::value::RawValue>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_ignore_users: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_log_diffs: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_metadata_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub write_watched_indices: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DashboardsInfo {
    #[doc = "The default tenant setting for the dashboard."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_tenant: Option<String>,
    #[doc = "Indicates whether multi-tenancy is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multitenancy_enabled: Option<bool>,
    #[doc = "Indicates whether `DNFOF` is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_fail_on_forbidden_enabled: Option<bool>,
    #[doc = "The name of the dashboard's index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opensearch_dashboards_index: Option<String>,
    #[doc = "Indicates whether multi-tenancy is enabled."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opensearch_dashboards_mt_enabled: Option<bool>,
    #[doc = "The name of the user used to connect dashboard's to the server."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opensearch_dashboards_server_user: Option<String>,
    #[doc = "The error message when a password validation fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_validation_error_message: Option<String>,
    #[doc = "The regular expression used perform password validation."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password_validation_regex: Option<String>,
    #[doc = "Indicates whether a private tenant is enabled for all users."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_tenant_enabled: Option<bool>,
    #[doc = "A list of available sign-in options."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_in_options: Option<Vec<String>>,
    #[doc = "User's name"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DistinguishedNames {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes_dn: Option<Vec<String>>,
}

pub type DistinguishedNamesMap =
    std::collections::BTreeMap<String, crate::types::security_common::DistinguishedNames>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dynamic: Option<crate::types::security_common::DynamicOptions>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DynamicOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_failure_listeners:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authc: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authz: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_intertransport_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disable_rest_auth: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub do_not_fail_on_forbidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub do_not_fail_on_forbidden_empty: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filtered_alias_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts_resolver_mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kibana: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multi_rolespan_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(rename = "opensearch-dashboards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opensearch_dashboards:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub respect_request_indices_options: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GenerateOBOToken {
    #[doc = "The generated OBO token."]
    #[serde(rename = "authenticationToken")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authentication_token: Option<String>,
    #[doc = "The duration of the token."]
    #[serde(rename = "durationSeconds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<String>,
    #[doc = "The name of the entity requesting token."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetCertificates {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http_certificates_list: Option<Vec<crate::types::security_common::CertificatesDetail>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transport_certificates_list: Option<Vec<crate::types::security_common::CertificatesDetail>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetCertificatesNew {
    #[doc = "Contains statistics about the number of nodes selected by the request's node filters."]
    #[serde(rename = "_nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nodes: Option<crate::types::common::NodeStatistics>,
    #[doc = "Name of this cluster."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_name: Option<String>,
    #[serde(rename = "nodes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub _nodes: Option<
        std::collections::BTreeMap<String, crate::types::security_common::CertificatesPerNode>,
    >,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HealthInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settings: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IndexPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dls: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fls: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_patterns: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub masked_fields: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MultiTenancyConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_tenant: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub multitenancy_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub private_tenant_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_in_options: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Ok {
    #[doc = "The message returned as part of an `OK` response."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<crate::types::security_common::OkStatus>,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum OkStatus {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for OkStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => serde_json::from_str(text).map(OkStatus::Number),
            Some(b'"') => serde_json::from_str(text).map(OkStatus::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "OkStatus"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PermissionsInfo {
    #[doc = "An object with disabled APIs as keys and an array of HTTP methods as values."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_endpoints: Option<std::collections::BTreeMap<String, Vec<String>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_api_access: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Role {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cluster_permissions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_permissions: Option<Vec<crate::types::security_common::IndexPermission>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_permissions: Option<Vec<crate::types::security_common::TenantPermission>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RoleMapping {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub and_backend_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosts: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<String>>,
}

pub type RoleMappings =
    std::collections::BTreeMap<String, crate::types::security_common::RoleMapping>;

pub type RolesMap = std::collections::BTreeMap<String, crate::types::security_common::Role>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SSLInfo {
    #[doc = "A list of domain names from local certificates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_certificates_list: Option<Vec<String>>,
    #[doc = "The number of certificates."]
    pub peer_certificates: crate::types::security_common::SSLInfoPeerCertificates,
    #[doc = "A list of domain names from peer certificates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub peer_certificates_list: Option<Vec<String>>,
    #[doc = "The user's principal."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub principal: Option<String>,
    #[doc = "The cipher for this SSL setup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_cipher: Option<String>,
    #[doc = "Whether OpenSSL is available."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_available: Option<bool>,
    #[doc = "The reason OpenSSL is unavailable."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_non_available_cause: Option<String>,
    #[doc = "Whether the hostname validation is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_supports_hostname_validation: Option<bool>,
    #[doc = "Whether `KMF` is supported."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_supports_key_manager_factory: Option<bool>,
    #[doc = "Version of OpenSSL."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_version: Option<crate::types::security_common::SSLInfoSslOpensslVersion>,
    #[doc = "The full version string for the OpenSSL version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_openssl_version_string: Option<String>,
    #[doc = "The protocol for this SSL setup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_protocol: Option<String>,
    #[doc = "Returns the HTTP provider's name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ssl_provider_http: Option<String>,
    #[doc = "Returns the transport client's name."]
    pub ssl_provider_transport_client: String,
    #[doc = "Returns the transport server's name."]
    pub ssl_provider_transport_server: String,
}

#[doc = "The number of certificates."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SSLInfoPeerCertificates {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SSLInfoPeerCertificates {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SSLInfoPeerCertificates::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SSLInfoPeerCertificates::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SSLInfoPeerCertificates"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Version of OpenSSL."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SSLInfoSslOpensslVersion {
    Number(f64),
    String(String),
}
impl<'de> serde::Deserialize<'de> for SSLInfoSslOpensslVersion {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {
            Some(b'-') | Some(b'0'..=b'9') => {
                serde_json::from_str(text).map(SSLInfoSslOpensslVersion::Number)
            }
            Some(b'"') => serde_json::from_str(text).map(SSLInfoSslOpensslVersion::String),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "SSLInfoSslOpensslVersion"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub config: Option<crate::types::security_common::DynamicConfig>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tenant {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
}

pub type TenantInfo = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TenantPermission {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allowed_actions: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tenant_patterns: Option<Vec<String>>,
}

pub type TenantsMap = std::collections::BTreeMap<String, crate::types::security_common::Tenant>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpgradeCheck {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "upgradeActions")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_actions:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>>>,
    #[serde(rename = "upgradeAvailable")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrade_available: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpgradePerform {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upgrades:
        Option<std::collections::BTreeMap<String, std::collections::BTreeMap<String, Vec<String>>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<crate::types::security_common::UserAttributes>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hash: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub opendistro_security_roles: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reserved: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#static: Option<bool>,
}

pub type UserAttributes = std::collections::BTreeMap<String, String>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserTenants {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub admin_tenant: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_tenant: Option<bool>,
}

pub type UsersMap = std::collections::BTreeMap<String, crate::types::security_common::User>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WhoAmI {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dn: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_admin: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_node_certificate_request: Option<bool>,
}
