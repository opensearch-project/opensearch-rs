- [User Guide](#user-guide)
  - [All Guides](#all-guides)
  - [Example](#example)
    - [Create a Client](#create-a-client)
    - [Display Server Version](#display-server-version)
    - [Create an Index](#create-an-index)
    - [Add a Document to the Index](#add-a-document-to-the-index)
    - [Search for a Document](#search-for-a-document)
    - [Delete the Index](#delete-the-index)
    - [Make raw json requests](#make-raw-json-requests)
  - [Amazon OpenSearch and OpenSearch Serverless](#amazon-opensearch-and-opensearch-serverless)
    - [Create a Client](#create-a-client-1)

# User Guide

## All Guides

- [Advanced Index Actions](guides/advanced_index_actions.md)

## Example

In the example below, we create a client, an index, insert a document to the index, search for the document, and finally delete the index.

### Create a Client

To create a client to make API calls to OpenSearch running on `http://localhost:9200`.

```rust
let client = OpenSearch::default();
```

Alternatively, you can create a client to make API calls against OpenSearch running on a
specific `url::Url`.

```rust
let transport = Transport::single_node("https://example.com")?;
let client = OpenSearch::new(transport);
```

### Display Server Version

```rust
let info: Value = client.info().send().await?.json().await?;
println!(
    "{}: {}",
    info["version"]["distribution"].as_str().unwrap(),
    info["version"]["number"].as_str().unwrap()
);
```

### Create an Index

```rust
client
    .indices()
    .create(opensearch::indices::IndicesCreateParts::Index("movies"))
    .send()
    .await?;
```

### Add a Document to the Index

```rust
client
    .index(opensearch::IndexParts::Index("movies"))
    .body(serde_json::json!({
            "title": "Moneyball",
            "director": "Bennett Miller",
            "year": 2011
        }
    ))
    .send()
    .await?;
```

### Search for a Document

Make a query and display the response body and the search results.

```rust
let response = client
    .search(opensearch::SearchParts::Index(&["movies"]))
    .body(serde_json::json!({
            "query": {
                "match": {
                    "director": "miller"
                }
            }
        }
    ))
    .send()
    .await?;

let response_body = response.json::<Value>().await?;
println!("{}", serde_json::to_string_pretty(&response_body).unwrap());

for hit in response_body["hits"]["hits"].as_array().unwrap() {
    println!("{:?}", hit["_source"]);
}
```

### Delete the Index

```rust
client
    .indices()
    .delete(opensearch::indices::IndicesDeleteParts::Index(&["movies"]))
    .send()
    .await?;
```

### Make raw json requests
Make raw http requests

```rust
  let response = client
      .send(
          Method::Delete,
          "/movies",
          HeaderMap::new(),
          Option::<&String>::None,
          Option::<&String>::None,
          None,
      )
      .await?;

  // create index
  let response = client
      .send(
          Method::Put,
          "/movies",
          HeaderMap::new(),
          Option::<&String>::None,
          Option::<&String>::None,
          None,
      )
      .await?;
  assert_eq!(response.status_code().as_u16(), 200_u16);

  // Add a document
  let document =
      r#"{
        "name" : "Mission Impossible",
        "year" : "2001"
      }"#;

  let response = client
      .send(
          Method::Post,
          "/movies/_doc",
          HeaderMap::new(),
          Option::<&String>::None,
          Some(document),
          None,
      )
      .await?;

  assert_eq!(response.status_code().as_u16(), 201_u16);

  // query document
  let query = json!({
      "query": {
        "match": {
             "name" : "Mission Impossible"
         }
    }
  });

  let response = client
      .send(
          Method::Get,
          "/movies/_search",
          HeaderMap::new(),
          Option::<&String>::None,
          Some(query.to_string()),
          None,
      )
      .await?;
    
  assert_eq!(response.status_code().as_u16(), 200_u16);
```

## Amazon OpenSearch and OpenSearch Serverless

This library supports [Amazon OpenSearch Service](https://aws.amazon.com/opensearch-service/) and [OpenSearch Serverless](https://docs.aws.amazon.com/opensearch-service/latest/developerguide/serverless.html).

### Create a Client

Create a client with AWS credentials as follows. Make sure to specify the correct service name and signing region.

```rust
let url = Url::parse("https://...");
let service_name = "es"; // use "aoss" for OpenSearch Serverless
let conn_pool = SingleNodeConnectionPool::new(url?);
let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
let aws_config = aws_config::from_env().region(region_provider).load().await.clone();
let transport = TransportBuilder::new(conn_pool)
    .auth(aws_config.clone().try_into()?)
    .service_name(service_name)
    .build()?;
let client = OpenSearch::new(transport);
```
