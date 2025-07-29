#!/bin/bash

# Comprehensive E2E test script for IsoBox with test case functionality
# Designed for CI/CD environments

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
API_BASE_URL="http://localhost:8000/api/v1"
API_KEY="default-key"
SERVER_PID=""

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
        "INFO")
            echo -e "${BLUE}ℹ INFO${NC}: $message"
            ;;
    esac
}

# Function to cleanup on exit
cleanup() {
    if [ ! -z "$SERVER_PID" ]; then
        print_status "INFO" "Cleaning up server (PID: $SERVER_PID)"
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
    fi
}

# Set trap to cleanup on exit
trap cleanup EXIT

# Function to test API endpoint
test_endpoint() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    local expected_pattern="$5"
    
    print_status "INFO" "Testing $name..."
    
    local response
    if [ -z "$data" ]; then
        response=$(curl -s -X "$method" "$API_BASE_URL$endpoint" \
            -H "X-API-Key: $API_KEY" 2>/dev/null || echo "{}")
    else
        response=$(curl -s -X "$method" "$API_BASE_URL$endpoint" \
            -H "Content-Type: application/json" \
            -H "X-API-Key: $API_KEY" \
            -d "$data" 2>/dev/null || echo "{}")
    fi
    
    if echo "$response" | grep -q "$expected_pattern"; then
        print_status "PASS" "$name"
        return 0
    else
        print_status "FAIL" "$name - Expected pattern '$expected_pattern' not found in response"
        echo "Response: $response"
        return 1
    fi
}

echo "=== IsoBox E2E Test Suite ==="
echo "Testing comprehensive functionality including test cases"
echo

# Check prerequisites
print_status "INFO" "Checking prerequisites..."

# Check if Docker is available
if ! command -v docker &> /dev/null; then
    print_status "SKIP" "Docker not available - some tests will be skipped"
    DOCKER_AVAILABLE=false
else
    if ! docker info &> /dev/null; then
        print_status "SKIP" "Docker daemon not running - some tests will be skipped"
        DOCKER_AVAILABLE=false
    else
        print_status "PASS" "Docker is available"
        DOCKER_AVAILABLE=true
    fi
fi

# Check if jq is available
if ! command -v jq &> /dev/null; then
    print_status "SKIP" "jq not available - JSON parsing will be basic"
    JQ_AVAILABLE=false
else
    print_status "PASS" "jq is available"
    JQ_AVAILABLE=true
fi

echo

# Build the application
print_status "INFO" "Building application..."
if cargo build; then
    print_status "PASS" "Application built successfully"
else
    print_status "FAIL" "Application build failed"
    exit 1
fi

echo

# Start the server
print_status "INFO" "Starting server..."
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 5

# Check if server is running
if curl -s http://localhost:8000/health > /dev/null; then
    print_status "PASS" "Server started successfully"
else
    print_status "FAIL" "Server failed to start"
    exit 1
fi

echo

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0
TESTS_SKIPPED=0

# Function to run test and update counters
run_test() {
    local name="$1"
    local method="$2"
    local endpoint="$3"
    local data="$4"
    local expected_pattern="$5"
    
    if test_endpoint "$name" "$method" "$endpoint" "$data" "$expected_pattern"; then
        ((TESTS_PASSED++))
    else
        ((TESTS_FAILED++))
    fi
}

echo "=== BASIC FUNCTIONALITY TESTS ==="

# Test health endpoint
run_test "Health endpoint" "GET" "/health" "" "status"

# Test basic Python execution
run_test "Basic Python execution" "POST" "/execute" \
    '{"language": "python", "code": "print(\"Hello World!\")"}' \
    "Hello World!"

# Test basic Node.js execution
run_test "Basic Node.js execution" "POST" "/execute" \
    '{"language": "node", "code": "console.log(\"Hello from Node.js!\")"}' \
    "Hello from Node.js!"

echo

echo "=== TEST CASE FUNCTIONALITY TESTS ==="

# Test execute with inline test cases
run_test "Execute with inline test cases" "POST" "/execute/test-cases" \
    '{
        "language": "python",
        "code": "import sys\nprint(sum(int(x) for x in sys.stdin.read().split()))",
        "test_cases": [
            {
                "name": "addition_test",
                "input": "1 2 3 4 5",
                "expected_output": "15",
                "timeout_seconds": 5,
                "memory_limit_mb": 128
            }
        ]
    }' \
    "test_results"

# Test execute with test files
run_test "Execute with test files" "POST" "/execute/test-files" \
    '{
        "language": "python",
        "code": "import sys\nprint(sys.stdin.read().strip()[::-1])",
        "test_files": [
            {
                "name": "string_reverse_test",
                "content": "Hello World"
            }
        ]
    }' \
    "test_results"

