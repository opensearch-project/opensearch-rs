/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Reads an [Api] from the OpenAPI specification published by
//! [opensearch-project/opensearch-api-specification](https://github.com/opensearch-project/opensearch-api-specification).
//!
//! The OpenAPI document is mapped onto the same internal model that the
//! legacy REST API spec reader produces, so that the code generation
//! backend can remain unchanged:
//!
//! - `x-operation-group` becomes the endpoint name (namespace + method)
//! - path parameters (`in: path`) become URL parts
//! - query parameters (`in: query`) become endpoint params, with
//!   `x-global` parameters collected as common params
//! - `externalDocs`/`description` become the endpoint documentation
//! - `deprecated` + `x-version-deprecated`/`x-deprecation-message`
//!   become [Deprecated] metadata
//!
//! Rather than deserializing into a parallel set of structs, each
//! operation group is converted into the legacy JSON format and fed
//! through the existing serde model, guaranteeing that both ingestion
//! paths produce identical structures.

use crate::generator::{build_api, endpoint_from_file, Api, Type};
use anyhow::{anyhow, bail};
use log::{info, warn};
use serde_json::{json, Map, Value};
use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    path::Path,
};

/// The release artifact of the OpenSearch API specification
pub const SPEC_URL: &str = "https://api-spec.opensearch.org/opensearch-openapi.yaml";

/// HTTP methods that may appear as keys of an OpenAPI path item
const METHODS: &[&str] = &["get", "put", "post", "delete", "head", "patch"];

/// Downloads the OpenAPI specification to the given file path
pub fn download_spec(to_file: &Path) -> anyhow::Result<()> {
    info!("downloading {}", SPEC_URL);
    let response = reqwest::blocking::get(SPEC_URL)?.error_for_status()?;
    let mut file = File::create(to_file)?;
    file.write_all(&response.bytes()?)?;
    info!("downloaded spec to {}", to_file.display());
    Ok(())
}

