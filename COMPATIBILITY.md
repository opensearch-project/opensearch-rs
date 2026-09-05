- [Compatibility with OpenSearch](#compatibility-with-opensearch)
- [Upgrading](#upgrading)

## Compatibility with OpenSearch

The below matrix shows the compatibility of the [`opensearch-rs`](https://docs.rs/opensearch/latest/opensearch/) with versions of [`OpenSearch`](https://opensearch.org/downloads.html#opensearch). Version ranges are inclusive of both endpoints.

| Client Version | OpenSearch Version |
| --- | --- |
| 1.0.0 | 1.0 - 1.3 |
| 2.x.0 | 1.x - 2.x |
| 3.x.0 | 2.19 - 3.x |

Starting with 3.0, the officially supported (CI-tested) set tracks the OpenSearch releases still receiving patches at each `opensearch-rs` release: every release of the current major plus the latest release of the previous major. Today that is 2.19.x and all of 3.x. This set is re-evaluated at each release.

The client may still function against older servers, but those lines are not part of the tested matrix; the 2.x client remains the documented fallback for OpenSearch lines that the OpenSearch project no longer patches (1.x through 2.18.x).

## Upgrading

Major versions of OpenSearch introduce breaking changes that require careful upgrades of the client. Please refer to the [OpenSearch documentation](https://opensearch.org/docs/latest/clients/index/) for more information.
