/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Generates typed response models under `opensearch/src/types/` from a
//! [ResponseIr] built by the OpenAPI response reader.
//!
//! One module is generated per specification namespace (`_core.search`
//! becomes `types/search.rs`, `_common.aggregations` becomes
//! `types/common_aggregations.rs`, ...). All cross-type references use
//! fully qualified `crate::types::...` paths, so the modules need no use
//! declarations and cannot conflict.

use crate::openapi::responses::{
    namespace, type_name, IrField, IrRef, IrStrEnum, IrStruct, IrType, IrUnion, JsonKind,
    ResponseIr, Scalar,
};
use inflector::Inflector;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

/// Generates the response type modules: returns `(module name, source)`
/// pairs and the content of the `response-modules` section of
/// `types/mod.rs`
pub fn generate(ir: &ResponseIr) -> anyhow::Result<(Vec<(String, String)>, String)> {
    let boxed = boxed_edges(ir);
    let total_hits_unions = total_hits_unions(ir);

    // render each type into its module
    let mut modules: BTreeMap<String, Vec<(String, String)>> = BTreeMap::new();
    for (ir_name, ty) in &ir.types {
        let module = module_name(namespace(ir_name));
        let rust_name = rust_type_name(ir_name);
        let source = match ty {
            IrType::Struct(s) => render_struct(ir_name, &rust_name, s, &boxed, &total_hits_unions),
            IrType::DispatchUnion(u) => {
                let mut out = render_dispatch_union(ir_name, &rust_name, u, &boxed);
                if total_hits_unions.contains(ir_name) {
                    render_total_hit_count(&mut out, &rust_name, u);
                }
                out
            }
            IrType::LazyUnion(u) => render_lazy_union(&rust_name, u),
            IrType::StrEnum(e) => render_str_enum(&rust_name, e),
            IrType::Alias(a) => {
                let mut out = String::new();
                doc(&mut out, a.description.as_deref());
                let _ = writeln!(out, "pub type {} = {};", rust_name, render_ref(&a.ty));
                out
            }
        };
        modules.entry(module).or_default().push((rust_name, source));
    }

    // per-endpoint response aliases, e.g. `IndexResponse` for the `index`
    // operation group whose root schema is the shared WriteResponseBase
    for (group, root) in &ir.endpoints {
        let (desired_module, desired_name) =
            crate::openapi::responses::endpoint_response_alias(group);
        let root_module = module_name(namespace(root));
        let root_name = rust_type_name(root);
        if root_module == desired_module && root_name == desired_name {
            continue;
        }
        let mut out = String::new();
        doc(
            &mut out,
            Some(&format!("The response of the `{}` API", group)),
        );
        let _ = writeln!(
            out,
            "pub type {} = crate::types::{}::{};",
            desired_name, root_module, root_name
        );
        modules
            .entry(desired_module)
            .or_default()
            .push((desired_name, out));
    }

    let mut files = Vec::new();
    let mut section = String::new();
    for (module, mut types) in modules {
        types.sort_by(|a, b| a.0.cmp(&b.0));
        let mut source = String::new();
        let _ = writeln!(
            source,
            "//! Response types of the `{}` specification namespace\n",
            module
        );
        source.push_str("#![allow(missing_docs)]\n\n");
        for (_, ty_source) in types {
            source.push_str(&ty_source);
            source.push('\n');
        }
        let _ = writeln!(section, "pub mod {};", module);
        files.push((module, source));
    }

    Ok((files, section))
}

/// The Rust module name of a specification namespace
pub fn module_name(ns: &str) -> String {
    let ns = ns.strip_prefix("_core.").unwrap_or(ns);
    let ns = ns.strip_prefix('_').unwrap_or(ns);
    let ns = if ns.is_empty() { "common" } else { ns };
    // `cluster._common` -> `cluster_common`, not `cluster__common`
    ns.replace("._", "_").replace('.', "_")
}

/// The Rust type name of an IR name
fn rust_type_name(ir_name: &str) -> String {
    let name = type_name(ir_name);
    if name.chars().next().is_some_and(|c| c.is_lowercase()) {
        name.to_pascal_case()
    } else {
        name.to_string()
    }
}

