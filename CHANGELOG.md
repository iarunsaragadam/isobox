# Changelog

All notable changes to IsoBox will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Comprehensive documentation structure
- Container registry documentation
- Development guide with contribution guidelines
- Multi-authentication support documentation
- Firebase OAuth2 integration examples
- Real Firebase project integration (easyloops)
- Test scripts for all authentication methods
- Performance optimization guidelines

### Changed

- Updated README.md with modern design and comprehensive information
- Enhanced API documentation with authentication examples
- Improved configuration documentation
- Updated test case documentation

## [1.0.0] - 2025-01-XX

### Added

- **Multi-Authentication Support**

  - API Key Authentication
  - JWT Authentication (Google, Auth0, custom providers)
  - OAuth2 Authentication (Google, Meta, GitHub, Firebase, AWS Cognito)
  - mTLS Authentication
  - No Authentication option for development

- **Code Execution Features**

  - Support for 10+ programming languages
  - Container isolation for each execution
  - Resource limits (CPU, memory, processes)
  - Timeout protection
  - Test case execution support

- **Test Case Functionality**

  - Inline test cases
  - File-based test cases
  - URL-based test cases
  - Individual test case limits
  - Test result analysis

- **Advanced Features**

  - Code deduplication with caching
  - Performance monitoring
  - Health check endpoints
  - Dual API support (HTTP REST + gRPC)

- **Security Features**
  - Container isolation
  - Network isolation
  - Privilege dropping
  - Resource limits
  - Authentication middleware

### Supported Languages

- **Python** (python:3.11-slim) - Standard library, pip packages
- **Node.js** (node:18-slim) - npm packages, async/await
- **Java** (openjdk:17-slim) - Collections, Streams, Time API
- **Go** (golang:1.21) - Goroutines, channels, modules
- **Rust** (rust:1.70-slim) - Cargo, async, traits
- **C++** (gcc:latest) - STL, modern C++ features
- **C** (gcc:latest) - Standard library
- **PHP** (php:8.2-cli) - Composer, modern PHP
- **Ruby** (ruby:3.2-slim) - Gems, modern Ruby
- **Bash** (ubuntu:22.04) - Shell scripting

### API Endpoints

- `POST /api/v1/execute` - Basic code execution
- `POST /api/v1/execute/test-cases` - Execute with inline test cases
- `POST /api/v1/execute/test-files` - Execute with file-based test cases
- `POST /api/v1/execute/test-urls` - Execute with URL-based test cases
- `GET /health` - Health check endpoint

### gRPC Services

- `CodeExecutionService.ExecuteCode` - Execute code
- `CodeExecutionService.HealthCheck` - Health check
- `CodeExecutionService.GetSupportedLanguages` - Get supported languages

### Configuration

- Environment-based configuration
- Multiple authentication strategies
- Resource limit configuration
- Caching configuration
- Logging configuration

### Docker Support

- Multi-stage Docker builds
- Docker Compose configurations
- Production-ready Docker images
- Platform-specific builds

### Testing

- Comprehensive unit tests
- Integration tests
- E2E tests
- Performance tests
- Authentication tests

### Documentation

- API documentation with examples
- Authentication guide
- Configuration guide
- Test cases guide
- Development guide
- Container registry documentation

## [0.9.0] - 2024-12-XX

### Added

- Initial authentication system
- Basic API key authentication
- JWT authentication support
- OAuth2 authentication framework
- Code execution engine
- Docker container isolation
- Resource limit enforcement
- Basic test case support

### Changed

- Refactored authentication middleware
- Improved error handling
- Enhanced logging system
- Updated Docker configuration

### Fixed

- Authentication token validation issues
- Docker container cleanup problems
- Resource limit enforcement bugs
- Test case execution errors

## [0.8.0] - 2024-11-XX

### Added

- gRPC service implementation
- Protocol buffer definitions
- Dual API support (HTTP + gRPC)
- Health check endpoints
- Basic monitoring

### Changed

- Updated service architecture
- Improved API response format
- Enhanced error messages
- Better logging structure

### Fixed

- gRPC connection issues
- Service discovery problems
- Health check endpoint bugs

## [0.7.0] - 2024-10-XX

### Added

- Test case execution framework
- Multiple test input formats
- Test result analysis
- Individual test limits
- Test case validation

### Changed

- Updated execution engine
- Improved test case handling
- Enhanced result formatting
- Better error reporting

### Fixed

