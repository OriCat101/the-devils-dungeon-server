FROM rustlang/rust:nightly as builder
WORKDIR /app

# Install SQLx CLI (for prepare if needed)
RUN cargo install sqlx-cli --no-default-features --features postgres

COPY . ./
WORKDIR /app/server

# Set DATABASE_URL for sqlx macros
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

RUN cargo build --release
