# IsoBox Test Cases - Comprehensive Testing Guide

This document provides comprehensive information about the test case functionality in IsoBox and how to test it.

## Overview

IsoBox now supports running code against multiple test cases with stdin input. Each test case runs in isolation with its own resource limits, providing a secure and efficient way to validate code submissions.

## Test Case Architecture

### One Container Per Submission, Loop Over Test Cases Internally

The implementation follows the optimal strategy:

1. **Spin up one sandbox per user submission**
2. **Inside that sandbox, run the user's code against all test cases sequentially**
3. **Each test case runs with a fresh interpreter state or subprocess**
4. **Return per-test-case pass/fail and output**

### Key Benefits

✅ **Isolated from other users**: No cross-user leakage or attack surface
✅ **Efficient**: Avoids the cost of container-per-test-case
✅ **Control**: Can cap memory/CPU/time for each test case individually
✅ **Scalable**: Fewer containers to manage and bill for

## Test Case Data Structures

### TestCase

```rust
pub struct TestCase {
    pub name: String,                    // Unique identifier for the test
    pub input: String,                   // Stdin input for the test
    pub expected_output: Option<String>, // Expected stdout output
    pub timeout_seconds: Option<u32>,    // Individual timeout limit
    pub memory_limit_mb: Option<u64>,    // Individual memory limit
}
```

### TestCaseResult

```rust
pub struct TestCaseResult {
    pub name: String,                    // Test case name
    pub passed: bool,                    // Whether test passed
    pub stdout: String,                  // Actual stdout output
    pub stderr: String,                  // Actual stderr output
    pub exit_code: i32,                  // Process exit code
    pub time_taken: Option<f64>,         // Execution time in seconds
    pub memory_used: Option<u64>,        // Memory usage in bytes
    pub error_message: Option<String>,   // Error description if failed
    pub input: String,                   // Original input
    pub expected_output: Option<String>, // Expected output
    pub actual_output: String,           // Actual output (same as stdout)
}
```

## API Endpoints

### 1. Execute with Inline Test Cases

```http
POST /api/v1/execute/test-cases
Content-Type: application/json
X-API-Key: your-api-key

{
  "language": "python",
  "code": "import sys\nprint(sum(int(x) for x in sys.stdin.read().split()))",
  "test_cases": [
    {
      "name": "test_1",
      "input": "1 2 3",
      "expected_output": "6",
      "timeout_seconds": 5,
      "memory_limit_mb": 128
    }
  ]
}
```

### 2. Execute with Test Files

```http
POST /api/v1/execute/test-files
Content-Type: application/json
X-API-Key: your-api-key

{
  "language": "python",
  "code": "import sys\nprint(sys.stdin.read().strip()[::-1])",
  "test_files": [
    {
      "name": "string_test",
      "content": "Hello World"
    }
  ]
}
```

### 3. Execute with Test URLs

```http
POST /api/v1/execute/test-urls
Content-Type: application/json
X-API-Key: your-api-key

{
  "language": "python",
  "code": "import sys\nprint(len(sys.stdin.read().strip()))",
  "test_urls": [
    {
      "name": "remote_test",
      "url": "https://example.com/test.txt"
    }
  ]
}
```

## Testing Strategy

### Unit Tests

The codebase includes comprehensive unit tests in `src/executor.rs`:

1. **`test_python_multiple_test_cases`**: Tests Python with multiple test cases
2. **`test_node_multiple_test_cases`**: Tests Node.js with multiple test cases
3. **`test_rust_multiple_test_cases`**: Tests Rust with multiple test cases
4. **`test_go_multiple_test_cases`**: Tests Go with multiple test cases
5. **`test_test_case_without_expected_output`**: Tests cases without expected output
6. **`test_test_case_with_failing_output`**: Tests failing test cases
7. **`test_test_case_with_timeout`**: Tests timeout functionality
8. **`test_multiple_languages_with_test_cases`**: Tests multiple languages

### Running Unit Tests

```bash
# Run all unit tests
cargo test --lib

# Run specific test
cargo test test_python_multiple_test_cases

# Run tests with output
cargo test --lib -- --nocapture
```

### Integration Tests

The `test_runner.sh` script provides comprehensive integration testing:

```bash
# Run all integration tests
./test_runner.sh

# Run specific test sections
./test_runner.sh 4.1  # Test basic execution only
```

## Language-Specific Test Examples

### Python

```python
import sys

# Read input from stdin
data = sys.stdin.read().strip()

# Simple example: if input is numbers, add them
if data.replace(' ', '').replace('\n', '').isdigit():
    numbers = [int(x) for x in data.split()]
    result = sum(numbers)
    print(result)
else:
    # If it's text, reverse it
    print(data[::-1])
```

**Test Cases:**

- Input: `"5\n3"` → Expected: `"8"`
- Input: `"Hello World"` → Expected: `"dlroW olleH"`
- Input: `"1 2 3 4 5"` → Expected: `"15"`

### Node.js

```javascript
const readline = require("readline");

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false,
});

let input = "";
rl.on("line", (line) => {
  input += line + "\n";
});

rl.on("close", () => {
  const data = input.trim();

  // If input contains only numbers, sum them
  if (/^\d[\d\s]*$/.test(data)) {
    const numbers = data.split(/\s+/).map(Number);
    const sum = numbers.reduce((a, b) => a + b, 0);
    console.log(sum);
  } else {
    // Otherwise, print the length
    console.log(data.length);
  }
});
```

