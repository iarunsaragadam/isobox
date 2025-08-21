# IsoBox 🚀

**A Secure, Containerized Code Execution Service with Multi-Authentication Support**

IsoBox is a production-ready code execution service that runs code in isolated Docker containers with comprehensive security, multiple authentication methods, and support for 10+ programming languages. Perfect for online IDEs, coding platforms, and educational applications.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/Docker-Required-blue.svg)](https://www.docker.com/)

## 🌟 Key Features

### 🔐 **Multi-Authentication Support**

- **API Key Authentication** - Simple key-based auth
- **JWT Authentication** - Google, Auth0, custom providers
- **OAuth2 Authentication** - Google, Meta, GitHub, Firebase, AWS Cognito
- **mTLS Authentication** - Client certificate-based auth
- **No Authentication** - For development/testing

### 🚀 **Code Execution**

- **10+ Programming Languages** - Python, Node.js, Java, Go, Rust, C++, C, PHP, Ruby, Bash
- **Container Isolation** - Each execution in separate Docker container
- **Resource Limits** - CPU, memory, process limits
- **Timeout Protection** - Configurable execution timeouts
- **Test Case Support** - Run code against multiple test cases

### 📊 **Advanced Features**

- **Test Case Execution** - Inline, file-based, and URL-based test cases
- **Code Deduplication** - Hash-based caching to prevent duplicate execution
- **Performance Monitoring** - Execution time and memory usage tracking
- **Health Monitoring** - Built-in health check endpoints
- **Dual API Support** - HTTP REST API and gRPC

## 🚀 Quick Start

### Prerequisites

- **Docker** (required for code execution)
- **Rust** (for building from source)
- **grpcurl** (for testing gRPC endpoints)

### Option 1: Docker Compose (Recommended)

#### API Key Authentication

```bash
# Clone the repository
git clone <repository-url>
cd isobox

# Start with API key authentication
docker compose -f docker-compose-with-auth.yml up -d

# Test the service
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key-123" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

#### Firebase OAuth2 Authentication

```bash
# Start with Firebase authentication
docker compose -f docker-compose-firebase.yml up -d

# Test with Firebase token
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_FIREBASE_TOKEN" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

### Option 2: Manual Docker

```bash
# Build the Docker image
docker build -t isobox .

# Run with API key authentication
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key-here,another-key" \
  -e API_KEY_HEADER=X-API-Key \
  isobox
```

### Option 3: Build from Source

```bash
# Clone and build
git clone <repository-url>
cd isobox

# Set environment variables
export AUTH_TYPE=apikey
export API_KEYS="your-api-key-here,another-key"
export API_KEY_HEADER=X-API-Key

# Build and run
cargo run
```

## 📚 Documentation

- **[API Documentation](API.md)** - Complete API reference with examples
- **[Authentication Guide](AUTHENTICATION.md)** - All authentication methods and configuration
- **[Configuration Guide](CONFIGURATION.md)** - Environment variables and settings
- **[Test Cases Guide](TEST_CASES.md)** - How to use test case functionality
- **[Development Guide](DEVELOPMENT.md)** - Contributing and development setup

## 🔧 Configuration

### Environment Variables

| Variable          | Description                                                     | Default       | Required |
| ----------------- | --------------------------------------------------------------- | ------------- | -------- |
| `AUTH_TYPE`       | Authentication type (`none`, `apikey`, `jwt`, `oauth2`, `mtls`) | `apikey`      | No       |
| `API_KEYS`        | Comma-separated API keys                                        | `default-key` | No       |
| `API_KEY_HEADER`  | Header name for API key                                         | `X-API-Key`   | No       |
| `REST_PORT`       | HTTP REST API port                                              | `8000`        | No       |
| `GRPC_PORT`       | gRPC API port                                                   | `9000`        | No       |
| `DEDUP_ENABLED`   | Enable code deduplication                                       | `true`        | No       |
| `DEDUP_CACHE_TTL` | Cache TTL in seconds                                            | `3600`        | No       |

### Authentication Configuration

#### API Key Authentication

```bash
AUTH_TYPE=apikey
API_KEYS=key1,key2,key3
API_KEY_HEADER=X-API-Key
```

#### JWT Authentication

```bash
AUTH_TYPE=jwt
JWT_ISSUER_URL=https://accounts.google.com
JWT_AUDIENCE=your-app-id
JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
```

#### OAuth2 Authentication (Firebase)

```bash
AUTH_TYPE=oauth2
OAUTH2_PROVIDER=firebase
OAUTH2_CLIENT_ID=your-firebase-client-id
OAUTH2_CLIENT_SECRET=your-firebase-client-secret
OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo
```

## 🛠️ API Usage

### HTTP REST API

#### Basic Code Execution

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "language": "python",
    "code": "print(\"Hello, World!\")"
  }'
