FROM rust:1.86.0 AS builder
WORKDIR /app

COPY Cargo.toml .
COPY src/ src/

RUN cargo build --release

###########################

FROM debian:bookworm-slim
WORKDIR /app

COPY --from=builder /app/target/release/server ./target/release/server

RUN apt-get update && apt-get install -y \
  curl \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

ENV TZ="Asia/Tokyo"

ENTRYPOINT ["./target/release/server"]