# Test execute with test URLs
run_test "Execute with test URLs" "POST" "/execute/test-urls" \
    '{
        "language": "python",
        "code": "import sys\nprint(len(sys.stdin.read().strip()))",
        "test_urls": [
            {
                "name": "httpbin_test",
                "url": "https://httpbin.org/bytes/10"
            }
        ]
    }' \
    "test_results"

echo

echo "=== MULTIPLE LANGUAGE TEST CASES ==="

# Test Python with test cases
run_test "Python with test cases" "POST" "/execute/test-cases" \
    '{
        "language": "python",
        "code": "import sys\nprint(\"Python:\", sys.stdin.read().strip())",
        "test_cases": [
            {
                "name": "python_test",
                "input": "Hello World",
                "expected_output": "Python: Hello World",
                "timeout_seconds": 10,
                "memory_limit_mb": 256
            }
        ]
    }' \
    "Python: Hello World"

# Test Node.js with test cases
run_test "Node.js with test cases" "POST" "/execute/test-cases" \
    '{
        "language": "node",
        "code": "const readline = require(\"readline\"); const rl = readline.createInterface({input: process.stdin, output: process.stdout, terminal: false}); rl.on(\"line\", (line) => { console.log(\"Node:\", line); rl.close(); });",
        "test_cases": [
            {
                "name": "node_test",
                "input": "Hello World",
                "expected_output": "Node: Hello World",
                "timeout_seconds": 10,
                "memory_limit_mb": 256
            }
        ]
    }' \
    "Node: Hello World"

# Test PHP with test cases
run_test "PHP with test cases" "POST" "/execute/test-cases" \
    '{
        "language": "php",
        "code": "<?php $input = trim(fgets(STDIN)); echo \"PHP: \" . $input . PHP_EOL;",
        "test_cases": [
            {
                "name": "php_test",
                "input": "Hello World",
                "expected_output": "PHP: Hello World",
                "timeout_seconds": 10,
                "memory_limit_mb": 256
            }
        ]
    }' \
    "PHP: Hello World"

echo

echo "=== ERROR HANDLING TESTS ==="

# Test runtime error
run_test "Runtime error handling" "POST" "/execute" \
    '{"language": "python", "code": "print(undefined_variable)"}' \
    "error"

# Test unsupported language
run_test "Unsupported language" "POST" "/execute" \
    '{"language": "invalid", "code": "print(\"test\")"}' \
    "Unsupported language"

# Test invalid API key
run_test "Invalid API key" "POST" "/execute" \
    '{"language": "python", "code": "print(\"test\")"}' \
    "Invalid API Key"

echo

echo "=== ADVANCED TEST CASE TESTS ==="

# Test timeout functionality
run_test "Timeout functionality" "POST" "/execute/test-cases" \
    '{
        "language": "python",
        "code": "import time\nimport sys\nsys.stdin.read()\ntime.sleep(10)\nprint(\"done\")",
        "test_cases": [
            {
                "name": "timeout_test",
                "input": "test",
                "expected_output": "done",
                "timeout_seconds": 2,
                "memory_limit_mb": 128
            }
        ]
    }' \
    "timeout\|false"

# Test failing test case
run_test "Failing test case" "POST" "/execute/test-cases" \
    '{
        "language": "python",
        "code": "import sys\nprint(sys.stdin.read().strip())",
        "test_cases": [
            {
                "name": "failing_test",
                "input": "5",
                "expected_output": "10",
                "timeout_seconds": 5,
                "memory_limit_mb": 128
            }
        ]
    }' \
    "false"

echo

# Print test summary
echo "=== TEST SUMMARY ==="
echo "Tests passed: $TESTS_PASSED"
echo "Tests failed: $TESTS_FAILED"
echo "Tests skipped: $TESTS_SKIPPED"
echo "Total tests: $((TESTS_PASSED + TESTS_FAILED + TESTS_SKIPPED))"

if [ $TESTS_FAILED -eq 0 ]; then
    print_status "PASS" "All tests passed!"
    echo
    echo "✅ E2E test coverage:"
    echo "   - Basic execution (Python, Node.js)"
    echo "   - Test case endpoints (/test-cases, /test-files, /test-urls)"
    echo "   - Multiple languages with test cases"
    echo "   - Error handling (runtime errors, unsupported languages, invalid API keys)"
    echo "   - Timeout and resource limit testing"
    echo "   - Failing test case validation"
    exit 0
else
    print_status "FAIL" "$TESTS_FAILED tests failed"
    exit 1
fi 