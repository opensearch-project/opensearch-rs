use serde_json::{json, Value};

use opensearch::auth::Credentials;
use opensearch::cert::CertificateValidation;
use opensearch::http::headers::HeaderMap;
use opensearch::http::transport::{SingleNodeConnectionPool, TransportBuilder};
use opensearch::http::{Method, Url};
use opensearch::OpenSearch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = Url::parse("https://localhost:9200")?;
    let credentials = Credentials::Basic("admin".into(), "admin".into());
    let transport = TransportBuilder::new(SingleNodeConnectionPool::new(url))
        .cert_validation(CertificateValidation::None)
        .auth(credentials)
        .build()?;
    let client = OpenSearch::new(transport);

    let index_name = "movies";
    let document_id = "1";

    let info: Value = client
        .send(
            Method::Get,
            "/",
            HeaderMap::new(),
            Option::<&String>::None,
            Option::<&String>::None,
            None,
        )
        .await?
        .json()
        .await?;

    println!(
        "Welcome to {} {}",
        info["version"]["distribution"], info["version"]["number"]
    );

    // Create an index
    let index_body = json!({
        "settings": {
            "index": {
                "number_of_shards" : 4
            }
        }
    });

    let create_index_response = client
        .send(
            Method::Put,
            &format!("/{index_name}"),
            HeaderMap::new(),
            Option::<&String>::None,
            Some(index_body.to_string()),
            None,
        )
        .await?;

    assert_eq!(create_index_response.status_code(), 200);

    // add a document to the index
    let document = json!({
        "title": "Moneyball",
        "director": "Bennett Miller",
        "year": "2011"
    });
    let create_document_response = client
        .send(
            Method::Put,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            Some(&vec![("refresh", "true")]),
            Some(index_body.to_string()),
            None,
        )
        .await?;

    assert_eq!(create_index_response.status_code(), 200);

    // Search for a document
    let q = "miller";
    let query = json!({
        "size": 5,
        "query": {
            "multi_match": {
                "query": q,
                "fields": ["title^2", "director"]
            }
        }
    });

    let search_response = client
        .send(
            Method::Post,
            "/movies/_search",
            HeaderMap::new(),
            Option::<&String>::None,
            Some(query.to_string()),
            None,
        )
        .await?;

    assert_eq!(search_response.status_code(), 200);

    // Delete the document
    let delete_document_response = client
        .send(
            Method::Delete,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            Option::<&String>::None,
            Option::<&String>::None,
            None,
        )
        .await?;

    assert_eq!(delete_document_response.status_code(), 200);

    // Delete the index
    let delete_response = client
        .send(
            Method::Delete,
            &format!("/{index_name}"),
            HeaderMap::new(),
            Option::<&String>::None,
            Option::<&String>::None,
            None,
        )
        .await?;

    assert_eq!(delete_response.status_code(), 200);

    Ok(())
}
