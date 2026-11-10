/*
 * SPDX-License-Identifier: Apache-2.0
 *
 * The OpenSearch Contributors require contributions made to
 * this file be licensed under the Apache-2.0 license or a
 * compatible open source license.
 */

//! Query parameters whose specification schema is a union take the type
//! generated from it, and are sent in their wire form.

pub mod common;

use common::server::MockServer;
use opensearch::{
    cluster::ClusterHealthParts,
    params::Slices as LegacySlices,
    types::common::{Slices, SlicesCalculation},
    DeleteByQueryParts,
};

async fn query_of(
    send: impl FnOnce(&opensearch::OpenSearch) -> futures::future::BoxFuture<'_, ()>,
) -> anyhow::Result<String> {
    let mut server = MockServer::start()?;
    let client = server.client();
    send(&client).await;
    Ok(server
        .received_request()
        .await?
        .query()
        .unwrap_or("")
        .to_string())
}

#[tokio::test]
async fn slices_are_sent_as_a_count_or_auto() -> anyhow::Result<()> {
    let count = query_of(|c| {
        Box::pin(async move {
            let _ = c
                .delete_by_query(DeleteByQueryParts::Index(&["movies"]))
                .slices(5)
                .send()
                .await;
        })
    })
    .await?;
    assert_eq!(count, "slices=5");

    let auto = query_of(|c| {
        Box::pin(async move {
            let _ = c
                .delete_by_query(DeleteByQueryParts::Index(&["movies"]))
                .slices(SlicesCalculation::Auto)
                .send()
                .await;
        })
    })
    .await?;
    assert_eq!(auto, "slices=auto");
    Ok(())
}

#[tokio::test]
async fn the_2x_slices_type_still_converts() -> anyhow::Result<()> {
    let auto = query_of(|c| {
        Box::pin(async move {
            let _ = c
                .delete_by_query(DeleteByQueryParts::Index(&["movies"]))
                .slices(LegacySlices::Auto)
                .send()
                .await;
        })
    })
    .await?;
    assert_eq!(auto, "slices=auto");
    let explicit: Slices = LegacySlices::Count(3).into();
    assert!(matches!(explicit, Slices::Count(3)));
    Ok(())
}

#[tokio::test]
async fn wait_for_nodes_takes_a_count_or_a_comparison() -> anyhow::Result<()> {
    let count = query_of(|c| {
        Box::pin(async move {
            let _ = c
                .cluster()
                .health(ClusterHealthParts::None)
                .wait_for_nodes(3)
                .send()
                .await;
        })
    })
    .await?;
    assert_eq!(count, "wait_for_nodes=3");

    // a string literal still works, as on 2.x
    let comparison = query_of(|c| {
        Box::pin(async move {
            let _ = c
                .cluster()
                .health(ClusterHealthParts::None)
                .wait_for_nodes(">=3")
                .send()
                .await;
        })
    })
    .await?;
    assert_eq!(comparison, "wait_for_nodes=%3E%3D3");
    Ok(())
}

#[tokio::test]
async fn stringified_booleans_are_booleans() -> anyhow::Result<()> {
    let query = query_of(|c| {
        Box::pin(async move {
            let _ = c.cat().nodes().full_id(true).send().await;
        })
    })
    .await?;
    assert_eq!(query, "full_id=true");
    Ok(())
}

#[test]
fn union_values_round_trip_as_json() -> anyhow::Result<()> {
    let auto: Slices = serde_json::from_str(r#""auto""#)?;
    assert!(matches!(auto, Slices::Calculation(SlicesCalculation::Auto)));
    assert_eq!(serde_json::to_string(&auto)?, r#""auto""#);
    let count: Slices = serde_json::from_str("4")?;
    assert!(matches!(count, Slices::Count(4)));
    Ok(())
}
