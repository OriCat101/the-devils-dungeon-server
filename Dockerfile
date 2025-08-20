FROM rustlang/rust:nightly as builder
WORKDIR /app

# Copy source code
COPY . ./
WORKDIR /app/server

# Build the project (release for smaller image)
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app

# Install required system dependencies
RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the built binaries and migrations from builder
COPY --from=builder /app/server/target/release/level_server ./level_server
COPY --from=builder /app/server/target/release/setup ./setup
COPY --from=builder /app/server/migrations ./migrations
COPY --from=builder /app/server/Cargo.toml ./Cargo.toml
COPY --from=builder /app/server/Cargo.lock ./Cargo.lock

EXPOSE 8080

# Entrypoint script to run migrations and start the server
COPY docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]