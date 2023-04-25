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
use crate::step::Expr;
use anyhow::anyhow;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use serde_yaml::Value;

pub struct Set {
    ident: syn::Ident,
    expr: Expr,
}

impl From<Set> for Step {
    fn from(set: Set) -> Self {
        Step::Set(set)
    }
}

impl Set {
    pub fn try_parse(yaml: &Value) -> anyhow::Result<Set> {
        let hash = yaml
            .as_mapping()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", yaml))?;

        let (k, v) = hash.iter().next().unwrap();
        let expr = k
            .as_str()
            .ok_or_else(|| anyhow!("expected string key but found {:?}", k))?;

        let id = v
            .as_str()
            .ok_or_else(|| anyhow!("expected string value but found {:?}", v))?;

        Ok(Set {
            ident: syn::Ident::new(id, Span::call_site()),
            expr: expr.into(),
        })
    }
}

impl ToTokens for Set {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = &self.ident;
        let expr = self.expr.expression();
        tokens.append_all(quote! {
            let #ident = json#expr.clone();
        });
    }
}
