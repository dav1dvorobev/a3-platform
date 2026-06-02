FROM rust:1-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY client ./client

RUN cargo build --release -p client

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/client /usr/local/bin/client
COPY --from=builder /app/client/static /app/client/static

CMD ["client"]
