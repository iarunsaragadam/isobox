use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::time::Duration;
use tokio::time::timeout;
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
    pub time_taken: Option<f64>,
    pub memory_used: Option<u64>,
}

// Resource limits configuration inspired by Judge0
#[derive(Clone, Debug)]
pub struct ResourceLimits {
    pub cpu_time_limit: Duration,
    pub wall_time_limit: Duration,
    pub memory_limit: u64, // in bytes
    pub stack_limit: u64,  // in bytes
    pub max_processes: u32,
    pub max_files: u32,
    pub enable_network: bool,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            // Judge0-inspired defaults
            cpu_time_limit: Duration::from_secs(5),    // 5 seconds CPU time
            wall_time_limit: Duration::from_secs(10),  // 10 seconds wall time
            memory_limit: 128 * 1024 * 1024,           // 128 MB
            stack_limit: 64 * 1024 * 1024,             // 64 MB stack
            max_processes: 50,                         // Max 50 processes
            max_files: 100,                            // Max 100 open files
            enable_network: false,                     // No network access
        }
    }
}

// Docker command builder for consistent container execution
#[derive(Debug)]
struct DockerCommandBuilder {
    args: Vec<String>,
}

impl DockerCommandBuilder {
    fn new() -> Self {
        Self {
            args: vec![
                "run".to_string(),
                "--rm".to_string(),
            ],
        }
    }

    fn with_network_isolation(mut self) -> Self {
        self.args.extend(vec![
            "--network".to_string(),
            "none".to_string(),
        ]);
        self
    }

    fn with_volume_mount(mut self, host_path: &str, container_path: &str) -> Self {
        self.args.extend(vec![
            "-v".to_string(),
            format!("{}:{}", host_path, container_path),
        ]);
        self
    }

    fn with_working_directory(mut self, work_dir: &str) -> Self {
        self.args.extend(vec![
            "-w".to_string(),
            work_dir.to_string(),
        ]);
        self
    }

    fn with_resource_limits(mut self, limits: &ResourceLimits) -> Self {
        // Memory limit
        self.args.extend(vec![
            "--memory".to_string(),
            format!("{}b", limits.memory_limit),
        ]);

        // CPU time limit (using ulimit)
        self.args.extend(vec![
            "--ulimit".to_string(),
            format!("cpu={}:{}", limits.cpu_time_limit.as_secs(), limits.cpu_time_limit.as_secs()),
        ]);

        // Stack limit
        self.args.extend(vec![
            "--ulimit".to_string(),
            format!("stack={}:{}", limits.stack_limit, limits.stack_limit),
        ]);

        // Process limit
        self.args.extend(vec![
            "--ulimit".to_string(),
            format!("nproc={}:{}", limits.max_processes, limits.max_processes),
        ]);

        // File descriptor limit
        self.args.extend(vec![
            "--ulimit".to_string(),
            format!("nofile={}:{}", limits.max_files, limits.max_files),
        ]);

        // Network access control
        if !limits.enable_network {
            self.args.push("--network".to_string());
            self.args.push("none".to_string());
        }

        // Additional security measures
        self.args.extend(vec![
            "--security-opt".to_string(),
            "no-new-privileges".to_string(),
            "--cap-drop".to_string(),
            "ALL".to_string(),
        ]);

        self
    }

    fn with_image(mut self, image: &str) -> Self {
        self.args.push(image.to_string());
        self
    }

    fn with_command(mut self, command: &[String]) -> Self {
        self.args.extend(command.to_vec());
        self
    }

    fn build(self) -> Vec<String> {
        self.args
    }
}

// Error types for better error handling
#[derive(Debug, thiserror::Error)]
pub enum ExecutionError {
    #[error("Unsupported language: {0}")]
    UnsupportedLanguage(String),
    #[error("Failed to create temp directory: {0}")]
    TempDirectoryCreation(String),
    #[error("Failed to write code file: {0}")]
    FileWrite(String),
    #[error("Failed to execute code: {0}")]
    Execution(String),
    #[error("Task join error: {0}")]
    TaskJoin(String),
    #[error("Execution timed out after {0:.3} seconds")]
    Timeout(f64),
}

// Language configuration
#[derive(Clone)]
struct LanguageConfig {
    docker_image: String,
    file_name: String,
    run_command: Vec<String>,
    compile_command: Option<Vec<String>>,
    // Language-specific resource limits (can override defaults)
    resource_limits: Option<ResourceLimits>,
}

// Trait for language configuration
trait LanguageConfigTrait {
    fn docker_image(&self) -> &str;
    fn file_name(&self) -> &str;
    fn run_command(&self) -> &[String];
    fn compile_command(&self) -> Option<&[String]>;
    fn resource_limits(&self) -> Option<&ResourceLimits>;
}

