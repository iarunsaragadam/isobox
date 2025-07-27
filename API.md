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

- `language` (required): The programming language to use. See supported languages below.
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
| D        | `dlang2/dmd-ubuntu:latest`          | `.d`           | Yes                  |
| Fortran  | `gcc:latest`                        | `.f90`         | Yes                  |
| Pascal   | `fpc:latest`                        | `.pas`         | Yes                  |
| Assembly | `nasm:latest`                       | `.asm`         | Yes                  |
| COBOL    | `gnucobol:latest`                   | `.cob`         | Yes                  |
| Basic    | `freebasic/fbc:latest`              | `.bas`         | Yes                  |

### Functional Languages

| Language    | Docker Image                       | File Extension | Compilation Required |
| ----------- | ---------------------------------- | -------------- | -------------------- |
| Clojure     | `clojure:openjdk-17`               | `.clj`         | No                   |
| F#          | `mcr.microsoft.com/dotnet/sdk:7.0` | `.fsx`         | No                   |
| Elixir      | `elixir:1.15`                      | `.exs`         | No                   |
| Common Lisp | `daewok/lisp-devel:latest`         | `.lisp`        | No                   |
| Erlang      | `erlang:latest`                    | `.erl`         | No                   |

### Other Languages

| Language          | Docker Image                       | File Extension | Compilation Required |
| ----------------- | ---------------------------------- | -------------- | -------------------- |
| Dart              | `dart:stable`                      | `.dart`        | No                   |
| Groovy            | `openjdk:17`                       | `.groovy`      | No                   |
| Prolog            | `swipl:latest`                     | `.pl`          | No                   |
| Visual Basic .NET | `mcr.microsoft.com/dotnet/sdk:7.0` | `.vb`          | No                   |
| SQL               | `sqlite:latest`                    | `.sql`         | No                   |
| Objective-C       | `gcc:latest`                       | `.m`           | Yes                  |

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

### Rust Examples

#### Basic Rust

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "rust",
    "code": "fn main() {\n    println!(\"Hello from Rust!\");\n}"
  }'
```

#### Rust with Vector Operations

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "rust",
    "code": "fn main() {\n    let numbers: Vec<i32> = (1..=10).collect();\n    let sum: i32 = numbers.iter().sum();\n    println!(\"Sum of 1 to 10: {}\", sum);\n}"
  }'
```

### C Examples

#### Basic C

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "c",
    "code": "#include <stdio.h>\n\nint main() {\n    printf(\"Hello from C!\\n\");\n    return 0;\n}"
  }'
```

#### C with Math

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "c",
    "code": "#include <stdio.h>\n#include <math.h>\n\nint main() {\n    printf(\"π ≈ %.6f\\n\", M_PI);\n    printf(\"e ≈ %.6f\\n\", M_E);\n    return 0;\n}"
  }'
```

### C++ Examples

#### Basic C++

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "cpp",
    "code": "#include <iostream>\n#include <vector>\n\nint main() {\n    std::cout << \"Hello from C++!\" << std::endl;\n    return 0;\n}"
  }'
```

#### C++ with STL

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "cpp",
    "code": "#include <iostream>\n#include <vector>\n#include <algorithm>\n\nint main() {\n    std::vector<int> numbers = {3, 1, 4, 1, 5, 9, 2, 6};\n    std::sort(numbers.begin(), numbers.end());\n    for (int n : numbers) {\n        std::cout << n << \" \";\n    }\n    std::cout << std::endl;\n    return 0;\n}"
  }'
```

### Java Examples

#### Basic Java

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "java",
    "code": "public class Main {\n    public static void main(String[] args) {\n        System.out.println(\"Hello from Java!\");\n    }\n}"
  }'
```

#### Java with Collections

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "java",
    "code": "import java.util.*;\n\npublic class Main {\n    public static void main(String[] args) {\n        List<String> fruits = Arrays.asList(\"apple\", \"banana\", \"cherry\");\n        fruits.forEach(System.out::println);\n    }\n}"
  }'
```

### C# Examples

#### Basic C#

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "csharp",
    "code": "using System;\n\nclass Program {\n    static void Main() {\n        Console.WriteLine(\"Hello from C#!\");\n    }\n}"
  }'
