# syntax=docker/dockerfile:1

# Build stage
FROM --platform=$BUILDPLATFORM rust:1.83-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Accept platform arguments for cross-compilation
ARG TARGETPLATFORM
ARG BUILDPLATFORM
ARG TARGETOS
ARG TARGETARCH

# Install cross-compilation tools if needed
RUN case "$TARGETARCH" in \
    "arm64") \
        if [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
            apt-get update && \
            apt-get install -y gcc-aarch64-linux-gnu && \
            rm -rf /var/lib/apt/lists/*; \
        fi \
        ;; \
    "amd64") \
        if [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
            apt-get update && \
            apt-get install -y gcc-x86-64-linux-gnu && \
            rm -rf /var/lib/apt/lists/*; \
        fi \
        ;; \
    esac

# Set the Rust target based on the platform
RUN case "$TARGETARCH" in \
    "amd64") echo "x86_64-unknown-linux-gnu" > /rust_target.txt ;; \
    "arm64") echo "aarch64-unknown-linux-gnu" > /rust_target.txt ;; \
    *) echo "unknown architecture: $TARGETARCH" && exit 1 ;; \
    esac && \
    rustup target add $(cat /rust_target.txt)

# Configure cargo for cross-compilation
RUN case "$TARGETARCH" in \
    "arm64") \
        if [ "$BUILDPLATFORM" != "$TARGETPLATFORM" ]; then \
            mkdir -p ~/.cargo && \
            echo '[target.aarch64-unknown-linux-gnu]' >> ~/.cargo/config.toml && \
            echo 'linker = "aarch64-linux-gnu-gcc"' >> ~/.cargo/config.toml; \
        fi \
        ;; \
    esac

# Copy manifests first for better layer caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main to cache dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target $(cat /rust_target.txt) && \
    rm -rf src

# Copy actual source code
COPY src ./src

# Build the application for the target platform
# Touch main.rs to force rebuild of our code (not dependencies)
RUN touch src/main.rs && \
    cargo build --release --target $(cat /rust_target.txt) && \
    cp target/$(cat /rust_target.txt)/release/bb /bb

# Runtime stage
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /bb /usr/local/bin/bb

# Create directory for database and config
RUN mkdir -p /data /config

# Set environment variable for database location
ENV BB_DATA_DIR=/data

ENTRYPOINT ["bb"]
CMD ["--help"]
