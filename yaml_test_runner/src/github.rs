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

use flate2::read::GzDecoder;
use globset::Glob;
use io::Write;
use log::{info, warn};
use reqwest::{
    blocking::{ClientBuilder, Response},
    header::{HeaderMap, HeaderValue, USER_AGENT},
};
use std::{fs, fs::File, io, path::Path};
use tar::{Archive, Entry};

/// Ensures yaml test suites are available in the download directory.
///
/// By default existing (previously downloaded or vendored) test suites are
/// reused as-is: when the requested branch differs from the existing one, a
/// warning is emitted but the suites are kept, so that test generation is
/// reproducible and works offline. Pass `download = true` to replace the
/// existing suites with the ones for the requested branch.
pub fn ensure_test_suites(branch: &str, download_dir: &Path, download: bool) -> anyhow::Result<()> {
    let last_downloaded_version = download_dir.join("last_downloaded_version");
    let existing = if last_downloaded_version.exists() {
        Some(
            fs::read_to_string(&last_downloaded_version)
                .expect("Unable to read last_downloaded_version of yaml tests")
                .trim()
                .to_string(),
        )
    } else {
        None
    };

    match (existing, download) {
        (Some(version), false) => {
            if version == branch {
                info!("Using existing yaml tests from {}", branch);
            } else {
                warn!(
                    "Using existing yaml tests from {} which do not match the cluster ({}). \
                     Pass --download-tests to download the matching test suites.",
                    version, branch
                );
            }
            Ok(())
        }
        (None, false) => {
            warn!(
                "No yaml tests found at {}; downloading from {}",
                download_dir.display(),
                branch
            );
            download_test_suites(branch, download_dir)
        }
        (_, true) => download_test_suites(branch, download_dir),
    }
}

/// Downloads the yaml tests if not already downloaded
pub fn download_test_suites(branch: &str, download_dir: &Path) -> anyhow::Result<()> {
    let last_downloaded_version = download_dir.join("last_downloaded_version");
    if last_downloaded_version.exists() {
        let version = fs::read_to_string(&last_downloaded_version)
            .expect("Unable to read last_downloaded_version of yaml tests");
        if version == branch {
            info!("Already downloaded yaml tests from {}", branch);
            return Ok(());
        }
        // remove stale suites so files deleted upstream do not linger
        let free_dir = download_dir.join("free");
        if free_dir.exists() {
            fs::remove_dir_all(&free_dir)?;
        }
    }

    info!("Downloading yaml tests from {}", branch);
    let url = format!(
        "https://api.github.com/repos/opensearch-project/opensearch/tarball/{}",
        branch
    );
    let mut headers = HeaderMap::new();
    headers.append(
        USER_AGENT,
        HeaderValue::from_str(&format!("opensearch-rs/{}", env!("CARGO_PKG_NAME")))?,
    );
    let client = ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = client.get(url).send()?;
    let tar = GzDecoder::new(response);
    let mut archive = Archive::new(tar);

    let oss_test = Glob::new("**/rest-api-spec/src/main/resources/rest-api-spec/test/**/*.yml")?
        .compile_matcher();

    for entry in archive.entries()? {
        let file = entry?;
        let path = file.path()?;
        if oss_test.is_match(&path) {
            write_test_file(download_dir, "free", file)?;
        }
    }

    info!("Downloaded yaml tests from {}", &branch);
    File::create(last_downloaded_version)
        .expect("failed to create last_downloaded_version file")
        .write_all(branch.as_bytes())
        .expect("unable to write branch to last_downloaded_version file");

    Ok(())
}

fn write_test_file(
    download_dir: &Path,
    suite_dir: &str,
    mut entry: Entry<GzDecoder<Response>>,
) -> anyhow::Result<()> {
    let path = entry.path()?;

    let mut dir = {
        let mut dir = download_dir.join(suite_dir);
        let parent = path.parent().unwrap().file_name().unwrap();
        dir.push(parent);
        dir
    };

    fs::create_dir_all(&dir)?;
    dir.push(path.file_name().unwrap());
    let mut file = File::create(&dir)?;
    io::copy(&mut entry, &mut file)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write as _;

    fn existing_dir(version: &str) -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        let mut file = File::create(dir.path().join("last_downloaded_version")).unwrap();
        file.write_all(version.as_bytes()).unwrap();
        fs::create_dir_all(dir.path().join("free")).unwrap();
        dir
    }

    #[test]
    fn existing_suites_matching_branch_are_reused() {
        let dir = existing_dir("mybranch");
        // must not attempt any download
        ensure_test_suites("mybranch", dir.path(), false).unwrap();
        assert!(dir.path().join("free").exists());
    }

    #[test]
    fn existing_suites_are_kept_on_branch_mismatch() {
        let dir = existing_dir("otherbranch");
        ensure_test_suites("mybranch", dir.path(), false).unwrap();
        // existing suites and pin are left untouched
        assert!(dir.path().join("free").exists());
        let pin = fs::read_to_string(dir.path().join("last_downloaded_version")).unwrap();
        assert_eq!(pin, "otherbranch");
    }
}
