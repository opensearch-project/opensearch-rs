/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! The retry policy of the transport: which failures are retried, how many
//! times, on which node, and with what request.

pub mod common;

use std::{
    net::TcpListener,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use common::server::MockServer;
use opensearch::{
    http::transport::{MultiNodeConnectionPool, TransportBuilder},
    http::Url,
    IndexParts, OpenSearch,
};
use reqwest::RequestBuilder;
use serde_json::json;

const OK: (u16, &str) = (200, "{}");
const UNAVAILABLE: (u16, &str) = (503, r#"{"error":"unavailable"}"#);

/// A client of the server retrying without delay, to keep the tests fast
fn client(
    server: &MockServer,
    configure: impl FnOnce(TransportBuilder) -> TransportBuilder,
) -> OpenSearch {
    server.client_with(|b| configure(b.retry_backoff(Duration::ZERO)))
}

/// The address of a port nothing listens on: connecting is refused
fn refused_address() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = listener.local_addr().unwrap();
    drop(listener);
    format!("http://{}", addr)
}

#[tokio::test]
async fn a_retryable_status_is_retried_until_success() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, (502, ""), OK])
        .start()?;
    let response = client(&server, |b| b).ping().send().await?;

    assert_eq!(response.status_code(), 200);
    assert_eq!(server.received_requests().len(), 3);
    Ok(())
}

#[tokio::test]
async fn the_last_response_is_returned_when_the_retries_are_exhausted() -> anyhow::Result<()> {
    let mut server = MockServer::builder().response(503, UNAVAILABLE.1).start()?;
    let response = client(&server, |b| b.max_retries(2)).info().send().await?;

    // the generic client answers with the response, whatever its status
    assert_eq!(response.status_code(), 503);
    assert_eq!(response.text().await?, UNAVAILABLE.1);
    // the first attempt and 2 retries
    assert_eq!(server.received_requests().len(), 3);
    Ok(())
}

#[tokio::test]
async fn three_retries_by_default() -> anyhow::Result<()> {
    let mut server = MockServer::builder().response(504, "").start()?;
    let response = client(&server, |b| b).ping().send().await?;

    assert_eq!(response.status_code(), 504);
    assert_eq!(server.received_requests().len(), 4);
    Ok(())
}

#[tokio::test]
async fn retries_can_be_disabled() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, OK])
        .start()?;
    let response = client(&server, |b| b.disable_retry()).ping().send().await?;

    assert_eq!(response.status_code(), 503);
    assert_eq!(server.received_requests().len(), 1);
    Ok(())
}

#[tokio::test]
async fn other_statuses_are_not_retried() -> anyhow::Result<()> {
    for status in [400, 404, 429, 500] {
        let mut server = MockServer::builder()
            .responses(&[(status, "{}"), OK])
            .start()?;
        let response = client(&server, |b| b).ping().send().await?;

        assert_eq!(response.status_code(), status);
        assert_eq!(server.received_requests().len(), 1, "status {}", status);
    }
    Ok(())
}

#[tokio::test]
async fn the_retried_statuses_can_be_chosen() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[(429, "{}"), OK])
        .start()?;
    let response = client(&server, |b| b.retry_on_status(&[429]))
        .ping()
        .send()
        .await?;
    assert_eq!(response.status_code(), 200);
    assert_eq!(server.received_requests().len(), 2);

    // 503 is no longer in the list
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, OK])
        .start()?;
    let response = client(&server, |b| b.retry_on_status(&[429]))
        .ping()
        .send()
        .await?;
    assert_eq!(response.status_code(), 503);
    assert_eq!(server.received_requests().len(), 1);
    Ok(())
}

#[tokio::test]
async fn a_retry_sends_the_same_request() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, OK])
        .start()?;
    let response = client(&server, |b| b)
        .index(IndexParts::IndexId("movies", "1"))
        .refresh(opensearch::params::Refresh::True)
        .body(json!({ "title": "Moneyball" }))
        .send()
        .await?;
    assert_eq!(response.status_code(), 200);

    let requests = server.received_requests();
    assert_eq!(requests.len(), 2);
    let (first, retry) = (&requests[0], &requests[1]);
    assert_eq!(first.method(), retry.method());
    assert_eq!(first.path(), "/movies/_doc/1");
    assert_eq!(first.path(), retry.path());
    assert_eq!(first.query(), retry.query());
    assert_eq!(first.body(), br#"{"title":"Moneyball"}"#);
    assert_eq!(first.body(), retry.body());
    assert_eq!(first.header("content-type"), retry.header("content-type"));
    Ok(())
}

