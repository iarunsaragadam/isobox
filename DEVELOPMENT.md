# Development Guide

This guide provides comprehensive information for developers who want to contribute to IsoBox, set up the development environment, and understand the codebase.

## Table of Contents

1. [Development Setup](#development-setup)
2. [Project Structure](#project-structure)
3. [Code Style Guidelines](#code-style-guidelines)
4. [Testing](#testing)
5. [Authentication Development](#authentication-development)
6. [Adding New Languages](#adding-new-languages)
7. [API Development](#api-development)
8. [Docker Development](#docker-development)
9. [Performance Optimization](#performance-optimization)
10. [Debugging](#debugging)
11. [Contributing Guidelines](#contributing-guidelines)
12. [Release Process](#release-process)

## Development Setup

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Docker** - [Install Docker](https://docs.docker.com/get-docker/)
- **grpcurl** - For testing gRPC endpoints
- **Git** - Version control

### Quick Setup

```bash
# Clone the repository
git clone <repository-url>
cd isobox

# Install dependencies
cargo build

# Run tests
cargo test

# Start development server
cargo run
```

### Environment Configuration

Create a `.env` file for development:

```bash
# Authentication
AUTH_TYPE=apikey
API_KEYS=dev-key-123,test-key-456
API_KEY_HEADER=X-API-Key

# Server Configuration
REST_PORT=8000
GRPC_PORT=9000
RUST_LOG=debug

# Development Features
DEDUP_ENABLED=true
DEDUP_CACHE_TTL=3600
DEDUP_CACHE_TYPE=memory
```

### IDE Setup

#### VS Code

Install recommended extensions:

- `rust-analyzer` - Rust language support
- `crates` - Cargo.toml dependency management
- `CodeLLDB` - Debugging support

#### IntelliJ IDEA / CLion

- Install Rust plugin
- Configure run configurations for development

## Project Structure

```
isobox/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── executor.rs          # Code execution logic
│   ├── grpc.rs              # gRPC service implementation
│   ├── generated.rs         # Generated protobuf code
│   └── auth/                # Authentication modules
│       ├── mod.rs           # Authentication module exports
│       ├── config.rs        # Authentication configuration
│       ├── middleware.rs    # Authentication middleware
│       ├── cache.rs         # Authentication caching
│       ├── dedup.rs         # Code deduplication
│       └── strategies/      # Authentication strategies
│           ├── mod.rs       # Strategy module exports
│           ├── apikey.rs    # API key authentication
│           ├── jwt.rs       # JWT authentication
│           ├── oauth2.rs    # OAuth2 authentication
│           ├── mtls.rs      # mTLS authentication
│           └── none.rs      # No authentication
├── proto/
│   └── isobox.proto         # gRPC service definitions
├── examples/                # Example requests and demos
├── tests/                   # Integration tests
├── docs/                    # Documentation
├── scripts/                 # Build and deployment scripts
├── Dockerfile               # Container definition
├── docker-compose.yml       # Development environment
├── Cargo.toml               # Rust dependencies
└── README.md                # Project documentation
```

## Code Style Guidelines

### Rust Code Style

Follow the official Rust style guide:

```bash
# Format code
cargo fmt

# Check code style
cargo clippy

# Run linter with warnings
cargo clippy -- -W clippy::all
```

### Code Organization

1. **Modules**: Group related functionality in modules
2. **Error Handling**: Use proper error types and propagation
3. **Documentation**: Document all public APIs with doc comments
4. **Tests**: Write unit tests for all functions
5. **Logging**: Use appropriate log levels (debug, info, warn, error)

### Example Code Style

```rust
/// Executes code in the specified language with given parameters.
///
/// # Arguments
///
/// * `language` - The programming language to use
/// * `code` - The source code to execute
/// * `test_cases` - Optional test cases to run
///
/// # Returns
///
/// A `Result` containing the execution result or an error.
pub async fn execute_code(
    language: &str,
    code: &str,
    test_cases: Option<Vec<TestCase>>,
) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
    // Implementation
}
```

## Testing

### Test Categories

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test API endpoints and workflows
3. **E2E Tests** - Test complete system functionality
4. **Performance Tests** - Test system performance and limits

### Running Tests

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration

# Run specific test
cargo test test_python_execution

# Run tests with output
cargo test -- --nocapture

# Run tests with coverage
cargo tarpaulin
```

### Test Examples

#### Unit Test Example

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_python_execution() {
        let result = execute_code("python", "print('Hello')", None);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert_eq!(result, expected_value);
    }
}
```

#### Integration Test Example

```rust
#[tokio::test]
async fn test_api_endpoint() {
    let app = create_test_app().await;
    let client = TestClient::new(app);

    let response = client
        .post("/api/v1/execute")
        .header("X-API-Key", "test-key")
        .json(&json!({
            "language": "python",
            "code": "print('Hello')"
        }))
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}
```

### Test Data

Create test data in the `tests/` directory:

```bash
tests/
├── data/
│   ├── python_simple.py
│   ├── java_hello.java
│   └── test_cases.json
├── fixtures/
│   ├── auth_tokens.json
│   └── docker_configs.json
└── integration/
    ├── api_tests.rs
    └── grpc_tests.rs
```

## Authentication Development

### Adding New Authentication Strategy

1. **Create Strategy Module**

```rust
// src/auth/strategies/new_auth.rs
use async_trait::async_trait;
use crate::auth::AuthError;

pub struct NewAuthStrategy {
    // Configuration fields
}

#[async_trait]
impl AuthStrategy for NewAuthStrategy {
    async fn authenticate(&self, request: &HttpRequest) -> Result<(), AuthError> {
        // Implementation
    }
}
```

2. **Register Strategy**

```rust
// src/auth/strategies/mod.rs
pub mod new_auth;

pub fn create_strategy(auth_type: &str) -> Box<dyn AuthStrategy> {
    match auth_type {
        "new_auth" => Box::new(NewAuthStrategy::new()),
        // ... other strategies
    }
}
```

3. **Add Configuration**

```rust
// src/auth/config.rs
pub struct AuthConfig {
    pub auth_type: String,
    pub new_auth_config: Option<NewAuthConfig>,
    // ... other configs
}
```

### Testing Authentication

```bash
# Test API key authentication
cargo test test_api_key_auth

# Test JWT authentication
cargo test test_jwt_auth

# Test OAuth2 authentication
cargo test test_oauth2_auth

# Test authentication middleware
cargo test test_auth_middleware
```

## Adding New Languages

### 1. Define Language Configuration

```rust
// src/executor.rs
pub struct LanguageConfig {
    pub name: String,
    pub docker_image: String,
    pub compile_command: Option<String>,
    pub run_command: String,
    pub file_extension: String,
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
}

pub fn get_language_config(language: &str) -> Option<LanguageConfig> {
    match language {
        "newlang" => Some(LanguageConfig {
            name: "newlang".to_string(),
            docker_image: "newlang:latest".to_string(),
            compile_command: Some("newlangc".to_string()),
            run_command: "newlang".to_string(),
            file_extension: ".nl".to_string(),
            timeout_seconds: 30,
            memory_limit_mb: 512,
        }),
        // ... other languages
    }
}
```

### 2. Create Docker Image

```dockerfile
# Dockerfile.newlang
FROM ubuntu:22.04

# Install newlang
RUN apt-get update && apt-get install -y newlang

# Set working directory
WORKDIR /workspace

# Create non-root user
RUN useradd -m -u 1000 coder
USER coder

# Default command
CMD ["newlang"]
```

### 3. Add Tests

```rust
#[test]
fn test_newlang_execution() {
    let result = execute_code("newlang", "print('Hello')", None);
    assert!(result.is_ok());
}

#[test]
fn test_newlang_with_test_cases() {
    let test_cases = vec![
        TestCase {
            name: "test_1".to_string(),
            input: "test".to_string(),
            expected_output: "test".to_string(),
            timeout_seconds: 5,
            memory_limit_mb: 128,
        }
    ];

    let result = execute_code("newlang", "echo $1", Some(test_cases));
    assert!(result.is_ok());
}
```

### 4. Update Documentation

Update the following files:

- `README.md` - Add to supported languages table
- `API.md` - Add language examples
- `CONFIGURATION.md` - Add language-specific configuration

## API Development

### Adding New Endpoints

1. **Define Route**

```rust
// src/main.rs
async fn new_endpoint(
    req: HttpRequest,
    payload: web::Json<NewRequest>,
) -> Result<HttpResponse, actix_web::Error> {
    // Implementation
}

// Register route
.service(
    web::resource("/api/v1/new-endpoint")
        .route(web::post().to(new_endpoint))
)
```

2. **Add Request/Response Types**

```rust
#[derive(Deserialize)]
pub struct NewRequest {
    pub field1: String,
    pub field2: Option<i32>,
}

#[derive(Serialize)]
pub struct NewResponse {
    pub result: String,
    pub status: String,
}
```

3. **Add Tests**

```rust
#[tokio::test]
async fn test_new_endpoint() {
    let app = create_test_app().await;
    let client = TestClient::new(app);

    let response = client
        .post("/api/v1/new-endpoint")
        .json(&json!({
            "field1": "test",
            "field2": 123
        }))
        .send()
        .await;

    assert_eq!(response.status(), StatusCode::OK);
}
```

### gRPC Development

1. **Update Protocol Buffer**

```protobuf
// proto/isobox.proto
service CodeExecutionService {
    rpc ExecuteCode(ExecuteCodeRequest) returns (ExecuteCodeResponse);
    rpc NewMethod(NewRequest) returns (NewResponse);
}

message NewRequest {
    string field1 = 1;
    int32 field2 = 2;
}

message NewResponse {
    string result = 1;
    string status = 2;
}
```

2. **Generate Code**

```bash
# Generate Rust code from protobuf
cargo build
```

3. **Implement Service**

```rust
// src/grpc.rs
impl CodeExecutionService for IsoBoxService {
    async fn new_method(
        &self,
        request: Request<NewRequest>,
    ) -> Result<Response<NewResponse>, Status> {
        // Implementation
    }
}
```

## Docker Development

### Development Dockerfile

```dockerfile
# Dockerfile.dev
FROM rust:1.70 as builder

WORKDIR /app
COPY . .

# Install dependencies
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/isobox /usr/local/bin/isobox

EXPOSE 8000 9000

CMD ["isobox"]
```

### Docker Compose for Development

```yaml
# docker-compose.dev.yml
version: "3.8"
services:
  isobox-dev:
    build:
      context: .
      dockerfile: Dockerfile.dev
    ports:
      - "8000:8000"
      - "9000:9000"
    volumes:
      - .:/app
      - /var/run/docker.sock:/var/run/docker.sock
    environment:
      - RUST_LOG=debug
      - AUTH_TYPE=apikey
      - API_KEYS=dev-key
    command: cargo run
```

### Multi-stage Builds

```dockerfile
# Optimized production build
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/isobox /usr/local/bin/
EXPOSE 8000 9000
CMD ["isobox"]
```

## Performance Optimization

### Profiling

```bash
# Install profiling tools
cargo install flamegraph

# Generate flamegraph
cargo flamegraph

# Profile with perf
perf record --call-graph=dwarf cargo run
perf report
```

### Benchmarking

```rust
// benches/execution_bench.rs
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_execution(c: &mut Criterion) {
    c.bench_function("python_execution", |b| {
        b.iter(|| {
            // Benchmark code
        })
    });
}

criterion_group!(benches, benchmark_execution);
criterion_main!(benches);
```

### Memory Optimization

1. **Use appropriate data structures**
2. **Implement proper error handling**
3. **Use async/await for I/O operations**
4. **Implement connection pooling**
5. **Use caching strategies**

## Debugging

### Logging

```rust
use log::{debug, info, warn, error};

// Configure logging
env_logger::init();

// Use appropriate log levels
debug!("Debug information");
info!("General information");
warn!("Warning message");
error!("Error message");
```

### Debugging Tools

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with specific module logging
RUST_LOG=isobox::executor=debug cargo run

# Attach debugger
rust-gdb target/debug/isobox

# Use println! for quick debugging
println!("Debug: {:?}", variable);
```

### Error Handling

```rust
// Use proper error types
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),
    #[error("Docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),
    #[error("Timeout after {0} seconds")]
    Timeout(u64),
}

// Propagate errors properly
pub async fn execute_code(code: &str) -> Result<String, ExecutionError> {
    // Implementation with proper error handling
}
```

## Contributing Guidelines

### Pull Request Process

1. **Fork the repository**
2. **Create a feature branch**
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Add tests for new functionality**
5. **Ensure all tests pass**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
6. **Update documentation**
7. **Submit a pull request**

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

Examples:

- `feat(auth): add OAuth2 authentication support`
- `fix(executor): resolve timeout issue in Python execution`
- `docs(api): update API documentation with new endpoints`
- `test(integration): add comprehensive E2E tests`

### Code Review Checklist

- [ ] Code follows style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] No security vulnerabilities
- [ ] Performance impact is considered
- [ ] Error handling is proper
- [ ] Logging is appropriate

### Issue Reporting

When reporting issues, include:

1. **Environment details**

   - OS and version
   - Rust version
   - Docker version
   - IsoBox version

2. **Steps to reproduce**

   - Clear, step-by-step instructions
   - Sample code if applicable

3. **Expected vs actual behavior**

   - What you expected to happen
   - What actually happened

4. **Logs and error messages**
   - Relevant log output
   - Error messages
   - Stack traces

## Release Process

### Version Management

Use semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Release Steps

1. **Update version in Cargo.toml**
2. **Update CHANGELOG.md**
3. **Create release branch**
   ```bash
   git checkout -b release/v1.2.0
   ```
4. **Run full test suite**
   ```bash
   cargo test
   ./test_runner.sh
   ./e2e_tests.sh
   ```
5. **Build and test Docker image**
   ```bash
   docker build -t isobox:1.2.0 .
   docker run --rm isobox:1.2.0
   ```
6. **Create release tag**
   ```bash
   git tag -a v1.2.0 -m "Release v1.2.0"
   git push origin v1.2.0
   ```
7. **Merge to main**
8. **Update documentation**
9. **Announce release**

### Release Checklist

- [ ] Version updated in Cargo.toml
- [ ] CHANGELOG.md updated
- [ ] All tests passing
- [ ] Docker image builds successfully
- [ ] Documentation updated
- [ ] Release notes prepared
- [ ] Security review completed
- [ ] Performance benchmarks run

---

For more information, see the [main README](README.md) and other documentation files.
