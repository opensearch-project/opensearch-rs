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

use std::{
    convert::Infallible,
    future::Future,
    net::{self, SocketAddr},
    sync::mpsc as std_mpsc,
    thread,
    time::Duration,
};

use bytes::Bytes;
use http_body_util::Empty;
use hyper::{
    body::{Body, Incoming},
    server::conn::http1,
    service::service_fn,
    Request, Response,
};
use hyper_util::rt::TokioIo;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{broadcast, mpsc},
};

use tokio::runtime;

pub struct Server {
    addr: net::SocketAddr,
    panic_rx: std_mpsc::Receiver<()>,
    shutdown_tx: Option<broadcast::Sender<()>>,
}

impl Server {
    pub fn addr(&self) -> net::SocketAddr {
        self.addr
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            tx.send(()).unwrap();
        }

        if !::std::thread::panicking() {
            self.panic_rx
                .recv_timeout(Duration::from_secs(3))
                .expect("test server should not panic");
        }
    }
}

pub fn http<F, Fut, B>(func: F) -> Server
where
    F: Fn(Request<Incoming>) -> Fut + Clone + Send + 'static,
    Fut: Future<Output = Response<B>> + Send + 'static,
    B: Body + Send + 'static,
    B::Data: Send,
    B::Error: std::error::Error + Send + Sync,
{
    let thread_name = thread::current().name().unwrap_or("<unknown>").to_owned();

    thread::spawn(move || {
        let rt = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("new rt");
        let _ = rt.enter();

        let (shutdown_tx, mut shutdown_rx) = broadcast::channel(1);
        let listener = rt
            .block_on(TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], 0))))
            .unwrap();
        let addr = listener.local_addr().unwrap();

        let srv = async move {
            loop {
                let (stream, _) = tokio::select! {
                    res = listener.accept() => res?,
                    _ = shutdown_rx.recv() => break
                };
                let io = TokioIo::new(stream);

                let mut func = func.clone();
                let mut shutdown_rx = shutdown_rx.resubscribe();

                tokio::task::spawn(async move {
                    let conn = http1::Builder::new().serve_connection(
                        io,
                        service_fn(move |req| {
                            let func = func.clone();
                            async move { Ok::<_, Infallible>(func(req).await) }
                        }),
                    );
                    tokio::pin!(conn);
                    tokio::select! {
                        res = conn.as_mut() => {},
                        _ = shutdown_rx.recv() => conn.as_mut().graceful_shutdown()
                    }
                });
            }
            Ok::<(), anyhow::Error>(())
        };

        let (panic_tx, panic_rx) = std_mpsc::channel();
        let thread_name = format!("test({})-support-server", thread_name);
        thread::Builder::new()
            .name(thread_name)
            .spawn(move || {
                rt.block_on(srv).unwrap();
                let _ = panic_tx.send(());
            })
            .expect("thread spawn");

        Server {
            addr,
            panic_rx,
            shutdown_tx: Some(shutdown_tx),
        }
    })
    .join()
    .unwrap()
}

pub fn capturing_http() -> (Server, mpsc::UnboundedReceiver<Request<Incoming>>) {
    let (tx, rx) = mpsc::unbounded_channel();
    let server = http(move |req| {
        let tx = tx.clone();
        async move {
            tx.send(req).unwrap();
            empty_response()
        }
    });
    (server, rx)
}

pub fn empty_response() -> Response<Empty<Bytes>> {
    Default::default()
}
