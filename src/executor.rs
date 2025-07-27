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
}

impl CodeExecutor {
    pub fn new() -> Self {
        let mut language_configs = HashMap::new();
        
        // Python configuration
        language_configs.insert(
            "python".to_string(),
            LanguageConfig {
                docker_image: "python:3.11".to_string(),
                file_name: "main.py".to_string(),
                run_command: vec!["python".to_string(), "main.py".to_string()],
            },
        );
        
        // Node.js configuration
        language_configs.insert(
            "node".to_string(),
            LanguageConfig {
                docker_image: "node:20".to_string(),
                file_name: "main.js".to_string(),
                run_command: vec!["node".to_string(), "main.js".to_string()],
            },
        );
        
        // Go configuration
        language_configs.insert(
            "go".to_string(),
            LanguageConfig {
                docker_image: "golang:1.21".to_string(),
                file_name: "main.go".to_string(),
                run_command: vec!["go".to_string(), "run".to_string(), "main.go".to_string()],
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
        
        // Create temp directory
        fs::create_dir_all(&temp_dir)
            .map_err(|e| format!("Failed to create temp directory: {}", e))?;
        
        // Ensure cleanup happens even if execution fails
        let result = self.execute_in_container(&temp_dir, config, &request.code).await;
        
        // Clean up temp directory
        if let Err(e) = fs::remove_dir_all(&temp_dir) {
            log::warn!("Failed to clean up temp directory {}: {}", temp_dir, e);
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
        fs::write(&file_path, code)
            .map_err(|e| format!("Failed to write code file: {}", e))?;
        
        // Build docker command
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
            .map_err(|e| format!("Failed to execute docker command: {}", e))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);
        
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
        
        // Verify that Python, Node.js, and Go languages are supported
        assert!(executor.language_configs.contains_key("python"));
        assert!(executor.language_configs.contains_key("node"));
        assert!(executor.language_configs.contains_key("go"));
        
        // Verify configuration details
        let python_config = executor.language_configs.get("python").unwrap();
        assert_eq!(python_config.docker_image, "python:3.11");
        assert_eq!(python_config.file_name, "main.py");
        
        let node_config = executor.language_configs.get("node").unwrap();
        assert_eq!(node_config.docker_image, "node:20");
        assert_eq!(node_config.file_name, "main.js");
        
        let go_config = executor.language_configs.get("go").unwrap();
        assert_eq!(go_config.docker_image, "golang:1.21");
        assert_eq!(go_config.file_name, "main.go");
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
