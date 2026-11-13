/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Builds a model of typed response types.
//!
//! For each requested operation group, the success response body is taken
//! from the resolved client model of the specification
//! ([ClientModel](super::client_model::ClientModel), whose `$ref`s, `allOf`s
//! and unions are already resolved), and its transitive type closure is
//! converted into [TypeModel]s by [read_response_model], from which the
//! `response_types` code generation backend renders Rust types into
//! `opensearch/src/types/`. Operation groups and status codes come from the
//! OpenAPI document.
//!
//! # Union strategies
//!
//! JSON carries no discriminator for `oneOf`/`anyOf` values, so unions are
//! classified into one of three shapes:
//!
//! - **String enums**: unions whose variants are all fixed string values
//!   become open-set Rust enums (unknown values fall into an `Unknown`
//!   variant instead of failing the whole response).
//! - **Dispatch unions**: when every variant has a distinct JSON kind
//!   (object / array / string / number / boolean), a Rust enum with a
//!   generated `Deserialize` visitor dispatches on the kind of the
//!   incoming value. No trial-and-error decoding takes place.
//! - **Lazy unions**: when variant shapes overlap (notably aggregations,
//!   where dozens of variants are all objects), the raw JSON is captured
//!   unparsed and only decoded when a branch accessor is called by the
//!   caller, who knows which branch to expect having written the request.

use super::client_model::ClientModel;
use inflector::Inflector;
use serde_json::{Map, Value};
use std::collections::{BTreeMap, BTreeSet};

/// The Rust path, relative to the `opensearch` crate root, of the typed
/// response type of an operation group in the given model: e.g.
/// `types::search::SearchResponse` for `search`. Returns `None` for groups
/// without a response type (see [ResponseModel::untyped_groups]).
///
/// The path follows the per-endpoint alias rule of the `response_types`
/// code generation backend: module `<group with '.' replaced by '_'>`,
/// type `<PascalCase of the last group segment>Response`. The backend
/// guarantees a type with this exact path exists for every generated
/// group, either as the root type itself or as an emitted alias.
pub fn typed_response_path(model: &ResponseModel, operation_group: &str) -> Option<String> {
    if !model.endpoints.contains_key(operation_group) {
        return None;
    }
    let (module, name) = endpoint_response_alias(operation_group);
    Some(format!("types::{}::{}", module, name))
}

/// The `(module, type name)` under which the response of an operation
/// group is reachable in the generated `types` tree, e.g.
/// `("search", "SearchResponse")`. Shared by the code generation backend
/// (which emits an alias when the root type lives elsewhere) and by
/// [typed_response_path].
pub fn endpoint_response_alias(operation_group: &str) -> (String, String) {
    let module = operation_group.replace('.', "_");
    let name = operation_group.split('.').next_back().unwrap();
    (module, format!("{}Response", name.to_pascal_case()))
}

/// The model of the response types for a set
/// of operation groups
#[derive(Debug, Default)]
pub struct ResponseModel {
    /// operation group name -> type key of the root response type
    pub endpoints: BTreeMap<String, String>,
    /// type key (`namespace___TypeName`) -> type definition
    pub types: BTreeMap<String, TypeModel>,
    /// Operation groups requested but left without a response type, with
    /// the reason: no success response is declared, or the success response
    /// carries no JSON body (HEAD endpoints, text-only `cat` output, ...)
    pub untyped_groups: BTreeMap<String, String>,
    /// operation group name -> the non-2xx status codes for which the
    /// specification declares the same body as the response type, e.g.
    /// `404` of `get` (`{"found": false, ...}`): a typed call decodes these
    /// as the response type rather than reporting an error. Every other non-2xx
    /// status is an error response.
    pub typed_statuses: BTreeMap<String, Vec<u16>>,
}

/// The success status codes whose response schema is taken as the response
/// response type of an operation group, in order of preference. `create` for
/// instance only declares `201`, the dangling-indices APIs only `202`.
pub(crate) const SUCCESS_STATUS_CODES: &[&str] = &["200", "201", "202"];

/// Whether every operation of a group uses the HEAD method
pub(crate) fn is_head_only(spec: &Value, group: &str) -> bool {
    let mut methods = BTreeSet::new();
    if let Some(paths) = spec["paths"].as_object() {
        for operations in paths.values() {
            if let Some(operations) = operations.as_object() {
                for (method, op) in operations {
                    if op["x-operation-group"].as_str() == Some(group) {
                        methods.insert(method.to_lowercase());
                    }
                }
            }
        }
    }
    !methods.is_empty() && methods.iter().all(|m| m == "head")
}

