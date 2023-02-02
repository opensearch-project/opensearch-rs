ARG STACK_VERSION
FROM "opensearchproject/opensearch:$STACK_VERSION"

USER root
COPY root-ca.pem /usr/share/opensearch/config/root-ca.pem
COPY esnode.pem /usr/share/opensearch/config/esnode.pem
COPY esnode-key.pem /usr/share/opensearch/config/esnode-key.pem
RUN chmod a+wr /usr/share/opensearch/config/*pem

USER opensearch
