# IsoBox

A secure, containerized code execution service with support for multiple programming languages. IsoBox provides both HTTP REST API and gRPC interfaces for executing code in isolated Docker containers.

## Features

- **Multi-language Support**: Python, Node.js, Rust, Go, C++, Java, C#, PHP, Ruby, Bash
- **Dual API Support**: HTTP REST API and gRPC
- **Container Isolation**: Each execution runs in a separate Docker container
- **Resource Limits**: Configurable CPU, memory, and process limits
- **Test Case Support**: Run code against multiple test cases with stdin input
- **Multiple Test Input Formats**: Inline test cases, file uploads, and URL-based test cases
- **Individual Test Limits**: Per-test-case timeout and memory limits
- **Test Result Analysis**: Detailed pass/fail results with expected vs actual output
- **Simple Authentication**: API key-based authentication
- **In-memory Caching**: Fast response times with in-memory caching
- **Health Monitoring**: Built-in health check endpoints

## Quick Start

### Prerequisites

- Docker
- Rust (for building from source)
- `grpcurl` (for testing gRPC endpoints)

### Running with Docker

```bash
# Build the Docker image
docker build -t isobox .

# Run with API key authentication
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 50051:50051 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e API_KEYS="your-api-key-here,another-key" \
  isobox
```

### Building from Source

```bash
# Clone the repository
git clone <repository-url>
cd isobox

# Set API keys
export API_KEYS="your-api-key-here,another-key"

# Build and run
cargo run
```

## Environment Variables

| Variable    | Description                            | Default       |
| ----------- | -------------------------------------- | ------------- |
| `PORT`      | HTTP server port                       | `8000`        |
| `GRPC_PORT` | gRPC server port                       | `50051`       |
| `API_KEYS`  | Comma-separated list of valid API keys | `default-key` |

## Authentication

IsoBox uses simple API key authentication:

### HTTP API

Include the API key in the `X-API-Key` header:

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key-here" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}'
```

### gRPC API

Include the API key in the `authorization` metadata:

```bash
grpcurl -plaintext \
  -proto proto/isobox.proto \
  -H "authorization: your-api-key-here" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' \
  localhost:50051 isobox.CodeExecutionService/ExecuteCode
```

## API Endpoints

### HTTP REST API

#### Execute Code

```http
POST /api/v1/execute
Content-Type: application/json
X-API-Key: your-api-key-here

{
  "language": "python",
  "code": "print('Hello World!')"
}
```

#### Execute Code with Test Cases

IsoBox supports running code against multiple test cases with stdin input. Each test case runs in isolation with its own resource limits.

##### Execute with Inline Test Cases

```http
POST /api/v1/execute/test-cases
Content-Type: application/json
X-API-Key: your-api-key-here

{
  "language": "python",
  "code": "import sys\n\ndata = sys.stdin.read().strip()\nnumbers = [int(x) for x in data.split()]\nprint(sum(numbers))",
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
}
```

##### Execute with Test Files

```http
POST /api/v1/execute/test-files
Content-Type: application/json
X-API-Key: your-api-key-here

{
  "language": "python",
  "code": "import sys\n\ndata = sys.stdin.read().strip()\nprint(data[::-1])",
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
}
```

##### Execute with Test URLs

```http
POST /api/v1/execute/test-urls
Content-Type: application/json
X-API-Key: your-api-key-here

