/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Parses an OpenAPI specification and prints a summary of the resulting
//! [Api] model, optionally comparing endpoint coverage against the legacy
//! REST API specs. Useful for triaging differences between the two
//! ingestion paths.
//!
//! Usage (from the repository root):
//!
//! ```text
//! cargo run -p api_generator --example openapi_summary -- <openapi.yaml> [rest_specs_dir]
//! ```

use api_generator::{generator, openapi};
use std::collections::BTreeSet;

fn endpoint_names(api: &generator::Api) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for name in api.root.endpoints().keys() {
        names.insert(name.clone());
    }
    for (ns, namespace) in &api.namespaces {
        for name in namespace.endpoints().keys() {
            names.insert(format!("{}.{}", ns, name));
        }
    }
    names
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let spec_file = args
        .get(1)
        .expect("usage: openapi_summary <openapi.yaml> [rest_specs_dir]");

    let api = openapi::read_api("openapi", std::path::Path::new(spec_file))?;
    let openapi_names = endpoint_names(&api);

    println!("== OpenAPI ingestion summary ==");
    println!("root endpoints:  {}", api.root.endpoints().len());
    println!("namespaces:      {}", api.namespaces.len());
    println!("total endpoints: {}", openapi_names.len());
    println!("common params:   {:?}", api.common_params.keys().collect::<Vec<_>>());
    println!("enums:           {}", api.enums.len());

    if let Some(rest_specs_dir) = args.get(2) {
        let legacy = generator::read_api("legacy", std::path::Path::new(rest_specs_dir))?;
        let legacy_names = endpoint_names(&legacy);

        println!("\n== Coverage vs legacy rest_specs ==");
        println!("legacy endpoints: {}", legacy_names.len());

        let missing: Vec<_> = legacy_names.difference(&openapi_names).collect();
        println!("\nIn legacy but NOT in OpenAPI ({}):", missing.len());
        for name in missing {
            println!("  - {}", name);
        }

        let added: Vec<_> = openapi_names.difference(&legacy_names).collect();
        println!("\nIn OpenAPI but NOT in legacy ({}):", added.len());
        for name in added {
            println!("  + {}", name);
        }
    }

    Ok(())
}