```

#### Code Execution with Test Cases

```bash
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "language": "python",
    "code": "import sys\n\ndata = sys.stdin.read().strip()\nnumbers = [int(x) for x in data.split()]\nprint(sum(numbers))",
    "test_cases": [
      {
        "name": "test_1",
        "input": "1 2 3",
        "expected_output": "6",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      }
    ]
  }'
```

### gRPC API

```bash
grpcurl -plaintext \
  -proto proto/isobox.proto \
  -H "authorization: your-api-key" \
  -d '{"language": "python", "code": "print(\"Hello, World!\")"}' \
  localhost:9000 isobox.CodeExecutionService/ExecuteCode
```

## 🗣️ Supported Languages

| Language    | Docker Image       | Compilation | Features                          |
| ----------- | ------------------ | ----------- | --------------------------------- |
| **Python**  | `python:3.11-slim` | No          | ✅ Standard library, pip packages |
| **Node.js** | `node:18-slim`     | No          | ✅ npm packages, async/await      |
| **Java**    | `openjdk:17-slim`  | Yes         | ✅ Collections, Streams, Time API |
| **Go**      | `golang:1.21`      | Yes         | ✅ Goroutines, channels, modules  |
| **Rust**    | `rust:1.70-slim`   | Yes         | ✅ Cargo, async, traits           |
| **C++**     | `gcc:latest`       | Yes         | ✅ STL, modern C++ features       |
| **C**       | `gcc:latest`       | Yes         | ✅ Standard library               |
| **PHP**     | `php:8.2-cli`      | No          | ✅ Composer, modern PHP           |
| **Ruby**    | `ruby:3.2-slim`    | No          | ✅ Gems, modern Ruby              |
| **Bash**    | `ubuntu:22.04`     | No          | ✅ Shell scripting                |

## 🧪 Testing

### Quick Test

```bash
# Test all languages with API key authentication
./quick-test.sh

# Test with Firebase authentication
./test-firebase-auth.sh

# Test with real Firebase project
./test-easyloops-real.sh
```

### Comprehensive Testing

```bash
# Run all tests
./test_runner.sh

# Run E2E tests
./e2e_tests.sh

# Run specific language tests
cargo test test_python_execution
cargo test test_java_execution
cargo test test_go_execution
```

### Test Case Examples

```bash
# Run test case demos
./examples/test_cases_demo.sh

# Test with different input formats
./examples/demo.sh
```

## 🏗️ Architecture

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
│  │   Port 8000 │  │ Port 9000   │  │   (Multi-Strategy)      │  │
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

## 🔒 Security Features

- **Container Isolation** - Each execution in separate Docker container
- **Resource Limits** - CPU, memory, process, and file descriptor limits
- **Network Isolation** - Containers run with `--network none`
- **Privilege Dropping** - Containers run with dropped capabilities
- **Multi-Authentication** - Support for various authentication methods
- **Code Deduplication** - Prevents duplicate code execution
- **Timeout Protection** - Configurable execution timeouts

## 🚀 Performance

- **Fast Execution** - Optimized Docker container startup
- **Caching** - In-memory and Redis caching support
- **Resource Efficiency** - Minimal resource overhead
- **Scalability** - Horizontal scaling support

## 🤝 Contributing

We welcome contributions! Please see our [Development Guide](DEVELOPMENT.md) for details on:

- Setting up the development environment
- Running tests
- Code style guidelines
- Pull request process
- Issue reporting

### Quick Development Setup

```bash
# Clone and setup
git clone <repository-url>
cd isobox

# Install dependencies
cargo build

# Run tests
cargo test

# Start development server
cargo run
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: Check the [docs](docs/) directory
- **Issues**: Report bugs and feature requests on GitHub
- **Discussions**: Join our community discussions
- **Examples**: See the [examples/](examples/) directory for usage examples

## 🙏 Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and [Actix-Web](https://actix.rs/)
- Container isolation powered by [Docker](https://www.docker.com/)
- Authentication support for multiple providers
- Community contributors and maintainers

---

**Made with ❤️ for the developer community**
