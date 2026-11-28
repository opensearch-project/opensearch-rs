/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The fields of the generated response types declared as raw JSON, with
//! the reason each cannot, or should not, be typed. See the parent module.
//!
//! Generation fails when this table and the generated response types disagree, and
//! prints the declarations to add. Keep the entries grouped by namespace and
//! sorted by key.

#[allow(unused_imports)]
use super::UNREVIEWED;
use super::{raw, RawField};
use crate::openapi::responses::RawCause::*;

pub const RAW_FIELDS: &[RawField] = &[
    // --- _common.aggregations ---
    // bare
    raw("_common.aggregations:ScriptedMetricAggregate/value", NoType, "scripted_metric: the reduce script decides the result type"),
    // --- _common.mapping ---
    // bare
    raw("_common.mapping:ConstantKeywordProperty/value", NoType, "constant_keyword value as set in the mapping: user-supplied"),
    // map<bare>
    raw("_common.mapping:KnnVectorMethod/parameters", NoType, "engine/method-specific parameters: the set depends on the chosen method"),
    // map<bare>
    raw("_common.mapping:SemanticChunkingStrategy/parameters", NoType, "engine/method-specific parameters: the set depends on the chosen method"),
    // --- _common.query_dsl ---
    // list<bare>
    raw("_common.query_dsl:GeoShape/coordinates", UntypedItems, "GeoJSON coordinates: nesting depth depends on the geometry type (point, polygon, ...)"),
    // map<bare>
    raw("_common.query_dsl:KnnQuery/method_parameters", OpenObject, "engine/method-specific query parameters: the set depends on the chosen method"),
    // bare
    raw("_common.query_dsl:LikeDocument/doc", NoType, "query input document: user-defined shape"),
    // map<bare> open-object
    raw("_common.query_dsl:NeuralQuery/method_parameters", OpenObject, "engine/method-specific query parameters: the set depends on the chosen method"),
    // bare
    raw("_common.query_dsl:PercolateQuery/document", NoType, "query input document: user-defined shape"),
    // list<bare>
    raw("_common.query_dsl:PercolateQuery/documents", NoType, "query input document: user-defined shape"),
    // map<bare>
    raw("_common.query_dsl:QueryContainer/template", OpenObject, "template query: placeholders resolved by search request processors, arbitrary query DSL"),
    // map<bare> open-object
    raw("_common.query_dsl:StoredLtrQuery/params", OpenObject, "parameters of the stored features: user-defined names and values"),
    // list<bare>
    raw("_common.query_dsl:XyShape/coordinates", UntypedItems, "GeoJSON coordinates: nesting depth depends on the geometry type (point, polygon, ...)"),
    // --- _common ---
    // bare
    raw("_common:ErrorCause/[additional]", NoType, "error-specific extra properties: each exception type adds its own keys"),
    // bare
    raw("_common:InlineGet/[additional]", OpenObject, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_common:InlineGet/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_common:InlineGet/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // map<bare>
    raw("_common:InlineGetDictUserDefined/[additional]", OpenObject, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // map<bare>
    raw("_common:InlineGetDictUserDefined/_source", OpenObject, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<map<bare>>
    raw("_common:InlineGetDictUserDefined/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // map<bare>
    raw("_common:InlineScriptObject/params", OpenObject, "script parameters: user-defined names and values"),
    // bare
    raw("_common:Metadata/[additional]", OpenObject, "user-defined `_meta`: arbitrary JSON by design"),
    // map<bare>
    raw("_common:StoredScriptId/params", OpenObject, "script parameters: user-defined names and values"),
    // bare
    raw("_common:TDocument/-", GenericParameter, "generic type parameter TDocument: the whole get_source response is the document body"),
    // --- _core.explain ---
    // bare
    raw("_core.explain:ExplainResponseGet/[additional]", OpenObject, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.explain:ExplainResponseGet/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.explain:ExplainResponseGet/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- _core.get ---
    // bare
    raw("_core.get:GetResponse/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.get:GetResponse/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.get:GetResult/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.get:GetResult/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- _core.msearch ---
    // bare
    raw("_core.msearch:MultiSearchItemHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.msearch:MultiSearchItemHitsHitsItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.msearch:MultiSearchItemHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- _core.rank_eval ---
    // map<map<bare>>
    raw("_core.rank_eval:RankEvalMetricDetail/metric_details", OpenObject, "metric-specific details: content varies for each rank_eval metric (per spec description)"),
    // map<bare>
    raw("_core.rank_eval:RankEvalResponse/failures", OpenObject, "SPEC GAP: failures keyed by request id, values are error causes and could reference _common___ErrorCause"),
    // --- _core.render_search_template ---
    // map<bare>
    raw("_core.render_search_template:RenderSearchTemplateResponse/template_output", OpenObject, "rendered search template: arbitrary search request body"),
    // --- _core.scripts_painless_execute ---
    // bare
    raw("_core.scripts_painless_execute:ScriptsPainlessExecuteResponse/result", GenericParameter, "generic type parameter TResult: the script decides the result type"),
    // --- _core.search ---
    // map<map<bare>>
    raw("_core.search:Highlight/options", OpenObject, "highlighter-specific options: open set per highlighter implementation"),
    // map<map<bare>>
    raw("_core.search:HighlightField/options", OpenObject, "highlighter-specific options: open set per highlighter implementation"),
    // bare
    raw("_core.search:HitsMetadataHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.search:HitsMetadataHitsItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:HitsMetadataHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.search:HitsMetadataJsonValueHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.search:HitsMetadataJsonValueHitsItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:HitsMetadataJsonValueHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.search:ProcessorExecutionDetail/input_data", NoType, "search pipeline processor input/output: request, response or document fragments depending on the processor"),
    // bare
    raw("_core.search:ProcessorExecutionDetail/output_data", NoType, "search pipeline processor input/output: request, response or document fragments depending on the processor"),
    // bare
    raw("_core.search:SearchResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.search:SearchResponseHitsHitsItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:SearchResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.search:SearchResultHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.search:SearchResultHitsHitsItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:SearchResultHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.search:SuggestCompletionOptionsArrayItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:SuggestCompletionOptionsArrayItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("_core.search:SuggestCompletionOptionsObject/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search:SuggestCompletionOptionsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- _core.search_template ---
    // bare
    raw("_core.search_template:SearchTemplateResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("_core.search_template:SearchTemplateResponseHitsHitsItem/_source", GenericParameter, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("_core.search_template:SearchTemplateResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- cluster.allocation_explain ---
    // bare open-object
    raw("cluster.allocation_explain:NodeResourceUsageStats/[additional]", OpenObject, "per-node resource usage statistics: the structure depends on the version"),
    // --- cluster ---
    // map<bare>
    raw("cluster:GetSettingsResponse/defaults", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // map<bare>
    raw("cluster:GetSettingsResponse/persistent", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // map<bare>
    raw("cluster:GetSettingsResponse/transient", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // map<bare>
    raw("cluster:PutSettingsResponse/persistent", NoType, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // map<bare>
    raw("cluster:PutSettingsResponse/transient", NoType, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // bare
    raw("cluster:RerouteResponse/state", NoType, "cluster state: internal, version- and plugin-dependent structure (spec: may differ from the external representation)"),
    // bare
    raw("cluster:StateResponse/-", NoType, "SPEC GAP: cluster.state response is declared as `{}`; its top-level sections (metadata, routing_table, nodes, ...) could be typed"),
    // --- flow_framework.common ---
    // map<bare> open-object
    raw("flow_framework.common:FlowFrameworkGetResponse/ui_metadata", OpenObject, "SPEC GAP: UI metadata declared as an untyped object; its content is defined by the UI"),
    // map<bare>
    raw("flow_framework.common:FlowFrameworkGetResponse/workflows", OpenObject, "SPEC GAP: workflow name -> workflow definition, value schema missing"),
    // bare no-type
    raw("flow_framework.common:StateItems/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // map<bare> open-object
    raw("flow_framework.common:StateItems/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // map<bare> open-object
    raw("flow_framework.common:WorkFlowStatusFullResponse/user_outputs", OpenObject, "essential API responses of the steps, for backend use and lookup: the keys depend on the workflow"),
    // bare no-type
    raw("flow_framework.common:itemsObject/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // map<bare> open-object
    raw("flow_framework.common:itemsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- indices._common ---
    // bare
    raw("indices._common:IndexSettings/[additional]", NoType, "custom index settings (plugin and user-defined `index.*` keys)"),
    // map<bare> open-object
    raw("indices._common:IngestionSource/mapper_settings", OpenObject, "settings of the selected mapper type: the set depends on the mapper"),
    // map<bare>
    raw("indices._common:IngestionSource/param", OpenObject, "ingestion-source-specific parameters (kafka, kinesis, ...)"),
    // map<bare>
    raw("indices._common:TemplateMapping/settings", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // --- ingest._common ---
    // list<bare>
    raw("ingest._common:AppendProcessor/value", NoType, "user-supplied value of any JSON type (supports template snippets)"),
    // bare
    raw("ingest._common:CsvProcessor/empty_value", NoType, "user-supplied value of any JSON type"),
    // bare
    raw("ingest._common:SetProcessor/value", NoType, "user-supplied value of any JSON type (supports template snippets)"),
    // --- ingest.simulate ---
    // map<bare>
    raw("ingest.simulate:DocumentSimulation/_source", OpenObject, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // --- insights._common ---
    // map<bare>
    raw("insights._common:TopQuery/labels", OpenObject, "user-defined query labels"),
    // --- ism._common ---
    // map<bare>
    raw("ism._common:ActionAllocation/exclude", OpenObject, "SPEC GAP: node attribute -> value map, values are strings (additionalProperties should be string)"),
    // map<bare>
    raw("ism._common:ActionAllocation/include", OpenObject, "SPEC GAP: node attribute -> value map, values are strings (additionalProperties should be string)"),
    // map<bare>
    raw("ism._common:ActionAllocation/require", OpenObject, "SPEC GAP: node attribute -> value map, values are strings (additionalProperties should be string)"),
    // bare
    raw("ism._common:ActionCustom/[additional]", OpenObject, "ISM action configuration: plugin- and version-specific extra options"),
    // map<bare> open-object
    raw("ism._common:ActionRollup/ism_rollup", OpenObject, "the rollup job to run, as in the rollups API: a job definition with its own options"),
    // map<bare> open-object
    raw("ism._common:ActionTransform/ism_transform", OpenObject, "the transform job to run, as in the transforms API: a job definition with its own options"),
    // map<bare> open-object
    raw("ism._common:ExplainActionMetadata/action_properties", OpenObject, "action-specific properties: the set depends on the action type"),
    // map<bare> open-object
    raw("ism._common:ExplainPolicy/info", OpenObject, "additional information about the status of the managed index: free-form, depends on the state"),
    // --- knn._common ---
    // map<bare> open-object
    raw("knn._common:ModelResponse/model_definition", OpenObject, "engine-specific model definition: the set depends on the engine"),
    // map<bare>
    raw("knn._common:NodeStats/indices_in_cache", OpenObject, "SPEC GAP: index name -> per-index cache stats, value schema missing"),
    // --- knn ---
    // bare
    raw("knn:SearchModelsResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("knn:SearchModelsResponseHitsHitsItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("knn:SearchModelsResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("knn:SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("knn:SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("knn:SearchModelsResponseSuggestValueItemCompletionOptionsObject/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("knn:SearchModelsResponseSuggestValueItemCompletionOptionsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- ltr._common ---
    // map<bare>
    raw("ltr._common:CacheStatsResponse/stores", OpenObject, "SPEC GAP: store name -> cache stats, value schema missing"),
    // map<bare>
    raw("ltr._common:StoreDetails/counts", OpenObject, "SPEC GAP: count statistics declared as an untyped object"),
    // --- ltr ---
    // bare
    raw("ltr:GetFeatureResponse/[additional]", OpenObject, "SPEC GAP: LTR get responses are declared as untyped objects (they are document-get shaped)"),
    // bare
    raw("ltr:GetFeaturesetResponse/[additional]", OpenObject, "SPEC GAP: LTR get responses are declared as untyped objects (they are document-get shaped)"),
    // bare
    raw("ltr:GetModelResponse/[additional]", OpenObject, "SPEC GAP: LTR get responses are declared as untyped objects (they are document-get shaped)"),
    // bare
    raw("ltr:SearchFeaturesResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("ltr:SearchFeaturesResponseHitsHitsItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("ltr:SearchFeaturesResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchFeaturesResponseSuggestValueItemCompletionOptionsArrayItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchFeaturesResponseSuggestValueItemCompletionOptionsArrayItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchFeaturesResponseSuggestValueItemCompletionOptionsObject/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchFeaturesResponseSuggestValueItemCompletionOptionsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchFeaturesetsResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("ltr:SearchFeaturesetsResponseHitsHitsItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("ltr:SearchFeaturesetsResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchFeaturesetsResponseSuggestValueItemCompletionOptionsArrayItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchFeaturesetsResponseSuggestValueItemCompletionOptionsArrayItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchFeaturesetsResponseSuggestValueItemCompletionOptionsObject/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchFeaturesetsResponseSuggestValueItemCompletionOptionsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchModelsResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // bare
    raw("ltr:SearchModelsResponseHitsHitsItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare>
    raw("ltr:SearchModelsResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchModelsResponseSuggestValueItemCompletionOptionsArrayItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ltr:SearchModelsResponseSuggestValueItemCompletionOptionsObject/_source", NoType, "document body: its shape is user-defined (decode with serde_json or a caller type)"),
    // map<bare> open-object
    raw("ltr:SearchModelsResponseSuggestValueItemCompletionOptionsObject/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // --- ml._common ---
    // bare
    raw("ml._common:AdditionalInfo/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:Content/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // map<bare>
    raw("ml._common:GetAgenticMemoryResponse/after", OpenObject, "agentic memory content: user-defined"),
    // map<bare>
    raw("ml._common:GetAgenticMemoryResponse/before", OpenObject, "agentic memory content: user-defined"),
    // map<bare>
    raw("ml._common:GetAgenticMemoryResponse/tags", OpenObject, "agentic memory content: user-defined"),
    // bare
    raw("ml._common:Headers/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:Index/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:IndexSettings/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:Metadata/[additional]", OpenObject, "user-defined `_meta`: arbitrary JSON by design"),
    // bare
    raw("ml._common:Namespace/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:Parameters/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare no-type
    raw("ml._common:SearchResponseHitsHitsItem/[additional]", NoType, "unknown metadata fields: plugins and future versions add `_`-prefixed hit/document metadata"),
    // map<bare> open-object
    raw("ml._common:SearchResponseHitsHitsItem/fields", OpenObject, "stored/docvalue fields requested per query: names and value types depend on the mapping"),
    // bare
    raw("ml._common:Source/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:ToolAttributes/[additional]", NoType, "ML tool attributes: tool-specific open set of keys"),
    // bare
    raw("ml._common:ToolItems/[additional]", OpenObject, "ML connector/model/agent configuration: provider-specific open set of keys"),
    // bare
    raw("ml._common:UndeployModelNodeStats/[additional]", OpenObject, "SPEC GAP: model id -> per-node status map, value schema missing"),
    // bare
    raw("ml._common:UnloadModelNodeStats/[additional]", OpenObject, "SPEC GAP: model id -> per-node status map, value schema missing"),
    // --- nodes.info ---
    // map<bare>
    raw("nodes.info:NodeInfoSettingsNode/attr", OpenObject, "custom node attributes: user-defined names (values are strings in practice)"),
    // bare
    raw("nodes.info:NodeInfoSettingsPlugins/[additional]", OpenObject, "plugin settings: open key space"),
    // map<bare> open-object
    raw("nodes.info:NodeInfoSettingsTransport/ssl", OpenObject, "SPEC GAP: transport-layer SSL settings declared as an untyped object"),
    // --- nodes.usage ---
    // map<bare>
    raw("nodes.usage:NodeUsage/aggregations", OpenObject, "aggregation usage counters keyed by aggregation and value-source type: open set"),
    // --- query._common ---
    // map<bare>
    raw("query._common:DataSource/properties", OpenObject, "data-source-connector-specific configuration (s3glue, prometheus, ...)"),
    // map<bare>
    raw("query._common:DataSourceRetrieve/properties", OpenObject, "data-source-connector-specific configuration (s3glue, prometheus, ...)"),
    // --- rollups._common ---
    // map<bare>
    raw("rollups._common:Explain/rollup_metadata", OpenObject, "SPEC GAP: rollup execution metadata declared as an untyped object"),
    // map<bare>
    raw("rollups._common:MetricsConfigMetrics/avg", OpenObject, "rollup metric marker: an empty object (`\"avg\": {}`) by design"),
    // map<bare>
    raw("rollups._common:MetricsConfigMetrics/max", OpenObject, "rollup metric marker: an empty object (`\"avg\": {}`) by design"),
    // map<bare>
    raw("rollups._common:MetricsConfigMetrics/min", OpenObject, "rollup metric marker: an empty object (`\"avg\": {}`) by design"),
    // map<bare>
    raw("rollups._common:MetricsConfigMetrics/sum", OpenObject, "rollup metric marker: an empty object (`\"avg\": {}`) by design"),
    // map<bare>
    raw("rollups._common:MetricsConfigMetrics/value_count", OpenObject, "rollup metric marker: an empty object (`\"avg\": {}`) by design"),
    // --- search_pipeline._common ---
    // map<bare> open-object
    raw("search_pipeline._common:MLInferenceRequestProcessor/model_config", OpenObject, "model configuration: the set depends on the model"),
    // --- search_relevance ---
    // map<bare>
    raw("search_relevance:GetNodeStatsResponse/all_nodes", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // map<bare>
    raw("search_relevance:GetNodeStatsResponse/info", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // map<bare>
    raw("search_relevance:GetNodeStatsResponse/nodes", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // map<bare>
    raw("search_relevance:GetStatsResponse/all_nodes", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // map<bare>
    raw("search_relevance:GetStatsResponse/info", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // map<bare>
    raw("search_relevance:GetStatsResponse/nodes", OpenObject, "SPEC GAP: search relevance stats sections declared as untyped objects"),
    // --- security._common ---
    // bare
    raw("security._common:ComplianceConfig/read_watched_fields", NoType, "SPEC GAP: index pattern -> list of field names, declared as `{}`"),
    // map<bare>
    raw("security._common:DynamicOptions/auth_failure_listeners", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/authc", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/authz", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/http", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/kibana", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/on_behalf_of", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:DynamicOptions/opensearch-dashboards", OpenObject, "SPEC GAP: security config sections declared as `type: object` without properties"),
    // map<bare>
    raw("security._common:HealthInfo/settings", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // --- sql._common ---
    // map<bare>
    raw("sql._common:ExplainBody/description", OpenObject, "query plan step details: engine- and step-specific"),
    // list<list<bare>>
    raw("sql._common:QueryResponse/datarows", UntypedItems, "result rows: each value is typed by its column at runtime (see `schema`)"),
    // list<map<bare>>
    raw("sql._common:QueryResponse/schema", OpenObject, "SPEC GAP: column descriptors with known `name`/`alias`/`type` fields, declared as untyped objects"),
    // map<bare>
    raw("sql._common:SqlSettingsResponse/persistent", OpenObject, "settings: open key space (flat or nested setting names, plugin settings), values of any JSON type"),
    // --- tasks._common ---
    // bare
    raw("tasks._common:RawTaskStatus/[additional]", NoType, "task status is defined by each task type (reindex, update_by_query, plugin tasks)"),
    // --- transforms._common ---
    // list<map<bare>>
    raw("transforms._common:Preview/documents", OpenObject, "transform preview: target index documents, shape defined by the transform"),
];
