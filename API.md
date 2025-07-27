# Isobox API Documentation

Isobox is a secure code execution REST API that allows you to execute arbitrary code in isolated Docker containers with comprehensive resource limits and timeout protection. This document provides comprehensive API documentation.

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

**Description:** Execute code in an isolated Docker container with resource limits and timeout protection.

**Request Body:**

```json
{
  "language": "string",
  "code": "string"
}
```

**Parameters:**

- `language` (required): The programming language to use. See supported languages below.
- `code` (required): The source code to execute

**Response:**

```json
{
  "stdout": "string",
  "stderr": "string",
  "exit_code": number,
  "time_taken": number,
  "memory_used": number
}
```

**Response Fields:**

- `stdout`: Standard output from the program execution
- `stderr`: Standard error output from the program execution
- `exit_code`: Program exit code (0 for success, non-zero for errors)
- `time_taken`: Execution time in seconds (if available)
- `memory_used`: Memory usage in bytes (if available)

**Example:**

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "python",
    "code": "print(\"Hello, World!\")"
  }'
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

### Python Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
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
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "rust",
    "code": "fn main() {\n    let numbers: Vec<i32> = (1..=10).collect();\n    let sum: i32 = numbers.iter().sum();\n    println!(\"Sum of 1 to 10: {}\", sum);\n}"
  }'
```

### Java Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "java",
    "code": "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello from Java!\");\n    }\n}"
  }'
```

### Go Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "go",
    "code": "package main\n\nimport (\n    \"fmt\"\n    \"time\"\n)\n\nfunc main() {\n    fmt.Println(\"Hello from Go!\")\n    fmt.Printf(\"Current time: %s\\n\", time.Now().Format(time.RFC3339))\n}"
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

### C++ Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "cpp",
    "code": "#include <iostream>\n#include <vector>\n#include <algorithm>\n\nint main() {\n    std::vector<int> numbers = {3, 1, 4, 1, 5, 9, 2, 6};\n    std::sort(numbers.begin(), numbers.end());\n    for (int n : numbers) {\n        std::cout << n << \" \";\n    }\n    std::cout << std::endl;\n    return 0;\n}"
  }'
```

### Timeout Example

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
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
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
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

## Performance Considerations

- **First Run**: The first execution of each language may take longer as Docker images are pulled
- **Compilation**: Compiled languages (C, C++, Rust, Java, etc.) have an additional compilation step
- **Memory Usage**: Different languages have varying memory requirements
- **Execution Time**: Scripting languages typically start faster than compiled languages
- **Resource Limits**: All executions are subject to timeout and memory constraints
- **Container Overhead**: Each execution creates a new isolated container

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

1. **Language Not Supported**: Check the supported languages list above
2. **Compilation Errors**: Review the `stderr` output for specific error messages
3. **Timeout Issues**: Programs may be terminated if they exceed the time limit
4. **Memory Issues**: Large programs may exceed container memory limits
5. **Resource Limits**: Check if your program is hitting CPU, memory, or process limits

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

## Rate Limiting

Currently, there are no rate limits implemented. However, it's recommended to implement rate limiting for production use.

---

For more information, see the main [README.md](README.md) file.
