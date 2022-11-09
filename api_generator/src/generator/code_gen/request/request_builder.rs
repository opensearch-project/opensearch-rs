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
use crate::generator::{
    code_gen::{url::enum_builder::EnumBuilder, *},
    ApiEndpoint, Deprecated, HttpMethod, Type, TypeKind,
};
use inflector::Inflector;
use quote::ToTokens;
use reqwest::Url;
use std::{collections::BTreeMap, fs, path::Path, str};
use syn::{parse::Parser, Field, FieldValue, ImplItem};

macro_rules! parse_named_field {
    ($($tt:tt)*) => {
        Field::parse_named.parse2(quote!($($tt)*)).unwrap()
    };
}

/// Builder that generates the AST for a request builder struct
pub struct RequestBuilder<'a> {
    /// Path to markdown docs that may be combined with generated docs
    docs_dir: &'a Path,
    /// The namespace of the API
    namespace_name: &'a str,
    /// The name of the API to which the generated struct relates
    name: &'a str,
    /// The name of the generated struct
    builder_name: &'a str,
    /// Parameters that are common to all APIs
    common_params: &'a BTreeMap<String, Type>,
    /// The endpoint to which the API relates
    endpoint: &'a ApiEndpoint,
    /// The builder for the Url parts enum related to the API
    enum_builder: EnumBuilder<'a>,
    /// Whether the API exists on the root client or on a namespace client
    is_root_method: bool,
    /// Whether the API accepts a newline delimited body
    accepts_nd_body: bool,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(
        docs_dir: &'a Path,
        namespace_name: &'a str,
        name: &'a str,
        builder_name: &'a str,
        common_params: &'a BTreeMap<String, Type>,
        endpoint: &'a ApiEndpoint,
        is_root_method: bool,
    ) -> Self {
        let mut enum_builder = EnumBuilder::new(builder_name.to_pascal_case().as_ref());
        for path in &endpoint.url.paths {
            enum_builder = enum_builder.with_path(path);
        }

        let accepts_nd_body = endpoint.supports_nd_body();

        RequestBuilder {
            docs_dir,
            namespace_name,
            name,
            builder_name,
            common_params,
            endpoint,
            enum_builder,
            is_root_method,
            accepts_nd_body,
        }
    }

    /// Create the AST for an expression that assigns a HttpMethod value
    fn create_method_expression(builder_name: &str, endpoint: &ApiEndpoint) -> syn::Expr {
        let methods = {
            let mut m = Vec::new();
            for path in &endpoint.url.paths {
                for method in &path.methods {
                    if !m.contains(&method) {
                        m.push(method);
                    }
                }
            }
            m
        };

        match methods.len() {
            1 => {
                let method = *methods.first().unwrap();
                let mut tokens = TokenStream::new();
                method.to_tokens(&mut tokens);
                parse_expr(tokens)
            }
            _ => match methods.as_slice() {
                [HttpMethod::Post, HttpMethod::Put] => {
                    if builder_name.contains("Put") {
                        parse_expr(quote!(Method::Put))
                    } else {
                        parse_expr(quote!(Method::Post))
                    }
                }
                [HttpMethod::Get, HttpMethod::Post] => parse_expr(quote!(match self.body {
                    Some(_) => Method::Post,
                    None => Method::Get,
                })),
                _ => panic!("Combination of methods unexpected"),
            },
        }
    }

    /// Create the AST for an expression that builds a struct of query string parameters
    fn create_query_string_expression(endpoint_params: &BTreeMap<String, Type>) -> TokenStream {
        if endpoint_params.is_empty() {
            quote!(None::<()>)
        } else {
            let query_struct_ty = ident("QueryParams");
            let struct_fields = endpoint_params.iter().map(|(param_name, param_type)| {
                let field = Self::create_struct_field((param_name, param_type));

                let renamed = field.ident.as_ref().unwrap() != param_name;
                let serde_rename = if renamed {
                    let field_rename = lit(param_name);
                    quote! {
                        #[serde(rename = #field_rename)]
                    }
                } else {
                    quote!()
                };

                // TODO: we special case expand_wildcards here to be a list, but this should be fixed upstream
                let expand = param_type.ty == TypeKind::List || param_name == "expand_wildcards";
                let serialize_with = if expand {
                    quote! {
                        #[serde(serialize_with = "crate::client::serialize_coll_qs")]
                    }
                } else {
                    quote!()
                };

                quote! {
                    #serde_rename
                    #serialize_with
                    #field
                }
            });

            let query_ctor = endpoint_params.iter().map(|(param_name, _)| {
                let field_name = ident(&valid_name(param_name).to_lowercase());
                quote! {
                    #field_name: self.#field_name
                }
            });

            quote! {
                {
                    #[serde_with::skip_serializing_none]
                    #[derive(Serialize)]
                    struct #query_struct_ty<'b> {
                        #(#struct_fields,)*
                    }
                    let query_params = #query_struct_ty {
                        #(#query_ctor,)*
                    };
                    Some(query_params)
                }
            }
        }
    }

    /// Creates the AST for a ctor new fn for a builder struct
    fn create_new_fn(
        builder_name: &str,
        builder_ident: &syn::Ident,
        enum_builder: &EnumBuilder,
        default_fields: &[&syn::Ident],
    ) -> TokenStream {
        let (enum_ty, _, _) = enum_builder.clone().build();
        let default_fields = Self::create_default_fields(default_fields);

        // default cat APIs to using text/plain Content-Type and Accept headers. Not all
        // headers are added here, but instead are added in Transport::send, because
        // we want to apply default headers when send is called directly, which may
        // be the case for experimental and beta APIs
        let headers = {
            if builder_name.starts_with("Cat") {
                quote! {
                    let mut headers = HeaderMap::with_capacity(2);
                    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
                    headers.insert(ACCEPT, HeaderValue::from_static("text/plain"));
                }
            } else {
                quote! {let headers = HeaderMap::new();}
            }
        };

        if enum_builder.contains_single_parameterless_part() {
            let doc = doc(&format!("Creates a new instance of [{}]", &builder_name));
            quote!(
                #doc
                pub fn new(transport: &'a Transport) -> Self {
                    #headers

                    #builder_ident {
                        transport,
                        parts: #enum_ty::None,
                        headers,
                        #(#default_fields),*,
                    }
                }
            )
        } else {
            let doc = doc(&format!(
                "Creates a new instance of [{}] with the specified API parts",
                &builder_name
            ));
            quote!(
                #doc
                pub fn new(transport: &'a Transport, parts: #enum_ty) -> Self {
                    #headers

                    #builder_ident {
                        transport,
                        parts,
                        headers,
                        #(#default_fields),*,
                    }
                }
            )
        }
    }

    /// Creates the AST for the body fn for a builder struct that supports sending a body
    fn create_body_fn(
        builder_ident: &syn::Ident,
        default_fields: &[&syn::Ident],
        accepts_nd_body: bool,
    ) -> syn::ImplItem {
        let fields = default_fields
            .iter()
            .filter(|&&part| part != &ident("body"))
            .map::<syn::FieldValue, _>(|&part| parse_quote!(#part: self.#part));

        let (fn_arg, field_arg, body_ty, t_constraint): (
            syn::Type,
            TokenStream,
            syn::Type,
            syn::Type,
        ) = if accepts_nd_body {
            (
                parse_quote!(Vec<T>),
                quote!(Some(NdBody(body))),
                parse_quote!(NdBody<T>),
                parse_quote!(Body),
            )
        } else {
            (
                parse_quote!(T),
                quote!(Some(body.into())),
                parse_quote!(JsonBody<T>),
                parse_quote!(Serialize),
            )
        };

        parse_quote! {
            #[doc = "The body for the API call"]
            pub fn body<T>(self, body: #fn_arg) -> #builder_ident<'a, 'b, #body_ty>
            where T: #t_constraint {
                #builder_ident {
                    transport: self.transport,
                    parts: self.parts,
                    body: #field_arg,
                    #(#fields),*,
                }
            }
        }
    }

    /// Creates the AST for a builder fn to add a HTTP header
    fn create_header_fn(field: &syn::Ident) -> syn::ImplItem {
        parse_quote! {
            #[doc = "Adds a HTTP header"]
            pub fn header(mut self, key: HeaderName, value: HeaderValue) -> Self {
                self.#field.insert(key, value);
                self
            }
        }
    }

    /// Creates the AST for a builder fn to add a request timeout
    fn create_request_timeout_fn(field: &syn::Ident) -> syn::ImplItem {
        parse_quote! {
            #[doc = "Sets a request timeout for this API call.\n\nThe timeout is applied from when the request starts connecting until the response body has finished."]
            pub fn request_timeout(mut self, timeout: Duration) -> Self {
                self.#field = Some(timeout);
                self
            }
        }
    }

    /// Creates the AST for a builder fn for a builder impl
    fn create_impl_fn(f: (&String, &Type)) -> syn::ImplItem {
        let ident = ident(&valid_name(f.0).to_lowercase());

        let (ty, value, fn_generics) = {
            let ty = typekind_to_ty(f.0, &f.1.ty, true, true);
            match ty {
                syn::Type::Path(ref p) => {
                    if p.get_ident() == "Into" {
                        let ty = parse_quote!(T);
                        let value = quote!(#ident.into());
                        let generics = parse_quote!(<T: #p>);
                        (ty, value, generics)
                    } else {
                        (ty, quote!(#ident), generics_none())
                    }
                }
                _ => (ty, quote!(#ident), generics_none()),
            }
        };

        let doc_attr = f.1.description.as_ref().map(doc_escaped);
        let (deprecated_attr, _) = Deprecated::attr(&f.1.deprecated);

        parse_quote! {
            #doc_attr
            #deprecated_attr
            pub fn #ident #fn_generics (mut self, #ident: #ty) -> Self {
                self.#ident = Some(#value);
                self
            }
        }
    }

    /// creates the AST for a builder struct
    fn create_builder_struct(
        builder_name: &str,
        endpoint: &ApiEndpoint,
        common_params: &BTreeMap<String, Type>,
        enum_builder: &EnumBuilder,
        accepts_nd_body: bool,
    ) -> TokenStream {
        let mut common_fields: Vec<Field> = common_params
            .iter()
            .map(Self::create_struct_field)
            .collect();

        let mut common_builder_fns: Vec<ImplItem> =
            common_params.iter().map(Self::create_impl_fn).collect();

        let supports_body = endpoint.supports_body();
        let builder_ident = ident(builder_name);
        let (enum_ty, enum_struct, enum_impl) = enum_builder.clone().build();

        // collect all the fields for the builder struct. Start with url parameters
        let mut fields: Vec<Field> = endpoint
            .params
            .iter()
            .map(Self::create_struct_field)
            .collect();

        let headers_field_ident = ident("headers");
        let request_timeout_ident = ident("request_timeout");

        // add a field for HTTP headers
        fields.push(parse_named_field!(#headers_field_ident: HeaderMap));
        fields.push(parse_named_field!(#request_timeout_ident: Option<Duration>));

        if supports_body {
            fields.push(parse_named_field!(body: Option<B>));
        }

        // Combine common fields with struct fields, sort and deduplicate
        fields.append(&mut common_fields);
        fields.sort_by(|a, b| a.ident.cmp(&b.ident));
        fields.dedup_by(|a, b| a.ident.eq(&b.ident));

        let default_fields = {
            fields
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>()
        };

        // collect all the functions for the builder struct
        let mut builder_fns: Vec<ImplItem> =
            endpoint.params.iter().map(Self::create_impl_fn).collect();

        builder_fns.push(Self::create_header_fn(&headers_field_ident));
        builder_fns.push(Self::create_request_timeout_fn(&request_timeout_ident));

        // add a body impl if supported
        if supports_body {
            let body_fn = Self::create_body_fn(&builder_ident, &default_fields, accepts_nd_body);
            builder_fns.push(body_fn);
        }

        // Combine common fns with builder fns, sort and deduplicate.
        builder_fns.append(&mut common_builder_fns);
        builder_fns.sort_by(|a, b| a.get_ident().cmp(b.get_ident()));
        builder_fns.dedup_by(|a, b| a.get_ident().eq(b.get_ident()));

        let new_fn =
            Self::create_new_fn(builder_name, &builder_ident, enum_builder, &default_fields);

        let method_expr = Self::create_method_expression(builder_name, endpoint);

        let query_string_params = {
            let mut p = endpoint.params.clone();
            p.append(&mut common_params.clone());
            p
        };

        let query_string_expr = Self::create_query_string_expression(&query_string_params);

        let body_expr = {
            if supports_body {
                quote!(self.body)
            } else {
                quote!(Option::<()>::None)
            }
        };

        let (builder_expr, builder_impl) = {
            if supports_body {
                (
                    quote!(#builder_ident<'a, 'b, B>),
                    quote!(impl<'a, 'b, B> #builder_ident<'a, 'b, B> where B: Body),
                )
            } else {
                (
                    quote!(#builder_ident<'a, 'b>),
                    quote!(impl<'a, 'b> #builder_ident<'a, 'b>),
                )
            }
        };

        let api_name_for_docs = split_on_pascal_case(builder_name);

        let builder_doc = match (
            endpoint.documentation.description.as_ref(),
            endpoint.documentation.url.as_ref(),
        ) {
            (Some(d), Some(u)) if Url::parse(u).is_ok() => lit(&format!(
                "Builder for the [{} API]({})\n\n{}",
                api_name_for_docs, u, d
            )),
            (Some(d), None) => lit(&format!(
                "Builder for the {} API\n\n{}",
                api_name_for_docs, d
            )),
            (None, Some(u)) if Url::parse(u).is_ok() => lit(&format!(
                "Builder for the [{} API]({})",
                api_name_for_docs, u
            )),
            _ => lit(&format!("Builder for the {} API", api_name_for_docs)),
        };

        let send_doc = lit(&format!(
            "Creates an asynchronous call to the {} API that can be awaited",
            api_name_for_docs
        ));

        let cfg_attr = endpoint.stability.outer_cfg_attr();
        let cfg_doc = stability_doc(endpoint.stability);
        let (deprecated_attr, allow_deprecated_attr) = Deprecated::attr(&endpoint.deprecated);

        quote! {
            #cfg_attr
            #deprecated_attr
            #enum_struct

            #cfg_attr
            #allow_deprecated_attr
            #enum_impl

            #[doc = #builder_doc]
            #cfg_doc
            #cfg_attr
            #deprecated_attr
            #allow_deprecated_attr
            #[derive(Clone, Debug)]
            pub struct #builder_expr {
                transport: &'a Transport,
                parts: #enum_ty,
                #(#fields),*,
            }

            #cfg_attr
            #allow_deprecated_attr
            #builder_impl {
                #new_fn
                #(#builder_fns)*

                #[doc = #send_doc]
                pub async fn send(self) -> Result<Response, Error> {
                      let path = self.parts.url();
                      let method = #method_expr;
                      let headers = self.headers;
                      let timeout = self.request_timeout;
                      let query_string = #query_string_expr;
                      let body = #body_expr;
                      let response = self.transport.send(method, &path, headers, query_string.as_ref(), body, timeout).await?;
                      Ok(response)
                }
            }
        }
    }

    /// Creates the AST for a fn that returns a new instance of a builder struct
    /// from the root or namespace client
    fn create_builder_struct_ctor_fns(
        docs_dir: &Path,
        namespace_name: &str,
        name: &str,
        builder_name: &str,
        endpoint: &ApiEndpoint,
        is_root_method: bool,
        enum_builder: &EnumBuilder,
    ) -> TokenStream {
        let cfg_attr = endpoint.stability.outer_cfg_attr();
        let cfg_doc = stability_doc(endpoint.stability);
        let (deprecated_attr, allow_deprecated_attr) = Deprecated::attr(&endpoint.deprecated);

        let builder_ident = ident(builder_name);

        let (fn_name, builder_ident_ret) = {
            let i = ident(name);
            let b = builder_ident.clone();

            match (endpoint.supports_body(), is_root_method) {
                (true, true) => (quote!(#i<'a, 'b>), quote!(#b<'a, 'b, ()>)),
                (false, true) => (quote!(#i<'a, 'b>), quote!(#b<'a, 'b>)),
                (true, false) => (quote!(#i<'b>), quote!(#b<'a, 'b, ()>)),
                (false, false) => (quote!(#i<'b>), quote!(#b<'a, 'b>)),
            }
        };

        let api_name_for_docs = split_on_pascal_case(builder_name);

        let markdown_doc = {
            let mut path = docs_dir.to_path_buf();
            path.push("functions");
            path.push(format!("{}.{}.md", namespace_name, name));
            if path.exists() {
                let mut s = fs::read_to_string(&path)
                    .unwrap_or_else(|_| panic!("Could not read file at {:?}", &path));
                s = s.replace("\r\n", "\n");
                if !s.starts_with("\n\n") {
                    s.insert_str(0, "\n\n");
                }
                s
            } else {
                String::new()
            }
        };

        let method_doc = match (
            endpoint.documentation.description.as_ref(),
            endpoint.documentation.url.as_ref(),
        ) {
            (Some(d), Some(u)) if Url::parse(u).is_ok() => doc(&format!(
                "[{} API]({})\n\n{}{}",
                api_name_for_docs, u, d, markdown_doc
            )),
            (Some(d), None) => doc(&format!(
                "{} API\n\n{}{}",
                api_name_for_docs, d, markdown_doc
            )),
            (None, Some(u)) if Url::parse(u).is_ok() => doc(&format!(
                "[{} API]({}){}",
                api_name_for_docs, u, markdown_doc
            )),
            _ => doc(&format!("{} API{}", api_name_for_docs, markdown_doc)),
        };

        let clone_expr = quote!(self.transport());

        if enum_builder.contains_single_parameterless_part() {
            quote!(
                #method_doc
                #cfg_doc
                #cfg_attr
                #deprecated_attr
                #allow_deprecated_attr
                pub fn #fn_name(&'a self) -> #builder_ident_ret {
                    #builder_ident::new(#clone_expr)
                }
            )
        } else {
            let (enum_ty, _, _) = enum_builder.clone().build();
            quote!(
                #method_doc
                #cfg_doc
                #cfg_attr
                #deprecated_attr
                #allow_deprecated_attr
                pub fn #fn_name(&'a self, parts: #enum_ty) -> #builder_ident_ret {
                    #builder_ident::new(#clone_expr, parts)
                }
            )
        }
    }

    /// Creates the AST for a field for a struct
    fn create_struct_field(f: (&String, &Type)) -> syn::Field {
        let ident = ident(&valid_name(f.0).to_lowercase());
        let ty = typekind_to_ty(f.0, &f.1.ty, false, false);
        parse_named_field!(#ident: #ty)
    }

    /// Creates the AST for field values initialized with a default value.
    /// Since all default values are Option<T>, the default value for all is None
    fn create_default_fields(default_fields: &[&syn::Ident]) -> Vec<FieldValue> {
        default_fields
            .iter()
            .filter(|&&part| part != &ident("headers"))
            .map(|part| parse_quote!(#part: None))
            .collect()
    }

    /// builds the AST that represent the builder structs
    /// and the ctor function for the builder struct on the root/namespace client
    pub fn build(self) -> (TokenStream, TokenStream) {
        let builder_struct = Self::create_builder_struct(
            self.builder_name,
            self.endpoint,
            self.common_params,
            &self.enum_builder,
            self.accepts_nd_body,
        );

        let ctor_fn = Self::create_builder_struct_ctor_fns(
            self.docs_dir,
            self.namespace_name,
            self.name,
            self.builder_name,
            self.endpoint,
            self.is_root_method,
            &self.enum_builder,
        );

        (builder_struct, ctor_fn)
    }
}