#[tokio::test]
async fn request_initializers_run_for_every_attempt() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, UNAVAILABLE, OK])
        .start()?;
    let attempts = Arc::new(AtomicUsize::new(1));
    let counter = {
        let attempts = attempts.clone();
        move |request: RequestBuilder| {
            let n = attempts.fetch_add(1, Ordering::SeqCst);
            request.header("x-attempt", n.to_string())
        }
    };
    client(&server, |b| b.with_initializer_fn(counter))
        .ping()
        .send()
        .await?;

    let headers: Vec<String> = server
        .received_requests()
        .iter()
        .map(|r| r.header("x-attempt").unwrap().to_string())
        .collect();
    assert_eq!(headers, ["1", "2", "3"]);
    Ok(())
}

#[tokio::test]
async fn a_refused_connection_fails_over_to_the_next_node() -> anyhow::Result<()> {
    let mut server = MockServer::builder().response(200, "{}").start()?;
    let pool = MultiNodeConnectionPool::round_robin(vec![
        Url::parse(&refused_address())?,
        Url::parse(server.uri())?,
    ]);
    let transport = TransportBuilder::new(pool)
        .retry_backoff(Duration::ZERO)
        .build()?;
    let response = OpenSearch::new(transport).ping().send().await?;

    assert_eq!(response.status_code(), 200);
    assert_eq!(server.received_requests().len(), 1);
    Ok(())
}

#[tokio::test]
async fn a_refused_connection_is_an_error_without_retries() -> anyhow::Result<()> {
    let pool = MultiNodeConnectionPool::round_robin(vec![Url::parse(&refused_address())?]);
    let transport = TransportBuilder::new(pool).disable_retry().build()?;
    let err = OpenSearch::new(transport).ping().send().await.unwrap_err();

    assert!(err.to_string().contains("http error"), "{}", err);
    Ok(())
}

#[tokio::test]
async fn timeouts_are_not_retried_by_default() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .response_delay(Duration::from_millis(500))
        .start()?;
    let err = client(&server, |b| b.timeout(Duration::from_millis(100)))
        .ping()
        .send()
        .await
        .unwrap_err();

    assert!(err.is_timeout(), "{}", err);
    assert_eq!(server.received_requests().len(), 1);
    Ok(())
}

#[tokio::test]
async fn timeouts_can_be_retried() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .response_delay(Duration::from_millis(500))
        .start()?;
    let err = client(&server, |b| {
        b.timeout(Duration::from_millis(100))
            .retry_on_timeout(true)
            .max_retries(2)
    })
    .ping()
    .send()
    .await
    .unwrap_err();

    assert!(err.is_timeout(), "{}", err);
    // the server records a request on arrival, before the delay
    tokio::time::sleep(Duration::from_millis(50)).await;
    assert_eq!(server.received_requests().len(), 3);
    Ok(())
}

#[tokio::test]
async fn retries_back_off() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, OK])
        .start()?;
    // with no budget and a fixed ceiling, the one delay is at most 300ms
    let client = server.client_with(|b| {
        b.retry_backoff(Duration::from_millis(300))
            .retry_budget(None)
    });
    let started = Instant::now();
    let response = client.ping().send().await?;

    assert_eq!(response.status_code(), 200);
    assert!(started.elapsed() < Duration::from_secs(2));
    assert_eq!(server.received_requests().len(), 2);
    Ok(())
}

#[tokio::test]
async fn a_retry_past_the_budget_is_not_made() -> anyhow::Result<()> {
    let mut server = MockServer::builder()
        .responses(&[UNAVAILABLE, OK])
        .start()?;
    // every delay is drawn from 0..=10s, so a zero budget leaves no room for one
    let client = server.client_with(|b| {
        b.retry_backoff(Duration::from_secs(10))
            .retry_budget(Some(Duration::ZERO))
    });
    let started = Instant::now();
    let response = client.ping().send().await?;

    assert_eq!(response.status_code(), 503);
    assert!(started.elapsed() < Duration::from_secs(2));
    assert_eq!(server.received_requests().len(), 1);
    Ok(())
}
