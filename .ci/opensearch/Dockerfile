ARG STACK_VERSION
FROM "opensearchproject/opensearch:$STACK_VERSION"

RUN /usr/share/opensearch/opensearch-onetime-setup.sh
COPY esnode.pem /usr/share/opensearch/config/esnode.pem
COPY esnode-key.pem /usr/share/opensearch/config/esnode-key.pem

RUN echo "appender.header_warning.type = HeaderWarningAppender" >> /usr/share/opensearch/config/log4j2.properties
RUN echo "appender.header_warning.name = header_warning" >> /usr/share/opensearch/config/log4j2.properties
RUN echo "logger.deprecation.name = org.opensearch.deprecation" >> /usr/share/opensearch/config/log4j2.properties
RUN echo "logger.deprecation.level = deprecation" >> /usr/share/opensearch/config/log4j2.properties
RUN echo "logger.deprecation.appenderRef.header_warning.ref = header_warning" >> /usr/share/opensearch/config/log4j2.properties
