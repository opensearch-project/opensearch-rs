/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Generates the typed client (`opensearch::typed`): for every endpoint with
//! a typed response model, a typed builder wrapping the generic one, whose
//! `send` returns the model, and the method creating it on the typed root or
//! namespace client.

use super::{
    doc, ident, request::request_builder::RequestBuilder, stability_doc, use_declarations,
};
use crate::generator::Api;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};
use std::path::Path;

/// A request option that changes the shape of the response, so that the
/// response no longer matches the typed model
struct OmittedParam {
    /// Operation groups the option is omitted from: a group name, a
    /// namespace prefix ending with `.`, or `*` for every group
    groups: &'static str,
    name: &'static str,
    /// The value the typed builder always sends, when the API needs one to
    /// answer with the model
    fixed: Option<&'static str>,
}

/// The options the typed builders do not offer. A typed builder must not be
/// able to express a request whose response cannot be decoded as its model.
const OMITTED_PARAMS: &[OmittedParam] = &[
    // keeps only the matching parts of the response, dropping fields the
    // model requires
    OmittedParam {
        groups: "*",
        name: "filter_path",
        fixed: None,
    },
    // the cat and list APIs answer plain text unless asked for JSON, whatever
    // the Accept header
    OmittedParam {
        groups: "cat.",
        name: "format",
        fixed: Some("json"),
    },
    OmittedParam {
        groups: "list.",
        name: "format",
        fixed: Some("json"),
    },
];

/// The options omitted from the typed builder of an operation group, with
/// the value sent instead when there is one
pub fn omitted_params(group: &str) -> Vec<(&'static str, Option<&'static str>)> {
    OMITTED_PARAMS
        .iter()
        .filter(|p| {
            p.groups == "*"
                || p.groups == group
                || (p.groups.ends_with('.') && group.starts_with(p.groups))
        })
        .map(|p| (p.name, p.fixed))
        .collect()
}

/// Generates the source of the typed builders and clients
pub fn generate(api: &Api, docs_dir: &Path) -> anyhow::Result<String> {
    let mut tokens = TokenStream::new();
    tokens.append_all(use_declarations());
    tokens.append_all(quote!(
        use super::TypedOpenSearch;
    ));

    // root APIs
    let (builders, methods): (Vec<TokenStream>, Vec<TokenStream>) = api
        .root
        .endpoints()
        .iter()
        .filter_map(|(name, endpoint)| {
            let builder_name = name.to_pascal_case();
            RequestBuilder::new(
                docs_dir,
                "OpenSearch",
                name,
                &builder_name,
                &api.common_params,
                endpoint,
                true,
            )
            .build_typed()
        })
        .unzip();
    tokens.append_all(quote!(
        #(#builders)*

        impl TypedOpenSearch {
            #(#methods)*
        }
    ));

    // namespaces with at least one typed API
    for (namespace_name, namespace) in &api.namespaces {
        let namespace_pascal_case = namespace_name.to_pascal_case();
        let (builders, methods): (Vec<TokenStream>, Vec<TokenStream>) = namespace
            .endpoints()
            .iter()
            .filter_map(|(name, endpoint)| {
                let builder_name = format!("{}{}", namespace_pascal_case, name.to_pascal_case());
                RequestBuilder::new(
                    docs_dir,
                    &namespace_pascal_case,
                    name,
                    &builder_name,
                    &api.common_params,
                    endpoint,
                    false,
                )
                .build_typed()
            })
            .unzip();
        if builders.is_empty() {
            continue;
        }

        let client_ident = ident(&format!("Typed{}", namespace_pascal_case));
        let fn_ident = ident(namespace_name);
        let name_for_docs = namespace_name.replace('_', " ").to_pascal_case();
        let client_doc = doc(&format!(
            "Typed namespace client for {} APIs: the APIs with a typed response model",
            name_for_docs
        ));
        let fn_doc = doc(&format!(
            "Creates a typed namespace client for {} APIs",
            name_for_docs
        ));
        let cfg_attr = namespace.stability().outer_cfg_attr();
        let cfg_doc = stability_doc(namespace.stability());
        tokens.append_all(quote!(
            #(#builders)*

            #client_doc
            #cfg_doc
            #cfg_attr
            #[derive(Clone, Debug)]
            pub struct #client_ident<'a> {
                transport: &'a Transport,
            }

            #cfg_attr
            impl<'a> #client_ident<'a> {
                /// The transport used by this client
                pub fn transport(&self) -> &Transport {
                    self.transport
                }

                #(#methods)*
            }

            #cfg_attr
            impl TypedOpenSearch {
                #fn_doc
                pub fn #fn_ident(&self) -> #client_ident<'_> {
                    #client_ident { transport: self.transport() }
                }
            }
        ));
    }

    Ok(tokens.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn omitted_params_match_by_group_and_namespace() {
        assert_eq!(omitted_params("search"), vec![("filter_path", None)]);
        assert_eq!(
            omitted_params("cat.indices"),
            vec![("filter_path", None), ("format", Some("json"))]
        );
        assert_eq!(
            omitted_params("list.shards"),
            vec![("filter_path", None), ("format", Some("json"))]
        );
        // a prefix only matches whole namespaces
        assert_eq!(omitted_params("catalog.get"), vec![("filter_path", None)]);
    }
}
