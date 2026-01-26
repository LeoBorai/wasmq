FROM alpine:3

COPY ./tmp/mate /opt/mate

COPY ./tmp/local.toml /opt/local.toml

ENTRYPOINT ["/opt/mate", "hub", "start", "--config", "./local.toml"]
