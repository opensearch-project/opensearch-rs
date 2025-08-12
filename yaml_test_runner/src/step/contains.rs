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
use crate::{rusty_json::rusty_json, step::Expr};
use anyhow::anyhow;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use serde_yaml::Value;

pub struct Contains {
    expr: Expr,
    value: Value,
}

impl From<Contains> for Step {
    fn from(contains: Contains) -> Self {
        Step::Contains(contains)
    }
}

impl Contains {
    pub fn try_parse(yaml: &Value) -> anyhow::Result<Contains> {
        let hash = yaml
            .as_mapping()
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
            Value::Number(n) => {
                if n.is_f64() {
                    let f = n.as_f64().unwrap();
                    tokens.append_all(quote! {
                        {
                            let expr = &json#expr;
                            crate::assert_contains!(expr, json!(#f));
                        }
                    });
                } else if n.is_u64() {
                    let u = n.as_u64().unwrap();
                    tokens.append_all(quote! {
                        {
                            let expr = &json#expr;
                            crate::assert_contains!(expr, json!(#u));
                        }
                    });
                } else {
                    let i = n.as_i64().unwrap();
                    tokens.append_all(quote! {
                        {
                            let expr = &json#expr;
                            crate::assert_contains!(expr, json!(#i));
                        }
                    });
                }
            }
            Value::String(s) => {
                tokens.append_all(quote! {
                    {
                        let expr = &json#expr;
                        crate::assert_contains!(expr, json!(#s));
                    }
                });
            }
            Value::Bool(b) => {
                tokens.append_all(quote! {
                    {
                        let expr = &json#expr;
                        crate::assert_contains!(expr, json!(#b));
                    }
                });
            }
            yaml if yaml.is_sequence() || yaml.is_mapping() => {
                let json = rusty_json(yaml);

                tokens.append_all(quote! {
                    {
                        let expr = &json#expr;
                        crate::assert_contains!(expr, json!(#json));
                    }
                });
            }
            yaml => {
                panic!("Bad yaml value {:?}", &yaml);
            }
        }
    }
}
