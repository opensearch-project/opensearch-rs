#!/usr/bin/env bash

set -eo pipefail

openssl genrsa -out root-ca.key

openssl genrsa -out esnode.key
openssl pkcs8 -topk8 -in esnode.key -nocrypt -out ../opensearch/esnode-key.pem

openssl genrsa -out kirk.key