# =============================================================================
# BUILDER STAGE - Build the Rust application
# =============================================================================
FROM rust:latest AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    protobuf-compiler \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Copy source code and proto files
COPY src ./src
COPY proto ./proto
COPY build.rs .

# Build the application (this will be fast since dependencies are cached)
RUN cargo build --release

# =============================================================================
# RUNTIME STAGE - Final lightweight image
# =============================================================================
FROM debian:unstable-slim AS runtime

# Install only runtime dependencies
RUN apt-get update && apt-get install -y \
    gnupg \
    docker.io \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user and add to docker group
RUN useradd -r -s /bin/false isobox && \
    groupadd -g 997 docker || true && \
    usermod -aG docker isobox

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/isobox .

# Change ownership
RUN chown isobox:isobox /app/isobox

# Expose ports
EXPOSE 8000 9000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run the binary
CMD ["./isobox"]
