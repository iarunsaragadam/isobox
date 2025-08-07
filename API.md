# Isobox API Documentation

Isobox is a secure code execution REST API that allows you to execute arbitrary code in isolated Docker containers with comprehensive resource limits, timeout protection, and test case functionality. This document provides comprehensive API documentation.

## Base URL

```
http://localhost:8000
```

## Authentication

Isobox uses API key authentication for all execution endpoints. You must include your API key in the `X-API-Key` header.

### Environment Configuration

Set your API keys using the `API_KEYS` environment variable:

```bash
export API_KEYS="your-api-key-1,your-api-key-2,default-key"
```

### Default API Key

For development, the default API key is `default-key` if no `API_KEYS` environment variable is set.

## Endpoints

### 1. Health Check

**Endpoint:** `GET /health`

**Description:** Check if the service is running and healthy.

**Authentication:** Not required

**Response:**

```json
{
  "service": "isobox",
  "status": "healthy"
}
```

**Example:**

```bash
curl http://localhost:8000/health
```

### 2. Execute Code

**Endpoint:** `POST /api/v1/execute`

**Description:** Execute code in an isolated Docker container with resource limits and timeout protection.

**Authentication:** Required (`X-API-Key` header)

**Request Body:**

```json
{
  "language": "string",
  "code": "string",
  "test_cases": "array (optional)"
}
```

**Parameters:**

- `language` (required): The programming language to use. See supported languages below.
- `code` (required): The source code to execute
- `test_cases` (optional): Array of test cases to run against the code

**Response:**

```json
{
  "stdout": "string",
  "stderr": "string",
  "exit_code": number,
  "time_taken": number,
  "memory_used": number,
  "test_results": "array (optional)"
}
```

**Response Fields:**

- `stdout`: Standard output from the program execution
- `stderr`: Standard error output from the program execution
- `exit_code`: Program exit code (0 for success, non-zero for errors)
- `time_taken`: Execution time in seconds (if available)
- `memory_used`: Memory usage in bytes (if available)
- `test_results`: Array of test case results (if test cases were provided)

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "print(\"Hello, World!\")"
  }'
```

### 3. Execute Code with Inline Test Cases

**Endpoint:** `POST /api/v1/execute/test-cases`

**Description:** Execute code against multiple inline test cases with stdin input.

**Authentication:** Required (`X-API-Key` header)

**Request Body:**

```json
{
  "language": "string",
  "code": "string",
  "test_cases": [
    {
      "name": "string",
      "input": "string",
      "expected_output": "string (optional)",
      "timeout_seconds": "number (optional)",
      "memory_limit_mb": "number (optional)"
    }
  ]
}
```

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(sum(int(x) for x in sys.stdin.read().split()))",
    "test_cases": [
      {
        "name": "test_1",
        "input": "1 2 3",
        "expected_output": "6",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      },
      {
        "name": "test_2",
        "input": "10 20 30",
        "expected_output": "60",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      }
    ]
  }'
```

### 4. Execute Code with Test Files

**Endpoint:** `POST /api/v1/execute/test-files`

**Description:** Execute code against test cases defined as file content.

**Authentication:** Required (`X-API-Key` header)

**Request Body:**

```json
{
  "language": "string",
  "code": "string",
  "test_files": [
    {
      "name": "string",
      "content": "string"
    }
  ]
}
```

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/execute/test-files \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(sys.stdin.read().strip()[::-1])",
    "test_files": [
      {
        "name": "string_test",
        "content": "Hello World"
      },
      {
        "name": "number_test",
        "content": "12345"
      }
    ]
  }'
```

### 5. Execute Code with Test URLs

**Endpoint:** `POST /api/v1/execute/test-urls`

**Description:** Execute code against test cases downloaded from URLs.

**Authentication:** Required (`X-API-Key` header)

**Request Body:**

```json
{
  "language": "string",
  "code": "string",
  "test_urls": [
    {
      "name": "string",
      "url": "string"
    }
  ]
}
```

**Example:**

```bash
curl -X POST http://localhost:8000/api/v1/execute/test-urls \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import sys\nprint(len(sys.stdin.read().strip()))",
    "test_urls": [
      {
        "name": "remote_test_1",
        "url": "https://example.com/test1.txt"
      },
      {
        "name": "remote_test_2",
        "url": "https://example.com/test2.txt"
      }
    ]
  }'
