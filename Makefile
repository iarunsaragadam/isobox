.PHONY: help build run test clean docker-build docker-run demo

# Default target
help:
	@echo "🔒 isobox - Secure Code Execution API"
	@echo ""
	@echo "Available commands:"
	@echo "  build        Build the project"
	@echo "  run          Run the server locally"
	@echo "  test         Run unit tests"
	@echo "  demo         Run example demonstrations"
	@echo "  clean        Clean build artifacts"
	@echo "  docker-build Build Docker image"
	@echo "  docker-run   Run with Docker Compose"
	@echo "  docker-pull  Pull required language images"

# Build the project
build:
	cargo build

# Build release version
build-release:
	cargo build --release

# Run the server locally
run:
	RUST_LOG=info cargo run

# Run in development mode with auto-reload (requires cargo-watch)
dev:
	cargo watch -x run

# Run unit tests
test:
	cargo test

# Run tests with verbose output
test-verbose:
	cargo test -- --nocapture

# Clean build artifacts
clean:
	cargo clean

# Run demo examples
demo:
	./examples/demo.sh

# Build Docker image
docker-build:
	docker build -t isobox .

# Run with Docker Compose
docker-run:
	docker-compose up --build

# Pull required language images
docker-pull:
	docker pull python:3.11
	docker pull node:20

# Check code formatting
fmt:
	cargo fmt

# Run clippy linter
lint:
	cargo clippy

# Run all checks (test, fmt, lint)
check-all: test fmt lint

# Quick manual test of the API
quick-test:
	@echo "Testing Python execution..."
	@curl -s -X POST http://localhost:8000/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "python", "code": "print(\"Hello from isobox!\")"}' \
		| jq
	@echo ""
	@echo "Testing Node.js execution..."
	@curl -s -X POST http://localhost:8000/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "node", "code": "console.log(\"Hello from Node.js!\");"}' \
		| jq

# Install development dependencies
install-deps:
	cargo install cargo-watch
	@echo "Consider installing jq for better JSON output: brew install jq"