```

#### C# with LINQ

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "csharp",
    "code": "using System;\nusing System.Linq;\n\nclass Program {\n    static void Main() {\n        var numbers = Enumerable.Range(1, 10);\n        var sum = numbers.Sum();\n        Console.WriteLine($\"Sum: {sum}\");\n    }\n}"
  }'
```

### PHP Examples

#### Basic PHP

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "php",
    "code": "<?php\necho \"Hello from PHP!\\n\";\n?>"
  }'
```

#### PHP with Arrays

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "php",
    "code": "<?php\n$fruits = [\"apple\", \"banana\", \"cherry\"];\nforeach ($fruits as $fruit) {\n    echo $fruit . \"\\n\";\n}\n?>"
  }'
```

### Ruby Examples

#### Basic Ruby

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "ruby",
    "code": "puts \"Hello from Ruby!\""
  }'
```

#### Ruby with Blocks

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "ruby",
    "code": "[1, 2, 3, 4, 5].each { |n| puts n * 2 }"
  }'
```

### Kotlin Examples

#### Basic Kotlin

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "kotlin",
    "code": "fun main() {\n    println(\"Hello from Kotlin!\")\n}"
  }'
```

#### Kotlin with Collections

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "kotlin",
    "code": "fun main() {\n    val numbers = listOf(1, 2, 3, 4, 5)\n    val doubled = numbers.map { it * 2 }\n    println(doubled)\n}"
  }'
```

### Swift Examples

#### Basic Swift

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "swift",
    "code": "print(\"Hello from Swift!\")"
  }'
```

#### Swift with Arrays

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "swift",
    "code": "let numbers = [1, 2, 3, 4, 5]\nlet doubled = numbers.map { $0 * 2 }\nprint(doubled)"
  }'
```

### Haskell Examples

#### Basic Haskell

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "haskell",
    "code": "main = putStrLn \"Hello from Haskell!\""
  }'
```

#### Haskell with Lists

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "haskell",
    "code": "main = do\n    let numbers = [1..10]\n    let doubled = map (*2) numbers\n    print doubled"
  }'
```

### TypeScript Examples

#### Basic TypeScript

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "typescript",
    "code": "interface Person {\n    name: string;\n    age: number;\n}\n\nconst person: Person = { name: \"Alice\", age: 30 };\nconsole.log(person);"
  }'
```

### R Examples

#### Basic R

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "r",
    "code": "cat(\"Hello from R!\\n\")"
  }'
```

#### R with Data Analysis

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "r",
    "code": "numbers <- 1:10\nmean_val <- mean(numbers)\ncat(\"Mean:\", mean_val, \"\\n\")"
  }'
```

### Bash Examples

#### Basic Bash

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "bash",
    "code": "echo \"Hello from Bash!\"\ndate"
  }'
```

### SQL Examples

#### Basic SQL

```bash
curl -X POST http://localhost:8000/execute \
  -H "Content-Type: application/json" \
  -d '{
    "language": "sql",
    "code": "CREATE TABLE users (id INTEGER, name TEXT);\nINSERT INTO users VALUES (1, \"Alice\");\nSELECT * FROM users;"
  }'
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
  "exit_code": 1
}
```

## Security Features

- **Network Isolation**: Containers run with `--network none`
- **Ephemeral Containers**: All containers are removed after execution (`--rm`)
- **Temporary Files**: Code files are written to unique temp directories and cleaned up
- **No Persistence**: No data persists between executions
- **Resource Limits**: Containers have limited access to system resources
- **Language-Specific Isolation**: Each language runs in its own optimized container

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

## Performance Considerations

- **First Run**: The first execution of each language may take longer as Docker images are pulled
- **Compilation**: Compiled languages (C, C++, Rust, Java, etc.) have an additional compilation step
- **Memory Usage**: Different languages have varying memory requirements
- **Execution Time**: Scripting languages typically start faster than compiled languages

## Development

For development setup and contributing guidelines, see the main [README.md](README.md) file.

## Language-Specific Notes

### Compiled Languages

- C, C++, Rust, Java, Kotlin, Swift, Scala, Haskell, OCaml, D, Fortran, Pascal, Assembly, COBOL, Basic, Objective-C require compilation
- Compilation errors are returned in the `stderr` field
- Successful compilation produces an executable that is then run

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
3. **Timeout Issues**: Some languages may take longer to start up
4. **Memory Issues**: Large programs may exceed container memory limits

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
