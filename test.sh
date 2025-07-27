#!/bin/bash

# Test script for isobox API

echo "🔒 Testing isobox API..."

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

# Test Python execution
echo "🐍 Testing Python execution..."
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Hello from Python!\")\nprint(\"2 + 2 =\", 2 + 2)"
  }' | jq

# Test Node.js execution
echo "🟢 Testing Node.js execution..."
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "node",
    "code": "console.log(\"Hello from Node.js!\"); console.log(\"Current time:\", new Date().toISOString());"
  }' | jq

# Test error handling
echo "❌ Testing error handling..."
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(undefined_variable)"
  }' | jq

# Test unsupported language
echo "🚫 Testing unsupported language..."
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "invalid",
    "code": "print(\"test\")"
  }' | jq

echo "🎉 Tests completed!"
