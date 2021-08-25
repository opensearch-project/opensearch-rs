# Examples

Create an index with a mapping

```rust,no_run
# use opensearch::{OpenSearch, Error, indices::IndicesCreateParts};
# use serde_json::{json, Value};
# async fn doc() -> Result<(), Box<dyn std::error::Error>> {
let client = OpenSearch::default();
let response = client
    .indices()
    .create(IndicesCreateParts::Index("test_index"))
    .body(json!({
        "mappings" : {
            "properties" : {
                "field1" : { "type" : "text" }
            }
        }
    }))
    .send()
    .await?;
    
# Ok(())
# }
```