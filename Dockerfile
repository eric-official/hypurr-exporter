FROM lukemathwalker/cargo-chef:latest-rust-1.86-slim-bookworm AS chef
RUN apt update && apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  ca-certificates

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
RUN apt update && apt install -y \
  build-essential \
  pkg-config \
  libssl-dev \
  ca-certificates

WORKDIR /app
COPY --from=builder /app/target/release/hypurr-exporter /usr/local/bin

CMD ["/usr/local/bin/hypurr-exporter"]
