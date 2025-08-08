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
    pub test_cases: Option<Vec<TestCase>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TestCase {
    pub name: String,
    pub input: String,
    pub expected_output: Option<String>,
    pub timeout_seconds: Option<u32>,
    pub memory_limit_mb: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestCaseResult {
    pub name: String,
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub time_taken: Option<f64>,
    pub memory_used: Option<u64>,
    pub error_message: Option<String>,
    pub input: String,
    pub expected_output: Option<String>,
    pub actual_output: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExecuteResponse {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub time_taken: Option<f64>,
    pub memory_used: Option<u64>,
    pub test_results: Option<Vec<TestCaseResult>>,
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
            cpu_time_limit: Duration::from_secs(5), // 5 seconds CPU time
            wall_time_limit: Duration::from_secs(10), // 10 seconds wall time
            memory_limit: 128 * 1024 * 1024,        // 128 MB
            stack_limit: 64 * 1024 * 1024,          // 64 MB stack
            max_processes: 50,                      // Max 50 processes
            max_files: 100,                         // Max 100 open files
            enable_network: false,                  // No network access
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
            args: vec!["run".to_string(), "--rm".to_string(), "-i".to_string()],
        }
    }

    fn with_volume_mount(mut self, host_path: &str, container_path: &str) -> Self {
        self.args.extend(vec![
            "-v".to_string(),
            format!("{}:{}", host_path, container_path),
        ]);
        self
    }

    fn with_working_directory(mut self, work_dir: &str) -> Self {
        self.args
            .extend(vec!["-w".to_string(), work_dir.to_string()]);
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
            format!(
                "cpu={}:{}",
                limits.cpu_time_limit.as_secs(),
                limits.cpu_time_limit.as_secs()
            ),
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
            (
                "python",
                "python:3.11",
                "main.py",
                vec!["python".to_string(), "main.py".to_string()],
            ),
            (
                "python2",
                "python:2.7",
                "main.py",
                vec!["python".to_string(), "main.py".to_string()],
            ),
            (
                "node",
                "node:20",
                "main.js",
                vec!["node".to_string(), "main.js".to_string()],
            ),
            (
                "php",
                "php:8.2",
                "main.php",
                vec!["php".to_string(), "main.php".to_string()],
            ),
            (
                "ruby",
                "ruby:3.2",
                "main.rb",
                vec!["ruby".to_string(), "main.rb".to_string()],
            ),
            (
                "perl",
                "perl:5.38",
                "main.pl",
                vec!["perl".to_string(), "main.pl".to_string()],
            ),
            (
                "bash",
                "bash:latest",
                "main.sh",
                vec!["bash".to_string(), "main.sh".to_string()],
            ),
            (
                "lua",
                "lua:5.4",
                "main.lua",
                vec!["lua".to_string(), "main.lua".to_string()],
            ),
            (
                "r",
                "r-base:latest",
                "main.r",
                vec!["Rscript".to_string(), "main.r".to_string()],
            ),
            (
                "octave",
                "octave/octave:latest",
                "main.m",
                vec![
                    "octave".to_string(),
                    "--no-gui".to_string(),
                    "main.m".to_string(),
                ],
            ),
            (
                "dart",
                "dart:stable",
                "main.dart",
                vec!["dart".to_string(), "main.dart".to_string()],
            ),
            (
                "groovy",
                "openjdk:17",
                "main.groovy",
                vec!["groovy".to_string(), "main.groovy".to_string()],
            ),
            (
                "prolog",
                "swipl:latest",
                "main.pl",
                vec!["swipl".to_string(), "main.pl".to_string()],
            ),
            (
                "basic",
                "basic:latest",
                "main.bas",
                vec!["basic".to_string(), "main.bas".to_string()],
            ),
        ];

        for (name, image, file, run_cmd) in scripting_languages {
            languages.insert(
                name.to_string(),
                LanguageConfig {
                    docker_image: image.to_string(),
                    file_name: file.to_string(),
                    run_command: run_cmd,
                    compile_command: None,
                    resource_limits: None,
                },
            );
        }
    }

    fn register_compiled_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let compiled_languages = vec![
            (
                "rust",
                "rust:latest",
                "main.rs",
                vec!["./main".to_string()],
                Some(vec!["rustc".to_string(), "main.rs".to_string()]),
            ),
            (
                "c",
                "gcc:latest",
                "main.c",
                vec!["./a.out".to_string()],
                Some(vec!["gcc".to_string(), "main.c".to_string()]),
            ),
            (
                "cpp",
                "gcc:latest",
                "main.cpp",
                vec!["./a.out".to_string()],
                Some(vec!["g++".to_string(), "main.cpp".to_string()]),
            ),
            (
                "java",
                "openjdk:17",
                "Main.java",
                vec!["java".to_string(), "Main".to_string()],
                Some(vec!["javac".to_string(), "Main.java".to_string()]),
            ),
            (
                "kotlin",
                "openjdk:17",
                "Main.kt",
                vec!["kotlin".to_string(), "MainKt".to_string()],
                Some(vec!["kotlinc".to_string(), "Main.kt".to_string()]),
            ),
            (
                "swift",
                "swift:5.9",
                "main.swift",
                vec!["./main".to_string()],
                Some(vec!["swiftc".to_string(), "main.swift".to_string()]),
            ),
            (
                "scala",
                "openjdk:17",
                "Main.scala",
                vec!["scala".to_string(), "Main".to_string()],
                Some(vec!["scalac".to_string(), "Main.scala".to_string()]),
            ),
            (
                "haskell",
                "haskell:9.4",
                "main.hs",
                vec!["./main".to_string()],
                Some(vec!["ghc".to_string(), "main.hs".to_string()]),
            ),
            (
                "ocaml",
                "ocaml/opam:ubuntu-22.04-ocaml-5.0",
                "main.ml",
                vec!["./a.out".to_string()],
                Some(vec!["ocamlc".to_string(), "main.ml".to_string()]),
            ),
            (
                "d",
                "dlang2/dmd-ubuntu:latest",
                "main.d",
                vec!["./main".to_string()],
                Some(vec!["dmd".to_string(), "main.d".to_string()]),
            ),
            (
                "fortran",
                "gcc:latest",
                "main.f90",
                vec!["./a.out".to_string()],
                Some(vec!["gfortran".to_string(), "main.f90".to_string()]),
            ),
            (
                "pascal",
                "fpc:latest",
                "main.pas",
                vec!["./main".to_string()],
                Some(vec!["fpc".to_string(), "main.pas".to_string()]),
            ),
            (
                "assembly",
                "nasm:latest",
                "main.asm",
                vec!["./main".to_string()],
                Some(vec![
                    "nasm".to_string(),
                    "-f".to_string(),
                    "elf64".to_string(),
                    "main.asm".to_string(),
                    "&&".to_string(),
                    "ld".to_string(),
                    "main.o".to_string(),
                    "-o".to_string(),
                    "main".to_string(),
                ]),
            ),
            (
                "cobol",
                "gnucobol:latest",
                "main.cob",
                vec!["./main".to_string()],
                Some(vec![
                    "cobc".to_string(),
                    "-free".to_string(),
                    "-x".to_string(),
                    "main.cob".to_string(),
                ]),
            ),
            (
                "objective-c",
                "gcc:latest",
                "main.m",
                vec!["./main".to_string()],
                Some(vec![
                    "gcc".to_string(),
                    "-framework".to_string(),
                    "Foundation".to_string(),
                    "main.m".to_string(),
                    "-o".to_string(),
                    "main".to_string(),
                ]),
            ),
        ];

        for (name, image, file, run_cmd, compile_cmd) in compiled_languages {
            languages.insert(
                name.to_string(),
                LanguageConfig {
                    docker_image: image.to_string(),
                    file_name: file.to_string(),
                    run_command: run_cmd,
                    compile_command: compile_cmd,
                    resource_limits: None,
                },
            );
        }
    }

    fn register_functional_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let functional_languages = vec![
            (
                "clojure",
                "clojure:latest",
                "main.clj",
                vec!["clojure".to_string(), "main.clj".to_string()],
            ),
            (
                "elixir",
                "elixir:1.15",
                "main.exs",
                vec!["elixir".to_string(), "main.exs".to_string()],
            ),
            (
                "common-lisp",
                "sbcl:latest",
                "main.lisp",
                vec![
                    "sbcl".to_string(),
                    "--script".to_string(),
                    "main.lisp".to_string(),
                ],
            ),
            (
                "erlang",
                "erlang:latest",
                "main.erl",
                vec!["escript".to_string(), "main.erl".to_string()],
            ),
        ];

        for (name, image, file, run_cmd) in functional_languages {
            languages.insert(
                name.to_string(),
                LanguageConfig {
                    docker_image: image.to_string(),
                    file_name: file.to_string(),
                    run_command: run_cmd,
                    compile_command: None,
                    resource_limits: None,
                },
            );
        }
    }

    fn register_other_languages(languages: &mut HashMap<String, LanguageConfig>) {
        let other_languages = vec![
            (
                "go",
                "golang:1.21",
                "main.go",
                vec!["go".to_string(), "run".to_string(), "main.go".to_string()],
                None,
            ),
            (
                "csharp",
                "mcr.microsoft.com/dotnet/sdk:7.0",
                "Program.cs",
                vec!["dotnet".to_string(), "run".to_string()],
                None,
            ),
            (
                "fsharp",
                "mcr.microsoft.com/dotnet/sdk:7.0",
                "Program.fs",
                vec!["dotnet".to_string(), "run".to_string()],
                None,
            ),
            (
                "vbnet",
                "mcr.microsoft.com/dotnet/sdk:7.0",
                "Program.vb",
                vec!["dotnet".to_string(), "run".to_string()],
                None,
            ),
            (
                "typescript",
                "node:20",
                "main.ts",
                vec!["node".to_string(), "main.js".to_string()],
                Some(vec![
                    "npx".to_string(),
                    "tsc".to_string(),
                    "main.ts".to_string(),
                ]),
            ),
            (
                "sql",
                "sqlite:latest",
                "main.sql",
                vec![
                    "sqlite3".to_string(),
                    "database.db".to_string(),
                    ".read".to_string(),
                    "main.sql".to_string(),
                ],
                None,
            ),
        ];

        for (name, image, file, run_cmd, compile_cmd) in other_languages {
            // Add specific resource limits for Go
            let resource_limits = if name == "go" {
                Some(ResourceLimits {
                    cpu_time_limit: Duration::from_secs(15), // 15 seconds CPU time
                    wall_time_limit: Duration::from_secs(30), // 30 seconds wall time
                    memory_limit: 512 * 1024 * 1024,         // 512 MB
                    stack_limit: 128 * 1024 * 1024,          // 128 MB stack
                    max_processes: 100,                      // Max 100 processes
                    max_files: 200,                          // Max 200 open files
                    enable_network: false,                   // No network access
                })
            } else {
                None
            };

            languages.insert(
                name.to_string(),
                LanguageConfig {
                    docker_image: image.to_string(),
                    file_name: file.to_string(),
                    run_command: run_cmd,
                    compile_command: compile_cmd,
                    resource_limits,
                },
            );
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
        // Create temp directory on host system with proper permissions
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
            return Err(ExecutionError::TempDirectoryCreation(format!(
                "Temp directory does not exist: {}",
                temp_dir
            )));
        }

        // Write the file
        fs::write(&file_path, code).map_err(|e| ExecutionError::FileWrite(e.to_string()))?;

        // Force sync to ensure the file is written to disk
        if let Ok(file) = std::fs::File::open(&file_path) {
            file.sync_all()
                .map_err(|e| ExecutionError::FileWrite(format!("Failed to sync file: {}", e)))?;
        }

        // Verify the file was actually written
        match fs::metadata(&file_path) {
            Ok(metadata) => {
                log::info!("File created successfully. Size: {} bytes", metadata.len());

                // Additional verification: read the file back to ensure it was written correctly
                match fs::read_to_string(&file_path) {
                    Ok(content) => {
                        log::info!(
                            "File content verification: {} bytes read back",
                            content.len()
                        );
                        if content != code {
                            log::warn!(
                                "File content mismatch! Expected {} bytes, got {} bytes",
                                code.len(),
                                content.len()
                            );
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read back file for verification: {}", e);
                    }
                }
            }
            Err(e) => {
                log::error!("Failed to verify file creation: {}", e);
                return Err(ExecutionError::FileWrite(format!(
                    "File verification failed: {}",
                    e
                )));
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
            tokio::task::spawn_blocking(move || Command::new("docker").args(&docker_args).output())
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

    async fn execute_with_timeout_and_stdin(
        docker_args: Vec<String>,
        timeout_duration: Duration,
        stdin_data: &[u8],
    ) -> Result<std::process::Output, ExecutionError> {
        let start_time = std::time::Instant::now();
        let stdin_data = stdin_data.to_vec();

        let output_result = timeout(timeout_duration, async {
            tokio::task::spawn_blocking(move || {
                let mut child = Command::new("docker")
                    .args(&docker_args)
                    .stdin(std::process::Stdio::piped())
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .spawn()
                    .map_err(|e| ExecutionError::Execution(e.to_string()))?;

                if let Some(mut stdin) = child.stdin.take() {
                    use std::io::Write;
                    stdin
                        .write_all(&stdin_data)
                        .map_err(|e| ExecutionError::Execution(e.to_string()))?;
                    // Close stdin to signal EOF
                    drop(stdin);
                }

                child
                    .wait_with_output()
                    .map_err(|e| ExecutionError::Execution(e.to_string()))
            })
            .await
            .map_err(|e| ExecutionError::TaskJoin(e.to_string()))?
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

    pub async fn execute(
        &self,
        request: ExecuteRequest,
    ) -> Result<ExecuteResponse, ExecutionError> {
        let config = self
            .language_registry
            .get_language_config(&request.language)
            .ok_or_else(|| ExecutionError::UnsupportedLanguage(request.language.clone()))?;

        // Generate unique job ID
        let job_id = Uuid::new_v4().to_string();

        // Create temp directory
        let temp_dir = FileManager::create_temp_directory(&job_id)?;

        // Ensure cleanup happens even if execution fails
        let result = if let Some(test_cases) = request.test_cases {
            let result = self
                .execute_with_test_cases(&temp_dir, config, &request.code, test_cases)
                .await;
            // Clean up temp directory after execution
            FileManager::cleanup_temp_directory(&temp_dir);
            result
        } else {
            let result = self
                .execute_in_container(&temp_dir, config, &request.code)
                .await;
            // Clean up temp directory after execution
            FileManager::cleanup_temp_directory(&temp_dir);
            result
        };

        result
    }

    async fn execute_with_test_cases(
        &self,
        temp_dir: &str,
        config: &LanguageConfig,
        code: &str,
        test_cases: Vec<TestCase>,
    ) -> Result<ExecuteResponse, ExecutionError> {
        // Write code to file
        FileManager::write_code_file(temp_dir, config.file_name(), code)?;

        // Get resource limits for this language (use language-specific or default)
        let limits = config.resource_limits().unwrap_or(&self.resource_limits);

        // If compilation is needed, compile first
        if let Some(compile_cmd) = config.compile_command() {
            log::info!("Compiling with: {}", compile_cmd.join(" "));

            let docker_compile_args =
                DockerExecutor::build_docker_command(temp_dir, config, limits, compile_cmd);

            let compile_output =
                DockerExecutor::execute_with_timeout(docker_compile_args, limits.wall_time_limit)
                    .await?;

            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr);
                return Ok(ExecuteResponse {
                    stdout: String::new(),
                    stderr: stderr.to_string(),
                    exit_code: compile_output.status.code().unwrap_or(1),
                    time_taken: None,
                    memory_used: None,
                    test_results: None,
                });
            }
        }

        // Execute each test case
        let mut test_results = Vec::new();
        let mut overall_stdout = String::new();
        let mut overall_stderr = String::new();
        let mut overall_exit_code = 0;

        for test_case in test_cases {
            let test_result = self
                .execute_single_test_case(temp_dir, config, limits, &test_case)
                .await?;

            // Update overall results
            if !test_result.passed {
                overall_exit_code = 1;
            }
            overall_stdout.push_str(&format!("=== Test Case: {} ===\n", test_case.name));
            overall_stdout.push_str(&test_result.stdout);
            overall_stdout.push_str("\n");

            if !test_result.stderr.is_empty() {
                overall_stderr
                    .push_str(&format!("=== Test Case: {} (stderr) ===\n", test_case.name));
                overall_stderr.push_str(&test_result.stderr);
                overall_stderr.push_str("\n");
            }

            test_results.push(test_result);
        }

        Ok(ExecuteResponse {
            stdout: overall_stdout,
            stderr: overall_stderr,
            exit_code: overall_exit_code,
            time_taken: None, // TODO: Calculate total time
            memory_used: None,
            test_results: Some(test_results),
        })
    }

    async fn execute_single_test_case(
        &self,
        temp_dir: &str,
        config: &LanguageConfig,
        limits: &ResourceLimits,
        test_case: &TestCase,
    ) -> Result<TestCaseResult, ExecutionError> {
        // Create custom limits for this test case if specified
        let mut test_limits = limits.clone();
        if let Some(timeout) = test_case.timeout_seconds {
            test_limits.wall_time_limit = Duration::from_secs(timeout as u64);
        }
        if let Some(memory_mb) = test_case.memory_limit_mb {
            test_limits.memory_limit = memory_mb * 1024 * 1024;
        }

        // Build docker command for execution with stdin input
        let docker_args = DockerExecutor::build_docker_command(
            temp_dir,
            config,
            &test_limits,
            config.run_command(),
        );

        // Add stdin input
        let input_data = test_case.input.as_bytes();

        log::info!(
            "Executing test case '{}' with input: {}",
            test_case.name,
            test_case.input
        );

        // Execute docker command with timeout and stdin
        let start_time = std::time::Instant::now();

        let output = DockerExecutor::execute_with_timeout_and_stdin(
            docker_args,
            test_limits.wall_time_limit,
            input_data,
        )
        .await?;

        let time_taken = start_time.elapsed().as_secs_f64();

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(1);

        // Determine if test passed
        let passed = if let Some(expected) = &test_case.expected_output {
            stdout.trim() == expected.trim() && exit_code == 0
        } else {
            exit_code == 0
        };

        let error_message = if !passed {
            if let Some(expected) = &test_case.expected_output {
                Some(format!(
                    "Expected: '{}', Got: '{}'",
                    expected.trim(),
                    stdout.trim()
                ))
            } else {
                Some(format!("Exit code: {}", exit_code))
            }
        } else {
            None
        };

        log::info!(
            "Test case '{}' completed: passed={}, exit_code={}, time_taken={:.3}s",
            test_case.name,
            passed,
            exit_code,
            time_taken
        );

        let actual_output = stdout.clone();
        Ok(TestCaseResult {
            name: test_case.name.clone(),
            passed,
            stdout,
            stderr,
            exit_code,
            time_taken: Some(time_taken),
            memory_used: None,
            error_message,
            input: test_case.input.clone(),
            expected_output: test_case.expected_output.clone(),
            actual_output,
        })
    }

    async fn execute_in_container(
        &self,
        temp_dir: &str,
        config: &LanguageConfig,
        code: &str,
    ) -> Result<ExecuteResponse, ExecutionError> {
        // Write code to file
        FileManager::write_code_file(temp_dir, config.file_name(), code)?;

        // Verify file exists before running Docker
        let file_path = format!("{}/{}", temp_dir, config.file_name());
        if !std::path::Path::new(&file_path).exists() {
            return Err(ExecutionError::FileWrite(format!(
                "File does not exist after creation: {}",
                file_path
            )));
        }

        // Small delay to ensure file is properly synced
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Get resource limits for this language (use language-specific or default)
        let limits = config.resource_limits().unwrap_or(&self.resource_limits);

        // If compilation is needed, compile first
        if let Some(compile_cmd) = config.compile_command() {
            log::info!("Compiling with: {}", compile_cmd.join(" "));

            let docker_compile_args =
                DockerExecutor::build_docker_command(temp_dir, config, limits, compile_cmd);

            let compile_output =
                DockerExecutor::execute_with_timeout(docker_compile_args, limits.wall_time_limit)
                    .await?;

            if !compile_output.status.success() {
                let stderr = String::from_utf8_lossy(&compile_output.stderr);
                return Ok(ExecuteResponse {
                    stdout: String::new(),
                    stderr: stderr.to_string(),
                    exit_code: compile_output.status.code().unwrap_or(1),
                    time_taken: None,
                    memory_used: None,
                    test_results: None,
                });
            }
        }

        // Build docker command for execution
        let docker_args =
            DockerExecutor::build_docker_command(temp_dir, config, limits, config.run_command());

        log::info!("Executing: docker {}", docker_args.join(" "));

        // Execute docker command with timeout
        let start_time = std::time::Instant::now();

        let output =
            DockerExecutor::execute_with_timeout(docker_args, limits.wall_time_limit).await?;

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
            test_results: None,
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
        assert!(executor
            .language_registry
            .get_language_config("python")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("node")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("go")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("rust")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("c")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("cpp")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("java")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("csharp")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("php")
            .is_some());
        assert!(executor
            .language_registry
            .get_language_config("ruby")
            .is_some());

        // Verify configuration details for a few languages
        let python_config = executor
            .language_registry
            .get_language_config("python")
            .unwrap();
        assert_eq!(python_config.docker_image(), "python:3.11");
        assert_eq!(python_config.file_name(), "main.py");

        let rust_config = executor
            .language_registry
            .get_language_config("rust")
            .unwrap();
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
            test_cases: None,
        };

        // This should fail with an unsupported language error
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));
        assert!(result.is_err());
        match result.unwrap_err() {
            ExecutionError::UnsupportedLanguage(lang) => assert_eq!(lang, "unsupported"),
            _ => panic!("Expected UnsupportedLanguage error"),
        }
    }

    #[test]
    fn test_python_multiple_test_cases() {
        // Skip test if Docker is not available
        if std::process::Command::new("docker")
            .arg("--version")
            .output()
            .is_err()
        {
            println!("Docker not available, skipping test_python_multiple_test_cases");
            return;
        }

        let executor = CodeExecutor::new();
        let test_cases = vec![
            TestCase {
                name: "addition_test".to_string(),
                input: "5\n3".to_string(),
                expected_output: Some("8".to_string()),
                timeout_seconds: Some(5),
                memory_limit_mb: Some(128),
            },
            TestCase {
                name: "string_reverse_test".to_string(),
                input: "Hello World".to_string(),
                expected_output: Some("dlroW olleH".to_string()),
                timeout_seconds: Some(5),
                memory_limit_mb: Some(128),
            },
            TestCase {
                name: "array_sum_test".to_string(),
                input: "1 2 3 4 5".to_string(),
                expected_output: Some("15".to_string()),
                timeout_seconds: Some(5),
                memory_limit_mb: Some(128),
            },
        ];

        let request = ExecuteRequest {
            language: "python".to_string(),
            code: r#"
import sys

# Read input from stdin
data = sys.stdin.read().strip()

# Simple example: if input is numbers, add them
if data.replace(' ', '').replace('\n', '').isdigit():
    numbers = [int(x) for x in data.split()]
    result = sum(numbers)
    print(result)
else:
    # If it's text, reverse it
    print(data[::-1])
"#
            .to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        // Check if execution was successful
        if let Err(e) = &result {
            println!("Execution failed: {:?}", e);
            // If it's an unsupported language error, skip the test
            if matches!(e, ExecutionError::UnsupportedLanguage(_)) {
                println!("Python language not supported, skipping test");
                return;
            }
        }

        assert!(
            result.is_ok(),
            "Execution failed: {:?}",
            result.unwrap_err()
        );
        let response = result.unwrap();

        // Check that test results are present
        assert!(response.test_results.is_some());
        let test_results = response.test_results.unwrap();
        assert_eq!(test_results.len(), 3);

        // Check each test case
        for test_result in &test_results {
            assert!(
                test_result.passed,
                "Test {} failed: {:?}",
                test_result.name, test_result.error_message
            );
            assert!(test_result.time_taken.is_some());
            assert!(test_result.time_taken.unwrap() >= 0.0);
        }

        // Verify specific test results
        let addition_test = test_results
            .iter()
            .find(|t| t.name == "addition_test")
            .unwrap();
        println!("Addition test output: {:?}", addition_test.actual_output);
        assert!(
            addition_test.passed,
            "Addition test failed: {:?}",
            addition_test.error_message
        );
        assert_eq!(addition_test.actual_output.trim(), "8");
        assert_eq!(addition_test.input, "5\n3");

        let string_test = test_results
            .iter()
            .find(|t| t.name == "string_reverse_test")
            .unwrap();
        println!("String test output: {:?}", string_test.actual_output);
        assert!(
            string_test.passed,
            "String test failed: {:?}",
            string_test.error_message
        );
        assert_eq!(string_test.actual_output.trim(), "dlroW olleH");
        assert_eq!(string_test.input, "Hello World");

        let array_test = test_results
            .iter()
            .find(|t| t.name == "array_sum_test")
            .unwrap();
        println!("Array test output: {:?}", array_test.actual_output);
        assert!(
            array_test.passed,
            "Array test failed: {:?}",
            array_test.error_message
        );
        assert_eq!(array_test.actual_output.trim(), "15");
        assert_eq!(array_test.input, "1 2 3 4 5");
    }

    #[test]
    fn test_node_multiple_test_cases() {
        let executor = CodeExecutor::new();
        let test_cases = vec![
            TestCase {
                name: "number_sum_test".to_string(),
                input: "1 2 3 4 5".to_string(),
                expected_output: Some("15".to_string()),
                timeout_seconds: Some(5),
                memory_limit_mb: Some(128),
            },
            TestCase {
                name: "string_length_test".to_string(),
                input: "Hello World".to_string(),
                expected_output: Some("11".to_string()),
                timeout_seconds: Some(5),
                memory_limit_mb: Some(128),
            },
        ];

        let request = ExecuteRequest {
            language: "node".to_string(),
            code: r#"
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

let input = '';
rl.on('line', (line) => {
  input += line + '\n';
});

rl.on('close', () => {
  const data = input.trim();
  
  // If input contains only numbers, sum them
  if (/^\d[\d\s]*$/.test(data)) {
    const numbers = data.split(/\s+/).map(Number);
    const sum = numbers.reduce((a, b) => a + b, 0);
    console.log(sum);
  } else {
    // Otherwise, print the length
    console.log(data.length);
  }
});
"#
            .to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        assert!(result.is_ok());
        let response = result.unwrap();

        assert!(response.test_results.is_some());
        let test_results = response.test_results.unwrap();
        assert_eq!(test_results.len(), 2);

        for test_result in &test_results {
            assert!(
                test_result.passed,
                "Test {} failed: {:?}",
                test_result.name, test_result.error_message
            );
        }

        let sum_test = test_results
            .iter()
            .find(|t| t.name == "number_sum_test")
            .unwrap();
        assert_eq!(sum_test.actual_output.trim(), "15");

        let length_test = test_results
            .iter()
            .find(|t| t.name == "string_length_test")
            .unwrap();
        assert_eq!(length_test.actual_output.trim(), "11");
    }

    #[test]
    fn test_rust_multiple_test_cases() {
        let executor = CodeExecutor::new();
        let test_cases = vec![
            TestCase {
                name: "number_sum_test".to_string(),
                input: "1 2 3 4 5".to_string(),
                expected_output: Some("15".to_string()),
                timeout_seconds: Some(10),
                memory_limit_mb: Some(256),
            },
            TestCase {
                name: "string_reverse_test".to_string(),
                input: "Hello".to_string(),
                expected_output: Some("olleH".to_string()),
                timeout_seconds: Some(10),
                memory_limit_mb: Some(256),
            },
        ];

        let request = ExecuteRequest {
            language: "rust".to_string(),
            code: r#"
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let data = input.trim();
    
    // If input contains only numbers, sum them
    if data.chars().all(|c| c.is_ascii_digit() || c.is_ascii_whitespace()) {
        let sum: i32 = data.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            .sum();
        println!("{}", sum);
    } else {
        // Otherwise, reverse the string
        let reversed: String = data.chars().rev().collect();
        println!("{}", reversed);
    }
}
"#
            .to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        assert!(result.is_ok());
        let response = result.unwrap();

        assert!(response.test_results.is_some());
        let test_results = response.test_results.unwrap();
        assert_eq!(test_results.len(), 2);

        for test_result in &test_results {
            assert!(
                test_result.passed,
                "Test {} failed: {:?}",
                test_result.name, test_result.error_message
            );
        }

        let sum_test = test_results
            .iter()
            .find(|t| t.name == "number_sum_test")
            .unwrap();
        assert_eq!(sum_test.actual_output.trim(), "15");

        let reverse_test = test_results
            .iter()
            .find(|t| t.name == "string_reverse_test")
            .unwrap();
        assert_eq!(reverse_test.actual_output.trim(), "olleH");
    }

    #[test]
    fn test_go_multiple_test_cases() {
        let executor = CodeExecutor::new();
        let test_cases = vec![
            TestCase {
                name: "number_sum_test".to_string(),
                input: "1 2 3 4 5".to_string(),
                expected_output: Some("15".to_string()),
                timeout_seconds: Some(15), // Increased timeout
                memory_limit_mb: Some(256),
            },
            TestCase {
                name: "string_uppercase_test".to_string(),
                input: "hello world".to_string(),
                expected_output: Some("HELLO WORLD".to_string()),
                timeout_seconds: Some(15), // Increased timeout
                memory_limit_mb: Some(256),
            },
        ];

        let request = ExecuteRequest {
            language: "go".to_string(),
            code: r#"
package main

import (
    "bufio"
    "fmt"
    "os"
    "strconv"
    "strings"
    "unicode"
)

func main() {
    scanner := bufio.NewScanner(os.Stdin)
    var input string
    for scanner.Scan() {
        input += scanner.Text() + "\n"
    }
    data := strings.TrimSpace(input)
    
    // If input contains only numbers, sum them
    allDigits := true
    for _, char := range data {
        if !unicode.IsDigit(char) && !unicode.IsSpace(char) {
            allDigits = false
            break
        }
    }
    
    if allDigits {
        sum := 0
        for _, numStr := range strings.Fields(data) {
            if num, err := strconv.Atoi(numStr); err == nil {
                sum += num
            }
        }
        fmt.Println(sum)
    } else {
        // Otherwise, convert to uppercase
        fmt.Println(strings.ToUpper(data))
    }
}
"#
            .to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        // Add better error handling to understand what's failing
        match result {
            Ok(response) => {
                assert!(response.test_results.is_some());
                let test_results = response.test_results.unwrap();
                assert_eq!(test_results.len(), 2);

                for test_result in &test_results {
                    if !test_result.passed {
                        println!("Test {} failed:", test_result.name);
                        println!("  Expected: {:?}", test_result.expected_output);
                        println!("  Got: {:?}", test_result.actual_output);
                        println!("  Error: {:?}", test_result.error_message);
                        println!("  Exit code: {}", test_result.exit_code);
                        println!("  Stdout: {:?}", test_result.stdout);
                        println!("  Stderr: {:?}", test_result.stderr);
                    }
                    assert!(
                        test_result.passed,
                        "Test {} failed: {:?}",
                        test_result.name, test_result.error_message
                    );
                }

                let sum_test = test_results
                    .iter()
                    .find(|t| t.name == "number_sum_test")
                    .unwrap();
                assert_eq!(sum_test.actual_output.trim(), "15");

                let upper_test = test_results
                    .iter()
                    .find(|t| t.name == "string_uppercase_test")
                    .unwrap();
                assert_eq!(upper_test.actual_output.trim(), "HELLO WORLD");
            }
            Err(e) => {
                panic!("Go execution failed: {}", e);
            }
        }
    }

    #[test]
    fn test_test_case_without_expected_output() {
        let executor = CodeExecutor::new();
        let test_cases = vec![TestCase {
            name: "simple_print_test".to_string(),
            input: "test input".to_string(),
            expected_output: None, // No expected output
            timeout_seconds: Some(5),
            memory_limit_mb: Some(128),
        }];

        let request = ExecuteRequest {
            language: "python".to_string(),
            code: "import sys\nprint('Hello from Python!')\nprint('Input was:', sys.stdin.read().strip())".to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        assert!(result.is_ok());
        let response = result.unwrap();

        assert!(response.test_results.is_some());
        let test_results = response.test_results.unwrap();
        assert_eq!(test_results.len(), 1);

        let test_result = &test_results[0];
        // Should pass if exit code is 0, regardless of output
        assert!(test_result.passed);
        assert!(test_result.actual_output.contains("Hello from Python!"));
        assert!(test_result.actual_output.contains("Input was: test input"));
    }

    #[test]
    fn test_test_case_with_failing_output() {
        let executor = CodeExecutor::new();
        let test_cases = vec![TestCase {
            name: "failing_test".to_string(),
            input: "5".to_string(),
            expected_output: Some("10".to_string()), // Expect 10, but code will output 5
            timeout_seconds: Some(5),
            memory_limit_mb: Some(128),
        }];

        let request = ExecuteRequest {
            language: "python".to_string(),
            code: "import sys\nprint(sys.stdin.read().strip())".to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        assert!(result.is_ok());
        let response = result.unwrap();

        assert!(response.test_results.is_some());
        let test_results = response.test_results.unwrap();
        assert_eq!(test_results.len(), 1);

        let test_result = &test_results[0];
        // Should fail because output doesn't match expected
        assert!(!test_result.passed);
        assert_eq!(test_result.actual_output.trim(), "5");
        assert!(test_result.error_message.is_some());
        assert!(test_result
            .error_message
            .as_ref()
            .unwrap()
            .contains("Expected: '10', Got: '5'"));
    }

    #[test]
    fn test_test_case_with_timeout() {
        let executor = CodeExecutor::new();
        let test_cases = vec![TestCase {
            name: "timeout_test".to_string(),
            input: "test".to_string(),
            expected_output: Some("test".to_string()),
            timeout_seconds: Some(1), // Very short timeout
            memory_limit_mb: Some(128),
        }];

        let request = ExecuteRequest {
            language: "python".to_string(),
            code: r#"
import time
import sys

# Read input
data = sys.stdin.read().strip()

# Simulate a long-running operation
time.sleep(2)

# Print the input
print(data)
"#
            .to_string(),
            test_cases: Some(test_cases),
        };

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(executor.execute(request));

        // This should either timeout or fail due to the sleep
        match result {
            Ok(response) => {
                assert!(response.test_results.is_some());
                let test_results = response.test_results.unwrap();
                assert_eq!(test_results.len(), 1);

                let test_result = &test_results[0];
                // Should fail due to timeout or unexpected behavior
                assert!(!test_result.passed, "Timeout test should have failed");
            }
            Err(ExecutionError::Timeout(_)) => {
                // Expected timeout error
                println!("Expected timeout occurred");
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_multiple_languages_with_test_cases() {
        let executor = CodeExecutor::new();
        let languages_and_codes = vec![
            ("python", "import sys\nprint('Python:', sys.stdin.read().strip())"),
            ("node", "const readline = require('readline');\nconst rl = readline.createInterface({\n  input: process.stdin,\n  output: process.stdout,\n  terminal: false\n});\n\nrl.on('line', (line) => {\n  console.log('Node:', line);\n  rl.close();\n});"),
            ("php", "<?php\n$input = trim(fgets(STDIN));\necho 'Php: ' . $input . PHP_EOL;"),
            ("ruby", "input = gets.chomp\nputs \"Ruby: #{input}\""),
        ];

        for (language, code) in languages_and_codes {
            let test_cases = vec![TestCase {
                name: "basic_test".to_string(),
                input: "Hello World".to_string(),
                expected_output: Some(format!("{}: Hello World", language.capitalize())),
                timeout_seconds: Some(10),
                memory_limit_mb: Some(256),
            }];

            let request = ExecuteRequest {
                language: language.to_string(),
                code: code.to_string(),
                test_cases: Some(test_cases),
            };

            let result = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(executor.execute(request));

            assert!(result.is_ok(), "Failed for language: {}", language);
            let response = result.unwrap();

            assert!(
                response.test_results.is_some(),
                "No test results for language: {}",
                language
            );
            let test_results = response.test_results.unwrap();
            assert_eq!(
                test_results.len(),
                1,
                "Wrong number of test results for language: {}",
                language
            );

            let test_result = &test_results[0];
            assert!(
                test_result.passed,
                "Test failed for language {}: {:?}",
                language, test_result.error_message
            );
        }
    }
}

trait Capitalize {
    fn capitalize(&self) -> String;
}

impl Capitalize for str {
    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().chain(chars).collect(),
        }
    }
}
