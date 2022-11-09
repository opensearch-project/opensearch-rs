/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */
/* Some types from or based on types from elastic: https://github.com/elastic-rs/elastic
 *
 * Licensed under Apache 2.0: https://github.com/elastic-rs/elastic/blob/51298dd64278f34d2db911bd1a35eb757c336198/LICENSE-APACHE
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */
use crate::generator::{code_gen::*, ApiEndpoint, Path};
use inflector::Inflector;
use syn::token::{FatArrow, Paren};

use super::url_builder::UrlBuilder;

/// Builder for request url parts enum
///
/// The output of this structure is an enum that only accepts valid parameter combinations,
/// based on what's given in the paths for an endpoint.
#[derive(Debug, Clone)]
pub struct EnumBuilder<'a> {
    ident: syn::Ident,
    api_name: String,
    variants: Vec<syn::Variant>,
    paths: Vec<&'a Path>,
    has_lifetime: bool,
}

impl<'a> EnumBuilder<'a> {
    pub fn new(prefix: &str) -> Self {
        let name = Self::name(prefix);
        let api_name = split_on_pascal_case(prefix);
        EnumBuilder {
            ident: ident(&name),
            api_name,
            variants: vec![],
            paths: vec![],
            has_lifetime: false,
        }
    }

    fn name(prefix: &str) -> String {
        format!("{}Parts", prefix.to_pascal_case())
    }

    /// Whether this instance already contains a path with parts matching the given path
    fn contains_path_with_parts(&self, path: &'a Path) -> bool {
        let params = path.path.params();
        self.paths.iter().any(|&p| p.path.params() == params)
    }

    /// Whether this instance contains only a single path with no parts
    pub fn contains_single_parameterless_part(&self) -> bool {
        match self.paths.len() {
            1 => self.paths[0].parts.is_empty(),
            _ => false,
        }
    }

    pub fn with_path(mut self, path: &'a Path) -> Self {
        if !self.contains_path_with_parts(path) {
            let variant = match &path.parts.len() {
                0 => Self::parts_none(),
                _ => {
                    self.has_lifetime = true;
                    Self::parts(path)
                }
            };

            self.variants.push(variant);
            self.paths.push(path);
        }

        self
    }

    /// AST for a parts variant.
    fn parts(path: &Path) -> syn::Variant {
        let params = &path.path.params();

        let name = params
            .iter()
            .map(|k| k.to_pascal_case())
            .collect::<Vec<_>>()
            .join("");

        let doc = match params.len() {
            1 => doc(&params[0].replace('_', " ").to_pascal_case()),
            n => {
                let mut d: String = params
                    .iter()
                    .enumerate()
                    .filter(|&(i, _)| i != n - 1)
                    .map(|(_, e)| e.replace('_', " ").to_pascal_case())
                    .collect::<Vec<_>>()
                    .join(", ");

                d.push_str(
                    format!(" and {}", params[n - 1].replace('_', " ").to_pascal_case()).as_str(),
                );
                doc(&d)
            }
        };

        let ident = ident(&name);
        let params = path.path.params();
        let fields = params
            .iter()
            .map(|&p| typekind_to_ty(p, &path.parts[p].ty, true, false));

        parse_quote!(
            #doc
            #ident (#(#fields),*)
        )
    }

    /// AST for a `None` parts variant.
    fn parts_none() -> syn::Variant {
        parse_quote!(
            #[doc = "No parts"]
            None
        )
    }

    fn match_path(ty: &syn::Type, variant: &syn::Variant) -> syn::Path {
        let mut path = ty.get_path().to_owned();
        // Remove lifetimes from the enum type.
        for segment in &mut path.segments {
            segment.arguments = syn::PathArguments::None;
        }

        path.segments
            .push(syn::PathSegment::from(variant.ident.clone()));
        path
    }

    /// Get the field names for the enum tuple variant to match.
    fn match_fields(path: &Path) -> Vec<syn::Pat> {
        path.path
            .params()
            .iter()
            .map(|&p| {
                syn::Pat::Ident(syn::PatIdent {
                    attrs: vec![],
                    by_ref: None,
                    mutability: None,
                    ident: ident(valid_name(p)),
                    subpat: None,
                })
            })
            .collect()
    }

    /// Build this enum and return ASTs for its type, struct declaration and impl
    pub fn build(self) -> (syn::Type, syn::Item, syn::Item) {
        let variants = match self.variants.len() {
            0 => vec![Self::parts_none()],
            _ => self.variants,
        };

        let (enum_ty, generics) = {
            if self.has_lifetime {
                (ty_b(&self.ident.to_string()), generics_b())
            } else {
                (ty(&self.ident.to_string()), generics_none())
            }
        };

        let enum_decl = {
            let doc = doc(&format!("API parts for the {} API", self.api_name));
            parse_quote! {
                #[derive(Debug, Clone, PartialEq, Eq)]
                #doc
                pub enum #enum_ty {
                    #(#variants),*
                }
            }
        };

