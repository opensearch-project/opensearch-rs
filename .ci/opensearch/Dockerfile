ARG SOURCE_IMAGE
FROM $SOURCE_IMAGE

WORKDIR /usr/share/opensearch

COPY --chown=opensearch:opensearch opensearch.yml ./config/

RUN if [[ -f "./opensearch-onetime-setup.sh" ]]; then ./opensearch-onetime-setup.sh ; fi

COPY --chown=opensearch:opensearch *.pem ./config/

RUN sed -i'' 's/plugins.security.audit.type:.*/plugins.security.audit.type: log4j/' ./config/opensearch.yml

RUN echo "appender.header_warning.type = HeaderWarningAppender" >> ./config/log4j2.properties
RUN echo "appender.header_warning.name = header_warning" >> ./config/log4j2.properties
RUN echo "logger.deprecation.name = org.opensearch.deprecation" >> ./config/log4j2.properties
RUN echo "logger.deprecation.level = deprecation" >> ./config/log4j2.properties
RUN echo "logger.deprecation.appenderRef.header_warning.ref = header_warning" >> ./config/log4j2.properties
