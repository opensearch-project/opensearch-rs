# Index Lifecycle

This guide covers OpenSearch Rust Client API actions for Index Lifecycle. You'll learn how to create, read, update, and delete indices in your OpenSearch cluster. We will also leverage index templates to create default settings and mappings for indices of certain patterns.

## Setup

In this guide, we will need an OpenSearch cluster with more than one node. Let's use the sample [docker-compose.yml](https://opensearch.org/samples/docker-compose.yml) to start a cluster with two nodes. The cluster's API will be available at `localhost:9200` with basic authentication enabled with default username and password of `admin:admin`.

To start the cluster, run the following command:

```bash
cd /path/to/docker-compose.yml
docker-compose up -d
```

Let's create a client instance to access this cluster:

```rust
use opensearch::{auth::Credentials, http::transport::TransportBuilder, OpenSearch};
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://admin:admin@localhost:9200")?;
    let transport = TransportBuilder::new().url(url).disable_certificate_validation(true).build()?;
    let client = OpenSearch::new(transport);
    println!("{:?}", client.info().send().await?); // Check server info and make sure the client is connected
    Ok(())
}
```

## Index API Actions

### Create a new index

You can quickly create an index with default settings and mappings by using the `indices.create` API action. The following example creates an index named `paintings` with default settings and mappings:

```rust
client.indices().create().index("paintings").send().await?;
```

To specify settings and mappings, you can pass them as the `body` of the request. The following example creates an index named `movies` with custom settings and mappings:

```rust
client.indices().create("movies").body(json!({
    "settings": {
        "index": {
            "number_of_shards": 2,
            "number_of_replicas": 1
        }
    },
    "mappings": {
        "properties": {
            "title": { "type": "text" },
            "year": { "type": "integer" }
        }
    }
})).send().await?;
```

When you create a new document for an index, OpenSearch will automatically create the index if it doesn't exist:

```rust
println!("{}", client.indices().exists("burner").send().await?.json::<bool>().await?); // => false
client.index("burner").body(json!({ "lorem": "ipsum" })).send().await?;
println!("{}", client.indices().exists("burner").send().await?.json::<bool>().await?); // => true
```

### Update an Index

You can update an index's settings and mappings by using the `indices.put_settings` and `indices.put_mapping` API actions.

The following example updates the `movies` index's number of replicas to `0`:

```rust
client.indices().put_settings(PutSettingsParts::Index("movies")).body(json!({
    "index": {
        "number_of_replicas": 0
    }
})).send().await?;
```

The following example updates the `movies` index's mappings to add a new field named `director`:

```rust
client.indices().put_mapping(PutMappingRequest::new("movies").body(json!({ // OR put_mapping(PutMappingParts::Index(&["movies"]))
    "properties": {
        "director": { "type": "text" }
    }
}))).send().await?;
```

### Get Metadata for an Index

Let's check if the index's settings and mappings have been updated by using the `indices.get` API action:

```rust
println!("{:#?}", client.indices().get("movies").send().await?.json::<serde_json::Value>().await?);
```

The response body contains the index's settings and mappings:

```rust
{
  "movies": {
    "aliases": {},
    "mappings": {
      "properties": {
        "title": { "type": "text" },
        "year": { "type": "integer" },
        "director": { "type": "text" }
      }
    },
    "settings": {
      "index": {
        "creation_date": "1680297372024",
        "number_of_shards": "2",
        "number_of_replicas": "0",
        "uuid": "FEDWXgmhSLyrCqWa8F_aiA",
        "version": { "created": "136277827" },
        "provided_name": "movies"
      }
    }
  }
}
```

### Delete an Index

Let's delete the `movies` index by using the `indices.delete` API action:

```rust
client.indices().delete("movies").send().await?;
```

We can also delete multiple indices at once:

```rust
client.indices().delete(IndicesDeleteParts::Index(&["movies", "paintings", "burner"])).ignore(404).send().await?;
```

Notice that we are passing `ignore: 404` to the request. This tells the client to ignore the `404` error if the index doesn't exist for deletion. Without it, the above `delete` request will throw an error because the `movies` index has already been deleted in the previous example.

## Cleanup

All resources created in this guide are automatically deleted when the cluster is stopped. You can stop the cluster by running the following command:

```bash
docker-compose down
```
