# Search

OpenSearch provides a powerful search API that allows you to search for documents in an index. The search API supports a number of parameters that allow you to customize the search operation. In this guide, we will explore the search API and its parameters.

# Setup

Let's start by creating an index and adding some documents to it:

```rust
let url = Url::parse("https://localhost:9200")?;
let credentials = Credentials::Basic("admin".into(), "admin".into());
let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
    .cert_validation(CertificateValidation::None)
    .auth(credentials)
    .build()?;
let client = OpenSearch::new(transport);
client
  .indices()
  .create(IndicesCreateParts::Index("movies"))
  .send()
  .await?;

for n in 0..11 {
    client
        .index(IndexParts::IndexId("movies", i))
        .body(json!({
            "id": i,
            "title": format!("The Dark Knight {}", i),
            "director": "Christopher Nolan",
            "year": 2008 + i
        }))
        .send()
        .await?;
}

client
    .index(IndexParts::IndexId("movies", i))
    .body(json!({
        "id": 11,
        "title": "The Godfather",
        "director": "Francis Ford Coppola",
        "year": "1972"
    }))
    .send()
    .await?;

client
    .index(IndexParts::IndexId("movies", i))
    .body(json!({
        "id": 12,
        "title": "The Shawshank Redemption",
        "director": "Frank Darabont",
        "year": "1994"
    }))
    .send()
    .await?;

client
    .indices
    .refresh(IndicesRefreshParts::Index(&["movies"]))
    .send()
    .await?
```

## Search API

### Basic Search

The search API allows you to search for documents in an index. The following example searches for ALL documents in the `movies` index:

```rust
let response = client
    .search(SearchParts::Index(&["movies"]))
    .send()
    .await?
    .json::<Value>()
    .await?;
println!("{}", response["hits"]["count"]);
```

You can also search for documents that match a specific query. The following example searches for documents that match the query `dark knight`:

```rust
let response = client
    .search(SearchParts::Index(&["movies"]))
    .body(json!({
        "query": {
            "multi_match": {
                "query": "dark knight",
                "fields": ["title"]
            }
        }
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
println!("{:?}", response["hits"]["hits"].as_array().unwrap());
```

OpenSearch query DSL allows you to specify complex queries. Check out the [OpenSearch query DSL documentation](https://opensearch.org/docs/latest/query-dsl/) for more information.

### Basic Pagination

The search API allows you to paginate through the search results. The following example searches for documents that match the query `dark knight`, sorted by `year` in ascending order, and returns the first 2 results after skipping the first 5 results:

```rust
let search_body = json!({
    "query": {
        "match": {
            "title": "dark knight"
        }
    },
    "sort": [
        {
            "year": {
                "order": "asc"
            }
        }
    ]
});

let response = client
    .search(SearchParts::Index(&["movies"]))
    .from(5)
    .size(2)
    .body(search_body)
    .send()
    .await?
    .json::<Value>()
    .await?;
println!("{:?}", response["hits"]["hits"].as_array().unwrap());
```

With sorting, you can also use the `search_after` parameter to paginate through the search results. Let's say you have already displayed the first page of results, and you want to display the next page. You can use the `search_after` parameter to paginate through the search results. The following example will demonstrate how to get the first 3 pages of results using the search query of the previous example:

```rust
let page_1 = client
    .search(SearchParts::Index(&["movies"]))
    .from(5)
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ]
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_1_hits = page_1["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_1_hits);

let page_2 = client
    .search(SearchParts::Index(&["movies"]))
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ],
        "search_after": page_1_hits[page_1_hits.len() - 1]["sort"]
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_2_hits = page_2["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_2_hits);

let page_3 = client
    .search(SearchParts::Index(&["movies"]))
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ],
        "search_after": page_2_hits[page_2_hits.len() - 1]["sort"]
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_3_hits = page_3["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_3_hits);
```

### Pagination with scroll

When retrieving large amounts of non-real-time data, you can use the `scroll` parameter to paginate through the search results.

```rust
// Create a point in time
let pit_id = client
    .create_pit(CreatePitParts::Index(&["movies"]))
    .keep_alive("1m")
    .send()
    .await?
    .json::<Value>()
    .await?["pit_id"]
    .to_string();

// Get the first 3 pages of results, including pit info in the search body
let page_1 = client
    .search(SearchParts::Index(&["movies"]))
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ],
        "pit": {
            "id": pit_id,
            "keep_alive": "1m"
        }
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_1_hits = page_1["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_1_hits);

let page_2 = client
    .search(SearchParts::Index(&["movies"]))
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ],
        "pit": {
            "id": pit_id,
            "keep_alive": "1m"
        },
        "search_after": page_1_hits[page_1_hits.len() - 1]["sort"]
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_2_hits = page_2["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_2_hits);

let page_3 = client
    .search(SearchParts::Index(&["movies"]))
    .size(2)
    .body(json!({
        "query": {
            "match": {
                "title": "dark knight"
            }
        },
        "sort": [
            {
                "year": {
                    "order": "asc"
                }
            }
        ],
        "pit": {
            "id": pit_id,
            "keep_alive": "1m"
        },
        "search_after": page_2_hits[page_2_hits.len() - 1]["sort"]
    }))
    .send()
    .await?
    .json::<Value>()
    .await?;
let page_3_hits = page_3["hits"]["hits"].as_array().unwrap();
println!("{:?}", page_3_hits);

// Print out the titles of the first 3 pages of results
page_1_hits
    .iter()
    .for_each(|hit| println!("{}", hit["_source"]["title"]));
page_2_hits
    .iter()
    .for_each(|hit| println!("{}", hit["_source"]["title"]));
page_3_hits
    .iter()
    .for_each(|hit| println!("{}", hit["_source"]["title"]));

// Delete the point in time
client
    .delete_pit(DeletePitParts::None)
    .body(json!({ "pit_id": pit_id }))
    .send()
    .await?;
```

Note that a point-in-time is associated with an index or a set of index. So, when performing a search with a point-in-time, you DO NOT specify the index in the search.

## Cleanup

```rust
client
    .indices()
    .delete(IndicesDeleteParts::Index(&["movies"]))
    .send()
    .await?;
```
