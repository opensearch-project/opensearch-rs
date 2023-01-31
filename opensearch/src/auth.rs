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

//! Authentication components

/// Credentials for authentication
#[derive(Debug, Clone)]
pub enum Credentials {
    /// A username and password to use for Basic authentication
    Basic(String, String),
    /// An access_token to use for Bearer authentication
    Bearer(String),
    /// A client certificate to use for PKI (Client Certificate) authentication.
    /// # Optional
    ///
    /// This requires the `native-tls` or `rustls-tls` feature to be enabled.
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    Certificate(ClientCertificate),
    /// An id and api_key to use for API key authentication
    ApiKey(String, String),
    /// AWS credentials used for AWS SigV4 request signing.
    ///
    /// # Optional
    ///
    /// This requires the `aws-auth` feature to be enabled.
    #[cfg(feature = "aws-auth")]
    AwsSigV4(
        aws_credential_types::provider::SharedCredentialsProvider,
        aws_types::region::Region,
    ),
}

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
#[derive(Debug, Clone)]
pub enum ClientCertificate {
    /// Bytes of a DER-formatted PKCS#12 archive and optional passphrase.
    ///
    /// The archive should contain a leaf certificate and its private key,
    /// as well any intermediate certificates that allow clients to build a chain to
    /// a trusted root. The chain certificates
    /// should be in order from the leaf certificate towards the root.
    ///
    /// # Optional
    ///
    /// This requires the `native-tls` feature to be enabled.
    #[cfg(feature = "native-tls")]
    Pkcs12(Vec<u8>, Option<String>),

    /// Bytes of a PEM encoded private key and
    /// at least one PEM encoded certificate.
    ///
    /// # Optional
    ///
    /// This requires the `rustls-tls` feature to be enabled.
    #[cfg(feature = "rustls-tls")]
    Pem(Vec<u8>),
}

#[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
impl From<ClientCertificate> for Credentials {
    fn from(cert: ClientCertificate) -> Self {
        Credentials::Certificate(cert)
    }
}

#[cfg(any(feature = "aws-auth"))]
impl std::convert::TryFrom<&aws_types::SdkConfig> for Credentials {
    type Error = super::Error;

    fn try_from(value: &aws_types::SdkConfig) -> Result<Self, Self::Error> {
        let credentials = value
            .credentials_provider()
            .ok_or_else(|| super::error::lib("SdkConfig does not have a credentials_provider"))?
            .clone();
        let region = value
            .region()
            .ok_or_else(|| super::error::lib("SdkConfig does not have a region"))?
            .clone();
        Ok(Credentials::AwsSigV4(credentials, region))
    }
}

#[cfg(any(feature = "aws-auth"))]
impl std::convert::TryFrom<aws_types::SdkConfig> for Credentials {
    type Error = super::Error;

    fn try_from(value: aws_types::SdkConfig) -> Result<Self, Self::Error> {
        Credentials::try_from(&value)
    }
}
