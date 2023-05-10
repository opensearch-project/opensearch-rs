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

/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 *
 * Modifications Copyright OpenSearch Contributors. See
 * GitHub history for details.
 */

use super::{ResultIterExt, Step};
use crate::{
    regex::clean_regex,
    rusty_json::{from_set_value, rusty_json},
};
use anyhow::anyhow;
use api_generator::generator::{Api, ApiEndpoint, TypeKind};
use inflector::Inflector;
use itertools::Itertools;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use serde_yaml::Value;
use std::collections::BTreeMap;
use syn::__private::TokenStreamExt;

/// A catch expression on a do step
pub struct Catch(String);

impl Catch {
    fn needs_response_body(&self) -> bool {
        self.0.starts_with('/')
    }
}

impl ToTokens for Catch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        fn http_status_code(status_code: u16, tokens: &mut TokenStream) {
            tokens.append_all(quote! {
                crate::assert_response_status_code!(response, #status_code);
            });
        }

        match self.0.as_ref() {
            "bad_request" => http_status_code(400, tokens),
            "unauthorized" => http_status_code(401, tokens),
            "forbidden" => http_status_code(403, tokens),
            "missing" => http_status_code(404, tokens),
            "request_timeout" => http_status_code(408, tokens),
            "conflict" => http_status_code(409, tokens),
            "request" => {
                tokens.append_all(quote! {
                    crate::assert_request_status_code!(response.status_code());
                });
            }
            "unavailable" => http_status_code(503, tokens),
            "param" => {
                // Not possible to pass a bad param to the client so ignore.
            }
            s => {
                let t = clean_regex(s);
                tokens.append_all(quote! {
                    crate::assert_regex_match!(&text, #t);
                });
            }
        }
    }
}

pub struct Do {
    api_call: ApiCall,
    warnings: Vec<String>,
    allowed_warnings: Vec<String>,
    catch: Option<Catch>,
}

impl ToTokens for Do {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let _ = self.to_tokens(false, tokens);
    }
}

impl From<Do> for Step {
    fn from(d: Do) -> Self {
        Step::Do(d)
    }
}

