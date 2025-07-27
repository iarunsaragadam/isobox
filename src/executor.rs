use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct ExecuteRequest {
    pub language: String,
    pub code: String,
}

#[derive(Debug, Serialize)]
pub struct ExecuteResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub struct CodeExecutor {
    language_configs: HashMap<String, LanguageConfig>,
}

#[derive(Clone)]
struct LanguageConfig {
    docker_image: String,
    file_name: String,
    run_command: Vec<String>,
    compile_command: Option<Vec<String>>,
}

impl CodeExecutor {
    pub fn new() -> Self {
        let mut language_configs = HashMap::new();
        
        // Python configurations
        language_configs.insert(
            "python".to_string(),
            LanguageConfig {
                docker_image: "python:3.11".to_string(),
                file_name: "main.py".to_string(),
                run_command: vec!["python".to_string(), "main.py".to_string()],
                compile_command: None,
            },
        );
        
        language_configs.insert(
            "python2".to_string(),
            LanguageConfig {
                docker_image: "python:2.7".to_string(),
                file_name: "main.py".to_string(),
                run_command: vec!["python".to_string(), "main.py".to_string()],
                compile_command: None,
            },
        );
        
        // Node.js configuration
        language_configs.insert(
            "node".to_string(),
            LanguageConfig {
                docker_image: "node:20".to_string(),
                file_name: "main.js".to_string(),
                run_command: vec!["node".to_string(), "main.js".to_string()],
                compile_command: None,
            },
        );
        
        // Go configuration
        language_configs.insert(
            "go".to_string(),
            LanguageConfig {
                docker_image: "golang:1.21".to_string(),
                file_name: "main.go".to_string(),
                run_command: vec!["go".to_string(), "run".to_string(), "main.go".to_string()],
                compile_command: None,
            },
        );
        
        // Rust configuration
        language_configs.insert(
            "rust".to_string(),
            LanguageConfig {
                docker_image: "rust:latest".to_string(),
                file_name: "main.rs".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["rustc".to_string(), "main.rs".to_string()]),
            },
        );
        
        // C configuration
        language_configs.insert(
            "c".to_string(),
            LanguageConfig {
                docker_image: "gcc:latest".to_string(),
                file_name: "main.c".to_string(),
                run_command: vec!["./a.out".to_string()],
                compile_command: Some(vec!["gcc".to_string(), "main.c".to_string()]),
            },
        );
        
        // C++ configuration
        language_configs.insert(
            "cpp".to_string(),
            LanguageConfig {
                docker_image: "gcc:latest".to_string(),
                file_name: "main.cpp".to_string(),
                run_command: vec!["./a.out".to_string()],
                compile_command: Some(vec!["g++".to_string(), "main.cpp".to_string()]),
            },
        );
        
        // Java configuration
        language_configs.insert(
            "java".to_string(),
            LanguageConfig {
                docker_image: "openjdk:17".to_string(),
                file_name: "Main.java".to_string(),
                run_command: vec!["java".to_string(), "Main".to_string()],
                compile_command: Some(vec!["javac".to_string(), "Main.java".to_string()]),
            },
        );
        
        // C# configuration
        language_configs.insert(
            "csharp".to_string(),
            LanguageConfig {
                docker_image: "mcr.microsoft.com/dotnet/sdk:7.0".to_string(),
                file_name: "Program.cs".to_string(),
                run_command: vec!["dotnet".to_string(), "run".to_string()],
                compile_command: None,
            },
        );
        
        // PHP configuration
        language_configs.insert(
            "php".to_string(),
            LanguageConfig {
                docker_image: "php:8.2".to_string(),
                file_name: "main.php".to_string(),
                run_command: vec!["php".to_string(), "main.php".to_string()],
                compile_command: None,
            },
        );
        
