# ---- Build stage ----
FROM rustlang/rust:nightly AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy Cargo files first (for caching)
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY .sqlx ./.sqlx
COPY migrations ./migrations

# Enable SQLx offline mode
ENV SQLX_OFFLINE true

# Build the binary
RUN cargo build --release --bin level_server --bin setup

# ---- Runtime stage ----
FROM debian:bookworm-slim

# Install runtime deps
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binarys
COPY --from=builder /app/target/release/level_server .
COPY --from=builder /app/target/release/setup .

# Copy any migrations (needed by sqlx migrate)
COPY migrations ./migrations
COPY --from=builder /app/.sqlx ./.sqlx

# Expose server port
EXPOSE 8080

# Run the server
CMD ["./level_server"]
