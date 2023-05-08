# Document Lifecycle

This guide covers OpenSearch Rust Client API actions for Document Lifecycle. You'll learn how to create, read, update, and delete documents in your OpenSearch cluster. Whether you're new to OpenSearch or an experienced user, this guide provides the information you need to manage your document lifecycle effectively.

## Setup

Assuming you have OpenSearch running locally on port 9200, you can create a client instance
with the following code:

```rust
let url = Url::parse("https://localhost:9200")?;
let credentials = Credentials::Basic("admin".into(), "admin".into());
let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
    .cert_validation(CertificateValidation::None)
    .auth(credentials)
    .build()?;
let client = OpenSearch::new(transport);
```

Next, create an index named `movies` with the default settings:

```rust
let index = "movies";
client.indices().create(IndicesCreateParts::Index(index)).send().await?;
```

## Document API Actions

### Create a new document with specified ID

To create a new document, use the `create` or `index` API action. The following code creates two new documents with IDs of `1` and `2`:

```rust
client
  .indices()
  .create(IndexParts::IndexId(index, "1"))
  .body(json!({ "title": "Beauty and the Beast", "year": 1991 }))
  .send()
  .await?;

client
  .indices()
  .create(IndexParts::IndexId(index, "2"))
  .body(json!(
    { "title": "Beauty and the Beast - Live Action", "year":  2017 }))
  .send()
  .await?;
```

Note that the `create` action is NOT idempotent. If you try to create a document with an ID that already exists, the request will fail:

```rust
let err = client
  .indices()
  .create(IndexParts::IndexId(index, 1))
  .body(json!({"title": "Just Another Movie" }))
  .send()
  .await?
  .exception()
  .await?
  .unwrap();
println!("{:#?}", err);
```

The `index` action, on the other hand, is idempotent. If you try to index a document with an existing ID, the request will succeed and overwrite the existing document. Note that no new document will be created in this case. You can think of the `index` action as an upsert:

```rust
client
  .index(IndexParts::IndexId(index, "2"))
  .body(json!({ "title": "Updated Title" }))
  .send()
  .await?;

client
  .index(IndexParts::IndexId(index, "2"))
  .body(json!(
    { "title": "The Lion King", "year": 1994 }))
  .send()
  .await?;
```

### Create a new document with auto-generated ID

You can also create a new document with an auto-generated ID by omitting the `id` parameter. The following code creates documents with an auto-generated IDs in the `movies` index:

```rust
client
  .indices()
  .create(IndicesCreateParts::Index(index))
  .body(json!({ "title": "The Lion King 2", "year": 1998 }))
  .send()
  .await?;
 // OR
 client
   .index(IndexParts::IndexId(index))
   .body(json!({ "title": "The Lion King 2", "year": 1998 }))
   .send()
   .await?;
```

In this case, the ID of the created document in the `result` field of the response body:

```json
{
  "_index": "movies",
  "_type": "_doc",
  "_id": "1",
  "_version": 1,
  "result": "created",
  "_shards": {
    "total": 2,
    "successful": 1,
    "failed": 0
  },
  "_seq_no": 0,
  "_primary_term": 1
}
```

### Get a document

To get a document, use the `get` API action. The following code gets the document with ID `1` from the `movies` index:

```rust
let response = client
  .get(GetParts::IndexId(index, "1"))
  .send()
  .await?
  .json::<Value>()
  .await?;

println!("{}", response["_source"]);
// OUTPUT: {"title": "Beauty and the Beast", "year": 1991}
```

You can also use `_source_include` and `_source_exclude` parameters to specify which fields to include or exclude in the response:

```rust
let response = client
  .get(GetParts::IndexId(index, "1"))
  ._source_includes("title")
  .send()
  .await?
  .json::<Value>()
  .await?;

println!("{}", response["_source"]);
// OUTPUT: {"title"=>"Beauty and the Beast"}

let response = client
  .get(GetParts::IndexId(index, "1"))
  ._source_includes("title")
  .send()
  .await?
  .json::<Value>()
  .await?;

println!("{}", response["_source"]);
// OUTPUT: {"year"=>1991}
```

### Get multiple documents

To get multiple documents, use the `mget` API action:

```rust
let response = client
  .mget(MgetParts::Index(index))
  .body(json!({
    { "docs": vec![{ "_id": 1 }, { "_id": 2 }] }
  }))
  .send()
  .await?
  .json::<Value>()
  .await?;

for doc in response["docs"].iter() {
  println!("{}", doc["_source"]);
}
```

### Check if a document exists

To check if a document exists, use the `exists` API action. The following code checks if the document with ID `1` exists in the `movies` index:

```rust
client
  .exists(ExistsParts::IndexId(index, "1"))
  .send()
  .await?;
```

### Update a document

To update a document, use the `update` API action. The following code updates the `year` field of the document with ID `1` in the `movies` index:

```rust
client
  .update(UpdateParts::IndexId(index, "1"))
  .body(json!({ "doc": { "year": 1995 } }))
  .send()
  .await?
```

Alternatively, you can use the `script` parameter to update a document using a script. The following code increments the `year` field of the of document with ID `1` by 5 using painless script, the default scripting language in OpenSearch:

```rust
client
  .update(UpdateParts::IndexId(index, "1"))
  .body(json!({ "script": { "source": "ctx._source.year += 5" }}))
  .send()
  .await?;
```

Note that while both `update` and `index` actions perform updates, they are not the same. The `update` action is a partial update, while the `index` action is a full update. The `update` action only updates the fields that are specified in the request body, while the `index` action overwrites the entire document with the new document.

### Update multiple documents by query

To update documents that match a query, use the `update_by_query` API action. The following code decreases the `year` field of all documents with `year` greater than 2023:

```rust
client
  .update_by_query(UpdateByQueryParts::Index(&[index]))
  .body(json!({
    "script": { "source": "ctx._source.year -= 1" },
    "query": { "range": { "year": { "gt": 2023 } }}
    }))
  .send()
  .await?;
```

### Delete a document

To delete a document, use the `delete` API action. The following code deletes the document with ID `1`:

```rust
client
  .delete(DeleteParts::IndexId(index, "1"))
  .send()
  .await?;
```

### Delete multiple documents by query

To delete documents that match a query, use the `delete_by_query` API action. The following code deletes all documents with `year` greater than 2023:

```rust
client
  .delete_by_query(DeleteByQueryParts::Index(&[index]))
  .body(json!({
    "query": { "range": { "year": { "gt": 2023 } } }
  }))
  .send()
  .await?;
```

## Cleanup

To clean up the resources created in this guide, delete the `movies` index:

```rust
client
  .indices()
  .delete(IndicesDeleteParts::Index(&[index]))
  .send()
  .await?;
```
