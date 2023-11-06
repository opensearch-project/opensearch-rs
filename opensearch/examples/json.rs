/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 *
 * Modifications Copyright OpenSearch Contributors. See
 * GitHub history for details.
 */

use opensearch::{
    auth::Credentials,
    cert::CertificateValidation,
    http::{
        headers::HeaderMap,
        request::JsonBody,
        transport::{SingleNodeConnectionPool, TransportBuilder},
        Method, Url,
    },
    OpenSearch,
};
use serde_json::{json, Value};

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
        .send::<(), ()>(Method::Get, "/", HeaderMap::new(), None, None, None)
        .await?
        .json()
        .await?;

    println!(
        "Welcome to {} {}",
        info["version"]["distribution"], info["version"]["number"]
    );

    // Create an index
    let index_body: JsonBody<_> = json!({
        "settings": {
            "index": {
                "number_of_shards" : 4
            }
        }
    })
    .into();

    let create_index_response = client
        .send(
            Method::Put,
            &format!("/{index_name}"),
            HeaderMap::new(),
            Option::<&()>::None,
            Some(index_body),
            None,
        )
        .await?;

    assert_eq!(create_index_response.status_code(), 200);

    // add a document to the index
    let document: JsonBody<_> = json!({
        "title": "Moneyball",
        "director": "Bennett Miller",
        "year": "2011"
    })
    .into();
    let create_document_response = client
        .send(
            Method::Put,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            Some(&[("refresh", "true")]),
            Some(document),
            None,
        )
        .await?;

    assert_eq!(create_document_response.status_code(), 201);

    // Search for a document
    let q = "miller";
    let query: JsonBody<_> = json!({
        "size": 5,
        "query": {
            "multi_match": {
                "query": q,
                "fields": ["title^2", "director"]
            }
        }
    })
    .into();

    let search_response = client
        .send(
            Method::Post,
            &format!("/{index_name}/_search"),
            HeaderMap::new(),
            Option::<&()>::None,
            Some(query),
            None,
        )
        .await?;

    assert_eq!(search_response.status_code(), 200);
    let search_result = search_response.json::<Value>().await?;
    println!(
        "Hits: {:#?}",
        search_result["hits"]["hits"].as_array().unwrap()
    );

    // Delete the document
    let delete_document_response = client
        .send::<(), ()>(
            Method::Delete,
            &format!("/{index_name}/_doc/{document_id}"),
            HeaderMap::new(),
            None,
            None,
            None,
        )
        .await?;

    assert_eq!(delete_document_response.status_code(), 200);

    // Delete the index
    let delete_response = client
        .send::<(), ()>(
            Method::Delete,
            &format!("/{index_name}"),
            HeaderMap::new(),
            None,
            None,
            None,
        )
        .await?;

    assert_eq!(delete_response.status_code(), 200);

    Ok(())
}
