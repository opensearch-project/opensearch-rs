/*
 * Licensed to Elasticsearch B.V. under one or more contributor
 * license agreements. See the NOTICE file distributed with
 * this work for additional information regarding copyright
 * ownership. Elasticsearch B.V. licenses this file to you under
 * the Apache License, Version 2.0 (the "License"); you may
 * not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *	http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

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

// From reqwest crate
// Licensed under Apache License, Version 2.0
// https://github.com/seanmonstar/reqwest/blob/master/LICENSE-APACHE

use std::{convert::identity, net::SocketAddr, sync::mpsc as std_mpsc, thread, time::Duration};

use bytes::Bytes;
use http_body_util::Empty;
use hyper::{
    body::Incoming, server::conn::http1, service::service_fn, HeaderMap, Method, Request, Response,
    Uri,
};
use hyper_util::rt::TokioIo;
use opensearch::{http::transport::TransportBuilder, OpenSearch};
use tokio::{
    net::{TcpListener, TcpStream},
    pin, runtime, select,
    sync::{mpsc, watch},
    task,
    time::sleep,
};

use super::client::TestClientBuilder;

#[derive(Clone)]
struct RequestState {
    requests_tx: mpsc::UnboundedSender<ReceivedRequest>,
    response_delay: Option<Duration>,
    shutdown_rx: watch::Receiver<bool>,
}

#[derive(Default)]
pub struct MockServerBuilder {
    response_delay: Option<Duration>,
}

impl MockServerBuilder {
    pub fn response_delay(mut self, delay: Duration) -> Self {
        self.response_delay = Some(delay);
        self
    }

    async fn handle_request(
        req: Request<Incoming>,
        state: RequestState,
    ) -> anyhow::Result<Response<Empty<Bytes>>> {
        state.requests_tx.send(req.into())?;
        if let Some(response_delay) = state.response_delay {
            sleep(response_delay).await;
        }
        Ok(Default::default())
    }

    async fn serve_connection(io: TokioIo<TcpStream>, state: RequestState) {
        let mut shutdown_rx = state.shutdown_rx.clone();
        let conn = http1::Builder::new().serve_connection(
            io,
            service_fn(move |req| Self::handle_request(req, state.clone())),
        );
        pin!(conn);
        select! {
            _ = conn.as_mut() => {},
            _ = shutdown_rx.changed() => conn.as_mut().graceful_shutdown()
        }
    }

    async fn serve(listener: TcpListener, state: RequestState) -> anyhow::Result<()> {
        let mut shutdown_rx = state.shutdown_rx.clone();
        loop {
            let (stream, _) = tokio::select! {
                res = listener.accept() => res?,
                _ = shutdown_rx.changed() => break
            };
            let io = TokioIo::new(stream);

            task::spawn(Self::serve_connection(io, state.clone()));
        }
        Ok(())
    }

    fn start_inner(self, thread_name: String) -> anyhow::Result<MockServer> {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let _ = rt.enter();

        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let (requests_tx, requests_rx) = mpsc::unbounded_channel();
        let listener = rt.block_on(TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))))?;
        let addr = listener.local_addr()?;

        let srv = Self::serve(
            listener,
            RequestState {
                requests_tx,
                response_delay: self.response_delay,
                shutdown_rx,
            },
        );

        let (panic_tx, panic_rx) = std_mpsc::channel();
        thread::Builder::new()
            .name(format!("test({})-support-server", thread_name))
            .spawn(move || {
                rt.block_on(srv).unwrap();
                let _ = panic_tx.send(());
            })?;

        Ok(MockServer {
            uri: format!("http://{}", addr),
            requests_rx,
            panic_rx,
            shutdown_tx: Some(shutdown_tx),
        })
    }

    pub fn start(self) -> anyhow::Result<MockServer> {
        let thread_name = thread::current().name().unwrap_or("<unknown>").to_owned();

        match thread::spawn(move || self.start_inner(thread_name)).join() {
            Ok(r) => r,
            Err(e) => Err(anyhow::anyhow!("MockServer startup panicked: {:?}", e)),
        }
    }
}

pub struct MockServer {
    uri: String,
    requests_rx: mpsc::UnboundedReceiver<ReceivedRequest>,
    panic_rx: std_mpsc::Receiver<()>,
    shutdown_tx: Option<watch::Sender<bool>>,
}

impl MockServer {
    pub fn builder() -> MockServerBuilder {
        MockServerBuilder::default()
    }

    pub fn start() -> anyhow::Result<Self> {
        Self::builder().start()
    }

    pub fn client(&self) -> OpenSearch {
        self.client_with(identity)
    }

    pub fn client_with(
        &self,
        configurator: impl FnOnce(TransportBuilder) -> TransportBuilder,
    ) -> OpenSearch {
        self.client_builder().with(configurator).build()
    }

    pub fn client_builder(&self) -> TestClientBuilder {
        super::client::builder_with_url(&self.uri)
    }

    pub async fn received_request(&mut self) -> anyhow::Result<ReceivedRequest> {
        self.requests_rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("no request received"))
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(true).unwrap();
        }

        if !::std::thread::panicking() {
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("test server should not panic");
        }
    }
}

pub struct ReceivedRequest {
    method: Method,
    uri: Uri,
    headers: HeaderMap,
}

impl ReceivedRequest {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn path(&self) -> &str {
        self.uri.path()
    }

    pub fn query(&self) -> Option<&str> {
        self.uri.query()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| v.to_str().ok())
    }
}

impl From<Request<Incoming>> for ReceivedRequest {
    fn from(req: Request<Incoming>) -> Self {
        ReceivedRequest {
            method: req.method().clone(),
            uri: req.uri().clone(),
            headers: req.headers().clone(),
        }
    }
}
