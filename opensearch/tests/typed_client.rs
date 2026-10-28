/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The typed client: `send` decodes the body as the typed response model
//! of the endpoint, and reports errors as `Error`

pub mod common;
use common::server::MockServer;

use opensearch::{http::StatusCode, GetParts, SearchParts};

const SEARCH: &str = r#"{"took":3,"timed_out":false,
  "_shards":{"total":1,"successful":1,"skipped":0,"failed":0},
  "hits":{"total":{"value":1,"relation":"eq"},"max_score":1.0,
    "hits":[{"_index":"movies","_id":"1","_score":1.0,"_source":{"title":"Moneyball"}}]}}"#;

#[tokio::test]
async fn success_is_decoded_as_the_model() -> anyhow::Result<()> {
    let server = MockServer::builder().response(200, SEARCH).start()?;
    let resp = server
        .client()
        .typed()
        .search(SearchParts::Index(&["movies"]))
        .send()
        .await?;
    assert_eq!(resp.took, 3);
    let doc: serde_json::Value = resp.hits.hits[0].source_as()?.unwrap();
    assert_eq!(doc["title"], "Moneyball");
    Ok(())
}

#[tokio::test]
async fn not_found_answered_with_the_model_is_not_an_error() -> anyhow::Result<()> {
    let server = MockServer::builder()
        .response(404, r#"{"_index":"movies","_id":"nope","found":false}"#)
        .start()?;
    let resp = server
        .client()
        .typed()
        .get(GetParts::IndexId("movies", "nope"))
        .send()
        .await?;
    assert!(!resp.found);
    Ok(())
}

#[tokio::test]
async fn error_status_is_an_api_error() -> anyhow::Result<()> {
    let server = MockServer::builder()
        .response(
            404,
            r#"{"error":{"root_cause":[{"type":"index_not_found_exception","reason":"no such index [nope]"}],
               "type":"index_not_found_exception","reason":"no such index [nope]"},"status":404}"#,
        )
        .start()?;
    let err = server
        .client()
        .typed()
        .search(SearchParts::Index(&["nope"]))
        .send()
        .await
        .unwrap_err();

    assert_eq!(err.status_code(), Some(StatusCode::NOT_FOUND));
    let api = err.api_error().expect("an error response");
    assert_eq!(api.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        api.exception().unwrap().error().ty(),
        Some("index_not_found_exception")
    );
    assert!(api.body().contains("no such index"));
    assert!(
        err.to_string().contains("404 Not Found for http://"),
        "{}",
        err
    );
    assert!(
        err.to_string()
            .ends_with(": index_not_found_exception: no such index [nope]"),
        "{}",
        err
    );
    Ok(())
}

