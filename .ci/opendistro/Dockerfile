FROM amazon/opendistro-for-elasticsearch:1.13.2
ARG SECURE_INTEGRATION
ARG es_path=/usr/share/elasticsearch

RUN if [ "$SECURE_INTEGRATION" != "true" ] ; then $es_path/bin/elasticsearch-plugin remove opendistro_security; fi