/// Reads an [Api] from an OpenAPI specification file
pub fn read_api(commit: &str, openapi_file: &Path) -> anyhow::Result<Api> {
    let file = File::open(openapi_file)?;
    let spec: Value = serde_yaml::from_reader(file)?;

    let components = &spec["components"];
    let parameters = components["parameters"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    let request_bodies = components["requestBodies"]
        .as_object()
        .cloned()
        .unwrap_or_default();
    let schemas = components["schemas"]
        .as_object()
        .cloned()
        .unwrap_or_default();

    let resolver = Resolver {
        parameters,
        request_bodies,
        schemas,
    };

    let paths = spec["paths"]
        .as_object()
        .ok_or_else(|| anyhow!("OpenAPI document has no paths object"))?;

    // Collect operations grouped by x-operation-group
    let mut groups: BTreeMap<String, Vec<Operation>> = BTreeMap::new();
    for (path, path_item) in paths {
        let path_item = match path_item.as_object() {
            Some(o) => o,
            None => continue,
        };
        for method in METHODS {
            if let Some(op) = path_item.get(*method) {
                let group = match op["x-operation-group"].as_str() {
                    Some(g) => g.to_string(),
                    None => {
                        warn!(
                            "skipping operation without x-operation-group: {} {}",
                            method, path
                        );
                        continue;
                    }
                };
                // Skip placeholder operations documenting unsupported
                // methods (e.g. security.cache), recognizable by the
                // absence of any 2xx response
                if !has_success_response(op) {
                    warn!(
                        "skipping operation without success response: {} {}",
                        method, path
                    );
                    continue;
                }
                groups.entry(group).or_default().push(Operation {
                    path: path.clone(),
                    method: method.to_uppercase(),
                    op: op.clone(),
                });
            }
        }
    }

    let mut endpoints = Vec::with_capacity(groups.len());
    for (name, operations) in groups {
        let legacy = to_legacy_endpoint(&name, &operations, &resolver)?;
        let json = serde_json::to_string(&json!({ &name: legacy }))?;
        let (name, endpoint) =
            endpoint_from_file(format!("openapi:{}", name), &mut json.as_bytes())?;
        endpoints.push((name, endpoint));
    }

    let common_params = common_params(&resolver)?;

    build_api(commit, endpoints, common_params)
}

/// A single OpenAPI operation (path + method + operation object)
struct Operation {
    path: String,
    method: String,
    op: Value,
}

/// Resolves local `$ref` pointers against the components section
struct Resolver {
    parameters: Map<String, Value>,
    request_bodies: Map<String, Value>,
    schemas: Map<String, Value>,
}

impl Resolver {
    /// Resolves a parameter that may be a `$ref` into the parameter object
    fn parameter<'a>(&'a self, param: &'a Value) -> anyhow::Result<&'a Value> {
        match param["$ref"].as_str() {
            Some(r) => {
                let name = r
                    .strip_prefix("#/components/parameters/")
                    .ok_or_else(|| anyhow!("unsupported parameter $ref: {}", r))?;
                self.parameters
                    .get(name)
                    .ok_or_else(|| anyhow!("unresolved parameter $ref: {}", r))
            }
            None => Ok(param),
        }
    }

    /// Resolves a request body that may be a `$ref` into the request body object
    fn request_body<'a>(&'a self, body: &'a Value) -> anyhow::Result<&'a Value> {
        match body["$ref"].as_str() {
            Some(r) => {
                let name = r
                    .strip_prefix("#/components/requestBodies/")
                    .ok_or_else(|| anyhow!("unsupported requestBody $ref: {}", r))?;
                self.request_bodies
                    .get(name)
                    .ok_or_else(|| anyhow!("unresolved requestBody $ref: {}", r))
            }
            None => Ok(body),
        }
    }

    /// Resolves a schema `$ref` chain, returning the schema and the name of
    /// the last `$ref` followed (used for heuristics such as Duration -> time).
    ///
    /// Returns an error if a `$ref` does not point into
    /// `#/components/schemas/`, if a referenced schema does not exist, or
    /// if the chain does not resolve within the depth limit (a cycle or a
    /// pathologically deep chain)
    fn schema<'a>(&'a self, schema: &'a Value) -> anyhow::Result<(&'a Value, Option<&'a str>)> {
        let mut current = schema;
        let mut last_name = None;
        // depth-limit to guard against reference cycles
        for _ in 0..8 {
            let r = match current["$ref"].as_str() {
                Some(r) => r,
                None => return Ok((current, last_name)),
            };
            let name = r
                .strip_prefix("#/components/schemas/")
                .ok_or_else(|| anyhow!("unsupported schema $ref: {}", r))?;
            let schema = self
                .schemas
                .get(name)
                .ok_or_else(|| anyhow!("unresolved schema $ref: {}", r))?;
            last_name = Some(name);
            current = schema;
        }
        match current["$ref"].as_str() {
            Some(r) => bail!("schema $ref chain too deep or cyclic at: {}", r),
            None => Ok((current, last_name)),
        }
    }
}

