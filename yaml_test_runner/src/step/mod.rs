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

use anyhow::anyhow;
use api_generator::generator::Api;
use itertools::Itertools;
use proc_macro2::TokenStream;
use serde_yaml::Value;
use std::fmt::Write;

mod comparison;
mod contains;
mod r#do;
mod is_false;
mod is_true;
mod length;
mod r#match;
mod set;
mod skip;
mod transform_and_set;
pub use comparison::{Comparison, OPERATORS};
pub use contains::*;
pub use is_false::*;
pub use is_true::*;
pub use length::*;
pub use r#do::*;
pub use r#match::*;
pub use set::*;
pub use skip::*;
pub use transform_and_set::*;

pub fn parse_steps(api: &Api, steps: &[Value]) -> anyhow::Result<Vec<Step>> {
    let mut parsed_steps: Vec<Step> = Vec::new();
    for step in steps {
        let hash = step
            .as_mapping()
            .ok_or_else(|| anyhow!("expected hash but found {:?}", step))?;

        let (key, value) = {
            let (k, yaml) = hash.iter().next().unwrap();
            let key = k
                .as_str()
                .ok_or_else(|| anyhow!("expected string key but found {:?}", k))?;

            (key, yaml)
        };

        match key {
            "skip" => {
                let skip = Skip::try_parse(value)?;
                parsed_steps.push(skip.into());
            }
            "do" => {
                let d = Do::try_parse(api, value)?;
                parsed_steps.push(d.into())
            }
            "set" => {
                let s = Set::try_parse(value)?;
                parsed_steps.push(s.into());
            }
            "transform_and_set" => {
                let t = TransformAndSet::try_parse(value)?;
                parsed_steps.push(t.into());
            }
            "match" => {
                let m = Match::try_parse(value)?;
                parsed_steps.push(m.into());
            }
            "contains" => {
                let c = Contains::try_parse(value)?;
                parsed_steps.push(c.into());
            }
            "is_true" => {
                let e = IsTrue::try_parse(value)?;
                parsed_steps.push(e.into())
            }
            "is_false" => {
                let e = IsFalse::try_parse(value)?;
                parsed_steps.push(e.into())
            }
            "length" => {
                let l = Length::try_parse(value)?;
                parsed_steps.push(l.into())
            }
            op if OPERATORS.contains(&op) => {
                let comp = Comparison::try_parse(value, op)?;
                parsed_steps.push(comp.into())
            }
            op => return Err(anyhow!("unknown step operation: {}", op)),
        }
    }

    Ok(parsed_steps)
}

/// An expression to apply to the response. Can be the whole body ($body or "") or an
/// indexer expression into a JSON response.
pub struct Expr {
    expr: String,
}

impl From<&str> for Expr {
    fn from(s: &str) -> Self {
        Expr::new(s)
    }
}

impl Expr {
    pub fn new<S: Into<String>>(expr: S) -> Self {
        Self { expr: expr.into() }
    }

    /// Whether the expression is "$body" or "", which are both used to express the whole body
    pub fn is_body(&self) -> bool {
        Self::is_string_body(&self.expr) || self.expr.is_empty()
    }

    fn is_string_body(s: &str) -> bool {
        s == "$body"
    }

    pub fn expression(&self) -> TokenStream {
        if self.is_body() {
            syn::parse_str(&self.expr).unwrap()
        } else {
            let mut values = Vec::new();
            let mut value = String::new();
            let mut chars = self.expr.chars();
            while let Some(ch) = chars.next() {
                match ch {
                    '\\' => {
                        // consume the next character too
                        if let Some(next) = chars.next() {
                            value.push(next);
                        }
                    }
                    '.' => {
                        values.push(value);
                        value = String::new();
                    }
                    _ => {
                        value.push(ch);
                    }
                }
            }
            values.push(value);

            // some APIs specify the response body as the first part of the path
            // which should be removed.
            // some tests start the json path with a dot, leading to an empty first element
            if Self::is_string_body(values[0].as_ref()) || values[0].is_empty() {
                values.remove(0);
            }

            let mut expr = String::new();
            for s in values {
                if s.is_empty() {
                    write!(expr, "[\"\"]").unwrap();
                } else if s.chars().all(char::is_numeric) {
                    write!(expr, ".n({})", s).unwrap();
                } else if s.starts_with('$') {
                    // handle "set" values
                    let t = s
                        .trim_start_matches('$')
                        .trim_start_matches('{')
                        .trim_end_matches('}');
                    write!(expr, "[{}.as_str().unwrap()]", t).unwrap();
                } else if s.as_str() == "_arbitrary_key_" {
                    // handle _arbitrary_key_.
                    // wrap in Value::String to allow uniform unwrapping in subsequent steps
                    write!(
                        expr,
                        ".as_object().unwrap().iter().next().map(|(k, _)| json!(k)).unwrap()"
                    )
                    .unwrap();
                } else {
                    write!(expr, "[\"{}\"]", s).unwrap();
                }
            }
            syn::parse_str(&expr).unwrap()
        }
    }
}

/// Steps defined in a yaml test
pub enum Step {
    Skip(Skip),
    Set(Set),
    Do(Do),
    Match(Match),
    Length(Length),
    IsTrue(IsTrue),
    IsFalse(IsFalse),
    Comparison(Comparison),
    Contains(Contains),
    TransformAndSet(TransformAndSet),
}

impl Step {
    /// Gets a Do step
    pub fn as_do(&self) -> Option<&Do> {
        match self {
            Step::Do(d) => Some(d),
            _ => None,
        }
    }

    pub fn as_skip(&self) -> Option<&Skip> {
        match self {
            Step::Skip(s) => Some(s),
            _ => None,
        }
    }
}

pub trait ResultIterExt<T>: Iterator<Item = anyhow::Result<T>> + Sized {
    fn collect_results(self) -> anyhow::Result<Vec<T>> {
        let (oks, errs): (Vec<_>, Vec<_>) = self.partition_result();

        if errs.is_empty() {
            Ok(oks)
        } else {
            let mut msgs = errs.iter().map(|e| e.to_string()).collect::<Vec<_>>();
            msgs.sort();
            msgs.dedup_by(|a, b| a == b);
            Err(anyhow!("{}", msgs.join(", ")))
        }
    }
}

impl<T, I> ResultIterExt<T> for I where I: Iterator<Item = anyhow::Result<T>> + Sized {}
