# Use an official Rust runtime as a parent image
FROM rust:1.73-bullseye as builder

# Set the working directory in the image to /app
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y cmake libpq-dev && rm -rf /var/lib/apt/lists/*

# Copy over your manifest and source code
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src

# Build for debug.
RUN cargo build

# Install Diesel CLI with no features to prevent it from installing unnecessary dependencies
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy migrations
COPY ./migrations ./migrations

# Our second stage, that creates the final executable
FROM debian:bullseye-slim

# Install OpenSSL and libpq
RUN apt-get update && apt-get install -y openssl libpq5 && rm -rf /var/lib/apt/lists/*

# Copy the build artifact from the builder stage and create a new binary.
COPY --from=builder /app/target/debug/three-tier-app /usr/local/bin

# Copy the Diesel CLI binary from the builder stage
COPY --from=builder /usr/local/cargo/bin/diesel /usr/local/bin/diesel

# Copy the migrations from the builder stage
COPY --from=builder /app/migrations /app/migrations

# Copy the static directory from the builder stage
COPY --from=builder /app/src/static /app/src/static

# Set the startup command to run your binary
CMD ["three-tier-app"]