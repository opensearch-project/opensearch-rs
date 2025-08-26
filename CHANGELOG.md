# CHANGELOG
Inspired from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]

### Added

### Dependencies
- Bumps `sysinfo` from 0.31.2 to 0.36.0 ([#331](https://github.com/opensearch-project/opensearch-rs/pull/331), [#339](https://github.com/opensearch-project/opensearch-rs/pull/339), [#346](https://github.com/opensearch-project/opensearch-rs/pull/346))
- Bump `dangoslen/dependabot-changelog-helper` from 2 to 4 ([#298](https://github.com/opensearch-project/opensearch-rs/pull/298), [#329](https://github.com/opensearch-project/opensearch-rs/pull/329))
- Bump `VachaShah/backport` from 1.1.4 to 2.2.0 ([#299](https://github.com/opensearch-project/opensearch-rs/pull/299))
- Bump `stefanzweifel/git-auto-commit-action` from 4 to 6 ([#300](https://github.com/opensearch-project/opensearch-rs/pull/300), [#343](https://github.com/opensearch-project/opensearch-rs/pull/343))
- Bump `softprops/action-gh-release` from 1 to 2 ([#303](https://github.com/opensearch-project/opensearch-rs/pull/303))
- Bump `actions/cache` from 3 to 4 ([#304](https://github.com/opensearch-project/opensearch-rs/pull/304))
- Bump `actions/setup-java` from 3 to 4 ([#302](https://github.com/opensearch-project/opensearch-rs/pull/302))
- Bump `lycheeverse/lychee-action` from 1.5.0 to 2.5.0 ([#310](https://github.com/opensearch-project/opensearch-rs/pull/310), [#321](https://github.com/opensearch-project/opensearch-rs/pull/321), [#332](https://github.com/opensearch-project/opensearch-rs/pull/332), [#341](https://github.com/opensearch-project/opensearch-rs/pull/341), [#351](https://github.com/opensearch-project/opensearch-rs/pull/351))
- Bump `peter-evans/create-pull-request` from 5 to 7 ([#308](https://github.com/opensearch-project/opensearch-rs/pull/308))
- Bump `actions/github-script` from 6 to 7 ([#309](https://github.com/opensearch-project/opensearch-rs/pull/309))
- Bump `itertools` from 0.13.0 to 0.14.0 ([#319](https://github.com/opensearch-project/opensearch-rs/pull/319))
- Bump `toml` from 0.8.0 to 0.9.2 ([#345](https://github.com/opensearch-project/opensearch-rs/pull/345))

### Changed
- Changed documentation link in Cargo.toml to utilize standard docs.rs generation ([#323](https://github.com/opensearch-project/opensearch-rs/pull/323))

### Deprecated

### Removed

### Fixed

### Security

## [2.3.0]

### Added
- Added new BulkCreate operation constructor without providing optional `id` field ([#245](https://github.com/opensearch-project/opensearch-rs/pull/245))

### Dependencies
- Bumps `aws-*` dependencies to `1` ([#219](https://github.com/opensearch-project/opensearch-rs/pull/219))
- Bumps `itertools` from 0.11.0 to 0.13.0
- Bumps `hyper` from 0.14 to 1 in tests ([#221](https://github.com/opensearch-project/opensearch-rs/pull/221))
- Bumps `sysinfo` from 0.29.0 to 0.31.2
- Bumps `base64` from 0.21 to 0.22
- Bumps `reqwest` from 0.11 to 0.12
- Bumps `simple_logger` from 4.0.0 to 5.0.0

## [2.2.0]

### Added
- Added InfoResponse structure ([#187](https://github.com/opensearch-project/opensearch-rs/pull/187))
- Added documentation on how to make raw json requests ([#196](https://github.com/opensearch-project/opensearch-rs/pull/196))

### Dependencies
- Bumps `sysinfo` from 0.28.0 to 0.29.0
- Bumps `serde_with` from ~2 to ~3
- Bumps `itertools` from 0.10.0 to 0.11.0
- Bumps `syn` from 1.0 to 2.0
- Bumps `toml` from 0.7.1 to 0.8.0
- Bumps `dialoguer` from 0.10.2 to 0.11.0
- Bumps `aws-*` from >=0.53 to >=0.57 ([#201](https://github.com/opensearch-project/opensearch-rs/pull/201))

### Changed
- Moved @aditjind to Emeritus maintainers ([#170](https://github.com/opensearch-project/opensearch-rs/pull/170))

## [2.1.0]

### Added
- Adds a `TryFrom` implementation from a borrowed `aws_types::SdkConfig` for `Credentials::AwsSigV4` ([#68](https://github.com/opensearch-project/opensearch-rs/pull/65))
- Adds support for inclusive naming ([#67](https://github.com/opensearch-project/opensearch-rs/pull/67))
- Adds GitHub workflow for changelog verification ([#89](https://github.com/opensearch-project/opensearch-rs/pull/89))
- Adds GitHub workflow for unit tests ([#112](https://github.com/opensearch-project/opensearch-rs/pull/112))
- Adds support for OpenSearch Serverless ([#96](https://github.com/opensearch-project/opensearch-rs/pull/96))
- Adds release workflows to publish opensearch-rs to crates.io ([#153](https://github.com/opensearch-project/opensearch-rs/pull/153))
- Adds delete_all & get_all Point-in-Time APIs introduced in OpenSearch 2.4 ([#157](https://github.com/opensearch-project/opensearch-rs/pull/157))

### Dependencies
- Bumps `simple_logger` from 1.9.0 to 4.0.0
- Bumps `rustc_version` from 0.2 to 0.4
- Bumps `path-slash` from 0.1.3 to 0.2.1
- Bumps `serde_with` from ~1 to ~2
- Bumps `textwrap` from ^0.11 to ^0.16
- Bumps `base64` from ^0.11 to ^0.20 ([#90](https://github.com/opensearch-project/opensearch-rs/pull/90), [#95](https://github.com/opensearch-project/opensearch-rs/pull/95), [#105](https://github.com/opensearch-project/opensearch-rs/pull/105))
- Bumps `aws-*` from >=0.10 to >=0.53 ([#108](https://github.com/opensearch-project/opensearch-rs/pull/108))
- Bumps `toml` from 0.5.6 to 0.7.1
- Bumps `sysinfo` from 0.12.0 to 0.28.0
- Bumps `syn` from ~1.0 to ~2.0

### Changed
- Updates users guide with complete examples ([#114](https://github.com/opensearch-project/opensearch-rs/pull/114))
- Updates Point-in-Time APIs to match those introduced in OpenSearch 2.4 ([#136](https://github.com/opensearch-project/opensearch-rs/pull/136), [#157](https://github.com/opensearch-project/opensearch-rs/pull/157))
- Updates GitHub workflow to additionally run `cargo make test` ([#120](https://github.com/opensearch-project/opensearch-rs/pull/120))
- Updates GitHub workflows to use caching to speed up builds ([#121](https://github.com/opensearch-project/opensearch-rs/pull/121))

### Fixed
- Fixes `cargo make test` failing out of the box ([#117](https://github.com/opensearch-project/opensearch-rs/pull/117))
- Fixes f64 comparison in `yaml_test_runner` to use numeric-based comparison instead of string-based ([#150](https://github.com/opensearch-project/opensearch-rs/pull/150))
- Fixes YAML spec tests by adding u64 (unsigned long) support ([#167](https://github.com/opensearch-project/opensearch-rs/pull/167))

[Unreleased]: https://github.com/opensearch-project/opensearch-rs/compare/v2.3.0...HEAD
[2.3.0]: https://github.com/opensearch-project/opensearch-rs/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/opensearch-project/opensearch-rs/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/opensearch-project/opensearch-rs/compare/v2.0.0...v2.1.0
