# Frequently Asked Questions (FAQ)

This document answers common questions about IsoBox, its features, usage, and troubleshooting.

## Table of Contents

1. [General Questions](#general-questions)
2. [Installation & Setup](#installation--setup)
3. [Authentication](#authentication)
4. [Code Execution](#code-execution)
5. [Languages & Features](#languages--features)
6. [Security](#security)
7. [Performance & Scaling](#performance--scaling)
8. [Troubleshooting](#troubleshooting)
9. [API Usage](#api-usage)
10. [Docker & Deployment](#docker--deployment)

## General Questions

### What is IsoBox?

IsoBox is a secure, containerized code execution service that allows you to run code in isolated Docker containers. It supports multiple programming languages and provides both HTTP REST API and gRPC interfaces.

### What are the main features of IsoBox?

- **Multi-language Support**: Python, Node.js, Java, Go, Rust, C++, C, PHP, Ruby, Bash
- **Container Isolation**: Each execution runs in a separate Docker container
- **Multi-Authentication**: API keys, JWT, OAuth2, mTLS, and no authentication
- **Test Case Support**: Run code against multiple test cases
- **Resource Limits**: CPU, memory, process limits
- **Dual API Support**: HTTP REST API and gRPC
- **Code Deduplication**: Hash-based caching
- **Security Features**: Network isolation, privilege dropping, timeout protection

### What is IsoBox used for?

IsoBox is commonly used for:

- Online IDEs and code editors
- Coding platforms and competitions
- Educational programming environments
- Automated code testing and evaluation
- Code execution APIs for applications
- Development and testing environments

### Is IsoBox open source?

Yes, IsoBox is open source and available under the MIT License. You can find the source code on GitHub.

### What license does IsoBox use?

IsoBox uses the MIT License, which is a permissive open source license that allows commercial use, modification, distribution, and private use.

## Installation & Setup

### What are the system requirements?

**Minimum Requirements:**

- Docker 20.10+
- 2GB RAM
- 10GB disk space
- Linux, macOS, or Windows with Docker support

**Recommended Requirements:**

- Docker 24.0+
- 4GB RAM
- 20GB disk space
- SSD storage
- Multi-core CPU

### How do I install IsoBox?

**Option 1: Docker (Recommended)**

```bash
# Pull the image
docker pull isobox/isobox:latest

# Run with API key authentication
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key" \
  isobox/isobox:latest
```

**Option 2: Docker Compose**

```bash
# Create docker-compose.yml
cat > docker-compose.yml << 'EOF'
version: '3.8'
services:
  isobox:
    image: isobox/isobox:latest
    ports:
      - "8000:8000"
      - "9000:9000"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /tmp:/tmp
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=your-api-key
    restart: unless-stopped
EOF

# Start the service
docker compose up -d
```

**Option 3: Build from Source**

```bash
# Clone the repository
git clone <repository-url>
cd isobox

# Build and run
cargo build --release
cargo run
```

### How do I verify the installation?

```bash
# Check if the container is running
docker ps | grep isobox

# Test the health endpoint
curl http://localhost:8000/health

# Test code execution
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

### What ports does IsoBox use?

- **8000**: HTTP REST API
- **9000**: gRPC API

You can change these ports using the `REST_PORT` and `GRPC_PORT` environment variables.

### Do I need Docker installed?

Yes, Docker is required for IsoBox to work. IsoBox uses Docker containers to isolate code execution for security and resource management.

## Authentication

### What authentication methods are supported?

IsoBox supports multiple authentication methods:

1. **API Key Authentication** (Default)

   ```bash
   AUTH_TYPE=apikey
   API_KEYS=key1,key2,key3
   API_KEY_HEADER=X-API-Key
   ```

2. **JWT Authentication**

   ```bash
   AUTH_TYPE=jwt
   JWT_ISSUER_URL=https://accounts.google.com
   JWT_AUDIENCE=your-app-id
   JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs
   ```

3. **OAuth2 Authentication**

   ```bash
   AUTH_TYPE=oauth2
   OAUTH2_PROVIDER=firebase
   OAUTH2_CLIENT_ID=your-client-id
   OAUTH2_CLIENT_SECRET=your-client-secret
   ```

4. **mTLS Authentication**

   ```bash
   AUTH_TYPE=mtls
   MTLS_CA_CERT=/path/to/ca.crt
   MTLS_CLIENT_CERT=/path/to/client.crt
   ```

5. **No Authentication** (Development only)
   ```bash
   AUTH_TYPE=none
   ```

### How do I generate a secure API key?

```bash
# Generate a random API key
openssl rand -hex 32

# Or use a UUID
uuidgen

# Or use a secure random string
head -c 32 /dev/urandom | base64
```

### Can I use multiple API keys?

Yes, you can specify multiple API keys separated by commas:

```bash
API_KEYS=key1,key2,key3,key4
```

### How do I rotate API keys?

1. **Add new keys**:

   ```bash
   API_KEYS=old-key1,old-key2,new-key1,new-key2
   ```

2. **Update clients** to use new keys

3. **Remove old keys**:
   ```bash
   API_KEYS=new-key1,new-key2
   ```

### How do I set up Firebase authentication?

1. **Create a Firebase project** and get your credentials

2. **Configure environment variables**:

   ```bash
   AUTH_TYPE=oauth2
   OAUTH2_PROVIDER=firebase
   OAUTH2_CLIENT_ID=your-firebase-client-id
   OAUTH2_CLIENT_SECRET=your-firebase-client-secret
   OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token
   OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo
   ```

3. **Get a Firebase token** and use it in requests:
   ```bash
   curl -X POST http://localhost:8000/api/v1/execute \
     -H "Authorization: Bearer YOUR_FIREBASE_TOKEN" \
     -H "Content-Type: application/json" \
     -d '{"language": "python", "code": "print(\"Hello\")"}'
   ```

## Code Execution

### What programming languages are supported?

| Language    | Docker Image       | Compilation | Features                       |
| ----------- | ------------------ | ----------- | ------------------------------ |
| **Python**  | `python:3.11-slim` | No          | Standard library, pip packages |
| **Node.js** | `node:18-slim`     | No          | npm packages, async/await      |
| **Java**    | `openjdk:17-slim`  | Yes         | Collections, Streams, Time API |
| **Go**      | `golang:1.21`      | Yes         | Goroutines, channels, modules  |
| **Rust**    | `rust:1.70-slim`   | Yes         | Cargo, async, traits           |
| **C++**     | `gcc:latest`       | Yes         | STL, modern C++ features       |
| **C**       | `gcc:latest`       | Yes         | Standard library               |
| **PHP**     | `php:8.2-cli`      | No          | Composer, modern PHP           |
| **Ruby**    | `ruby:3.2-slim`    | No          | Gems, modern Ruby              |
| **Bash**    | `ubuntu:22.04`     | No          | Shell scripting                |

### How do I execute code?

**Basic Code Execution**:

```bash
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "language": "python",
    "code": "print(\"Hello, World!\")"
  }'
```

**With Test Cases**:

```bash
curl -X POST http://localhost:8000/api/v1/execute/test-cases \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "language": "python",
    "code": "import sys\n\ndata = sys.stdin.read().strip()\nprint(data[::-1])",
    "test_cases": [
      {
        "name": "test_1",
        "input": "Hello",
        "expected_output": "olleH",
        "timeout_seconds": 5,
        "memory_limit_mb": 128
      }
    ]
  }'
```

### What are the resource limits?

**Default Limits**:

- **CPU**: 1 core
- **Memory**: 512MB
- **Processes**: 100
- **File descriptors**: 1024
- **Execution time**: 30 seconds
- **Disk space**: 100MB

**Custom Limits**:

```bash
# Set custom limits via environment variables
export MAX_MEMORY_MB=1024
export MAX_CPU_CORES=2
export MAX_EXECUTION_TIME=60
export MAX_PROCESSES=200
```

### How do I handle timeouts?

**Configure Timeout**:

```bash
# Set global timeout
export MAX_EXECUTION_TIME=60

# Set per-request timeout
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{
    "language": "python",
    "code": "import time\ntime.sleep(10)\nprint(\"Done\")",
    "timeout_seconds": 15
  }'
```

**Handle Timeout Errors**:

```json
{
  "error": "Execution timeout",
  "message": "Code execution exceeded the maximum time limit of 15 seconds",
  "time_taken": 15.0,
  "stdout": "",
  "stderr": "",
  "exit_code": null
}
```

### Can I use external libraries?

**Python**:

```python
# Standard library (available by default)
import json
import datetime
import math

# External packages (need to be installed)
import requests  # May not be available
import numpy     # May not be available
```

**Node.js**:

```javascript
// Built-in modules (available by default)
const fs = require("fs");
const http = require("http");

// External packages (need to be installed)
const axios = require("axios"); // May not be available
```

**Note**: External libraries may not be available in the default containers. You may need to build custom Docker images with additional packages.

### How do I handle input/output?

**Reading Input**:

```python
# Python
import sys
data = sys.stdin.read().strip()
print(f"Received: {data}")
```

```javascript
// Node.js
const fs = require("fs");
const data = fs.readFileSync(0, "utf8").trim();
console.log(`Received: ${data}`);
```

```java
// Java
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        String data = scanner.nextLine();
        System.out.println("Received: " + data);
    }
}
```

**Writing Output**:

```python
# Python
print("Standard output")
import sys
sys.stderr.write("Error output\n")
```

```javascript
// Node.js
console.log("Standard output");
console.error("Error output");
```

## Languages & Features

### How do I add a new programming language?

1. **Define Language Configuration**:

   ```rust
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

2. **Create Docker Image**:

   ```dockerfile
   FROM ubuntu:22.04
   RUN apt-get update && apt-get install -y newlang
   WORKDIR /workspace
   USER coder
   CMD ["newlang"]
   ```

3. **Add Tests**:
   ```rust
   #[test]
   fn test_newlang_execution() {
       let result = execute_code("newlang", "print('Hello')", None);
       assert!(result.is_ok());
   }
   ```

### What are the differences between interpreted and compiled languages?

**Interpreted Languages** (Python, Node.js, PHP, Ruby, Bash):

- No compilation step
- Faster startup time
- Slower execution for complex code
- Examples: `python script.py`, `node script.js`

**Compiled Languages** (Java, Go, Rust, C++, C):

- Compilation step required
- Slower startup time
- Faster execution
- Examples: `javac Main.java && java Main`, `go run main.go`

### How do I handle dependencies?

**Python Dependencies**:

```python
# Try to import, handle gracefully if not available
try:
    import requests
    response = requests.get('https://api.example.com')
    print(response.json())
except ImportError:
    print("requests library not available")
    # Fallback implementation
```

**Node.js Dependencies**:

```javascript
// Try to require, handle gracefully if not available
try {
  const axios = require("axios");
  const response = await axios.get("https://api.example.com");
  console.log(response.data);
} catch (error) {
  console.log("axios library not available");
  // Fallback implementation
}
```

### Can I use databases or network access?

**Network Access**: By default, containers run with `--network none` for security. Network access is disabled.

**Databases**: No external database access is available due to network isolation.

**File System**: Limited access to `/tmp` directory only.

**Alternative Approaches**:

- Use in-memory data structures
- Work with provided input data
- Use standard library features
- Implement algorithms without external dependencies

## Security

### Is IsoBox secure?

Yes, IsoBox implements multiple security layers:

1. **Container Isolation**: Each execution runs in a separate Docker container
2. **Resource Limits**: CPU, memory, process limits
3. **Network Isolation**: Containers run with `--network none`
4. **Privilege Dropping**: Containers run with dropped capabilities
5. **Authentication**: Multiple authentication methods
6. **Input Validation**: Comprehensive request validation
7. **Timeout Protection**: Configurable execution timeouts

### Can code escape the container?

The risk is minimized through:

- **No new privileges**: `--security-opt no-new-privileges`
- **Dropped capabilities**: `--cap-drop=ALL`
- **Read-only filesystem**: `--read-only`
- **Network isolation**: `--network=none`
- **Resource limits**: Memory, CPU, process limits

However, no security system is 100% foolproof. Always run in a controlled environment.

### How do I handle malicious code?

1. **Input Validation**: Validate code before execution
2. **Resource Limits**: Set strict resource limits
3. **Timeout Protection**: Use short timeouts
4. **Monitoring**: Monitor for suspicious activity
5. **Isolation**: Run in isolated environment

### Can I disable authentication for development?

Yes, but only for development:

```bash
AUTH_TYPE=none
```

**Warning**: Never use `AUTH_TYPE=none` in production.

### How do I secure the Docker socket?

**Option 1: Docker Socket Proxy**:

```bash
# Run Docker socket proxy
docker run -d \
  --name docker-socket-proxy \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -p 2375:2375 \
  tecnativa/docker-socket-proxy

# Use proxy with IsoBox
docker run -d \
  --name isobox \
  -e DOCKER_HOST=tcp://docker-socket-proxy:2375 \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-key" \
  isobox/isobox:latest
```

**Option 2: Docker-in-Docker**:

```bash
# Use Docker-in-Docker
docker run -d \
  --name isobox \
  --privileged \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-key" \
  isobox/isobox:latest
```

## Performance & Scaling

### How do I improve performance?

1. **Use Caching**:

   ```bash
   DEDUP_ENABLED=true
   DEDUP_CACHE_TTL=3600
   DEDUP_CACHE_TYPE=memory
   ```

2. **Optimize Resource Limits**:

   ```bash
   MAX_MEMORY_MB=1024
   MAX_CPU_CORES=2
   MAX_EXECUTION_TIME=30
   ```

3. **Use SSD Storage**:

   ```bash
   # Mount SSD volume
   docker run -v /ssd/tmp:/tmp isobox/isobox:latest
   ```

4. **Enable Compression**:
   ```bash
   # Use gzip compression
   curl -H "Accept-Encoding: gzip" ...
   ```

### How do I scale IsoBox?

**Horizontal Scaling**:

```yaml
# docker-compose.yml with multiple instances
version: "3.8"
services:
  isobox:
    image: isobox/isobox:latest
    deploy:
      replicas: 3
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=your-key
```

**Load Balancing**:

```nginx
# nginx.conf
upstream isobox {
    server isobox1:8000;
    server isobox2:8000;
    server isobox3:8000;
}

server {
    listen 80;
    location / {
        proxy_pass http://isobox;
    }
}
```

**Kubernetes Deployment**:

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: isobox
spec:
  replicas: 3
  selector:
    matchLabels:
      app: isobox
  template:
    metadata:
      labels:
        app: isobox
    spec:
      containers:
        - name: isobox
          image: isobox/isobox:latest
          env:
            - name: AUTH_TYPE
              value: "apikey"
            - name: API_KEYS
              valueFrom:
                secretKeyRef:
                  name: isobox-secrets
                  key: api-keys
```

### How do I monitor performance?

**Resource Monitoring**:

```bash
# Monitor container resources
docker stats isobox

# Monitor system resources
htop
iostat
df -h
```

**Application Monitoring**:

```bash
# Monitor logs
docker logs -f isobox

# Monitor API endpoints
curl -w "@curl-format.txt" -o /dev/null -s http://localhost:8000/health
```

**Performance Metrics**:

- Execution time
- Memory usage
- CPU usage
- Request rate
- Error rate
- Cache hit rate

### What's the maximum concurrent executions?

The maximum concurrent executions depends on:

- Available system resources
- Docker daemon limits
- Container resource limits
- Network capacity

**Typical Limits**:

- **Small instance**: 10-50 concurrent executions
- **Medium instance**: 50-200 concurrent executions
- **Large instance**: 200-1000 concurrent executions

## Troubleshooting

### Container won't start

**Check Docker logs**:

```bash
docker logs isobox
```

**Common Issues**:

1. **Port already in use**:

   ```bash
   # Check what's using the port
   sudo lsof -i :8000

   # Use different ports
   docker run -p 8001:8000 -p 9001:9000 isobox/isobox:latest
   ```

2. **Docker socket permission**:

   ```bash
   # Fix permissions
   sudo chmod 666 /var/run/docker.sock

   # Or add user to docker group
   sudo usermod -aG docker $USER
   newgrp docker
   ```

3. **Insufficient resources**:
   ```bash
   # Check available resources
   free -h
   df -h
   ```

### Authentication errors

**Check API key**:

```bash
# Verify API key is set
echo $API_KEYS

# Test with curl
curl -X POST http://localhost:8000/api/v1/execute \
  -H "X-API-Key: your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"test\")"}'
```

**Check authentication type**:

```bash
# Verify authentication type
echo $AUTH_TYPE

# Check logs for authentication errors
docker logs isobox | grep -i auth
```

### Code execution fails

**Check language support**:

```bash
# Get supported languages
curl http://localhost:8000/api/v1/languages
```

**Check resource limits**:

```bash
# Monitor resource usage
docker stats isobox

# Check if limits are too low
docker logs isobox | grep -i "memory\|cpu\|timeout"
```

**Check code syntax**:

```bash
# Test with simple code first
curl -X POST http://localhost:8000/api/v1/execute \
  -H "X-API-Key: your-api-key" \
  -H "Content-Type: application/json" \
  -d '{"language": "python", "code": "print(\"Hello\")"}'
```

### Performance issues

**Check resource usage**:

```bash
# Monitor system resources
htop
free -h
df -h

# Monitor container resources
docker stats isobox
```

**Check logs for errors**:

```bash
# Check for errors
docker logs isobox | grep -i error

# Check for warnings
docker logs isobox | grep -i warning
```

**Optimize configuration**:

```bash
# Increase resource limits
export MAX_MEMORY_MB=2048
export MAX_CPU_CORES=4

# Enable caching
export DEDUP_ENABLED=true
export DEDUP_CACHE_TTL=3600
```

## API Usage

### What's the API response format?

**Success Response**:

```json
{
  "stdout": "Hello, World!\n",
  "stderr": "",
  "exit_code": 0,
  "time_taken": 0.123,
  "memory_used": 1024000,
  "test_results": null
}
```

**Error Response**:

```json
{
  "error": "Authentication failed",
  "message": "Invalid API key",
  "details": null
}
```

### How do I handle errors?

**Common Error Codes**:

- `400`: Bad Request (invalid input)
- `401`: Unauthorized (authentication failed)
- `403`: Forbidden (insufficient permissions)
- `404`: Not Found (endpoint not found)
- `429`: Too Many Requests (rate limited)
- `500`: Internal Server Error (server error)

**Error Handling Example**:

```javascript
try {
  const response = await fetch("http://localhost:8000/api/v1/execute", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "X-API-Key": "your-api-key",
    },
    body: JSON.stringify({
      language: "python",
      code: 'print("Hello")',
    }),
  });

  if (!response.ok) {
    const error = await response.json();
    console.error("Error:", error.message);
    return;
  }

  const result = await response.json();
  console.log("Output:", result.stdout);
} catch (error) {
  console.error("Request failed:", error);
}
```

### How do I use the gRPC API?

**Install grpcurl**:

```bash
# macOS
brew install grpcurl

# Linux
go install github.com/fullstorydev/grpcurl/cmd/grpcurl@latest
```

**Basic Usage**:

```bash
# Execute code
grpcurl -plaintext \
  -proto proto/isobox.proto \
  -H "authorization: your-api-key" \
  -d '{"language": "python", "code": "print(\"Hello\")"}' \
  localhost:9000 isobox.CodeExecutionService/ExecuteCode

# Health check
grpcurl -plaintext localhost:9000 isobox.CodeExecutionService/HealthCheck

# Get supported languages
grpcurl -plaintext localhost:9000 isobox.CodeExecutionService/GetSupportedLanguages
```

### How do I implement rate limiting?

**Client-side Rate Limiting**:

```javascript
class RateLimiter {
  constructor(maxRequests, timeWindow) {
    this.maxRequests = maxRequests;
    this.timeWindow = timeWindow;
    this.requests = [];
  }

  async execute(fn) {
    const now = Date.now();
    this.requests = this.requests.filter(
      (time) => now - time < this.timeWindow
    );

    if (this.requests.length >= this.maxRequests) {
      throw new Error("Rate limit exceeded");
    }

    this.requests.push(now);
    return fn();
  }
}

const limiter = new RateLimiter(10, 60000); // 10 requests per minute

// Use rate limiter
await limiter.execute(async () => {
  const response = await fetch("http://localhost:8000/api/v1/execute", {
    // ... request options
  });
  return response.json();
});
```

## Docker & Deployment

### How do I build a custom Docker image?

**Custom Dockerfile**:

```dockerfile
FROM isobox/isobox:latest

# Add custom packages
RUN apt-get update && apt-get install -y \
    python3-pip \
    nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Python packages
RUN pip3 install requests numpy pandas

# Install Node.js packages
RUN npm install -g axios lodash

# Copy custom configuration
COPY custom-config.toml /app/config/

# Set custom environment variables
ENV CUSTOM_FEATURE=true
ENV EXTRA_PACKAGES=requests,numpy

CMD ["isobox"]
```

**Build and Run**:

```bash
# Build custom image
docker build -t my-isobox:latest .

# Run custom image
docker run -d \
  --name my-isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-key" \
  my-isobox:latest
```

### How do I deploy to production?

**Production Checklist**:

- [ ] Use HTTPS/TLS
- [ ] Configure proper authentication
- [ ] Set up monitoring and alerting
- [ ] Implement rate limiting
- [ ] Use secure secrets management
- [ ] Configure firewall rules
- [ ] Enable security scanning
- [ ] Set up log monitoring
- [ ] Configure backup and recovery
- [ ] Test security measures

**Production Docker Compose**:

```yaml
version: "3.8"
services:
  isobox:
    image: isobox/isobox:latest
    ports:
      - "8000:8000"
      - "9000:9000"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /tmp:/tmp
      - /var/log/isobox:/app/logs
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=${API_KEYS}
      - REST_PORT=8000
      - GRPC_PORT=9000
      - DEDUP_ENABLED=true
      - DEDUP_CACHE_TTL=3600
      - RUST_LOG=info
    restart: unless-stopped
    security_opt:
      - no-new-privileges
    cap_drop:
      - ALL
    read_only: true
    tmpfs:
      - /tmp
      - /var/run
    networks:
      - isobox-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf
      - ./ssl:/etc/nginx/ssl
    depends_on:
      - isobox
    networks:
      - isobox-network

networks:
  isobox-network:
    driver: bridge
```

### How do I update IsoBox?

**Update Process**:

```bash
# Stop current container
docker stop isobox

# Remove old container
docker rm isobox

# Pull new image
docker pull isobox/isobox:latest

# Run new container
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-key" \
  isobox/isobox:latest

# Verify update
curl http://localhost:8000/health
```

**Rollback Process**:

```bash
# If update fails, rollback to previous version
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-key" \
  isobox/isobox:v1.0.0
```

### How do I backup and restore?

**Backup Configuration**:

```bash
# Backup environment variables
docker inspect isobox | grep -A 20 "Env"

# Backup volumes
docker run --rm -v isobox_data:/data -v $(pwd):/backup alpine tar czf /backup/isobox_backup.tar.gz -C /data .

# Backup logs
docker logs isobox > isobox_logs.txt
```

**Restore Configuration**:

```bash
# Restore environment variables
export $(docker inspect isobox | grep -A 20 "Env" | grep -o 'API_KEYS=[^,]*')

# Restore volumes
docker run --rm -v isobox_data:/data -v $(pwd):/backup alpine tar xzf /backup/isobox_backup.tar.gz -C /data
```

---

For more information, see the [main README](README.md) and other documentation files.