impl LanguageConfigTrait for LanguageConfig {
    fn docker_image(&self) -> &str {
        &self.docker_image
    }

    fn file_name(&self) -> &str {
        &self.file_name
    }

    fn run_command(&self) -> &[String] {
        &self.run_command
    }

    fn compile_command(&self) -> Option<&[String]> {
        self.compile_command.as_deref()
    }

    fn resource_limits(&self) -> Option<&ResourceLimits> {
        self.resource_limits.as_ref()
    }
}

// Language registry for managing supported languages
struct LanguageRegistry {
    languages: HashMap<String, LanguageConfig>,
}

impl LanguageRegistry {
    fn new() -> Self {
        let mut languages = HashMap::new();
        
        // Register all supported languages
        Self::register_scripting_languages(&mut languages);
        Self::register_compiled_languages(&mut languages);
        Self::register_functional_languages(&mut languages);
        Self::register_other_languages(&mut languages);
        
        Self { languages }
    }

    fn register_scripting_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let scripting_languages = vec![
            ("python", "python:3.11", "main.py", vec!["python".to_string(), "main.py".to_string()]),
            ("python2", "python:2.7", "main.py", vec!["python".to_string(), "main.py".to_string()]),
            ("node", "node:20", "main.js", vec!["node".to_string(), "main.js".to_string()]),
            ("php", "php:8.2", "main.php", vec!["php".to_string(), "main.php".to_string()]),
            ("ruby", "ruby:3.2", "main.rb", vec!["ruby".to_string(), "main.rb".to_string()]),
            ("perl", "perl:5.38", "main.pl", vec!["perl".to_string(), "main.pl".to_string()]),
            ("bash", "bash:latest", "main.sh", vec!["bash".to_string(), "main.sh".to_string()]),
            ("lua", "lua:5.4", "main.lua", vec!["lua".to_string(), "main.lua".to_string()]),
            ("r", "r-base:latest", "main.r", vec!["Rscript".to_string(), "main.r".to_string()]),
            ("octave", "octave/octave:latest", "main.m", vec!["octave".to_string(), "--no-gui".to_string(), "main.m".to_string()]),
            ("dart", "dart:stable", "main.dart", vec!["dart".to_string(), "main.dart".to_string()]),
            ("groovy", "openjdk:17", "main.groovy", vec!["groovy".to_string(), "main.groovy".to_string()]),
            ("prolog", "swipl:latest", "main.pl", vec!["swipl".to_string(), "main.pl".to_string()]),
            ("basic", "basic:latest", "main.bas", vec!["basic".to_string(), "main.bas".to_string()]),
        ];