        let enum_impl = {
            let url_doc = doc(&format!(
                "Builds a relative URL path to the {} API",
                self.api_name
            ));

            let arms = variants
                .iter()
                .zip(self.paths.iter())
                .map(|(variant, path)| {
                    let match_path = Self::match_path(&enum_ty, variant);
                    let fields = Self::match_fields(path);

                    let pat = match fields.len() {
                        0 => syn::Pat::Path(syn::PatPath {
                            attrs: vec![],
                            qself: None,
                            path: match_path,
                        }),
                        _ => syn::Pat::TupleStruct(syn::PatTupleStruct {
                            attrs: vec![],
                            path: match_path,
                            pat: syn::PatTuple {
                                attrs: vec![],
                                paren_token: Paren(Span::call_site()),
                                elems: fields.into_iter().collect(),
                            },
                        }),
                    };

                    let body = Box::new(UrlBuilder::new(path).build());
                    syn::Arm {
                        attrs: vec![],
                        pat,
                        guard: None,
                        fat_arrow_token: FatArrow(Span::call_site()),
                        body,
                        comma: None,
                    }
                });

            parse_quote! {
                impl #generics #enum_ty {
                    #url_doc
                    pub fn url(self) -> Cow<'static, str> {
                        match self {
                            #(#arms),*
                        }
                    }
                }
            }
        };

        (enum_ty, enum_decl, enum_impl)
    }
}

impl<'a> From<&'a (String, ApiEndpoint)> for EnumBuilder<'a> {
    fn from(value: &'a (String, ApiEndpoint)) -> Self {
        let endpoint = &value.1;
        let mut builder = EnumBuilder::new(value.0.to_pascal_case().as_ref());
        for path in &endpoint.url.paths {
            builder = builder.with_path(path);
        }

        builder
    }
}

#[cfg(test)]
mod tests {
    #![cfg_attr(rustfmt, rustfmt_skip)]

    use super::*;
    use crate::generator::{Url, Path, HttpMethod, Body, Deprecated, Type, TypeKind, Documentation, ast_eq, Stability};
    use std::collections::BTreeMap;
    use crate::generator::code_gen::url::url_builder::PathString;

    #[test]
    fn generate_parts_enum_from_endpoint() {
        let endpoint = (
            "search".to_string(),
            ApiEndpoint {
                full_name: Some("search".to_string()),
                documentation: Documentation {
                    description: None,
                    url: None,
                },
                stability: Stability::Stable,
                deprecated: None,
                url: Url {
                    paths: vec![
                        Path {
                            path: PathString("/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: BTreeMap::new(),
                            deprecated: None,
                        },
                        Path {
                            path: PathString("/{index}/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: {
                                let mut map = BTreeMap::new();
                                map.insert("index".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of document types to search".to_string()),
                                    options: vec![],
                                    default: None,
                                    deprecated: None,
                                });
                                map
                            },
                            deprecated: None,
                        },
                        Path {
                            path: PathString("/{index}/_search".to_string()),
                            methods: vec![HttpMethod::Get, HttpMethod::Post],
                            parts: {
                                let mut map = BTreeMap::new();
                                map.insert("index".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of index names to search".to_string()),
                                    options: vec![],
                                    default: None,
                                    deprecated: None,
                                });
                                map.insert("type".to_string(), Type {
                                    ty: TypeKind::List,
                                    description: Some("A comma-separated list of document types to search".to_string()),
                                    options: vec![],
                                    default: None,
                                    deprecated: None,
                                });
                                map
                            },
                            deprecated: Some(Deprecated {
                                version: "7.0.0".to_string(),
                                description: "types are going away".to_string()
                            }),
                        },
                    ],
                },
                params: BTreeMap::new(),
                body: Some(Body {
                    description: Some("The search request".to_string()),
                    required: Some(false),
                    serialize: None
                }),
            },
        );

        let (enum_ty, enum_decl, enum_impl) = EnumBuilder::from(&endpoint).build();

        assert_eq!(ty_b("SearchParts"), enum_ty);

        let expected_decl = quote!(
            #[derive(Debug, Clone, PartialEq, Eq)]
            #[doc = "API parts for the Search API"]
            pub enum SearchParts<'b> {
                #[doc = "No parts"]
                None,
                #[doc = "Index"]
                Index(&'b [&'b str])
            }
        );

        ast_eq(expected_decl, enum_decl);

        let expected_impl = quote!(
            impl<'b> SearchParts<'b> {
                #[doc = "Builds a relative URL path to the Search API"]
                pub fn url(self) -> Cow<'static, str> {
                    match self {
                        SearchParts::None => "/_search".into(),
                        SearchParts::Index(index) => {
                            let index_str = index.join(",");
                            let encoded_index: Cow<str> = percent_encode(index_str.as_bytes(), PARTS_ENCODED).into();
                            let mut p = String::with_capacity(9usize + encoded_index.len());
                            p.push('/');
                            p.push_str(encoded_index.as_ref());
                            p.push_str("/_search");
                            p.into()
                        }
                    }
                }
            }
        );

        ast_eq(expected_impl, enum_impl);
    }
}