#[tokio::test]
async fn error_in_a_plugin_shape_keeps_the_body() -> anyhow::Result<()> {
    // the security plugin answers errors with its own body
    let server = MockServer::builder()
        .response(
            500,
            r#"{"status":"INTERNAL_SERVER_ERROR","message":"boom"}"#,
        )
        .start()?;
    let err = server
        .client()
        .typed()
        .search(SearchParts::None)
        .send()
        .await
        .unwrap_err();
    let api = err.api_error().unwrap();
    assert_eq!(api.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert!(api.exception().is_none());
    assert!(err.to_string().contains("\"message\":\"boom\""), "{}", err);
    Ok(())
}

#[tokio::test]
async fn decode_error_names_the_type_and_the_cause() -> anyhow::Result<()> {
    let server = MockServer::builder()
        .response(200, r#"{"took":3}"#)
        .start()?;
    let err = server
        .client()
        .typed()
        .search(SearchParts::None)
        .send()
        .await
        .unwrap_err();
    assert!(err.is_json());
    assert!(err.api_error().is_none());
    let message = err.to_string();
    assert!(
        message.contains("types::search::SearchResponse"),
        "{}",
        message
    );
    assert!(message.contains("missing field"), "{}", message);
    Ok(())
}

#[tokio::test]
async fn model_status_with_an_error_body_is_an_api_error() -> anyhow::Result<()> {
    // `get` answers 404 for a missing document (the model) and for a
    // missing index (an error body)
    let server = MockServer::builder()
        .response(
            404,
            r#"{"error":{"type":"index_not_found_exception","reason":"no such index [nope]"},"status":404}"#,
        )
        .start()?;
    let err = server
        .client()
        .typed()
        .get(GetParts::IndexId("nope", "1"))
        .send()
        .await
        .unwrap_err();
    assert_eq!(
        err.api_error().unwrap().exception().unwrap().error().ty(),
        Some("index_not_found_exception")
    );
    Ok(())
}

#[tokio::test]
async fn the_typed_client_shares_the_transport() -> anyhow::Result<()> {
    let mut server = MockServer::builder().response(200, SEARCH).start()?;
    let client = server.client();
    let typed: opensearch::typed::TypedOpenSearch = client.typed();
    let resp = typed
        .search(SearchParts::Index(&["movies"]))
        .size(1)
        .body(serde_json::json!({ "query": { "match_all": {} } }))
        .send()
        .await?;
    assert_eq!(resp.took, 3);

    // options are those of the generic builder, sent the same way
    let request = server.received_request().await?;
    assert_eq!(request.path(), "/movies/_search");
    assert_eq!(request.query(), Some("size=1"));
    Ok(())
}

/// A search response with keys the model does not define: at the top
/// level, nested in `_shards`, inside a hit, and inside `hits.total` (a
/// union), plus a `null` one
const SEARCH_WITH_UNKNOWN_FIELDS: &str = r#"{"took":3,"timed_out":false,
  "_shards":{"total":1,"successful":1,"skipped":0,"failed":0,"extra":[1,2]},
  "hits":{"total":{"value":1,"relation":"eq","approx":true},"max_score":1.0,
    "hits":[{"_index":"movies","_id":"1","_score":1.0,"_source":{"title":"Moneyball"},"_plugin":"x"}]},
  "ext":{"rag":{"answer":"42"}},"absent":null}"#;

#[tokio::test]
async fn unknown_fields_are_ignored_by_default() -> anyhow::Result<()> {
    let server = MockServer::builder()
        .response(200, SEARCH_WITH_UNKNOWN_FIELDS)
        .start()?;
    let resp = server
        .client()
        .typed()
        .search(SearchParts::Index(&["movies"]))
        .send()
        .await?;
    assert_eq!(resp.took, 3);
    assert_eq!(resp.hits.total_value(), Some(1));
    // keys the specification allows are kept where it declares them
    assert_eq!(resp.hits.hits[0].additional["_plugin"].get(), r#""x""#);
    Ok(())
}

#[tokio::test]
async fn unknown_fields_can_be_reported() -> anyhow::Result<()> {
    let server = MockServer::builder()
        .response(200, SEARCH_WITH_UNKNOWN_FIELDS)
        .start()?;
    let err = server
        .client()
        .typed()
        .ignore_unknown_fields(false)
        .search(SearchParts::Index(&["movies"]))
        .size(1) // the setting survives the builder options
        .send()
        .await
        .unwrap_err();

    // every unknown key, wherever it is, by its path; `null` is not
    // reported, and neither is a key the specification allows: hits declare
    // `additionalProperties`, so the hit's `_plugin` is kept in `additional`
    assert_eq!(
        err.unknown_fields().unwrap(),
        &[
            "._shards.extra".to_string(),
            ".ext".to_string(),
            ".hits.total.approx".to_string(),
        ]
    );
    assert!(err.is_json());
    assert!(err.api_error().is_none());
    let message = err.to_string();
    assert!(
        message.contains("types::search::SearchResponse"),
        "{}",
        message
    );
    assert!(message.contains(".ext"), "{}", message);
    Ok(())
}

#[tokio::test]
async fn a_response_the_model_fully_describes_passes_the_strict_check() -> anyhow::Result<()> {
    let server = MockServer::builder().response(200, SEARCH).start()?;
    let resp = server
        .client()
        .typed()
        .ignore_unknown_fields(false)
        .search(SearchParts::Index(&["movies"]))
        .send()
        .await?;
    assert_eq!(resp.took, 3);

    // a 404 answered with the model is checked the same way
    let server = MockServer::builder()
        .response(404, r#"{"_index":"movies","_id":"nope","found":false}"#)
        .start()?;
    let doc = server
        .client()
        .typed()
        .ignore_unknown_fields(false)
        .get(GetParts::IndexId("movies", "nope"))
        .send()
        .await?;
    assert!(!doc.found);
    Ok(())
}
