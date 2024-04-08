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
//! These tests require a cluster configured with Security. One can be spun up using the
//! .ci/run-opensearch.sh script as follows:
//!
//!
//! DETACH=true .ci/run-opensearch.sh
#![cfg(any(feature = "native-tls", feature = "rustls-tls"))]
#![allow(unused)]

pub mod common;
use common::*;

use anyhow::anyhow;
use opensearch::{
    cert::{Certificate, CertificateValidation},
    http::response::Response,
    Error,
};

static CA_CERT: &[u8] = include_bytes!("../../.ci/certs/root-ca.crt");
static CA_CHAIN_CERT: &[u8] = include_bytes!("../../.ci/certs/ca-chain.crt");
static ESNODE_CERT: &[u8] = include_bytes!("../../.ci/certs/esnode.crt");
static ESNODE_NO_SAN_CERT: &[u8] = include_bytes!("../../.ci/certs/esnode-no-san.crt");

fn assert_is_untrusted_cert_error(res: Result<Response, Error>) {
    let e = res.expect_err("Result should be an untrusted cert error");

    let untrusted_cert_error = if cfg!(windows) {
        "terminated in a root certificate which is not trusted by the trust provider"
    } else if cfg!(target_os = "macos") {
        "The certificate was not trusted"
    } else {
        "unable to get local issuer certificate"
    };

    let mut source: Option<&dyn std::error::Error> = Some(&e);
    while let Some(err) = source {
        if err.to_string().contains(untrusted_cert_error) {
            return;
        }

        source = err.source();
    }

    panic!(
        "Expected error message to contain '{}' but was {:#?}",
        untrusted_cert_error, e
    )
}

/// Default certificate validation with a self signed certificate
#[tokio::test]
#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
async fn default_certificate_validation() {
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Default));
    let res = client.ping().send().await;
    assert_is_untrusted_cert_error(res);
}

/// Allows any certificate through
#[tokio::test]
async fn none_certificate_validation() -> anyhow::Result<()> {
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::None));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server contains the one given to the client
/// within the authority chain, and hostname matches
#[tokio::test]
#[cfg(all(
    any(feature = "native-tls", feature = "rustls-tls"),
    not(target_os = "macos")
))]
async fn full_certificate_ca_validation() -> anyhow::Result<()> {
    let cert = Certificate::from_pem(CA_CERT)?;
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Full(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Try to load a certificate chain.
#[tokio::test]
#[cfg(all(
    any(feature = "native-tls", feature = "rustls-tls"),
    not(target_os = "macos")
))]
async fn full_certificate_ca_chain_validation() -> anyhow::Result<()> {
    let mut cert = Certificate::from_pem(CA_CHAIN_CERT)?;
    cert.append(Certificate::from_pem(CA_CERT)?);
    assert_eq!(cert.len(), 3, "expected three certificates in CA chain");
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Full(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client and hostname matches
#[tokio::test]
#[cfg(all(windows, feature = "native-tls"))]
async fn full_certificate_validation() -> anyhow::Result<()> {
    let cert = Certificate::from_pem(ESNODE_CERT)?;
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Full(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client and hostname matches, using rustls-tls
#[tokio::test]
#[cfg(all(feature = "rustls-tls", not(target_os = "macos")))]
async fn full_certificate_validation_rustls_tls() -> anyhow::Result<()> {
    let mut chain: Vec<u8> = Vec::with_capacity(ESNODE_CERT.len() + CA_CERT.len());
    chain.extend(CA_CERT);
    chain.extend(ESNODE_CERT);

    let cert = Certificate::from_pem(chain.as_slice())?;
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Full(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(all(unix, feature = "native-tls"))]
async fn full_certificate_validation() {
    let cert = Certificate::from_pem(ESNODE_CERT).unwrap();
    let client = client::create_with(|b| b.cert_validation(CertificateValidation::Full(cert)));
    let res = client.ping().send().await;
    assert_is_untrusted_cert_error(res);
}

/// Certificate provided by the server is the one given to the client
#[tokio::test]
#[cfg(all(windows, feature = "native-tls"))]
async fn certificate_certificate_validation() -> anyhow::Result<()> {
    let cert = Certificate::from_pem(ESNODE_CERT)?;
    let client =
        client::create_with(|b| b.cert_validation(CertificateValidation::Certificate(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server is the one given to the client. This fails on Linux because
/// it appears that it also needs the CA for the cert
#[tokio::test]
#[cfg(all(unix, feature = "native-tls"))]
async fn certificate_certificate_validation() {
    let cert = Certificate::from_pem(ESNODE_CERT).unwrap();
    let client =
        client::create_with(|b| b.cert_validation(CertificateValidation::Certificate(cert)));
    let res = client.ping().send().await;
    assert_is_untrusted_cert_error(res);
}

/// Certificate provided by the server contains the one given to the client
/// within the authority chain
#[tokio::test]
#[cfg(all(feature = "native-tls", not(target_os = "macos")))]
async fn certificate_certificate_ca_validation() -> anyhow::Result<()> {
    let cert = Certificate::from_pem(CA_CERT)?;
    let client =
        client::create_with(|b| b.cert_validation(CertificateValidation::Certificate(cert)));
    let _response = client.ping().send().await?;
    Ok(())
}

/// Certificate provided by the server does not match the one given to the client
#[tokio::test]
#[cfg(feature = "native-tls")]
async fn fail_certificate_certificate_validation() {
    let cert = Certificate::from_pem(ESNODE_NO_SAN_CERT).unwrap();
    let client =
        client::create_with(|b| b.cert_validation(CertificateValidation::Certificate(cert)));
    let res = client.ping().send().await;
    assert_is_untrusted_cert_error(res);
}
