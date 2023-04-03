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
pub mod namespace_clients;
pub mod params;
pub mod request;
pub mod root;
pub mod url;

use crate::generator::{Stability, TypeKind};
use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::str;
use syn::{
    parse_quote,
    punctuated::Punctuated,
    token::{Gt, Lt},
    AngleBracketedGenericArguments, GenericArgument, ImplItem, LitStr, PathArguments, PathSegment,
};

/// use declarations common across builders
pub fn use_declarations() -> TokenStream {
    quote!(
        #![allow(unused_imports)]

        use crate::{
            client::OpenSearch,
            params::*,
            error::Error,
            http::{
                headers::{HeaderName, HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT},
                Method,
                request::{Body, NdBody, JsonBody, PARTS_ENCODED},
                response::Response,
                transport::Transport,
            },
        };
        use std::{
            borrow::Cow,
            time::Duration
        };
        use percent_encoding::percent_encode;
        use serde::Serialize;
    )
}

/// AST for a string literal
fn lit<S: AsRef<str> + ?Sized>(lit: &S) -> syn::Lit {
    syn::Lit::Str(LitStr::new(lit.as_ref(), Span::call_site()))
}

/// AST for an identifier
fn ident<S: AsRef<str> + ?Sized>(name: &S) -> syn::Ident {
    syn::Ident::new(name.as_ref(), Span::call_site())
}

