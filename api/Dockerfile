FROM rust:1.93

WORKDIR /app

ENV SQLX_OFFLINE=true

RUN cargo install sqlx-cli --features postgres,rustls

COPY api/.sqlx /app/.sqlx
COPY api/Cargo.toml api/Cargo.lock /app/.
COPY api/src ./src
RUN cargo build --release


EXPOSE 3000

CMD ["./target/release/rust-app"]
