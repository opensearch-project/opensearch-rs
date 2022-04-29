ARG STACK_VERSION
FROM "opensearchproject/opensearch:$STACK_VERSION"
ARG SECURE_INTEGRATION
ARG opensearch_path=/usr/share/opensearch
ARG opensearch_yml=$opensearch_path/config/opensearch.yml
RUN if [ "$SECURE_INTEGRATION" != "true" ] ; then $opensearch_path/bin/opensearch-plugin remove opensearch-security; fi