/// All operation groups declared in an OpenAPI document (`x-operation-group`
/// of every operation), sorted. Response types are generated for every
/// one of them that declares a JSON success response; the others are
/// listed in [ResponseModel::untyped_groups] with the reason. The raw
/// fields a group introduces must be declared, with the reason, in
/// `openapi::raw_fields` (generation fails and prints the declarations).
pub fn all_operation_groups(spec: &Value) -> Vec<String> {
    let mut groups = BTreeSet::new();
    if let Some(paths) = spec["paths"].as_object() {
        for operations in paths.values() {
            if let Some(operations) = operations.as_object() {
                for op in operations.values() {
                    if let Some(g) = op["x-operation-group"].as_str() {
                        groups.insert(g.to_string());
                    }
                }
            }
        }
    }
    groups.into_iter().collect()
}

/// Reads the response model of every operation group of the document, with
/// the types of its client model
pub fn read_all_response_models(
    spec: &Value,
    model: &ClientModel,
) -> anyhow::Result<ResponseModel> {
    let groups = all_operation_groups(spec);
    let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
    super::model_reader::read_response_model(spec, model, &refs)
}

/// A type in the response model, keyed by its type key
#[derive(Debug)]
pub enum TypeModel {
    Struct(StructModel),
    /// A union whose variants have pairwise distinct JSON kinds
    DispatchUnion(UnionModel),
    /// A union whose variants cannot be told apart cheaply; decoded lazily
    LazyUnion(UnionModel),
    /// An open set of fixed string values
    StrEnum(StrEnumModel),
    /// A named alias of a scalar, list, map or raw type
    Alias(AliasModel),
}

#[derive(Debug)]
pub struct StructModel {
    pub description: Option<String>,
    pub fields: Vec<FieldModel>,
    /// Typed `additionalProperties` alongside fixed fields: the remaining
    /// keys are collected into a map field with this (name, value type)
    pub residual: Option<(String, TypeRef)>,
}

#[derive(Debug)]
pub struct FieldModel {
    /// The JSON property name
    pub json_name: String,
    pub ty: TypeRef,
    /// Present in the schema's `required` list
    pub required: bool,
    /// The schema allows an explicit `null` value
    pub nullable: bool,
    pub description: Option<String>,
}

#[derive(Debug)]
pub struct UnionModel {
    pub description: Option<String>,
    pub variants: Vec<VariantModel>,
    /// The union has a `null` variant
    pub nullable: bool,
}

#[derive(Debug)]
pub struct VariantModel {
    /// Variant name in snake_case, from the variant `title` or `$ref` name
    pub name: String,
    pub ty: TypeRef,
    /// JSON kind, filled in by strategy resolution; `None` when the kind
    /// cannot be determined statically
    pub kind: Option<JsonKind>,
}

#[derive(Debug)]
pub struct StrEnumModel {
    pub description: Option<String>,
    pub values: Vec<StrEnumValue>,
}

/// One value of a string enum
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrEnumValue {
    /// The spelling written back when serializing
    pub value: String,
    /// Other spellings the specification lists for the same value, e.g.
    /// `async` next to `ASYNC`: accepted when deserializing
    pub aliases: Vec<String>,
}

#[derive(Debug)]
pub struct AliasModel {
    pub description: Option<String>,
    pub ty: TypeRef,
}

/// A reference to a type from a field, item, or variant position
#[derive(Debug, Clone, PartialEq)]
pub enum TypeRef {
    /// Another type in the model, by type key
    Named(String),
    Scalar(Scalar),
    List(Box<TypeRef>),
    /// An object with arbitrary keys and typed values
    Map(Box<TypeRef>),
    /// Untyped JSON: the escape hatch for shapes decided at runtime
    /// (documents, generic type parameters, untyped `additionalProperties`),
    /// with the schema construct that caused it
    Raw(RawCause),
}

/// Why the generator fell back to raw JSON: declared with each raw field
/// (`openapi::raw_fields`) so each degradation is reviewed by its cause
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RawCause {
    /// The schema declares no type at all (`{}` or description only)
    NoType,
    /// A generic type parameter (`x-is-generic-type-parameter`, e.g.
    /// `TDocument`): the caller decides the type
    GenericParameter,
    /// An object without fixed fields whose values are untyped
    /// (`type: object` alone, or `additionalProperties: true`)
    OpenObject,
    /// An array without an `items` schema
    UntypedItems,
    /// The schema only allows `null`
    NullOnly,
}