/// Converts all operations of an operation group into a legacy-format
/// endpoint JSON object
fn to_legacy_endpoint(
    name: &str,
    operations: &[Operation],
    resolver: &Resolver,
) -> anyhow::Result<Value> {
    if operations.is_empty() {
        bail!("operation group {} has no operations", name);
    }

    // Documentation: prefer a non-deprecated operation
    let doc_op = operations
        .iter()
        .find(|o| !is_deprecated(&o.op))
        .unwrap_or(&operations[0]);
    let documentation = json!({
        "url": doc_op.op["externalDocs"]["url"].as_str(),
        "description": doc_op.op["description"].as_str(),
    });

    // URL paths: merge methods of operations sharing the same path
    let mut paths: BTreeMap<String, PathAccumulator> = BTreeMap::new();
    // Query params and body: union across all operations of the group
    let mut params = Map::new();
    let mut body: Option<Value> = None;

    for operation in operations {
        let acc = paths
            .entry(operation.path.clone())
            .or_insert_with(|| PathAccumulator {
                methods: Vec::new(),
                parts: Map::new(),
                deprecated: deprecation(&operation.op),
            });
        if !acc.methods.contains(&operation.method) {
            acc.methods.push(operation.method.clone());
        }
        // a path is only deprecated if all its operations are deprecated
        if !is_deprecated(&operation.op) {
            acc.deprecated = None;
        }

        if let Some(op_params) = operation.op["parameters"].as_array() {
            for param in op_params {
                let param = resolver.parameter(param)?;
                if param["x-global"].as_bool().unwrap_or(false) {
                    continue;
                }
                let name = match param["name"].as_str() {
                    Some(n) => n,
                    None => continue,
                };
                let legacy_type = to_legacy_type(param, resolver)?;
                match param["in"].as_str() {
                    Some("path") => {
                        // legacy REST specs never type URL parts as enums,
                        // and the code generator does not collect part enums;
                        // treat them as plain strings
                        let mut legacy_type = legacy_type;
                        if legacy_type["type"] == "enum" {
                            legacy_type["type"] = Value::String("string".to_string());
                            if let Some(obj) = legacy_type.as_object_mut() {
                                obj.remove("options");
                            }
                        }
                        acc.parts.insert(name.to_string(), legacy_type);
                    }
                    Some("query") => {
                        params.entry(name.to_string()).or_insert(legacy_type);
                    }
                    _ => {}
                }
            }
        }

        if body.is_none() {
            if let Some(request_body) = operation.op.get("requestBody") {
                let request_body = resolver.request_body(request_body)?;
                body = Some(to_legacy_body(request_body));
            }
        }
    }

    // The endpoint is deprecated only if every operation is deprecated;
    // take the deprecation details from the most recently added operation
    let deprecated = if operations.iter().all(|o| is_deprecated(&o.op)) {
        deprecation(&operations[0].op)
    } else {
        None
    };

    let paths: Vec<Value> = paths
        .into_iter()
        .map(|(path, acc)| {
            let mut obj = json!({
                "path": path,
                "methods": acc.methods,
                "parts": acc.parts,
            });
            if let Some(d) = acc.deprecated {
                obj["deprecated"] = d;
            }
            obj
        })
        .collect();

    let mut endpoint = json!({
        "documentation": documentation,
        // the OpenAPI specification does not model stability levels yet;
        // all endpoints are considered stable
        "stability": "stable",
        "url": { "paths": paths },
        "params": params,
    });
    if let Some(b) = body {
        endpoint["body"] = b;
    }
    if let Some(d) = deprecated {
        endpoint["deprecated"] = d;
    }

    Ok(endpoint)
}

/// Accumulates methods, parts and deprecation for a single URL path
struct PathAccumulator {
    methods: Vec<String>,
    parts: Map<String, Value>,
    deprecated: Option<Value>,
}

/// Whether an operation is marked deprecated
fn is_deprecated(op: &Value) -> bool {
    op["deprecated"].as_bool().unwrap_or(false)
}

/// Extracts legacy deprecation metadata from an operation
fn deprecation(op: &Value) -> Option<Value> {
    if !is_deprecated(op) {
        return None;
    }
    Some(json!({
        "version": op["x-version-deprecated"].as_str().unwrap_or(""),
        "description": op["x-deprecation-message"].as_str().unwrap_or("Deprecated"),
    }))
}

/// Converts an OpenAPI request body into the legacy body object
fn to_legacy_body(request_body: &Value) -> Value {
    let nd_json = request_body["content"]
        .as_object()
        .map(|c| c.contains_key("application/x-ndjson"))
        .unwrap_or(false);

    let mut body = json!({
        "description": request_body["description"].as_str(),
        "required": request_body["required"].as_bool(),
    });
    if nd_json {
        body["serialize"] = json!("bulk");
    }
    body
}

/// Converts an OpenAPI parameter into a legacy [Type] JSON object
fn to_legacy_type(param: &Value, resolver: &Resolver) -> anyhow::Result<Value> {
    let (schema, ref_name) = resolver.schema(&param["schema"])?;
    let (kind, options) = to_legacy_type_kind(schema, ref_name, resolver)?;

    let mut ty = json!({
        "type": kind,
        "description": param["description"].as_str(),
    });
    if let Some(options) = options {
        ty["options"] = Value::Array(options);
    }
    if let Some(default) = schema.get("default") {
        ty["default"] = default.clone();
    }
    if param["deprecated"].as_bool().unwrap_or(false) {
        ty["deprecated"] = json!({
            "version": param["x-version-deprecated"].as_str().unwrap_or(""),
            "description": param["x-deprecation-message"].as_str().unwrap_or("Deprecated"),
        });
    }
    Ok(ty)
}

