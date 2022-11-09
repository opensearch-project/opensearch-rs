use std::collections::{BTreeMap, BTreeSet};

use log::warn;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize, Clone, Default)]
pub struct SkippedFeaturesAndTests {
    #[serde(default)]
    features: BTreeSet<String>,
    #[serde(default)]
    tests: BTreeMap<String, BTreeSet<String>>,
}

impl SkippedFeaturesAndTests {
    pub fn add_all(&mut self, skips: &SkippedFeaturesAndTests) {
        for feature in skips.features.iter().cloned() {
            self.features.insert(feature);
        }
        for (test_file, tests) in skips.tests.iter() {
            let tests_for_file = self.tests.entry(test_file.clone()).or_default();
            for test in tests.iter().cloned() {
                tests_for_file.insert(test);
            }
        }
    }

    pub fn should_skip_test(&self, path: &str, name: &str) -> bool {
        self.tests
            .get(path)
            .map_or(false, |tests| tests.contains("*") || tests.contains(name))
    }

    pub fn should_skip_feature(&self, name: &str) -> bool {
        self.features.contains(name)
    }
}

#[derive(Deserialize, Default)]
pub struct SkipsBySecurity {
    #[serde(flatten, default)]
    always: SkippedFeaturesAndTests,
    #[serde(default)]
    when_secure: SkippedFeaturesAndTests,
    #[serde(default)]
    when_insecure: SkippedFeaturesAndTests,
}

impl SkipsBySecurity {
    pub fn get_skips_for(&self, secure: bool) -> SkippedFeaturesAndTests {
        let mut skips = self.always.clone();
        skips.add_all(if secure {
            &self.when_secure
        } else {
            &self.when_insecure
        });
        skips
    }
}

#[derive(Deserialize)]
pub struct GlobalSkips {
    #[serde(flatten, default)]
    for_version: BTreeMap<String, SkipsBySecurity>,
}

impl GlobalSkips {
    pub fn get_skips_for(&self, version: &Version, secure: bool) -> SkippedFeaturesAndTests {
        let mut skips = SkippedFeaturesAndTests::default();
        for (version_req, version_skips) in self.for_version.iter() {
            match semver::VersionReq::parse(version_req) {
                Ok(version_req) => {
                    if version_req.matches(version) {
                        skips.add_all(&version_skips.get_skips_for(secure));
                    }
                }
                Err(e) => warn!(
                    "Invalid version requirement in skip.yml: `{}`: {}",
                    version_req, e
                ),
            }
        }
        skips
    }
}