/// Renders an [IrRef] as a fully qualified Rust type
fn render_ref(ty: &IrRef) -> String {
    match ty {
        IrRef::Named(n) => format!(
            "crate::types::{}::{}",
            module_name(namespace(n)),
            rust_type_name(n)
        ),
        IrRef::Scalar(s) => scalar_type(*s).to_string(),
        IrRef::List(t) => format!("Vec<{}>", render_ref(t)),
        IrRef::Map(t) => format!("std::collections::BTreeMap<String, {}>", render_ref(t)),
        IrRef::Raw(_) => "Box<serde_json::value::RawValue>".to_string(),
    }
}

fn scalar_type(s: Scalar) -> &'static str {
    match s {
        Scalar::Bool => "bool",
        Scalar::I32 => "i32",
        Scalar::I64 => "i64",
        Scalar::U64 => "u64",
        Scalar::F64 => "f64",
        Scalar::String => "String",
    }
}

/// Appends a `#[doc = "..."]` attribute
fn doc(out: &mut String, description: Option<&str>) {
    if let Some(d) = description {
        let escaped = d
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', " ");
        let _ = writeln!(out, "#[doc = \"{}\"]", escaped);
    }
}

/// Rust keywords that require a raw identifier
const KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "do", "dyn",
    "else", "enum", "extern", "false", "final", "fn", "for", "if", "impl", "in", "let", "loop",
    "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref", "return", "static",
    "struct", "trait", "true", "try", "type", "typeof", "unsafe", "unsized", "use", "virtual",
    "where", "while", "yield",
];

/// Converts a JSON property name into a Rust field identifier
fn field_ident(json_name: &str, taken: &BTreeSet<String>) -> String {
    let trimmed = json_name.trim_start_matches('_');
    let mut ident = trimmed.to_snake_case();
    if ident.is_empty() || taken.contains(&ident) {
        // e.g. "_type" alongside "type": keep the underscore prefix
        ident = format!("_{}", trimmed.to_snake_case());
    }
    if KEYWORDS.contains(&ident.as_str()) {
        ident = format!("r#{}", ident);
    }
    if ident.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        ident = format!("_{}", ident);
    }
    ident
}

/// A struct field prepared for rendering
struct Field<'a> {
    ir: &'a IrField,
    ident: String,
    /// The payload type (without the `Option` of optional fields)
    payload: String,
    optional: bool,
}

/// Prepares the fields of a struct: identifiers, payload types (boxed
/// when part of a containment cycle) and optionality
fn prepare_fields<'a>(
    ir_name: &str,
    s: &'a IrStruct,
    boxed: &BTreeSet<(String, String)>,
) -> Vec<Field<'a>> {
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut fields = Vec::new();
    for f in &s.fields {
        let ident = field_ident(&f.json_name, &taken);
        taken.insert(ident.clone());
        let mut payload = render_ref(&f.ty);
        if let IrRef::Named(target) = &f.ty {
            if boxed.contains(&(ir_name.to_string(), target.clone())) {
                payload = format!("Box<{}>", payload);
            }
        }
        fields.push(Field {
            ir: f,
            ident,
            payload,
            optional: !f.required || f.nullable,
        });
    }
    fields
}

