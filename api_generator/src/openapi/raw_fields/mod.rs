/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The fields of the generated response models that are left as raw JSON.
//!
//! 100% typing is a non-goal: shapes decided at runtime (documents,
//! generic type parameters, untyped `additionalProperties`, schemas
//! without a type) are held as raw JSON ([IrRef::Raw]). Each of them is
//! however a deliberate, reviewed decision, declared in [declared::RAW_FIELDS]
//! with the reason it cannot be typed. Generation checks the generated
//! models against that table and fails when
//!
//! - a field is generated as raw JSON but not declared: a specification
//!   change silently turning a typed field into raw JSON fails generation
//!   instead of quietly shipping a weaker model,
//! - a declaration has no review reason ([UNREVIEWED]): the table cannot be
//!   filled by pasting whatever the generator emits without a review,
//! - a declaration does not match a raw field any more (stale), or its
//!   cause differs from the generated one, or the same key is declared twice.
//!
//! On failure, the generator prints the declarations to add, ready to paste.

mod declared;

use super::responses::{IrRef, IrType, RawCause, ResponseIr};
use anyhow::bail;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;

pub use declared::RAW_FIELDS;

/// The review reason of a declaration nobody has reviewed yet. Generation
/// fails while a declaration has it.
pub const UNREVIEWED: &str = "UNREVIEWED";

/// A field of the generated response models declared as raw JSON
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawField {
    /// `namespace:TypeName/member`: the IR namespace and type name of the
    /// owning type, and the JSON name of the field, `-` for a whole-type
    /// alias, `[additional]` for a residual map, or the variant name of a
    /// union variant. The namespace qualifies the key, as type names repeat
    /// across namespaces (`indices._common` and `ml._common` both have an
    /// `IndexSettings`)
    pub key: &'static str,
    /// Why the generator falls back to raw JSON. Checked against the
    /// generated models, so that a change of cause is reviewed again
    pub cause: RawCause,
    /// Why the field cannot, or should not, be typed
    pub reason: &'static str,
}

/// Declares a raw field: see [RawField]
pub const fn raw(key: &'static str, cause: RawCause, reason: &'static str) -> RawField {
    RawField { key, cause, reason }
}

/// One raw-JSON use in the generated models
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawUse {
    /// The IR namespace of the owning type
    pub namespace: String,
    /// The key of the use: see [RawField::key]
    pub key: String,
    /// The shape the raw value sits in: `bare`, `list<bare>`, `map<bare>`, ...
    pub form: String,
    /// The schema construct that caused the fallback
    pub cause: RawCause,
}

/// Collects every raw-JSON use reachable in the IR, sorted by namespace
/// then key
pub fn collect_raw_uses(ir: &ResponseIr) -> Vec<RawUse> {
    let mut uses = BTreeSet::new();
    for (ir_name, ty) in &ir.types {
        let namespace = super::responses::namespace(ir_name).to_string();
        let type_name = super::responses::type_name(ir_name);
        let mut push = |member: &str, raw: Option<(String, RawCause)>| {
            if let Some((form, cause)) = raw {
                uses.insert(RawUse {
                    namespace: namespace.clone(),
                    key: format!("{}:{}/{}", namespace, type_name, member),
                    form,
                    cause,
                });
            }
        };
        match ty {
            IrType::Struct(s) => {
                for f in &s.fields {
                    push(&f.json_name, raw_form(&f.ty));
                }
                if let Some((_, t)) = &s.residual {
                    push("[additional]", raw_form(t));
                }
            }
            IrType::DispatchUnion(u) | IrType::LazyUnion(u) => {
                for v in &u.variants {
                    push(&v.name, raw_form(&v.ty));
                }
            }
            IrType::Alias(a) => push("-", raw_form(&a.ty)),
            IrType::StrEnum(_) => {}
        }
    }
    uses.into_iter().collect()
}

/// The form and cause of a raw use inside a reference, or `None` when the
/// reference holds no raw JSON
fn raw_form(ty: &IrRef) -> Option<(String, RawCause)> {
    match ty {
        IrRef::Raw(cause) => Some(("bare".to_string(), *cause)),
        IrRef::List(inner) => raw_form(inner).map(|(f, c)| (format!("list<{}>", f), c)),
        IrRef::Map(inner) => raw_form(inner).map(|(f, c)| (format!("map<{}>", f), c)),
        IrRef::Named(_) | IrRef::Scalar(_) => None,
    }
}

