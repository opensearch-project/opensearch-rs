use opensearch::auth::Credentials;
use opensearch::cert::CertificateValidation;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::OpenSearch;
use opensearch::{
    indices::{
        IndicesCreateParts, IndicesDeleteParts, IndicesExistsParts, IndicesGetParts,
        IndicesPutMappingParts, IndicesPutSettingsParts,
    },
    IndexParts,
};
use serde_json::json;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client to make API calls to OpenSearch running on https://localhost:9200.
    let url = Url::parse("https://localhost:9200")?;
    let credentials = Credentials::Basic("admin".into(), "admin".into());
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
        .cert_validation(CertificateValidation::None)
        .auth(credentials)
        .build()?;
    let client = OpenSearch::new(transport);

    // Display Server Version
    println!("{:?}", client.info().send().await?);

    // You can quickly create an index with default settings and mappings by using the `indices.create` API action. The following example creates an index named `paintings` with default settings and mappings:
    client
        .indices()
        .create(IndicesCreateParts::Index("paintings"))
        .send()
        .await?;

    // To specify settings and mappings, you can pass them as the `body` of the request. The following example creates an index named `movies` with custom settings and mappings:
    client
        .indices()
        .create(IndicesCreateParts::Index("movies"))
        .body(json!({
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
        }))
        .send()
        .await?;

    // When you create a new document for an index, OpenSearch will automatically create the index if it doesn't exist:
    println!(
        "{}",
        client
            .indices()
            .exists(IndicesExistsParts::Index(&["burner"]))
            .send()
            .await?
            .json::<bool>()
            .await?
    ); // => false
    client
        .index(IndexParts::Index("burner"))
        .body(json!({ "lorem": "ipsum" }))
        .send()
        .await?;
    println!(
        "{}",
        client
            .indices()
            .exists(IndicesExistsParts::Index(&["burner"]))
            .send()
            .await?
            .json::<bool>()
            .await?
    ); // => true

    // You can update an index's settings and mappings by using the `indices.put_settings` and `indices.put_mapping` API actions.

    // The following example updates the `movies` index's number of replicas to `0`:
    client
        .indices()
        .put_settings(IndicesPutSettingsParts::Index(&["movies"]))
        .body(json!({
            "index": {
                "number_of_replicas": 0
            }
        }))
        .send()
        .await?;

    // The following example updates the `movies` index's mappings to add a new field named `director`:
    client
        .indices()
        .put_mapping(IndicesPutMappingParts::Index(&["movies"]))
        .body(json!({
            "properties": {
                "director": { "type": "text" }
            }
        }))
        .send()
        .await?;

    // Let's check if the index's settings and mappings have been updated by using the `indices.get` API action:
    println!(
        "{:#?}",
        client
            .indices()
            .get(IndicesGetParts::Index(&["movies"]))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?
    );

    // Let's delete the `movies` index by using the `indices.delete` API action:
    client
        .indices()
        .delete(IndicesDeleteParts::Index(&["movies"]))
        .send()
        .await?;

    // We can also delete multiple indices at once:
    client
        .indices()
        .delete(IndicesDeleteParts::Index(&[
            "movies",
            "paintings",
            "burner",
        ]))
        .ignore_unavailable(true)
        .send()
        .await?;
    Ok(())
}
