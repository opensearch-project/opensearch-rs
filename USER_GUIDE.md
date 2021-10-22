# User Guide

## Example
In the example below, we create a client, an index with non-default settings, insert a document to the index,
search for the document, delete the document and finally delete the index.


#### Create a client

To create a client to make API calls to OpenSearch running on `http://localhost:9200`

```rust
let client = OpenSearch::default();
```

Alternatively, you can create a client to make API calls against OpenSearch running on a
specific `url::Url`

```rust
let transport = Transport::single_node("https://example.com")?;
let client = OpenSearch::new(transport);
```

#### Making API calls

The following makes an API call to `tweets/_search` with the json body
`{"query":{"match":{"message":"OpenSearch"}}}`

```rust
let response = client
    .search(SearchParts::Index(&["tweets"]))
    .from(0)
    .size(10)
    .body(json!({
        "query": {
            "match": {
                "message": "OpenSearch rust"
            }
        }
    }))
    .send()
    .await?;

let response_body = response.json::<Value>().await?;
let took = response_body["took"].as_i64().unwrap();
for hit in response_body["hits"]["hits"].as_array().unwrap() {
    // print the source document
    println!("{:?}", hit["_source"]);
}
```