### Rust

```rust
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let data = input.trim();

    // If input contains only numbers, sum them
    if data.chars().all(|c| c.is_ascii_digit() || c.is_ascii_whitespace()) {
        let sum: i32 = data.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .sum();
        println!("{}", sum);
    } else {
        // Otherwise, reverse the string
        let reversed: String = data.chars().rev().collect();
        println!("{}", reversed);
    }
}
```

### Go

```go
package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
    "strings"
    "unicode"
)

func main() {
    scanner := bufio.NewScanner(os.Stdin)
    var input string
    for scanner.Scan() {
        input += scanner.Text() + "\n"
    }
    data := strings.TrimSpace(input)

    // If input contains only numbers, sum them
    allDigits := true
    for _, char := range data {
        if !unicode.IsDigit(char) && !unicode.IsSpace(char) {
            allDigits = false
            break
        }
    }

    if allDigits {
        sum := 0
        for _, numStr := range strings.Fields(data) {
            if num, err := strconv.Atoi(numStr); err == nil {
                sum += num
            }
        }
        fmt.Println(sum)
    } else {
        // Otherwise, convert to uppercase
        fmt.Println(strings.ToUpper(data))
    }
}
```

## Test Case Validation

### Pass/Fail Criteria

A test case is considered **PASSED** if:

1. **Exit code is 0** (successful execution)
2. **If expected_output is provided**: stdout matches expected_output exactly (after trimming whitespace)
3. **If no expected_output**: only exit code matters

A test case is considered **FAILED** if:

1. **Exit code is non-zero** (runtime error, compilation error, etc.)
2. **stdout doesn't match expected_output** (when provided)
3. **Timeout occurs** (execution exceeds timeout_seconds)
4. **Memory limit exceeded** (when memory_limit_mb is set)

### Error Messages

When a test fails, the system provides detailed error messages:

- **Output mismatch**: `"Expected: 'expected', Got: 'actual'"`
- **Exit code error**: `"Exit code: 1"`
- **Timeout**: `"Execution timed out after X.XXX seconds"`

## Performance Testing

### Resource Limits

Each test case can have individual resource limits:

```json
{
  "name": "performance_test",
  "input": "large_input_data",
  "expected_output": "expected_result",
  "timeout_seconds": 30, // 30 second timeout
  "memory_limit_mb": 512 // 512 MB memory limit
}
```

### Benchmarking

To test performance across multiple test cases:

```bash
# Run performance test
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d @examples/performance_test.json
```

## Security Testing

### Sandbox Isolation

Test cases run in isolated Docker containers with:

- **Network isolation**: `--network none`
- **Privilege dropping**: `--cap-drop ALL`
- **No new privileges**: `--security-opt no-new-privileges`
- **Resource limits**: CPU, memory, process limits

### Malicious Code Testing

Test cases can include malicious code to verify security:

```json
{
  "name": "security_test",
  "input": "test",
  "expected_output": "test",
  "timeout_seconds": 5,
  "memory_limit_mb": 128
}
```

With code that attempts:

- File system access outside container
- Network access
- Privilege escalation
- Resource exhaustion

## Continuous Integration

### GitHub Actions

Example CI configuration:

```yaml
name: Test Cases CI

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Run unit tests
        run: cargo test --lib
      - name: Run integration tests
        run: ./test_runner.sh
```

### Local Development

For local development and testing:

```bash
# Quick test
cargo test

# Full integration test
./test_runner.sh

# Test specific language
cargo test test_python_multiple_test_cases

# Test with verbose output
cargo test -- --nocapture
```

## Troubleshooting

### Common Issues

1. **Docker not available**: Ensure Docker is installed and running
2. **Timeout errors**: Increase timeout_seconds for long-running tests
3. **Memory errors**: Increase memory_limit_mb for memory-intensive tests
4. **Network errors**: Test URLs require internet connectivity

### Debug Mode

Enable debug logging:

```bash
RUST_LOG=debug cargo run
```

### Test Case Debugging

To debug specific test cases:

```bash
# Run with detailed output
curl -v -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d @examples/python_with_test_cases.json
```

## Best Practices

### Test Case Design

1. **Clear naming**: Use descriptive test case names
2. **Minimal input**: Keep test inputs focused and minimal
3. **Expected outputs**: Always provide expected outputs when possible
4. **Resource limits**: Set appropriate timeouts and memory limits
5. **Edge cases**: Include edge cases and error conditions

### Performance Considerations

1. **Batch testing**: Use multiple test cases per request for efficiency
2. **Resource limits**: Set realistic limits to prevent resource exhaustion
3. **Timeout values**: Balance between thorough testing and quick feedback
4. **Memory usage**: Monitor memory usage patterns

### Security Considerations

1. **Input validation**: Validate all inputs before processing
2. **Resource limits**: Always set resource limits to prevent abuse
3. **Isolation**: Ensure proper container isolation
4. **Monitoring**: Monitor for suspicious patterns or resource usage

## Conclusion

The test case functionality in IsoBox provides a robust, secure, and efficient way to validate code submissions across multiple programming languages. The comprehensive test suite ensures reliability and correctness of the implementation.

For more information, see the main README.md and API documentation.