        for (name, image, file, run_cmd) in scripting_languages {
            languages.insert(name.to_string(), LanguageConfig {
                docker_image: image.to_string(),
                file_name: file.to_string(),
                run_command: run_cmd,
                compile_command: None,
                resource_limits: None,
            });
        }
    }

    fn register_compiled_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let compiled_languages = vec![
            ("rust", "rust:latest", "main.rs", vec!["./main".to_string()], Some(vec!["rustc".to_string(), "main.rs".to_string()])),
            ("c", "gcc:latest", "main.c", vec!["./a.out".to_string()], Some(vec!["gcc".to_string(), "main.c".to_string()])),
            ("cpp", "gcc:latest", "main.cpp", vec!["./a.out".to_string()], Some(vec!["g++".to_string(), "main.cpp".to_string()])),
            ("java", "openjdk:17", "Main.java", vec!["java".to_string(), "Main".to_string()], Some(vec!["javac".to_string(), "Main.java".to_string()])),
            ("kotlin", "openjdk:17", "Main.kt", vec!["kotlin".to_string(), "MainKt".to_string()], Some(vec!["kotlinc".to_string(), "Main.kt".to_string()])),
            ("swift", "swift:5.9", "main.swift", vec!["./main".to_string()], Some(vec!["swiftc".to_string(), "main.swift".to_string()])),
            ("scala", "openjdk:17", "Main.scala", vec!["scala".to_string(), "Main".to_string()], Some(vec!["scalac".to_string(), "Main.scala".to_string()])),
            ("haskell", "haskell:9.4", "main.hs", vec!["./main".to_string()], Some(vec!["ghc".to_string(), "main.hs".to_string()])),
            ("ocaml", "ocaml/opam:ubuntu-22.04-ocaml-5.0", "main.ml", vec!["./a.out".to_string()], Some(vec!["ocamlc".to_string(), "main.ml".to_string()])),
            ("d", "dlang2/dmd-ubuntu:latest", "main.d", vec!["./main".to_string()], Some(vec!["dmd".to_string(), "main.d".to_string()])),
            ("fortran", "gcc:latest", "main.f90", vec!["./a.out".to_string()], Some(vec!["gfortran".to_string(), "main.f90".to_string()])),
            ("pascal", "fpc:latest", "main.pas", vec!["./main".to_string()], Some(vec!["fpc".to_string(), "main.pas".to_string()])),
            ("assembly", "nasm:latest", "main.asm", vec!["./main".to_string()], Some(vec!["nasm".to_string(), "-f".to_string(), "elf64".to_string(), "main.asm".to_string(), "&&".to_string(), "ld".to_string(), "main.o".to_string(), "-o".to_string(), "main".to_string()])),
            ("cobol", "gnucobol:latest", "main.cob", vec!["./main".to_string()], Some(vec!["cobc".to_string(), "-free".to_string(), "-x".to_string(), "main.cob".to_string()])),
            ("objective-c", "gcc:latest", "main.m", vec!["./main".to_string()], Some(vec!["gcc".to_string(), "-framework".to_string(), "Foundation".to_string(), "main.m".to_string(), "-o".to_string(), "main".to_string()])),
        ];

        for (name, image, file, run_cmd, compile_cmd) in compiled_languages {
            languages.insert(name.to_string(), LanguageConfig {
                docker_image: image.to_string(),
                file_name: file.to_string(),
                run_command: run_cmd,
                compile_command: compile_cmd,
                resource_limits: None,
            });
        }
    }

    fn register_functional_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let functional_languages = vec![
            ("clojure", "clojure:latest", "main.clj", vec!["clojure".to_string(), "main.clj".to_string()]),
            ("elixir", "elixir:1.15", "main.exs", vec!["elixir".to_string(), "main.exs".to_string()]),
            ("common-lisp", "sbcl:latest", "main.lisp", vec!["sbcl".to_string(), "--script".to_string(), "main.lisp".to_string()]),
            ("erlang", "erlang:latest", "main.erl", vec!["escript".to_string(), "main.erl".to_string()]),
        ];

        for (name, image, file, run_cmd) in functional_languages {
            languages.insert(name.to_string(), LanguageConfig {
                docker_image: image.to_string(),
                file_name: file.to_string(),
                run_command: run_cmd,
                compile_command: None,
                resource_limits: None,
            });
        }
    }

    fn register_other_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let other_languages = vec![
            ("go", "golang:1.21", "main.go", vec!["go".to_string(), "run".to_string(), "main.go".to_string()], None),
            ("csharp", "mcr.microsoft.com/dotnet/sdk:7.0", "Program.cs", vec!["dotnet".to_string(), "run".to_string()], None),
            ("fsharp", "mcr.microsoft.com/dotnet/sdk:7.0", "Program.fs", vec!["dotnet".to_string(), "run".to_string()], None),
            ("vbnet", "mcr.microsoft.com/dotnet/sdk:7.0", "Program.vb", vec!["dotnet".to_string(), "run".to_string()], None),
            ("typescript", "node:20", "main.ts", vec!["node".to_string(), "main.js".to_string()], Some(vec!["npx".to_string(), "tsc".to_string(), "main.ts".to_string()])),
            ("sql", "sqlite:latest", "main.sql", vec!["sqlite3".to_string(), "database.db".to_string(), ".read".to_string(), "main.sql".to_string()], None),
        ];

        for (name, image, file, run_cmd, compile_cmd) in other_languages {
            languages.insert(name.to_string(), LanguageConfig {
                docker_image: image.to_string(),
                file_name: file.to_string(),
                run_command: run_cmd,
                compile_command: compile_cmd,
                resource_limits: None,
            });
        }
    }

    fn get_language_config(&self, language: &str) -> Option<&LanguageConfig> {
        self.languages.get(language)
    }
}

// File manager for handling temporary files
struct FileManager;

impl FileManager {
    fn create_temp_directory(job_id: &str) -> Result<String, ExecutionError> {
        let temp_dir = format!("/tmp/isobox-{}", job_id);
        
        log::info!("Creating temp directory: {}", temp_dir);
        
        fs::create_dir_all(&temp_dir)
            .map_err(|e| ExecutionError::TempDirectoryCreation(e.to_string()))?;
        
        log::info!("Successfully created temp directory: {}", temp_dir);
        Ok(temp_dir)
    }