- Test case execution bugs
- Input validation issues
- Result parsing problems

## [0.6.0] - 2024-09-XX

### Added

- Code deduplication system
- Caching support (in-memory + Redis)
- Performance optimization
- Resource usage tracking
- Execution time monitoring

### Changed

- Improved caching strategy
- Enhanced performance monitoring
- Updated resource management
- Better memory handling

### Fixed

- Memory leak issues
- Cache invalidation problems
- Performance bottlenecks

## [0.5.0] - 2024-08-XX

### Added

- Multi-language support
- Language-specific configurations
- Docker image management
- Compilation support for compiled languages
- Language validation

### Changed

- Updated language registry
- Improved Docker image handling
- Enhanced compilation process
- Better language detection

### Fixed

- Language execution issues
- Docker image problems
- Compilation errors

## [0.4.0] - 2024-07-XX

### Added

- Docker container isolation
- Security sandboxing
- Resource limit enforcement
- Network isolation
- Privilege dropping

### Changed

- Updated security model
- Improved container management
- Enhanced resource limits
- Better isolation strategy

### Fixed

- Security vulnerabilities
- Container escape issues
- Resource limit bugs

## [0.3.0] - 2024-06-XX

### Added

- HTTP REST API
- JSON request/response format
- Error handling
- Request validation
- Response formatting

### Changed

- Updated API structure
- Improved error messages
- Enhanced request validation
- Better response format

### Fixed

- API endpoint bugs
- JSON parsing issues
- Validation errors

## [0.2.0] - 2024-05-XX

### Added

- Basic code execution engine
- Docker integration
- Simple API structure
- Basic error handling
- Logging system

### Changed

- Updated execution engine
- Improved Docker integration
- Enhanced error handling
- Better logging

### Fixed

- Execution engine bugs
- Docker integration issues
- Error handling problems

## [0.1.0] - 2024-04-XX

### Added

- Initial project structure
- Basic Rust application
- Docker support
- Basic configuration
- Development environment

### Changed

- Project initialization
- Basic setup
- Development workflow

---

## Migration Guides

### Upgrading from 0.x to 1.0.0

#### Breaking Changes

- Authentication is now required by default
- API response format has changed
- Environment variable names have been updated
- Docker image structure has changed

#### Migration Steps

1. Update environment variables:

   ```bash
   # Old
   API_KEYS=key1,key2

   # New
   AUTH_TYPE=apikey
   API_KEYS=key1,key2
   API_KEY_HEADER=X-API-Key
   ```

2. Update Docker run commands:

   ```bash
   # Old
   docker run -p 8000:8000 -e API_KEYS=key isobox

   # New
   docker run -p 8000:8000 -p 9000:9000 \
     -e AUTH_TYPE=apikey \
     -e API_KEYS=key \
     -e API_KEY_HEADER=X-API-Key \
     isobox/isobox:latest
   ```

3. Update API requests:

   ```bash
   # Old
   curl -X POST http://localhost:8000/execute \
     -d '{"language": "python", "code": "print(\"hello\")"}'

   # New
   curl -X POST http://localhost:8000/api/v1/execute \
     -H "X-API-Key: your-key" \
     -H "Content-Type: application/json" \
     -d '{"language": "python", "code": "print(\"hello\")"}'
   ```

## Deprecation Notices

### Deprecated in 1.0.0

- Old API endpoints (will be removed in 2.0.0)
- Old environment variable names (will be removed in 2.0.0)
- Old Docker image tags (will be removed in 2.0.0)

### Planned for 2.0.0

- Removal of deprecated API endpoints
- Removal of old environment variables
- Removal of old Docker image tags
- New authentication system
- Enhanced security features

## Contributing

To add entries to this changelog:

1. Add your changes under the appropriate section
2. Use the following format:

   ```
   ### Added
   - New feature description

   ### Changed
   - Changed feature description

   ### Fixed
   - Bug fix description
   ```

3. Follow the [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format
4. Use clear, concise descriptions
5. Reference issue numbers when applicable

## Release Process

1. **Version Update**: Update version in `Cargo.toml`
2. **Changelog**: Add new version section to `CHANGELOG.md`
3. **Testing**: Run full test suite
4. **Documentation**: Update documentation if needed
5. **Tag**: Create git tag for the release
6. **Build**: Build and test Docker images
7. **Publish**: Push to container registries
8. **Announce**: Create GitHub release

---

For more information, see the [main README](README.md) and other documentation files.
