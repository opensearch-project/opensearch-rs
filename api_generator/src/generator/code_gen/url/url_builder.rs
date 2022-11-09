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
use crate::generator::{code_gen::*, Path, Type, TypeKind};
use serde::{Deserialize, Deserializer};
use std::{collections::BTreeMap, fmt, iter::Iterator, str};

/// A URL path
#[derive(Debug, Deserialize, PartialEq, Eq, Clone)]
pub struct PathString(#[serde(deserialize_with = "rooted_path_string")] pub String);

/// Ensure all deserialized paths have a leading `/`
fn rooted_path_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    if !s.starts_with('/') {
        Ok(format!("/{}", s))
    } else {
        Ok(s)
    }
}

impl fmt::Display for PathString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PathString {
    /// Splits a path into a vector of parameter and literal parts
    pub fn split(&self) -> Vec<PathPart> {
        PathString::parse(self.0.as_bytes(), PathParseState::Literal, Vec::new())
    }

    /// Gets the parameters from the path
    pub fn params(&self) -> Vec<&str> {
        self.split()
            .iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }

    fn parse<'a>(i: &'a [u8], state: PathParseState, r: Vec<PathPart<'a>>) -> Vec<PathPart<'a>> {
        if i.is_empty() {
            return r;
        }

        let mut r = r;
        match state {
            PathParseState::Literal => {
                let (rest, part) = PathString::parse_literal(i);
                if !part.is_empty() {
                    r.push(PathPart::Literal(part));
                }

                PathString::parse(rest, PathParseState::Param, r)
            }
            PathParseState::Param => {
                let (rest, part) = PathString::parse_param(i);
                if !part.is_empty() {
                    r.push(PathPart::Param(part));
                }

                PathString::parse(rest, PathParseState::Literal, r)
            }
        }
    }

    fn parse_literal(i: &[u8]) -> (&[u8], &str) {
        if i[0] == b'}' {
            let i = shift(i, 1);
            take_while(i, |c| c != b'{')
        } else {
            take_while(i, |c| c != b'{')
        }
    }

    fn parse_param(i: &[u8]) -> (&[u8], &str) {
        if i[0] == b'{' {
            let i = shift(i, 1);
            take_while(i, |c| c != b'}')
        } else {
            take_while(i, |c| c != b'}')
        }
    }
}

enum PathParseState {
    Literal,
    Param,
}

/// A part of a Path
#[derive(Debug, PartialEq, Eq)]
pub enum PathPart<'a> {
    Literal(&'a str),
    Param(&'a str),
}

pub trait PathParams<'a> {
    fn params(&'a self) -> Vec<&'a str>;
}

impl<'a> PathParams<'a> for Vec<PathPart<'a>> {
    fn params(&'a self) -> Vec<&'a str> {
        self.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => Some(p),
                _ => None,
            })
            .collect()
    }
}

/// Builder for an efficient url value replacer.
pub struct UrlBuilder<'a> {
    path: Vec<PathPart<'a>>,
    parts: &'a BTreeMap<String, Type>,
}

impl<'a> UrlBuilder<'a> {
    pub fn new(path: &'a Path) -> Self {
        let path_parts = path.path.split();
        let parts = &path.parts;

        UrlBuilder {
            path: path_parts,
            parts,
        }
    }

    /// Build the AST for an allocated url from the path literals and params.
    fn build_owned(self) -> syn::Block {
        // collection of let {name}_str = [self.]{name}.[join(",")|to_string()];
        let let_params_exprs = Self::let_parameters_exprs(&self.path, self.parts);

        let let_encoded_params_exprs = Self::let_encoded_exprs(&self.path, self.parts);

        let url_ident = ident("p");
        let len_expr = {
            let lit_len_expr = Self::literal_length_expr(&self.path);
            let mut params_len_exprs = Self::parameter_length_exprs(&self.path);
            let mut len_exprs = vec![lit_len_expr];
            len_exprs.append(&mut params_len_exprs);
            Self::summed_length_expr(len_exprs)
        };
        let let_stmt = Self::let_p_stmt(url_ident.clone(), len_expr);

        let push_stmts = Self::push_str_stmts(url_ident.clone(), &self.path);

        parse_quote!({
            #(#let_params_exprs)*
            #(#let_encoded_params_exprs)*
            #let_stmt
            #(#push_stmts)*
            #url_ident.into()
        })
    }

