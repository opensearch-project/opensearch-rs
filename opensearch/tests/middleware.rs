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

mod common;

use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use opensearch::http::middleware::{
    async_trait, RequestHandler, RequestInitializer, RequestPipeline, RequestPipelineError,
};
use reqwest::RequestBuilder;

use crate::common::{server::MockServer, tracing_init};

#[tokio::test]
async fn request_initializer() -> anyhow::Result<()> {
    #[derive(Debug, Clone)]
    struct Counter(Arc<AtomicUsize>);

    impl RequestInitializer for Counter {
        type Result = RequestBuilder;

        fn init(&self, request: RequestBuilder) -> Self::Result {
            let counter = self.0.fetch_add(1, Ordering::SeqCst);
            request.header("x-counter", counter.to_string())
        }
    }

    tracing_init();

    let mut server = MockServer::start()?;

    let counter_fn = {
        let counter = Arc::new(AtomicUsize::new(1));
        move |request_builder: RequestBuilder| {
            let counter = counter.fetch_add(1, Ordering::SeqCst);
            request_builder.header("x-counter-fn", counter.to_string())
        }
    };

    let client = server.client_with(|b| {
        b.with_req_init(Counter(Arc::new(AtomicUsize::new(101))))
            .with_req_init_fn(counter_fn)
    });

    client.ping().send().await?;
    client.ping().send().await?;

    let req1 = server.received_request().await?;
    let req2 = server.received_request().await?;

    assert_eq!(req1.header("x-counter-fn"), Some("1"));
    assert_eq!(req1.header("x-counter"), Some("101"));

    assert_eq!(req2.header("x-counter-fn"), Some("2"));
    assert_eq!(req2.header("x-counter"), Some("102"));

    Ok(())
}

#[tokio::test]
async fn request_handler() -> anyhow::Result<()> {
    #[derive(Debug, Clone)]
    struct Handler(Arc<AtomicUsize>);

    #[async_trait]
    impl RequestHandler for Handler {
        async fn handle(
            &self,
            request: reqwest::Request,
            next: RequestPipeline<'_>,
        ) -> Result<reqwest::Response, RequestPipelineError> {
            self.0.fetch_add(1, Ordering::SeqCst);
            next.run(request).await
        }
    }

    tracing_init();

    let server = MockServer::start()?;

    let handler_called = Arc::new(AtomicUsize::new(0));

    let client = server.client_with(|b| b.with_handler(Handler(handler_called.clone())));

    client.ping().send().await?;

    assert_eq!(handler_called.load(Ordering::SeqCst), 1);

    client.ping().send().await?;

    assert_eq!(handler_called.load(Ordering::SeqCst), 2);

    Ok(())
}
