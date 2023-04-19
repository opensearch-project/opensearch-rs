# Advanced Index Actions

In this guide, we will look at some advanced index actions that are not covered in the [Index Lifecycle](index_lifecycle.md) guide.

## Setup

Let's create a client instance, and an index named `movies`:

```rust
// Create a client to make API calls to OpenSearch running on https://localhost:9200.
let url = Url::parse("https://localhost:9200")?;
let credentials = Credentials::Basic("admin".into(), "admin".into());
let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
        .cert_validation(CertificateValidation::None)
        .auth(credentials)
        .build()?;
let client = OpenSearch::new(transport);
client.indices().create(IndicesCreateParts::Index("movies")).send().await?;
```

## API Actions

### Clear index cache

You can clear the cache of an index or indices by using the `indices.clear_cache` API action. The following example clears the cache of the `movies` index:

```rust
client.indices().clear_cache(IndicesClearCacheParts::Index(&["movies"])).send().await?;
```

By default, the `indices.clear_cache` API action clears all types of cache. To clear specific types of cache pass the the `query`, `fielddata`, or `request` parameter to the API action:

```rust
client.indices().clear_cache(IndicesClearCacheParts::Index(&["movies"])).query(true).send().await?;
    client.indices().clear_cache(IndicesClearCacheParts::Index(&["movies"])).fielddata(true).request(true).send().await?;
```

### Flush index

Sometimes you might want to flush an index or indices to make sure that all data in the transaction log is persisted to the index. To flush an index or indices use the `indices.flush` API action. The following example flushes the `movies` index:

```rust
client.indices().flush(IndicesFlushParts::Index(&["movies"])).send().await?;
```

### Refresh index

You can refresh an index or indices to make sure that all changes are available for search. To refresh an index or indices use the `indices.refresh` API action:

```rust
client.indices().refresh(IndicesRefreshParts::Index(&["movies"])).send().await?;
```

### Open/Close index

You can close an index to prevent read and write operations on the index. A closed index does not have to maintain certain data structures that an opened index require, reducing the memory and disk space required by the index. The following example closes and reopens the `movies` index:

```rust
client.indices().close(IndicesCloseParts::Index(&["movies"])).send().await?;
client.indices().open(IndicesOpenParts::Index(&["movies"])).send().await?;
```

### Force merge index

You can force merge an index or indices to reduce the number of segments in the index. This can be useful if you have a large number of small segments in the index. Merging segments reduces the memory footprint of the index. Do note that this action is resource intensive and it is only recommended for read-only indices. The following example force merges the `movies` index:

```rust
client.indices().forcemerge(IndicesForcemergeParts::Index(&["movies"])).send().await?;
```

### Clone index

You can clone an index to create a new index with the same mappings, data, and MOST of the settings. The source index must be in read-only state for cloning. The following example blocks write operations from `movies` index, clones the said index to create a new index named `movies_clone`, then re-enables write:

```rust
client.indices().add_block(IndicesAddBlockParts::IndexBlock(&["movies"],"write")).send().await?;
client.indices().clone(IndicesCloneParts::IndexTarget("movies","movies_clone")).send().await?;
client.indices().put_settings(IndicesPutSettingsParts::Index(&["movies"]))
    .body(json!({
    "index": {
        "blocks": {
            "write": false
        }
    }
    })).send().await?;
```

### Split index

You can split an index into another index with more primary shards. The source index must be in read-only state for splitting. The following example create the read-only `books` index with 30 routing shards and 5 shards (which is divisible by 30), splits index into `bigger_books` with 10 shards (which is also divisible by 30), then re-enables write:

```rust
client.indices().create(IndicesCreateParts::Index("books"))
    .body(json!({
    "settings": {
        "index": {
            "number_of_shards": 5,
            "number_of_routing_shards": 30,
            "blocks": {
                "write": true
            }
        }
    }
    })).send().await?;
    client.indices().split(IndicesSplitParts::IndexTarget("books","bigger_books")).body(json!({"settings": {"index": {"number_of_shards": 10}}})).send().await?;
    client.indices().put_settings(IndicesPutSettingsParts::Index(&["books"])).body(json!({"index": {"blocks": {"write": false}}})).send().await?;
```

## Cleanup

Let's delete all the indices we created in this guide:

```rust
client.indices().delete(IndicesDeleteParts::Index(&["movies", "books", "movies_clone", "bigger_books"])).send().await?;
```