impl RawCause {
    /// A short label of the cause, shown in generator messages
    pub fn label(self) -> &'static str {
        match self {
            RawCause::NoType => "no-type",
            RawCause::GenericParameter => "generic",
            RawCause::OpenObject => "open-object",
            RawCause::UntypedItems => "untyped-items",
            RawCause::NullOnly => "null-only",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scalar {
    Bool,
    I32,
    I64,
    U64,
    /// JSON numbers are 64-bit: `float` schemas also map here to keep
    /// round-trips lossless
    F64,
    String,
}

/// The JSON value kind a schema accepts, used for union dispatch
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JsonKind {
    Object,
    Array,
    String,
    Number,
    Bool,
}

/// Reads the response model for the given operation groups: bodies and
/// types from the client model, groups and status codes from the OpenAPI
/// document
pub use super::model_reader::read_response_model;

/// Non-2xx statuses the server answers with the success body although the
/// specification does not declare them, as `(operation group, status)`.
/// Each is a gap of the specification to be fixed upstream; listed here so
/// that a typed call returns the response type rather than an error.
const UNDECLARED_TYPED_STATUSES: &[(&str, u16)] = &[
    // `wait_for_*` timing out: 408 with the regular health body and
    // `"timed_out": true`
    ("cluster.health", 408),
];

/// The non-2xx status codes of a group whose declared JSON body is the
/// success schema `success` (after following plain `$ref`s on both sides),
/// plus its [UNDECLARED_TYPED_STATUSES]
pub(crate) fn typed_statuses(
    schemas: &Map<String, Value>,
    responses: &Map<String, Value>,
    group: &str,
    success: &Value,
) -> Vec<u16> {
    fn resolve<'a>(schemas: &'a Map<String, Value>, mut schema: &'a Value) -> &'a Value {
        // a cycle of bare references is not a schema; bound the walk
        for _ in 0..16 {
            let target = match schema.as_object() {
                Some(o) if o.len() == 1 => o.get("$ref").and_then(Value::as_str),
                _ => None,
            };
            match target
                .and_then(|r| r.strip_prefix("#/components/schemas/"))
                .and_then(|name| schemas.get(name))
            {
                Some(next) => schema = next,
                None => break,
            }
        }
        schema
    }

    let success = resolve(schemas, success);
    let prefix = format!("{}___", group);
    let mut statuses: Vec<u16> = responses
        .iter()
        .filter_map(|(key, response)| {
            let code: u16 = key.strip_prefix(&prefix)?.parse().ok()?;
            if (200..300).contains(&code) {
                return None;
            }
            let schema = &response["content"]["application/json"]["schema"];
            (!schema.is_null() && resolve(schemas, schema) == success).then_some(code)
        })
        .collect();
    statuses.extend(
        UNDECLARED_TYPED_STATUSES
            .iter()
            .filter(|(g, _)| *g == group)
            .map(|(_, code)| *code),
    );
    statuses.sort_unstable();
    statuses.dedup();
    statuses
}

/// Drops types not reachable from any endpoint root: merging specialized
/// property declarations with their generic bases can leave intermediate
/// synthesized types without references
pub(crate) fn prune_unreachable(model: &mut ResponseModel) {
    fn mark(ty: &TypeRef, model: &ResponseModel, reachable: &mut BTreeSet<String>) {
        match ty {
            TypeRef::Named(n) => {
                if reachable.insert(n.clone()) {
                    if let Some(t) = model.types.get(n) {
                        mark_type(t, model, reachable);
                    }
                }
            }
            TypeRef::List(t) | TypeRef::Map(t) => mark(t, model, reachable),
            _ => {}
        }
    }
    fn mark_type(ty: &TypeModel, model: &ResponseModel, reachable: &mut BTreeSet<String>) {
        match ty {
            TypeModel::Struct(s) => {
                for f in &s.fields {
                    mark(&f.ty, model, reachable);
                }
                if let Some((_, t)) = &s.residual {
                    mark(t, model, reachable);
                }
            }
            TypeModel::DispatchUnion(u) | TypeModel::LazyUnion(u) => {
                for v in &u.variants {
                    mark(&v.ty, model, reachable);
                }
            }
            TypeModel::Alias(a) => mark(&a.ty, model, reachable),
            TypeModel::StrEnum(_) => {}
        }
    }

    let mut reachable = BTreeSet::new();
    for root in model.endpoints.values() {
        mark(&TypeRef::Named(root.clone()), model, &mut reachable);
    }
    model.types.retain(|name, _| reachable.contains(name));
}

/// The namespace used for a synthesized root response type of an operation
/// group, e.g. `get` -> `_core.get`
pub(crate) fn root_namespace(group: &str) -> String {
    match group.split_once('.') {
        Some((ns, _)) => ns.to_string(),
        None => format!("_core.{}", group),
    }
}

/// The type-name part of a type key (`namespace___TypeName`)
pub fn type_name(type_key: &str) -> &str {
    match type_key.rsplit_once("___") {
        Some((_, n)) => n,
        None => type_key,
    }
}

/// The namespace part of a type key (`namespace___TypeName`)
pub fn namespace(type_key: &str) -> &str {
    match type_key.rsplit_once("___") {
        Some((ns, _)) => ns,
        None => "",
    }
}

/// Decides the decoding strategy of every union in the model: unions whose
/// variants have pairwise distinct JSON kinds become dispatch unions,
/// everything else stays lazy
pub(crate) fn resolve_union_strategies(model: &mut ResponseModel) {
    // compute variant kinds against the completed closure
    let kinds: BTreeMap<String, Option<JsonKind>> = model
        .types
        .keys()
        .map(|name| (name.clone(), kind_of_named(name, model, 0)))
        .collect();

    let mut upgrades: Vec<String> = Vec::new();
    for (name, ty) in &model.types {
        if let TypeModel::LazyUnion(union) = ty {
            let variant_kinds: Vec<Option<JsonKind>> = union
                .variants
                .iter()
                .map(|v| kind_of_ref(&v.ty, &kinds))
                .collect();
            let all_known = variant_kinds.iter().all(|k| k.is_some());
            let distinct = {
                let mut seen = Vec::new();
                variant_kinds.iter().flatten().all(|k| {
                    if seen.contains(k) {
                        false
                    } else {
                        seen.push(*k);
                        true
                    }
                })
            };
            if all_known && distinct {
                upgrades.push(name.clone());
            }
        }
    }

    for name in upgrades {
        let kinds_snapshot = kinds.clone();
        if let Some(TypeModel::LazyUnion(mut union)) = model.types.remove(&name) {
            for variant in &mut union.variants {
                variant.kind = kind_of_ref(&variant.ty, &kinds_snapshot);
            }
            model.types.insert(name, TypeModel::DispatchUnion(union));
        }
    }
}

/// The JSON kind of a named type, resolved through aliases; `None` when
/// the type accepts multiple kinds (unions) or any kind (Raw)
fn kind_of_named(name: &str, model: &ResponseModel, depth: usize) -> Option<JsonKind> {
    if depth > 8 {
        return None;
    }
    match model.types.get(name)? {
        TypeModel::Struct(_) => Some(JsonKind::Object),
        TypeModel::StrEnum(_) => Some(JsonKind::String),
        TypeModel::DispatchUnion(_) | TypeModel::LazyUnion(_) => None,
        TypeModel::Alias(alias) => match &alias.ty {
            TypeRef::Named(n) => kind_of_named(n, model, depth + 1),
            other => kind_of_shallow(other),
        },
    }
}

/// The JSON kind of a non-named [TypeRef]
fn kind_of_shallow(ty: &TypeRef) -> Option<JsonKind> {
    match ty {
        TypeRef::Scalar(Scalar::Bool) => Some(JsonKind::Bool),
        TypeRef::Scalar(Scalar::String) => Some(JsonKind::String),
        TypeRef::Scalar(_) => Some(JsonKind::Number),
        TypeRef::List(_) => Some(JsonKind::Array),
        TypeRef::Map(_) => Some(JsonKind::Object),
        TypeRef::Raw(_) => None,
        TypeRef::Named(_) => None,
    }
}

/// The JSON kind of an [TypeRef] against precomputed named-type kinds
fn kind_of_ref(ty: &TypeRef, kinds: &BTreeMap<String, Option<JsonKind>>) -> Option<JsonKind> {
    match ty {
        TypeRef::Named(n) => kinds.get(n).copied().flatten(),
        other => kind_of_shallow(other),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> Value {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let file = std::fs::File::open(path).unwrap();
        serde_yaml::from_reader(file).unwrap()
    }

    /// The client model vendored with the specification
    fn model() -> ClientModel {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-client-model.json");
        ClientModel::read(std::path::Path::new(path)).unwrap()
    }

    /// Reads the model of the four core APIs from the vendored specification
    /// (a subset, to keep the assertions below focused)
    fn core_model() -> ResponseModel {
        read_response_model(&spec(), &model(), &["search", "index", "bulk", "get"]).unwrap()
    }

    #[test]
    fn typed_statuses_are_the_error_codes_answering_with_the_response_type() {
        let groups = ["get", "delete", "indices.delete", "search", "clear_scroll"];
        let model = read_response_model(&spec(), &model(), &groups).unwrap();
        // a missing document is answered with the regular body
        assert_eq!(model.typed_statuses.get("get"), Some(&vec![404]));
        assert_eq!(model.typed_statuses.get("delete"), Some(&vec![404]));
        assert_eq!(model.typed_statuses.get("clear_scroll"), Some(&vec![404]));
        // a missing index is an error body (`ErrorResponseBase`)
        assert_eq!(model.typed_statuses.get("indices.delete"), None);
        assert_eq!(model.typed_statuses.get("search"), None);
    }

    #[test]
    fn undeclared_typed_statuses_are_added() {
        let model = read_response_model(&spec(), &model(), &["cluster.health"]).unwrap();
        assert_eq!(model.typed_statuses.get("cluster.health"), Some(&vec![408]));
    }

    #[test]
    fn typed_response_path_follows_the_model_endpoints() {
        let model = core_model();
        assert_eq!(
            typed_response_path(&model, "search").as_deref(),
            Some("types::search::SearchResponse")
        );
        assert_eq!(
            typed_response_path(&model, "index").as_deref(),
            Some("types::index::IndexResponse")
        );
        assert_eq!(
            typed_response_path(&model, "bulk").as_deref(),
            Some("types::bulk::BulkResponse")
        );
        assert_eq!(
            typed_response_path(&model, "get").as_deref(),
            Some("types::get::GetResponse")
        );
        // groups not in this model have no path
        assert_eq!(typed_response_path(&model, "indices.create"), None);
    }

    #[test]
    fn every_operation_group_is_typed_or_explained() {
        // the whole specification reads without error: each group either
        // gets a root type or is recorded as untyped with a reason
        let spec = spec();
        let groups = all_operation_groups(&spec);
        assert!(
            groups.len() > 400,
            "unexpectedly few groups: {}",
            groups.len()
        );
        let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
        let model = read_response_model(&spec, &model(), &refs).unwrap();
        for g in &groups {
            assert!(
                model.endpoints.contains_key(g) || model.untyped_groups.contains_key(g),
                "{} neither typed nor explained",
                g
            );
        }
        // `create` only declares a 201: it is typed through it
        assert!(model.endpoints.contains_key("create"));
        // HEAD endpoints have no body: explained, not failed
        assert!(model.untyped_groups.contains_key("exists"));
        assert!(model.untyped_groups["exists"].contains("HEAD-only"));
        // typed groups vastly outnumber untyped ones
        assert!(
            model.endpoints.len() > 400,
            "{} typed",
            model.endpoints.len()
        );
        assert!(
            model.untyped_groups.len() < 40,
            "{} untyped",
            model.untyped_groups.len()
        );
    }

    #[test]
    fn endpoint_alias_rule_handles_namespaced_groups() {
        assert_eq!(
            endpoint_response_alias("indices.create"),
            ("indices_create".to_string(), "CreateResponse".to_string())
        );
        assert_eq!(
            endpoint_response_alias("search"),
            ("search".to_string(), "SearchResponse".to_string())
        );
    }

    #[test]
    fn maps_endpoints_to_root_types() {
        let model = core_model();
        assert_eq!(model.endpoints["search"], "_core.search___SearchResponse");
        assert_eq!(model.endpoints["index"], "_common___WriteResponseBase");
        assert_eq!(model.endpoints["bulk"], "_core.bulk___BulkResponse");
        // get's 200 schema is inline: a root type is synthesized
        assert_eq!(model.endpoints["get"], "_core.get___GetResponse");
        assert!(model.types.contains_key("_core.get___GetResponse"));
    }

    #[test]
    fn expands_all_of_physically() {
        let model = core_model();
        // StringTermsBucket = TermsBucketBase(+MultiBucketBase) + {key}
        let bucket = match &model.types["_common.aggregations___StringTermsBucket"] {
            TypeModel::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let names: Vec<&str> = bucket.fields.iter().map(|f| f.json_name.as_str()).collect();
        assert!(names.contains(&"key"), "own field: {:?}", names);
        assert!(
            names.contains(&"doc_count"),
            "from MultiBucketBase: {:?}",
            names
        );
        assert!(
            names.contains(&"doc_count_error_upper_bound"),
            "from TermsBucketBase: {:?}",
            names
        );
        // doc_count is required through the base
        let doc_count = bucket
            .fields
            .iter()
            .find(|f| f.json_name == "doc_count")
            .unwrap();
        assert!(doc_count.required);
        // sub-aggregations: residual keys from MultiBucketBase
        let (residual_name, residual_ty) = bucket.residual.as_ref().unwrap();
        assert_eq!(residual_name, "aggregations");
        assert_eq!(
            residual_ty,
            &TypeRef::Named("_common.aggregations___Aggregate".to_string())
        );
    }

    #[test]
    fn aggregate_union_stays_lazy() {
        let model = core_model();
        let agg = match &model.types["_common.aggregations___Aggregate"] {
            TypeModel::LazyUnion(u) => u,
            other => panic!("expected lazy union, got {:?}", other),
        };
        assert!(agg.variants.len() > 50);
        assert!(agg.variants.iter().any(|v| v.name == "avg"));
        assert!(agg.variants.iter().any(|v| v.name == "sterms"));
    }

    #[test]
    fn kind_disjoint_unions_become_dispatch() {
        let model = core_model();
        // buckets are specialized from the generic Buckets union per
        // aggregate: keyed map | array
        let buckets = match &model.types["_common.aggregations___DateHistogramAggregateBuckets"] {
            TypeModel::DispatchUnion(u) => u,
            other => panic!("expected dispatch union, got {:?}", other),
        };
        assert_eq!(buckets.variants.len(), 2);
        assert_eq!(buckets.variants[0].kind, Some(JsonKind::Object));
        assert_eq!(buckets.variants[1].kind, Some(JsonKind::Array));

        // FieldValue: bool | number | string (+ null)
        let field_value = match &model.types["_common___FieldValue"] {
            TypeModel::DispatchUnion(u) => u,
            other => panic!("expected dispatch union, got {:?}", other),
        };
        assert!(field_value.nullable);
        assert_eq!(field_value.variants.len(), 3);
    }

    #[test]
    fn overlapping_object_unions_stay_lazy() {
        let model = core_model();
        // GeoLocation: 2 object variants overlap
        assert!(matches!(
            &model.types["_common___GeoLocation"],
            TypeModel::LazyUnion(_)
        ));
        // Suggest: 3 object variants
        assert!(matches!(
            &model.types["_core.search___Suggest"],
            TypeModel::LazyUnion(_)
        ));
    }

    #[test]
    fn case_variants_become_aliases_unless_titled_apart() {
        // the two enums as fields of a fixture root, with their vendored entries
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-client-model.json");
        let vendored: Value =
            serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
        let mut types = serde_json::json!({ "t___Root": { "type_kind": "object", "fields": {}, "constraints": [] } });
        for name in [
            "indices._common___TranslogDurability",
            "_common.aggregations___CalendarInterval",
        ] {
            types[name] = vendored["types"][name].clone();
            types["t___Root"]["fields"][name] =
                serde_json::json!({ "type": { "ref": name }, "required": false });
        }
        let all = fixture_model(types);
        let values = |name: &str| match &all.types[name] {
            TypeModel::StrEnum(e) => e.values.clone(),
            other => panic!("expected string enum, got {:?}", other),
        };

        // ASYNC / async: one value, the lowercase spelling as an alias
        let durability = values("indices._common___TranslogDurability");
        assert_eq!(
            durability,
            vec![
                StrEnumValue {
                    value: "ASYNC".into(),
                    aliases: vec!["async".into()]
                },
                StrEnumValue {
                    value: "REQUEST".into(),
                    aliases: vec!["request".into()]
                },
            ]
        );

        // 1m (titled minute) and 1M (titled month) stay distinct values
        let interval = values("_common.aggregations___CalendarInterval");
        let one_m = interval.iter().find(|v| v.value == "1m").unwrap();
        let one_big_m = interval.iter().find(|v| v.value == "1M").unwrap();
        assert!(one_m.aliases.is_empty() && one_big_m.aliases.is_empty());
    }

    #[test]
    fn const_unions_become_open_string_enums() {
        let model = core_model();
        let relation = match &model.types["_core.search___TotalHitsRelation"] {
            TypeModel::StrEnum(e) => e,
            other => panic!("expected string enum, got {:?}", other),
        };
        let values: Vec<&str> = relation.values.iter().map(|v| v.value.as_str()).collect();
        assert_eq!(values, vec!["eq", "gte"]);
    }

    #[test]
    fn generic_type_parameters_are_raw() {
        let model = core_model();
        // the synthesized hit type merges Hit with the inline {_source: TDocument}
        let hits_metadata = match &model.types["_core.search___HitsMetadata"] {
            TypeModel::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let hits = hits_metadata
            .fields
            .iter()
            .find(|f| f.json_name == "hits")
            .unwrap();
        let item = match &hits.ty {
            TypeRef::List(item) => item.as_ref(),
            other => panic!("expected list, got {:?}", other),
        };
        let hit_name = match item {
            TypeRef::Named(n) => n,
            other => panic!("expected named hit type, got {:?}", other),
        };
        let hit = match &model.types[hit_name] {
            TypeModel::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        let source = hit
            .fields
            .iter()
            .find(|f| f.json_name == "_source")
            .unwrap();
        assert_eq!(source.ty, TypeRef::Raw(RawCause::GenericParameter));
        // fields from Hit itself were merged in
        assert!(hit.fields.iter().any(|f| f.json_name == "_index"));
    }

    /// The response model of a single `t` operation group whose 200 response
    /// is the `t___Root` type, with the given client model types
    fn fixture_model(types: Value) -> ResponseModel {
        let spec = serde_json::json!({
            "components": {
                "schemas": {},
                "responses": { "t___200": {} }
            }
        });
        let model = serde_json::json!({
            "format": "opensearch-client-model",
            "version": "0.2.0",
            "types": types,
            "responses": {
                "t___200": { "media_type": "application/json", "type": { "ref": "t___Root" } }
            },
            "request_bodies": {}
        });
        let model = ClientModel::parse(&model.to_string()).unwrap();
        read_response_model(&spec, &model, &["t"]).unwrap()
    }

    fn field<'a>(model: &'a ResponseModel, ty: &str, name: &str) -> &'a FieldModel {
        match &model.types[ty] {
            TypeModel::Struct(s) => s.fields.iter().find(|f| f.json_name == name).unwrap(),
            other => panic!("expected struct, got {:?}", other),
        }
    }

    #[test]
    fn head_only_groups_and_generic_roots() {
        let spec = spec();
        // ism.exists_policy declares a JSON 200 body, but is HEAD-only
        let model =
            read_response_model(&spec, &model(), &["ism.exists_policy", "get_source"]).unwrap();
        assert!(model.untyped_groups["ism.exists_policy"].contains("HEAD-only"));
        // get_source's whole response is the generic TDocument
        let root = &model.endpoints["get_source"];
        match &model.types[root] {
            TypeModel::Alias(a) => assert_eq!(a.ty, TypeRef::Raw(RawCause::GenericParameter)),
            other => panic!("expected a raw alias, got {:?}", other),
        }
    }

    #[test]
    fn integers_without_format_are_64_bit() {
        let model = fixture_model(serde_json::json!({
            "t___Root": {
                "type_kind": "object",
                "fields": {
                    "no_format": { "type": { "scalar": "integer" }, "required": false },
                    "int32": { "type": { "scalar": "integer", "format": "int32" }, "required": false },
                    "int64": { "type": { "scalar": "integer", "format": "int64" }, "required": false },
                    "uint64": { "type": { "scalar": "integer", "format": "uint64" }, "required": false }
                },
                "constraints": []
            }
        }));
        let ty = |name: &str| field(&model, "t___Root", name).ty.clone();
        assert_eq!(ty("no_format"), TypeRef::Scalar(Scalar::I64));
        assert_eq!(ty("int32"), TypeRef::Scalar(Scalar::I32));
        assert_eq!(ty("int64"), TypeRef::Scalar(Scalar::I64));
        assert_eq!(ty("uint64"), TypeRef::Scalar(Scalar::U64));
    }

    #[test]
    fn type_arrays_become_typed_fields() {
        // the client model writes `type: [null, X]` as a union with `null`
        let union = |members: Value| {
            serde_json::json!({
                "type": { "union": members, "union_kind": "scalar", "exclusive": true }, "required": false
            })
        };
        let model = fixture_model(serde_json::json!({
            "t___Root": {
                "type_kind": "object",
                "fields": {
                    "nullable_string": union(serde_json::json!([{ "scalar": "null" }, { "scalar": "string" }])),
                    "nullable_number": union(serde_json::json!([{ "scalar": "null" }, { "scalar": "number" }])),
                    "nullable_bool": union(serde_json::json!([{ "scalar": "boolean" }, { "scalar": "null" }])),
                    "nullable_list": union(serde_json::json!([{ "array": { "scalar": "string" } }, { "scalar": "null" }])),
                    "nullable_object": union(serde_json::json!([{ "scalar": "null" }, { "object": {
                        "fields": { "a": { "type": { "scalar": "integer" }, "required": false } },
                        "constraints": []
                    } }])),
                    "number_or_string": union(serde_json::json!([{ "scalar": "number" }, { "scalar": "string" }]))
                },
                "constraints": []
            }
        }));
        let root = "t___Root";
        let f = field(&model, root, "nullable_string");
        assert_eq!(
            (f.ty.clone(), f.nullable),
            (TypeRef::Scalar(Scalar::String), true)
        );
        let f = field(&model, root, "nullable_number");
        assert_eq!(
            (f.ty.clone(), f.nullable),
            (TypeRef::Scalar(Scalar::F64), true)
        );
        let f = field(&model, root, "nullable_bool");
        assert_eq!(
            (f.ty.clone(), f.nullable),
            (TypeRef::Scalar(Scalar::Bool), true)
        );
        let f = field(&model, root, "nullable_list");
        assert_eq!(
            (f.ty.clone(), f.nullable),
            (
                TypeRef::List(Box::new(TypeRef::Scalar(Scalar::String))),
                true
            )
        );
        // the object variant keeps its properties
        let f = field(&model, root, "nullable_object");
        assert!(f.nullable);
        match &f.ty {
            TypeRef::Named(n) => assert!(matches!(&model.types[n], TypeModel::Struct(_))),
            other => panic!("expected a struct, got {:?}", other),
        }
        // several non-null types: a union dispatching on the JSON kind
        let f = field(&model, root, "number_or_string");
        match &f.ty {
            TypeRef::Named(n) => match &model.types[n] {
                TypeModel::DispatchUnion(u) => {
                    let names: Vec<&str> = u.variants.iter().map(|v| v.name.as_str()).collect();
                    assert_eq!(names, vec!["number", "string"]);
                }
                other => panic!("expected a dispatch union, got {:?}", other),
            },
            other => panic!("expected a named union, got {:?}", other),
        }
    }

    #[test]
    fn no_type_array_falls_back_to_raw_in_the_vendored_spec() {
        // regression guard: every type array of the specification is typed
        let spec = spec();
        let groups = all_operation_groups(&spec);
        let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
        let model = read_response_model(&spec, &model(), &refs).unwrap();
        let schemas = spec["components"]["schemas"].as_object().unwrap();
        for (name, ty) in &model.types {
            let s = match ty {
                TypeModel::Struct(s) => s,
                _ => continue,
            };
            let props = match schemas.get(name) {
                Some(schema) => &schema["properties"],
                None => continue,
            };
            for f in &s.fields {
                if props[&f.json_name]["type"].is_array() {
                    assert!(
                        !matches!(f.ty, TypeRef::Raw(_)),
                        "{}.{} is a type array left raw",
                        name,
                        f.json_name
                    );
                }
            }
        }
    }

    #[test]
    fn nullable_one_of_collapses_to_option() {
        let model = core_model();
        let hits_metadata = match &model.types["_core.search___HitsMetadata"] {
            TypeModel::Struct(s) => s,
            other => panic!("expected struct, got {:?}", other),
        };
        // max_score: oneOf [null, float]
        let max_score = hits_metadata
            .fields
            .iter()
            .find(|f| f.json_name == "max_score")
            .unwrap();
        assert!(max_score.nullable);
        assert_eq!(max_score.ty, TypeRef::Scalar(Scalar::F64));
        // total: oneOf [TotalHits, i64] synthesized as a union
        let total = hits_metadata
            .fields
            .iter()
            .find(|f| f.json_name == "total")
            .unwrap();
        match &total.ty {
            TypeRef::Named(n) => {
                assert!(
                    matches!(&model.types[n], TypeModel::DispatchUnion(_)),
                    "total should dispatch on object vs number"
                );
            }
            other => panic!("expected named union, got {:?}", other),
        }
    }

    #[test]
    fn closure_is_complete() {
        let model = core_model();
        // every named reference resolves within the model
        fn check(ty: &TypeRef, model: &ResponseModel) {
            match ty {
                TypeRef::Named(n) => {
                    assert!(model.types.contains_key(n), "dangling reference: {}", n);
                }
                TypeRef::List(t) | TypeRef::Map(t) => check(t, model),
                _ => {}
            }
        }
        for ty in model.types.values() {
            match ty {
                TypeModel::Struct(s) => {
                    for f in &s.fields {
                        check(&f.ty, &model);
                    }
                    if let Some((_, t)) = &s.residual {
                        check(t, &model);
                    }
                }
                TypeModel::DispatchUnion(u) | TypeModel::LazyUnion(u) => {
                    for v in &u.variants {
                        check(&v.ty, &model);
                    }
                }
                TypeModel::Alias(a) => check(&a.ty, &model),
                TypeModel::StrEnum(_) => {}
            }
        }
    }
}