/// Renders a struct. Structs with typed residual keys (sub-aggregations)
/// get hand-rolled `Deserialize`/`Serialize` impls that route unknown keys
/// into the residual map; plain structs derive both.
fn render_struct(
    ir_name: &str,
    rust_name: &str,
    s: &IrStruct,
    boxed: &BTreeSet<(String, String)>,
    total_hits_unions: &BTreeSet<String>,
) -> String {
    // an object with no fixed fields is just a map
    if s.fields.is_empty() {
        let mut out = String::new();
        doc(&mut out, s.description.as_deref());
        let value = match &s.residual {
            Some((_, ty)) => render_ref(ty),
            None => "serde_json::Value".to_string(),
        };
        let _ = writeln!(
            out,
            "pub type {} = std::collections::BTreeMap<String, {}>;",
            rust_name, value
        );
        return out;
    }

    let fields = prepare_fields(ir_name, s, boxed);
    let mut out = String::new();

    // A struct whose specification declares a residual
    // (`additionalProperties`, sub-aggregations in buckets) collects the
    // remaining keys into that map through a generated visitor. Every other
    // struct derives its impls: keys the specification does not define are
    // ignored, as by the other OpenSearch clients; the typed client can
    // report them instead (`ignore_unknown_fields(false)`).
    doc(&mut out, s.description.as_deref());
    if s.residual.is_some() {
        out.push_str("#[derive(Debug, Clone)]\n");
    } else {
        out.push_str("#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]\n");
    }
    let _ = writeln!(out, "pub struct {} {{", rust_name);
    for f in &fields {
        doc(&mut out, f.ir.description.as_deref());
        if s.residual.is_none() {
            let rename = f.ident.trim_start_matches("r#");
            if rename != f.ir.json_name {
                let _ = writeln!(out, "    #[serde(rename = \"{}\")]", f.ir.json_name);
            }
            if f.optional {
                out.push_str("    #[serde(default, skip_serializing_if = \"Option::is_none\")]\n");
            }
        }
        if f.optional {
            let _ = writeln!(out, "    pub {}: Option<{}>,", f.ident, f.payload);
        } else {
            let _ = writeln!(out, "    pub {}: {},", f.ident, f.payload);
        }
    }
    if let Some((residual_name, residual_ty)) = &s.residual {
        let _ = writeln!(
            out,
            "    pub {}: std::collections::BTreeMap<String, {}>,",
            residual_name,
            render_ref(residual_ty)
        );
    }
    out.push_str("}\n");

    if let Some((residual_name, residual_ty)) = &s.residual {
        render_residual_impls(
            &mut out,
            rust_name,
            &fields,
            residual_name,
            &render_ref(residual_ty),
        );
    }
    render_source_accessor(&mut out, rust_name, &fields);
    render_total_value(&mut out, rust_name, &fields, total_hits_unions);

    out
}

/// The IR name of the object form of a search's total hit count
const TOTAL_HITS: &str = "_core.search___TotalHits";

/// The unions of the shape `TotalHits | integer`: the total hit count of a
/// search (`hits.total`). The specification declares it inline in each kind
/// of hits object, so the generator synthesizes one such union per hits
/// object (search, inner hits, msearch, search_template, plugin searches).
fn total_hits_unions(ir: &ResponseIr) -> BTreeSet<String> {
    ir.types
        .iter()
        .filter_map(|(name, ty)| match ty {
            IrType::DispatchUnion(u) if is_total_hits_union(u) => Some(name.clone()),
            _ => None,
        })
        .collect()
}

fn is_total_hits_union(u: &IrUnion) -> bool {
    if u.nullable || u.variants.len() != 2 {
        return false;
    }
    let object = u
        .variants
        .iter()
        .any(|v| matches!(&v.ty, IrRef::Named(n) if n == TOTAL_HITS));
    let integer = u
        .variants
        .iter()
        .any(|v| matches!(v.ty, IrRef::Scalar(Scalar::I64)));
    object && integer
}

/// Renders the reading of a total hit count union: the
/// `opensearch::types::TotalHitCount` implementation, shared by every such
/// union, and the same methods as inherent methods, callable without
/// importing the trait
fn render_total_hit_count(out: &mut String, rust_name: &str, u: &IrUnion) {
    let variant = |object: bool| {
        u.variants
            .iter()
            .find(|v| matches!(&v.ty, IrRef::Named(_)) == object)
            .map(|v| v.name.to_pascal_case())
            .unwrap()
    };
    let (object, integer) = (variant(true), variant(false));
    let _ = writeln!(
        out,
        r#"impl crate::types::TotalHitCount for {name} {{
    fn value(&self) -> i64 {{
        match self {{
            Self::{object}(total) => total.value,
            Self::{integer}(value) => *value,
        }}
    }}

    fn relation(&self) -> &crate::types::search::TotalHitsRelation {{
        match self {{
            Self::{object}(total) => &total.relation,
            Self::{integer}(_) => &crate::types::total_hits::ACCURATE,
        }}
    }}
}}

impl {name} {{
    #[doc = "The number of hits: exact when [`is_accurate`](Self::is_accurate) is true, and otherwise a lower bound. See [`TotalHitCount`](crate::types::TotalHitCount)."]
    pub fn value(&self) -> i64 {{
        crate::types::TotalHitCount::value(self)
    }}

    #[doc = "How [`value`](Self::value) relates to the actual number of hits: `Eq` when exact, `Gte` when a lower bound. The integer form is always `Eq`."]
    pub fn relation(&self) -> &crate::types::search::TotalHitsRelation {{
        crate::types::TotalHitCount::relation(self)
    }}

    #[doc = "Whether [`value`](Self::value) is the exact number of hits (relation `eq`). False for a lower bound, and for a relation this client does not know."]
    pub fn is_accurate(&self) -> bool {{
        crate::types::TotalHitCount::is_accurate(self)
    }}
}}"#,
        name = rust_name,
        object = object,
        integer = integer
    );
}

