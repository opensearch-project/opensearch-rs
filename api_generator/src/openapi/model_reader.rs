/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Builds the [ResponseModel] from the resolved client model
//! ([ClientModel]): the client model has already followed every `$ref`,
//! merged every `allOf` and classified every union, so this reader only maps
//! its object, union and alias entries onto [TypeModel]s. Operation groups,
//! status codes and paths still come from the OpenAPI document.

use super::client_model::{ClientModel, ObjectShape, TypeEntry, TypeExpr, UnionEntry};
use super::responses::{
    is_head_only, prune_unreachable, resolve_union_strategies, root_namespace, type_name,
    typed_statuses, AliasModel, FieldModel, RawCause, ResponseModel, Scalar, StrEnumModel,
    StrEnumValue, StructModel, TypeModel, TypeRef, UnionModel, VariantModel, SUCCESS_STATUS_CODES,
};
use anyhow::anyhow;
use inflector::Inflector;
use serde_json::{Map, Value};
use std::collections::{BTreeSet, VecDeque};

/// Reads the response model for the given operation groups: the response
/// bodies and their types from the client model, the operation groups and
/// declared status codes from the OpenAPI document
pub fn read_response_model(
    spec: &Value,
    model: &ClientModel,
    operation_groups: &[&str],
) -> anyhow::Result<ResponseModel> {
    let empty = Map::new();
    let schemas = spec["components"]["schemas"].as_object().unwrap_or(&empty);
    let responses = spec["components"]["responses"]
        .as_object()
        .unwrap_or(&empty);

    let mut reader = ModelReader {
        model,
        out: ResponseModel::default(),
        queue: VecDeque::new(),
        queued: BTreeSet::new(),
    };

    for group in operation_groups {
        // HEAD responses carry no body by definition (RFC 9110 §9.3.2), even
        // where the specification declares one (e.g. ism.exists_policy)
        if is_head_only(spec, group) {
            reader.out.untyped_groups.insert(
                group.to_string(),
                "HEAD-only endpoint: responses have no body".to_string(),
            );
            continue;
        }
        // the first declared success response with a JSON body is the response type
        let declared: Vec<&str> = SUCCESS_STATUS_CODES
            .iter()
            .copied()
            .filter(|code| responses.contains_key(&format!("{}___{}", group, code)))
            .collect();
        if declared.is_empty() {
            reader.out.untyped_groups.insert(
                group.to_string(),
                "no success response declared".to_string(),
            );
            continue;
        }
        let body = declared
            .iter()
            .filter_map(|code| model.responses.get(&format!("{}___{}", group, code)))
            .find(|body| body.media_type == "application/json");
        let body = match body {
            Some(b) => b,
            None => {
                reader.out.untyped_groups.insert(
                    group.to_string(),
                    format!(
                        "success response ({}) has no application/json body",
                        declared.join(", ")
                    ),
                );
                continue;
            }
        };

        let root = match &body.ty {
            // a plain reference: the referenced type is the root type
            TypeExpr::Ref { name } => {
                reader.enqueue(name);
                name.clone()
            }
            // an inline body: synthesize a root type named after the group
            inline => {
                let name = format!(
                    "{}___{}Response",
                    root_namespace(group),
                    group.split('.').next_back().unwrap().to_pascal_case()
                );
                let ty = reader.convert_type(inline, &name, body.description.clone())?;
                reader.out.types.insert(name.clone(), ty);
                name
            }
        };
        let success = declared
            .iter()
            .map(|code| {
                &responses[&format!("{}___{}", group, code)]["content"]["application/json"]
                    ["schema"]
            })
            .find(|s| !s.is_null())
            .unwrap_or(&Value::Null);
        let statuses = typed_statuses(schemas, responses, group, success);
        if !statuses.is_empty() {
            reader
                .out
                .typed_statuses
                .insert(group.to_string(), statuses);
        }
        reader.out.endpoints.insert(group.to_string(), root);
    }

    // drain the worklist of referenced types
    while let Some(name) = reader.queue.pop_front() {
        let ty = reader.convert_named(&name)?;
        reader.out.types.insert(name, ty);
    }

    resolve_union_strategies(&mut reader.out);
    prune_unreachable(&mut reader.out);
    Ok(reader.out)
}