/// Maps a resolved OpenAPI schema to a legacy type kind string and
/// optional enum options
fn to_legacy_type_kind(
    schema: &Value,
    ref_name: Option<&str>,
    resolver: &Resolver,
) -> anyhow::Result<(&'static str, Option<Vec<Value>>)> {
    // enum-like schemas: a flat `enum`, a `const`, or a `oneOf` whose
    // variants each contribute fixed string values. This covers mixed
    // forms such as _common___ByteUnit (const and multi-value enum
    // variants side by side), _common___VersionType (const variants),
    // _common___ExpandWildcards (scalar and array-of-scalar variants for
    // the comma-separated form) and _common___Refresh (a boolean variant,
    // ignored because the "true"/"false" consts cover it)
    if let Some(values) = collect_enum_values(schema, resolver)? {
        return Ok(match valid_enum_values(values) {
            Some(values) => ("enum", Some(values)),
            // values that cannot become Rust identifiers, e.g. "1".."5"
            None => ("string", None),
        });
    }

    if let Some(one_of) = schema["oneOf"].as_array() {
        let mut variant_types = Vec::with_capacity(one_of.len());
        for variant in one_of {
            variant_types.push(resolver.schema(variant)?.0["type"].as_str());
        }
        // oneOf [scalar, array of scalar] used for comma-separated values
        // of arbitrary strings, e.g. _common___Indices
        if variant_types.contains(&Some("array")) {
            return Ok(("list", None));
        }
        // boolean alongside numeric variants (e.g. _core.search___TrackHits)
        // maps to the legacy boolean type; the code generator special-cases
        // parameters like track_total_hits by name
        let types: Vec<&str> = variant_types.iter().flatten().copied().collect();
        if types.len() == one_of.len()
            && types.contains(&"boolean")
            && types
                .iter()
                .all(|t| matches!(*t, "boolean" | "integer" | "number"))
        {
            return Ok(("boolean", None));
        }
    }

    Ok(match schema["type"].as_str() {
        Some("array") => ("list", None),
        Some("boolean") => ("boolean", None),
        Some("integer") => match schema["format"].as_str() {
            Some("int64") => ("long", None),
            _ => ("number", None),
        },
        Some("number") => match schema["format"].as_str() {
            Some("float") => ("float", None),
            Some("double") => ("double", None),
            _ => ("number", None),
        },
        Some("string") => {
            // Duration/time-like schemas map to the legacy time type
            match ref_name {
                Some(name) if name.contains("Duration") || name.contains("TimeUnit") => {
                    ("time", None)
                }
                _ => ("string", None),
            }
        }
        // union types and unspecified schemas: default to string
        _ => ("string", None),
    })
}

/// Whether an operation declares at least one 2xx (or default) response.
/// The specification documents some unsupported method/path combinations
/// as operations with only 4xx/5xx responses; these must not be generated.
fn has_success_response(op: &Value) -> bool {
    match op["responses"].as_object() {
        Some(responses) if !responses.is_empty() => responses
            .keys()
            .any(|code| code.starts_with('2') || code == "default"),
        // no responses declared at all: assume a regular operation
        _ => true,
    }
}

/// Recursively collects the fixed string values a schema can take,
/// aggregating across all `oneOf` variants. Returns `Ok(None)` if the
/// schema (or any variant that must contribute) allows values that are not
/// fixed strings, so that the caller falls back to a non-enum type.
/// Broken `$ref`s encountered while collecting are hard errors.
///
/// - `enum` lists and `const` values contribute their strings
/// - `oneOf` unions the values of all its variants
/// - array variants contribute the values of their `items` (the
///   comma-separated form of an enum, e.g. _common___ExpandWildcards)
/// - boolean variants are skipped (e.g. _common___Refresh, where the
///   "true"/"false" consts cover the boolean form)
fn collect_enum_values(schema: &Value, resolver: &Resolver) -> anyhow::Result<Option<Vec<Value>>> {
    /// `Ok(true)`: collected, `Ok(false)`: not enum-like (soft failure),
    /// `Err`: broken specification (hard error)
    fn collect(
        schema: &Value,
        resolver: &Resolver,
        depth: usize,
        out: &mut Vec<Value>,
    ) -> anyhow::Result<bool> {
        // guard against pathological nesting
        if depth > 4 {
            return Ok(false);
        }
        let (schema, _) = resolver.schema(schema)?;

        if let Some(values) = schema["enum"].as_array() {
            if values.is_empty() || !values.iter().all(|v| v.is_string()) {
                return Ok(false);
            }
            out.extend(values.iter().cloned());
            return Ok(true);
        }
        match &schema["const"] {
            c @ Value::String(_) => {
                out.push(c.clone());
                return Ok(true);
            }
            Value::Null => {}
            // non-string const, e.g. a number
            _ => return Ok(false),
        }
        if let Some(one_of) = schema["oneOf"].as_array() {
            for variant in one_of {
                let (resolved, _) = resolver.schema(variant)?;
                let collected = match resolved["type"].as_str() {
                    // covered by explicit "true"/"false" consts where relevant
                    Some("boolean") => continue,
                    // comma-separated form: collect from the item schema
                    Some("array") => collect(&resolved["items"], resolver, depth + 1, out)?,
                    _ => collect(resolved, resolver, depth + 1, out)?,
                };
                if !collected {
                    return Ok(false);
                }
            }
            return Ok(true);
        }
        Ok(false)
    }

    let mut values = Vec::new();
    Ok(match collect(schema, resolver, 0, &mut values)? {
        true if !values.is_empty() => Some(values),
        _ => None,
    })
}