/// Renders `total_value` for a hits object holding a total hit count: the
/// number alone, for the common case not needing to know whether it is a
/// lower bound
fn render_total_value(
    out: &mut String,
    rust_name: &str,
    fields: &[Field<'_>],
    total_hits_unions: &BTreeSet<String>,
) {
    let total = match fields.iter().find(|f| {
        f.ir.json_name == "total"
            && matches!(&f.ir.ty, IrRef::Named(n) if total_hits_unions.contains(n))
    }) {
        Some(f) => f,
        None => return,
    };
    let body = if total.optional {
        format!("self.{}.as_ref().map(|total| total.value())", total.ident)
    } else {
        format!("Some(self.{}.value())", total.ident)
    };
    let _ = writeln!(
        out,
        r#"impl {name} {{
    #[doc = "The total number of hits (`total`), or `None` when the request disabled counting (`track_total_hits: false`).

The number may be a lower bound: by default the count is only tracked accurately up to 10,000 hits. Use `total` and its [`is_accurate`](crate::types::TotalHitCount::is_accurate) when that matters."]
    pub fn total_value(&self) -> Option<i64> {{
        {body}
    }}
}}"#,
        name = rust_name,
        body = body
    );
}

/// Renders `source_as` for a struct holding a document body: a `_source`
/// field generated as raw JSON (the generic `TDocument` of the
/// specification, or an untyped `_source`). Documents are user-defined, so
/// the model keeps them raw; `source_as` decodes one into the caller's type
/// without `serde_json` plumbing and without panicking when the response
/// carries no `_source` (`_source: false` in the request, or a mapping that
/// disables it).
fn render_source_accessor(out: &mut String, rust_name: &str, fields: &[Field<'_>]) {
    let source = match fields
        .iter()
        .find(|f| f.ir.json_name == "_source" && matches!(f.ir.ty, IrRef::Raw(_)))
    {
        Some(f) => f,
        None => return,
    };
    let raw = if source.optional {
        format!("self.{}.as_deref()", source.ident)
    } else {
        format!("Some(&*self.{})", source.ident)
    };
    let _ = writeln!(
        out,
        r#"impl {name} {{
    #[doc = "Decodes the document body (`_source`) into `T`.\n\nReturns `Ok(None)` when the response carries no `_source` (`_source: false` in the request, or a mapping that disables it), and an error when the document does not match `T`.\n\n`T` may borrow from the response to avoid copying strings: use `#[serde(borrow)] Cow<'a, str>` fields rather than `&'a str`, which cannot hold a string containing JSON escapes."]
    pub fn source_as<'a, T>(&'a self) -> std::result::Result<Option<T>, serde_json::Error>
    where
        T: serde::Deserialize<'a>,
    {{
        {raw}
            .map(|raw| serde_json::from_str(raw.get()))
            .transpose()
    }}
}}"#,
        name = rust_name,
        raw = raw
    );
}

