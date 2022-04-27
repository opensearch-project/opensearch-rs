ARG STACK_VERSION
FROM "amazon/opendistro-for-elasticsearch:$STACK_VERSION"
ARG SECURE_INTEGRATION
ARG es_path=/usr/share/elasticsearch

RUN if [ "$SECURE_INTEGRATION" != "true" ] ; then $es_path/bin/elasticsearch-plugin remove opendistro_security; fi