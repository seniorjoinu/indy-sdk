FROM frolvlad/alpine-bash

COPY target/debug/indy-dummy-agent agent
COPY sample-config.json config.json

EXPOSE 8080

CMD ["./agent config.json"]