/// Validates candidate enum values, dropping duplicates that differ only in
/// case (e.g. "and"/"AND" both become the `And` variant) and rejecting sets
/// whose values cannot be converted into Rust identifiers
fn valid_enum_values(values: Vec<Value>) -> Option<Vec<Value>> {
    // Values starting with a digit (e.g. severityLevel: "1".."5") cannot be
    // converted into valid Rust identifiers; treat the parameter as a string
    if values.iter().any(|v| {
        v.as_str()
            .unwrap()
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
    }) {
        return None;
    }
    let mut seen = std::collections::HashSet::new();
    let deduped: Vec<Value> = values
        .into_iter()
        .filter(|v| seen.insert(v.as_str().unwrap().to_lowercase()))
        .collect();
    if deduped.is_empty() {
        return None;
    }
    Some(deduped)
}

/// Collects `x-global` query parameters as legacy common params
fn common_params(resolver: &Resolver) -> anyhow::Result<BTreeMap<String, Type>> {
    let mut common = Map::new();
    for param in resolver.parameters.values() {
        if param["x-global"].as_bool().unwrap_or(false)
            && param["in"].as_str() == Some("query")
        {
            if let Some(name) = param["name"].as_str() {
                if !common.contains_key(name) {
                    common.insert(name.to_string(), to_legacy_type(param, resolver)?);
                }
            }
        }
    }
    let common: BTreeMap<String, Type> = serde_json::from_value(Value::Object(common))?;
    Ok(common)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::TypeKind;
    use std::io::Write;

    const FIXTURE: &str = r#"
openapi: 3.1.0
info:
  title: Test
  version: 0.0.0
paths:
  /_cat/cluster_manager:
    get:
      operationId: cat.cluster_manager.0
      x-operation-group: cat.cluster_manager
      description: Returns information about the cluster-manager node.
      externalDocs:
        url: https://opensearch.org/docs/latest/api-reference/cat/cat-cluster_manager/
      parameters:
        - $ref: '#/components/parameters/_global___query.pretty'
        - $ref: '#/components/parameters/cat.cluster_manager___query.local'
        - $ref: '#/components/parameters/cat.cluster_manager___query.bytes'
        - $ref: '#/components/parameters/cat.cluster_manager___query.health'
      responses: {}
  /_search:
    get:
      operationId: search.0
      x-operation-group: search
      description: Returns results matching a query.
      externalDocs:
        url: https://opensearch.org/docs/latest/api-reference/search/
      parameters:
        - $ref: '#/components/parameters/search___query.expand_wildcards'
      responses: {}
    post:
      operationId: search.1
      x-operation-group: search
      description: Returns results matching a query.
      parameters:
        - $ref: '#/components/parameters/search___query.expand_wildcards'
      requestBody:
        $ref: '#/components/requestBodies/search'
      responses: {}
  /{index}/_search:
    get:
      operationId: search.2
      x-operation-group: search
      description: Returns results matching a query.
      parameters:
        - $ref: '#/components/parameters/search___path.index'
        - $ref: '#/components/parameters/search___query.expand_wildcards'
      responses: {}
  /_cat/master:
    get:
      operationId: cat.master.0
      x-operation-group: cat.master
      deprecated: true
      x-version-deprecated: '2.0'
      x-deprecation-message: Use '/_cat/cluster_manager' instead.
      description: Returns information about the cluster-manager node.
      responses: {}
  /_bulk:
    post:
      operationId: bulk.0
      x-operation-group: bulk
      description: Bulk operations.
      requestBody:
        $ref: '#/components/requestBodies/bulk'
      responses: {}
components:
  parameters:
    _global___query.pretty:
      name: pretty
      in: query
      description: Whether to pretty-format the response.
      schema:
        type: boolean
        default: false
      x-global: true
    cat.cluster_manager___query.local:
      name: local
      in: query
      description: Return local information.
      schema:
        type: boolean
    cat.cluster_manager___query.bytes:
      name: bytes
      in: query
      description: The units used to display byte values.
      schema:
        $ref: '#/components/schemas/_common___ByteUnit'
    cat.cluster_manager___query.health:
      name: health
      in: query
      description: Filter by health status.
      schema:
        $ref: '#/components/schemas/_common___HealthStatus'
    search___path.index:
      name: index
      in: path
      required: true
      description: Indexes to search.
      schema:
        $ref: '#/components/schemas/_common___Indices'
    search___query.expand_wildcards:
      name: expand_wildcards
      in: query
      description: Type of index that wildcard patterns can match.
      schema:
        $ref: '#/components/schemas/_common___ExpandWildcards'
  requestBodies:
    search:
      content:
        application/json:
          schema:
            type: object
    bulk:
      required: true
      content:
        application/x-ndjson:
          schema:
            type: array
  schemas:
    _common___IndexName:
      type: string
    _common___Indices:
      oneOf:
        - $ref: '#/components/schemas/_common___IndexName'
        - type: array
          items:
            $ref: '#/components/schemas/_common___IndexName'
    _common___ExpandWildcard:
      type: string
      enum: [all, closed, hidden, none, open]
    _common___ExpandWildcards:
      oneOf:
        - $ref: '#/components/schemas/_common___ExpandWildcard'
        - type: array
          items:
            $ref: '#/components/schemas/_common___ExpandWildcard'
    _common___ByteUnit:
      oneOf:
        - type: string
          const: b
        - type: string
          enum: [kb, k]
        - type: string
          enum: [mb, m]
    _common___HealthStatus:
      oneOf:
        - type: string
          enum: [green, GREEN]
        - type: string
          enum: [yellow, YELLOW]
        - type: string
          enum: [red, RED]
"#;

    fn read_fixture(spec: &str) -> anyhow::Result<Api> {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        file.write_all(spec.as_bytes()).unwrap();
        read_api("test", file.path())
    }

    /// Reads a spec expected to fail, returning the error message
    fn read_fixture_err(spec: &str) -> String {
        match read_fixture(spec) {
            Ok(_) => panic!("expected reading the spec to fail"),
            Err(e) => e.to_string(),
        }
    }

    fn fixture_api() -> Api {
        read_fixture(FIXTURE).unwrap()
    }

    #[test]
    fn reads_endpoints_into_namespaces() {
        let api = fixture_api();
        assert!(api.namespaces.contains_key("cat"));
        assert!(api.root.endpoints().contains_key("search"));
        assert!(api.root.endpoints().contains_key("bulk"));
    }

    #[test]
    fn merges_methods_and_paths_of_an_operation_group() {
        let api = fixture_api();
        let search = &api.root.endpoints()["search"];
        assert_eq!(search.url.paths.len(), 2);
        let root_path = search
            .url
            .paths
            .iter()
            .find(|p| p.path.0 == "/_search")
            .unwrap();
        assert_eq!(root_path.methods.len(), 2);
        let index_path = search
            .url
            .paths
            .iter()
            .find(|p| p.path.0 == "/{index}/_search")
            .unwrap();
        assert_eq!(index_path.parts["index"].ty, TypeKind::List);
    }

    #[test]
    fn maps_enum_parameters_with_options() {
        let api = fixture_api();
        let search = &api.root.endpoints()["search"];
        let param = &search.params["expand_wildcards"];
        assert_eq!(param.ty, TypeKind::Enum);
        let options: Vec<&str> = param.options.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(options, vec!["all", "closed", "hidden", "none", "open"]);
        // enum collected at api level
        assert!(api.enums.iter().any(|e| e.name == "expand_wildcards"));
    }

    #[test]
    fn aggregates_mixed_one_of_enum_variants() {
        let api = fixture_api();
        let cat = &api.namespaces["cat"].endpoints()["cluster_manager"];

        // const and multi-value enum variants side by side (_common___ByteUnit)
        let bytes = &cat.params["bytes"];
        assert_eq!(bytes.ty, TypeKind::Enum);
        let options: Vec<&str> = bytes.options.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(options, vec!["b", "kb", "k", "mb", "m"]);

        // per-status enum variants with upper-case aliases (_common___HealthStatus)
        let health = &cat.params["health"];
        assert_eq!(health.ty, TypeKind::Enum);
        let options: Vec<&str> = health.options.iter().map(|v| v.as_str().unwrap()).collect();
        assert_eq!(options, vec!["green", "yellow", "red"]);
    }

    #[test]
    fn excludes_global_params_from_endpoint_and_collects_common_params() {
        let api = fixture_api();
        let cat = &api.namespaces["cat"].endpoints()["cluster_manager"];
        assert!(!cat.params.contains_key("pretty"));
        assert!(cat.params.contains_key("local"));
        assert!(api.common_params.contains_key("pretty"));
    }

    #[test]
    fn marks_fully_deprecated_groups() {
        let api = fixture_api();
        let master = &api.namespaces["cat"].endpoints()["master"];
        let deprecated = master.deprecated.as_ref().unwrap();
        assert_eq!(deprecated.version, "2.0");
    }

    #[test]
    fn detects_ndjson_bulk_body() {
        let api = fixture_api();
        let bulk = &api.root.endpoints()["bulk"];
        assert!(bulk.supports_nd_body());
        let search = &api.root.endpoints()["search"];
        assert!(search.supports_body());
        assert!(!search.supports_nd_body());
    }

    /// A minimal spec with a single query parameter referencing `Root`,
    /// alongside the given `components.schemas` entries (indented by 4)
    fn broken_ref_fixture(schemas: &str) -> String {
        format!(
            r#"
openapi: 3.1.0
info:
  title: Test
  version: 0.0.0
paths:
  /_broken:
    get:
      operationId: broken.0
      x-operation-group: broken
      description: An endpoint with a broken schema reference.
      parameters:
        - name: p
          in: query
          description: A parameter.
          schema:
            $ref: '#/components/schemas/Root'
      responses: {{}}
components:
  schemas:
{schemas}
"#
        )
    }

    #[test]
    fn errors_on_unresolved_schema_ref() {
        let spec = broken_ref_fixture(
            r#"
    Root:
      $ref: '#/components/schemas/Missing'
"#,
        );
        let err = read_fixture_err(&spec);
        assert!(
            err.contains("unresolved schema $ref: #/components/schemas/Missing"),
            "unexpected error: {}", err
        );
    }

    #[test]
    fn errors_on_unsupported_schema_ref() {
        let spec = broken_ref_fixture(
            r#"
    Root:
      $ref: 'other.yaml#/components/schemas/External'
"#,
        );
        let err = read_fixture_err(&spec);
        assert!(
            err.contains("unsupported schema $ref"),
            "unexpected error: {}", err
        );
    }

    #[test]
    fn errors_on_cyclic_schema_ref() {
        let spec = broken_ref_fixture(
            r#"
    Root:
      $ref: '#/components/schemas/Other'
    Other:
      $ref: '#/components/schemas/Root'
"#,
        );
        let err = read_fixture_err(&spec);
        assert!(
            err.contains("too deep or cyclic"),
            "unexpected error: {}", err
        );
    }

    #[test]
    fn errors_on_broken_ref_inside_one_of() {
        // a broken $ref reached during enum collection is a hard error,
        // not a silent fallback to the string type
        let spec = broken_ref_fixture(
            r#"
    Root:
      oneOf:
        - $ref: '#/components/schemas/Missing'
        - type: string
          const: a
"#,
        );
        let err = read_fixture_err(&spec);
        assert!(
            err.contains("unresolved schema $ref: #/components/schemas/Missing"),
            "unexpected error: {}", err
        );
    }

    #[test]
    fn resolves_schema_ref_chains_within_depth_limit() {
        // a valid (acyclic) chain still resolves
        let spec = broken_ref_fixture(
            r#"
    Root:
      $ref: '#/components/schemas/Middle'
    Middle:
      $ref: '#/components/schemas/Leaf'
    Leaf:
      type: boolean
"#,
        );
        let api = read_fixture(&spec).unwrap();
        let broken = &api.root.endpoints()["broken"];
        assert_eq!(broken.params["p"].ty, TypeKind::Boolean);
    }
}
