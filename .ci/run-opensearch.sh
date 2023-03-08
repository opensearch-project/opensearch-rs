#!/usr/bin/env bash
#
# Launch one or more OpenSearch nodes via the Docker image,
# to form a cluster suitable for running the REST API tests.
#
# Version 1.1.0
# - Initial version of the run-opensearch.sh script
# - Deleting the volume should not dependent on the container still running
# - Fixed `ES_JAVA_OPTS` config

script_path=$(dirname $(realpath -s $0))
source $script_path/functions/imports.sh
set -euo pipefail

echo -e "\033[34;1mINFO:\033[0m Take down node if called twice with the same arguments (DETACH=true) or on seperate terminals \033[0m"
cleanup_node $opensearch_node_name

manager_node_name=${opensearch_node_name}
cluster_name=search-rest-test

environment=""

if [[ "$SECURE_INTEGRATION" != "true" ]]; then
  environment+=($(cat <<-END
    --env DISABLE_SECURITY_PLUGIN=true
END
))
fi

NUMBER_OF_NODES=${NUMBER_OF_NODES-1}
http_port=9200
for (( i=0; i<$NUMBER_OF_NODES; i++, http_port++ )); do
  node_name=${opensearch_node_name}$i
  node_url=${external_opensearch_url/9200/${http_port}}
  if [[ "$i" == "0" ]]; then node_name=$opensearch_node_name; fi
  echo "$i: $http_port $node_url "

  # make sure we detach for all but the last node if DETACH=false (default) so all nodes are started
  local_detach="true"
  if [[ "$i" == "$((NUMBER_OF_NODES-1))" ]]; then local_detach=$DETACH; fi

  if [[ "$STACK_VERSION" != *"SNAPSHOT" ]]; then
    SOURCE_IMAGE="opensearchproject/opensearch:${STACK_VERSION}"
  else
    SOURCE_IMAGE="opensearch:test"
  fi

  CLUSTER_TAG=opensearch-secure-$SECURE_INTEGRATION
  echo -e "\033[34;1mINFO: building opensearch container\033[0m"

  docker build \
    --build-arg SOURCE_IMAGE=$SOURCE_IMAGE \
    --tag=$CLUSTER_TAG \
    .ci/opensearch/

  echo -e "\033[34;1mINFO:\033[0m Starting container $node_name \033[0m"
  set -x
  if [[ "$SECURE_INTEGRATION" == "true" ]]; then
    healthcmd="curl -vvv -s --cacert /usr/share/opensearch/config/root-ca.pem -u admin:admin --fail https://localhost:9200/_cluster/health || exit 1"
  else
    healthcmd="curl -vvv -s --fail http://localhost:9200/_cluster/health || exit 1"
  fi

  docker run \
    --name "$node_name" \
    --network "$network_name" \
    --env "OPENSEARCH_JAVA_OPTS=-Xms1g -Xmx1g" \
    ${environment[@]} \
    --env "node.name=$node_name" \
    --volume "${node_name}-rest-test-data":"/usr/share/opensearch/data${i}" \
    --publish "$http_port":9200 \
    --ulimit nofile=65536:65536 \
    --ulimit memlock=-1:-1 \
    --detach="$local_detach" \
    --health-cmd="$(echo $healthcmd)" \
    --health-interval=2s \
    --health-retries=20 \
    --health-timeout=2s \
    --rm \
    -d \
    $CLUSTER_TAG;

  set +x
  if wait_for_container "$opensearch_node_name" "$network_name"; then
    echo -e "\033[32;1mSUCCESS:\033[0m Running on: $node_url\033[0m"
  fi

done

