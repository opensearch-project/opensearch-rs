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
    #[doc = "Restores the index from a snapshot as a remote snapshot index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub convert_index_to_remote: Option<crate::types::ism_common::ActionConvertIndexToRemote>,
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
    #[doc = "Computes and publishes the value domains of index fields."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_field_domains: Option<crate::types::ism_common::ActionPublishFieldDomains>,
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
    #[doc = "Scales the index to search-only mode."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_only: Option<crate::types::ism_common::ActionSearchOnly>,
    #[doc = "Determines when to shrink the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shrink: Option<crate::types::ism_common::ActionShrink>,
    #[doc = "Takes a snapshot of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<crate::types::ism_common::ActionSnapshot>,
    #[doc = "Stops cross-cluster replication on a follower index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stop_replication: Option<crate::types::ism_common::ActionStopReplication>,
    #[doc = "The amount of time to wait before the action fails."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<crate::types::common::Duration>,
    #[doc = "Determines when to perform an index transform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transform: Option<crate::types::ism_common::ActionTransform>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAlias {
    #[doc = "The alias actions to perform."]
    pub actions: Vec<crate::types::ism_common::ActionAliasAction>,
}

#[doc = "An alias action on the managed index. Only `add` and `remove` are supported."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAliasAction {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<crate::types::ism_common::ActionAliasAdd>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<crate::types::ism_common::ActionAliasRemove>,
}

#[doc = "Adds aliases to the managed index. Requires exactly one of `alias` or `aliases`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAliasAdd {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<crate::types::common::IndexAlias>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<crate::types::ism_common::ActionAliasNames>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter: Option<crate::types::common_query_dsl::QueryContainer>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_routing: Option<crate::types::common::Routing>,
    #[doc = "Whether the alias is hidden."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_hidden: Option<bool>,
    #[doc = "Whether the managed index is the write index for the alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_write_index: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub routing: Option<crate::types::common::Routing>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_routing: Option<crate::types::common::Routing>,
}

#[doc = "The aliases. Index alias names support date math."]
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum ActionAliasNames {
    IndexAlias(crate::types::common::IndexAlias),
    Array(Vec<crate::types::common::IndexAlias>),
}
impl<'de> serde::Deserialize<'de> for ActionAliasNames {
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
            Some(b'"') => serde_json::from_str(text).map(ActionAliasNames::IndexAlias),
            Some(b'[') => serde_json::from_str(text).map(ActionAliasNames::Array),
            other => {
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {}, expected a {} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "ActionAliasNames"
                )));
            }
        };
        parsed.map_err(serde::de::Error::custom)
    }
}

#[doc = "Removes aliases from the managed index. Requires exactly one of `alias` or `aliases`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionAliasRemove {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alias: Option<crate::types::common::IndexAlias>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases: Option<crate::types::ism_common::ActionAliasNames>,
    #[doc = "Whether the alias must exist to perform the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub must_exist: Option<bool>,
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

#[doc = "The convert index to remote action configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionConvertIndexToRemote {
    #[doc = "Whether to delete the original index after the restore."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delete_original_index: Option<bool>,
    #[doc = "Comma-separated list of index settings to ignore when restoring."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ignore_index_settings: Option<String>,
    #[doc = "Whether to restore the aliases of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_aliases: Option<bool>,
    #[doc = "The number of replicas of the restored index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number_of_replicas: Option<i64>,
    #[doc = "The pattern for the name of the restored index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_pattern: Option<String>,
    #[doc = "The repository that holds the snapshot."]
    pub repository: String,
    #[doc = "The name of the snapshot to restore."]
    pub snapshot: String,
}

#[doc = "The custom action configuration."]
pub type ActionCustom = std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>;

#[doc = "The delete action configuration."]
pub type ActionDelete = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionForceMerge {
    #[doc = "The maximum number of segments to merge to."]
    pub max_num_segments: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionIndexPriority {
    #[doc = "The priority to set for the index."]
    pub priority: i64,
}

#[doc = "The notification action configuration. Requires exactly one of `destination` or `channel`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<crate::types::ism_common::Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::types::ism_common::ErrorNotificationDestination>,
    pub message_template: crate::types::common::Script,
}