/// A fixed string value with the title of its enclosing union branch
type ConstValue = (String, Option<String>);

struct ModelReader<'a> {
    model: &'a ClientModel,
    out: ResponseModel,
    queue: VecDeque<String>,
    queued: BTreeSet<String>,
}

impl ModelReader<'_> {
    /// Adds a named type to the conversion worklist
    fn enqueue(&mut self, name: &str) {
        if self.queued.insert(name.to_string()) {
            self.queue.push_back(name.to_string());
        }
    }

    fn entry(&self, name: &str) -> anyhow::Result<&TypeEntry> {
        self.model
            .get(name)
            .ok_or_else(|| anyhow!("type {} is not in the client model", name))
    }

    /// Converts a named type of the client model
    fn convert_named(&mut self, name: &str) -> anyhow::Result<TypeModel> {
        let model = self.model;
        let entry = model
            .get(name)
            .ok_or_else(|| anyhow!("type {} is not in the client model", name))?;
        let description = entry.description().map(str::to_string);
        match entry {
            TypeEntry::Alias(alias) if alias.generic_parameter => {
                Ok(TypeModel::Alias(AliasModel {
                    description,
                    ty: TypeRef::Raw(RawCause::GenericParameter),
                }))
            }
            TypeEntry::Union(union) => {
                if let Some(values) = self.const_values_of_union(union, None, 0)? {
                    return Ok(TypeModel::StrEnum(StrEnumModel {
                        description,
                        values: merge_case_aliases(values),
                    }));
                }
                let members = union.members();
                let union =
                    self.convert_union(&members, union.titles.as_ref(), name, description)?;
                // strategy is resolved globally once the closure is complete
                Ok(TypeModel::LazyUnion(union))
            }
            TypeEntry::Object(object) => Ok(TypeModel::Struct(self.expand(
                &object.shape,
                name,
                description,
            )?)),
            TypeEntry::Alias(alias) => self.convert_type(&alias.ty, name, description),
        }
    }

    /// Converts the type of a named alias or of an inline response body into
    /// the [TypeModel] registered under `name`
    fn convert_type(
        &mut self,
        ty: &TypeExpr,
        name: &str,
        description: Option<String>,
    ) -> anyhow::Result<TypeModel> {
        match ty {
            TypeExpr::Object { object, .. } => {
                Ok(TypeModel::Struct(self.expand(object, name, description)?))
            }
            // an object without fixed fields is a struct of its remaining keys
            TypeExpr::Map { map } => Ok(TypeModel::Struct(StructModel {
                description,
                fields: Vec::new(),
                residual: Some(self.residual(map, name)?),
            })),
            TypeExpr::Union { union, titles, .. } => {
                if let Some(values) = self.const_values(ty, None, 0)? {
                    return Ok(TypeModel::StrEnum(StrEnumModel {
                        description,
                        values: merge_case_aliases(values),
                    }));
                }
                let members: Vec<&TypeExpr> = union.iter().collect();
                Ok(TypeModel::LazyUnion(self.convert_union(
                    &members,
                    titles.as_ref(),
                    name,
                    description,
                )?))
            }
            TypeExpr::Enum { .. } => match self.const_values(ty, None, 0)? {
                Some(values) => Ok(TypeModel::StrEnum(StrEnumModel {
                    description,
                    values: merge_case_aliases(values),
                })),
                None => {
                    let (ty, _) = self.convert_ref(ty, name, None)?;
                    Ok(TypeModel::Alias(AliasModel { description, ty }))
                }
            },
            other => {
                let (ty, _) = self.convert_ref(other, name, None)?;
                Ok(TypeModel::Alias(AliasModel { description, ty }))
            }
        }
    }

    /// The fields of an object, in model order
    fn expand(
        &mut self,
        shape: &ObjectShape,
        name: &str,
        description: Option<String>,
    ) -> anyhow::Result<StructModel> {
        let mut fields = Vec::new();
        for field_name in self.field_order(shape, 0) {
            let field = match shape.fields.get(&field_name) {
                Some(f) => f,
                None => continue,
            };
            let ctx = format!("{}{}", name, field_name.to_pascal_case());
            let (ty, nullable) = self.convert_ref(&field.ty, &ctx, field.description.clone())?;
            fields.push(FieldModel {
                json_name: field_name.clone(),
                ty,
                required: field.required,
                nullable,
                description: field.description.clone(),
            });
        }
        let residual = match &shape.additional_fields {
            None => None,
            Some(additional) => Some(self.residual(additional, name)?),
        };
        Ok(StructModel {
            description,
            fields,
            residual,
        })
    }

    /// The order of an object's fields in the generated struct: the fields
    /// of each base in turn (recursively), then the type's own fields, each
    /// group by name. A field keeps the position of its first declaration.
    fn field_order(&self, shape: &ObjectShape, depth: usize) -> Vec<String> {
        let mut order: Vec<String> = Vec::new();
        if depth < 16 {
            for base in &shape.bases {
                if let Some(TypeEntry::Object(o)) = self.model.get(base) {
                    for name in self.field_order(&o.shape, depth + 1) {
                        if !order.contains(&name) {
                            order.push(name);
                        }
                    }
                }
            }
        }
        let mut own: Vec<String> = shape
            .fields
            .iter()
            .map(|(name, _)| name.clone())
            .filter(|name| !order.contains(name))
            .collect();
        own.sort();
        order.extend(own);
        order
    }

    /// The map field collecting the keys not listed in an object's fields
    fn residual(&mut self, additional: &TypeExpr, name: &str) -> anyhow::Result<(String, TypeRef)> {
        if matches!(additional, TypeExpr::Any { .. }) {
            return Ok(("additional".to_string(), TypeRef::Raw(RawCause::OpenObject)));
        }
        let (ty, _) = self.convert_ref(additional, &format!("{}Additional", name), None)?;
        // sub-aggregations keep their conventional field name
        let field_name = match &ty {
            TypeRef::Named(n) if n.ends_with("___Aggregate") => "aggregations",
            _ => "additional",
        };
        Ok((field_name.to_string(), ty))
    }

    /// Converts a union's members into a [UnionModel], synthesizing types
    /// for inline members
    fn convert_union(
        &mut self,
        members: &[&TypeExpr],
        titles: Option<&Vec<Option<String>>>,
        name: &str,
        description: Option<String>,
    ) -> anyhow::Result<UnionModel> {
        let mut variants: Vec<VariantModel> = Vec::new();
        let mut nullable = false;
        for (index, member) in members.iter().enumerate() {
            if is_null(member) {
                nullable = true;
                continue;
            }
            let title = titles.and_then(|t| t.get(index)).and_then(|t| t.as_deref());
            let mut variant_name = match (title, member) {
                (Some(t), _) => t.to_snake_case(),
                (None, TypeExpr::Ref { name }) => type_name(name).to_snake_case(),
                (None, other) => variant_kind_name(other),
            };
            // a title shared by a variant and an inline object variant
            // (e.g. `value: string` next to an object with a `value` field)
            // yields the same name: disambiguate by JSON kind, then by index
            if variants.iter().any(|v| v.name == variant_name) {
                let by_kind = format!("{}_{}", variant_name, variant_kind_name(member));
                variant_name = if variants.iter().any(|v| v.name == by_kind) {
                    format!("{}_{}", variant_name, variants.len())
                } else {
                    by_kind
                };
            }
            let ctx = format!("{}{}", name, variant_name.to_pascal_case());
            let (ty, _) = self.convert_ref(member, &ctx, None)?;
            variants.push(VariantModel {
                name: variant_name,
                ty,
                kind: None,
            });
        }
        Ok(UnionModel {
            description,
            variants,
            nullable,
        })
    }

    /// Converts a type in a field, item, map value or variant position into
    /// a [TypeRef], registering synthesized types for inline objects, unions
    /// and enums under `ctx`. Returns the reference and whether `null` is
    /// allowed. `description` documents a synthesized type.
    fn convert_ref(
        &mut self,
        ty: &TypeExpr,
        ctx: &str,
        description: Option<String>,
    ) -> anyhow::Result<(TypeRef, bool)> {
        Ok(match ty {
            TypeExpr::Ref { name } => {
                // generic type parameters (e.g. TDocument): the caller decides
                if let TypeEntry::Alias(a) = self.entry(name)? {
                    if a.generic_parameter {
                        return Ok((TypeRef::Raw(RawCause::GenericParameter), false));
                    }
                }
                self.enqueue(name);
                (TypeRef::Named(name.clone()), false)
            }
            TypeExpr::Union { union, titles, .. } => {
                let nullable = union.iter().any(is_null);
                let non_null: Vec<&TypeExpr> = union.iter().filter(|m| !is_null(m)).collect();
                // [null, X] is just a nullable X
                if non_null.len() == 1 {
                    let (inner, _) = self.convert_ref(non_null[0], ctx, description)?;
                    return Ok((inner, true));
                }
                // a union of fixed strings is an open string enum
                if let Some(values) = self.const_values(ty, None, 0)? {
                    self.out.types.insert(
                        ctx.to_string(),
                        TypeModel::StrEnum(StrEnumModel {
                            description,
                            values: merge_case_aliases(values),
                        }),
                    );
                    return Ok((TypeRef::Named(ctx.to_string()), nullable));
                }
                let members: Vec<&TypeExpr> = union.iter().collect();
                let union = self.convert_union(&members, titles.as_ref(), ctx, description)?;
                self.out
                    .types
                    .insert(ctx.to_string(), TypeModel::LazyUnion(union));
                (TypeRef::Named(ctx.to_string()), nullable)
            }
            TypeExpr::Enum { values } => match self.const_values(ty, None, 0)? {
                Some(strings) => {
                    self.out.types.insert(
                        ctx.to_string(),
                        TypeModel::StrEnum(StrEnumModel {
                            description,
                            values: merge_case_aliases(strings),
                        }),
                    );
                    (TypeRef::Named(ctx.to_string()), false)
                }
                // fixed non-string values: the type of the values
                None => (scalar_of_values(values), false),
            },
            // an object without declared fields in a field position is an
            // open map, as in the OpenAPI document
            TypeExpr::Object { object, .. }
                if object.fields.is_empty() && object.additional_fields.is_none() =>
            {
                (
                    TypeRef::Map(Box::new(TypeRef::Raw(RawCause::OpenObject))),
                    false,
                )
            }
            TypeExpr::Object {
                object,
                description: own,
            } => {
                let s = self.expand(object, ctx, own.clone().or(description))?;
                self.out.types.insert(ctx.to_string(), TypeModel::Struct(s));
                (TypeRef::Named(ctx.to_string()), false)
            }
            TypeExpr::Map { map } => match map.as_ref() {
                TypeExpr::Any { .. } => (
                    TypeRef::Map(Box::new(TypeRef::Raw(RawCause::OpenObject))),
                    false,
                ),
                value => {
                    let (value, _) = self.convert_ref(value, &format!("{}Value", ctx), None)?;
                    (TypeRef::Map(Box::new(value)), false)
                }
            },
            TypeExpr::Array { array } => match array.as_ref() {
                TypeExpr::Any { .. } => (
                    TypeRef::List(Box::new(TypeRef::Raw(RawCause::UntypedItems))),
                    false,
                ),
                item => {
                    let (item, _) = self.convert_ref(item, &format!("{}Item", ctx), None)?;
                    (TypeRef::List(Box::new(item)), false)
                }
            },
            TypeExpr::Scalar { scalar, format } => match scalar.as_str() {
                "boolean" => (TypeRef::Scalar(Scalar::Bool), false),
                // only an explicit int32 is narrowed: an integer without a
                // format is any JSON integer, and real responses carry epoch
                // milliseconds and byte counts in such fields (e.g. ISM
                // `last_updated_time`), which overflow i32
                "integer" => match format.as_deref() {
                    Some("int32") => (TypeRef::Scalar(Scalar::I32), false),
                    Some("uint64") => (TypeRef::Scalar(Scalar::U64), false),
                    _ => (TypeRef::Scalar(Scalar::I64), false),
                },
                "number" => (TypeRef::Scalar(Scalar::F64), false),
                "string" => (TypeRef::Scalar(Scalar::String), false),
                "null" => (TypeRef::Raw(RawCause::NullOnly), true),
                other => return Err(anyhow!("unknown scalar type {} in the client model", other)),
            },
            // no type at all: untyped JSON
            TypeExpr::Any { .. } => (TypeRef::Raw(RawCause::NoType), false),
        })
    }

    /// The fixed string values of a type whose members are all string
    /// enums, following references, with the title of the enclosing union
    /// branch of each value. `None` when any member allows other values.
    fn const_values(
        &self,
        ty: &TypeExpr,
        group: Option<&str>,
        depth: usize,
    ) -> anyhow::Result<Option<Vec<ConstValue>>> {
        if depth > 4 {
            return Ok(None);
        }
        match ty {
            TypeExpr::Enum { values } => {
                if values.is_empty() || !values.iter().all(Value::is_string) {
                    return Ok(None);
                }
                Ok(Some(
                    values
                        .iter()
                        .map(|v| (v.as_str().unwrap().to_string(), group.map(str::to_string)))
                        .collect(),
                ))
            }
            TypeExpr::Ref { name } => match self.entry(name)? {
                TypeEntry::Alias(alias) if !alias.generic_parameter => {
                    self.const_values(&alias.ty, group, depth)
                }
                TypeEntry::Union(union) => self.const_values_of_union(union, group, depth),
                _ => Ok(None),
            },
            TypeExpr::Union { union, titles, .. } => {
                let members: Vec<&TypeExpr> = union.iter().collect();
                self.const_values_of_members(&members, titles.as_ref(), group, depth)
            }
            _ => Ok(None),
        }
    }

    fn const_values_of_union(
        &self,
        union: &UnionEntry,
        group: Option<&str>,
        depth: usize,
    ) -> anyhow::Result<Option<Vec<ConstValue>>> {
        self.const_values_of_members(&union.members(), union.titles.as_ref(), group, depth)
    }

    fn const_values_of_members(
        &self,
        members: &[&TypeExpr],
        titles: Option<&Vec<Option<String>>>,
        group: Option<&str>,
        depth: usize,
    ) -> anyhow::Result<Option<Vec<ConstValue>>> {
        let mut out = Vec::new();
        for (index, member) in members.iter().enumerate() {
            let title = titles.and_then(|t| t.get(index)).and_then(|t| t.as_deref());
            match self.const_values(member, title.or(group), depth + 1)? {
                Some(values) => out.extend(values),
                None => return Ok(None),
            }
        }
        Ok(if out.is_empty() { None } else { Some(out) })
    }
}