impl Do {
    pub fn to_tokens(&self, mut read_response: bool, tokens: &mut TokenStream) -> bool {
        self.api_call.to_tokens(tokens);

        if !self.warnings.is_empty() {
            tokens.append_all(quote! {
                let warnings: Vec<&str> = response
                    .warning_headers()
                    .collect();
            });
            for warning in &self.warnings {
                tokens.append_all(quote! {
                    crate::assert_warnings_contain!(warnings, #warning);
                });
            }
        } else if !self.allowed_warnings.is_empty() {
            let allowed = &self.allowed_warnings;
            tokens.append_all(quote! {
                let allowed_warnings = vec![#(#allowed),*];
                let warnings: Vec<&str> = response.warning_headers()
                    .filter(|w| !allowed_warnings.iter().any(|a| w.contains(a)))
                    .collect();
                crate::assert_warnings_is_empty!(warnings);
            });
        }

        if let Some(c) = &self.catch {
            if !read_response && c.needs_response_body() {
                read_response = true;
                tokens.append_all(quote! {
                    let (method, status_code, text, json) = client::read_response(response).await?;
                });
            }
            c.to_tokens(tokens);
        }

        if let Some(i) = &self.api_call.ignore {
            tokens.append_all(quote! {
                crate::assert_response_success_or!(response, #i);
            });
        }

        read_response
    }

    pub fn try_parse(api: &Api, yaml: &Value) -> anyhow::Result<Do> {
        let hash = yaml
            .as_mapping()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", yaml))?;

        let mut call: Option<(&str, &Value)> = None;
        let mut headers = BTreeMap::new();
        let mut warnings: Vec<String> = Vec::new();
        let mut allowed_warnings: Vec<String> = Vec::new();
        let mut catch = None;

        fn to_string_vec(v: &Value) -> Vec<String> {
            v.as_sequence()
                .map(|a| a.iter().map(|y| y.as_str().unwrap().to_string()).collect())
                .unwrap()
        }

        hash.iter()
            .map(|(k, v)| {
                let key = k
                    .as_str()
                    .ok_or_else(|| anyhow!("expected string but found {:?}", k))?;

                match key {
                    "headers" => {
                        let hash = v
                            .as_mapping()
                            .ok_or_else(|| anyhow!("expected hash but found {:?}", v))?;
                        for (hk, hv) in hash.iter() {
                            let h = hk
                                .as_str()
                                .ok_or_else(|| anyhow!("expected string but found {:?}", hk))?;
                            let v = hv
                                .as_str()
                                .ok_or_else(|| anyhow!("expected string but found {:?}", hv))?;
                            headers.insert(h.into(), v.into());
                        }
                        Ok(())
                    }
                    "catch" => {
                        catch = v.as_str().map(|s| Catch(s.to_string()));
                        Ok(())
                    }
                    "node_selector" => Ok(()),
                    "warnings" => {
                        warnings = to_string_vec(v);
                        Ok(())
                    }
                    "allowed_warnings" => {
                        allowed_warnings = to_string_vec(v);
                        Ok(())
                    }
                    api_call => {
                        call = Some((api_call, v));
                        Ok(())
                    }
                }
            })
            .collect_results()?;

        let (call, value) = call.ok_or_else(|| anyhow!("no API found in do"))?;
        let endpoint = api
            .endpoint_for_api_call(call)
            .ok_or_else(|| anyhow!(r#"no API found for "{}""#, call))?;
        let api_call = ApiCall::try_from(api, endpoint, value, headers)?;

        Ok(Do {
            api_call,
            catch,
            warnings,
            allowed_warnings,
        })
    }

    pub fn namespace(&self) -> Option<&String> {
        self.api_call.namespace.as_ref()
    }
}

/// The components of an API call
pub struct ApiCall {
    pub namespace: Option<String>,
    function: syn::Expr,
    parts: Option<TokenStream>,
    params: Option<TokenStream>,
    headers: BTreeMap<String, String>,
    body: Option<TokenStream>,
    ignore: Option<u16>,
}

impl ToTokens for ApiCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let function = &self.function;
        let parts = &self.parts;
        let params = &self.params;
        let body = &self.body;

        let headers: Vec<TokenStream> = self
            .headers
            .iter()
            .map(|(k, v)| {
                // header names **must** be lowercase to satisfy Header lib
                let k = k.to_lowercase();
                let v = from_set_value(v);

                // handle "set" value in headers
                quote! { .header(
                    HeaderName::from_static(#k),
                    HeaderValue::from_str(#v.as_ref())?)
                }
            })
            .collect();

        tokens.append_all(quote! {
            let response = client.#function(#parts)
                #(#headers)*
                #params
                #body
                .send()
                .await?;
        });
    }
}

impl ApiCall {
    /// Try to create an API call
    pub fn try_from(
        api: &Api,
        endpoint: &ApiEndpoint,
        yaml: &Value,
        headers: BTreeMap<String, String>,
    ) -> anyhow::Result<ApiCall> {
        let hash = yaml
            .as_mapping()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", yaml))?;

        let mut parts: Vec<(&str, &Value)> = vec![];
        let mut params: Vec<(&str, &Value)> = vec![];
        let mut body: Option<TokenStream> = None;
        let mut ignore: Option<u16> = None;

        // work out what's a URL part and what's a param in the supplied
        // arguments for the API call
        for (k, v) in hash.iter() {
            match k.as_str().unwrap() {
                "body" => body = Self::generate_body(endpoint, v)?,
                "ignore" => {
                    ignore = match v.as_i64() {
                        Some(i) => Some(i as u16),
                        // handle ignore as an array of i64
                        None => Some(v.as_sequence().unwrap()[0].as_i64().unwrap() as u16),
                    }
                }
                key if endpoint.params.contains_key(key) || api.common_params.contains_key(key) => {
                    params.push((key, v))
                }
                key => parts.push((key, v)),
            }
        }

        let api_call = endpoint.full_name.as_ref().unwrap();
        let parts = Self::generate_parts(api_call, endpoint, &parts)?;
        let params = Self::generate_params(api, endpoint, &params)?;
        let function: syn::Expr = syn::parse_str(&api_call.replace('.', "().")).unwrap();
        let namespace: Option<String> = if api_call.contains('.') {
            let namespaces: Vec<&str> = api_call.splitn(2, '.').collect();
            Some(namespaces[0].to_string())
        } else {
            None
        };

        Ok(ApiCall {
            namespace,
            function,
            parts,
            params,
            headers,
            body,
            ignore,
        })
    }

    fn generate_enum(
        enum_name: &str,
        variant: &str,
        options: &[serde_json::Value],
    ) -> anyhow::Result<TokenStream> {
        if !variant.is_empty() && !options.contains(&serde_json::Value::String(variant.to_owned()))
        {
            return Err(anyhow!(
                "options {:?} does not contain value {}",
                &options,
                variant
            ));
        }

        let e: String = enum_name.to_pascal_case();
        let enum_name = syn::Ident::new(e.as_str(), Span::call_site());
        let variant = if variant.is_empty() {
            // TODO: Should we simply omit empty Refresh tests?
            if e == "Refresh" {
                syn::Ident::new("True", Span::call_site())
            } else if e == "Size" {
                syn::Ident::new("Unspecified", Span::call_site())
            } else {
                return Err(anyhow!("unhandled empty value for {}", &e));
            }
        } else {
            syn::Ident::new(&variant.to_pascal_case(), Span::call_site())
        };

        Ok(quote!(#enum_name::#variant))
    }

    fn generate_params(
        api: &Api,
        endpoint: &ApiEndpoint,
        params: &[(&str, &Value)],
    ) -> anyhow::Result<Option<TokenStream>> {
        match params.len() {
            0 => Ok(None),
            _ => {
                let mut tokens = TokenStream::new();
                for (n, v) in params {
                    let param_ident = syn::Ident::new(
                        api_generator::generator::code_gen::valid_name(n),
                        Span::call_site(),
                    );

                    let ty = match endpoint.params.get(*n) {
                        Some(t) => Ok(t),
                        None => match api.common_params.get(*n) {
                            Some(t) => Ok(t),
                            None => Err(anyhow!(r#"no param found for "{}""#, n)),
                        },
                    }?;

                    let kind = &ty.ty;

                    match v {
                        Value::String(ref s) => {
                            let is_set_value = s.starts_with('$');

                            match kind {
                                TypeKind::Enum => {
                                    if n == &"expand_wildcards" {
                                        // expand_wildcards might be defined as a comma-separated
                                        // string. e.g.
                                        let idents: Vec<TokenStream> = s
                                            .split(',')
                                            .collect::<Vec<_>>()
                                            .iter()
                                            .map(|e| Self::generate_enum(n, e, &ty.options))
                                            .collect_results()?;

                                        tokens.append_all(quote! {
                                            .#param_ident(&[#(#idents),*])
                                        });
                                    } else {
                                        let e = Self::generate_enum(n, s.as_str(), &ty.options)?;
                                        tokens.append_all(quote! {
                                            .#param_ident(#e)
                                        });
                                    }
                                }
                                TypeKind::List => {
                                    let values = s.split(',');
                                    tokens.append_all(quote! {
                                        .#param_ident(&[#(#values),*])
                                    })
                                }
                                TypeKind::Boolean => match s.parse::<bool>() {
                                    Ok(b) => tokens.append_all(quote! {
                                        .#param_ident(#b)
                                    }),
                                    Err(e) => {
                                        return Err(anyhow!(
                                            r#"cannot parse bool from "{}" for param "{}", {}"#,
                                            s,
                                            n,
                                            e
                                        ))
                                    }
                                },
                                TypeKind::Double => match s.parse::<f64>() {
                                    Ok(f) => tokens.append_all(quote! {
                                        .#param_ident(#f)
                                    }),
                                    Err(e) => {
                                        return Err(anyhow!(
                                            r#"cannot parse f64 from "{}" for param "{}", {}"#,
                                            s,
                                            n,
                                            e
                                        ))
                                    }
                                },
                                TypeKind::Integer => {
                                    if is_set_value {
                                        let set_value = from_set_value(s);
                                        tokens.append_all(quote! {
                                           .#param_ident(#set_value.as_i64().unwrap() as i32)
                                        });
                                    } else {
                                        match s.parse::<i32>() {
                                            Ok(i) => tokens.append_all(quote! {
                                                .#param_ident(#i)
                                            }),
                                            Err(e) => {
                                                return Err(anyhow!(
                                                    r#"cannot parse i32 from "{}" for param "{}", {}"#,
                                                    s,
                                                    n,
                                                    e
                                                ))
                                            }
                                        }
                                    }
                                }
                                TypeKind::Number | TypeKind::Long => {
                                    if is_set_value {
                                        let set_value = from_set_value(s);
                                        tokens.append_all(quote! {
                                           .#param_ident(#set_value.as_i64().unwrap())
                                        });
                                    } else {
                                        let i = s.parse::<i64>()?;
                                        tokens.append_all(quote! {
                                            .#param_ident(#i)
                                        });
                                    }
                                }
                                _ => {
                                    // handle set values
                                    let t = if is_set_value {
                                        let set_value = from_set_value(s);
                                        quote! { #set_value.as_str().unwrap() }
                                    } else {
                                        quote! { #s }
                                    };

                                    tokens.append_all(quote! {
                                        .#param_ident(#t)
                                    })
                                }
                            }
                        }
                        Value::Bool(ref b) => match kind {
                            TypeKind::Enum => {
                                let enum_name =
                                    syn::Ident::new(&n.to_pascal_case(), Span::call_site());
                                let variant = syn::Ident::new(
                                    &b.to_string().to_pascal_case(),
                                    Span::call_site(),
                                );
                                tokens.append_all(quote! {
                                    .#param_ident(#enum_name::#variant)
                                })
                            }
                            TypeKind::List => {
                                // TODO: _source filter can be true|false|list of strings
                                let s = b.to_string();
                                tokens.append_all(quote! {
                                    .#param_ident(&[#s])
                                })
                            }
                            _ => {
                                tokens.append_all(quote! {
                                    .#param_ident(#b)
                                });
                            }
                        },
                        Value::Number(ref i) if !i.is_f64() => match kind {
                            TypeKind::String => {
                                let s = i.to_string();
                                tokens.append_all(quote! {
                                    .#param_ident(#s)
                                })
                            }
                            TypeKind::Integer => {
                                let int = i.as_i64().unwrap() as i32;
                                tokens.append_all(quote! {
                                    .#param_ident(#int)
                                });
                            }
                            TypeKind::Float => {
                                let f = i.as_f64().unwrap() as f32;
                                tokens.append_all(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            TypeKind::Double => {
                                let f = i.as_f64().unwrap();
                                tokens.append_all(quote! {
                                    .#param_ident(#f)
                                });
                            }
                            _ => {
                                let i = i.as_i64().unwrap();
                                tokens.append_all(quote! {
                                    .#param_ident(#i)
                                });
                            }
                        },
                        Value::Number(r) if r.is_f64() => match kind {
                            TypeKind::Long | TypeKind::Number => {
                                let f = r.as_f64().unwrap();
                                tokens.append_all(quote! {
                                    .#param_ident(#f as i64)
                                });
                            }
                            _ => {
                                let f = r.as_f64().unwrap();
                                tokens.append_all(quote! {
                                    .#param_ident(#f)
                                });
                            }
                        },
                        Value::Sequence(arr) => {
                            // only support param string arrays
                            let result: Vec<&String> = arr
                                .iter()
                                .map(|i| match i {
                                    Value::String(s) => Ok(s),
                                    y => Err(anyhow!("unsupported array value {:?}", y)),
                                })
                                .filter_map(Result::ok)
                                .collect();

                            if n == &"expand_wildcards" {
                                let result: Vec<TokenStream> = result
                                    .iter()
                                    .map(|s| Self::generate_enum(n, s.as_str(), &ty.options))
                                    .collect_results()?;

                                tokens.append_all(quote! {
                                    .#param_ident(&[#(#result),*])
                                });
                            } else {
                                tokens.append_all(quote! {
                                    .#param_ident(&[#(#result),*])
                                });
                            }
                        }
                        _ => println!("unsupported value {:?} for param {}", v, n),
                    }
                }

                Ok(Some(tokens))
            }
        }
    }

    fn generate_parts(
        api_call: &str,
        endpoint: &ApiEndpoint,
        parts: &[(&str, &Value)],
    ) -> anyhow::Result<Option<TokenStream>> {
        // TODO: ideally, this should share the logic from EnumBuilder
        let enum_name = {
            let name = api_call.to_pascal_case().replace('.', "");
            syn::Ident::new(&format!("{}Parts", name), Span::call_site())
        };

        // Enum variants containing no URL parts where there is only a single API URL,
        // are not required to be passed in the API.
        //
        // Also, short circuit for tests where the only parts specified are null
        // e.g. security API test. It seems these should simply omit the value though...
        if parts.is_empty() || parts.iter().all(|(_, v)| v.is_null()) {
            let mut param_counts = endpoint.url.paths.iter().map(|p| p.path.params().len());

            // check there's actually a None value
            if !param_counts.any(|c| c == 0) {
                return Err(anyhow!(
                    r#"no path for "{}" API with no url parts"#,
                    api_call
                ));
            }

            return match endpoint.url.paths.len() {
                1 => Ok(None),
                _ => Ok(Some(quote!(#enum_name::None))),
            };
        }

        let path = match endpoint.url.paths.len() {
            1 => {
                let path = &endpoint.url.paths[0];
                if path.path.params().len() == parts.len() {
                    Some(path)
                } else {
                    None
                }
            }
            _ => {
                // get the matching path parts
                let matching_path_parts = endpoint
                    .url
                    .paths
                    .iter()
                    .filter(|path| {
                        let p = path.path.params();
                        if p.len() != parts.len() {
                            return false;
                        }

                        let contains =
                            parts
                                .iter()
                                .filter_map(|i| if p.contains(&i.0) { Some(()) } else { None });
                        contains.count() == parts.len()
                    })
                    .collect::<Vec<_>>();

                match matching_path_parts.len() {
                    0 => None,
                    _ => Some(matching_path_parts[0]),
                }
            }
        }
        .ok_or_else(|| {
            anyhow!(
                r#"no path for "{}" API with url parts {:?}"#,
                &api_call,
                parts
            )
        })?;

        let path_parts = path.path.params();
        let variant_name = {
            let v = path_parts
                .iter()
                .map(|k| k.to_pascal_case())
                .collect::<Vec<_>>()
                .join("");
            syn::Ident::new(&v, Span::call_site())
        };

        let part_tokens: Vec<TokenStream> = parts
            .iter()
            // don't rely on URL parts being ordered in the yaml test in the same order as specified
            // in the REST spec.
            .sorted_by(|(p, _), (p2, _)| {
                let f = path_parts.iter().position(|x| x == p).unwrap_or_default(); // unwrap_or_default() here as we can't bubble up the error from sorted_by(|| ...),
                let s = path_parts.iter().position(|x| x == p2).unwrap_or_default(); // instead it'll hit the "no url part found for..." error below in the map
                f.cmp(&s)
            })
            .map(|(p, v)| {
                let ty = path
                    .parts
                    .get(*p)
                    .ok_or_else(|| anyhow!(r#"no url part found for "{}" in {}"#, p, &path.path))?;

                match v {
                    Value::String(s) => {
                        let is_set_value = s.starts_with('$') || s.contains("${");

                        match ty.ty {
                            TypeKind::List => {
                                let values = s.split(',').map(|s| {
                                    if is_set_value {
                                        let set_value = from_set_value(s);
                                        quote! { #set_value.as_str().unwrap() }
                                    } else {
                                        quote! { #s }
                                    }
                                });
                                Ok(quote! { &[#(#values),*] })
                            }
                            TypeKind::Long => {
                                if is_set_value {
                                    let set_value = from_set_value(s);
                                    Ok(quote! { #set_value.as_i64().unwrap() })
                                } else {
                                    let l = s.parse::<i64>().unwrap();
                                    Ok(quote! { #l })
                                }
                            }
                            _ => {
                                if is_set_value {
                                    let set_value = from_set_value(s);
                                    Ok(quote! { #set_value.as_str().unwrap() })
                                } else {
                                    Ok(quote! { #s })
                                }
                            }
                        }
                    }
                    Value::Bool(b) => {
                        let s = b.to_string();
                        Ok(quote! { #s })
                    }
                    Value::Number(i) => match ty.ty {
                        TypeKind::Long => {
                            let i = i.as_u64().unwrap();
                            Ok(quote! { #i })
                        }
                        _ => {
                            let s = i.to_string();
                            Ok(quote! { #s })
                        }
                    },
                    Value::Sequence(arr) => {
                        // only support param string arrays
                        let result: Vec<_> = arr
                            .iter()
                            .map(|i| match i {
                                Value::String(s) => Ok(s),
                                y => Err(anyhow!("unsupported array value {:?}", y)),
                            })
                            .collect_results()?;

                        match ty.ty {
                            // Some APIs specify a part is a string in the REST API spec
                            // but is really a list, which is what a YAML test might pass
                            // e.g. security.get_role_mapping.
                            // see https://github.com/elastic/elasticsearch/pull/53207
                            TypeKind::String => {
                                let s = result.iter().join(",");
                                Ok(quote! { #s })
                            }
                            _ => Ok(quote! { &[#(#result),*] }),
                        }
                    }
                    _ => Err(anyhow!("unsupported value {:?}", v)),
                }
            })
            .collect_results()?;

        Ok(Some(
            quote! { #enum_name::#variant_name(#(#part_tokens),*) },
        ))
    }

    /// Creates the body function call from a YAML value.
    ///
    /// When reading a body from the YAML test, it'll be converted to a Yaml variant,
    /// usually a Hash. To get the JSON representation back requires converting
    /// back to JSON
    fn generate_body(endpoint: &ApiEndpoint, v: &Value) -> anyhow::Result<Option<TokenStream>> {
        fn nd_body(items: &[Value]) -> TokenStream {
            let items = items.iter().map(|v| {
                let json = if v.is_string() {
                    rusty_json(
                        &serde_yaml::from_str(v.as_str().unwrap())
                            .expect("string sequence item should be JSON/YAML"),
                    )
                } else {
                    rusty_json(v)
                };
                quote! { JsonBody::from(json! { #json }) }
            });
            quote! { .body(vec![ #(#items),* ]) }
        }

        match v {
            Value::Null => Ok(None),
            Value::String(s) => {
                if endpoint.supports_nd_body() {
                    let contains_newlines = s.trim_end_matches('\n').contains('\n');
                    let items = s
                        .split(if contains_newlines {
                            |c| c == '\n'
                        } else {
                            char::is_whitespace
                        })
                        .filter(|s| !s.is_empty())
                        .map(ToOwned::to_owned)
                        .map(Value::String)
                        .collect::<Vec<_>>();

                    Ok(Some(nd_body(&items)))
                } else {
                    Self::generate_body(
                        endpoint,
                        &serde_yaml::from_str(s).expect("body should be JSON/YAML"),
                    )
                }
            }
            Value::Mapping(_) => {
                let json = rusty_json(v);
                Ok(Some(quote! { .body(json!{ #json }) }))
            }
            Value::Sequence(values) if endpoint.supports_nd_body() => Ok(Some(nd_body(values))),
            _ => panic!("Unsupported body: {:?}", v),
        }
    }
}
