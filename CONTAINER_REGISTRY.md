# Container Registry Documentation

This document provides comprehensive information about using IsoBox from various container registries, including Docker Hub, GitHub Container Registry, and other popular registries.

## Table of Contents

1. [Available Images](#available-images)
2. [Docker Hub](#docker-hub)
3. [GitHub Container Registry](#github-container-registry)
4. [Quick Start](#quick-start)
5. [Configuration](#configuration)
6. [Authentication Methods](#authentication-methods)
7. [Production Deployment](#production-deployment)
8. [Security Considerations](#security-considerations)
9. [Troubleshooting](#troubleshooting)

## Available Images

### Official Images

| Registry   | Image Name                     | Description           |
| ---------- | ------------------------------ | --------------------- |
| Docker Hub | `isobox/isobox:latest`         | Latest stable release |
| Docker Hub | `isobox/isobox:1.0.0`          | Specific version      |
| GitHub CR  | `ghcr.io/isobox/isobox:latest` | Latest from GitHub    |
| GitHub CR  | `ghcr.io/isobox/isobox:v1.0.0` | Versioned release     |

### Image Tags

- `latest` - Latest stable release
- `v1.0.0`, `v1.1.0`, etc. - Specific versions
- `dev` - Development builds
- `alpine` - Alpine Linux based (smaller size)
- `debian` - Debian based (default)

## Docker Hub

### Pull Image

```bash
# Pull latest version
docker pull isobox/isobox:latest

# Pull specific version
docker pull isobox/isobox:v1.0.0

# Pull with platform specification
docker pull --platform linux/amd64 isobox/isobox:latest
```

### Run Container

```bash
# Basic run with API key authentication
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key-here,another-key" \
  -e API_KEY_HEADER=X-API-Key \
  isobox/isobox:latest
```

### Docker Compose

```yaml
# docker-compose.yml
version: "3.8"
services:
  isobox:
    image: isobox/isobox:latest
    ports:
      - "8000:8000" # REST API
      - "9000:9000" # gRPC API
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /tmp:/tmp
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=your-api-key-here,another-key
      - API_KEY_HEADER=X-API-Key
      - REST_PORT=8000
      - GRPC_PORT=9000
      - DEDUP_ENABLED=true
      - DEDUP_CACHE_TTL=3600
    restart: unless-stopped
```

## GitHub Container Registry

### Pull Image

```bash
# Pull latest version
docker pull ghcr.io/isobox/isobox:latest

# Pull specific version
docker pull ghcr.io/isobox/isobox:v1.0.0

# Authenticate (if needed)
echo $GITHUB_TOKEN | docker login ghcr.io -u USERNAME --password-stdin
```

### Run Container

```bash
# Run with GitHub Container Registry image
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key-here" \
  ghcr.io/isobox/isobox:latest
```

### Docker Compose with GitHub CR

```yaml
# docker-compose-ghcr.yml
version: "3.8"
services:
  isobox:
    image: ghcr.io/isobox/isobox:latest
    ports:
      - "8000:8000"
      - "9000:9000"
    volumes:
      - /var/run/docker.sock:/var/run/docker.sock
      - /tmp:/tmp
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=your-api-key-here
    restart: unless-stopped
```

## Quick Start

### 1. Pull and Run (API Key Auth)

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
  -e API_KEYS="test-key-123,dev-key-456" \
  -e API_KEY_HEADER=X-API-Key \
  isobox/isobox:latest

# Test the service
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key-123" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

### 2. Pull and Run (Firebase OAuth2)

```bash
# Pull the image
docker pull isobox/isobox:latest

# Run with Firebase authentication
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=oauth2 \
  -e OAUTH2_PROVIDER=firebase \
  -e OAUTH2_CLIENT_ID=your-firebase-client-id \
  -e OAUTH2_CLIENT_SECRET=your-firebase-client-secret \
  -e OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token \
  -e OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo \
  isobox/isobox:latest

# Test with Firebase token
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_FIREBASE_TOKEN" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

### 3. Docker Compose Quick Start

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
      - API_KEYS=test-key-123,dev-key-456
      - API_KEY_HEADER=X-API-Key
      - REST_PORT=8000
      - GRPC_PORT=9000
      - DEDUP_ENABLED=true
      - DEDUP_CACHE_TTL=3600
    restart: unless-stopped
EOF

# Start the service
docker compose up -d

# Test the service
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: test-key-123" \
  -d '{"language": "python", "code": "print(\"Hello, IsoBox!\")"}'
```

## Configuration

### Environment Variables

| Variable          | Description               | Default       | Required |
| ----------------- | ------------------------- | ------------- | -------- |
| `AUTH_TYPE`       | Authentication type       | `apikey`      | No       |
| `API_KEYS`        | Comma-separated API keys  | `default-key` | No       |
| `API_KEY_HEADER`  | Header name for API key   | `X-API-Key`   | No       |
| `REST_PORT`       | HTTP REST API port        | `8000`        | No       |
| `GRPC_PORT`       | gRPC API port             | `9000`        | No       |
| `DEDUP_ENABLED`   | Enable code deduplication | `true`        | No       |
| `DEDUP_CACHE_TTL` | Cache TTL in seconds      | `3600`        | No       |
| `RUST_LOG`        | Log level                 | `info`        | No       |

### Volume Mounts

| Path                   | Description          | Required |
| ---------------------- | -------------------- | -------- |
| `/var/run/docker.sock` | Docker daemon socket | Yes      |
| `/tmp`                 | Temporary files      | Yes      |
| `/app/config`          | Configuration files  | No       |
| `/app/logs`            | Log files            | No       |

### Port Mappings

| Container Port | Host Port | Protocol | Description |
| -------------- | --------- | -------- | ----------- |
| `8000`         | `8000`    | HTTP     | REST API    |
| `9000`         | `9000`    | gRPC     | gRPC API    |

## Authentication Methods

### 1. API Key Authentication

```bash
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="key1,key2,key3" \
  -e API_KEY_HEADER=X-API-Key \
  isobox/isobox:latest
```

### 2. JWT Authentication

```bash
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=jwt \
  -e JWT_ISSUER_URL=https://accounts.google.com \
  -e JWT_AUDIENCE=your-app-id \
  -e JWT_PUBLIC_KEY_URL=https://www.googleapis.com/oauth2/v1/certs \
  isobox/isobox:latest
```

### 3. OAuth2 Authentication (Firebase)

```bash
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=oauth2 \
  -e OAUTH2_PROVIDER=firebase \
  -e OAUTH2_CLIENT_ID=your-firebase-client-id \
  -e OAUTH2_CLIENT_SECRET=your-firebase-client-secret \
  -e OAUTH2_TOKEN_URL=https://oauth2.googleapis.com/token \
  -e OAUTH2_USERINFO_URL=https://www.googleapis.com/oauth2/v2/userinfo \
  isobox/isobox:latest
```

### 4. No Authentication (Development)

```bash
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=none \
  isobox/isobox:latest
```

## Production Deployment

### 1. Kubernetes Deployment

```yaml
# k8s-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: isobox
  labels:
    app: isobox
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
          ports:
            - containerPort: 8000
              name: rest-api
            - containerPort: 9000
              name: grpc-api
          env:
            - name: AUTH_TYPE
              value: "apikey"
            - name: API_KEYS
              valueFrom:
                secretKeyRef:
                  name: isobox-secrets
                  key: api-keys
            - name: REST_PORT
              value: "8000"
            - name: GRPC_PORT
              value: "9000"
          volumeMounts:
            - name: docker-sock
              mountPath: /var/run/docker.sock
            - name: tmp-volume
              mountPath: /tmp
      volumes:
        - name: docker-sock
          hostPath:
            path: /var/run/docker.sock
        - name: tmp-volume
          emptyDir: {}
---
apiVersion: v1
kind: Service
metadata:
  name: isobox-service
spec:
  selector:
    app: isobox
  ports:
    - name: rest-api
      port: 8000
      targetPort: 8000
    - name: grpc-api
      port: 9000
      targetPort: 9000
  type: LoadBalancer
```

### 2. Docker Swarm

```yaml
# docker-stack.yml
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
    environment:
      - AUTH_TYPE=apikey
      - API_KEYS=${API_KEYS}
      - API_KEY_HEADER=X-API-Key
      - REST_PORT=8000
      - GRPC_PORT=9000
    deploy:
      replicas: 3
      restart_policy:
        condition: on-failure
        delay: 5s
        max_attempts: 3
```

### 3. AWS ECS

```json
{
  "family": "isobox",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "executionRoleArn": "arn:aws:iam::123456789012:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::123456789012:role/isobox-task-role",
  "containerDefinitions": [
    {
      "name": "isobox",
      "image": "isobox/isobox:latest",
      "portMappings": [
        {
          "containerPort": 8000,
          "protocol": "tcp"
        },
        {
          "containerPort": 9000,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "AUTH_TYPE",
          "value": "apikey"
        },
        {
          "name": "API_KEYS",
          "value": "your-api-key-here"
        },
        {
          "name": "REST_PORT",
          "value": "8000"
        },
        {
          "name": "GRPC_PORT",
          "value": "9000"
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/isobox",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

## Security Considerations

### 1. Docker Socket Security

The container needs access to the Docker socket to create execution containers. This is a security consideration:

```bash
# Use Docker socket proxy for better security
docker run -d \
  --name docker-socket-proxy \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -p 2375:2375 \
  tecnativa/docker-socket-proxy

# Use the proxy instead of direct socket access
docker run -d \
  --name isobox \
  -p 8000:8000 \
  -p 9000:9000 \
  -e DOCKER_HOST=tcp://docker-socket-proxy:2375 \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key" \
  isobox/isobox:latest
```

### 2. Network Security

```bash
# Create custom network
docker network create isobox-network

# Run with custom network
docker run -d \
  --name isobox \
  --network isobox-network \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key" \
  isobox/isobox:latest
```

### 3. Resource Limits

```bash
# Run with resource limits
docker run -d \
  --name isobox \
  --memory=2g \
  --cpus=2.0 \
  --pids-limit=100 \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="your-api-key" \
  isobox/isobox:latest
```

### 4. Secrets Management

```bash
# Use Docker secrets
echo "your-api-key-here" | docker secret create isobox-api-key -

# Run with secrets
docker run -d \
  --name isobox \
  --secret isobox-api-key \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS_FILE=/run/secrets/isobox-api-key \
  isobox/isobox:latest
```

## Troubleshooting

### Common Issues

#### 1. Docker Socket Permission Denied

```bash
# Fix Docker socket permissions
sudo chmod 666 /var/run/docker.sock

# Or add user to docker group
sudo usermod -aG docker $USER
newgrp docker
```

#### 2. Port Already in Use

```bash
# Check what's using the port
sudo lsof -i :8000

# Kill the process or use different ports
docker run -d \
  --name isobox \
  -p 8001:8000 \
  -p 9001:9000 \
  # ... other options
```

#### 3. Container Won't Start

```bash
# Check container logs
docker logs isobox

# Run in interactive mode for debugging
docker run -it \
  --name isobox-debug \
  -p 8000:8000 \
  -p 9000:9000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  -e AUTH_TYPE=apikey \
  -e API_KEYS="test-key" \
  -e RUST_LOG=debug \
  isobox/isobox:latest
```

#### 4. Authentication Issues

```bash
# Test authentication
curl -X POST http://localhost:8000/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-api-key" \
  -d '{"language": "python", "code": "print(\"test\")"}'

# Check authentication logs
docker logs isobox | grep -i auth
```

### Health Checks

```bash
# Check service health
curl http://localhost:8000/health

# Check container health
docker ps --filter "name=isobox"

# Check resource usage
docker stats isobox
```

### Logs and Monitoring

```bash
# View logs
docker logs -f isobox

# View logs with timestamps
docker logs -f -t isobox

# View last 100 lines
docker logs --tail 100 isobox

# Export logs
docker logs isobox > isobox.log
```

### Performance Monitoring

```bash
# Monitor container performance
docker stats isobox

# Check disk usage
docker system df

# Clean up unused resources
docker system prune -a
```

## Support

For issues related to container images:

- **Docker Hub**: [IsoBox on Docker Hub](https://hub.docker.com/r/isobox/isobox)
- **GitHub**: [IsoBox Repository](https://github.com/isobox/isobox)
- **Documentation**: [Full Documentation](https://github.com/isobox/isobox/docs)
- **Issues**: [GitHub Issues](https://github.com/isobox/isobox/issues)

---

For more information, see the [main README](README.md) and other documentation files.