```

### 6. Authentication Status

**Endpoint:** `GET /auth/status`

**Description:** Check authentication status.

**Authentication:** Not required

**Response:**

```json
{
  "authenticated": false,
  "message": "Authentication not implemented in simplified version"
}
```

### 7. Deduplication Statistics

**Endpoint:** `GET /admin/dedup/stats`

**Description:** Get deduplication statistics.

**Authentication:** Not required

**Response:**

```json
{
  "dedup_enabled": false,
  "message": "Deduplication not implemented in simplified version"
}
```

## Test Case Response Format

When executing with test cases, the response includes detailed test results:

```json
{
  "stdout": "=== Test Case: test_1 ===\n6\n\n=== Test Case: test_2 ===\n60\n",
  "stderr": "",
  "exit_code": 0,
  "time_taken": null,
  "memory_used": null,
  "test_results": [
    {
      "name": "test_1",
      "passed": true,
      "stdout": "6",
      "stderr": "",
      "exit_code": 0,
      "time_taken": 0.123,
      "memory_used": null,
      "error_message": null,
      "input": "1 2 3",
      "expected_output": "6",
      "actual_output": "6"
    },
    {
      "name": "test_2",
      "passed": true,
      "stdout": "60",
      "stderr": "",
      "exit_code": 0,
      "time_taken": 0.098,
      "memory_used": null,
      "error_message": null,
      "input": "10 20 30",
      "expected_output": "60",
      "actual_output": "60"
    }
  ]
}
```

## Resource Limits & Security

Isobox implements comprehensive resource limits inspired by Judge0 to ensure secure and controlled code execution:

### Default Resource Limits

- **CPU Time Limit**: 5 seconds
- **Wall Time Limit**: 10 seconds
- **Memory Limit**: 128 MB
- **Stack Limit**: 64 MB
- **Max Processes**: 50
- **Max Open Files**: 100
- **Network Access**: Disabled

### Security Features

- **Network Isolation**: Containers run with `--network none`
- **Ephemeral Containers**: All containers are removed after execution (`--rm`)
- **Resource Constraints**: Memory, CPU, and process limits enforced
- **Privilege Dropping**: Containers run with `--security-opt no-new-privileges`
- **Capability Restrictions**: All capabilities dropped with `--cap-drop ALL`
- **Temporary Files**: Code files are written to unique temp directories and cleaned up
- **No Persistence**: No data persists between executions
- **Language-Specific Isolation**: Each language runs in its own optimized container

### Timeout Handling

- **Wall Time Timeout**: Programs are automatically terminated after the wall time limit
- **CPU Time Limit**: CPU usage is limited using ulimit
- **Graceful Termination**: Programs receive SIGTERM when timeouts occur
- **Exit Code 124**: Standard exit code for timeout termination

## Supported Languages

Isobox supports **50+ programming languages** including all major languages supported by Judge0:

### Scripting Languages

| Language   | Docker Image           | File Extension | Compilation Required |
| ---------- | ---------------------- | -------------- | -------------------- |
| Python     | `python:3.11`          | `.py`          | No                   |
| Python 2   | `python:2.7`           | `.py`          | No                   |
| Node.js    | `node:20`              | `.js`          | No                   |
| TypeScript | `node:20`              | `.ts`          | Yes                  |
| PHP        | `php:8.2`              | `.php`         | No                   |
| Ruby       | `ruby:3.2`             | `.rb`          | No                   |
| Perl       | `perl:5.38`            | `.pl`          | No                   |
| Bash       | `bash:latest`          | `.sh`          | No                   |
| Lua        | `lua:5.4`              | `.lua`         | No                   |
| R          | `r-base:latest`        | `.r`           | No                   |
| Octave     | `octave/octave:latest` | `.m`           | No                   |

### Compiled Languages

| Language | Docker Image                        | File Extension | Compilation Required |
| -------- | ----------------------------------- | -------------- | -------------------- |
| C        | `gcc:latest`                        | `.c`           | Yes                  |
| C++      | `gcc:latest`                        | `.cpp`         | Yes                  |
| Rust     | `rust:latest`                       | `.rs`          | Yes                  |
| Go       | `golang:1.21`                       | `.go`          | No                   |
| Java     | `openjdk:17`                        | `.java`        | Yes                  |
| C#       | `mcr.microsoft.com/dotnet/sdk:7.0`  | `.cs`          | No                   |
| Kotlin   | `openjdk:17`                        | `.kt`          | Yes                  |
| Swift    | `swift:5.9`                         | `.swift`       | Yes                  |
| Scala    | `openjdk:17`                        | `.scala`       | Yes                  |
| Haskell  | `haskell:9.4`                       | `.hs`          | Yes                  |
| OCaml    | `ocaml/opam:ubuntu-22.04-ocaml-5.0` | `.ml`          | Yes                  |

### Functional Languages

| Language    | Docker Image                       | File Extension | Compilation Required |
| ----------- | ---------------------------------- | -------------- | -------------------- |
| Clojure     | `clojure:latest`                   | `.clj`         | No                   |
| F#          | `mcr.microsoft.com/dotnet/sdk:7.0` | `.fs`          | No                   |
| Elixir      | `elixir:1.15`                      | `.exs`         | No                   |
| Common Lisp | `sbcl:latest`                      | `.lisp`        | No                   |
| Erlang      | `erlang:latest`                    | `.erl`         | No                   |

### Other Languages

| Language          | Docker Image                       | File Extension | Compilation Required |
| ----------------- | ---------------------------------- | -------------- | -------------------- |
| Dart              | `dart:stable`                      | `.dart`        | No                   |
| Groovy            | `openjdk:17`                       | `.groovy`      | No                   |
| Visual Basic .NET | `mcr.microsoft.com/dotnet/sdk:7.0` | `.vb`          | No                   |
| SQL               | `sqlite:latest`                    | `.sql`         | No                   |
| D                 | `dlang2/dmd-ubuntu:latest`         | `.d`           | Yes                  |
| Fortran           | `gcc:latest`                       | `.f90`         | Yes                  |
| Pascal            | `fpc:latest`                       | `.pas`         | Yes                  |
| Assembly          | `nasm:latest`                      | `.asm`         | Yes                  |
| COBOL             | `gnucobol:latest`                  | `.cob`         | Yes                  |
| Prolog            | `swipl:latest`                     | `.pl`          | No                   |
| Basic             | `basic:latest`                     | `.bas`         | No                   |
| Objective-C       | `gcc:latest`                       | `.m`           | Yes                  |

## Examples

### Basic Python Execution

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import math\nprint(f\"π = {math.pi}\")\nprint(f\"e = {math.e}\")"
  }'
```

