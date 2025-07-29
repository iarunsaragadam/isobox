#!/bin/bash

# Demo script for IsoBox test case functionality
# This script demonstrates the new test case features

set -e

# Configuration
API_BASE_URL="http://localhost:8000/api/v1"
API_KEY="default-key"

echo "=== IsoBox Test Cases Demo ==="
echo "Testing the new test case functionality..."
echo

# Test 1: Execute with inline test cases
echo "1. Testing /execute/test-cases endpoint..."
curl -X POST "${API_BASE_URL}/execute/test-cases" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: ${API_KEY}" \
  -d @python_with_test_cases.json \
  | jq '.'
echo

# Test 2: Execute with test files
echo "2. Testing /execute/test-files endpoint..."
curl -X POST "${API_BASE_URL}/execute/test-files" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: ${API_KEY}" \
  -d @python_with_test_files.json \
  | jq '.'
echo

# Test 3: Execute with test URLs (commented out as URLs don't exist)
echo "3. Testing /execute/test-urls endpoint..."
echo "Note: This test is commented out as the URLs don't exist in this demo"
# curl -X POST "${API_BASE_URL}/execute/test-urls" \
#   -H "Content-Type: application/json" \
#   -H "X-API-Key: ${API_KEY}" \
#   -d @python_with_test_urls.json \
#   | jq '.'
echo

# Test 4: Regular execution (for comparison)
echo "4. Testing regular /execute endpoint (for comparison)..."
curl -X POST "${API_BASE_URL}/execute" \
  -H "Content-Type: application/json" \
  -H "X-API-Key: ${API_KEY}" \
  -d '{
    "language": "python",
    "code": "print(\"Hello from regular execution!\")"
  }' \
  | jq '.'
echo

echo "=== Demo completed ==="
echo
echo "Key features demonstrated:"
echo "- Test cases with expected outputs"
echo "- Individual timeout and memory limits per test case"
echo "- Test case results with pass/fail status"
echo "- Multiple input formats (inline, files, URLs)"
echo "- Stdin input handling for each test case" 