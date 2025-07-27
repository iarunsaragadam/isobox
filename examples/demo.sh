#!/bin/bash

# Example usage scripts for isobox

# Start the isobox server in the background
echo "🚀 Starting isobox server..."
cargo run &
SERVER_PID=$!

# Wait for server to start
sleep 3

echo "📊 Testing health endpoint..."
curl -s http://localhost:8000/health | jq

echo ""
echo "🐍 Example 1: Simple Python calculation"
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "result = 2 ** 10\nprint(f\"2^10 = {result}\")\nprint(f\"Square root of 144 = {144**0.5}\")"
  }' | jq

echo ""
echo "🟢 Example 2: Node.js JSON processing"
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "node",
    "code": "const data = {name: \"isobox\", version: \"0.1.0\", secure: true}; console.log(\"Data:\", JSON.stringify(data, null, 2)); console.log(\"Keys:\", Object.keys(data));"
  }' | jq

echo ""
echo "🐍 Example 3: Python with imports"
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "import datetime\nimport os\nprint(f\"Current time: {datetime.datetime.now()}\")\nprint(f\"Platform: {os.name}\")\nprint(f\"Available modules: {[\"datetime\", \"os\", \"sys\", \"json\"]}\")"
  }' | jq

echo ""
echo "❌ Example 4: Error handling (Python)"
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Before error\")\nundefined_variable\nprint(\"After error\")"
  }' | jq

echo ""
echo "❌ Example 5: Unsupported language"
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "rust",
    "code": "fn main() { println!(\"Hello from Rust!\"); }"
  }' | jq

echo ""
echo "🛑 Stopping server..."
kill $SERVER_PID
echo "✅ Examples completed!"