**Response:**

```json
{
  "stdout": "π = 3.141592653589793\ne = 2.718281828459045\n",
  "stderr": "",
  "exit_code": 0,
  "time_taken": 0.123,
  "memory_used": null
}
```

### Rust Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "rust",
    "code": "fn main() {\n    let numbers: Vec<i32> = (1..=10).collect();\n    let sum: i32 = numbers.iter().sum();\n    println!(\"Sum of 1 to 10: {}\", sum);\n}"
  }'
```

### Java Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "java",
    "code": "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello from Java!\");\n    }\n}"
  }'
```

### Go Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "go",
    "code": "package main\n\nimport (\n    \"fmt\"\n    \"time\"\n)\n\nfunc main() {\n    fmt.Println(\"Hello from Go!\")\n    fmt.Printf(\"Current time: %s\\n\", time.Now().Format(time.RFC3339))\n}"
  }'
```

### Node.js Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "node",
    "code": "console.log(\"Hello from Node.js!\"); console.log(\"Current time:\", new Date().toISOString());"
  }'
```

### C++ Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "cpp",
    "code": "#include <iostream>\n#include <vector>\n#include <algorithm>\n\nint main() {\n    std::vector<int> numbers = {3, 1, 4, 1, 5, 9, 2, 6};\n    std::sort(numbers.begin(), numbers.end());\n    for (int n : numbers) {\n        std::cout << n << \" \";\n    }\n    std::cout << std::endl;\n    return 0;\n}"
  }'
```

### Timeout Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "import time\nwhile True:\n    print(\"Infinite loop\")\n    time.sleep(1)"
  }'
```

**Response:**

```json
{
  "stdout": "",
  "stderr": "Execution timed out after 10.000 seconds",
  "exit_code": 124,
  "time_taken": 10.001,
  "memory_used": null
}
```

### Memory Limit Example

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: default-key" \
  -d '{
    "language": "python",
    "code": "data = [0] * (1024 * 1024 * 200)  # Try to allocate 200MB"
  }'
```

**Response:**

```json
{
  "stdout": "",
  "stderr": "MemoryError: ...",
  "exit_code": 1,
  "time_taken": 0.045,
  "memory_used": null
}
```

## Error Responses

### Missing API Key

```json
{
  "error": "API Key not provided",
  "message": "Please provide an X-API-Key header"
}
```

### Invalid API Key

```json
{
  "error": "Invalid API Key",
  "message": "The provided API key is not valid"
}
```

### Unsupported Language

```json
{
  "error": "Unsupported language: unsupported_lang"
}
```

### Invalid JSON

```json
{
  "error": "Invalid JSON in request body"
}
```

### Missing Required Fields

```json
{
  "error": "Missing required field: language"
}
```

### Compilation Error

```json
{
  "stdout": "",
  "stderr": "error: expected `;`, found `}`\n  --> main.rs:3:5\n   |\n3 | }\n   |     ^\n",
  "exit_code": 1,
  "time_taken": 0.234,
  "memory_used": null
}
```

### Timeout Error

```json
{
  "stdout": "",
  "stderr": "Execution timed out after 10.000 seconds",
  "exit_code": 124,
  "time_taken": 10.001,
  "memory_used": null
}
```

### Test Case Download Error

```json
{
  "error": "Failed to download test case",
  "message": "Failed to download https://example.com/test.txt: Connection failed"
}
```

## Environment Variables

The service can be configured using the following environment variables:

- `PORT`: Server port (default: 8000)
- `GRPC_PORT`: gRPC server port (default: 50051)
- `RUST_LOG`: Log level (default: info)
- `API_KEYS`: Comma-separated list of valid API keys (default: "default-key")

## Docker Deployment

To run the service using Docker:

```bash
docker run -p 8000:8000 -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e API_KEYS="your-api-key-1,your-api-key-2" \
  --user root \
  ghcr.io/yourusername/isobox:latest