/// AST for document attribute
fn doc<S: AsRef<str> + ?Sized>(comment: &S) -> syn::Attribute {
    let comment = lit(comment);
    parse_quote!(#[doc = #comment])
}

fn doc_escaped<S: ?Sized + AsRef<str>>(comment: &S) -> syn::Attribute {
    doc(&html_escape::encode_text(comment))
}

fn stability_doc(stability: Stability) -> Option<syn::Attribute> {
    match stability {
        Stability::Experimental => Some(doc(r#"&nbsp;
# Optional, experimental
This requires the `experimental-apis` feature. Can have breaking changes in future
versions or might even be removed entirely.
        "#)),
        Stability::Beta => Some(doc(r#"&nbsp;
# Optional, beta
This requires the `beta-apis` feature. On track to become stable but breaking changes can
happen in minor versions.
        "#)),
        Stability::Stable => None,
    }
}

/// AST for an expression parsed from quoted tokens
pub fn parse_expr(input: TokenStream) -> syn::Expr {
    syn::parse_str(&input.to_string()).unwrap()
}

/// Ensures that the name generated is one that is valid for Rust
pub fn valid_name(s: &str) -> &str {
    match s {
        "type" => "ty",
        s => s,
    }
}

/// AST for a path variable.
fn path(path: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Type>) -> syn::Path {
    path_segments(vec![(path, lifetimes, types)])
}

/// AST for a simple path variable.
fn path_none(path_ident: &str) -> syn::Path {
    path(path_ident, vec![], vec![])
}

/// AST for a path variable.
fn path_segments(paths: Vec<(&str, Vec<syn::Lifetime>, Vec<syn::Type>)>) -> syn::Path {
    syn::Path {
        leading_colon: None,
        segments: paths
            .into_iter()
            .map::<PathSegment, _>(|(path, lifetimes, types)| {
                let ident = ident(valid_name(path));
                PathSegment {
                    ident,
                    arguments: if !lifetimes.is_empty() || !types.is_empty() {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: None,
                            lt_token: Lt(Span::call_site()),
                            args: lifetimes
                                .into_iter()
                                .map(GenericArgument::Lifetime)
                                .chain(types.into_iter().map(GenericArgument::Type))
                                .collect(),
                            gt_token: Gt(Span::call_site()),
                        })
                    } else {
                        PathArguments::None
                    },
                }
            })
            .collect(),
    }
}

pub trait GetPath {
    fn get_path(&self) -> &syn::Path;
}

impl GetPath for syn::Type {
    fn get_path(&self) -> &syn::Path {
        match *self {
            syn::Type::Path(ref p) => p.get_path(),
            ref p => panic!("Expected syn::Type::Path, but found {:?}", p),
        }
    }
}

impl GetPath for syn::Path {
    fn get_path(&self) -> &syn::Path {
        self
    }
}

impl GetPath for syn::TypePath {
    fn get_path(&self) -> &syn::Path {
        &self.path
    }
}

pub trait GetIdent {
    fn get_ident(&self) -> &syn::Ident;
}

impl<T: GetPath> GetIdent for T {
    fn get_ident(&self) -> &syn::Ident {
        &self.get_path().segments[0].ident
    }
}

impl GetIdent for ImplItem {
    fn get_ident(&self) -> &syn::Ident {
        match self {
            ImplItem::Const(i) => &i.ident,
            ImplItem::Fn(i) => &i.sig.ident,
            ImplItem::Type(i) => &i.ident,
            _ => panic!("{:?} has no ident", self),
        }
    }
}

/// Gets the Ty syntax token for a TypeKind
/// TODO: This function is serving too many purposes. Refactor it
fn typekind_to_ty(name: &str, kind: &TypeKind, required: bool, fn_arg: bool) -> syn::Type {
    let mut v = String::new();
    if !required {
        v.push_str("Option<");
    }

    let str_type = "&'b str";
    match kind {
        TypeKind::Unknown(_) => v.push_str(str_type),
        TypeKind::List => {
            v.push_str("&'b [");
            v.push_str(str_type);
            v.push(']');
        }
        TypeKind::Enum => match name {
            // opened https://github.com/elastic/elasticsearch/issues/53212
            // to discuss whether this really should be a collection
            "expand_wildcards" => {
                // Expand wildcards should
                v.push_str("&'b [");
                v.push_str(name.to_pascal_case().as_str());
                v.push(']');
            }
            _ => v.push_str(name.to_pascal_case().as_str()),
        },
        TypeKind::String => v.push_str(str_type),
        TypeKind::Text => v.push_str(str_type),
        TypeKind::Boolean => match name {
            "track_total_hits" => {
                if fn_arg {
                    v.push_str(format!("Into<{}>", name.to_pascal_case()).as_str())
                } else {
                    v.push_str(name.to_pascal_case().as_str())
                }
            }
            _ => v.push_str("bool"),
        },
        TypeKind::Number => v.push_str("i64"),
        TypeKind::Float => v.push_str("f32"),
        TypeKind::Double => v.push_str("f64"),
        TypeKind::Integer => v.push_str("i32"),
        TypeKind::Long => v.push_str("i64"),
        TypeKind::Date => v.push_str(str_type),
        TypeKind::Time => v.push_str(str_type),
        TypeKind::Union(u) => match name {
            "slices" => v.push_str("Slices"),
            _ => panic!("unsupported union type: {:?}", u),
        },
    };

    if !required {
        v.push('>');
    }

    syn::parse_str(v.as_str()).unwrap()
}

/// A standard `'b` lifetime
pub fn lifetime_b() -> syn::Lifetime {
    parse_quote!('b)
}

pub trait HasLifetime {
    fn has_lifetime(&self) -> bool;
}

impl<T: GetPath> HasLifetime for T {
    fn has_lifetime(&self) -> bool {
        match self.get_path().segments[0].arguments {
            syn::PathArguments::AngleBracketed(ref params) => params
                .args
                .iter()
                .any(|a| matches!(a, syn::GenericArgument::Lifetime(_))),
            _ => false,
        }
    }
}

/// Generics with a standard `'b` lifetime
pub fn generics_b() -> syn::Generics {
    generics(vec![lifetime_b()], vec![])
}

/// Generics with no parameters.
pub fn generics_none() -> syn::Generics {
    syn::Generics {
        lt_token: None,
        params: Punctuated::default(),
        gt_token: None,
        where_clause: None,
    }
}

/// Generics with the given lifetimes and type bounds.
pub fn generics(lifetimes: Vec<syn::Lifetime>, types: Vec<syn::TypeParam>) -> syn::Generics {
    let params: Punctuated<_, _> = lifetimes
        .into_iter()
        .map(|l| {
            syn::GenericParam::Lifetime(syn::LifetimeParam {
                attrs: vec![],
                lifetime: l,
                colon_token: None,
                bounds: Punctuated::default(),
            })
        })
        .chain(types.into_iter().map(syn::GenericParam::Type))
        .collect();

    let has_params = !params.is_empty();

    syn::Generics {
        lt_token: if has_params {
            Some(Lt(Span::call_site()))
        } else {
            None
        },
        params,
        gt_token: if has_params {
            Some(Gt(Span::call_site()))
        } else {
            None
        },
        where_clause: None,
    }

    // parse_quote!(<#(#lifetimes),* #(#types),*>)
}

/// AST for a path type with lifetimes and type parameters.
pub fn ty_path(ty: &str, lifetimes: Vec<syn::Lifetime>, types: Vec<syn::Type>) -> syn::Type {
    syn::Type::Path(syn::TypePath {
        qself: None,
        path: path(ty, lifetimes, types),
    })
}

/// AST for a path type with a `'b` lifetime.
pub fn ty_b(ty: &str) -> syn::Type {
    ty_path(ty, vec![lifetime_b()], vec![])
}

/// AST for a simple path type.
pub fn ty(ty: &str) -> syn::Type {
    ty_path(ty, vec![], vec![])
}

/// Helper for wrapping a value as a quotable statement.
pub trait IntoStmt {
    fn into_stmt(self) -> syn::Stmt;
}

impl IntoStmt for syn::Item {
    fn into_stmt(self) -> syn::Stmt {
        syn::Stmt::Item(self)
    }
}

impl IntoStmt for syn::Expr {
    fn into_stmt(self) -> syn::Stmt {
        syn::Stmt::Expr(self, Some(syn::token::Semi(Span::call_site())))
    }
}

pub fn take_while<F>(i: &[u8], f: F) -> (&[u8], &str)
where
    F: Fn(u8) -> bool,
{
    let mut ctr = 0;

    for c in i {
        if f(*c) {
            ctr += 1;
        } else {
            break;
        }
    }

    (&i[ctr..], str::from_utf8(&i[0..ctr]).unwrap())
}

pub fn shift(i: &[u8], c: usize) -> &[u8] {
    match c {
        c if c >= i.len() => &[],
        _ => &i[c..],
    }
}

pub fn split_on_pascal_case(s: &str) -> String {
    s.chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i != 0 && c.is_uppercase() {
                Some(' ')
            } else {
                None
            }
            .into_iter()
            .chain(std::iter::once(c))
        })
        .collect()
}
