/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The resolved client model of opensearch-api-specification
//! (`opensearch-client-model.json`, built by `npm run model` in that
//! repository): every type of the specification with its `$ref`s followed,
//! its `allOf`s merged and its unions classified. The response types are
//! read from this model instead of interpreting `allOf`/`oneOf`/`anyOf` of
//! the OpenAPI document; see "Client Model" in the specification's
//! `CLIENT_GENERATOR_GUIDE.md`.

use anyhow::{anyhow, bail};
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

/// The file name of the client model, vendored next to the OpenAPI document
pub const MODEL_FILE_NAME: &str = "opensearch-client-model.json";

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

    /// The client model of an OpenAPI document: [MODEL_FILE_NAME] next to
    /// it. A document without response components needs none.
    pub fn for_spec(openapi_file: &Path, spec: &Value) -> anyhow::Result<ClientModel> {
        let path = openapi_file.with_file_name(MODEL_FILE_NAME);
        if path.exists() {
            return Self::read(&path);
        }
        let has_responses = spec["components"]["responses"]
            .as_object()
            .map(|r| !r.is_empty())
            .unwrap_or(false);
        if !has_responses {
            return Ok(Self::empty());
        }
        bail!(
            "no client model at {}: build it from the specification with \
             `npm run model -- --label -i {} -o {}` in opensearch-api-specification",
            path.display(),
            openapi_file.display(),
            path.display()
        )
    }

    /// Reads a model file and checks its format and version
    pub fn read(path: &Path) -> anyhow::Result<ClientModel> {
        let text = std::fs::read_to_string(path)
            .map_err(|e| anyhow!("cannot read client model {}: {}", path.display(), e))?;
        Self::parse(&text)
    }

    pub fn parse(text: &str) -> anyhow::Result<ClientModel> {
        let model: ClientModel = serde_json::from_str(text)?;
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
    fn reads_the_vendored_model_in_document_order() {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-client-model.json");
        let model = ClientModel::read(Path::new(path)).unwrap();
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