```

## gRPC API

Isobox also provides a gRPC API on port 50051. See the `proto/isobox.proto` file for the complete gRPC service definition.

### gRPC Endpoints

- `ExecuteCode`: Execute code in a specified language
- `HealthCheck`: Health check endpoint
- `GetSupportedLanguages`: Get list of supported languages

### gRPC Authentication

Include the API key in the `authorization` metadata:

```bash
grpcurl -plaintext \
  -proto proto/isobox.proto \
  -H "authorization: your-api-key-here" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' \
  localhost:50051 isobox.CodeExecutionService/ExecuteCode
```

## Performance Considerations

- **First Run**: The first execution of each language may take longer as Docker images are pulled
- **Compilation**: Compiled languages (C, C++, Rust, Java, etc.) have an additional compilation step
- **Memory Usage**: Different languages have varying memory requirements
- **Execution Time**: Scripting languages typically start faster than compiled languages
- **Resource Limits**: All executions are subject to timeout and memory constraints
- **Container Overhead**: Each execution creates a new isolated container
- **Test Cases**: Multiple test cases run sequentially in separate containers

## Development

For development setup and contributing guidelines, see the main [README.md](README.md) file.

## Language-Specific Notes

### Compiled Languages

- C, C++, Rust, Java, Kotlin, Swift, Scala, Haskell, OCaml, D, Fortran, Pascal, Assembly, COBOL, Basic, Objective-C require compilation
- Compilation errors are returned in the `stderr` field
- Successful compilation produces an executable that is then run
- Compilation is also subject to resource limits

### Scripting Languages

- Python, Node.js, PHP, Ruby, Perl, Bash, Lua, R, Octave, TypeScript, Dart, Elixir, Clojure, F#, Groovy, Prolog, Visual Basic .NET, SQL run directly
- No compilation step required

### Special Cases

- **TypeScript**: Requires compilation to JavaScript before execution
- **C#**: Uses .NET runtime, no explicit compilation step
- **SQL**: Executes against SQLite database
- **Assembly**: Uses NASM assembler and GNU linker
- **Objective-C**: Requires Foundation framework

## Troubleshooting

### Common Issues

1. **Missing API Key**: Ensure you include the `X-API-Key` header with a valid API key
2. **Language Not Supported**: Check the supported languages list above
3. **Compilation Errors**: Review the `stderr` output for specific error messages
4. **Timeout Issues**: Programs may be terminated if they exceed the time limit
5. **Memory Issues**: Large programs may exceed container memory limits
6. **Resource Limits**: Check if your program is hitting CPU, memory, or process limits
7. **Test Case Issues**: Ensure test case input format matches your code's expectations

### Debugging

Enable debug logging by setting `RUST_LOG=debug`:

```bash
RUST_LOG=debug cargo run
```

This will provide detailed information about:

- Container creation and execution
- File operations
- Compilation steps
- Cleanup processes
- Resource limit enforcement
- Timeout handling
- Authentication checks

### Resource Limit Troubleshooting

If your program is hitting resource limits:

1. **Timeout Issues**: Optimize your code or break it into smaller chunks
2. **Memory Issues**: Reduce memory usage or use streaming approaches
3. **Process Limits**: Avoid spawning too many subprocesses
4. **File Limits**: Close file handles properly

## Security Considerations

- **Network Isolation**: All containers run without network access
- **Resource Limits**: Strict limits prevent resource exhaustion attacks
- **Privilege Dropping**: Containers run with minimal privileges
- **Ephemeral Execution**: No data persists between executions
- **Automatic Cleanup**: All temporary files and containers are removed
- **API Key Authentication**: All execution endpoints require valid API keys

## Rate Limiting

Currently, there are no rate limits implemented. However, it's recommended to implement rate limiting for production use.

---

For more information, see the main [README.md](README.md) file.
