# Bulk

In this guide, you'll learn how to use the OpenSearch Rust Client API to perform bulk operations. You'll learn how to index, update, and delete multiple documents in a single request.

## Setup

First, create a client instance with the following code:

```rust
let url = Url::parse("https://localhost:9200")?;
let credentials = Credentials::Basic("admin".into(), "admin".into());
let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
    .cert_validation(CertificateValidation::None)
    .auth(credentials)
    .build()?;
let client = OpenSearch::new(transport);
```

Next, create an index named `movies` and another named `books` with the default settings:

```rust
let movies = "movies";
let books = "books";
client.indices().create(IndicesCreateParts::Index(movies)).send().await?;
client.indices().create(IndicesCreateParts::Index(books)).send().await?;
```

## Bulk API

The `bulk` API action allows you to perform document operations in a single request. The body of the request is an array of objects that contains the bulk operations and the target documents to index, create, update, or delete.

### Indexing multiple documents

The following code creates two documents in the `movies` index and one document in the `books` index:

```rust
client
    .bulk(BulkParts::None)
    .body(Vec::<JsonBody<_>>::from([
        json!({ "index": { "_index": movies, "_id": "1" }}).into(),
        json!({ "title": "Beauty and the Beast", "year": "1991" }).into(),
        json!({ "index": { "_index": movies, "_id": "2" }}).into(),
        json!({ "title": "Beauty and the Beast - Live Action", "year": "2017" }).into(),
        json!({ "index": { "_index": books, "_id": "1" }}).into(),
        json!({ "title": "The Lion King", "year": "1994" }).into(),
    ]))
    .send()
    .await?;
```

As you can see, each bulk operation is comprised of two objects. The first object contains the operation type and the target document's `_index` and `_id`. The second object contains the document's data. As a result, the body of the request above contains six objects for three index actions.

Alternatively, the `bulk` method can accept an array of hashes where each hash represents a single operation. The following code is equivalent to the previous example:

```rust
let mut ops = BulkOperations::new();
ops.push(
    BulkOperation::index(json!({
        "title": "Beauty and the Beast",
        "year": "1991"
    }))
    .index(movies)
    .id("1"),
)?;
ops.push(
    BulkOperation::index(json!({
        "title": "Beauty and the Beast - Live Action",
        "year": "2017"
    }))
    .index(movies)
    .id("2"),
)?;
ops.push(
    BulkOperation::index(json!({
        "title": "The Lion King",
        "year": "1994"
    }))
    .index(books)
    .id("1"),
)?;

client.bulk(BulkParts::None).body(vec![ops]).send().await?;
```

We will use this format for the rest of the examples in this guide.

### Creating multiple documents

Similarly, instead of calling the `create` method for each document, you can use the `bulk` API to create multiple documents in a single request. The following code creates three documents in the `movies` index and one in the `books` index:

```rust
let mut ops = BulkOperations::new();
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 2",
        "year": "2030"
    }))
    .index(movies),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 3",
        "year": "2031"
    }))
    .index(movies),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 4",
        "year": "2049"
    }))
    .index(movies),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "The Lion King 2",
        "year": "1998"
    }))
    .index(books),
)?;
client.bulk(BulkParts::None).body(vec![ops]).send().await?;
```

Note that we specified only the `_index` for the last document in the request body. This is because the `bulk` method accepts an `index` parameter that specifies the default `_index` for all bulk operations in the request body. Moreover, we omit the `_id` for each document and let OpenSearch generate them for us in this example, just like we can with the `create` method.

### Updating multiple documents

```rust
let mut ops = BulkOperations::new();
ops.push(
    BulkOperation::update(json!({
        "year": "1992"
    }))
    .index(movies)
    .id("1"),
)?;
ops.push(
    BulkOperation::update(json!({
        "year": "2018"
    }))
    .index(movies)
    .id("2"),
)?;

client.bulk(BulkParts::None).body(vec![ops]).send().await?;
```

Note that the updated data is specified in the `doc` field of the `data` object.

### Deleting multiple documents

```rust
let mut ops = BulkOperations::new();
ops.push(
    BulkOperation::delete()
    .index(movies)
    .id("1"),
)?;
ops.push(
    BulkOperation::delete()
    .index(movies)
    .id("2"),
)?;

client.bulk(BulkParts::None).body(vec![ops]).send().await?;
```

### Mix and match operations

You can mix and match the different operations in a single request. The following code creates two documents, updates one document, and deletes another document:

```rust
let mut ops = BulkOperations::new();

ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 5"
        "year": "2050"
    }))
    .index(movies),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 6"
        "year": "2051"
    }))
    .index(movies),
)?;
ops.push(
    BulkOperation::update(json!({
        "year": "2052"
    }))
    .index(movies)
    .id("3"),
)?;
ops.push(
    BulkOperation::delete()
    .index(movies)
    .id("4"),
)?;

client.bulk(BulkParts::None).body(vec![ops]).send().await?;
```

### Handling errors

The `bulk` API returns an array of responses for each operation in the request body. Each response contains a `status` field that indicates whether the operation was successful or not. If the operation was successful, the `status` field is set to a `2xx` code. Otherwise, the response contains an error message in the `error` field.

The following code shows how to look for errors in the response:

```rust
let mut ops = BulkOperations::new();

ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast"
        "year": "1991"
    }))
    .index(movies)
    .id("1"),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 2"
        "year": "2030"
    }))
    .index(movies)
    .id("2"),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 3"
        "year": "2031"
    }))
    .index(movies)
    .id("1"),
)?;
ops.push(
    BulkOperation::create(json!({
        "title": "Beauty and the Beast 4"
        "year": "2049"
    }))
    .index(movies)
    .id("2"),
)?;

let response = client
  .bulk(BulkParts::None)
  .body(vec![ops])
  .send()
  .await?;

let response_body = response.json::<Value>().await?;

for item in response["items"].iter() {
  let range  = 200..299;
  if !(200..299).contains(&item["create"]["status"].as_u64().unwrap()) {
    println!("{}", item["create"]["error"]["reason"]);
  }
}
```

## Cleanup

To clean up the resources created in this guide, delete the `movies` and `books` indices:

```rust
client
  .indices()
  .delete(IndicesDeleteParts::Index(&[movies, books]))
  .send()
  .await?;
```
