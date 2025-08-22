FROM lukemathwalker/cargo-chef:latest-rust-1.86-slim-bookworm AS chef
RUN apt update && apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  ca-certificates \
  curl

WORKDIR /app
ENV SQLX_OFFLINE=true

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies.
RUN cargo chef cook --release --recipe-path recipe.json
# Build application.
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
 && apt-get install -y --no-install-recommends ca-certificates curl libssl3 \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/hypurr-exporter /usr/local/bin/hypurr-exporter

EXPOSE 3000
CMD ["/usr/local/bin/hypurr-exporter"]