#[doc = "The open action configuration."]
pub type ActionOpen = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The publish field domains action configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionPublishFieldDomains {
    #[doc = "The fields to compute the value domains for."]
    pub fields: Vec<crate::types::ism_common::FieldDomain>,
}

#[doc = "The read-only action configuration."]
pub type ActionReadOnly = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The read-write action configuration."]
pub type ActionReadWrite = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionReplicaCount {
    #[doc = "The number of replicas to set."]
    pub number_of_replicas: i64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRetry {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<crate::types::ism_common::ActionRetryBackoff>,
    #[doc = "The number of retry attempts."]
    pub count: i64,
    #[doc = "The delay between retry attempts."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub delay: Option<crate::types::common::Duration>,
}

#[doc = "The backoff strategy for retries."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ActionRetryBackoff {
    #[doc = "`constant`"]
    Constant,
    #[doc = "`exponential`"]
    Exponential,
    #[doc = "`linear`"]
    Linear,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ActionRetryBackoff {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ActionRetryBackoff::Constant => "constant",
            ActionRetryBackoff::Exponential => "exponential",
            ActionRetryBackoff::Linear => "linear",
            ActionRetryBackoff::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ActionRetryBackoff {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ActionRetryBackoff {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "constant" => ActionRetryBackoff::Constant,
            "exponential" => ActionRetryBackoff::Exponential,
            "linear" => ActionRetryBackoff::Linear,
            _ => ActionRetryBackoff::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRollover {
    #[doc = "Whether to copy the alias to the new index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_alias: Option<bool>,
    #[doc = "The minimum document count to trigger rollover."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<i64>,
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
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionRollup {
    #[doc = "The rollup job to run on the index."]
    pub ism_rollup: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}

#[doc = "The search-only action configuration."]
pub type ActionSearchOnly = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The shrink action configuration. Requires exactly one of `num_new_shards`, `max_shard_size` or `percentage_of_source_shards`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionShrink {
    #[doc = "The aliases to add to the shrunken index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliases:
        Option<Vec<std::collections::BTreeMap<String, crate::types::indices_common::Alias>>>,
    #[doc = "Whether to shrink the index even if it has no replicas."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub force_unsafe: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_shard_size: Option<crate::types::common::HumanReadableByteCount>,
    #[doc = "The number of primary shards of the shrunken index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub num_new_shards: Option<i64>,
    #[doc = "The number of primary shards of the shrunken index as a fraction of the source shards."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentage_of_source_shards: Option<f64>,
    #[doc = "Whether to move the aliases of the source index to the shrunken index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub switch_aliases: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_index_name_template: Option<crate::types::common::Script>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionSnapshot {
    #[doc = "The repository to store the snapshot."]
    pub repository: String,
    #[doc = "The name of the snapshot."]
    pub snapshot: String,
}

#[doc = "The stop replication action configuration."]
pub type ActionStopReplication = std::collections::BTreeMap<String, serde_json::Value>;

#[doc = "The transform action configuration."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionTransform {
    #[doc = "The transform job to run on the index."]
    pub ism_transform: std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>,
}

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
    pub updated_indices: Option<i64>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Channel {
    #[doc = "The ID of the notification channel."]
    pub id: String,
}

pub type DeletePolicyResponse = crate::types::common::WriteResponseBase;

#[doc = "The error notification configuration. Requires exactly one of `destination` or `channel`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotification {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<crate::types::ism_common::Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<crate::types::ism_common::ErrorNotificationDestination>,
    pub message_template: crate::types::common::Script,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationChime {
    #[doc = "The URL for the Chime webhook."]
    pub url: String,
}

#[doc = "The custom webhook. Requires `url` or `host`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationCustomWebhook {
    #[doc = "The header parameters for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub header_params: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The host for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host: Option<String>,
    #[doc = "The password for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[doc = "The path for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "The port for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,
    #[doc = "The query parameters for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_params: Option<std::collections::BTreeMap<String, String>>,
    #[doc = "The scheme for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<String>,
    #[doc = "The URL for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[doc = "The username for the custom webhook."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

#[doc = "The notification destination. Requires exactly one of `chime`, `slack` or `custom_webhook`."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationDestination {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chime: Option<crate::types::ism_common::ErrorNotificationChime>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_webhook: Option<crate::types::ism_common::ErrorNotificationCustomWebhook>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slack: Option<crate::types::ism_common::ErrorNotificationSlack>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorNotificationSlack {
    #[doc = "The URL for the Slack webhook."]
    pub url: String,
}

#[doc = "The current action of the managed index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainActionMetadata {
    #[doc = "The properties of the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_properties:
        Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[doc = "The number of retries the action has consumed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_retries: Option<i64>,
    #[doc = "Whether the action has failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
    #[doc = "The position of the action in the state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_retry_time: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The name of the action."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[derive(Debug, Clone)]
pub struct ExplainIndexResponse {
    #[doc = "The total number of managed indices."]
    pub total_managed_indices: Option<i64>,
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
                let mut f_total_managed_indices: Option<i64> = None;
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
                            f_total_managed_indices = map.next_value::<Option<i64>>()?;
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<crate::types::ism_common::ExplainActionMetadata>,
    #[doc = "Whether the policy is enabled for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "The name of the managed index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[doc = "The ID of the OpenDistro policy applied to the index."]
    #[serde(rename = "index.opendistro.index_state_management.policy_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_opendistro_index_state_management_policy_id: Option<String>,
    #[doc = "The ID of the policy applied to the index."]
    #[serde(rename = "index.plugins.index_state_management.policy_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_plugins_index_state_management_policy_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_creation_date: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The UUID of the managed index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_uuid: Option<String>,
    #[doc = "Additional information about the current status of the managed index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub info: Option<std::collections::BTreeMap<String, Box<serde_json::value::RawValue>>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy: Option<crate::types::ism_common::Policy>,
    #[doc = "Whether the policy has completed for the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_completed: Option<bool>,
    #[doc = "The ID of the policy applied to the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The primary term of the policy applied to the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_primary_term: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_seq_no: Option<crate::types::common::SequenceNumber>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry_info: Option<crate::types::ism_common::ExplainRetryInfo>,
    #[doc = "Whether the index has been rolled over."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolled_over: Option<bool>,
    #[doc = "The name of the index the managed index was rolled over to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolled_over_index_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<crate::types::ism_common::ExplainStateMetadata>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<crate::types::ism_common::ExplainStepMetadata>,
    #[doc = "The state the index is transitioning to."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transition_to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validate: Option<crate::types::ism_common::ExplainValidationResult>,
}

#[doc = "The retry information of the managed index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainRetryInfo {
    #[doc = "The number of retries the policy has consumed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consumed_retries: Option<i64>,
    #[doc = "Whether the policy has failed."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failed: Option<bool>,
}

#[doc = "The current state of the managed index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainStateMetadata {
    #[doc = "The name of the state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::EpochTimeUnitMillis>,
}

#[doc = "The current step of the managed index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainStepMetadata {
    #[doc = "The name of the step."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<crate::types::common::EpochTimeUnitMillis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step_status: Option<crate::types::ism_common::StepStatus>,
}

#[doc = "The validation result of the current action of the managed index."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExplainValidationResult {
    #[doc = "The validation message."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation_status: Option<crate::types::ism_common::ValidationStatus>,
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
pub struct FieldDomain {
    #[doc = "The name of the field."]
    pub field: String,
    pub r#type: crate::types::ism_common::FieldDomainType,
}

#[doc = "The type of the field domain calculator."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FieldDomainType {
    #[doc = "`date_range`"]
    DateRange,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl FieldDomainType {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            FieldDomainType::DateRange => "date_range",
            FieldDomainType::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for FieldDomainType {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FieldDomainType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "date_range" => FieldDomainType::DateRange,
            _ => FieldDomainType::Unknown(s),
        })
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GetPoliciesResponse {
    #[doc = "The list of policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policies: Option<Vec<crate::types::ism_common::PolicyWithMetadata>>,
    #[doc = "The total number of policies."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_policies: Option<i64>,
}

pub type GetPolicyResponse = crate::types::ism_common::PolicyWithMetadata;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IsmTemplate {
    #[doc = "The index patterns for the ISM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub index_patterns: Option<Vec<String>>,
    #[doc = "When the ISM template was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The priority of the ISM template."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i64>,
}

#[doc = "The policy defines how an index is managed throughout its lifetime."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Policy {
    #[doc = "The default state an index is in."]
    pub default_state: String,
    #[doc = "The description of the policy."]
    pub description: String,
    #[doc = "The configuration for error notifications."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_notification: Option<crate::types::ism_common::ErrorNotification>,
    #[doc = "The ISM template configuration."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ism_template: Option<crate::types::ism_common::PolicyIsmTemplate>,
    #[doc = "When the policy was last updated."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_time: Option<crate::types::common::EpochTimeUnitMillis>,
    #[doc = "The unique identifier for the policy."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[doc = "The version of the policy schema."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    #[doc = "The list of states for the index to transition between."]
    pub states: Vec<crate::types::ism_common::States>,
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
    pub primary_term: Option<i64>,
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
    pub primary_term: Option<i64>,
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

#[doc = "The list of actions to perform and transitions to enter a new state."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct States {
    #[doc = "The list of actions to perform."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<crate::types::ism_common::Action>>,
    #[doc = "The name of the state."]
    pub name: String,
    #[doc = "The list of transitions to enter a new state."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transitions: Option<Vec<crate::types::ism_common::Transition>>,
}

