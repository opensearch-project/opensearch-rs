FROM opensearchproject/opensearch:latest
ARG SECURE_INTEGRATION
RUN if [ "$SECURE_INTEGRATION" != "true" ] ; then /usr/share/opensearch/bin/opensearch-plugin remove opensearch-security; fi
COPY --chown=opensearch:opensearch .ci/opensearch/opensearch.yml /usr/share/opensearch/config/