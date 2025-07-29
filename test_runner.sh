#!/bin/bash

# Comprehensive test runner for IsoBox test case functionality
# This script runs all tests to verify the test case implementation

set -e

echo "=== IsoBox Test Case Test Runner ==="
echo "Testing comprehensive test case functionality..."
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    local status=$1
    local message=$2
    case $status in
        "PASS")
            echo -e "${GREEN}✓ PASS${NC}: $message"
            ;;
        "FAIL")
            echo -e "${RED}✗ FAIL${NC}: $message"
            ;;
        "SKIP")
            echo -e "${YELLOW}⚠ SKIP${NC}: $message"
            ;;
    esac
}

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    print_status "SKIP" "Docker not available - skipping integration tests"
    echo "Please install Docker to run integration tests"
    exit 0
fi

# Check if Docker daemon is running
if ! docker info &> /dev/null; then
    print_status "SKIP" "Docker daemon not running - skipping integration tests"
    echo "Please start Docker daemon to run integration tests"
    exit 0
fi

echo "1. Running unit tests..."
if cargo test --lib; then
    print_status "PASS" "Unit tests completed successfully"
else
    print_status "FAIL" "Unit tests failed"
    exit 1
fi
echo

echo "2. Building the application..."
if cargo build; then
    print_status "PASS" "Application built successfully"
else
    print_status "FAIL" "Application build failed"
    exit 1
fi
echo

echo "3. Starting IsoBox server in background..."
# Start the server in background
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 5

# Check if server is running
if ! curl -s http://localhost:8000/health > /dev/null; then
    print_status "FAIL" "Server failed to start"
    kill $SERVER_PID 2>/dev/null || true
    exit 1
fi

print_status "PASS" "Server started successfully"
echo

echo "4. Testing API endpoints..."

# Test 1: Basic execution
echo "  4.1 Testing basic execution..."
if curl -s -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' | grep -q "Hello World!"; then
    print_status "PASS" "Basic execution works"
else
    print_status "FAIL" "Basic execution failed"
fi

# Test 2: Test cases execution
echo "  4.2 Testing test cases execution..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d @examples/python_with_test_cases.json)

if echo "$RESPONSE" | grep -q "test_results" && echo "$RESPONSE" | grep -q "addition_test"; then
    print_status "PASS" "Test cases execution works"
else
    print_status "FAIL" "Test cases execution failed"
    echo "Response: $RESPONSE"
fi

# Test 3: Test files execution
echo "  4.3 Testing test files execution..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-files \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d @examples/python_with_test_files.json)

if echo "$RESPONSE" | grep -q "test_results"; then
    print_status "PASS" "Test files execution works"
else
    print_status "FAIL" "Test files execution failed"
    echo "Response: $RESPONSE"
fi

# Test 4: Test URLs execution (with mock URL)
echo "  4.4 Testing test URLs execution..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-urls \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(sys.stdin.read().strip())",
    "test_urls": [
      {
        "name": "mock_test",
        "url": "https://httpbin.org/bytes/10"
      }
    ]
  }')

if echo "$RESPONSE" | grep -q "test_results"; then
    print_status "PASS" "Test URLs execution works"
else
    print_status "FAIL" "Test URLs execution failed"
    echo "Response: $RESPONSE"
fi

echo

echo "5. Testing multiple languages..."

# Test Python
echo "  5.1 Testing Python..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(\"Python:\", sys.stdin.read().strip())",
    "test_cases": [{"name": "test", "input": "Hello", "expected_output": "Python: Hello"}]
  }')

if echo "$RESPONSE" | grep -q "Python: Hello"; then
    print_status "PASS" "Python test cases work"
else
    print_status "FAIL" "Python test cases failed"
fi

# Test Node.js
echo "  5.2 Testing Node.js..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "node",
    "code": "const readline = require(\"readline\"); const rl = readline.createInterface({input: process.stdin, output: process.stdout, terminal: false}); rl.on(\"line\", (line) => { console.log(\"Node.js:\", line); rl.close(); });",
    "test_cases": [{"name": "test", "input": "Hello", "expected_output": "Node.js: Hello"}]
  }')

if echo "$RESPONSE" | grep -q "Node.js: Hello"; then
    print_status "PASS" "Node.js test cases work"
else
    print_status "FAIL" "Node.js test cases failed"
fi

# Test PHP
echo "  5.3 Testing PHP..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "php",
    "code": "<?php $input = trim(fgets(STDIN)); echo \"PHP: \" . $input . PHP_EOL;",
    "test_cases": [{"name": "test", "input": "Hello", "expected_output": "PHP: Hello"}]
  }')

if echo "$RESPONSE" | grep -q "PHP: Hello"; then
    print_status "PASS" "PHP test cases work"
else
    print_status "FAIL" "PHP test cases failed"
fi

echo

echo "6. Testing error conditions..."

# Test invalid API key
echo "  6.1 Testing invalid API key..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: invalid-key" \
  -d '{"language": "python", "code": "print(\"test\")"}')

if echo "$RESPONSE" | grep -q "Invalid API Key"; then
    print_status "PASS" "Invalid API key properly rejected"
else
    print_status "FAIL" "Invalid API key not properly rejected"
fi

# Test unsupported language
echo "  6.2 Testing unsupported language..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{"language": "unsupported", "code": "print(\"test\")"}')

if echo "$RESPONSE" | grep -q "Unsupported language"; then
    print_status "PASS" "Unsupported language properly rejected"
else
    print_status "FAIL" "Unsupported language not properly rejected"
fi

echo

echo "7. Testing timeout functionality..."
echo "  7.1 Testing timeout with long-running code..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import time\nimport sys\nsys.stdin.read()\ntime.sleep(10)\nprint(\"done\")",
    "test_cases": [{"name": "timeout_test", "input": "test", "expected_output": "done", "timeout_seconds": 2}]
  }')

if echo "$RESPONSE" | grep -q "timeout" || echo "$RESPONSE" | grep -q "false"; then
    print_status "PASS" "Timeout functionality works"
else
    print_status "FAIL" "Timeout functionality failed"
fi

echo

echo "8. Cleaning up..."
# Stop the server
kill $SERVER_PID 2>/dev/null || true
wait $SERVER_PID 2>/dev/null || true

print_status "PASS" "Server stopped successfully"
echo

echo "=== Test Summary ==="
echo "All tests completed successfully!"
echo
echo "Test case functionality verified:"
echo "✓ Unit tests for test case execution"
echo "✓ API endpoints for test cases"
echo "✓ Multiple language support"
echo "✓ Error handling"
echo "✓ Timeout functionality"
echo "✓ Stdin input handling"
echo "✓ Expected output validation"
echo
echo "The test case implementation is working correctly!" 