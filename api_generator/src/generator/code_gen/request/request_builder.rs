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
    code_gen::{typed_client, url::enum_builder::EnumBuilder, *},
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
            // normalize: the combination must not depend on path order
            m.sort();
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

                let comma_separated = param_type.ty == TypeKind::List
                    || param_overrides::param_is_comma_separated(param_name, &param_type.ty);
                let serialize_with = if comma_separated {
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

    /// The fields of a builder struct: URL parameters, common parameters,
    /// the headers, the request timeout and, when supported, the body
    fn builder_fields(
        endpoint: &ApiEndpoint,
        common_params: &BTreeMap<String, Type>,
    ) -> Vec<Field> {
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

        if endpoint.supports_body() {
            fields.push(parse_named_field!(body: Option<B>));
        }

        // Combine common fields with struct fields, sort and deduplicate
        let mut common_fields: Vec<Field> = common_params
            .iter()
            .map(Self::create_struct_field)
            .collect();
        fields.append(&mut common_fields);
        fields.sort_by(|a, b| a.ident.cmp(&b.ident));
        fields.dedup_by(|a, b| a.ident.eq(&b.ident));
        fields
    }

    /// The methods of a builder struct, sorted by name: one per URL and common
    /// parameter, `header`, `request_timeout` and, when supported, `body`.
    /// Shared by the generic builders and the typed builders wrapping them, so
    /// that both offer the same options.
    pub(crate) fn builder_fns(
        builder_ident: &syn::Ident,
        endpoint: &ApiEndpoint,
        common_params: &BTreeMap<String, Type>,
        accepts_nd_body: bool,
    ) -> Vec<ImplItem> {
        let fields = Self::builder_fields(endpoint, common_params);
        let default_fields = fields
            .iter()
            .map(|f| f.ident.as_ref().unwrap())
            .collect::<Vec<_>>();

        // collect all the functions for the builder struct
        let mut builder_fns: Vec<ImplItem> =
            endpoint.params.iter().map(Self::create_impl_fn).collect();

        builder_fns.push(Self::create_header_fn(&ident("headers")));
        builder_fns.push(Self::create_request_timeout_fn(&ident("request_timeout")));

        // add a body impl if supported
        if endpoint.supports_body() {
            let body_fn = Self::create_body_fn(builder_ident, &default_fields, accepts_nd_body);
            builder_fns.push(body_fn);
        }

        // Combine common fns with builder fns, sort and deduplicate.
        let mut common_builder_fns: Vec<ImplItem> =
            common_params.iter().map(Self::create_impl_fn).collect();
        builder_fns.append(&mut common_builder_fns);
        builder_fns.sort_by(|a, b| a.get_ident().cmp(b.get_ident()));
        builder_fns.dedup_by(|a, b| a.get_ident().eq(b.get_ident()));
        builder_fns
    }

    /// creates the AST for a builder struct
    fn create_builder_struct(
        builder_name: &str,
        endpoint: &ApiEndpoint,
        common_params: &BTreeMap<String, Type>,
        enum_builder: &EnumBuilder,
        accepts_nd_body: bool,
    ) -> TokenStream {
        let supports_body = endpoint.supports_body();
        let builder_ident = ident(builder_name);
        let (enum_ty, enum_struct, enum_impl) = enum_builder.clone().build();

        let fields = Self::builder_fields(endpoint, common_params);
        let default_fields = {
            fields
                .iter()
                .map(|f| f.ident.as_ref().unwrap())
                .collect::<Vec<_>>()
        };
        let builder_fns =
            Self::builder_fns(&builder_ident, endpoint, common_params, accepts_nd_body);

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
    /// The documentation of the method creating a builder on the root or
    /// namespace client: the API description, its link, and the hand-written
    /// markdown docs of the API when present
    fn method_doc(
        docs_dir: &Path,
        namespace_name: &str,
        name: &str,
        builder_name: &str,
        endpoint: &ApiEndpoint,
        with_markdown: bool,
    ) -> syn::Attribute {
        let api_name_for_docs = split_on_pascal_case(builder_name);

        let markdown_doc = {
            let mut path = docs_dir.to_path_buf();
            path.push("functions");
            path.push(format!("{}.{}.md", namespace_name, name));
            if with_markdown && path.exists() {
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

        match (
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
        }
    }

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

        let method_doc =
            Self::method_doc(docs_dir, namespace_name, name, builder_name, endpoint, true);

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

    /// Builds the AST of the typed counterpart of this builder, when the
    /// endpoint has a typed response model: the typed builder struct with its
    /// methods, and the method creating it on the typed root or namespace
    /// client. `None` for endpoints without a model.
    ///
    /// The typed builder wraps the generic one and delegates every option to
    /// it, except those that change the shape of the response, which it does
    /// not offer at all (see [typed_client::omitted_params]): `send` decodes
    /// the body as the model, so a request that cannot produce the model's
    /// shape must not be expressible.
    pub fn build_typed(&self) -> Option<(TokenStream, TokenStream)> {
        let model_path = self.endpoint.typed_response.as_deref()?;
        let model: syn::Type = syn::parse_str(&format!("crate::{}", model_path)).unwrap();
        let model_name = lit(model_path);
        let statuses = &self.endpoint.typed_model_statuses;
        let group = self.endpoint.full_name.as_deref().unwrap_or(self.name);
        let omitted = typed_client::omitted_params(group);

        let builder_ident = ident(self.builder_name);
        let typed_ident = ident(&format!("Typed{}", self.builder_name));
        // the generic builder and its parts, from the typed module
        let module = if self.is_root_method {
            quote!(crate)
        } else {
            let m = ident(group.split('.').next().unwrap());
            quote!(crate::#m)
        };
        let supports_body = self.endpoint.supports_body();
        let (struct_generics, impl_header) = if supports_body {
            (
                quote!(<'a, 'b, B>),
                quote!(impl<'a, 'b, B> #typed_ident<'a, 'b, B> where B: Body),
            )
        } else {
            (quote!(<'a, 'b>), quote!(impl<'a, 'b> #typed_ident<'a, 'b>))
        };

        // every option of the generic builder, delegated, except the omitted
        let methods: Vec<TokenStream> = Self::builder_fns(
            &builder_ident,
            self.endpoint,
            self.common_params,
            self.accepts_nd_body,
        )
        .into_iter()
        .filter_map(|item| match item {
            ImplItem::Fn(f) => Some(f),
            _ => None,
        })
        .filter(|f| !omitted.iter().any(|(name, _)| f.sig.ident == name))
        .map(|f| Self::delegate_fn(&typed_ident, f, self.accepts_nd_body))
        .collect();

        let api_name_for_docs = split_on_pascal_case(self.builder_name);
        let model_link = format!("[`{path}`](crate::{path})", path = model_path);
        let omitted_doc = if omitted.is_empty() {
            String::new()
        } else {
            format!(
                "\n\nOptions that change the shape of the response are not available here: {}. Use the generic [`{}`]({}::{}) builder for them.",
                omitted
                    .iter()
                    .map(|(name, fixed)| match fixed {
                        Some(v) => format!("`{}` (always `{}`)", name, v),
                        None => format!("`{}`", name),
                    })
                    .collect::<Vec<_>>()
                    .join(", "),
                self.builder_name,
                if self.is_root_method { "crate".to_string() } else { format!("crate::{}", group.split('.').next().unwrap()) },
                self.builder_name,
            )
        };
        let struct_doc = lit(&format!(
            "Typed builder for the {} API: [`send`](Self::send) returns {}.{}",
            api_name_for_docs, model_link, omitted_doc
        ));
        let statuses_doc = if statuses.is_empty() {
            String::new()
        } else {
            format!(
                ", and for {}, for which the API answers with the same body (e.g. a document that is not found)",
                statuses.iter().map(|s| format!("`{}`", s)).collect::<Vec<_>>().join(", ")
            )
        };
        let send_doc = lit(&format!(
            "Sends the request and decodes the response body as {}.\n\n\
             The body is decoded for a success status{}. Any other status is an error, whose details are available from [`Error::api_error`](crate::Error::api_error). \
             To read the status code or the headers, use the generic client instead.",
            model_link, statuses_doc
        ));

        let cfg_attr = self.endpoint.stability.outer_cfg_attr();
        let cfg_doc = stability_doc(self.endpoint.stability);
        let (deprecated_attr, _) = Deprecated::attr(&self.endpoint.deprecated);

        let builder = quote! {
            #[doc = #struct_doc]
            #cfg_doc
            #cfg_attr
            #deprecated_attr
            #[derive(Clone, Debug)]
            pub struct #typed_ident #struct_generics {
                inner: #module::#builder_ident #struct_generics,
                report_unknown_fields: bool,
            }

            #cfg_attr
            #[allow(deprecated)]
            #impl_header {
                #(#methods)*

                #[doc = #send_doc]
                pub async fn send(self) -> Result<#model, Error> {
                    self.inner
                        .send()
                        .await?
                        .typed(#model_name, &[#(#statuses),*], self.report_unknown_fields)
                        .await
                }
            }
        };

        // the method creating the typed builder, with the omitted options that
        // have a fixed value set on the generic builder
        let fixed = omitted.iter().filter_map(|(name, fixed)| {
            fixed.map(|v| {
                let name = ident(name);
                quote!(.#name(#v))
            })
        });
        let inner_new = if self.enum_builder.contains_single_parameterless_part() {
            quote!(#module::#builder_ident::new(self.transport()) #(#fixed)*)
        } else {
            quote!(#module::#builder_ident::new(self.transport(), parts) #(#fixed)*)
        };
        let fn_ident = ident(self.name);
        let (fn_name, ret) = match (supports_body, self.is_root_method) {
            (true, true) => (quote!(#fn_ident<'a, 'b>), quote!(#typed_ident<'a, 'b, ()>)),
            (false, true) => (quote!(#fn_ident<'a, 'b>), quote!(#typed_ident<'a, 'b>)),
            (true, false) => (quote!(#fn_ident<'b>), quote!(#typed_ident<'a, 'b, ()>)),
            (false, false) => (quote!(#fn_ident<'b>), quote!(#typed_ident<'a, 'b>)),
        };
        let method_doc = Self::method_doc(
            self.docs_dir,
            self.namespace_name,
            self.name,
            self.builder_name,
            self.endpoint,
            false,
        );
        let params = if self.enum_builder.contains_single_parameterless_part() {
            quote!()
        } else {
            let (enum_ty, _, _) = self.enum_builder.clone().build();
            quote!(, parts: #module::#enum_ty)
        };
        let ctor = quote! {
            #method_doc
            #cfg_doc
            #cfg_attr
            #deprecated_attr
            #[allow(deprecated)]
            pub fn #fn_name(&'a self #params) -> #ret {
                #typed_ident {
                    inner: #inner_new,
                    report_unknown_fields: self.reports_unknown_fields(),
                }
            }
        };

        Some((builder, ctor))
    }

    /// A method of a typed builder delegating to the same method of the
    /// wrapped generic builder: same signature and docs, rewrapping the
    /// result. `body` changes the body type parameter of the builder.
    fn delegate_fn(
        typed_ident: &syn::Ident,
        f: syn::ImplItemFn,
        accepts_nd_body: bool,
    ) -> TokenStream {
        let attrs = &f.attrs;
        let name = &f.sig.ident;
        if name == "body" {
            let (fn_arg, body_ty, bound): (syn::Type, syn::Type, syn::Type) = if accepts_nd_body {
                (
                    parse_quote!(Vec<T>),
                    parse_quote!(NdBody<T>),
                    parse_quote!(Body),
                )
            } else {
                (
                    parse_quote!(T),
                    parse_quote!(JsonBody<T>),
                    parse_quote!(Serialize),
                )
            };
            return quote! {
                #(#attrs)*
                pub fn body<T>(self, body: #fn_arg) -> #typed_ident<'a, 'b, #body_ty>
                where T: #bound {
                    #typed_ident {
                        inner: self.inner.body(body),
                        report_unknown_fields: self.report_unknown_fields,
                    }
                }
            };
        }
        let mut sig = f.sig.clone();
        if let Some(syn::FnArg::Receiver(receiver)) = sig.inputs.first_mut() {
            receiver.mutability = None;
        }
        let args: Vec<&syn::Ident> = f
            .sig
            .inputs
            .iter()
            .filter_map(|arg| match arg {
                syn::FnArg::Typed(t) => match &*t.pat {
                    syn::Pat::Ident(p) => Some(&p.ident),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        quote! {
            #(#attrs)*
            pub #sig {
                Self {
                    inner: self.inner.#name(#(#args),*),
                    report_unknown_fields: self.report_unknown_fields,
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn api() -> crate::generator::Api {
        let path = concat!(env!("CARGO_MANIFEST_DIR"), "/opensearch-openapi.yaml");
        crate::openapi::read_api("test", Path::new(path)).unwrap()
    }

    /// The typed builder and the typed client method of an endpoint, as text
    fn typed(
        api: &crate::generator::Api,
        group: &str,
        endpoint: &ApiEndpoint,
    ) -> Option<(String, String)> {
        let (namespace, name, builder, root) = match group.split_once('.') {
            Some((ns, name)) => (
                ns.to_pascal_case(),
                name,
                format!("{}{}", ns.to_pascal_case(), name.to_pascal_case()),
                false,
            ),
            None => (
                "OpenSearch".to_string(),
                group,
                group.to_pascal_case(),
                true,
            ),
        };
        RequestBuilder::new(
            Path::new("/nonexistent"),
            &namespace,
            name,
            &builder,
            &api.common_params,
            endpoint,
            root,
        )
        .build_typed()
        .map(|(b, c)| (b.to_string(), c.to_string()))
    }

    #[test]
    fn typed_builders_return_the_model_and_omit_shape_changing_options() {
        let api = api();
        let get = api.endpoint_for_api_call("get").unwrap();
        let (builder, ctor) = typed(&api, "get", get).unwrap();

        // a wrapper of the generic builder whose send returns the model
        assert!(
            builder
                .contains("pub struct TypedGet < 'a , 'b > { inner : crate :: Get < 'a , 'b > , report_unknown_fields : bool , }"),
            "{}",
            builder
        );
        assert!(
            builder.contains("Result < crate :: types :: get :: GetResponse , Error >"),
            "{}",
            builder
        );
        // 404 answers with the model (`found: false`)
        assert!(builder.contains("& [404u16]"), "{}", builder);
        // the options of the generic builder are delegated...
        assert!(builder.contains("pub fn routing (self , routing : & 'b [& 'b str]) -> Self { Self { inner : self . inner . routing (routing) , report_unknown_fields : self . report_unknown_fields , } }"), "{}", builder);
        // the client's unknown-field setting reaches send
        assert!(
            builder.contains("self . report_unknown_fields) . await"),
            "{}",
            builder
        );
        assert!(
            ctor.contains("report_unknown_fields : self . reports_unknown_fields ()"),
            "{}",
            ctor
        );
        // ...except those that change the shape of the response
        assert!(!builder.contains("fn filter_path"), "{}", builder);
        assert!(ctor.contains("pub fn get < 'a , 'b > (& 'a self , parts : crate :: GetParts < 'b >) -> TypedGet < 'a , 'b >"), "{}", ctor);

        // the body changes the body type of the typed builder
        let search = api.endpoint_for_api_call("search").unwrap();
        let (builder, _) = typed(&api, "search", search).unwrap();
        assert!(
            builder.contains("-> TypedSearch < 'a , 'b , JsonBody < T > >"),
            "{}",
            builder
        );

        // no model, no typed builder
        let exists = api.endpoint_for_api_call("indices.exists").unwrap();
        assert!(typed(&api, "indices.exists", exists).is_none());
    }

    #[test]
    fn typed_cat_builders_always_ask_for_json() {
        // no cat API is typed yet: give one a model to check the output
        let api = api();
        let mut cat_indices = api.endpoint_for_api_call("cat.indices").unwrap().clone();
        assert_eq!(cat_indices.typed_response, None);
        cat_indices.typed_response = Some("types::cat_indices::IndicesResponse".to_string());
        let (builder, ctor) = typed(&api, "cat.indices", &cat_indices).unwrap();

        // the format option is not offered, and fixed to JSON on creation
        assert!(!builder.contains("fn format"), "{}", builder);
        assert!(ctor.contains("crate :: cat :: CatIndices :: new (self . transport () , parts) . format (\"json\")"), "{}", ctor);
        assert!(
            ctor.contains("parts : crate :: cat :: CatIndicesParts < 'b >"),
            "{}",
            ctor
        );
    }
}
