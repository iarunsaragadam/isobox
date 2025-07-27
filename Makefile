# isobox Makefile
# Comprehensive testing and build pipeline

.PHONY: help test test-unit test-integration test-e2e build clean docker-build docker-test docker-push all

# Default target
help:
	@echo "isobox - Secure Code Execution API"
	@echo ""
	@echo "Available targets:"
	@echo "  help          - Show this help message"
	@echo "  test          - Run all tests (unit + integration + e2e)"
	@echo "  test-unit     - Run unit tests only"
	@echo "  test-integration - Run integration tests"
	@echo "  test-e2e      - Run end-to-end tests against Docker image"
	@echo "  build         - Build the Rust application"
	@echo "  clean         - Clean build artifacts"
	@echo "  docker-build  - Build Docker image"
	@echo "  docker-test   - Run e2e tests against Docker image"
	@echo "  docker-push   - Push Docker image to registry"
	@echo "  all           - Run full pipeline: test -> build -> docker-build -> docker-test"

# Variables
IMAGE_NAME = isobox
IMAGE_TAG = latest
TEST_TIMEOUT = 30s
API_BASE_URL = http://localhost:8000

# Run all tests
test: test-unit test-integration test-e2e

# Unit tests
test-unit:
	@echo "🧪 Running unit tests..."
	cargo test
	@echo "✅ Unit tests completed"

# Integration tests (if any)
test-integration:
	@echo "🔗 Running integration tests..."
	@echo "✅ Integration tests completed (none defined yet)"

# End-to-end tests against local development server
test-e2e: build
	@echo "🚀 Running end-to-end tests..."
	@echo "Starting isobox server for testing..."
	@timeout $(TEST_TIMEOUT) cargo run &
	@sleep 3
	@echo "Testing basic functionality..."
	@curl -s -X POST $(API_BASE_URL)/health > /dev/null || (echo "❌ Health check failed"; exit 1)
	@echo "✅ Health check passed"
	@echo "Testing Python execution..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "python", "code": "print(\"Hello from Python!\")"}' | grep -q "Hello from Python!" || (echo "❌ Python test failed"; exit 1)
	@echo "✅ Python test passed"
	@echo "Testing Node.js execution..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "node", "code": "console.log(\"Hello from Node.js!\")"}' | grep -q "Hello from Node.js!" || (echo "❌ Node.js test failed"; exit 1)
	@echo "✅ Node.js test passed"
	@echo "Testing Go execution..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "go", "code": "package main\nimport \"fmt\"\nfunc main() { fmt.Println(\"Hello from Go!\") }"}' | grep -q "Hello from Go!" || (echo "❌ Go test failed"; exit 1)
	@echo "✅ Go test passed"
	@pkill -f "cargo run" || true
	@echo "✅ End-to-end tests completed"

# Build the application
build:
	@echo "🔨 Building isobox..."
	cargo build --release
	@echo "✅ Build completed"

# Clean build artifacts
clean:
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	@echo "✅ Clean completed"

# Build Docker image
docker-build:
	@echo "🐳 Building Docker image..."
	docker build -t $(IMAGE_NAME):$(IMAGE_TAG) .
	@echo "✅ Docker image built"

# Run e2e tests against Docker image
docker-test: docker-build
	@echo "🧪 Running e2e tests against Docker image..."
	@echo "Starting isobox container..."
	@docker run -d --name isobox-test \
		-p 8000:8000 \
		-v /var/run/docker.sock:/var/run/docker.sock \
		-v /tmp:/tmp \
		$(IMAGE_NAME):$(IMAGE_TAG)
	@sleep 5
	@echo "Testing container health..."
	@curl -s -X POST $(API_BASE_URL)/health > /dev/null || (echo "❌ Container health check failed"; docker logs isobox-test; docker stop isobox-test; docker rm isobox-test; exit 1)
	@echo "✅ Container health check passed"
	@echo "Running comprehensive language tests..."
	@$(MAKE) test-languages
	@echo "Stopping test container..."
	@docker stop isobox-test
	@docker rm isobox-test
	@echo "✅ Docker e2e tests completed"

