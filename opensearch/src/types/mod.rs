/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Typed response types generated from the
//! [OpenSearch API specification](https://github.com/opensearch-project/opensearch-api-specification),
//! together with the hand-written runtime support they rely on.
//!
//! # Getting the typed response of an API
//!
//! Response types are generated for every API whose specification declares a
//! JSON success response (APIs answering HEAD only or without a JSON body,
//! such as `indices.exists`, have none). They are obtained through the typed client
//! ([`crate::typed`]), whose `send` returns the response type of the API, so neither
//! the type nor its path needs to be written out:
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # use serde::Deserialize;
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! #[derive(Deserialize)]
//! struct Movie {
//!     title: String,
//! }
//!
//! // a `types::search::SearchResponse`
//! let resp = client
//!     .typed()
//!     .search(SearchParts::Index(&["movies"]))
//!     .send()
//!     .await?;
//! // `None` when the request disabled counting; see `TotalHitCount` for
//! // telling an exact count from a lower bound
//! println!("{:?} hits", resp.hits.total_value());
//! for hit in &resp.hits.hits {
//!     if let Some(movie) = hit.source_as::<Movie>()? {
//!         println!("{}", movie.title);
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! See [`crate::typed`] for errors, and for when to use the generic client
//! instead. With the generic client, a body can still be decoded into a
//! response type with [`Response::json`](crate::http::response::Response::json).
//!
//! # Design notes
//!
//! 100% typing is a non-goal: fields whose shape is decided at runtime
//! (documents, aggregation values, ...) intentionally hold raw JSON and
//! expose it through explicit escape hatches. Additions of such untyped
//! fields are reviewed: each is declared with its reason in the API
//! generator, and generation fails on an undeclared one.
//!
//! Keys of a response that its response type does not define (fields added by
//! plugins or newer OpenSearch versions, or missing from the specification)
//! are ignored, as by the other OpenSearch clients. The typed client can
//! report them as an error instead: see
//! [Fields unknown to the response types](crate::typed#fields-unknown-to-the-response-types).
//! Structs whose specification declares `additionalProperties` keep the
//! remaining keys in that typed map (`additional`, or `aggregations` in
//! buckets). These maps are filled by generated `Deserialize` impls rather
//! than `#[serde(flatten)]`, which cannot decode the `Box<RawValue>`
//! fields these structs contain.
//!
//! # Filtered responses (`filter_path`)
//!
//! The response types describe the full response of an API. The `filter_path`
//! parameter, accepted by almost every API, removes from the response
//! everything it does not match, including fields the response types require:
//! decoding a filtered search response as
//! [`SearchResponse`](search::SearchResponse) fails with an error such as
//! ``missing field `_shards` ``. This is expected, and the response types are not
//! loosened for it, as making every field optional would burden the
//! unfiltered use. The typed client ([`crate::typed`]) therefore does not
//! offer `filter_path`.
//!
//! Send a filtered request with the generic client and decode the response
//! into a type of your own that mirrors what the filter keeps, or into a
//! [`serde_json::Value`]:
//!
//! ```rust,no_run
//! # use opensearch::{OpenSearch, SearchParts};
//! # use serde::Deserialize;
//! # async fn doc() -> Result<(), Box<dyn std::error::Error>> {
//! # let client = OpenSearch::default();
//! // only what `hits.hits._source.title` keeps
//! #[derive(Deserialize)]
//! struct Titles {
//!     hits: Hits,
//! }
//! #[derive(Deserialize)]
//! struct Hits {
//!     hits: Vec<Hit>,
//! }
//! #[derive(Deserialize)]
//! struct Hit {
//!     _source: Movie,
//! }
//! #[derive(Deserialize)]
//! struct Movie {
//!     title: String,
//! }
//!
//! let titles: Titles = client
//!     .search(SearchParts::Index(&["movies"]))
//!     .filter_path(&["hits.hits._source.title"])
//!     .send()
//!     .await?
//!     .json()
//!     .await?;
//! for hit in titles.hits.hits {
//!     println!("{}", hit._source.title);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! A filter can also remove a whole section: with
//! `filter_path=hits.hits._id` a search with no match returns `{}`, so
//! make the fields of such a type optional where the filter may leave
//! nothing.

mod total_hits;
pub mod union;

pub use total_hits::TotalHitCount;
pub use union::UnionBranchError;

