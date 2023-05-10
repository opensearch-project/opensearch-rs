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

use crate::{skip::SkippedFeaturesAndTests, step::*};
use anyhow::anyhow;
use api_generator::generator::Api;
use inflector::Inflector;
use lazy_static::lazy_static;
use log::{error, info};
use path_slash::PathExt;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use regex::Regex;
use semver::Version;
use serde::Deserialize;
use serde_yaml::Value;
use std::{
    collections::HashSet,
    fs,
    fs::{File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

/// The test suite to compile
#[derive(Debug, PartialEq, Eq)]
pub enum TestSuite {
    Free,
}

/// The components of a test file, constructed from a yaml file
struct YamlTests<'a> {
    path: String,
    version: &'a Version,
    skip: &'a SkippedFeaturesAndTests,
    #[allow(dead_code)]
    suite: TestSuite,
    directives: HashSet<String>,
    setup: Option<TestFn>,
    teardown: Option<TestFn>,
    tests: Vec<TestFn>,
}

impl<'a> YamlTests<'a> {
    pub fn new(
        path: &'a Path,
        version: &'a semver::Version,
        skip: &'a SkippedFeaturesAndTests,
        suite: TestSuite,
        len: usize,
    ) -> Self {
        let path = path.to_slash_lossy().into_owned();
        Self {
            path,
            version,
            skip,
            suite,
            directives: HashSet::with_capacity(len),
            setup: None,
            teardown: None,
            tests: Vec::with_capacity(len),
        }
    }

    /// Collects the use directives required for all steps and tests
    fn use_directives_from_steps(steps: &[Step]) -> Vec<String> {
        steps
            .iter()
            .filter_map(Step::r#do)
            .filter_map(|d| d.namespace())
            .map(|s| s.to_string())
            .collect()
    }

    /// Adds a specific setup function
    pub fn add_setup(&mut self, setup: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&setup.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.setup = Some(setup);
        self
    }

    /// Adds a specific teardown function
    pub fn add_teardown(&mut self, teardown: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&teardown.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.teardown = Some(teardown);
        self
    }

    /// Adds a test to the collection of tests
    pub fn add_test_fn(&mut self, test_fn: TestFn) -> &mut Self {
        let directives = Self::use_directives_from_steps(&test_fn.steps);
        for directive in directives {
            self.directives.insert(directive);
        }

        self.tests.push(test_fn);
        self
    }

    /// Generates the AST for the Yaml test file
    pub fn build(self) -> TokenStream {
        let (setup_fn, setup_call) = Self::generate_fixture(&self.setup);
        let (teardown_fn, teardown_call) = Self::generate_fixture(&self.teardown);
        let general_setup_call = quote!(client::general_cluster_setup().await?;);

        let tests = self.fn_impls(general_setup_call, setup_call, teardown_call);

        let directives: Vec<TokenStream> = self
            .directives
            .iter()
            .map(|n| {
                let ident = syn::Ident::new(n.as_str(), Span::call_site());
                quote!(use opensearch::#ident::*;)
            })
            .collect();

        quote! {
            #![allow(unused_imports, unused_variables, dead_code, deprecated, clippy::redundant_clone, clippy::approx_constant)]
            use crate::common::{client, ValueExt};
            use opensearch::*;
            use opensearch::http::{
                headers::{HeaderName, HeaderValue},
                request::JsonBody,
                Method,
            };
            use opensearch::params::*;
            #(#directives)*
            use ::regex;
            use serde_json::{json, Value};

            #setup_fn
            #teardown_fn
            #(#tests)*
        }
    }

    /// Whether to emit code to read the last response, as text and optionally json
    pub fn read_response(read_response: bool, tokens: &mut TokenStream) -> bool {
        if !read_response {
            tokens.append_all(quote! {
                let (method, status_code, text, json) = client::read_response(response).await?;
            });
        }

        true
    }

    /// Whether the test should be skipped
    fn should_skip_test(&self, name: &str) -> bool {
        self.skip.should_skip_test(&self.path, name)
    }

    fn fn_impls(
        &self,
        general_setup_call: TokenStream,
        setup_call: Option<TokenStream>,
        teardown_call: Option<TokenStream>,
    ) -> Vec<Option<TokenStream>> {
        let mut seen_names = HashSet::new();

        self.tests
            .iter()
            .map(|test_fn| {
                let name = test_fn.name();
                let unique_name = test_fn.unique_name(&mut seen_names);
                if self.should_skip_test(name) {
                    info!(
                        r#"skipping "{}" in {} because it's included in skip.yml"#,
                        name,
                        self.path,
                    );
                    return None;
                }

                let fn_name = syn::Ident::new(unique_name.as_str(), Span::call_site());
                let mut body = TokenStream::new();
                let mut skip : Option<String> = None;
                let mut read_response = false;

                for step in &test_fn.steps {
                    match step {
                        Step::Skip(s) => {
                            skip = if s.skip_version(self.version) {
                                let m = format!(
                                    r#"skipping "{}" in {} because version "{}" is met. {}"#,
                                    name,
                                    &self.path,
                                    s.version(),
                                    s.reason()
                                );
                                Some(m)
                            } else if s.skip_features(self.skip) {
                                let m = format!(
                                    r#"skipping "{}" in {} because it needs features "{:?}" which are currently not implemented"#,
                                    name,
                                    &self.path,
                                    s.features()
                                );
                                Some(m)
                            } else {
                                None
                            }
                        }
                        Step::Do(d) => {
                            read_response = d.to_tokens(false, &mut body);
                        }
                        Step::Match(m) => {
                            read_response = Self::read_response(read_response,&mut body);
                            m.to_tokens(&mut body);
                        }
                        Step::Set(s) => {
                            read_response = Self::read_response(read_response, &mut body);
                            s.to_tokens(&mut body);
                        }
                        Step::Length(l) => {
                            read_response = Self::read_response(read_response,&mut body);
                            l.to_tokens(&mut body);
                        },
                        Step::IsTrue(t) => {
                            read_response = Self::read_response(read_response,&mut body);
                            t.to_tokens(&mut body);
                        },
                        Step::IsFalse(f) => {
                            read_response = Self::read_response(read_response, &mut body);
                            f.to_tokens(&mut body);
                        },
                        Step::Comparison(c) => {
                            read_response = Self::read_response(read_response,&mut body);
                            c.to_tokens(&mut body);
                        },
                        Step::Contains(c) => {
                            read_response = Self::read_response(read_response,&mut body);
                            c.to_tokens(&mut body);
                        },
                        Step::TransformAndSet(t) => {
                            read_response = Self::read_response(read_response,&mut body);
                            t.to_tokens(&mut body);
                        }
                    }
                }

                match skip {
                    Some(s) => {
                        info!("{}", s);
                        None
                    },
                    None => Some(quote! {
                        #[tokio::test]
                        async fn #fn_name() -> anyhow::Result<()> {
                            let client = client::get();
                            #general_setup_call
                            #setup_call
                            #body
                            #teardown_call
                            Ok(())
                        }
                    }),
                }
            })
            .collect()
    }

    /// Generates the AST for the fixture fn and its invocation
    fn generate_fixture(test_fn: &Option<TestFn>) -> (Option<TokenStream>, Option<TokenStream>) {
        if let Some(t) = test_fn {
            let ident = syn::Ident::new(t.name.as_str(), Span::call_site());

            // TODO: collect up the do calls for now. We do also need to handle skip, etc.
            let tokens = t
                .steps
                .iter()
                .filter_map(Step::r#do)
                .map(|d| {
                    let mut tokens = TokenStream::new();
                    ToTokens::to_tokens(d, &mut tokens);
                    tokens
                })
                .collect::<Vec<_>>();

            (
                Some(quote! {
                    async fn #ident(client: &OpenSearch) -> anyhow::Result<()> {
                        #(#tokens)*
                        Ok(())
                    }
                }),
                Some(quote! { #ident(client).await?; }),
            )
        } else {
            (None, None)
        }
    }
}

/// A test function
struct TestFn {
    name: String,
    steps: Vec<Step>,
}

impl TestFn {
    pub fn new<S: Into<String>>(name: S, steps: Vec<Step>) -> Self {
        Self {
            name: name.into(),
            steps,
        }
    }

    /// The function name as declared in yaml
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// some function descriptions are the same in YAML tests, which would result in
    /// duplicate generated test function names. Deduplicate by appending incrementing number
    pub fn unique_name(&self, seen_names: &mut HashSet<String>) -> String {
        let mut fn_name = self.name.replace(' ', "_").to_lowercase().to_snake_case();
        while !seen_names.insert(fn_name.clone()) {
            lazy_static! {
                static ref ENDING_DIGITS_REGEX: Regex = Regex::new(r"^(.*?)_(\d*?)$").unwrap();
            }
            if let Some(c) = ENDING_DIGITS_REGEX.captures(&fn_name) {
                let name = c.get(1).unwrap().as_str();
                let n = c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                fn_name = format!("{}_{}", name, n + 1);
            } else {
                fn_name.push_str("_2");
            }
        }
        fn_name
    }
}

pub fn generate_tests_from_yaml(
    api: &Api,
    _suite: &TestSuite,
    version: &semver::Version,
    base_download_dir: &Path,
    download_dir: &Path,
    generated_dir: &Path,
    skips: &SkippedFeaturesAndTests,
) -> anyhow::Result<()> {
    let paths = fs::read_dir(download_dir)?;
    for entry in paths.flatten() {
        if let Ok(file_type) = entry.file_type() {
            if file_type.is_dir() {
                generate_tests_from_yaml(
                    api,
                    _suite,
                    version,
                    base_download_dir,
                    &entry.path(),
                    generated_dir,
                    skips,
                )?;
            } else if file_type.is_file() {
                let path = entry.path();
                // skip non-yaml files
                let extension = path.extension().unwrap_or_else(|| "".as_ref());
                if extension != "yml" && extension != "yaml" {
                    continue;
                }

                let relative_path = path.strip_prefix(base_download_dir)?;
                let test_suite = TestSuite::Free;

                info!("Generating: {}", relative_path.display());
                let yaml = fs::read_to_string(&entry.path()).unwrap();

                let docs = match serde_yaml::Deserializer::from_str(&yaml)
                    .map(Value::deserialize)
                    .collect::<Result<Vec<_>, _>>()
                {
                    Ok(docs) => docs,
                    Err(err) => {
                        error!(
                            "skipping {}. contains one or more malformed YAML documents: {}",
                            relative_path.display(),
                            err
                        );
                        continue;
                    }
                };

                let mut test =
                    YamlTests::new(relative_path, version, skips, test_suite, docs.len());

                let result = docs
                        .iter()
                        .map(|doc| {
                            let hash = doc
                                .as_mapping()
                                .ok_or_else(|| anyhow!(
                                    "expected hash but found {:?}",
                                    &doc
                                ))?;

                            let (key, value) = hash.iter().next().unwrap();
                            match (key, value) {
                                (Value::String(name), Value::Sequence(steps)) => {
                                    let steps = parse_steps(api, steps)?;
                                    let test_fn = TestFn::new(name, steps);
                                    match name.as_str() {
                                        "setup" => test.add_setup(test_fn),
                                        "teardown" => test.add_teardown(test_fn),
                                        _ => test.add_test_fn(test_fn),
                                    };
                                    Ok(())
                                }
                                (k, v) => {
                                    Err(anyhow!(
                                        "expected string key and array value in {:?}, but found {:?} and {:?}",
                                        relative_path,
                                        &k,
                                        &v,
                                    ))
                                }
                            }
                        })
                        .collect_results();

                //if there has been an Err in any step of the yaml test file, don't create a test for it
                match result {
                    Ok(_) => write_test_file(test, relative_path, generated_dir)?,
                    Err(e) => {
                        info!("skipping {} because {}", relative_path.to_slash_lossy(), e)
                    }
                }
            }
        }
    }

    write_mod_files(generated_dir, true)?;

    Ok(())
}

/// Writes a mod.rs file in each generated directory
fn write_mod_files(generated_dir: &Path, toplevel: bool) -> anyhow::Result<()> {
    if !generated_dir.exists() {
        fs::create_dir(generated_dir)?;
    }

    let paths = fs::read_dir(generated_dir)?;
    let mut mods = vec![];
    for entry in paths.flatten() {
        let path = entry.path();
        let name = path.file_stem().unwrap().to_string_lossy();

        if name != "mod" {
            mods.push(format!(
                "pub mod {};",
                path.file_stem().unwrap().to_string_lossy()
            ));
        }

        if path.is_dir() && !(toplevel && name == "common") {
            write_mod_files(&entry.path(), false)?;
        }
    }

    // Make sure we have a stable output
    mods.sort();

    let path = generated_dir.join("mod.rs");
    let mut file = File::create(&path)?;
    let generated_mods: String = mods.join("\n");
    file.write_all(generated_mods.as_bytes())?;
    Ok(())
}

fn test_file_path(relative_path: &Path) -> anyhow::Result<PathBuf> {
    let mut relative = relative_path.to_path_buf();
    relative.set_extension("");
    // directories and files will form the module names so ensure they're valid module names
    let clean: String = relative.to_string_lossy().replace(['.', '-'], "_");

    relative = PathBuf::from(clean);

    let file_name = relative.file_name().unwrap().to_string_lossy().into_owned();
    // modules can't start with a number so prefix with underscore
    if file_name.starts_with(char::is_numeric) {
        relative.set_file_name(format!("_{}", file_name));
    }

    Ok(relative)
}

fn write_test_file(
    test: YamlTests,
    relative_path: &Path,
    generated_dir: &Path,
) -> anyhow::Result<()> {
    if test.should_skip_test("*") {
        info!(
            r#"skipping all tests in {} because it's included in skip.yml"#,
            test.path,
        );
        return Ok(());
    }

    let mut path = test_file_path(relative_path)?;
    path = generated_dir.join(path);
    path.set_extension("rs");

    fs::create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(&path)?;
    file.write_all(
        r#"/*
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
// -----------------------------------------------
// This file is generated, please do not edit it manually.
// Run the following in the root of the repo:
//
// cargo run -p yaml_test_runner -- --branch <branch> --token <token> --path <rest specs path>
// -----------------------------------------------
"#
        .as_bytes(),
    )?;

    let tokens = test.build();
    let generated = tokens.to_string();
    let mut file = OpenOptions::new().append(true).open(&path)?;
    file.write_all(generated.as_bytes())?;
    file.write_all(b"\n")?;

    Ok(())
}