# Test all supported languages
test-languages:
	@echo "Testing Python..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/python_math.json | grep -q "Python Math Test" || (echo "❌ Python test failed"; exit 1)
	@echo "✅ Python test passed"
	
	@echo "Testing Node.js..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/node_modern.json | grep -q "Node.js Modern Features Test" || (echo "❌ Node.js test failed"; exit 1)
	@echo "✅ Node.js test passed"
	
	@echo "Testing Go..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/go_basic.json | grep -q "Go Program Test" || (echo "❌ Go test failed"; exit 1)
	@echo "✅ Go test passed"
	
	@echo "Testing Rust..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/rust_basic.json | grep -q "Rust Program Test" || (echo "❌ Rust test failed"; exit 1)
	@echo "✅ Rust test passed"
	
	@echo "Testing Java..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/java_basic.json | grep -q "Java Program Test" || (echo "❌ Java test failed"; exit 1)
	@echo "✅ Java test passed"
	
	@echo "Testing C++..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/cpp_basic.json | grep -q "C++ Program Test" || (echo "❌ C++ test failed"; exit 1)
	@echo "✅ C++ test passed"
	
	@echo "Testing C..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/c_basic.json | grep -q "C Program Test" || (echo "❌ C test failed"; exit 1)
	@echo "✅ C test passed"
	
	@echo "Testing PHP..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "php", "code": "<?php echo \"Hello from PHP!\\n\"; ?>"}' | grep -q "Hello from PHP" || (echo "❌ PHP test failed"; exit 1)
	@echo "✅ PHP test passed"
	
	@echo "Testing Ruby..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "ruby", "code": "puts \"Hello from Ruby!\""}' | grep -q "Hello from Ruby" || (echo "❌ Ruby test failed"; exit 1)
	@echo "✅ Ruby test passed"
	
	@echo "Testing Bash..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "bash", "code": "echo \"Hello from Bash!\""}' | grep -q "Hello from Bash" || (echo "❌ Bash test failed"; exit 1)
	@echo "✅ Bash test passed"
	
	@echo "Testing Haskell..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "haskell", "code": "main = putStrLn \"Hello from Haskell!\""}' | grep -q "Hello from Haskell" || (echo "❌ Haskell test failed"; exit 1)
	@echo "✅ Haskell test passed"
	
	@echo "Testing error handling..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "python", "code": "print(\"This will work\")\nraise Exception(\"This is an error\")"}' | grep -q "This is an error" || (echo "❌ Error handling test failed"; exit 1)
	@echo "✅ Error handling test passed"
	
	@echo "Testing unsupported language..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d '{"language": "unsupported", "code": "test"}' | grep -q "Unsupported language" || (echo "❌ Unsupported language test failed"; exit 1)
	@echo "✅ Unsupported language test passed"
	
	@echo "Testing timeout functionality..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/timeout_test.json | grep -q "Execution timed out" || (echo "❌ Timeout test failed"; exit 1)
	@echo "✅ Timeout test passed"
	
	@echo "Testing memory limits..."
	@curl -s -X POST $(API_BASE_URL)/execute \
		-H "Content-Type: application/json" \
		-d @examples/memory_test.json | grep -q "MemoryError\|Killed" || (echo "❌ Memory limit test failed"; exit 1)
	@echo "✅ Memory limit test passed"
	
	@echo "🎉 All language tests passed!"

# Push Docker image (for CI/CD)
docker-push:
	@echo "📤 Pushing Docker image..."
	@echo "This would push to the configured registry"
	@echo "✅ Push completed (simulated)"

# Full pipeline
all: test docker-build docker-test
	@echo "🎉 Full pipeline completed successfully!"

# Development helpers
dev:
	@echo "🚀 Starting development server..."
	cargo run

dev-test:
	@echo "🧪 Starting development server for testing..."
	@timeout 60s cargo run &
	@sleep 3
	@echo "Server started. Run tests manually or use Ctrl+C to stop"
	@wait

# Quick test for development
quick-test:
	@echo "⚡ Quick test..."
	cargo test --lib
	@echo "✅ Quick test completed"