/// The result of checking the generated models against the declarations
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Check {
    /// Raw uses that are not declared
    pub undeclared: Vec<RawUse>,
    /// Declarations without a review reason
    pub unreviewed: Vec<&'static str>,
    /// Declarations matching no raw use
    pub stale: Vec<&'static str>,
    /// Declarations whose cause differs from the generated one:
    /// `(key, declared, generated)`
    pub cause_mismatch: Vec<(&'static str, RawCause, RawCause)>,
    /// Keys declared more than once
    pub duplicate: Vec<&'static str>,
}

impl Check {
    /// Whether the generated models match the declarations
    pub fn is_ok(&self) -> bool {
        *self == Check::default()
    }
}

/// Compares the raw uses of the generated models with the declarations
pub fn check(uses: &[RawUse], declared: &[RawField]) -> Check {
    let mut by_key: BTreeMap<&str, &RawField> = BTreeMap::new();
    let mut result = Check::default();
    for d in declared {
        if by_key.insert(d.key, d).is_some() && !result.duplicate.contains(&d.key) {
            result.duplicate.push(d.key);
        }
    }
    let emitted: BTreeMap<&str, &RawUse> = uses.iter().map(|u| (u.key.as_str(), u)).collect();

    for u in uses {
        match by_key.get(u.key.as_str()) {
            None => result.undeclared.push(u.clone()),
            Some(d) if d.cause != u.cause => result.cause_mismatch.push((d.key, d.cause, u.cause)),
            Some(_) => {}
        }
    }
    for (key, d) in &by_key {
        if !emitted.contains_key(key) {
            result.stale.push(d.key);
        } else if d.reason.trim().is_empty() || d.reason == UNREVIEWED {
            result.unreviewed.push(d.key);
        }
    }
    result
}

/// The Rust name of a cause, as written in the declarations
fn cause_variant(cause: RawCause) -> &'static str {
    match cause {
        RawCause::NoType => "NoType",
        RawCause::GenericParameter => "GenericParameter",
        RawCause::OpenObject => "OpenObject",
        RawCause::UntypedItems => "UntypedItems",
        RawCause::NullOnly => "NullOnly",
    }
}

/// The declaration of a raw use, ready to paste into [declared::RAW_FIELDS]
pub fn declaration(u: &RawUse) -> String {
    format!(
        "    // {} {}\n    raw(\"{}\", {}, UNREVIEWED),",
        u.form,
        u.cause.label(),
        u.key,
        cause_variant(u.cause)
    )
}

/// Checks the generated models against [RAW_FIELDS], failing generation
/// with a description of every problem and the declarations to add
pub fn enforce(ir: &ResponseIr) -> anyhow::Result<()> {
    enforce_with(ir, RAW_FIELDS)
}

