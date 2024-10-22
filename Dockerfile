FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app
# Install nightly toolchain
RUN rustup toolchain install nightly

FROM chef AS planner
COPY . .
RUN cargo +nightly chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo +nightly chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo +nightly build --release --bin libri_backend

# We do not need the Rust toolchain to run the binary!
FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/libri_backend /usr/local/bin
ENTRYPOINT ["/usr/local/bin/libri_backend"]
