# Build stage
FROM rust:1.70-slim AS builder

# Install required system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Final stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/zedi-gen /usr/local/bin/zedi-gen

# Set the working directory
WORKDIR /data

# Set the entrypoint
ENTRYPOINT ["zedi-gen"]

# Default command
CMD ["--help"]
