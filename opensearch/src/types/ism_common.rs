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

//! Response types of the `ism_common` specification namespace

#![allow(missing_docs)]

#[doc = "An action to perform."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Action {
    #[doc = "Adds or removes an index alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<crate::types::ism_common::ActionAlias>,
    #[doc = "Sets the allocations settings for index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub allocation: Option<crate::types::ism_common::ActionAllocation>,
    #[doc = "Closes an index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close: Option<crate::types::ism_common::ActionClose>,
    #[doc = "Enables custom operations to be performed on the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom: Option<crate::types::ism_common::ActionCustom>,
    #[doc = "Deletes an index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete: Option<crate::types::ism_common::ActionDelete>,
    #[doc = "Force merges an index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_merge: Option<crate::types::ism_common::ActionForceMerge>,
    #[doc = "The priority level of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_priority: Option<crate::types::ism_common::ActionIndexPriority>,
    #[doc = "Sets a policy which determines when to send a notification regarding the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notification: Option<crate::types::ism_common::ActionNotification>,
    #[doc = "Opens an index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<crate::types::ism_common::ActionOpen>,
    #[doc = "Changes the index permissions to read-only."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<crate::types::ism_common::ActionReadOnly>,
    #[doc = "Changes the index permissions to read-write."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_write: Option<crate::types::ism_common::ActionReadWrite>,
    #[doc = "The number of replicas for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replica_count: Option<crate::types::ism_common::ActionReplicaCount>,
    #[doc = "The number of times to retry the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<crate::types::ism_common::ActionRetry>,
    #[doc = "Sets a rollover policy for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollover: Option<crate::types::ism_common::ActionRollover>,
    #[doc = "Determines when to perform an index rollup."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollup: Option<crate::types::ism_common::ActionRollup>,
    #[doc = "Determines when to shrink the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shrink: Option<crate::types::ism_common::ActionShrink>,
    #[doc = "Takes a snapshot of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::ism_common::ActionSnapshot>,
    #[doc = "The amount of time to wait before the action fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::types::ism_common::ActionTimeout>,
    #[doc = "Determines when to perform an index transform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<crate::types::ism_common::ActionTransform>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAlias {
    #[doc = "The alias actions to perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAllocation {
    #[doc = "The excluded allocation attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exclude: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The included allocation attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The required allocation attributes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub require: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "Whether to wait for the allocation to complete."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<bool>,
}

#[doc = "The close action configuration."]
pub type ActionClose = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The custom action configuration."]
pub type ActionCustom = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The delete action configuration."]
pub type ActionDelete = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionForceMerge {
    #[doc = "The maximum number of segments to merge to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_num_segments: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionIndexPriority {
    #[doc = "The priority to set for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
}

#[doc = "The notification action configuration."]
pub type ActionNotification = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The open action configuration."]
pub type ActionOpen = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The read-only action configuration."]
pub type ActionReadOnly = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The read-write action configuration."]
pub type ActionReadWrite = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionReplicaCount {
    #[doc = "The number of replicas to set."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_replicas: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRetry {
    #[doc = "The backoff strategy for retries."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<String>,
    #[doc = "The number of retry attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[doc = "The delay between retry attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRollover {
    #[doc = "Whether to copy the alias to the new index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_alias: Option<bool>,
    #[doc = "The minimum document count to trigger rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<f64>,
    #[doc = "The minimum index age to trigger rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_index_age: Option<String>,
    #[doc = "The minimum primary shard size to trigger rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_primary_shard_size: Option<String>,
    #[doc = "The minimum size to trigger rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<String>,
}

