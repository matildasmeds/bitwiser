FROM rust:1.40-slim
COPY Cargo.toml .
COPY Cargo.lock .
COPY target target
CMD ./target/release/bitwiser