    fn write_code_file(temp_dir: &str, file_name: &str, code: &str) -> Result<(), ExecutionError> {
        let file_path = format!("{}/{}", temp_dir, file_name);
        
        log::info!("Writing code to file: {}", file_path);
        log::info!("Code content length: {} bytes", code.len());
        
        // Verify temp directory exists and is writable
        if !std::path::Path::new(temp_dir).exists() {
            return Err(ExecutionError::TempDirectoryCreation(format!("Temp directory does not exist: {}", temp_dir)));
        }
        
        fs::write(&file_path, code)
            .map_err(|e| ExecutionError::FileWrite(e.to_string()))?;
        
        // Verify the file was actually written
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                log::info!("File created successfully. Size: {} bytes", metadata.len());
            }
            Err(e) => {
                log::error!("Failed to verify file creation: {}", e);
                return Err(ExecutionError::FileWrite(format!("File verification failed: {}", e)));
            }
        }
        
        Ok(())
    }

    fn cleanup_temp_directory(temp_dir: &str) {
        log::info!("Cleaning up temp directory: {}", temp_dir);
        if let Err(e) = fs::remove_dir_all(temp_dir) {
            log::warn!("Failed to clean up temp directory {}: {}", temp_dir, e);
        } else {
            log::info!("Successfully cleaned up temp directory: {}", temp_dir);
        }
    }
}

// Docker executor for running containers
struct DockerExecutor;

impl DockerExecutor {
    async fn execute_with_timeout(
        docker_args: Vec<String>,
        timeout_duration: Duration,
    ) -> Result<std::process::Output, ExecutionError> {
        let start_time = std::time::Instant::now();
        
        let output_result = timeout(timeout_duration, async {
            tokio::task::spawn_blocking(move || {
                Command::new("docker")
                    .args(&docker_args)
                    .output()
            })
            .await
            .map_err(|e| ExecutionError::TaskJoin(e.to_string()))?
            .map_err(|e| ExecutionError::Execution(e.to_string()))
        })
        .await;
        
        match output_result {
            Ok(Ok(output)) => Ok(output),
            Ok(Err(e)) => Err(e),
            Err(_) => {
                let time_taken = start_time.elapsed().as_secs_f64();
                Err(ExecutionError::Timeout(time_taken))
            }
        }
    }

    fn build_docker_command(
        temp_dir: &str,
        config: &LanguageConfig,
        limits: &ResourceLimits,
        command: &[String],
    ) -> Vec<String> {
        DockerCommandBuilder::new()
            .with_network_isolation()
            .with_volume_mount(temp_dir, "/workspace")
            .with_working_directory("/workspace")
            .with_resource_limits(limits)
            .with_image(config.docker_image())
            .with_command(command)
            .build()
    }
}

// Main executor that orchestrates the execution process
pub struct CodeExecutor {
    language_registry: LanguageRegistry,
    resource_limits: ResourceLimits,
}

impl CodeExecutor {
    pub fn new() -> Self {
        Self::with_resource_limits(ResourceLimits::default())
    }

    pub fn with_resource_limits(resource_limits: ResourceLimits) -> Self {
        Self {
            language_registry: LanguageRegistry::new(),
            resource_limits,
        }
    }
    
    pub async fn execute(&self, request: ExecuteRequest) -> Result<ExecuteResponse, ExecutionError> {
        let config = self
            .language_registry
            .get_language_config(&request.language)
            .ok_or_else(|| ExecutionError::UnsupportedLanguage(request.language.clone()))?;
        
        // Generate unique job ID
        let job_id = Uuid::new_v4().to_string();
        
        // Create temp directory
        let temp_dir = FileManager::create_temp_directory(&job_id)?;
        
        // Ensure cleanup happens even if execution fails
        let result = self.execute_in_container(&temp_dir, config, &request.code).await;
        
        // Clean up temp directory
        FileManager::cleanup_temp_directory(&temp_dir);
        
        result
    }
    
