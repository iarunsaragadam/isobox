# 🔒 isobox – Secure Code Execution API

A minimal, safe REST API that executes arbitrary code inside language-specific Docker containers with full isolation.

## ✨ Features

- **REST API** with simple endpoint: `POST /execute`
- **Language Support**: Python and Node.js (easily extensible)
- **Docker Isolation**: Complete network isolation and ephemeral containers
- **Safety**: Automatic cleanup of temporary files and containers
- **Fast**: Async execution with Rust performance

## 🚀 Quick Start

### Prerequisites

- [Docker](https://docs.docker.com/get-docker/) installed and running
- [Rust](https://rustup.rs/) (for local development)

### Run with Docker Compose (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/isobox.git
cd isobox

# Start the service
docker-compose up --build

# The API will be available at http://localhost:8000
```

### Run Locally

```bash
# Install dependencies and run
cargo run

# Or build and run release version
cargo build --release
./target/release/isobox
```

### Deploy with Docker

The application is containerized and can be deployed to any container platform:

```bash
# Build the image
docker build -t isobox .

# Run locally
docker run -p 8000:8000 -v /var/run/docker.sock:/var/run/docker.sock isobox

# Or use the published image from GitHub Container Registry
docker run -p 8000:8000 -v /var/run/docker.sock:/var/run/docker.sock ghcr.io/yourusername/isobox:latest
```

### Deployment Options

The containerized application can be deployed to various platforms:

- **Kubernetes**: Use the Docker image in your Kubernetes manifests
- **Docker Swarm**: Deploy as a service in Docker Swarm
- **AWS ECS/Fargate**: Use the image in ECS task definitions
- **Azure Container Instances**: Deploy directly to ACI
- **Any container platform**: The image is platform-agnostic

**Note**: The application requires access to the Docker daemon socket (`/var/run/docker.sock`) for code execution isolation. Ensure your deployment environment provides this access.

## 📖 API Usage

### Execute Code

**Endpoint**: `POST /execute`

**Request Body**:

```json
{
  "language": "python",
  "code": "print('Hello from isobox!')"
}
```

**Response**:

```json
{
  "stdout": "Hello from isobox!\n",
  "stderr": "",
  "exit_code": 0
}
```

### Health Check

**Endpoint**: `GET /health`

**Response**:

```json
{
  "status": "healthy",
  "service": "isobox",
  "version": "0.1.0"
}
```

## 🧪 Examples

### Python Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "import math\nprint(f\"π = {math.pi}\")\nprint(f\"e = {math.e}\")"
  }'
```

### Node.js Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "node",
    "code": "console.log(\"Hello from Node.js!\"); console.log(\"Current time:\", new Date().toISOString());"
  }'
```

### Error Handling Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(undefined_variable)"
  }'
```

## 🛡️ Security Features

- **Network Isolation**: Containers run with `--network none`
- **Ephemeral Containers**: All containers are removed after execution (`--rm`)
- **Temporary Files**: Code files are written to unique temp directories and cleaned up
- **No Persistence**: No data persists between executions

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   REST API      │    │   Executor      │    │  Docker         │
│   (Actix-Web)   │───▶│   Module        │───▶│  Containers     │
│                 │    │                 │    │  (Isolated)     │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

### Components

- **REST API** (`main.rs`): Handles HTTP requests and responses
- **Executor** (`executor.rs`): Manages Docker container execution and cleanup
- **Docker Containers**: Language-specific isolated execution environments

## 🔧 Configuration

### Supported Languages

| Language | Docker Image  | File Extension |
| -------- | ------------- | -------------- |
| Python   | `python:3.11` | `.py`          |
| Node.js  | `node:20`     | `.js`          |

### Environment Variables

- `PORT`: Server port (default: 8000)
- `RUST_LOG`: Log level (default: info)

## 🔄 CI/CD Pipeline

This repository includes GitHub Actions workflows for automated testing and deployment:

### Workflows

- **Test and Build** (`.github/workflows/test.yml`): Runs on pull requests

  - Rust compilation and testing
  - Code formatting and linting checks
  - Docker image building and testing

- **Build and Publish** (`.github/workflows/docker-publish.yml`): Runs on main branch
  - Multi-platform Docker builds (linux/amd64, linux/arm64)
  - Publishes to GitHub Container Registry (GHCR)
  - Vulnerability scanning with Trivy
  - Comprehensive functionality testing

### Container Registry

Images are automatically published to GitHub Container Registry:

- **Latest**: `ghcr.io/yourusername/isobox:latest`
- **Branch**: `ghcr.io/yourusername/isobox:main`
- **Commit**: `ghcr.io/yourusername/isobox:sha-<commit-hash>`

## 🛠️ Development

### Adding New Languages

To add support for a new language, update the `CodeExecutor::new()` method in `src/executor.rs`:

```rust
// Add to language_configs HashMap
language_configs.insert(
    "go".to_string(),
    LanguageConfig {
        docker_image: "golang:1.19".to_string(),
        file_name: "main.go".to_string(),
        run_command: vec!["go".to_string(), "run".to_string(), "main.go".to_string()],
    },
);
```

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Run tests
cargo test

# Check code formatting
cargo fmt

# Run clippy (linter)
cargo clippy
```

### Docker Build

```bash
# Build the Docker image
docker build -t isobox .

# Run the container
docker run -p 8000:8000 -v /var/run/docker.sock:/var/run/docker.sock isobox
```

## 📝 Logging

The application uses structured logging. Set the `RUST_LOG` environment variable to control log levels:

```bash
# Debug level logging
RUST_LOG=debug cargo run

# Info level logging (default)
RUST_LOG=info cargo run
```

## 🚧 Future Enhancements

- [ ] **Resource Limits**: Memory and CPU constraints for containers
- [ ] **Timeout Configuration**: Configurable execution timeouts
- [ ] **Input Support**: Support for stdin and command-line arguments
- [ ] **More Languages**: Go, Rust, Java, C++, etc.
- [ ] **Metrics**: Prometheus metrics for monitoring
- [ ] **Rate Limiting**: API rate limiting and quotas
- [ ] **Authentication**: API key or token-based authentication

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ⚠️ Security Notice

This tool executes arbitrary code in Docker containers. While containers provide isolation, always:

- Run on isolated infrastructure
- Keep Docker and the host system updated
- Monitor resource usage
- Consider additional security layers for production use

---

Built with ❤️ and 🦀 Rust