fn enforce_with(ir: &ResponseIr, declared: &[RawField]) -> anyhow::Result<()> {
    let result = check(&collect_raw_uses(ir), declared);
    if result.is_ok() {
        return Ok(());
    }

    let mut out = String::new();
    if !result.undeclared.is_empty() {
        let _ = writeln!(
            out,
            "{} field(s) generated as raw JSON are not declared. If the degradation is \
             intended, add these declarations and replace UNREVIEWED with the reason; \
             otherwise fix the generator or the specification:",
            result.undeclared.len()
        );
        for u in &result.undeclared {
            let _ = writeln!(out, "{}", declaration(u));
        }
    }
    if !result.unreviewed.is_empty() {
        let _ = writeln!(
            out,
            "{} declaration(s) without a review reason:\n  {}",
            result.unreviewed.len(),
            result.unreviewed.join("\n  ")
        );
    }
    if !result.stale.is_empty() {
        let _ = writeln!(
            out,
            "{} declaration(s) match no raw field any more, remove them:\n  {}",
            result.stale.len(),
            result.stale.join("\n  ")
        );
    }
    if !result.cause_mismatch.is_empty() {
        let _ = writeln!(
            out,
            "{} declaration(s) have a different cause than the generated field, review them again:",
            result.cause_mismatch.len()
        );
        for (key, declared, generated) in &result.cause_mismatch {
            let _ = writeln!(
                out,
                "  {}: declared {}, generated {}",
                key,
                cause_variant(*declared),
                cause_variant(*generated)
            );
        }
    }
    if !result.duplicate.is_empty() {
        let _ = writeln!(
            out,
            "{} key(s) declared more than once:\n  {}",
            result.duplicate.len(),
            result.duplicate.join("\n  ")
        );
    }
    bail!(
        "raw fields of the response models (api_generator/src/openapi/raw_fields/declared.rs):\n{}",
        out.trim_end()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::openapi::responses::{IrAlias, IrField, IrStruct, Scalar};

    fn ir_with(types: Vec<(&str, IrType)>) -> ResponseIr {
        let mut ir = ResponseIr::default();
        for (n, t) in types {
            ir.types.insert(n.to_string(), t);
        }
        ir
    }

    fn field(name: &str, ty: IrRef) -> IrField {
        IrField {
            json_name: name.to_string(),
            ty,
            required: false,
            nullable: false,
            description: None,
        }
    }

    fn raw_use(key: &str, cause: RawCause) -> RawUse {
        RawUse {
            namespace: key.split(':').next().unwrap().into(),
            key: key.into(),
            form: "bare".into(),
            cause,
        }
    }

    #[test]
    fn collects_raw_uses_with_their_form_and_cause() {
        let ir = ir_with(vec![
            (
                "_core.search___Hit",
                IrType::Struct(IrStruct {
                    description: None,
                    fields: vec![
                        field("_source", IrRef::Raw(RawCause::GenericParameter)),
                        field("_id", IrRef::Scalar(Scalar::String)),
                        field(
                            "fields",
                            IrRef::Map(Box::new(IrRef::Raw(RawCause::OpenObject))),
                        ),
                        field("sort", IrRef::List(Box::new(IrRef::Raw(RawCause::NoType)))),
                    ],
                    residual: Some(("additional".to_string(), IrRef::Raw(RawCause::OpenObject))),
                }),
            ),
            (
                "_common___Metadata",
                IrType::Alias(IrAlias {
                    description: None,
                    ty: IrRef::Map(Box::new(IrRef::Raw(RawCause::OpenObject))),
                }),
            ),
        ]);
        let uses = collect_raw_uses(&ir);
        let got: Vec<(&str, &str, &str)> = uses
            .iter()
            .map(|u| (u.key.as_str(), u.form.as_str(), u.cause.label()))
            .collect();
        // sorted by namespace, then key
        assert_eq!(
            got,
            vec![
                ("_common:Metadata/-", "map<bare>", "open-object"),
                ("_core.search:Hit/[additional]", "bare", "open-object"),
                ("_core.search:Hit/_source", "bare", "generic"),
                ("_core.search:Hit/fields", "map<bare>", "open-object"),
                ("_core.search:Hit/sort", "list<bare>", "no-type"),
            ]
        );
    }

    #[test]
    fn check_reports_every_kind_of_mismatch() {
        let uses = vec![
            raw_use("ns:A/x", RawCause::NoType),
            raw_use("ns:B/y", RawCause::NoType),
            raw_use("ns:D/w", RawCause::OpenObject),
            raw_use("ns:E/v", RawCause::NoType),
        ];
        let declared = [
            raw("ns:B/y", RawCause::NoType, UNREVIEWED),
            raw("ns:C/z", RawCause::NoType, "nothing emits it"),
            raw("ns:D/w", RawCause::NoType, "the cause changed"),
            raw("ns:E/v", RawCause::NoType, "reviewed"),
            raw("ns:E/v", RawCause::NoType, "reviewed twice"),
        ];
        let result = check(&uses, &declared);
        assert_eq!(result.undeclared, vec![raw_use("ns:A/x", RawCause::NoType)]);
        assert_eq!(result.unreviewed, vec!["ns:B/y"]);
        assert_eq!(result.stale, vec!["ns:C/z"]);
        assert_eq!(
            result.cause_mismatch,
            vec![("ns:D/w", RawCause::NoType, RawCause::OpenObject)]
        );
        assert_eq!(result.duplicate, vec!["ns:E/v"]);
        assert!(!result.is_ok());

        let declared = [
            raw("ns:A/x", RawCause::NoType, "a"),
            raw("ns:B/y", RawCause::NoType, "b"),
            raw("ns:D/w", RawCause::OpenObject, "d"),
            raw("ns:E/v", RawCause::NoType, "e"),
        ];
        assert!(check(&uses, &declared).is_ok());
    }

    #[test]
    fn a_failure_prints_the_declarations_to_add() {
        let ir = ir_with(vec![(
            "ns___T",
            IrType::Struct(IrStruct {
                description: None,
                fields: vec![field(
                    "a",
                    IrRef::Map(Box::new(IrRef::Raw(RawCause::OpenObject))),
                )],
                residual: None,
            }),
        )]);
        let err = enforce_with(&ir, &[]).unwrap_err().to_string();
        assert!(
            err.contains(
                "    // map<bare> open-object\n    raw(\"ns:T/a\", OpenObject, UNREVIEWED),"
            ),
            "{}",
            err
        );
        // pasting it as is still fails: a reason is required
        let err = enforce_with(&ir, &[raw("ns:T/a", RawCause::OpenObject, UNREVIEWED)])
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("without a review reason:\n  ns:T/a"),
            "{}",
            err
        );
        assert!(enforce_with(&ir, &[raw("ns:T/a", RawCause::OpenObject, "reviewed")]).is_ok());
    }

    #[test]
    fn declarations_match_the_generated_models() {
        // the same check as generation, on the vendored specification
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let spec: serde_json::Value =
            serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        let ir = crate::openapi::responses::read_response_ir(
            &spec,
            crate::openapi::responses::TYPED_OPERATION_GROUPS,
        )
        .unwrap();
        enforce(&ir).unwrap();
    }

    #[test]
    fn keys_are_unique_across_the_vendored_spec() {
        // type names repeat across namespaces: keys must not collide, or
        // one declaration would silently cover another field
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let spec: serde_json::Value =
            serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        let groups = crate::openapi::responses::all_operation_groups(&spec);
        let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
        let ir = crate::openapi::responses::read_response_ir(&spec, &refs).unwrap();
        let uses = collect_raw_uses(&ir);
        let keys: BTreeSet<&str> = uses.iter().map(|u| u.key.as_str()).collect();
        assert_eq!(keys.len(), uses.len(), "duplicate keys");
    }
}
