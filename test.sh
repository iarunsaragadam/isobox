#!/bin/bash

# Comprehensive E2E test script for isobox API with test case functionality

echo "🔒 Testing isobox API with test case functionality..."

# Check if server is running
if ! curl -s http://localhost:8000/health > /dev/null; then
    echo "❌ Server is not running. Please start it first:"
    echo "   cargo run"
    echo "   or"
    echo "   docker-compose up"
    exit 1
fi

echo "✅ Server is running"

# Test health endpoint
echo "📊 Testing health endpoint..."
curl -s http://localhost:8000/health | jq

echo ""
echo "=== BASIC FUNCTIONALITY TESTS ==="

# Test Python execution
echo "🐍 Testing Python execution..."
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "print(\"Hello from Python!\")\nprint(\"2 + 2 =\", 2 + 2)"
  }' | jq

# Test Node.js execution
echo "🟢 Testing Node.js execution..."
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "node",
    "code": "console.log(\"Hello from Node.js!\"); console.log(\"Current time:\", new Date().toISOString());"
  }' | jq

echo ""
echo "=== TEST CASE FUNCTIONALITY TESTS ==="

# Test 1: Execute with inline test cases
echo "🧪 Testing /execute/test-cases endpoint..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(sum(int(x) for x in sys.stdin.read().split()))",
    "test_cases": [
      {
        "name": "addition_test",
        "input": "1 2 3 4 5",
        "expected_output": "15",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      },
      {
        "name": "single_number_test",
        "input": "42",
        "expected_output": "42",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      }
    ]
  }' | jq

# Test 2: Execute with test files
echo "📁 Testing /execute/test-files endpoint..."
curl -X POST http://localhost:8000/api/v1/execute/test-files \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(sys.stdin.read().strip()[::-1])",
    "test_files": [
      {
        "name": "string_reverse_test",
        "content": "Hello World"
      },
      {
        "name": "palindrome_test",
        "content": "racecar"
      }
    ]
  }' | jq

# Test 3: Execute with test URLs (using a real URL)
echo "🌐 Testing /execute/test-urls endpoint..."
curl -X POST http://localhost:8000/api/v1/execute/test-urls \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(len(sys.stdin.read().strip()))",
    "test_urls": [
      {
        "name": "httpbin_test",
        "url": "https://httpbin.org/bytes/10"
      }
    ]
  }' | jq

echo ""
echo "=== MULTIPLE LANGUAGE TEST CASES ==="

# Test Python with test cases
echo "🐍 Testing Python with test cases..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
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
  }' | jq

# Test Node.js with test cases
echo "🟢 Testing Node.js with test cases..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
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
  }' | jq

# Test PHP with test cases
echo "🐘 Testing PHP with test cases..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
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
  }' | jq

echo ""
echo "=== ERROR HANDLING TESTS ==="

# Test error handling
echo "❌ Testing error handling..."
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "print(undefined_variable)"
  }' | jq

# Test unsupported language
echo "🚫 Testing unsupported language..."
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "invalid",
    "code": "print(\"test\")"
  }' | jq

# Test invalid API key
echo "🔑 Testing invalid API key..."
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: invalid-key" \
  -d '{
    "language": "python",
    "code": "print(\"test\")"
  }' | jq

echo ""
echo "=== TIMEOUT AND RESOURCE LIMIT TESTS ==="

# Test timeout functionality
echo "⏰ Testing timeout functionality..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
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
  }' | jq

# Test failing test case
echo "❌ Testing failing test case..."
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
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
  }' | jq

echo ""
echo "🎉 E2E Tests completed!"
echo ""
echo "Test coverage:"
echo "✅ Basic execution (Python, Node.js)"
echo "✅ Test case endpoints (/test-cases, /test-files, /test-urls)"
echo "✅ Multiple languages with test cases"
echo "✅ Error handling (runtime errors, unsupported languages, invalid API keys)"
echo "✅ Timeout and resource limit testing"
echo "✅ Failing test case validation"
