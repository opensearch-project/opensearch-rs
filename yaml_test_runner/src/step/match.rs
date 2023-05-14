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
use crate::{regex::clean_regex, rusty_json::rusty_json, step::Expr};
use anyhow::anyhow;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens, TokenStreamExt};
use serde_yaml::Value;

pub struct Match {
    pub expr: Expr,
    value: Value,
}

impl From<Match> for Step {
    fn from(m: Match) -> Self {
        Step::Match(m)
    }
}

impl Match {
    pub fn try_parse(yaml: &Value) -> anyhow::Result<Match> {
        let hash = yaml
            .as_mapping()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", yaml))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k.as_str().unwrap().trim();
        Ok(Match {
            expr: expr.into(),
            value: v.clone(),
        })
    }
}

impl ToTokens for Match {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let expr = self.expr.expression();

        match &self.value {
            Value::String(s) => {
                if s.starts_with('/') {
                    let s = clean_regex(s);
                    if self.expr.is_body() {
                        tokens.append_all(quote! {
                            crate::assert_regex_match!(&text, #s, true);
                        });
                    } else {
                        tokens.append_all(quote! {
                            crate::assert_regex_match!(json#expr.as_str().unwrap(), #s, true);
                        });
                    }
                } else {
                    // handle set values
                    if s.starts_with('$') {
                        let t = {
                            let s = s
                                .trim_start_matches('$')
                                .trim_start_matches('{')
                                .trim_end_matches('}');
                            syn::parse_str::<TokenStream>(s).unwrap()
                        };

                        tokens.append_all(quote! {
                            crate::assert_match!(json#expr, json!(#t));
                        });
                    } else {
                        tokens.append_all(quote! {
                            crate::assert_match!(json#expr, json!(#s));
                        })
                    };
                }
            }
            Value::Number(i) => {
                if i.is_f64() {
                    let f = i.as_f64().unwrap();
                    if self.expr.is_body() {
                        panic!("match on $body with f64");
                    } else {
                        tokens.append_all(quote! {
                            crate::assert_numeric_match!(json#expr, #f);
                        });
                    }
                } else if i.is_i64() {
                    let i = i.as_i64().unwrap();
                    if self.expr.is_body() {
                        panic!("match on $body with i64");
                    } else {
                        tokens.append_all(quote! {
                            crate::assert_numeric_match!(json#expr, #i);
                        });
                    }
                } else {
                    let i = i.as_u64().unwrap();
                    if self.expr.is_body() {
                        panic!("match on $body with u64");
                    } else {
                        tokens.append_all(quote! {
                            crate::assert_numeric_match!(json#expr, #i);
                        });
                    }
                }
            }
            Value::Null => {
                if self.expr.is_body() {
                    tokens.append_all(quote! {
                        assert!(text.is_empty(), "expected response to be null (empty) but was {}", &text);
                    });
                } else {
                    tokens.append_all(quote! {
                        crate::assert_null!(json#expr);
                    });
                }
            }
            Value::Bool(b) => {
                if self.expr.is_body() {
                    panic!("match on $body with bool");
                } else {
                    tokens.append_all(quote! {
                        crate::assert_match!(json#expr, json!(#b));
                    });
                }
            }
            yaml if yaml.is_sequence() || yaml.is_mapping() => {
                let json = rusty_json(yaml);

                if self.expr.is_body() {
                    tokens.append_all(quote! {
                        crate::assert_match!(json, json!(#json));
                    });
                } else {
                    tokens.append_all(quote! {
                        crate::assert_match!(json#expr, json!(#json));
                    });
                }
            }
            yaml => {
                panic!("Bad yaml value {:?}", &yaml);
            }
        }
    }
}