{
  "language": "python",
  "code": "import sys\n\ndata = sys.stdin.read().strip()\nprint(len(data))",
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
}
```

##### Test Case Response Format

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

#### Health Check

```http
GET /health
```

### gRPC API

#### Execute Code

```protobuf
service CodeExecutionService {
  rpc ExecuteCode(ExecuteCodeRequest) returns (ExecuteCodeResponse);
  rpc HealthCheck(HealthCheckRequest) returns (HealthCheckResponse);
  rpc GetSupportedLanguages(GetSupportedLanguagesRequest) returns (GetSupportedLanguagesResponse);
}
```

## Supported Languages

| Language | Docker Image                     | Compilation Required |
| -------- | -------------------------------- | -------------------- |
| Python   | python:3.11-slim                 | No                   |
| Node.js  | node:18-slim                     | No                   |
| Rust     | rust:1.70-slim                   | Yes                  |
| Go       | golang:1.21                      | Yes                  |
| C++      | gcc:latest                       | Yes                  |
| Java     | openjdk:17-slim                  | Yes                  |
| C#       | mcr.microsoft.com/dotnet/sdk:7.0 | Yes                  |
| PHP      | php:8.2-cli                      | No                   |
| Ruby     | ruby:3.2-slim                    | No                   |
| Bash     | ubuntu:22.04                     | No                   |

## Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   HTTP Client   │    │   gRPC Client   │    │   Admin Client  │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          ▼                      ▼                      ▼
┌─────────────────────────────────────────────────────────────────┐
│                        IsoBox Server                           │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │ HTTP Server │  │ gRPC Server │  │   Authentication        │  │
│  │   Port 8000 │  │ Port 50051  │  │   (API Key)             │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
│                              │                                  │
│                              ▼                                  │
│  ┌─────────────────────────────────────────────────────────────┐  │
│  │                    Code Executor                           │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │  │
│  │  │   Parser    │  │   Validator │  │   Resource Limits   │  │  │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘  │  │
│  └─────────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Docker Container                            │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐  │
│  │   Language  │  │   Runtime   │  │   Security & Limits     │  │
│  │   Runtime   │  │   Isolation │  │   (CPU, Memory, etc.)   │  │
│  └─────────────┘  └─────────────┘  └─────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

## Security Considerations

- **Container Isolation**: Each code execution runs in a separate Docker container with network isolation
- **Resource Limits**: Configurable limits on CPU, memory, processes, and file descriptors
- **API Key Authentication**: Simple but effective authentication mechanism
- **No Network Access**: Containers run with `--network none` for security
- **Privilege Dropping**: Containers run with dropped capabilities and no new privileges

## Development

### Building

```bash
cargo build
```

### Testing

Run the comprehensive test suite:

```bash
# Run unit tests (fast, no Docker required)
cargo test --lib

# Run unit tests with verbose output
cargo test --lib -- --nocapture

# Run specific test
cargo test test_python_multiple_test_cases

# Run integration tests (requires Docker)
./test_runner.sh

# Run comprehensive E2E tests (CI/CD ready)
./e2e_tests.sh

# Run demo examples
./examples/demo.sh

# Run test cases demo
./examples/test_cases_demo.sh
```

#### Test Coverage

The test suite covers:

- **Unit Tests**: Language registry, resource limits, Docker command building, error handling
- **Integration Tests**: Full API functionality, multiple languages, test case execution
- **E2E Tests**: Complete system testing including test case functionality
- **Demo Scripts**: Example usage and API demonstrations

#### Test Case Testing

The test case functionality is thoroughly tested:

```bash
# Test Python with multiple test cases
cargo test test_python_multiple_test_cases

# Test Node.js with test cases
cargo test test_node_multiple_test_cases

# Test Rust with test cases
cargo test test_rust_multiple_test_cases

# Test Go with test cases
cargo test test_go_multiple_test_cases

# Test timeout functionality
cargo test test_test_case_with_timeout

# Test failing test cases
cargo test test_test_case_with_failing_output
```

#### Manual API Testing

```bash
# Test HTTP API
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}'

# Test gRPC API
grpcurl -plaintext \
  -proto proto/isobox.proto \
  -H "authorization: test-key" \
  -d '{"language": "python", "code": "print(\"Hello World!\")"}' \
  localhost:50051 isobox.CodeExecutionService/ExecuteCode

# Test with test cases
./examples/test_cases_demo.sh
```

### Project Structure

```
isobox/
├── src/
│   ├── main.rs          # Main application entry point
│   ├── executor.rs      # Code execution logic
│   ├── grpc.rs          # gRPC service implementation
│   ├── generated.rs     # Generated protobuf code
│   └── auth/            # Authentication modules
├── proto/
│   └── isobox.proto     # gRPC service definitions
├── examples/            # Example requests
└── Dockerfile           # Container definition
```

## License

[License information]
