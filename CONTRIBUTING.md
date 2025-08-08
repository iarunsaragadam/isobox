# Contributing to IsoBox

Thank you for your interest in contributing to IsoBox! This document provides guidelines and information for contributors.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Development Setup](#development-setup)
3. [Code Style Guidelines](#code-style-guidelines)
4. [Testing Guidelines](#testing-guidelines)
5. [Pull Request Process](#pull-request-process)
6. [Issue Reporting](#issue-reporting)
7. [Feature Requests](#feature-requests)
8. [Documentation](#documentation)
9. [Community Guidelines](#community-guidelines)
10. [Release Process](#release-process)

## Getting Started

### Before You Start

1. **Read the Documentation**: Familiarize yourself with the [README](README.md), [API documentation](API.md), and other docs
2. **Check Existing Issues**: Look for existing issues or discussions about your contribution
3. **Join the Community**: Join our [Discussions](https://github.com/isobox/isobox/discussions) or [Discord](https://discord.gg/isobox)

### Types of Contributions

We welcome various types of contributions:

- **Bug Fixes**: Fix bugs and issues
- **Feature Development**: Add new features and capabilities
- **Documentation**: Improve documentation and examples
- **Testing**: Add tests and improve test coverage
- **Performance**: Optimize performance and resource usage
- **Security**: Improve security features and practices
- **Examples**: Add example code and use cases
- **Translations**: Translate documentation to other languages

### Contribution Areas

- **Core Engine**: Code execution, container management, resource limits
- **Authentication**: API keys, JWT, OAuth2, mTLS authentication
- **Language Support**: Add new programming languages
- **API**: REST API and gRPC endpoints
- **Security**: Security features and hardening
- **Performance**: Optimization and caching
- **Documentation**: Guides, examples, and API docs
- **Testing**: Unit tests, integration tests, E2E tests

## Development Setup

### Prerequisites

- **Rust 1.70+** - [Install Rust](https://rustup.rs/)
- **Docker** - [Install Docker](https://docs.docker.com/get-docker/)
- **Git** - Version control
- **IDE** - VS Code, IntelliJ IDEA, or your preferred editor

### Quick Setup

```bash
# Clone the repository
git clone https://github.com/isobox/isobox.git
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
- `GitLens` - Git integration
- `Docker` - Docker support

#### IntelliJ IDEA / CLion

- Install Rust plugin
- Configure run configurations
- Set up debugging
- Enable code formatting

### Project Structure

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

Follow the official Rust style guide and use automated tools:

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

````rust
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
///
/// # Examples
///
/// ```
/// use isobox::executor::execute_code;
///
/// let result = execute_code("python", "print('Hello')", None).await?;
/// println!("Output: {}", result.stdout);
/// ```
pub async fn execute_code(
    language: &str,
    code: &str,
    test_cases: Option<Vec<TestCase>>,
) -> Result<ExecutionResult, Box<dyn std::error::Error>> {
    // Validate inputs
    validate_language(language)?;
    validate_code(code)?;

    // Execute code
    let result = execute_in_container(language, code, test_cases).await?;

    // Log execution
    log::info!("Code executed successfully: language={}, time={:?}",
               language, result.time_taken);

    Ok(result)
}
````

### Error Handling

Use proper error types and propagation:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Language not supported: {0}")]
    UnsupportedLanguage(String),

    #[error("Code too large: {0} bytes (max: {1})")]
    CodeTooLarge(usize, usize),

    #[error("Docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("Timeout after {0} seconds")]
    Timeout(u64),

    #[error("Memory limit exceeded: {0} MB")]
    MemoryLimitExceeded(u64),
}

// Use in functions
pub async fn execute_code(code: &str) -> Result<String, ExecutionError> {
    if code.len() > MAX_CODE_SIZE {
        return Err(ExecutionError::CodeTooLarge(code.len(), MAX_CODE_SIZE));
    }

    // Implementation
    Ok(result)
}
```

### Logging

Use appropriate log levels and structured logging:

```rust
use log::{debug, info, warn, error};

// Debug information
debug!("Processing request: language={}, code_length={}", language, code.len());

// General information
info!("Code execution completed: language={}, time={:?}", language, time_taken);

// Warnings
warn!("Resource usage high: memory={}MB, cpu={}%", memory_usage, cpu_usage);

// Errors
error!("Code execution failed: language={}, error={}", language, error);

// Structured logging with context
log::info!(target: "execution",
           language = %language,
           time_taken = ?time_taken,
           memory_used = memory_used,
           "Code execution completed");
```

## Testing Guidelines

### Test Categories

1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test API endpoints and workflows
3. **E2E Tests** - Test complete system functionality
4. **Performance Tests** - Test system performance and limits
5. **Security Tests** - Test security features and vulnerabilities

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

# Run tests in parallel
cargo test --jobs 4
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

        let result = result.unwrap();
        assert_eq!(result.stdout, "Hello\n");
        assert_eq!(result.exit_code, 0);
    }

    #[tokio::test]
    async fn test_async_function() {
        let result = async_function().await;
        assert_eq!(result, expected_value);
    }

    #[test]
    fn test_error_handling() {
        let result = execute_code("invalid_language", "print('Hello')", None);
        assert!(result.is_err());

        match result.unwrap_err() {
            ExecutionError::UnsupportedLanguage(lang) => {
                assert_eq!(lang, "invalid_language");
            }
            _ => panic!("Expected UnsupportedLanguage error"),
        }
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

    let result: ExecutionResult = response.json().await.unwrap();
    assert_eq!(result.stdout, "Hello\n");
    assert_eq!(result.exit_code, 0);
}
```

#### Performance Test Example

```rust
#[tokio::test]
async fn test_performance() {
    let start = std::time::Instant::now();

    for _ in 0..100 {
        let result = execute_code("python", "print('test')", None).await;
        assert!(result.is_ok());
    }

    let duration = start.elapsed();
    assert!(duration.as_secs() < 30, "Performance test took too long: {:?}", duration);
}
```

### Test Data

Create test data in the `tests/` directory:

```bash
tests/
├── data/
│   ├── python_simple.py
│   ├── java_hello.java
│   ├── go_main.go
│   └── test_cases.json
├── fixtures/
│   ├── auth_tokens.json
│   ├── docker_configs.json
│   └── expected_outputs.json
└── integration/
    ├── api_tests.rs
    ├── grpc_tests.rs
    └── auth_tests.rs
```

### Test Coverage

Maintain high test coverage:

```bash
# Check test coverage
cargo tarpaulin --out Html

# View coverage report
open tarpaulin-report.html
```

**Coverage Targets**:

- **Unit Tests**: 90%+ coverage
- **Integration Tests**: 80%+ coverage
- **Critical Paths**: 100% coverage

## Pull Request Process

### Before Submitting

1. **Fork the repository**
2. **Create a feature branch**:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. **Make your changes**
4. **Add tests for new functionality**
5. **Update documentation**
6. **Ensure all tests pass**:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
7. **Test manually**:
   ```bash
   cargo run
   # Test your changes manually
   ```

### Pull Request Guidelines

#### Title and Description

**Title Format**: `type(scope): description`

Examples:

- `feat(auth): add OAuth2 authentication support`
- `fix(executor): resolve timeout issue in Python execution`
- `docs(api): update API documentation with new endpoints`
- `test(integration): add comprehensive E2E tests`
- `perf(cache): optimize code deduplication performance`

**Description Template**:

```markdown
## Description

Brief description of the changes

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] Performance impact assessed

## Checklist

- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] Tests added/updated
- [ ] No breaking changes (or breaking changes documented)

## Related Issues

Closes #123
Fixes #456
```

#### Code Review Checklist

**For Contributors**:

- [ ] Code follows style guidelines
- [ ] Tests are included and passing
- [ ] Documentation is updated
- [ ] No security vulnerabilities
- [ ] Performance impact is considered
- [ ] Error handling is proper
- [ ] Logging is appropriate
- [ ] No breaking changes (or breaking changes documented)

**For Reviewers**:

- [ ] Code is readable and well-structured
- [ ] Tests are comprehensive
- [ ] Documentation is clear and complete
- [ ] Security considerations are addressed
- [ ] Performance impact is acceptable
- [ ] Error handling is robust
- [ ] Logging is appropriate
- [ ] Breaking changes are properly documented

### Review Process

1. **Automated Checks**: CI/CD pipeline runs tests and checks
2. **Code Review**: At least one maintainer reviews the PR
3. **Testing**: Manual testing may be required
4. **Approval**: PR is approved by maintainers
5. **Merge**: PR is merged to main branch

### Commit Message Format

Use conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

**Types**:

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples**:

```
feat(auth): add OAuth2 authentication support

- Add OAuth2 authentication strategy
- Support Google, GitHub, and Firebase providers
- Add comprehensive tests and documentation

Closes #123
```

```
fix(executor): resolve timeout issue in Python execution

The timeout was not being properly enforced for Python code execution.
This fix ensures that timeouts are correctly applied and enforced.

Fixes #456
```

## Issue Reporting

### Before Reporting

1. **Search existing issues** for similar problems
2. **Check documentation** for solutions
3. **Try the latest version** to see if the issue is fixed
4. **Reproduce the issue** consistently

### Issue Template

**Bug Report Template**:

```markdown
## Bug Description

Clear and concise description of the bug

## Steps to Reproduce

1. Go to '...'
2. Click on '...'
3. Scroll down to '...'
4. See error

## Expected Behavior

What you expected to happen

## Actual Behavior

What actually happened

## Environment

- OS: [e.g. Ubuntu 22.04]
- Docker Version: [e.g. 24.0.0]
- IsoBox Version: [e.g. 1.0.0]
- Rust Version: [e.g. 1.70.0]

## Additional Context

- Screenshots if applicable
- Logs and error messages
- Configuration files
- Any other relevant information
```

**Feature Request Template**:

```markdown
## Feature Description

Clear and concise description of the feature

## Problem Statement

What problem does this feature solve?

## Proposed Solution

How should this feature work?

## Alternative Solutions

Any alternative solutions you've considered

## Additional Context

- Use cases
- Examples
- Related features
```

### Issue Labels

We use the following labels:

- `bug`: Something isn't working
- `enhancement`: New feature or request
- `documentation`: Improvements or additions to documentation
- `good first issue`: Good for newcomers
- `help wanted`: Extra attention is needed
- `priority: high`: High priority issue
- `priority: medium`: Medium priority issue
- `priority: low`: Low priority issue

## Feature Requests

### Before Requesting

1. **Check existing features** to see if it's already implemented
2. **Search issues** for similar requests
3. **Consider alternatives** that might already exist
4. **Think about implementation** and complexity

### Feature Request Guidelines

1. **Clear Description**: Explain what you want and why
2. **Use Cases**: Provide specific use cases
3. **Implementation Ideas**: Suggest how it might be implemented
4. **Priority**: Indicate the priority level
5. **Alternatives**: Consider if there are simpler solutions

### Feature Development Process

1. **Discussion**: Discuss the feature in an issue
2. **Design**: Design the feature and get approval
3. **Implementation**: Implement the feature
4. **Testing**: Test thoroughly
5. **Documentation**: Update documentation
6. **Review**: Code review and approval
7. **Merge**: Merge to main branch

## Documentation

### Documentation Standards

1. **Clear and Concise**: Write clear, easy-to-understand documentation
2. **Examples**: Include practical examples
3. **Complete**: Cover all aspects of the feature
4. **Up-to-date**: Keep documentation current
5. **Searchable**: Use clear headings and structure

### Documentation Types

1. **API Documentation**: Complete API reference
2. **User Guides**: How-to guides and tutorials
3. **Developer Guides**: Development and contribution guides
4. **Configuration**: Configuration options and examples
5. **Examples**: Code examples and use cases

### Documentation Guidelines

````markdown
# Feature Name

Brief description of the feature.

## Overview

Detailed explanation of the feature and its purpose.

## Usage

### Basic Usage

```bash
# Example command
command --option value
```
````

### Advanced Usage

```rust
// Example code
let result = feature_function();
println!("Result: {}", result);
```

## Configuration

| Option    | Description | Default   | Required |
| --------- | ----------- | --------- | -------- |
| `option1` | Description | `default` | No       |
| `option2` | Description | `default` | Yes      |

## Examples

### Example 1: Basic Example

Description of the example.

```bash
# Command
example_command
```

### Example 2: Advanced Example

Description of the advanced example.

```rust
// Code
advanced_example();
```

## Troubleshooting

Common issues and solutions.

## Related

- [Related Feature 1](link)
- [Related Feature 2](link)

````

## Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please:

1. **Be Respectful**: Treat everyone with respect
2. **Be Inclusive**: Welcome people of all backgrounds
3. **Be Collaborative**: Work together constructively
4. **Be Professional**: Maintain professional behavior
5. **Be Helpful**: Help others learn and grow

### Communication

1. **Issues**: Use GitHub issues for bugs and feature requests
2. **Discussions**: Use GitHub Discussions for general questions
3. **Discord**: Join our Discord for real-time chat
4. **Email**: Use email for sensitive or private matters

### Getting Help

1. **Documentation**: Check the documentation first
2. **Issues**: Search existing issues
3. **Discussions**: Ask in GitHub Discussions
4. **Discord**: Ask in Discord for quick help
5. **Email**: Contact maintainers for private issues

## Release Process

### Version Management

We use semantic versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Breaking changes
- **MINOR**: New features, backward compatible
- **PATCH**: Bug fixes, backward compatible

### Release Steps

1. **Update Version**: Update version in `Cargo.toml`
2. **Update Changelog**: Add new version section to `CHANGELOG.md`
3. **Create Release Branch**:
   ```bash
   git checkout -b release/v1.2.0
````

4. **Run Full Test Suite**:
   ```bash
   cargo test
   ./test_runner.sh
   ./e2e_tests.sh
   ```
5. **Build and Test Docker Image**:
   ```bash
   docker build -t isobox:1.2.0 .
   docker run --rm isobox:1.2.0
   ```
6. **Create Release Tag**:
   ```bash
   git tag -a v1.2.0 -m "Release v1.2.0"
   git push origin v1.2.0
   ```
7. **Merge to Main**: Merge release branch to main
8. **Update Documentation**: Update documentation if needed
9. **Announce Release**: Create GitHub release and announce

### Release Checklist

- [ ] Version updated in `Cargo.toml`
- [ ] `CHANGELOG.md` updated
- [ ] All tests passing
- [ ] Docker image builds successfully
- [ ] Documentation updated
- [ ] Release notes prepared
- [ ] Security review completed
- [ ] Performance benchmarks run
- [ ] Breaking changes documented
- [ ] Migration guide updated (if needed)

### Release Notes

Release notes should include:

1. **New Features**: What's new
2. **Bug Fixes**: What's fixed
3. **Breaking Changes**: What's changed
4. **Performance**: Performance improvements
5. **Security**: Security updates
6. **Documentation**: Documentation updates
7. **Migration Guide**: How to upgrade

---

Thank you for contributing to IsoBox! Your contributions help make this project better for everyone.

For more information, see the [main README](README.md) and other documentation files.
