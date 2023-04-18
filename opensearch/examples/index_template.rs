use opensearch::cluster::{ClusterDeleteComponentTemplateParts, ClusterPutComponentTemplateParts};
use opensearch::indices::{
    IndicesCreateParts, IndicesDeleteIndexTemplateParts, IndicesDeleteParts,
    IndicesGetIndexTemplateParts, IndicesGetParts, IndicesPutIndexTemplateParts,
};
use opensearch::{
    cert::CertificateValidation, http::transport::SingleNodeConnectionPool,
    http::transport::TransportBuilder, OpenSearch,
};
use serde_json::json;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Let's create a client instance, and an index named `movies`:

    // Create a client to make API calls to OpenSearch running on http://localhost:9200.

    let url = Url::parse("https://localhost:9200")?; 
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
        .cert_validation(CertificateValidation::None)
        .build()?;
    let client = OpenSearch::new(transport);

    // You can create an index template to define default settings and mappings for indices of certain patterns. The following example creates an index template named `books` with default settings and mappings for indices of the `books-*` pattern:

    client
        .indices()
        .put_index_template(IndicesPutIndexTemplateParts::Name("books"))
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
        .send()
        .await?;

    // Now, when you create an index that matches the `books-*` pattern, OpenSearch will automatically apply the template's settings and mappings to the index.

    // Let's create an index named `books-nonfiction` and verify that its settings and mappings match those of the template:

    client
        .indices()
        .create(IndicesCreateParts::Index("books-nonfiction"))
        .send()
        .await?;
    client
        .indices()
        .get(IndicesGetParts::Index(&["books-nonfiction"]))
        .send()
        .await?;

    // If multiple index templates match the index's name, OpenSearch will apply the template with the highest priority. The following example creates two index templates named `books-*` and `books-fiction-*` with different settings:

    client
        .indices()
        .put_index_template(IndicesPutIndexTemplateParts::Name("books"))
        .body(json!({
            "index_patterns": ["books-*"],
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

    client
        .indices()
        .put_index_template(IndicesPutIndexTemplateParts::Name("books-fiction"))
        .body(json!({
            "index_patterns": ["books-fiction-*"],
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

    // When we create an index named `books-fiction-romance`, OpenSearch will apply the `books-fiction-*` template's settings to the index:

    client
        .indices()
        .create(IndicesCreateParts::Index("books-fiction-romance"))
        .send()
        .await?;
    client
        .indices()
        .get(IndicesGetParts::Index(&["books-fiction-romance"]))
        .send()
        .await?;

    // Composable index templates are a new type of index template that allow you to define multiple component templates and compose them into a final template. The following example creates a component template named `books_mappings` with default mappings for indices of the `books-*` and `books-fiction-*` patterns:

    client
        .cluster()
        .put_component_template(ClusterPutComponentTemplateParts::Name("books_mappings"))
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

    client
        .indices()
        .put_index_template(IndicesPutIndexTemplateParts::Name("books"))
        .body(json!({
            "index_patterns": ["books-*"],
            "composed_of": ["books_mappings"],
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

    client
        .indices()
        .put_index_template(IndicesPutIndexTemplateParts::Name("books"))
        .body(json!({
            "index_patterns": ["books-*"],
            "composed_of": ["books_mappings"],
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

    // When we create an index named `books-fiction-horror`, OpenSearch will apply the `books-fiction-*` template's settings, and `books_mappings` template mappings to the index:

    client
        .indices()
        .create(IndicesCreateParts::Index("books-fiction-horror"))
        .send()
        .await?;
    client
        .indices()
        .get(IndicesGetParts::Index(&["books-fiction-horror"]))
        .send()
        .await?;

    // You can get an index template with the `get_index_template` API action:

    client
        .indices()
        .get_index_template(IndicesGetIndexTemplateParts::Name(&["books"]))
        .send()
        .await?;

    // You can delete an index template with the `delete_template` API action:

    client
        .indices()
        .delete_index_template(IndicesDeleteIndexTemplateParts::Name("books"))
        .send()
        .await?;

    // Let's delete all resources created in this guide:

    client
        .indices()
        .delete(IndicesDeleteParts::Index(&["books-*"]))
        .send()
        .await?;
    client
        .indices()
        .delete_index_template(IndicesDeleteIndexTemplateParts::Name("books-fiction"))
        .send()
        .await?;
    client
        .cluster()
        .delete_component_template(ClusterDeleteComponentTemplateParts::Name("books_mappings"))
        .send()
        .await?;
    Ok(())
}
