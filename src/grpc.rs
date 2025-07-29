use crate::executor::{CodeExecutor, ExecuteRequest};
use crate::generated::isobox::code_execution_service_server::CodeExecutionService as CodeExecutionServiceTrait;
use crate::generated::isobox::{
    ExecuteCodeRequest, ExecuteCodeResponse, ExecutionStatus, GetSupportedLanguagesRequest,
    GetSupportedLanguagesResponse, HealthCheckRequest, HealthCheckResponse, LanguageInfo,
};
use std::sync::Arc;
use std::time::Instant;
use tonic::{Request, Response, Status};

#[derive(Clone)]
pub struct CodeExecutionServiceImpl {
    executor: Arc<CodeExecutor>,
    start_time: Instant,
}

impl CodeExecutionServiceImpl {
    pub fn new(executor: Arc<CodeExecutor>, _auth_service: Option<Arc<()>>) -> Self {
        Self {
            executor,
            start_time: Instant::now(),
        }
    }
}

#[tonic::async_trait]
impl CodeExecutionServiceTrait for CodeExecutionServiceImpl {
    async fn execute_code(
        &self,
        request: Request<ExecuteCodeRequest>,
    ) -> Result<Response<ExecuteCodeResponse>, Status> {
        // Check authentication first
        let metadata = request.metadata();
        let api_key = metadata.get("authorization");
        if api_key.is_none() {
            return Err(Status::unauthenticated("API Key not provided"));
        }

        // Validate API key
        let provided_key = api_key.unwrap().to_str().unwrap_or("");
        let api_keys_str = std::env::var("API_KEYS").unwrap_or_else(|_| "default-key".to_string());
        let valid_keys = api_keys_str
            .split(',')
            .map(|s| s.trim())
            .collect::<Vec<_>>();

        if !valid_keys.contains(&provided_key) {
            return Err(Status::unauthenticated("Invalid API Key"));
        }

        let req = request.into_inner();
        log::info!("gRPC: Executing code in language: {}", req.language);

        // Convert proto request to internal request
        let exec_request = ExecuteRequest {
            language: req.language,
            code: req.code,
            test_cases: None, // gRPC doesn't support test cases yet
        };

        // Execute the code
        match self.executor.execute(exec_request).await {
            Ok(response) => {
                let proto_response = ExecuteCodeResponse {
                    stdout: response.stdout,
                    stderr: response.stderr,
                    exit_code: response.exit_code,
                    time_taken: response.time_taken.unwrap_or(0.0),
                    memory_used: response.memory_used.unwrap_or(0),
                    status: if response.exit_code == 0 {
                        ExecutionStatus::Success as i32
                    } else {
                        ExecutionStatus::RuntimeError as i32
                    },
                    error_message: String::new(),
                };

                Ok(Response::new(proto_response))
            }
            Err(e) => {
                let status = match e {
                    crate::executor::ExecutionError::UnsupportedLanguage(_) => {
                        ExecutionStatus::UnsupportedLanguage as i32
                    }
                    crate::executor::ExecutionError::Timeout(_) => ExecutionStatus::Timeout as i32,
                    _ => ExecutionStatus::InternalError as i32,
                };

                let proto_response = ExecuteCodeResponse {
                    stdout: String::new(),
                    stderr: String::new(),
                    exit_code: -1,
                    time_taken: 0.0,
                    memory_used: 0,
                    status,
                    error_message: e.to_string(),
                };

                Ok(Response::new(proto_response))
            }
        }
    }

    async fn health_check(
        &self,
        _request: Request<HealthCheckRequest>,
    ) -> Result<Response<HealthCheckResponse>, Status> {
        let uptime = self.start_time.elapsed().as_secs();

        let response = HealthCheckResponse {
            status: "healthy".to_string(),
            service: "isobox".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            uptime_seconds: uptime,
        };

        Ok(Response::new(response))
    }

    async fn get_supported_languages(
        &self,
        _request: Request<GetSupportedLanguagesRequest>,
    ) -> Result<Response<GetSupportedLanguagesResponse>, Status> {
        // Define supported languages
        let languages = vec![
            LanguageInfo {
                name: "python".to_string(),
                display_name: "Python".to_string(),
                docker_image: "python:3.11-slim".to_string(),
                requires_compilation: false,
                file_extensions: vec!["py".to_string()],
            },
            LanguageInfo {
                name: "node".to_string(),
                display_name: "Node.js".to_string(),
                docker_image: "node:18-slim".to_string(),
                requires_compilation: false,
                file_extensions: vec!["js".to_string()],
            },
            LanguageInfo {
                name: "rust".to_string(),
                display_name: "Rust".to_string(),
                docker_image: "rust:1.70-slim".to_string(),
                requires_compilation: true,
                file_extensions: vec!["rs".to_string()],
            },
            LanguageInfo {
                name: "go".to_string(),
                display_name: "Go".to_string(),
                docker_image: "golang:1.21".to_string(),
                requires_compilation: true,
                file_extensions: vec!["go".to_string()],
            },
            LanguageInfo {
                name: "cpp".to_string(),
                display_name: "C++".to_string(),
                docker_image: "gcc:latest".to_string(),
                requires_compilation: true,
                file_extensions: vec!["cpp".to_string(), "cc".to_string()],
            },
            LanguageInfo {
                name: "java".to_string(),
                display_name: "Java".to_string(),
                docker_image: "openjdk:17-slim".to_string(),
                requires_compilation: true,
                file_extensions: vec!["java".to_string()],
            },
            LanguageInfo {
                name: "csharp".to_string(),
                display_name: "C#".to_string(),
                docker_image: "mcr.microsoft.com/dotnet/sdk:7.0".to_string(),
                requires_compilation: true,
                file_extensions: vec!["cs".to_string()],
            },
            LanguageInfo {
                name: "php".to_string(),
                display_name: "PHP".to_string(),
                docker_image: "php:8.2-cli".to_string(),
                requires_compilation: false,
                file_extensions: vec!["php".to_string()],
            },
            LanguageInfo {
                name: "ruby".to_string(),
                display_name: "Ruby".to_string(),
                docker_image: "ruby:3.2-slim".to_string(),
                requires_compilation: false,
                file_extensions: vec!["rb".to_string()],
            },
            LanguageInfo {
                name: "bash".to_string(),
                display_name: "Bash".to_string(),
                docker_image: "ubuntu:22.04".to_string(),
                requires_compilation: false,
                file_extensions: vec!["sh".to_string()],
            },
        ];

        let response = GetSupportedLanguagesResponse { languages };
        Ok(Response::new(response))
    }
}
