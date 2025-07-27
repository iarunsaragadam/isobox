# Build stage
FROM rust:latest AS builder

# Install dependencies for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build the application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install Docker CLI and runtime dependencies
RUN apt-get update && apt-get install -y \
    gnupg \
    docker.io \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd -r -s /bin/false isobox

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/isobox .

# Change ownership
RUN chown isobox:isobox /app/isobox

# Switch to non-root user
USER isobox

# Expose port
EXPOSE 8000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8000/health || exit 1

# Run the binary
CMD ["./isobox"]