// GENERATED-BEGIN:response-modules
// Generated code - do not edit until the next GENERATED-END marker

pub mod asynchronous_search;
pub mod asynchronous_search_common;
pub mod asynchronous_search_delete;
pub mod asynchronous_search_get;
pub mod asynchronous_search_search;
pub mod asynchronous_search_stats;
pub mod bulk;
pub mod bulk_stream;
pub mod cat;
pub mod cat_aliases;
pub mod cat_all_pit_segments;
pub mod cat_allocation;
pub mod cat_cluster_manager;
pub mod cat_count;
pub mod cat_fielddata;
pub mod cat_health;
pub mod cat_indices;
pub mod cat_master;
pub mod cat_nodeattrs;
pub mod cat_nodes;
pub mod cat_pending_tasks;
pub mod cat_pit_segments;
pub mod cat_plugins;
pub mod cat_recovery;
pub mod cat_repositories;
pub mod cat_segment_replication;
pub mod cat_segments;
pub mod cat_shards;
pub mod cat_snapshots;
pub mod cat_tasks;
pub mod cat_templates;
pub mod cat_thread_pool;
pub mod clear_scroll;
pub mod cluster;
pub mod cluster_allocation_explain;
pub mod cluster_common;
pub mod cluster_decommission_awareness;
pub mod cluster_delete_component_template;
pub mod cluster_delete_decommission_awareness;
pub mod cluster_delete_weighted_routing;
pub mod cluster_get_component_template;
pub mod cluster_get_decommission_awareness;
pub mod cluster_get_settings;
pub mod cluster_get_weighted_routing;
pub mod cluster_health;
pub mod cluster_pending_tasks;
pub mod cluster_put_component_template;
pub mod cluster_put_decommission_awareness;
pub mod cluster_put_settings;
pub mod cluster_put_weighted_routing;
pub mod cluster_remote_info;
pub mod cluster_reroute;
pub mod cluster_state;
pub mod cluster_stats;
pub mod common;
pub mod common_aggregations;
pub mod common_analysis;
pub mod common_mapping;
pub mod common_query_dsl;
pub mod count;
pub mod create;
pub mod create_pit;
pub mod dangling_indices;
pub mod dangling_indices_delete_dangling_index;
pub mod dangling_indices_import_dangling_index;
pub mod dangling_indices_list_dangling_indices;
pub mod delete;
pub mod delete_all_pits;
pub mod delete_by_query;
pub mod delete_by_query_rethrottle;
pub mod delete_pit;
pub mod delete_script;
pub mod explain;
pub mod field_caps;
pub mod flow_framework;
pub mod flow_framework_common;
pub mod flow_framework_create;
pub mod flow_framework_delete;
pub mod flow_framework_deprovision;
pub mod flow_framework_get;
pub mod flow_framework_get_status;
pub mod flow_framework_get_steps;
pub mod flow_framework_provision;
pub mod flow_framework_search;
pub mod flow_framework_search_state;
pub mod flow_framework_update;
pub mod geospatial_common;
pub mod geospatial_delete_ip2geo_datasource;
pub mod geospatial_geojson_upload_post;
pub mod geospatial_geojson_upload_put;
pub mod geospatial_get_ip2geo_datasource;
pub mod geospatial_get_upload_stats;
pub mod geospatial_put_ip2geo_datasource;
pub mod geospatial_put_ip2geo_datasource_settings;
pub mod get;
pub mod get_all_pits;
pub mod get_script;
pub mod get_script_context;
pub mod get_script_languages;
pub mod get_source;
pub mod index;
pub mod indices;
pub mod indices_add_block;
pub mod indices_analyze;
pub mod indices_clear_cache;
pub mod indices_clone;
pub mod indices_close;
pub mod indices_common;
pub mod indices_create;
pub mod indices_create_data_stream;
pub mod indices_data_streams_stats;
pub mod indices_delete;
pub mod indices_delete_alias;
pub mod indices_delete_data_stream;
pub mod indices_delete_index_template;
pub mod indices_delete_template;
pub mod indices_flush;
pub mod indices_forcemerge;
pub mod indices_get;
pub mod indices_get_alias;
pub mod indices_get_data_stream;
pub mod indices_get_field_mapping;
pub mod indices_get_index_template;
pub mod indices_get_mapping;
pub mod indices_get_settings;
pub mod indices_get_template;
pub mod indices_get_upgrade;
pub mod indices_open;
pub mod indices_put_alias;
pub mod indices_put_index_template;
pub mod indices_put_mapping;
pub mod indices_put_settings;
pub mod indices_put_template;
pub mod indices_recovery;
pub mod indices_refresh;
pub mod indices_resolve_index;
pub mod indices_rollover;
pub mod indices_segments;
pub mod indices_shard_stores;
pub mod indices_shrink;
pub mod indices_simulate_index_template;
pub mod indices_simulate_template;
pub mod indices_split;
pub mod indices_stats;
pub mod indices_update_aliases;
pub mod indices_upgrade;
pub mod indices_validate_query;
pub mod info;
pub mod ingest;
pub mod ingest_common;
pub mod ingest_delete_pipeline;
pub mod ingest_get_pipeline;
pub mod ingest_processor_grok;
pub mod ingest_put_pipeline;
pub mod ingest_simulate;
pub mod ingestion_common;
pub mod ingestion_get_state;
pub mod ingestion_pause;
pub mod ingestion_resume;
pub mod insights_common;
pub mod insights_top_queries;
pub mod ism_add_policy;
pub mod ism_change_policy;
pub mod ism_common;
pub mod ism_delete_policy;
pub mod ism_explain_policy;
pub mod ism_get_policies;
pub mod ism_get_policy;
pub mod ism_put_policies;
pub mod ism_put_policy;
pub mod ism_refresh_search_analyzers;
pub mod ism_remove_policy;
pub mod ism_retry_index;
pub mod knn;
pub mod knn_common;
pub mod knn_delete_model;
pub mod knn_get_model;
pub mod knn_search_models;
pub mod knn_stats;
pub mod knn_train_model;
pub mod knn_warmup;
pub mod list;
pub mod list_indices;
pub mod list_shards;
pub mod ltr;
pub mod ltr_add_features_to_set;
pub mod ltr_add_features_to_set_by_query;
pub mod ltr_cache_stats;
pub mod ltr_clear_cache;
pub mod ltr_common;
pub mod ltr_create_default_store;
pub mod ltr_create_feature;
pub mod ltr_create_featureset;
pub mod ltr_create_model;
pub mod ltr_create_model_from_set;
pub mod ltr_create_store;
pub mod ltr_delete_default_store;
pub mod ltr_delete_feature;
pub mod ltr_delete_featureset;
pub mod ltr_delete_model;
pub mod ltr_delete_store;
pub mod ltr_get_feature;
pub mod ltr_get_featureset;
pub mod ltr_get_model;
pub mod ltr_get_store;
pub mod ltr_list_stores;
pub mod ltr_search_features;
pub mod ltr_search_featuresets;
pub mod ltr_search_models;
pub mod ltr_stats;
pub mod ltr_update_feature;
pub mod ltr_update_featureset;
pub mod mget;
pub mod ml;
pub mod ml_add_agentic_memory;
pub mod ml_chunk_model;
pub mod ml_common;
pub mod ml_create_connector;
pub mod ml_create_controller;
pub mod ml_create_memory;
pub mod ml_create_memory_container;
pub mod ml_create_memory_container_session;
pub mod ml_create_message;
pub mod ml_create_model_meta;
pub mod ml_delete_agent;
pub mod ml_delete_agentic_memory;
pub mod ml_delete_agentic_memory_query;
pub mod ml_delete_connector;
pub mod ml_delete_controller;
pub mod ml_delete_memory;
pub mod ml_delete_memory_container;
pub mod ml_delete_model;
pub mod ml_delete_model_group;
pub mod ml_delete_task;
pub mod ml_deploy_model;
pub mod ml_execute_agent;
pub mod ml_execute_algorithm;
pub mod ml_execute_tool;
pub mod ml_get_agent;
pub mod ml_get_agentic_memory;
pub mod ml_get_all_memories;
pub mod ml_get_all_messages;
pub mod ml_get_all_tools;
pub mod ml_get_connector;
pub mod ml_get_controller;
pub mod ml_get_memory;
pub mod ml_get_memory_container;
pub mod ml_get_message;
pub mod ml_get_message_traces;
pub mod ml_get_model;
pub mod ml_get_model_group;
pub mod ml_get_profile;
pub mod ml_get_profile_models;
pub mod ml_get_profile_tasks;
pub mod ml_get_stats;
pub mod ml_get_task;
pub mod ml_get_tool;
pub mod ml_load_model;
pub mod ml_predict;
pub mod ml_predict_model;
pub mod ml_register_agents;
pub mod ml_register_model;
pub mod ml_register_model_group;
pub mod ml_register_model_meta;
pub mod ml_search_agentic_memory;
pub mod ml_search_agents;
pub mod ml_search_connectors;
pub mod ml_search_memory;
pub mod ml_search_memory_container;
pub mod ml_search_message;
pub mod ml_search_model_group;
pub mod ml_search_models;
pub mod ml_search_tasks;
pub mod ml_train;
pub mod ml_train_predict;
pub mod ml_undeploy_model;
pub mod ml_unload_model;
pub mod ml_update_agentic_memory;
pub mod ml_update_connector;
pub mod ml_update_controller;
pub mod ml_update_memory;
pub mod ml_update_memory_container;
pub mod ml_update_message;
pub mod ml_update_model;
pub mod ml_update_model_group;
pub mod ml_upload_chunk;
pub mod ml_upload_model;
pub mod msearch;
pub mod msearch_template;
pub mod mtermvectors;
pub mod neural_common;
pub mod neural_stats;
pub mod nodes;
pub mod nodes_info;
pub mod nodes_reload_secure_settings;
pub mod nodes_stats;
pub mod nodes_usage;
pub mod notifications;
pub mod notifications_common;
pub mod notifications_create_config;
pub mod notifications_delete_config;
pub mod notifications_delete_configs;
pub mod notifications_get_config;
pub mod notifications_get_configs;
pub mod notifications_list_channels;
pub mod notifications_list_features;
pub mod notifications_send_test;
pub mod notifications_update_config;
pub mod observability;
pub mod observability_common;
pub mod observability_create_object;
pub mod observability_delete_object;
pub mod observability_delete_objects;
pub mod observability_get_object;
pub mod observability_list_objects;
pub mod observability_update_object;
pub mod pit;
pub mod ppl_explain;
pub mod ppl_query;
pub mod put_script;
pub mod query;
pub mod query_common;
pub mod query_datasource_retrieve;
pub mod query_datasources_create;
pub mod query_datasources_list;
pub mod query_datasources_update;
pub mod rank_eval;
pub mod reindex;
pub mod reindex_rethrottle;
pub mod remote_store;
pub mod remote_store_common;
pub mod remote_store_restore;
pub mod render_search_template;
pub mod replication_autofollow_stats;
pub mod replication_common;
pub mod replication_create_replication_rule;
pub mod replication_delete_replication_rule;
pub mod replication_follower_stats;
pub mod replication_leader_stats;
pub mod replication_pause;
pub mod replication_resume;
pub mod replication_start;
pub mod replication_status;
pub mod replication_stop;
pub mod replication_update_settings;
pub mod rollups_common;
pub mod rollups_delete;
pub mod rollups_explain;
pub mod rollups_get;
pub mod rollups_put;
pub mod rollups_start;
pub mod rollups_stop;
pub mod scripts_painless_execute;
pub mod scroll;
pub mod search;
pub mod search_pipeline;
pub mod search_pipeline_common;
pub mod search_pipeline_delete;
pub mod search_pipeline_get;
pub mod search_pipeline_put;
pub mod search_relevance;
pub mod search_relevance_common;
pub mod search_relevance_delete_experiments;
pub mod search_relevance_delete_judgments;
pub mod search_relevance_delete_query_sets;
pub mod search_relevance_delete_scheduled_experiments;
pub mod search_relevance_delete_search_configurations;
pub mod search_relevance_get_experiments;
pub mod search_relevance_get_judgments;
pub mod search_relevance_get_node_stats;
pub mod search_relevance_get_query_sets;
pub mod search_relevance_get_scheduled_experiments;
pub mod search_relevance_get_search_configurations;
pub mod search_relevance_get_stats;
pub mod search_relevance_post_query_sets;
pub mod search_relevance_post_scheduled_experiments;
pub mod search_relevance_put_experiments;
pub mod search_relevance_put_judgments;
pub mod search_relevance_put_query_sets;
pub mod search_relevance_put_search_configurations;
pub mod search_shards;
pub mod search_template;
pub mod security_analytics_alerts;
pub mod security_analytics_findings;
pub mod security_analytics_get_alerts;
pub mod security_analytics_get_findings;
pub mod security_analytics_search_finding_correlations;
pub mod security_authinfo;
pub mod security_authtoken;
pub mod security_change_password;
pub mod security_common;
pub mod security_config_upgrade_check;
pub mod security_config_upgrade_perform;
pub mod security_create_action_group;
pub mod security_create_allowlist;
pub mod security_create_role;
pub mod security_create_role_mapping;
pub mod security_create_tenant;
pub mod security_create_update_tenancy_config;
pub mod security_create_user;
pub mod security_create_user_legacy;
pub mod security_delete_action_group;
pub mod security_delete_distinguished_name;
pub mod security_delete_role;
pub mod security_delete_role_mapping;
pub mod security_delete_tenant;
pub mod security_delete_user;
pub mod security_delete_user_legacy;
pub mod security_flush_cache;
pub mod security_generate_obo_token;
pub mod security_generate_user_token;
pub mod security_get_account_details;
pub mod security_get_action_group;
pub mod security_get_action_groups;
pub mod security_get_all_certificates;
pub mod security_get_allowlist;
pub mod security_get_audit_configuration;
pub mod security_get_certificates;
pub mod security_get_configuration;
pub mod security_get_dashboards_info;
pub mod security_get_distinguished_name;
pub mod security_get_distinguished_names;
pub mod security_get_node_certificates;
pub mod security_get_permissions_info;
pub mod security_get_role;
pub mod security_get_role_mapping;
pub mod security_get_role_mappings;
pub mod security_get_roles;
pub mod security_get_sslinfo;
pub mod security_get_tenancy_config;
pub mod security_get_tenant;
pub mod security_get_tenants;
pub mod security_get_user;
pub mod security_get_user_legacy;
pub mod security_get_users;
pub mod security_get_users_legacy;
pub mod security_health;
pub mod security_migrate;
pub mod security_patch_action_group;
pub mod security_patch_action_groups;
pub mod security_patch_allowlist;
pub mod security_patch_audit_configuration;
pub mod security_patch_configuration;
pub mod security_patch_distinguished_name;
pub mod security_patch_distinguished_names;
pub mod security_patch_role;
pub mod security_patch_role_mapping;
pub mod security_patch_role_mappings;
pub mod security_patch_roles;
pub mod security_patch_tenant;
pub mod security_patch_tenants;
pub mod security_patch_user;
pub mod security_patch_users;
pub mod security_post_dashboards_info;
pub mod security_reload_http_certificates;
pub mod security_reload_transport_certificates;
pub mod security_tenant_info;
pub mod security_update_audit_configuration;
pub mod security_update_configuration;
pub mod security_update_distinguished_name;
pub mod security_validate;
pub mod security_who_am_i;
pub mod security_who_am_i_protected;
pub mod sm;
pub mod sm_common;
pub mod sm_create_policy;
pub mod sm_delete_policy;
pub mod sm_explain_policy;
pub mod sm_get_policies;
pub mod sm_get_policy;
pub mod sm_start_policy;
pub mod sm_stop_policy;
pub mod sm_update_policy;
pub mod snapshot;
pub mod snapshot_cleanup_repository;
pub mod snapshot_clone;
pub mod snapshot_common;
pub mod snapshot_create;
pub mod snapshot_create_repository;
pub mod snapshot_delete;
pub mod snapshot_delete_repository;
pub mod snapshot_get;
pub mod snapshot_get_repository;
pub mod snapshot_restore;
pub mod snapshot_status;
pub mod snapshot_verify_repository;
pub mod sql_close;
pub mod sql_common;
pub mod sql_explain;
pub mod sql_query;
pub mod sql_settings;
pub mod tasks;
pub mod tasks_cancel;
pub mod tasks_common;
pub mod tasks_get;
pub mod tasks_list;
pub mod termvectors;
pub mod transforms_common;
pub mod transforms_delete;
pub mod transforms_explain;
pub mod transforms_get;
pub mod transforms_preview;
pub mod transforms_put;
pub mod transforms_search;
pub mod transforms_start;
pub mod transforms_stop;
pub mod update;
pub mod update_by_query;
pub mod update_by_query_rethrottle;
pub mod wlm;
pub mod wlm_common;
pub mod wlm_create_query_group;
pub mod wlm_delete_query_group;
pub mod wlm_get_query_group;
pub mod wlm_update_query_group;

// GENERATED-END
