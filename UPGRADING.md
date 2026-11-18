# Upgrading

- [Upgrading to 3.0](#upgrading-to-30)
  - [Checklist](#checklist)
  - [Generated APIs](#generated-apis)
    - [Removed APIs](#removed-apis)
    - [Changed URL parts](#changed-url-parts)
    - [Changed query parameter types](#changed-query-parameter-types)
    - [Removed query parameters](#removed-query-parameters)
    - [Changed parameter enums](#changed-parameter-enums)
  - [Transport and errors](#transport-and-errors)
  - [Retries are on by default](#retries-are-on-by-default)
  - [Non-exhaustive types](#non-exhaustive-types)
  - [New in 3.0](#new-in-30)

## Upgrading to 3.0

3.0 generates the client from the [OpenSearch API specification](https://github.com/opensearch-project/opensearch-api-specification)
instead of the frozen REST specs used up to 2.x. Most of the changes below follow from that: APIs and
parameters the specification no longer has are gone, and some types follow the specification more
closely. On top of that, the transport retries failed requests by default.

### Checklist

1. Replace the calls to [removed APIs](#removed-apis).
2. Fix the compile errors from [changed URL parts](#changed-url-parts) and
   [query parameter types](#changed-query-parameter-types): most are a `&str` becoming a slice, or the reverse.
3. If you build a `Transport` with `TransportBuilder`, check that the [retry defaults](#retries-are-on-by-default)
   suit you, and call `disable_retry()` otherwise.
4. Add a wildcard arm to matches on the [non-exhaustive types](#non-exhaustive-types).
5. If you named `BuildError`, use `Error` instead (see [Transport and errors](#transport-and-errors)).

An API or parameter that is not generated any more can still be called with a raw request, see
[Make Raw Json Requests](USER_GUIDE.md#make-raw-json-requests).

### Generated APIs

#### Removed APIs

These APIs are not in the OpenSearch API specification, because OpenSearch no longer has them:

| 2.x | Notes |
|---|---|
| `indices().unfreeze(..)` | Frozen indices do not exist in OpenSearch. |
| `indices().reload_search_analyzers(..)` | |
| `indices().migrate_to_data_stream(..)` | |
| `indices().promote_data_stream(..)` | |
| `snapshot().get_features(..)` | |
| `text_structure()` namespace (`find_structure`) | The namespace is gone, together with `params::Format`. |

`cat().master(..)` is still available and deprecated, as on 2.x; use `cat().cluster_manager(..)`.

#### Changed URL parts

Four `*Parts` enums follow the URL path parameters of the specification:

| Enum | 2.x | 3.0 |
|---|---|---|
| `IndicesGetIndexTemplateParts` | `Name(&[&str])` | `Name(&str)` |
| `ClusterGetComponentTemplateParts` | `Name(&[&str])` | `Name(&str)` |
| `IndicesStatsParts` | `Metric(&[&str])`, `IndexMetric(&[&str], &[&str])` | `Metric(&str)`, `IndexMetric(&[&str], &str)` |
| `NodesInfoParts` | `NodeId(&[&str])`, `Metric(&[&str])` | `NodeIdOrMetric(&str)` |

For example:

```rust
// 2.x
client.indices().get_index_template(IndicesGetIndexTemplateParts::Name(&["logs"]));
client.nodes().info(NodesInfoParts::NodeId(&["node-1"]));

// 3.0
client.indices().get_index_template(IndicesGetIndexTemplateParts::Name("logs"));
client.nodes().info(NodesInfoParts::NodeIdOrMetric("node-1"));
```

A comma-separated string still selects several values, e.g. `IndicesStatsParts::Metric("docs,store")`.

Some `*Parts` enums gained variants for the URL paths the specification adds, so a `match` on them
needs the new arms: `ClusterStatsParts` (`MetricNodeId`, `MetricIndexMetricNodeId`) and
`IndicesPutAliasParts` (`None`, `Name`, `Index`).

#### Changed query parameter types

| Parameter | 2.x | 3.0 | APIs |
|---|---|---|---|
| `routing` | `&str` | `&[&str]` | `create`, `delete`, `exists`, `exists_source`, `get`, `index`, `update` and others (13) |
| `_source` | `&[&str]` | `&str` | `bulk`, `delete_by_query`, `exists`, `exists_source`, `get`, `search`, `update_by_query` and others (11) |
| `refresh` | `bool` | `params::Refresh` | `delete_by_query`, `exists`, `exists_source`, `get`, `get_source`, `mget`, `reindex`, `update_by_query` |
| `requests_per_second` | `i64` | `f32` | `delete_by_query`, `reindex`, `update_by_query` and their `rethrottle` APIs |
| `min_score` | `i64` | `f32` | `count` |
| `metric` | `&[&str]` | `params::Metric` | `cluster().reroute(..)` |
| `status` | `&[&str]` | `params::Status` | `indices().shard_stores(..)` |
| `node_ids`, `node_names` | `&str` | `&[&str]` | `cluster().post_voting_config_exclusions(..)` |
| `groups` | `bool` | `&[&str]` | `nodes().stats(..)` |
| `size` | `params::Size` | `i64` | `cat().thread_pool(..)` |

For example:

```rust
// 2.x
client.get(GetParts::IndexId("movies", "1")).routing("user-1").refresh(true);
client.search(SearchParts::None)._source(&["title", "year"]);

// 3.0
client.get(GetParts::IndexId("movies", "1")).routing(&["user-1"]).refresh(Refresh::True);
client.search(SearchParts::None)._source("title,year");
```

`slices`, `wait_for_active_shards` and `wait_for_nodes` take the type the specification declares for
them (`types::common::Slices`, `types::common::WaitForActiveShards`, `types::cluster_common::WaitForNodes`)
through `impl Into<..>`. Numbers, the typed values and strings all convert, so 2.x code such as
`.slices(params::Slices::Auto)` or `.wait_for_active_shards("all")` keeps compiling. A `&String` does not
deref to `&str` through a generic parameter, so pass `.as_str()`:

```rust
client.delete_by_query(parts).slices(5);
client.index(parts).wait_for_active_shards(2);
client.index(parts).wait_for_active_shards(shards.as_str()); // shards: String
```

#### Removed query parameters

| API | Parameter |
|---|---|
| `cat().help(..)` | `help`, `s` |
| `create_pit(..)` | `ignore_unavailable` |
| `indices().delete_data_stream(..)` | `expand_wildcards` |
| `indices().simulate_index_template(..)` | `cause`, `create` |
| `indices().stats(..)` | `types` |
| `search(..)` | `min_compatible_shard_node` |
| `update_by_query(..)` | `version_type` |

#### Changed parameter enums

`params::Format` (of `text_structure`) and `params::Size` (of `cat().thread_pool(..)`, now an `i64`) are gone.
New value enums cover the parameters the specification adds, e.g. `params::Metric`, `params::Status`
and `params::SortOrder`.

The values of the existing enums follow the specification:

| Enum | Removed | Added |
|---|---|---|
| `params::VersionType` | `Force` | |
| `params::Level` | | `AwarenessAttributes` (`cluster().health(..)`) |

Several `params` enums (e.g. `ExpandWildcards`, `Refresh`, `Time`, `Bytes`) declare their variants in
another order, which changes their discriminants. This only matters if you cast them to integers
(`as i32`).

### Transport and errors

- `TransportBuilder::build` returns `opensearch::Error`; `BuildError` is no longer public.
  Code that only used `?` or `Error` is unchanged.
- `Error` keeps `status_code()`, `is_timeout()` and `is_json()`, and adds `api_error()`, which the typed
  client returns for error statuses.
- `http::Method` has a new variant, `Patch`, for the security plugin APIs; a `match` on it needs an arm
  for it.
- `ConnectionPool::next` returns a `Connection` rather than a `&Connection`. This change shipped in
  2.4.0; it only affects you when upgrading from an earlier release with your own `ConnectionPool`.

### Retries are on by default

The transport now sends a request again when the response status is 502, 503 or 504, or when the
connection fails, up to 3 times with an exponential backoff. Each attempt takes the next node of the
connection pool. Timeouts are not retried unless enabled.

A request is sent again with the same body. A request whose effect cannot be repeated (e.g. indexing
without an id) may be applied twice when the connection fails after the server received it. To send
every request once, as in 2.x:

```rust
let transport = TransportBuilder::new(pool).disable_retry().build()?;
```

The policy is set with `max_retries`, `retry_on_status`, `retry_on_timeout`, `retry_backoff` and
`retry_budget`, see [Retries](USER_GUIDE.md#retries).

### Non-exhaustive types

`auth::Credentials`, `auth::ClientCertificate` and `cert::CertificateValidation` are `#[non_exhaustive]`,
so that variants can be added in minor releases. Outside the crate, a `match` on them needs a wildcard arm:

```rust
match credentials {
    Credentials::Basic(user, password) => basic(user, password),
    _ => other(),
}
```

`http::Method` is not, but it gained `Patch` for the security plugin APIs, so a `match` on it needs an arm
for it either way.

### New in 3.0

Not breaking, but worth knowing when upgrading:

- **Typed responses**: `client.typed()` returns the response types of `opensearch::types` from `send`,
  see [Typed Responses](USER_GUIDE.md#typed-responses).
- **Multi-node pools**: `MultiNodeConnectionPool::round_robin(..)` and `Transport::multi_node(..)`.
- **Middleware**: `TransportBuilder::with_initializer` and `with_handler` intercept requests.
- **Cached AWS credentials**: `auth::cache::CachedCredentialsProvider`.
- **New namespaces** from the specification, e.g. `list`, `ml`, `knn`, `security`, `ism`, `sql`;
  see the [CHANGELOG](CHANGELOG.md).