    /// Build the AST for a literal path
    fn build_borrowed_literal(self) -> syn::Expr {
        let path: Vec<&'a str> = self
            .path
            .iter()
            .map(|p| match *p {
                PathPart::Literal(p) => p,
                _ => panic!("Only PathPart::Literal is supported by a borrowed url."),
            })
            .collect();

        let path = path.join("");
        parse_quote!(#path.into())
    }

    /// Get the number of chars in all literal parts for the url.
    fn literal_length_expr(url: &[PathPart<'a>]) -> syn::Expr {
        let len = url
            .iter()
            .filter_map(|p| match *p {
                PathPart::Literal(p) => Some(p),
                _ => None,
            })
            .fold(0, |acc, p| acc + p.len());

        parse_quote!(#len)
    }

    /// Creates the AST for a let expression to percent encode path parts
    fn let_encoded_exprs(url: &[PathPart<'a>], parts: &BTreeMap<String, Type>) -> Vec<syn::Stmt> {
        url.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => {
                    let name = valid_name(p);
                    let path_expr = match &parts[p].ty {
                        TypeKind::String => path_none(name).into_expr(),
                        _ => path_none(format!("{}_str", name).as_str()).into_expr(),
                    };

                    let encoded_ident = ident(&format!("encoded_{}", name));

                    Some(parse_quote!(let #encoded_ident: Cow<str> = percent_encode(#path_expr.as_bytes(), PARTS_ENCODED).into();))
                }
                _ => None,
            })
            .collect()
    }

    /// Creates the AST for a let expression for path parts
    fn let_parameters_exprs(
        url: &[PathPart<'a>],
        parts: &BTreeMap<String, Type>,
    ) -> Vec<syn::Stmt> {
        url.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => {
                    let name = valid_name(p);
                    let name_ident = ident(name);
                    let ty = &parts[p].ty;

                    // don't generate an assignment expression for strings
                    if ty == &TypeKind::String {
                        return None;
                    }

                    let name_str_ident = ident(&format!("{}_str", name));

                    // build a different expression, depending on the type of parameter
                    let init: syn::Expr = match ty {
                        TypeKind::List => {
                            // Join list values together
                            parse_quote!(#name_ident.join(","))
                        }
                        _ => {
                            // Handle enums, long, int, etc. by calling to_string()
                            parse_quote!(#name_ident.to_string())
                        }
                    };

                    Some(parse_quote!(let #name_str_ident = #init;))
                }
                _ => None,
            })
            .collect()
    }

    /// Get an expression to find the number of chars in each parameter part for the url.
    fn parameter_length_exprs(url: &[PathPart<'a>]) -> Vec<syn::Expr> {
        url.iter()
            .filter_map(|p| match *p {
                PathPart::Param(p) => {
                    let name = ident(&format!("encoded_{}", valid_name(p)));
                    Some(parse_quote!(#name.len()))
                }
                _ => None,
            })
            .collect()
    }

    /// Get an expression that is the binary addition of each of the given expressions.
    fn summed_length_expr(len_exprs: Vec<syn::Expr>) -> syn::Expr {
        match len_exprs.len() {
            1 => len_exprs.into_iter().next().unwrap(),
            _ => {
                let mut len_iter = len_exprs.into_iter();

                let first_expr = len_iter.next().unwrap();

                len_iter.fold(first_expr, |acc, p| parse_quote!(#acc + #p))
            }
        }
    }

    /// Get a statement to build a `String` with a capacity of the given expression.
    fn let_p_stmt(url_ident: syn::Ident, len_expr: syn::Expr) -> syn::Stmt {
        parse_quote!(let mut #url_ident = String::with_capacity(#len_expr);)
    }

    /// Get a list of statements that append each part to a `String` in order.
    fn push_str_stmts(url_ident: syn::Ident, url: &[PathPart<'a>]) -> Vec<syn::Stmt> {
        url.iter()
            .map(|p| match *p {
                PathPart::Literal(p) => {
                    if p.len() == 1 {
                        let lit = p.chars().next().unwrap();
                        parse_quote!(#url_ident.push(#lit);)
                    } else {
                        let lit = p.to_string();
                        parse_quote!(#url_ident.push_str(#lit);)
                    }
                }
                PathPart::Param(p) => {
                    let ident = ident(&format!("encoded_{}", valid_name(p)));
                    parse_quote!(#url_ident.push_str(#ident.as_ref());)
                }
            })
            .collect()
    }

    pub fn build(self) -> syn::Expr {
        let has_params = self.path.iter().any(|p| matches!(p, PathPart::Param(_)));

        if has_params {
            self.build_owned().into_expr()
        } else {
            self.build_borrowed_literal()
        }
    }
}

/// Helper for wrapping a value as a quotable expression.
pub trait IntoExpr {
    fn into_expr(self) -> syn::Expr;
}

impl IntoExpr for syn::Path {
    fn into_expr(self) -> syn::Expr {
        syn::Expr::Path(syn::ExprPath {
            attrs: vec![],
            qself: None,
            path: self,
        })
    }
}

impl IntoExpr for syn::Block {
    fn into_expr(self) -> syn::Expr {
        syn::Expr::Block(syn::ExprBlock {
            attrs: vec![],
            label: None,
            block: self,
        })
    }
}

impl IntoExpr for syn::Type {
    fn into_expr(self) -> syn::Expr {
        parse_quote!(#self)
    }
}

impl IntoExpr for syn::Pat {
    fn into_expr(self) -> syn::Expr {
        parse_quote!(#self)
    }
}
