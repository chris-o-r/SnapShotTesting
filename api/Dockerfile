# Stage 1: Build the Rust Axum API
FROM rust:latest AS builder

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Stage 2: Create a lightweight image with the compiled binary
FROM debian:bookworm-slim

# Install necessary libraries
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/snap-shot-api .

# Expose the API port
EXPOSE 8080

# Command to run the application
CMD ["./snap-shot-api"]