/// Renders `Deserialize`/`Serialize` for a struct with a residual map the
/// specification declares: known keys go to their fields, all remaining
/// keys are collected into the residual map (sub-aggregations inside a
/// bucket, declared `additionalProperties`).
///
/// A hand-written visitor rather than `#[serde(flatten)]`: flatten buffers
/// the object into serde's internal `Content`, which cannot deserialize a
/// `Box<RawValue>` (`_source`, raw unions, the residual itself) and fails
/// with "invalid type: newtype struct". The visitor reads each value
/// directly from the input, with no buffering.
fn render_residual_impls(
    out: &mut String,
    rust_name: &str,
    fields: &[Field<'_>],
    residual_name: &str,
    residual_value: &str,
) {
    let _ = writeln!(
        out,
        r#"impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        struct ResidualVisitor;
        impl<'de> serde::de::Visitor<'de> for ResidualVisitor {{
            type Value = {name};
            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                f.write_str("a {name} object")
            }}
            // locals are named after the field identifiers, which keep a
            // leading underscore when two JSON names would otherwise clash
            // (`_nodes` -> `f__nodes`)
            #[allow(non_snake_case)]
            fn visit_map<A>(self, mut map: A) -> std::result::Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {{"#,
        name = rust_name
    );
    for f in fields {
        let var = f.ident.trim_start_matches("r#");
        let _ = writeln!(
            out,
            "                let mut f_{}: Option<{}> = None;",
            var, f.payload
        );
        // optional fields can legitimately decode to None, so presence is
        // tracked separately to reject a repeated key like derived impls do
        if f.optional {
            let _ = writeln!(out, "                let mut seen_{} = false;", var);
        }
    }
    let _ = writeln!(
        out,
        "                let mut residual: std::collections::BTreeMap<String, {}> = Default::default();",
        residual_value
    );
    out.push_str(
        "                while let Some(k) = map.next_key::<String>()? {\n                    match k.as_str() {\n",
    );
    for f in fields {
        let var = f.ident.trim_start_matches("r#");
        let (present, assign) = if f.optional {
            (
                format!("seen_{}", var),
                format!(
                    "seen_{v} = true;\n                            f_{v} = map.next_value::<Option<{t}>>()?;",
                    v = var,
                    t = f.payload
                ),
            )
        } else {
            (
                format!("f_{}.is_some()", var),
                format!("f_{} = Some(map.next_value::<{}>()?);", var, f.payload),
            )
        };
        let _ = writeln!(
            out,
            "                        {json:?} => {{\n                            if {present} {{\n                                return Err(serde::de::Error::duplicate_field({json:?}));\n                            }}\n                            {assign}\n                        }}",
            json = f.ir.json_name,
            present = present,
            assign = assign
        );
    }
    // a repeated residual key keeps its last value, as a map would
    out.push_str(
        "                        _ => {\n                            residual.insert(k, map.next_value()?);\n                        }\n                    }\n                }\n",
    );
    let _ = writeln!(out, "                Ok({} {{", rust_name);
    for f in fields {
        let var = f.ident.trim_start_matches("r#");
        if f.optional {
            let _ = writeln!(out, "                    {}: f_{},", f.ident, var);
        } else {
            let _ = writeln!(
                out,
                "                    {}: f_{}.ok_or_else(|| serde::de::Error::missing_field({:?}))?,",
                f.ident, var, f.ir.json_name
            );
        }
    }
    let _ = writeln!(out, "                    {}: residual,", residual_name);
    out.push_str(
        "                })\n            }\n        }\n        deserializer.deserialize_map(ResidualVisitor)\n    }\n}\n",
    );

    // Serialize: fixed fields first, then the residual entries
    let _ = writeln!(
        out,
        r#"impl serde::Serialize for {name} {{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;"#,
        name = rust_name
    );
    for f in fields {
        if f.optional {
            let _ = writeln!(
                out,
                "        if let Some(v) = &self.{} {{\n            map.serialize_entry({:?}, v)?;\n        }}",
                f.ident, f.ir.json_name
            );
        } else {
            let _ = writeln!(
                out,
                "        map.serialize_entry({:?}, &self.{})?;",
                f.ir.json_name, f.ident
            );
        }
    }
    let _ = writeln!(
        out,
        "        for (k, v) in &self.{} {{\n            map.serialize_entry(k, v)?;\n        }}\n        map.end()\n    }}\n}}",
        residual_name
    );
}

/// Renders an open string enum.
///
/// Values the specification lists map to unit variants; the other spellings
/// it lists for the same value (`async` next to `ASYNC`) are accepted as
/// aliases. Anything else is kept verbatim in `Unknown(String)`, so values
/// added by newer versions or plugins never fail a response and round-trip
/// unchanged. `#[serde(other)]` cannot carry the string, hence the
/// generated `Serialize` / `Deserialize` impls.
fn render_str_enum(rust_name: &str, e: &IrStrEnum) -> String {
    let mut out = String::new();
    doc(&mut out, e.description.as_deref());
    out.push_str("#[derive(Debug, Clone, PartialEq, Eq, Hash)]\n");
    let _ = writeln!(out, "pub enum {} {{", rust_name);

    // (variant, serialized spelling, accepted spellings)
    let mut variants: Vec<(String, &str, Vec<&str>)> = Vec::new();
    // `Unknown` is reserved for the catch-all variant below
    let mut taken: BTreeSet<String> = ["Unknown".to_string()].iter().cloned().collect();
    // a spelling listed twice (in different groups) matches its first variant
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for v in &e.values {
        let accepted: Vec<&str> = std::iter::once(v.value.as_str())
            .chain(v.aliases.iter().map(String::as_str))
            .filter(|s| seen.insert(s))
            .collect();
        if accepted.is_empty() {
            continue;
        }
        let mut variant = v.value.to_pascal_case();
        if variant.is_empty() || !taken.insert(variant.clone()) {
            variant = format!("Value{}", taken.len());
            taken.insert(variant.clone());
        }
        let doc_line = if accepted.len() > 1 {
            format!(
                "`{}` (also accepted: {})",
                v.value,
                accepted[1..]
                    .iter()
                    .map(|a| format!("`{}`", a))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        } else {
            format!("`{}`", v.value)
        };
        let _ = writeln!(out, "    #[doc = {:?}]", doc_line);
        let _ = writeln!(out, "    {},", variant);
        variants.push((variant, &v.value, accepted));
    }
    out.push_str(
        "    #[doc = \"A value not defined by the specification, kept verbatim so that it round-trips.\"]\n",
    );
    out.push_str("    Unknown(String),\n}\n");

    // as_str
    let _ = writeln!(out, "impl {} {{", rust_name);
    out.push_str("    #[doc = \"The JSON string of the value\"]\n");
    out.push_str("    pub fn as_str(&self) -> &str {\n        match self {\n");
    for (variant, value, _) in &variants {
        let _ = writeln!(
            out,
            "            {}::{} => {:?},",
            rust_name, variant, value
        );
    }
    let _ = writeln!(out, "            {}::Unknown(s) => s.as_str(),", rust_name);
    out.push_str("        }\n    }\n}\n");

    // Serialize: the canonical spelling, or the original string
    let _ = writeln!(
        out,
        r#"impl serde::Serialize for {name} {{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {{
        serializer.serialize_str(self.as_str())
    }}
}}"#,
        name = rust_name
    );

    // Deserialize: exact match on every listed spelling, else Unknown
    let mut arms = String::new();
    for (variant, _, accepted) in &variants {
        let pattern = accepted
            .iter()
            .map(|a| format!("{:?}", a))
            .collect::<Vec<_>>()
            .join(" | ");
        let _ = writeln!(
            arms,
            "            {} => {}::{},",
            pattern, rust_name, variant
        );
    }
    let _ = writeln!(
        out,
        r#"impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        let s = <String as serde::Deserialize>::deserialize(deserializer)?;
        Ok(match s.as_str() {{
{arms}            _ => {name}::Unknown(s),
        }})
    }}
}}"#,
        name = rust_name,
        arms = arms
    );
    out
}

