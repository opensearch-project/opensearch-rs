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

use super::Step;
use crate::step::{json_string_from_yaml, Expr};
use anyhow::anyhow;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use yaml_rust::Yaml;

pub struct Contains {
    expr: Expr,
    value: Yaml,
}

impl From<Contains> for Step {
    fn from(contains: Contains) -> Self {
        Step::Contains(contains)
    }
}

impl Contains {
    pub fn try_parse(yaml: &Yaml) -> anyhow::Result<Contains> {
        let hash = yaml
            .as_hash()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", yaml))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k.as_str().unwrap().trim();
        Ok(Contains {
            expr: expr.into(),
            value: v.clone(),
        })
    }
}

impl ToTokens for Contains {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = self.expr.expression();

        match &self.value {
            Yaml::Real(r) => {
                let f = r.parse::<f64>().unwrap();
                tokens.append_all(quote! {
                    assert_contains!(json#expr, json!(#f));
                });
            }
            Yaml::Integer(i) => {
                tokens.append_all(quote! {
                    assert_contains!(json#expr, json!(#i));
                });
            }
            Yaml::String(s) => {
                tokens.append_all(quote! {
                    assert_contains!(json#expr, json!(#s));
                });
            }
            Yaml::Boolean(b) => {
                tokens.append_all(quote! {
                    assert_contains!(json#expr, json!(#b));
                });
            }
            yaml if yaml.is_array() || yaml.as_hash().is_some() => {
                let json = syn::parse_str::<TokenStream>(&json_string_from_yaml(yaml)).unwrap();

                tokens.append_all(quote! {
                    assert_contains!(json#expr, json!(#json));
                });
            }
            yaml => {
                panic!("Bad yaml value {:?}", &yaml);
            }
        }
    }
}
