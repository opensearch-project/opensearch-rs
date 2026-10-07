/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The untyped-field ledger of the generated response models.
//!
//! 100% typing is a non-goal: shapes decided at runtime (documents,
//! generic type parameters, untyped `additionalProperties`, schemas
//! without a type) are held as raw JSON ([IrRef::Raw]). Each such
//! degradation is however a deliberate, reviewed decision: this module
//! collects every place the IR falls back to raw JSON and checks it
//! against a checked-in ledger file, so that
//!
//! - a specification change silently turning a typed field into raw JSON
//!   fails generation instead of quietly shipping a weaker model, and
//! - every entry carries a human review reason: an entry without one fails
//!   generation too, so a ledger cannot be bootstrapped by blindly
//!   snapshotting whatever the generator happens to emit.
//!
//! # File format
//!
//! One line per entry, grouped under a `# --- namespace ---` banner:
//!
//! ```text
//! namespace:TypeName/field # <form> <cause> -- <review reason>
//! ```
//!
//! The key (first token) identifies the entry. The form (`bare`,
//! `map<bare>`, ...) and the cause ([RawCause] label) are computed by the
//! generator and rewritten on every update. The review reason after ` -- `
//! is written by a reviewer and preserved across updates; new entries get
//! [UNREVIEWED] as their reason.

use super::responses::{IrRef, IrType, RawCause, ResponseIr};
use anyhow::bail;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write;
use std::path::Path;

/// The ledger file name, relative to the `api_generator` directory
pub const FILE_NAME: &str = "raw_allowlist.txt";

/// The review reason placeholder of an entry nobody has reviewed yet
pub const UNREVIEWED: &str = "UNREVIEWED";

/// One raw-JSON use in the IR
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct RawUse {
    /// The IR namespace of the owning type, for banner grouping
    pub namespace: String,
    /// `namespace:TypeName/field`, with `-` as the member for a whole-type
    /// alias, `[additional]` for a residual map, the variant name for a
    /// union variant. The namespace qualifies the key: type names repeat
    /// across namespaces (`indices._common` and `ml._common` both have an
    /// `IndexSettings`)
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

/// Renders the ledger for the given uses, taking each entry's review
/// reason from `reasons` (by key), [UNREVIEWED] when absent
pub fn render(uses: &[RawUse], reasons: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    out.push_str(
        "# Untyped-field ledger of the generated response models.\n\
         # Regenerate with `cargo run -p api_generator -- --openapi <spec> --update-raw-allowlist`.\n\
         #\n\
         # Each line is a permitted raw JSON (Box<RawValue>) use in opensearch::types:\n\
         #\n\
         #   namespace:TypeName/field # <form> <cause> -- <review reason>\n\
         #\n\
         # A member \"-\" is a whole-type alias, \"[additional]\" the residual map of a\n\
         # struct. Form and cause are computed by the generator. The review reason\n\
         # after \" -- \" is written by hand and kept across updates: an entry whose reason\n\
         # is UNREVIEWED fails generation until someone confirms the degradation is\n\
         # intended (or fixes the generator / the specification instead).\n\
         #\n\
         # Causes: no-type (schema declares no type), generic (generic type parameter),\n\
         # open-object (object without fixed fields or with untyped additionalProperties),\n\
         # untyped-items (array without items), null-only (schema only allows null).\n",
    );
    let mut by_ns: BTreeMap<&str, Vec<&RawUse>> = BTreeMap::new();
    for u in uses {
        by_ns.entry(u.namespace.as_str()).or_default().push(u);
    }
    for (ns, entries) in by_ns {
        let _ = writeln!(out, "\n# --- {} ---", ns);
        for e in entries {
            let reason = reasons
                .get(&e.key)
                .map(String::as_str)
                .unwrap_or(UNREVIEWED);
            let _ = writeln!(
                out,
                "{} # {} {} -- {}",
                e.key,
                e.form,
                e.cause.label(),
                reason
            );
        }
    }
    out
}

/// Parses ledger content into `key -> review reason`. Comment lines (`#`
/// first) and blank lines are ignored; the key is the first token; the
/// reason is everything after the first ` -- ` of the comment part, and
/// [UNREVIEWED] when missing or empty.
pub fn parse(content: &str) -> BTreeMap<String, String> {
    let mut entries = BTreeMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let key = line.split_whitespace().next().unwrap().to_string();
        let reason = line
            .split_once('#')
            .and_then(|(_, comment)| comment.split_once(" -- "))
            .map(|(_, reason)| reason.trim())
            .filter(|r| !r.is_empty())
            .unwrap_or(UNREVIEWED)
            .to_string();
        entries.insert(key, reason);
    }
    entries
}

