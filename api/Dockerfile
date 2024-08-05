# Use the official Rust image as the base image
FROM rust:latest as builder

# Set the working directory inside the container
WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a minimal image for the final stage
FROM debian:bookworm-slim

# Install required runtime dependencies
RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN adduser --disabled-password --gecos '' appuser

# Create necessary directories and set ownership
RUN mkdir -p /usr/local/var/snap-shot-api \
    && chown -R appuser:appuser /usr/local/var/snap-shot-api

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/target/release/snap-shot-api /usr/local/bin/snap-shot-api

# Change ownership of the binary
RUN chown appuser:appuser /usr/local/bin/snap-shot-api

# Switch to the non-root user
USER appuser

# Set the command to run the binary
CMD ["snap-shot-api"]

# Expose the port that the Axum application listens on
EXPOSE 8080