/// Renders a dispatch union: an enum whose `Deserialize` impl routes on
/// the JSON kind of the incoming value, so no branch is decoded by trial
/// and error.
///
/// The kind is read from the first byte of the raw JSON text rather than
/// from the visitor method `deserialize_any` would call: with serde_json's
/// `arbitrary_precision` feature, which Cargo feature unification can turn
/// on from any crate in a user's dependency graph, `deserialize_any`
/// reports every number as a map (`{"$serde_json::private::Number": ..}`),
/// which a kind-based visitor rejects. The raw text is captured as a
/// `RawValue`, unaffected by that feature, and the selected branch is then
/// decoded from it through its own type-driven `Deserialize`.
fn render_dispatch_union(
    ir_name: &str,
    rust_name: &str,
    u: &IrUnion,
    boxed: &BTreeSet<(String, String)>,
) -> String {
    let mut out = String::new();
    doc(&mut out, u.description.as_deref());
    out.push_str("#[derive(Debug, Clone, serde::Serialize)]\n#[serde(untagged)]\n");
    let _ = writeln!(out, "pub enum {} {{", rust_name);
    let mut variant_idents = Vec::new();
    for v in &u.variants {
        let ident = v.name.to_pascal_case();
        let mut payload = render_ref(&v.ty);
        if let IrRef::Named(target) = &v.ty {
            if boxed.contains(&(ir_name.to_string(), target.clone())) {
                payload = format!("Box<{}>", payload);
            }
        }
        let _ = writeln!(out, "    {}({}),", ident, payload);
        variant_idents.push((ident, payload));
    }
    if u.nullable {
        out.push_str("    #[doc = \"An explicit null value.\"]\n    Null,\n}\n");
    } else {
        out.push_str("}\n");
    }

    // one arm per variant kind, plus `null` for nullable unions
    let mut arms = String::new();
    for (v, (ident, _)) in u.variants.iter().zip(&variant_idents) {
        let pattern = match v.kind {
            Some(JsonKind::Bool) => "Some(b't') | Some(b'f')",
            Some(JsonKind::Number) => "Some(b'-') | Some(b'0'..=b'9')",
            Some(JsonKind::String) => "Some(b'\"')",
            Some(JsonKind::Array) => "Some(b'[')",
            Some(JsonKind::Object) => "Some(b'{')",
            None => continue,
        };
        let _ = writeln!(
            arms,
            "            {} => serde_json::from_str(text).map({}::{}),",
            pattern, rust_name, ident
        );
    }
    if u.nullable {
        let _ = writeln!(
            arms,
            "            Some(b'n') => return Ok({}::Null),",
            rust_name
        );
    }

    let _ = writeln!(
        out,
        r#"impl<'de> serde::Deserialize<'de> for {name} {{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {{
        // dispatch on the first byte of the raw JSON text: unlike the value
        // kind reported by `deserialize_any`, it does not depend on serde_json's
        // `arbitrary_precision` feature
        let raw: Box<serde_json::value::RawValue> = serde::Deserialize::deserialize(deserializer)?;
        let text = raw.get().trim_start();
        let parsed = match text.as_bytes().first() {{
{arms}            other => {{
                // the type name is an argument, not part of the literal, so
                // that rustfmt can wrap the line for long names
                return Err(serde::de::Error::custom(format_args!(
                    "invalid type: {{}}, expected a {{}} value",
                    crate::types::union::json_kind_name(other.copied()),
                    "{name}"
                )))
            }}
        }};
        parsed.map_err(serde::de::Error::custom)
    }}
}}"#,
        name = rust_name,
        arms = arms
    );
    out
}