#[doc = "The rollup action configuration."]
pub type ActionRollup = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The shrink action configuration."]
pub type ActionShrink = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionSnapshot {
    #[doc = "Whether to include the global state in the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_global_state: Option<bool>,
    #[doc = "The repository to store the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repository: Option<String>,
    #[doc = "The name of the snapshot."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionTimeout {
    #[doc = "The timeout configuration for the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[doc = "The transform action configuration."]
pub type ActionTransform = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

pub type AddPolicyResponse = crate::types::ism_common::ChangePolicyResponse;

pub type ChangePolicyResponse = crate::types::ism_common::ChangeResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ChangeResponse {
    #[doc = "The list of indices that failed to update."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed_indices: Option<Vec<crate::types::ism_common::FailedIndex>>,
    #[doc = "Whether there were any failures."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failures: Option<bool>,
    #[doc = "The number of updated indices."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_indices: Option<f64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Channel {
    #[doc = "The ID of the notification channel."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

pub type DeletePolicyResponse = crate::types::common::WriteResponseBase;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<crate::types::ism_common::Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::types::ism_common::ErrorNotificationDestination>,
    #[doc = "The template for error notification messages."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_template:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationChime {
    #[doc = "The URL for the Chime webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ErrorNotificationDestination {
    #[doc = "When the notification destination was last updated."]
    pub last_update_time: Option<i64>,
    #[doc = "The name of the notification destination."]
    pub name: Option<String>,
    pub additional: std::collections::BTreeMap<
        String,
        crate::types::ism_common::ErrorNotificationDestinationAdditional,
    >,
}
impl<'de> serde::Deserialize<'de> for ErrorNotificationDestination {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ErrorNotificationDestination;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ErrorNotificationDestination object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_last_update_time: Option<i64> = None;
                let mut seen_last_update_time = false;
                let mut f_name: Option<String> = None;
                let mut seen_name = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::ism_common::ErrorNotificationDestinationAdditional,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "last_update_time" => {
                            if seen_last_update_time {
                                return Err(serde::de::Error::duplicate_field("last_update_time"));
                            }
                            seen_last_update_time = true;
                            f_last_update_time = map.next_value::<Option<i64>>()?;
                        }
                        "name" => {
                            if seen_name {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            seen_name = true;
                            f_name = map.next_value::<Option<String>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ErrorNotificationDestination {
                    last_update_time: f_last_update_time,
                    name: f_name,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ErrorNotificationDestination {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.last_update_time {
            map.serialize_entry("last_update_time", v)?;
        }
        if let Some(v) = &self.name {
            map.serialize_entry("name", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ErrorNotificationDestinationAdditional {
    raw: Box<serde_json::value::RawValue>,
}
impl ErrorNotificationDestinationAdditional {
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {
        &self.raw
    }
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {
        self.raw.get() == "null"
    }
    #[doc = "Parses the value as the `error_notification_chime` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_error_notification_chime(
        &self,
    ) -> std::result::Result<
        crate::types::ism_common::ErrorNotificationChime,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "error_notification_chime")
    }
    #[doc = "Parses the value as the `error_notification_slack` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_error_notification_slack(
        &self,
    ) -> std::result::Result<
        crate::types::ism_common::ErrorNotificationSlack,
        crate::types::UnionBranchError,
    > {
        crate::types::union::parse_branch(&self.raw, "error_notification_slack")
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationSlack {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_webhook: Option<crate::types::ism_common::SlackCustomWebhook>,
    #[doc = "The URL for the Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExplainIndexResponse {
    #[doc = "The total number of managed indices."]
    pub total_managed_indices: Option<f64>,
    pub additional: std::collections::BTreeMap<String, crate::types::ism_common::ExplainPolicy>,
}
impl<'de> serde::Deserialize<'de> for ExplainIndexResponse {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {
            type Value = ExplainIndexResponse;
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("a ExplainIndexResponse object")
            }
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut f_total_managed_indices: Option<f64> = None;
                let mut seen_total_managed_indices = false;
                let mut residual: std::collections::BTreeMap<
                    String,
                    crate::types::ism_common::ExplainPolicy,
                > = Default::default();
                while let Some(k) = map.next_key::<String>()? {
                    match k.as_str() {
                        "total_managed_indices" => {
                            if seen_total_managed_indices {
                                return Err(serde::de::Error::duplicate_field(
                                    "total_managed_indices",
                                ));
                            }
                            seen_total_managed_indices = true;
                            f_total_managed_indices = map.next_value::<Option<f64>>()?;
                        }
                        _ => {
                            residual.insert(k, map.next_value()?);
                        }
                    }
                }
                Ok(ExplainIndexResponse {
                    total_managed_indices: f_total_managed_indices,
                    additional: residual,
                })
            }
        }
        deserializer.deserialize_map(ResidualVisitor)
    }
}
impl serde::Serialize for ExplainIndexResponse {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        if let Some(v) = &self.total_managed_indices {
            map.serialize_entry("total_managed_indices", v)?;
        }
        for (k, v) in &self.additional {
            map.serialize_entry(k, v)?;
        }
        map.end()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainPolicy {
    #[doc = "Whether the policy is enabled for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The ID of the OpenDistro policy applied to the index."]
    #[serde(rename = "index.opendistro.index_state_management.policy_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_opendistro_index_state_management_policy_id: Option<String>,
    #[doc = "The ID of the policy applied to the index."]
    #[serde(rename = "index.plugins.index_state_management.policy_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_plugins_index_state_management_policy_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FailedIndex {
    #[doc = "The name of the failed index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_name: Option<String>,
    #[doc = "The UUID of the failed index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_uuid: Option<String>,
    #[doc = "The reason for the failure."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetPoliciesResponse {
    #[doc = "The list of policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::types::ism_common::PolicyWithMetadata>>,
    #[doc = "The total number of policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_policies: Option<f64>,
}

pub type GetPolicyResponse = crate::types::ism_common::PolicyWithMetadata;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IsmTemplate {
    #[doc = "The index patterns for the ISM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_patterns: Option<Vec<String>>,
    #[doc = "When the ISM template was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The priority of the ISM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
}

#[doc = "The policy defines how an index is managed throughout its lifetime."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Policy {
    #[doc = "The default state an index is in."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_state: Option<String>,
    #[doc = "The description of the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "The configuration for error notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_notification: Option<crate::types::ism_common::ErrorNotification>,
    #[doc = "The ISM template configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ism_template: Option<crate::types::ism_common::PolicyIsmTemplate>,
    #[doc = "When the policy was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<i64>,
    #[doc = "The unique identifier for the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The version of the policy schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<f64>,
    #[doc = "The list of states for the index to transition between."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<crate::types::ism_common::States>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyEnvelope {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::types::ism_common::Policy>,
}

#[doc = "The ISM template configuration."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum PolicyIsmTemplate {
    IsmTemplate(crate::types::ism_common::IsmTemplate),
    Array(Vec<crate::types::ism_common::IsmTemplate>),
    #[doc = "An explicit null value."]
    Null,
}
impl<'de> serde::Deserialize<'de> for PolicyIsmTemplate {
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
            Some(b'{') => serde_json::from_str(text).map(PolicyIsmTemplate::IsmTemplate),
            Some(b'[') => serde_json::from_str(text).map(PolicyIsmTemplate::Array),
            Some(b'n') => return Ok(PolicyIsmTemplate::Null),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "PolicyIsmTemplate"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PolicyWithMetadata {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<f64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::types::ism_common::Policy>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PutPolicyResponse {
    #[serde(rename = "_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::types::common::Id>,
    #[doc = "The primary term of the document."]
    #[serde(rename = "_primary_term")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_term: Option<f64>,
    #[serde(rename = "_seq_no")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(rename = "_version")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<crate::types::common::VersionNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::types::ism_common::PolicyEnvelope>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshSearchAnalyzersResponse {
    #[serde(rename = "_shards")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shards: Option<crate::types::common::ShardStatistics>,
    #[doc = "The details of successful analyzer refreshes."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub successful_refresh_details:
        Option<Vec<crate::types::ism_common::RefreshSearchAnalyzersResponseDetails>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RefreshSearchAnalyzersResponseDetails {
    #[doc = "The name of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The list of refreshed analyzers."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refreshed_analyzers: Option<Vec<String>>,
}

pub type RemovePolicyResponse = crate::types::ism_common::ChangePolicyResponse;

pub type RetryIndexResponse = crate::types::ism_common::ChangeResponse;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SlackCustomWebhook {
    #[doc = "The header parameters for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The host for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The password for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The path for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The port for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The query parameters for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_params: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The scheme for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[doc = "The URL for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The username for the custom Slack webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[doc = "The list of actions to perform and transitions to enter a new state."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct States {
    #[doc = "The list of actions to perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::types::ism_common::Action>>,
    #[doc = "The name of the state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "The list of transitions to enter a new state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<crate::types::ism_common::Transition>>,
}

#[doc = "A transition into a new state."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transition {
    #[doc = "The conditions for the transition."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The name of the state to transition to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_name: Option<String>,
}
