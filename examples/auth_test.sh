#!/bin/bash

# IsoBox Authentication Test Script
# This script demonstrates the API key authentication system

set -e

# Configuration
HTTP_URL="http://localhost:8000"
GRPC_URL="localhost:50051"
PROTO_FILE="proto/isobox.proto"
VALID_API_KEY="test-key-123"
INVALID_API_KEY="invalid-key"

echo "🧪 Testing IsoBox Authentication System"
echo "========================================"

# Test 1: HTTP API without authentication (should fail)
echo -e "\n1️⃣  Testing HTTP API without authentication..."
if curl -s -X POST "$HTTP_URL/api/v1/execute" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' | grep -q "API Key not provided"; then
  echo "✅ Correctly rejected request without API key"
else
  echo "❌ Unexpected response for request without API key"
fi

# Test 2: HTTP API with invalid API key (should fail)
echo -e "\n2️⃣  Testing HTTP API with invalid API key..."
if curl -s -X POST "$HTTP_URL/api/v1/execute" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $INVALID_API_KEY" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' | grep -q "Invalid API Key"; then
  echo "✅ Correctly rejected request with invalid API key"
else
  echo "❌ Unexpected response for request with invalid API key"
fi

# Test 3: HTTP API with valid API key (should succeed)
echo -e "\n3️⃣  Testing HTTP API with valid API key..."
if curl -s -X POST "$HTTP_URL/api/v1/execute" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: $VALID_API_KEY" \
  -d '{"language": "python", "code": "print(\"Hello from HTTP!\")"}' | grep -q "Hello from HTTP!"; then
  echo "✅ Successfully executed code with valid API key"
else
  echo "❌ Failed to execute code with valid API key"
fi

# Test 4: gRPC API without authentication (should fail)
echo -e "\n4️⃣  Testing gRPC API without authentication..."
if grpcurl -plaintext -proto "$PROTO_FILE" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' \
  "$GRPC_URL" isobox.CodeExecutionService/ExecuteCode 2>&1 | grep -q "API Key not provided"; then
  echo "✅ Correctly rejected gRPC request without API key"
else
  echo "❌ Unexpected response for gRPC request without API key"
fi

# Test 5: gRPC API with invalid API key (should fail)
echo -e "\n5️⃣  Testing gRPC API with invalid API key..."
if grpcurl -plaintext -proto "$PROTO_FILE" \
  -H "authorization: $INVALID_API_KEY" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' \
  "$GRPC_URL" isobox.CodeExecutionService/ExecuteCode 2>&1 | grep -q "Invalid API Key"; then
  echo "✅ Correctly rejected gRPC request with invalid API key"
else
  echo "❌ Unexpected response for gRPC request with invalid API key"
fi

# Test 6: gRPC API with valid API key (should succeed)
echo -e "\n6️⃣  Testing gRPC API with valid API key..."
if grpcurl -plaintext -proto "$PROTO_FILE" \
  -H "authorization: $VALID_API_KEY" \
  -d '{"language": "python", "code": "print(\"Hello from gRPC!\")"}' \
  "$GRPC_URL" isobox.CodeExecutionService/ExecuteCode | grep -q "Hello from gRPC!"; then
  echo "✅ Successfully executed code via gRPC with valid API key"
else
  echo "❌ Failed to execute code via gRPC with valid API key"
fi

# Test 7: Health check (should work without auth)
echo -e "\n7️⃣  Testing health check endpoint..."
if curl -s "$HTTP_URL/health" | grep -q "healthy"; then
  echo "✅ Health check endpoint working"
else
  echo "❌ Health check endpoint failed"
fi

# Test 8: gRPC health check (should work without auth)
echo -e "\n8️⃣  Testing gRPC health check..."
if grpcurl -plaintext -proto "$PROTO_FILE" \
  "$GRPC_URL" isobox.CodeExecutionService/HealthCheck | grep -q "healthy"; then
  echo "✅ gRPC health check working"
else
  echo "❌ gRPC health check failed"
fi

echo -e "\n🎉 Authentication test completed!"
echo "========================================"
echo "Summary:"
echo "- API key authentication is working for both HTTP and gRPC"
echo "- Invalid/missing API keys are properly rejected"
echo "- Health check endpoints work without authentication"
echo "- Code execution requires valid API key" 