/// Renders a lazily decoded union: the raw JSON is captured unparsed and
/// decoded only when a branch accessor is called
fn render_lazy_union(rust_name: &str, u: &IrUnion) -> String {
    let mut out = String::new();
    doc(&mut out, u.description.as_deref());
    let _ = writeln!(
        out,
        r#"#[doc = "The branches of this union cannot be told apart by their JSON shape alone, so the value is captured unparsed: call the `as_...` accessor matching what the request asked for, and bind the parsed result instead of calling the accessor repeatedly. The raw JSON always remains available through [`raw`](Self::raw)."]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct {name} {{
    raw: Box<serde_json::value::RawValue>,
}}
impl {name} {{
    #[doc = "The raw JSON of the value"]
    pub fn raw(&self) -> &serde_json::value::RawValue {{
        &self.raw
    }}
    #[doc = "Whether the value is an explicit null"]
    pub fn is_null(&self) -> bool {{
        self.raw.get() == "null"
    }}"#,
        name = rust_name
    );
    let mut taken = BTreeSet::new();
    for v in &u.variants {
        let mut accessor = v.name.to_snake_case();
        if accessor.is_empty() || !taken.insert(accessor.clone()) {
            continue;
        }
        if KEYWORDS.contains(&accessor.as_str()) {
            accessor = format!("{}_", accessor);
        }
        let payload = render_ref(&v.ty);
        let _ = writeln!(
            out,
            r#"    #[doc = "Parses the value as the `{branch}` branch. Fails when the value is a different branch: the raw JSON stays accessible through [`raw`](Self::raw)."]
    pub fn as_{accessor}(&self) -> std::result::Result<{payload}, crate::types::UnionBranchError> {{
        crate::types::union::parse_branch(&self.raw, "{branch}")
    }}"#,
            branch = v.name,
            accessor = accessor,
            payload = payload
        );
    }
    out.push_str("}\n");
    out
}

