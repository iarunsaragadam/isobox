#!/bin/bash

# Comprehensive example usage scripts for isobox with test case functionality

# Start the isobox server in the background
echo "🚀 Starting isobox server..."
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "📊 Testing health endpoint..."
curl -s http://localhost:8000/health | jq

echo ""
echo "=== BASIC FUNCTIONALITY EXAMPLES ==="

echo "🐍 Example 1: Simple Python calculation"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "result = 2 ** 10\nprint(f\"2^10 = {result}\")\nprint(f\"Square root of 144 = {144**0.5}\")"
  }' | jq

echo ""
echo "🟢 Example 2: Node.js JSON processing"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "node",
    "code": "const data = {name: \"isobox\", version: \"0.1.0\", secure: true}; console.log(\"Data:\", JSON.stringify(data, null, 2)); console.log(\"Keys:\", Object.keys(data));"
  }' | jq

echo ""
echo "🐍 Example 3: Python with imports"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import datetime\nimport os\nprint(f\"Current time: {datetime.datetime.now()}\")\nprint(f\"Platform: {os.name}\")\nprint(f\"Available modules: {[\"datetime\", \"os\", \"sys\", \"json\"]}\")"
  }' | jq

echo ""
echo "=== TEST CASE FUNCTIONALITY EXAMPLES ==="

echo "🧪 Example 4: Execute with inline test cases"
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

echo ""
echo "📁 Example 5: Execute with test files"
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

echo ""
echo "🌐 Example 6: Execute with test URLs"
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

echo "🐍 Example 7: Python with test cases"
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

echo ""
echo "🟢 Example 8: Node.js with test cases"
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

echo ""
echo "=== ERROR HANDLING EXAMPLES ==="

echo "❌ Example 9: Error handling (Python)"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "print(\"Before error\")\nundefined_variable\nprint(\"After error\")"
  }' | jq

echo ""
echo "❌ Example 10: Unsupported language"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "invalid",
    "code": "print(\"test\")"
  }' | jq

echo ""
echo "🔑 Example 11: Invalid API key"
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: invalid-key" \
  -d '{
    "language": "python",
    "code": "print(\"test\")"
  }' | jq

echo ""
echo "=== ADVANCED TEST CASE EXAMPLES ==="

echo "⏰ Example 12: Timeout functionality"
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

echo ""
echo "❌ Example 13: Failing test case"
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
echo "🛑 Stopping server..."
kill $SERVER_PID
echo "✅ Examples completed!"
echo ""
echo "Examples covered:"
echo "✅ Basic execution (Python, Node.js)"
echo "✅ Test case endpoints (/test-cases, /test-files, /test-urls)"
echo "✅ Multiple languages with test cases"
echo "✅ Error handling (runtime errors, unsupported languages, invalid API keys)"
echo "✅ Timeout and resource limit testing"
echo "✅ Failing test case validation"