    async fn execute_in_container(
        &self,
        temp_dir: &str,
        config: &LanguageConfig,
        code: &str,
    ) -> Result<ExecuteResponse, ExecutionError> {
        // Write code to file
        FileManager::write_code_file(temp_dir, config.file_name(), code)?;
        
        // Get resource limits for this language (use language-specific or default)
        let limits = config.resource_limits().unwrap_or(&self.resource_limits);
        
        // If compilation is needed, compile first
        if let Some(compile_cmd) = config.compile_command() {
            log::info!("Compiling with: {}", compile_cmd.join(" "));
            
            let docker_compile_args = DockerExecutor::build_docker_command(
                temp_dir,
                config,
                limits,
                compile_cmd,
            );
            
            let compile_output = DockerExecutor::execute_with_timeout(
                docker_compile_args,
                limits.wall_time_limit,
            )
            .await?;
            
            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr);
                return Ok(ExecuteResponse {
                    stdout: String::new(),
                    stderr: stderr.to_string(),
                    exit_code: compile_output.status.code().unwrap_or(1),
                    time_taken: None,
                    memory_used: None,
                });
            }
        }
        
        // Build docker command for execution
        let docker_args = DockerExecutor::build_docker_command(
            temp_dir,
            config,
            limits,
            config.run_command(),
        );
        
        log::info!("Executing: docker {}", docker_args.join(" "));
        
        // Execute docker command with timeout
        let start_time = std::time::Instant::now();
        
        let output = DockerExecutor::execute_with_timeout(docker_args, limits.wall_time_limit).await?;
        
        let time_taken = start_time.elapsed().as_secs_f64();
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(1);
        
        log::info!(
            "Execution completed: exit_code={}, stdout_len={}, stderr_len={}, time_taken={:.3}s",
            exit_code,
            stdout.len(),
            stderr.len(),
            time_taken
        );
        
        Ok(ExecuteResponse {
            stdout,
            stderr,
            exit_code,
            time_taken: Some(time_taken),
            memory_used: None, // TODO: Implement memory tracking
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
        assert!(executor.language_registry.get_language_config("python").is_some());
        assert!(executor.language_registry.get_language_config("node").is_some());
        assert!(executor.language_registry.get_language_config("go").is_some());
        assert!(executor.language_registry.get_language_config("rust").is_some());
        assert!(executor.language_registry.get_language_config("c").is_some());
        assert!(executor.language_registry.get_language_config("cpp").is_some());
        assert!(executor.language_registry.get_language_config("java").is_some());
        assert!(executor.language_registry.get_language_config("csharp").is_some());
        assert!(executor.language_registry.get_language_config("php").is_some());
        assert!(executor.language_registry.get_language_config("ruby").is_some());
        
        // Verify configuration details for a few languages
        let python_config = executor.language_registry.get_language_config("python").unwrap();
        assert_eq!(python_config.docker_image(), "python:3.11");
        assert_eq!(python_config.file_name(), "main.py");
        
        let rust_config = executor.language_registry.get_language_config("rust").unwrap();
        assert_eq!(rust_config.docker_image(), "rust:latest");
        assert_eq!(rust_config.file_name(), "main.rs");
        assert!(rust_config.compile_command().is_some());
    }

    #[test]
    fn test_resource_limits_defaults() {
        let limits = ResourceLimits::default();
        assert_eq!(limits.cpu_time_limit.as_secs(), 5);
        assert_eq!(limits.wall_time_limit.as_secs(), 10);
        assert_eq!(limits.memory_limit, 128 * 1024 * 1024);
        assert_eq!(limits.max_processes, 50);
        assert_eq!(limits.max_files, 100);
        assert!(!limits.enable_network);
    }

    #[test]
    fn test_docker_command_builder() {
        let limits = ResourceLimits::default();
        let config = LanguageConfig {
            docker_image: "python:3.11".to_string(),
            file_name: "main.py".to_string(),
            run_command: vec!["python".to_string(), "main.py".to_string()],
            compile_command: None,
            resource_limits: None,
        };

        let docker_args = DockerExecutor::build_docker_command(
            "/tmp/test",
            &config,
            &limits,
            &["python".to_string(), "main.py".to_string()],
        );

        assert!(docker_args.contains(&"--rm".to_string()));
        assert!(docker_args.contains(&"--network".to_string()));
        assert!(docker_args.contains(&"none".to_string()));
        assert!(docker_args.contains(&"--memory".to_string()));
        assert!(docker_args.contains(&"--security-opt".to_string()));
        assert!(docker_args.contains(&"no-new-privileges".to_string()));
        assert!(docker_args.contains(&"--cap-drop".to_string()));
        assert!(docker_args.contains(&"ALL".to_string()));
        assert!(docker_args.contains(&"python:3.11".to_string()));
    }

    #[test]
    fn test_unsupported_language() {
        let executor = CodeExecutor::new();
        let request = ExecuteRequest {
            language: "unsupported".to_string(),
            code: "print('test')".to_string(),
        };
        
        // This should fail with an unsupported language error
        let result = tokio::runtime::Runtime::new().unwrap().block_on(executor.execute(request));
        assert!(result.is_err());
        match result.unwrap_err() {
            ExecutionError::UnsupportedLanguage(lang) => assert_eq!(lang, "unsupported"),
            _ => panic!("Expected UnsupportedLanguage error"),
        }
    }
}