/// Computes the `(container, contained)` edges that must be boxed to keep
/// recursive types finite: an edge is boxed when the contained type can
/// reach the container back through direct (non-indirected) containment
fn boxed_edges(ir: &ResponseIr) -> BTreeSet<(String, String)> {
    // direct containment edges: a contained value stored inline (fields
    // and dispatch variants); Vec / BTreeMap / lazy raw already indirect
    let mut edges: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (name, ty) in &ir.types {
        let mut targets = BTreeSet::new();
        match ty {
            IrType::Struct(s) => {
                for f in &s.fields {
                    if let IrRef::Named(n) = &f.ty {
                        targets.insert(n.as_str());
                    }
                }
            }
            IrType::DispatchUnion(u) => {
                for v in &u.variants {
                    if let IrRef::Named(n) = &v.ty {
                        targets.insert(n.as_str());
                    }
                }
            }
            IrType::Alias(a) => {
                if let IrRef::Named(n) = &a.ty {
                    targets.insert(n.as_str());
                }
            }
            IrType::LazyUnion(_) | IrType::StrEnum(_) => {}
        }
        edges.insert(name.as_str(), targets);
    }

    fn reaches(
        from: &str,
        to: &str,
        edges: &BTreeMap<&str, BTreeSet<&str>>,
        visited: &mut BTreeSet<String>,
    ) -> bool {
        if from == to {
            return true;
        }
        if !visited.insert(from.to_string()) {
            return false;
        }
        edges
            .get(from)
            .map(|targets| targets.iter().any(|t| reaches(t, to, edges, visited)))
            .unwrap_or(false)
    }

    let mut boxed = BTreeSet::new();
    for (container, targets) in &edges {
        for target in targets {
            let mut visited = BTreeSet::new();
            if reaches(target, container, &edges, &mut visited) {
                boxed.insert((container.to_string(), target.to_string()));
            }
        }
    }
    boxed
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::responses::{all_operation_groups, read_response_ir};

    fn spec_ir() -> ResponseIr {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let spec: serde_json::Value =
            serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        let groups = all_operation_groups(&spec);
        let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
        read_response_ir(&spec, &refs).unwrap()
    }

    #[test]
    fn total_hit_count_unions_are_found_in_every_search_like_response() {
        let found = total_hits_unions(&spec_ir());
        // one per kind of hits object: the specification declares
        // `hits.total` inline in each
        for name in [
            "_core.search___HitsMetadataTotal",
            "_core.search___SearchResponseHitsTotal",
            "_core.msearch___MultiSearchItemHitsTotal",
            "_core.search_template___SearchTemplateResponseHitsTotal",
        ] {
            assert!(found.contains(name), "{} not found in {:?}", name, found);
        }
        assert_eq!(found.len(), 10, "{:?}", found);
    }

    #[test]
    fn string_enum_aliases_are_read_and_the_first_spelling_written() {
        use crate::openapi::responses::{IrStrEnum, IrStrValue};
        let e = IrStrEnum {
            description: None,
            values: vec![
                IrStrValue {
                    value: "ASYNC".into(),
                    aliases: vec!["async".into()],
                },
                IrStrValue {
                    value: "request".into(),
                    aliases: vec![],
                },
            ],
        };
        let out = render_str_enum("TranslogDurability", &e);
        // every listed spelling is read, matching exactly
        assert!(
            out.contains(r#""ASYNC" | "async" => TranslogDurability::Async"#),
            "{}",
            out
        );
        assert!(
            out.contains(r#""request" => TranslogDurability::Request"#),
            "{}",
            out
        );
        // the first spelling is written back; unknown values are kept
        assert!(
            out.contains(r#"TranslogDurability::Async => "ASYNC""#),
            "{}",
            out
        );
        assert!(out.contains("Unknown(String)"), "{}", out);
        assert!(out.contains("(also accepted: `async`)"), "{}", out);
    }

    #[test]
    fn only_the_total_hits_or_integer_shape_is_a_total_hit_count() {
        use crate::openapi::responses::IrVariant;
        let union = |variants: Vec<IrRef>, nullable: bool| IrUnion {
            description: None,
            variants: variants
                .into_iter()
                .enumerate()
                .map(|(i, ty)| IrVariant {
                    name: format!("v{}", i),
                    ty,
                    kind: None,
                })
                .collect(),
            nullable,
        };
        let object = || IrRef::Named(TOTAL_HITS.to_string());
        let integer = || IrRef::Scalar(Scalar::I64);

        assert!(is_total_hits_union(&union(
            vec![object(), integer()],
            false
        )));
        assert!(is_total_hits_union(&union(
            vec![integer(), object()],
            false
        )));
        // `null` has no count
        assert!(!is_total_hits_union(&union(
            vec![object(), integer()],
            true
        )));
        // another object, or another number type
        assert!(!is_total_hits_union(&union(
            vec![IrRef::Named("_common___Other".into()), integer()],
            false
        )));
        assert!(!is_total_hits_union(&union(
            vec![object(), IrRef::Scalar(Scalar::F64)],
            false
        )));
        assert!(!is_total_hits_union(&union(
            vec![object(), integer(), IrRef::Scalar(Scalar::String)],
            false
        )));
    }
}