/// Merges spellings differing only in letter case (`ASYNC` / `async`) into
/// one value with aliases, unless they are in differently titled union
/// branches: case can carry meaning (`1m` is a minute, `1M` a month)
fn merge_case_aliases(raw: Vec<ConstValue>) -> Vec<StrEnumValue> {
    let mut values: Vec<(StrEnumValue, Option<String>)> = Vec::new();
    for (value, group) in raw {
        let known = values.iter_mut().find(|(v, g)| {
            *g == group
                && (v.value.eq_ignore_ascii_case(&value)
                    || v.aliases.iter().any(|a| a.eq_ignore_ascii_case(&value)))
        });
        match known {
            Some((v, _)) => {
                if v.value != value && !v.aliases.contains(&value) {
                    v.aliases.push(value);
                }
            }
            None => values.push((
                StrEnumValue {
                    value,
                    aliases: Vec::new(),
                },
                group,
            )),
        }
    }
    values.into_iter().map(|(v, _)| v).collect()
}

fn is_null(ty: &TypeExpr) -> bool {
    matches!(ty, TypeExpr::Scalar { scalar, .. } if scalar == "null")
}

/// The scalar type of fixed non-string values (`enum: [true]`)
fn scalar_of_values(values: &[Value]) -> TypeRef {
    if values.iter().all(Value::is_boolean) {
        TypeRef::Scalar(Scalar::Bool)
    } else if values.iter().all(|v| v.is_i64() || v.is_u64()) {
        TypeRef::Scalar(Scalar::I64)
    } else if values.iter().all(Value::is_number) {
        TypeRef::Scalar(Scalar::F64)
    } else {
        TypeRef::Raw(RawCause::NoType)
    }
}

/// A placeholder variant name derived from the JSON kind of an untitled
/// inline member
fn variant_kind_name(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Scalar { scalar, .. } => scalar.clone(),
        TypeExpr::Enum { values } if values.iter().all(Value::is_string) => "string".to_string(),
        TypeExpr::Array { .. } => "array".to_string(),
        TypeExpr::Object { .. } | TypeExpr::Map { .. } => "object".to_string(),
        _ => "value".to_string(),
    }
}
