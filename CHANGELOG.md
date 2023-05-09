# CHANGELOG
Inspired from [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)

## [Unreleased]

### Added

### Dependencies

### Changed

### Deprecated

### Removed

### Fixed

### Security

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

### Deprecated

### Removed

### Fixed
- Fixes `cargo make test` failing out of the box ([#117](https://github.com/opensearch-project/opensearch-rs/pull/117))
- Fixes f64 comparison in `yaml_test_runner` to use numeric-based comparison instead of string-based ([#150](https://github.com/opensearch-project/opensearch-rs/pull/150))
- Fixes YAML spec tests by adding u64 (unsigned long) support ([#167](https://github.com/opensearch-project/opensearch-rs/pull/167))

### Security

[Unreleased]: https://github.com/opensearch-project/opensearch-rs/compare/v2.1.0...HEAD
[2.1.0]: https://github.com/opensearch-project/opensearch-rs/compare/v2.0.0...v2.1.0