        // Ruby configuration
        language_configs.insert(
            "ruby".to_string(),
            LanguageConfig {
                docker_image: "ruby:3.2".to_string(),
                file_name: "main.rb".to_string(),
                run_command: vec!["ruby".to_string(), "main.rb".to_string()],
                compile_command: None,
            },
        );
        
        // Kotlin configuration
        language_configs.insert(
            "kotlin".to_string(),
            LanguageConfig {
                docker_image: "openjdk:17".to_string(),
                file_name: "Main.kt".to_string(),
                run_command: vec!["kotlin".to_string(), "MainKt".to_string()],
                compile_command: Some(vec!["kotlinc".to_string(), "Main.kt".to_string()]),
            },
        );
        
        // Swift configuration
        language_configs.insert(
            "swift".to_string(),
            LanguageConfig {
                docker_image: "swift:5.9".to_string(),
                file_name: "main.swift".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["swiftc".to_string(), "main.swift".to_string()]),
            },
        );
        
        // Scala configuration
        language_configs.insert(
            "scala".to_string(),
            LanguageConfig {
                docker_image: "openjdk:17".to_string(),
                file_name: "Main.scala".to_string(),
                run_command: vec!["scala".to_string(), "Main".to_string()],
                compile_command: Some(vec!["scalac".to_string(), "Main.scala".to_string()]),
            },
        );
        
        // Haskell configuration
        language_configs.insert(
            "haskell".to_string(),
            LanguageConfig {
                docker_image: "haskell:9.4".to_string(),
                file_name: "main.hs".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["ghc".to_string(), "main.hs".to_string()]),
            },
        );
        
        // Lua configuration
        language_configs.insert(
            "lua".to_string(),
            LanguageConfig {
                docker_image: "lua:5.4".to_string(),
                file_name: "main.lua".to_string(),
                run_command: vec!["lua".to_string(), "main.lua".to_string()],
                compile_command: None,
            },
        );
        
        // Perl configuration
        language_configs.insert(
            "perl".to_string(),
            LanguageConfig {
                docker_image: "perl:5.38".to_string(),
                file_name: "main.pl".to_string(),
                run_command: vec!["perl".to_string(), "main.pl".to_string()],
                compile_command: None,
            },
        );
        
        // R configuration
        language_configs.insert(
            "r".to_string(),
            LanguageConfig {
                docker_image: "r-base:latest".to_string(),
                file_name: "main.r".to_string(),
                run_command: vec!["Rscript".to_string(), "main.r".to_string()],
                compile_command: None,
            },
        );
        
        // Bash configuration
        language_configs.insert(
            "bash".to_string(),
            LanguageConfig {
                docker_image: "bash:latest".to_string(),
                file_name: "main.sh".to_string(),
                run_command: vec!["bash".to_string(), "main.sh".to_string()],
                compile_command: None,
            },
        );
        
        // TypeScript configuration
        language_configs.insert(
            "typescript".to_string(),
            LanguageConfig {
                docker_image: "node:20".to_string(),
                file_name: "main.ts".to_string(),
                run_command: vec!["node".to_string(), "main.js".to_string()],
                compile_command: Some(vec!["npx".to_string(), "tsc".to_string(), "main.ts".to_string()]),
            },
        );
        
        // Dart configuration
        language_configs.insert(
            "dart".to_string(),
            LanguageConfig {
                docker_image: "dart:stable".to_string(),
                file_name: "main.dart".to_string(),
                run_command: vec!["dart".to_string(), "main.dart".to_string()],
                compile_command: None,
            },
        );
        
        // Elixir configuration
        language_configs.insert(
            "elixir".to_string(),
            LanguageConfig {
                docker_image: "elixir:1.15".to_string(),
                file_name: "main.exs".to_string(),
                run_command: vec!["elixir".to_string(), "main.exs".to_string()],
                compile_command: None,
            },
        );
        
        // Clojure configuration
        language_configs.insert(
            "clojure".to_string(),
            LanguageConfig {
                docker_image: "clojure:openjdk-17".to_string(),
                file_name: "main.clj".to_string(),
                run_command: vec!["clojure".to_string(), "main.clj".to_string()],
                compile_command: None,
            },
        );
        
        // F# configuration
        language_configs.insert(
            "fsharp".to_string(),
            LanguageConfig {
                docker_image: "mcr.microsoft.com/dotnet/sdk:7.0".to_string(),
                file_name: "main.fsx".to_string(),
                run_command: vec!["dotnet".to_string(), "fsi".to_string(), "main.fsx".to_string()],
                compile_command: None,
            },
        );
        
        // Groovy configuration
        language_configs.insert(
            "groovy".to_string(),
            LanguageConfig {
                docker_image: "openjdk:17".to_string(),
                file_name: "main.groovy".to_string(),
                run_command: vec!["groovy".to_string(), "main.groovy".to_string()],
                compile_command: None,
            },
        );
        
        // OCaml configuration
        language_configs.insert(
            "ocaml".to_string(),
            LanguageConfig {
                docker_image: "ocaml/opam:ubuntu-22.04-ocaml-5.0".to_string(),
                file_name: "main.ml".to_string(),
                run_command: vec!["./a.out".to_string()],
                compile_command: Some(vec!["ocamlc".to_string(), "main.ml".to_string()]),
            },
        );
        
        // D configuration
        language_configs.insert(
            "d".to_string(),
            LanguageConfig {
                docker_image: "dlang2/dmd-ubuntu:latest".to_string(),
                file_name: "main.d".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["dmd".to_string(), "main.d".to_string()]),
            },
        );
        
        // Fortran configuration
        language_configs.insert(
            "fortran".to_string(),
            LanguageConfig {
                docker_image: "gcc:latest".to_string(),
                file_name: "main.f90".to_string(),
                run_command: vec!["./a.out".to_string()],
                compile_command: Some(vec!["gfortran".to_string(), "main.f90".to_string()]),
            },
        );
        
        // Pascal configuration
        language_configs.insert(
            "pascal".to_string(),
            LanguageConfig {
                docker_image: "fpc:latest".to_string(),
                file_name: "main.pas".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["fpc".to_string(), "main.pas".to_string()]),
            },
        );
        
        // Assembly (NASM) configuration
        language_configs.insert(
            "assembly".to_string(),
            LanguageConfig {
                docker_image: "nasm:latest".to_string(),
                file_name: "main.asm".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["nasm".to_string(), "-f".to_string(), "elf64".to_string(), "main.asm".to_string(), "&&".to_string(), "ld".to_string(), "main.o".to_string(), "-o".to_string(), "main".to_string()]),
            },
        );
        
        // COBOL configuration
        language_configs.insert(
            "cobol".to_string(),
            LanguageConfig {
                docker_image: "gnucobol:latest".to_string(),
                file_name: "main.cob".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["cobc".to_string(), "-free".to_string(), "-x".to_string(), "main.cob".to_string()]),
            },
        );
        
        // Prolog configuration
        language_configs.insert(
            "prolog".to_string(),
            LanguageConfig {
                docker_image: "swipl:latest".to_string(),
                file_name: "main.pl".to_string(),
                run_command: vec!["swipl".to_string(), "main.pl".to_string()],
                compile_command: None,
            },
        );
        
        // Octave configuration
        language_configs.insert(
            "octave".to_string(),
            LanguageConfig {
                docker_image: "octave/octave:latest".to_string(),
                file_name: "main.m".to_string(),
                run_command: vec!["octave".to_string(), "--no-gui".to_string(), "main.m".to_string()],
                compile_command: None,
            },
        );
        
        // Basic configuration
        language_configs.insert(
            "basic".to_string(),
            LanguageConfig {
                docker_image: "freebasic/fbc:latest".to_string(),
                file_name: "main.bas".to_string(),
                run_command: vec!["./main".to_string()],
                compile_command: Some(vec!["fbc".to_string(), "main.bas".to_string()]),
            },
        );
        
        // Erlang configuration
        language_configs.insert(
            "erlang".to_string(),
            LanguageConfig {
                docker_image: "erlang:latest".to_string(),
                file_name: "main.erl".to_string(),
                run_command: vec!["escript".to_string(), "main.erl".to_string()],
                compile_command: None,
            },
        );
        
        // Common Lisp configuration
        language_configs.insert(
            "lisp".to_string(),
            LanguageConfig {
                docker_image: "daewok/lisp-devel:latest".to_string(),
                file_name: "main.lisp".to_string(),
                run_command: vec!["sbcl".to_string(), "--script".to_string(), "main.lisp".to_string()],
                compile_command: None,
            },
        );
        
        // Visual Basic .NET configuration
        language_configs.insert(
            "vbnet".to_string(),
            LanguageConfig {
                docker_image: "mcr.microsoft.com/dotnet/sdk:7.0".to_string(),
                file_name: "Program.vb".to_string(),
                run_command: vec!["dotnet".to_string(), "run".to_string()],
                compile_command: None,
            },
        );
        
        // SQL configuration
        language_configs.insert(
            "sql".to_string(),
            LanguageConfig {
                docker_image: "sqlite:latest".to_string(),
                file_name: "main.sql".to_string(),
                run_command: vec!["sqlite3".to_string(), "database.db".to_string(), "<".to_string(), "main.sql".to_string()],
                compile_command: None,
            },
        );
        
        // Objective-C configuration
        language_configs.insert(
            "objc".to_string(),
            LanguageConfig {
                docker_image: "gcc:latest".to_string(),
                file_name: "main.m".to_string(),
                run_command: vec!["./a.out".to_string()],
                compile_command: Some(vec!["clang".to_string(), "-framework".to_string(), "Foundation".to_string(), "main.m".to_string()]),
            },
        );
        
        Self { language_configs }
    }
    
    pub async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, String> {
        let config = self
            .language_configs
            .get(&request.language)
            .ok_or_else(|| format!("Unsupported language: {}", request.language))?;
        
        // Generate unique job ID
        let job_id = Uuid::new_v4().to_string();
        let temp_dir = format!("/tmp/isobox-{}", job_id);
        
        log::info!("Creating temp directory: {}", temp_dir);
        
        // Create temp directory
        match fs::create_dir_all(&temp_dir) {
            Ok(_) => {
                log::info!("Successfully created temp directory: {}", temp_dir);
            }
            Err(e) => {
                log::error!("Failed to create temp directory: {}", e);
                return Err(format!("Failed to create temp directory: {}", e));
            }
        }
        
        // Ensure cleanup happens even if execution fails
        let result = self.execute_in_container(&temp_dir, config, &request.code).await;
        
        // Clean up temp directory
        log::info!("Cleaning up temp directory: {}", temp_dir);
        if let Err(e) = fs::remove_dir_all(&temp_dir) {
            log::warn!("Failed to clean up temp directory {}: {}", temp_dir, e);
        } else {
            log::info!("Successfully cleaned up temp directory: {}", temp_dir);
        }
        
        result
    }
    
    async fn execute_in_container(
        &self,
        temp_dir: &str,
        config: &LanguageConfig,
        code: &str,
    ) -> Result<ExecuteResponse, String> {
        // Write code to file
        let file_path = format!("{}/{}", temp_dir, config.file_name);
        
        log::info!("Writing code to file: {}", file_path);
        log::info!("Code content length: {} bytes", code.len());
        
        // Verify temp directory exists and is writable
        if !std::path::Path::new(temp_dir).exists() {
            return Err(format!("Temp directory does not exist: {}", temp_dir));
        }
        
        // Write the file with detailed error handling
        match fs::write(&file_path, code) {
            Ok(_) => {
                log::info!("Successfully wrote code to file: {}", file_path);
                
                // Verify the file was actually written
                match fs::metadata(&file_path) {
                    Ok(metadata) => {
                        log::info!("File created successfully. Size: {} bytes", metadata.len());
                    }
                    Err(e) => {
                        log::error!("Failed to verify file creation: {}", e);
                        return Err(format!("File verification failed: {}", e));
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to write code file: {}", e);
                return Err(format!("Failed to write code file: {}", e));
            }
        }
        
        // If compilation is needed, compile first
        if let Some(compile_cmd) = &config.compile_command {
            log::info!("Compiling with: {}", compile_cmd.join(" "));
            
            let mut docker_compile_args = vec![
                "run".to_string(),
                "--rm".to_string(),
                "--network".to_string(),
                "none".to_string(),
                "-v".to_string(),
                format!("{}:/workspace", temp_dir),
                "-w".to_string(),
                "/workspace".to_string(),
                config.docker_image.clone(),
            ];
            docker_compile_args.extend(compile_cmd.clone());
            
            let compile_output = Command::new("docker")
                .args(&docker_compile_args)
                .output()
                .map_err(|e| format!("Failed to execute compilation: {}", e))?;
            
            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr);
                return Ok(ExecuteResponse {
                    stdout: String::new(),
                    stderr: stderr.to_string(),
                    exit_code: compile_output.status.code().unwrap_or(1),
                });
            }
        }
        
        // Build docker command for execution
        let mut docker_args = vec![
            "run".to_string(),
            "--rm".to_string(),
            "--network".to_string(),
            "none".to_string(),
            "-v".to_string(),
            format!("{}:/workspace", temp_dir),
            "-w".to_string(),
            "/workspace".to_string(),
            config.docker_image.clone(),
        ];
        docker_args.extend(config.run_command.clone());
        
        log::info!("Executing: docker {}", docker_args.join(" "));
        
        // Execute docker command
        let output = Command::new("docker")
            .args(&docker_args)
            .output()
            .map_err(|e| format!("Failed to execute code: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(1);
        
        log::info!(
            "Execution completed: exit_code={}, stdout_len={}, stderr_len={}",
            exit_code,
            stdout.len(),
            stderr.len()
        );
        
        Ok(ExecuteResponse {
            stdout,
            stderr,
            exit_code,
        })
    }
}

impl Default for CodeExecutor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        let executor = CodeExecutor::new();
        
        // Verify that major languages are supported
        assert!(executor.language_configs.contains_key("python"));
        assert!(executor.language_configs.contains_key("node"));
        assert!(executor.language_configs.contains_key("go"));
        assert!(executor.language_configs.contains_key("rust"));
        assert!(executor.language_configs.contains_key("c"));
        assert!(executor.language_configs.contains_key("cpp"));
        assert!(executor.language_configs.contains_key("java"));
        assert!(executor.language_configs.contains_key("csharp"));
        assert!(executor.language_configs.contains_key("php"));
        assert!(executor.language_configs.contains_key("ruby"));
        
        // Verify configuration details for a few languages
        let python_config = executor.language_configs.get("python").unwrap();
        assert_eq!(python_config.docker_image, "python:3.11");
        assert_eq!(python_config.file_name, "main.py");
        
        let rust_config = executor.language_configs.get("rust").unwrap();
        assert_eq!(rust_config.docker_image, "rust:latest");
        assert_eq!(rust_config.file_name, "main.rs");
        assert!(rust_config.compile_command.is_some());
        
        let java_config = executor.language_configs.get("java").unwrap();
        assert_eq!(java_config.docker_image, "openjdk:17");
        assert_eq!(java_config.file_name, "Main.java");
        assert!(java_config.compile_command.is_some());
    }

    #[test]
    fn test_unsupported_language() {
        let executor = CodeExecutor::new();
        let request = ExecuteRequest {
            language: "unsupported".to_string(),
            code: "test".to_string(),
        };
        
        let runtime = tokio::runtime::Runtime::new().unwrap();
        let result = runtime.block_on(executor.execute(request));
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unsupported language"));
    }
}
