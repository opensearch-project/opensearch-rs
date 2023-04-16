# Index Template

Index templates are a convenient way to define settings, mappings, and aliases for one or more indices when they are created. In this guide, you'll learn how to create an index template and apply it to an index.

## Setup

Assuming you have OpenSearch running locally on port 9200, you can create a client instance
with the following code:

```rust
// Create a client to make API calls to OpenSearch running on http://localhost:9200.
let client = OpenSearch::default();
// Alternatively, you can create a client to make API calls against OpenSearch running on a specific url::Url.
let url = Url::parse("https://example.com")?;
let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url)).cert_validation(CertificateValidation::None).build()?;
let client = OpenSearch::new(transport);
```

## Index Template API Actions

### Create an Index Template

You can create an index template to define default settings and mappings for indices of certain patterns. The following example creates an index template named `books` with default settings and mappings for indices of the `books-*` pattern:

```rust
client.indices().put_index_template(IndicesPutIndexTemplateParts::Name("books"))
    .body(json!({
        "index_patterns": ["books-*"],
        "template": {
            "settings": {
                "index": {
                    "number_of_shards": 3,
                    "number_of_replicas": 0
                }
            },
            "mappings": {
                "properties": {
                    "title": { "type": "text" },
                    "author": { "type": "text" },
                    "published_on": { "type": "date" },
                    "pages": { "type": "integer" }
                }
            }
        }
    }))
    .send().await?;
```

Now, when you create an index that matches the `books-*` pattern, OpenSearch will automatically apply the template's settings and mappings to the index.
Let's create an index named `books-nonfiction` and verify that its settings and mappings match those of the template:

```rust
client.indices().create(IndicesCreateParts::Index("books-nonfiction")).send().await?;
client.indices().get(IndicesGetParts::Index(&["books-nonfiction"])).send().await?;
```

### Multiple Index Templates

If multiple index templates match the index's name, OpenSearch will apply the template with the highest priority. The following example creates two index templates named `books-*` and `books-fiction-*` with different settings:

```rust
client.indices().put_index_template(PutIndexTemplateParts::Name("books"))
    .body(json!({
        "index_patterns": ["books-*"],
        "priority": 0, // default priority
        "template": {
            "settings": {
                "index": {
                    "number_of_shards": 3,
                    "number_of_replicas": 0
                }
            }
        }
    }))
    .send().await?;

client.indices().put_index_template(PutIndexTemplateParts::Name("books-fiction"))
    .body(json!({
        "index_patterns": ["books-fiction-*"],
        "priority": 1, // higher priority than the `books` template
        "template": {
            "settings": {
                "index": {
                    "number_of_shards": 1,
                    "number_of_replicas": 1
                }
            }
        }
    }))
    .send().await?;
```

When we create an index named `books-fiction-romance`, OpenSearch will apply the `books-fiction-*` template's settings to the index:

```rust
client.indices().create(IndicesCreateParts::Index("books-fiction-romance")).send().await?;
client.indices().get(IndicesGetParts::Index(&["books-fiction-romance"])).send().await?;
```

### Composable Index Templates

Composable index templates are a new type of index template that allow you to define multiple component templates and compose them into a final template. The following example creates a component template named `books_mappings` with default mappings for indices of the `books-*` and `books-fiction-*` patterns:

```rust
client.cluster().put_component_template(ClusterPutComponentTemplateParts::Name("books_mappings"))
    .body(json!({
        "template": {
            "mappings": {
                "properties": {
                    "title": { "type": "text" },
                    "author": { "type": "text" },
                    "published_on": { "type": "date" },
                    "pages": { "type": "integer" }
                }
            }
        }
    }))
    .send()
    .await?;

client.indices().put_index_template(IndicesPutIndexTemplateParts::Name("books"))
    .body(json!({
        "index_patterns": ["books-*"],
        "composed_of": ["books_mappings"], // use the `books_mappings` component template
        "priority": 0,
        "template": {
            "settings": {
                "index": {
                    "number_of_shards": 3,
                    "number_of_replicas": 0
                }
            }
        }
    }))
    .send()
    .await?;

client.indices().put_index_template(IndicesPutIndexTemplateParts::Name("books"))
    .body(json!({
        "index_patterns": ["books-*"],
        "composed_of": ["books_mappings"], // use the `books_mappings` component template
        "priority": 1,
        "template": {
            "settings": {
                "index": {
                    "number_of_shards": 1,
                    "number_of_replicas": 1
                }
            }
        }
    }))
    .send()
    .await?;
```

When we create an index named `books-fiction-horror`, OpenSearch will apply the `books-fiction-*` template's settings, and `books_mappings` template mappings to the index:

```rust
client.indices().create(IndicesCreateParts::Index("books-fiction-horror")).send().await?;
client.indices().get(IndicesGetParts::Index(&["books-fiction-horror"])).send().await?;
```

### Get an Index Template

You can get an index template with the `get_index_template` API action:

```rust
client.indices().get_index_template(GetIndexTemplateParts::Name(&["books"])).send().await?;
```

### Delete an Index Template

You can delete an index template with the `delete_template` API action:

```rust
client.indices().delete_index_template(IndicesDeleteIndexTemplateParts::Name("books")).send().await?;
```

## Cleanup

Let's delete all resources created in this guide:

```rust
client.indices().delete(IndicesDeleteParts::Index(&["books-*"])).send().await?;
client.indices().delete_index_template(IndicesDeleteIndexTemplateParts::Name("books-fiction")).send().await?;
client.cluster().delete_component_template(ClusterDeleteComponentTemplateParts::Name("books_mappings")).send().await?;
```
