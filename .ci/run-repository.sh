#!/usr/bin/env bash
# parameters are available to this script

# TEST_SUITE -- which test suite to run: free or platinum
# OPENSEARCH_URL -- The url at which opensearch is reachable
# RUST_TOOLCHAIN -- Rust toolchain version to compile and run tests
script_path=$(dirname $(realpath -s $0))
source $script_path/functions/imports.sh
set -euo pipefail

RUST_TOOLCHAIN=latest
OPENSEARCH_URL=${OPENSEARCH_URL-"$opensearch_url"}
opensearch_container=${opensearch_container-}

echo -e "\033[34;1mINFO:\033[0m TEST_SUITE ${TEST_SUITE}\033[0m"
echo -e "\033[34;1mINFO:\033[0m URL ${OPENSEARCH_URL}\033[0m"
echo -e "\033[34;1mINFO:\033[0m CONTAINER ${opensearch_container}\033[0m"
echo -e "\033[34;1mINFO:\033[0m RUST_TOOLCHAIN ${RUST_TOOLCHAIN}\033[0m"

echo -e "\033[1m>>>>> Build [opensearch-project/opensearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

docker pull rust:"${RUST_TOOLCHAIN}"

docker build --build-arg RUST_TOOLCHAIN="${RUST_TOOLCHAIN}" --file .ci/DockerFile.Repository --tag opensearch-project/opensearch-rs .

echo -e "\033[1m>>>>> Run [opensearch-project/opensearch-rs container] >>>>>>>>>>>>>>>>>>>>>>>>>>>>>\033[0m"

repo=$(realpath $(dirname $(realpath -s $0))/../)

docker run \
  --network=${network_name} \
  --env "TEST_SUITE=${TEST_SUITE}" \
  --env "OPENSEARCH_URL=${OPENSEARCH_URL}" \
  --env "CI=true" \
  --name test-runner \
  --volume ${repo}/test_results:/usr/src/opensearch-rs/test_results \
  --rm \
  opensearch-project/opensearch-rs \
  /bin/bash -c "cargo make test-yaml"