/// The result of checking the IR against a ledger
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Check {
    /// Raw uses in the IR not present in the ledger: generation must fail
    pub unlisted: Vec<String>,
    /// Ledger entries still waiting for a review reason: generation must fail
    pub unreviewed: Vec<String>,
    /// Ledger entries nothing emits any more: advisory only
    pub stale: Vec<String>,
}

/// Compares the raw uses of the IR against the ledger entries
pub fn check(uses: &[RawUse], ledger: &BTreeMap<String, String>) -> Check {
    let emitted: BTreeSet<&str> = uses.iter().map(|u| u.key.as_str()).collect();
    Check {
        unlisted: emitted
            .iter()
            .filter(|k| !ledger.contains_key(**k))
            .map(|k| k.to_string())
            .collect(),
        unreviewed: ledger
            .iter()
            .filter(|(k, r)| emitted.contains(k.as_str()) && r.as_str() == UNREVIEWED)
            .map(|(k, _)| k.clone())
            .collect(),
        stale: ledger
            .keys()
            .filter(|k| !emitted.contains(k.as_str()))
            .cloned()
            .collect(),
    }
}

/// Enforces the ledger.
///
/// With `update`, the file is rewritten from the IR: entries are added and
/// removed, form and cause recomputed, and existing review reasons kept.
/// Otherwise the IR is checked against it: an unlisted raw use or an
/// unreviewed entry is an error. Stale entries are reported on stderr.
pub fn enforce(ir: &ResponseIr, path: &Path, update: bool) -> anyhow::Result<()> {
    let uses = collect_raw_uses(ir);
    let existing = match std::fs::read_to_string(path) {
        Ok(content) => Some(parse(&content)),
        Err(e) if update && e.kind() == std::io::ErrorKind::NotFound => None,
        Err(e) => bail!(
            "cannot read raw allowlist {}: {} (run with --update-raw-allowlist to create it)",
            path.display(),
            e
        ),
    };

    if update {
        let reasons = existing.unwrap_or_default();
        std::fs::write(path, render(&uses, &reasons))?;
        let unreviewed = uses
            .iter()
            .filter(|u| reasons.get(&u.key).map_or(true, |r| r == UNREVIEWED))
            .count();
        eprintln!(
            "raw allowlist: wrote {} entries to {} ({} unreviewed)",
            uses.len(),
            path.display(),
            unreviewed
        );
        return Ok(());
    }

    let result = check(&uses, &existing.unwrap_or_default());
    for k in &result.stale {
        eprintln!(
            "raw allowlist: stale entry `{}` (nothing emits it any more)",
            k
        );
    }
    let mut problems = Vec::new();
    if !result.unlisted.is_empty() {
        problems.push(format!(
            "{} raw JSON use(s) not in the ledger:\n  {}",
            result.unlisted.len(),
            result.unlisted.join("\n  ")
        ));
    }
    if !result.unreviewed.is_empty() {
        problems.push(format!(
            "{} ledger entr(ies) without a review reason:\n  {}",
            result.unreviewed.len(),
            result.unreviewed.join("\n  ")
        ));
    }
    if !problems.is_empty() {
        bail!(
            "raw allowlist {}: {}\n\
             Each is a field the generator could not type. Fix the generator or the \
             specification, or confirm the degradation is intended: run with \
             --update-raw-allowlist, then replace {} with the reason in the ledger.",
            path.display(),
            problems.join("\n"),
            UNREVIEWED
        );
    }
    Ok(())
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

    fn raw_use(ns: &str, key: &str) -> RawUse {
        RawUse {
            namespace: ns.into(),
            key: key.into(),
            form: "bare".into(),
            cause: RawCause::NoType,
        }
    }

    fn ledger(entries: &[(&str, &str)]) -> BTreeMap<String, String> {
        entries
            .iter()
            .map(|(k, r)| (k.to_string(), r.to_string()))
            .collect()
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
    fn render_and_parse_round_trip_with_reasons() {
        let uses = vec![
            raw_use("_common", "Metadata/-"),
            raw_use("_core.search", "Hit/_source"),
        ];
        let text = render(
            &uses,
            &ledger(&[("Hit/_source", "documents are user-defined")]),
        );
        assert!(text.contains("# --- _common ---"));
        assert!(text.contains("Hit/_source # bare no-type -- documents are user-defined"));
        assert!(text.contains(&format!("Metadata/- # bare no-type -- {}", UNREVIEWED)));
        let parsed = parse(&text);
        assert_eq!(
            parsed,
            ledger(&[
                ("Hit/_source", "documents are user-defined"),
                ("Metadata/-", UNREVIEWED),
            ])
        );
    }

    #[test]
    fn parse_tolerates_missing_or_empty_reasons() {
        let parsed =
            parse("A/x # bare no-type\nB/y # bare no-type -- \nC/z\n# comment -- not an entry\n");
        assert_eq!(
            parsed,
            ledger(&[
                ("A/x", UNREVIEWED),
                ("B/y", UNREVIEWED),
                ("C/z", UNREVIEWED)
            ])
        );
    }

    #[test]
    fn check_reports_unlisted_unreviewed_and_stale() {
        let uses = vec![raw_use("ns", "A/x"), raw_use("ns", "B/y")];
        assert_eq!(
            check(&uses, &ledger(&[("B/y", UNREVIEWED), ("C/z", "ok")])),
            Check {
                unlisted: vec!["A/x".into()],
                unreviewed: vec!["B/y".into()],
                stale: vec!["C/z".into()],
            }
        );
        assert_eq!(
            check(&uses, &ledger(&[("A/x", "ok"), ("B/y", "ok")])),
            Check::default()
        );
    }

    #[test]
    fn keys_are_unique_across_the_vendored_spec() {
        // type names repeat across namespaces: keys must not collide, or
        // one entry's review reason would silently cover another
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        let spec: serde_json::Value =
            serde_yaml::from_reader(std::fs::File::open(path).unwrap()).unwrap();
        let groups = crate::openapi::responses::all_operation_groups(&spec);
        let refs: Vec<&str> = groups.iter().map(String::as_str).collect();
        let ir = crate::openapi::responses::read_response_ir(&spec, &refs).unwrap();
        let uses = collect_raw_uses(&ir);
        let keys: BTreeSet<&str> = uses.iter().map(|u| u.key.as_str()).collect();
        assert_eq!(keys.len(), uses.len(), "duplicate ledger keys");
    }

    #[test]
    fn update_keeps_review_reasons() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(FILE_NAME);
        let ir = ir_with(vec![(
            "ns___T",
            IrType::Struct(IrStruct {
                description: None,
                fields: vec![
                    field("a", IrRef::Raw(RawCause::NoType)),
                    field("b", IrRef::Raw(RawCause::OpenObject)),
                ],
                residual: None,
            }),
        )]);
        // first update: everything unreviewed, and checking fails
        enforce(&ir, &path, true).unwrap();
        let err = enforce(&ir, &path, false).unwrap_err().to_string();
        assert!(err.contains("without a review reason"), "{}", err);

        // a reviewer fills in a reason for T/a only
        let content = std::fs::read_to_string(&path).unwrap().replace(
            &format!("ns:T/a # bare no-type -- {}", UNREVIEWED),
            "ns:T/a # bare no-type -- reviewed",
        );
        std::fs::write(&path, content).unwrap();
        // the update keeps it
        enforce(&ir, &path, true).unwrap();
        let parsed = parse(&std::fs::read_to_string(&path).unwrap());
        assert_eq!(parsed["ns:T/a"], "reviewed");
        assert_eq!(parsed["ns:T/b"], UNREVIEWED);
    }
}
