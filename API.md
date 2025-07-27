# Isobox API Documentation

Isobox is a secure code execution REST API that allows you to execute arbitrary code in isolated Docker containers. This document provides comprehensive API documentation.

## Base URL

```
http://localhost:8000
```

## Authentication

Currently, the API does not require authentication. All endpoints are publicly accessible.

## Endpoints

### 1. Health Check

**Endpoint:** `GET /health`

**Description:** Check if the service is running and healthy.

**Response:**

```json
{
  "service": "isobox",
  "status": "healthy",
  "version": "0.1.0"
}
```

**Example:**

```bash
curl http://localhost:8000/health
```

### 2. Execute Code

**Endpoint:** `POST /execute`

**Description:** Execute code in an isolated Docker container.

**Request Body:**

```json
{
  "language": "string",
  "code": "string"
}
```

**Parameters:**

- `language` (required): The programming language to use. Supported values: `python`, `node`, `go`
- `code` (required): The source code to execute

**Response:**

```json
{
  "stdout": "string",
  "stderr": "string",
  "exit_code": number
}
```

**Response Fields:**

- `stdout`: Standard output from the program execution
- `stderr`: Standard error output from the program execution
- `exit_code`: Program exit code (0 for success, non-zero for errors)

**Example:**

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Hello, World!\")"
  }'
```

## Supported Languages

| Language | Docker Image  | File Extension | Example Command  |
| -------- | ------------- | -------------- | ---------------- |
| Python   | `python:3.11` | `.py`          | `python main.py` |
| Node.js  | `node:20`     | `.js`          | `node main.js`   |
| Go       | `golang:1.21` | `.go`          | `go run main.go` |

## Examples

### Python Examples

#### Basic Python

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Hello from Python!\")"
  }'
```

#### Python with Math

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "import math\nprint(f\"π = {math.pi}\")\nprint(f\"e = {math.e}\")"
  }'
```

#### Python Error Handling

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Before error\")\nundefined_variable\nprint(\"After error\")"
  }'
```

### Node.js Examples

#### Basic Node.js

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "node",
    "code": "console.log(\"Hello from Node.js!\");"
  }'
```

#### Node.js with JSON

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "node",
    "code": "const data = {name: \"isobox\", version: \"0.1.0\"}; console.log(JSON.stringify(data, null, 2));"
  }'
```

### Go Examples

#### Basic Go

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "go",
    "code": "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"Hello from Go!\")\n}"
  }'
```

#### Go with Time

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "go",
    "code": "package main\n\nimport (\n    \"fmt\"\n    \"time\"\n)\n\nfunc main() {\n    fmt.Printf(\"Current time: %s\\n\", time.Now().Format(time.RFC3339))\n}"
  }'
```

#### Go with Error

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "go",
    "code": "package main\n\nimport \"fmt\"\n\nfunc main() {\n    fmt.Println(\"This will fail - missing import\")\n}"
  }'
```

## Error Responses

### Unsupported Language

```json
{
  "error": "Unsupported language: rust"
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

## Security Features

- **Network Isolation**: Containers run with `--network none`
- **Ephemeral Containers**: All containers are removed after execution (`--rm`)
- **Temporary Files**: Code files are written to unique temp directories and cleaned up
- **No Persistence**: No data persists between executions
- **Resource Limits**: Containers have limited access to system resources

## Rate Limiting

Currently, there are no rate limits implemented. However, it's recommended to implement rate limiting for production use.

## Environment Variables

The service can be configured using the following environment variables:

- `PORT`: Server port (default: 8000)
- `RUST_LOG`: Log level (default: info)

## Docker Deployment

To run the service using Docker:

```bash
docker run -p 8000:8000 \
  -v /var/run/docker.sock:/var/run/docker.sock \
  -v /tmp:/tmp \
  --user root \
  ghcr.io/yourusername/isobox:latest
```

## Development

For development setup and contributing guidelines, see the main [README.md](README.md) file.
