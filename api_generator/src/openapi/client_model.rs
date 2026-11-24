/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The resolved client model of opensearch-api-specification: every type of
//! the specification with its `$ref`s followed, its `allOf`s merged and its
//! unions classified. The specification carries it next to the JSON Schema
//! it is built from, as `x-model` on every component schema, response and
//! request body (written by `npm run merge` in that repository). The
//! response types are read from this model instead of interpreting
//! `allOf`/`oneOf`/`anyOf` of the OpenAPI document; see "Client Model" in
//! the specification's `CLIENT_GENERATOR_GUIDE.md`.

use anyhow::bail;
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use serde_json::Value;
use std::fmt;
use std::marker::PhantomData;
use std::path::Path;

/// The model format versions this reader understands
const SUPPORTED_VERSIONS: &[&str] = &["0.2.0"];

#[derive(Debug, Deserialize)]
pub struct ClientModel {
    pub format: String,
    pub version: String,
    pub types: OrderedMap<TypeEntry>,
    #[serde(default)]
    pub responses: OrderedMap<BodyEntry>,
    #[serde(default)]
    pub request_bodies: OrderedMap<BodyEntry>,
}

/// The OpenAPI extension that carries the model in the specification
pub const MODEL_KEY: &str = "x-model";

impl ClientModel {
    /// A model without types, for documents without response components
    pub fn empty() -> ClientModel {
        ClientModel {
            format: "opensearch-client-model".to_string(),
            version: SUPPORTED_VERSIONS[0].to_string(),
            types: OrderedMap::default(),
            responses: OrderedMap::default(),
            request_bodies: OrderedMap::default(),
        }
    }

    /// The client model of the OpenAPI document at `path`, from its
    /// `x-model` extensions (see [ClientModel::from_yaml])
    pub fn from_spec_file(path: &Path) -> anyhow::Result<ClientModel> {
        let file = std::fs::File::open(path)
            .map_err(|e| anyhow::anyhow!("cannot read {}: {}", path.display(), e))?;
        Self::from_yaml(&serde_yaml::from_reader(file)?)
    }

    /// The client model of an OpenAPI document, from its `x-model`
    /// extensions: `components.x-model` (format and version), and the
    /// `x-model` of every component schema (its type), response and request
    /// body (its body). Read from YAML, which keeps the order of fields. A
    /// document without response components needs none.
    pub fn from_yaml(spec: &serde_yaml::Value) -> anyhow::Result<ClientModel> {
        use serde_yaml::{Mapping, Value as Yaml};
        let key = Yaml::from(MODEL_KEY);
        let components = spec.get("components");
        let section = |name: &str| {
            components
                .and_then(|c| c.get(name))
                .and_then(Yaml::as_mapping)
        };
        let header = match components.and_then(|c| c.get(MODEL_KEY)) {
            Some(h) => h.clone(),
            None => {
                if section("responses").map(|r| !r.is_empty()).unwrap_or(false) {
                    bail!(
                        "the specification has no client model (`components.{}`): use a \
                         specification merged by opensearch-api-specification's `npm run merge`",
                        MODEL_KEY
                    );
                }
                return Ok(Self::empty());
            }
        };
        // the entries of one section, by component name in name order
        let entries = |name: &str| -> Mapping {
            let mut out: Vec<(Yaml, Yaml)> = section(name)
                .map(|c| {
                    c.iter()
                        .filter_map(|(k, v)| v.get(&key).map(|m| (k.clone(), m.clone())))
                        .collect()
                })
                .unwrap_or_default();
            out.sort_by(|a, b| a.0.as_str().cmp(&b.0.as_str()));
            out.into_iter().collect()
        };
        let mut model = Mapping::new();
        model.insert(
            "format".into(),
            header.get("format").cloned().unwrap_or(Yaml::Null),
        );
        model.insert(
            "version".into(),
            header.get("version").cloned().unwrap_or(Yaml::Null),
        );
        model.insert("types".into(), Yaml::Mapping(entries("schemas")));
        model.insert("responses".into(), Yaml::Mapping(entries("responses")));
        model.insert(
            "request_bodies".into(),
            Yaml::Mapping(entries("requestBodies")),
        );
        let model: ClientModel = serde_yaml::from_value(Yaml::Mapping(model))?;
        Self::check(model)
    }

