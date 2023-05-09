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

use ::regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // replace usages of "$.*" with the captured value
    pub static ref SET_REGEX: Regex =
        Regex::new(r#""\$(.*?)""#).unwrap();

    // replace usages of "${.*}" with the captured value
    pub static ref SET_QUOTED_DELIMITED_REGEX: Regex =
        Regex::new(r#""\$\{(.*?)\}""#).unwrap();

    // replace usages of ${.*} with the captured value
    pub static ref SET_DELIMITED_REGEX: Regex =
        Regex::new(r#"\$\{(.*?)\}"#).unwrap();
}

/// cleans up a regex as specified in YAML to one that will work with the regex crate.
pub fn clean_regex<S: AsRef<str>>(s: S) -> String {
    s.as_ref()
        .trim()
        .trim_matches('/')
        .replace("\\/", "/")
        .replace("\\:", ":")
        .replace("\\#", "#")
        .replace("\\%", "%")
        .replace("\\'", "'")
        .replace("\\`", "`")
}

/// Replaces a "set" step value with a variable
pub fn replace_set<S: AsRef<str>>(s: S) -> String {
    let mut s = SET_QUOTED_DELIMITED_REGEX
        .replace_all(s.as_ref(), "$1")
        .into_owned();

    s = SET_DELIMITED_REGEX
        .replace_all(s.as_ref(), "$1")
        .into_owned();

    SET_REGEX.replace_all(s.as_ref(), "$1").into_owned()
}
