FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin shalombot4

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
RUN apt-get update
RUN apt-get install -y openssl ca-certificates
WORKDIR /app
COPY --from=builder /app/target/release/shalombot4 /usr/local/bin
COPY config/default.yaml config/default.yaml
ENTRYPOINT ["/usr/local/bin/shalombot4"]