    pub fn parse(text: &str) -> anyhow::Result<ClientModel> {
        Self::check(serde_json::from_str(text)?)
    }

    /// Checks the format and version of a model
    fn check(model: ClientModel) -> anyhow::Result<ClientModel> {
        if model.format != "opensearch-client-model" {
            bail!("not an OpenSearch client model: format {}", model.format);
        }
        if !SUPPORTED_VERSIONS.contains(&model.version.as_str()) {
            bail!(
                "unsupported client model version {} (supported: {})",
                model.version,
                SUPPORTED_VERSIONS.join(", ")
            );
        }
        Ok(model)
    }

    pub fn get(&self, name: &str) -> Option<&TypeEntry> {
        self.types.get(name)
    }
}

/// A named type of the model
#[derive(Debug, Deserialize)]
#[serde(tag = "type_kind", rename_all = "snake_case")]
pub enum TypeEntry {
    Object(ObjectEntry),
    Union(UnionEntry),
    Alias(AliasEntry),
}

impl TypeEntry {
    pub fn description(&self) -> Option<&str> {
        match self {
            TypeEntry::Object(o) => o.description.as_deref(),
            TypeEntry::Union(u) => u.description.as_deref(),
            TypeEntry::Alias(a) => a.description.as_deref(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ObjectEntry {
    #[serde(flatten)]
    pub shape: ObjectShape,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct ObjectShape {
    /// the named types merged in through `allOf`, in order
    #[serde(default)]
    pub bases: Vec<String>,
    pub fields: OrderedMap<Field>,
    pub additional_fields: Option<TypeExpr>,
    /// Presence, container and other constraints; response types don't
    /// enforce them
    #[serde(default)]
    pub constraints: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Field {
    #[serde(rename = "type")]
    pub ty: TypeExpr,
    pub required: bool,
    pub description: Option<String>,
    /// the base the field is inherited from; `None` for the type's own fields
    pub declared_in: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UnionEntry {
    pub union_kind: String,
    pub exclusive: bool,
    pub discriminator: Option<String>,
    pub default_variant: Option<String>,
    /// tagged and titled unions: the members by tag value or title
    pub variants: Option<OrderedMap<TypeExpr>>,
    pub members: Option<Vec<TypeExpr>>,
    /// each branch's own title, by position
    pub titles: Option<Vec<Option<String>>>,
    pub description: Option<String>,
}

impl UnionEntry {
    /// The members in branch order
    pub fn members(&self) -> Vec<&TypeExpr> {
        match (&self.variants, &self.members) {
            (Some(v), _) => v.0.iter().map(|(_, t)| t).collect(),
            (None, Some(m)) => m.iter().collect(),
            (None, None) => Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AliasEntry {
    #[serde(rename = "type")]
    pub ty: TypeExpr,
    #[serde(default)]
    pub generic_parameter: bool,
    pub description: Option<String>,
}

/// The JSON body of a response or request body component
#[derive(Debug, Deserialize)]
pub struct BodyEntry {
    pub media_type: String,
    #[serde(rename = "type")]
    pub ty: TypeExpr,
    pub description: Option<String>,
}

/// The type of a field, item, map value or union member
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum TypeExpr {
    Ref {
        #[serde(rename = "ref")]
        name: String,
    },
    Scalar {
        scalar: String,
        format: Option<String>,
    },
    Enum {
        #[serde(rename = "enum")]
        values: Vec<Value>,
    },
    Array {
        array: Box<TypeExpr>,
    },
    Map {
        map: Box<TypeExpr>,
    },
    Union {
        union: Vec<TypeExpr>,
        union_kind: String,
        exclusive: bool,
        titles: Option<Vec<Option<String>>>,
    },
    Object {
        object: Box<ObjectShape>,
        description: Option<String>,
    },
    Any {
        any: bool,
    },
}

/// A JSON object read in document order (the model lists fields and
/// variants in declaration order)
#[derive(Debug, Clone, PartialEq)]
pub struct OrderedMap<V>(pub Vec<(String, V)>);

impl<V> Default for OrderedMap<V> {
    fn default() -> Self {
        OrderedMap(Vec::new())
    }
}

impl<V> OrderedMap<V> {
    pub fn get(&self, key: &str) -> Option<&V> {
        self.0.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &V)> {
        self.0.iter().map(|(k, v)| (k, v))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'de, V: Deserialize<'de>> Deserialize<'de> for OrderedMap<V> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct OrderedVisitor<V>(PhantomData<V>);
        impl<'de, V: Deserialize<'de>> Visitor<'de> for OrderedVisitor<V> {
            type Value = OrderedMap<V>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a JSON object")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
                let mut entries = Vec::with_capacity(map.size_hint().unwrap_or(0));
                while let Some((k, v)) = map.next_entry::<String, V>()? {
                    entries.push((k, v));
                }
                Ok(OrderedMap(entries))
            }
        }
        deserializer.deserialize_map(OrderedVisitor(PhantomData))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_spec_without_x_model_needs_responses_to_fail() {
        // no responses: nothing to type, an empty model
        let bare = serde_yaml::from_str(r#"{ "components": { "schemas": {} } }"#).unwrap();
        assert!(ClientModel::from_yaml(&bare).unwrap().types.is_empty());
        // responses but no x-model: the spec was not merged with the model
        let unmerged = serde_yaml::from_str(
            r#"{ "components": { "schemas": {}, "responses": { "a___200": {} } } }"#,
        )
        .unwrap();
        let error = ClientModel::from_yaml(&unmerged).unwrap_err().to_string();
        assert!(error.contains("no client model"), "{}", error);
    }

    #[test]
    fn reads_the_entries_of_every_section() {
        let spec = serde_yaml::from_str(r#"{ "components": {
            "x-model": { "format": "opensearch-client-model", "version": "0.2.0" },
            "schemas": {
                "B": { "type": "string", "x-model": { "type_kind": "alias", "type": { "scalar": "string" } } },
                "A": { "type": "object", "x-model": { "type_kind": "object", "fields": {}, "constraints": [] } },
                "C": { "type": "object" }
            },
            "responses": { "a___200": { "x-model": { "media_type": "application/json", "type": { "ref": "A" } } } },
            "requestBodies": { "a": { "x-model": { "media_type": "application/json", "type": { "ref": "B" } } } }
        } }"#).unwrap();
        let model = ClientModel::from_yaml(&spec).unwrap();
        assert!(matches!(model.get("A"), Some(TypeEntry::Object(_))));
        assert!(matches!(model.get("B"), Some(TypeEntry::Alias(_))));
        assert!(model.get("C").is_none());
        assert_eq!(
            model.responses.get("a___200").unwrap().media_type,
            "application/json"
        );
        assert_eq!(
            model.request_bodies.get("a").unwrap().media_type,
            "application/json"
        );
    }

    #[test]
    fn reads_the_model_of_the_vendored_spec_in_document_order() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let model = ClientModel::from_spec_file(Path::new(path)).unwrap();
        let lookup = match model.get("_common.query_dsl___TermsLookup").unwrap() {
            TypeEntry::Object(o) => o,
            other => panic!("not an object: {:?}", other),
        };
        let names: Vec<&str> = lookup
            .shape
            .fields
            .iter()
            .map(|(k, _)| k.as_str())
            .collect();
        assert_eq!(names, ["index", "id", "path", "routing", "store"]);
        assert!(model.responses.get("search___200").is_some());
    }

    #[test]
    fn rejects_unknown_versions() {
        let text = r#"{"format":"opensearch-client-model","version":"9.0.0","types":{}}"#;
        assert!(ClientModel::parse(text).is_err());
    }
}
