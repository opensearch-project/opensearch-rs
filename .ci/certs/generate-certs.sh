#!/usr/bin/env bash

set -eo pipefail

openssl req -new -x509 \
    -key root-ca.key \
    -subj "/DC=com/DC=example/O=Example Com Inc./OU=Example Com Inc. Root CA/CN=Example Com Inc. Root CA" \
    -config ./openssl.conf -extensions root-ca \
    -days 36500 \
    -out root-ca.crt

openssl req -new \
    -config ./openssl.conf \
    -key esnode.key \
    -subj "/DC=de/L=test/O=node/OU=node/CN=node-0.example.com" \
    -out esnode.csr

openssl x509 -req \
    -in esnode.csr \
    -extfile ./openssl.conf -extensions esnode \
    -days 36500 \
    -CA root-ca.crt -CAkey root-ca.key -CAcreateserial \
    -out esnode.crt

openssl x509 -req \
    -in esnode.csr \
    -extfile ./openssl.conf -extensions esnode-no-san \
    -days 36500 \
    -CA root-ca.crt -CAkey root-ca.key -CAcreateserial \
    -out esnode-no-san.crt

openssl req -new \
    -config ./openssl.conf \
    -subj "/C=de/L=test/O=client/OU=client/CN=kirk" \
    -key kirk.key \
    -out kirk.csr

openssl x509 -req \
    -in kirk.csr \
    -extfile ./openssl.conf -extensions kirk \
    -days 36500 \
    -CA root-ca.crt -CAkey root-ca.key -CAcreateserial \
    -out kirk.crt

openssl pkcs12 -export \
    -in kirk.crt \
    -inkey kirk.key \
    -descert \
    -passout pass:kirk \
    -out kirk.p12

cp root-ca.crt ../opensearch/root-ca.pem
cp esnode.crt ../opensearch/esnode.pem
rm *.srl *.csr