#[doc = "The status of the step."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StepStatus {
    #[doc = "`completed`"]
    Completed,
    #[doc = "`condition_not_met`"]
    ConditionNotMet,
    #[doc = "`failed`"]
    Failed,
    #[doc = "`starting`"]
    Starting,
    #[doc = "`timed_out`"]
    TimedOut,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl StepStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            StepStatus::Completed => "completed",
            StepStatus::ConditionNotMet => "condition_not_met",
            StepStatus::Failed => "failed",
            StepStatus::Starting => "starting",
            StepStatus::TimedOut => "timed_out",
            StepStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for StepStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for StepStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "completed" => StepStatus::Completed,
            "condition_not_met" => StepStatus::ConditionNotMet,
            "failed" => StepStatus::Failed,
            "starting" => StepStatus::Starting,
            "timed_out" => StepStatus::TimedOut,
            _ => StepStatus::Unknown(s),
        })
    }
}

#[doc = "A transition into a new state."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Transition {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<crate::types::ism_common::TransitionConditions>,
    #[doc = "The name of the state to transition to."]
    pub state_name: String,
}

#[doc = "The condition for the transition. Accepts at most one condition."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransitionConditions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cron: Option<crate::types::ism_common::TransitionCron>,
    #[doc = "The minimum document count of the index."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_doc_count: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_index_age: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_rollover_age: Option<crate::types::common::Duration>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_size: Option<crate::types::common::HumanReadableByteCount>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_state_age: Option<crate::types::common::Duration>,
    #[doc = "If `true`, the index must have no aliases. If `false`, the index must have at least one alias."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_alias: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransitionCron {
    pub cron: crate::types::rollups_common::Cron,
}

#[doc = "The validation status."]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ValidationStatus {
    #[doc = "`failed`"]
    Failed,
    #[doc = "`passed`"]
    Passed,
    #[doc = "`re_validating`"]
    ReValidating,
    #[doc = "A value not defined by the specification, kept verbatim so that it round-trips."]
    Unknown(String),
}
impl ValidationStatus {
    #[doc = "The JSON string of the value"]
    pub fn as_str(&self) -> &str {
        match self {
            ValidationStatus::Failed => "failed",
            ValidationStatus::Passed => "passed",
            ValidationStatus::ReValidating => "re_validating",
            ValidationStatus::Unknown(s) => s.as_str(),
        }
    }
}
impl serde::Serialize for ValidationStatus {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ValidationStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "failed" => ValidationStatus::Failed,
            "passed" => ValidationStatus::Passed,
            "re_validating" => ValidationStatus::ReValidating,
            _ => ValidationStatus::Unknown(s),
        })
    }
}
