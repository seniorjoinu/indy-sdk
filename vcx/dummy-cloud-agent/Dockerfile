FROM rust:1.31

COPY src src
COPY build.rs build.rs
COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml
COPY sample-config.json config.json
COPY lib/ /usr/lib/

EXPOSE 8080

CMD cargo run config.json