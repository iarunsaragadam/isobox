mod executor;
mod generated;
mod grpc;

use crate::executor::{CodeExecutor, ExecuteRequest, TestCase};
use crate::grpc::CodeExecutionServiceImpl;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Result};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct TestCaseFile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct TestCaseUrl {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteWithTestCasesRequest {
    pub language: String,
    pub code: String,
    pub test_cases: Vec<TestCase>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteWithTestFilesRequest {
    pub language: String,
    pub code: String,
    pub test_files: Vec<TestCaseFile>,
}

#[derive(Debug, Deserialize)]
pub struct ExecuteWithTestUrlsRequest {
    pub language: String,
    pub code: String,
    pub test_urls: Vec<TestCaseUrl>,
}

async fn execute_code(
    executor: web::Data<Arc<CodeExecutor>>,
    request: web::Json<crate::executor::ExecuteRequest>,
    http_request: HttpRequest,
) -> Result<HttpResponse> {
    // Simple API key check
    let api_key = http_request.headers().get("X-API-Key");
    if api_key.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "API Key not provided",
            "message": "Please provide an X-API-Key header"
        })));
    }

    // Validate API key (simple check for now)
    let provided_key = api_key.unwrap().to_str().unwrap_or("");
    let api_keys_str = std::env::var("API_KEYS").unwrap_or_else(|_| "default-key".to_string());
    let valid_keys = api_keys_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    if !valid_keys.contains(&provided_key) {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid API Key",
            "message": "The provided API key is not valid"
        })));
    }

    let result = executor.execute(request.into_inner()).await;

    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Execution failed",
            "message": e.to_string()
        }))),
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "isobox"
    })))
}

async fn auth_status(_request: HttpRequest) -> Result<HttpResponse> {
    // Simple authentication status endpoint
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "authenticated": false,
        "message": "Authentication not implemented in simplified version"
    })))
}

async fn dedup_stats(_executor: web::Data<Arc<CodeExecutor>>) -> Result<HttpResponse> {
    // Simple dedup stats endpoint
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "dedup_enabled": false,
        "message": "Deduplication not implemented in simplified version"
    })))
}

async fn execute_with_test_cases(
    executor: web::Data<Arc<CodeExecutor>>,
    request: web::Json<ExecuteWithTestCasesRequest>,
    http_request: HttpRequest,
) -> Result<HttpResponse> {
    // Simple API key check
    let api_key = http_request.headers().get("X-API-Key");
    if api_key.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "API Key not provided",
            "message": "Please provide an X-API-Key header"
        })));
    }

    // Validate API key (simple check for now)
    let provided_key = api_key.unwrap().to_str().unwrap_or("");
    let api_keys_str = std::env::var("API_KEYS").unwrap_or_else(|_| "default-key".to_string());
    let valid_keys = api_keys_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    if !valid_keys.contains(&provided_key) {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid API Key",
            "message": "The provided API key is not valid"
        })));
    }

    let execute_request = ExecuteRequest {
        language: request.language.clone(),
        code: request.code.clone(),
        test_cases: Some(request.test_cases.clone()),
    };

    let result = executor.execute(execute_request).await;

    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Execution failed",
            "message": e.to_string()
        }))),
    }
}

async fn execute_with_test_files(
    executor: web::Data<Arc<CodeExecutor>>,
    request: web::Json<ExecuteWithTestFilesRequest>,
    http_request: HttpRequest,
) -> Result<HttpResponse> {
    // Simple API key check
    let api_key = http_request.headers().get("X-API-Key");
    if api_key.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "API Key not provided",
            "message": "Please provide an X-API-Key header"
        })));
    }

    // Validate API key (simple check for now)
    let provided_key = api_key.unwrap().to_str().unwrap_or("");
    let api_keys_str = std::env::var("API_KEYS").unwrap_or_else(|_| "default-key".to_string());
    let valid_keys = api_keys_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    if !valid_keys.contains(&provided_key) {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid API Key",
            "message": "The provided API key is not valid"
        })));
    }

    // Convert test files to test cases
    let test_cases: Vec<TestCase> = request
        .test_files
        .iter()
        .map(|file| TestCase {
            name: file.name.clone(),
            input: file.content.clone(),
            expected_output: None,
            timeout_seconds: None,
            memory_limit_mb: None,
        })
        .collect();

    let execute_request = ExecuteRequest {
        language: request.language.clone(),
        code: request.code.clone(),
        test_cases: Some(test_cases),
    };

    let result = executor.execute(execute_request).await;

    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Execution failed",
            "message": e.to_string()
        }))),
    }
}

async fn execute_with_test_urls(
    executor: web::Data<Arc<CodeExecutor>>,
    request: web::Json<ExecuteWithTestUrlsRequest>,
    http_request: HttpRequest,
) -> Result<HttpResponse> {
    // Simple API key check
    let api_key = http_request.headers().get("X-API-Key");
    if api_key.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "API Key not provided",
            "message": "Please provide an X-API-Key header"
        })));
    }

    // Validate API key (simple check for now)
    let provided_key = api_key.unwrap().to_str().unwrap_or("");
    let api_keys_str = std::env::var("API_KEYS").unwrap_or_else(|_| "default-key".to_string());
    let valid_keys = api_keys_str
        .split(',')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    if !valid_keys.contains(&provided_key) {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Invalid API Key",
            "message": "The provided API key is not valid"
        })));
    }

    // Download test cases from URLs
    let mut test_cases = Vec::new();
    for test_url in &request.test_urls {
        match download_test_case(&test_url.url).await {
            Ok(content) => {
                test_cases.push(TestCase {
                    name: test_url.name.clone(),
                    input: content,
                    expected_output: None,
                    timeout_seconds: None,
                    memory_limit_mb: None,
                });
            }
            Err(e) => {
                return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                    "error": "Failed to download test case",
                    "message": format!("Failed to download {}: {}", test_url.url, e)
                })));
            }
        }
    }

    let execute_request = ExecuteRequest {
        language: request.language.clone(),
        code: request.code.clone(),
        test_cases: Some(test_cases),
    };

    let result = executor.execute(execute_request).await;

    match result {
        Ok(response) => Ok(HttpResponse::Ok().json(response)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": "Execution failed",
            "message": e.to_string()
        }))),
    }
}

async fn download_test_case(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let content = response.text().await?;
    Ok(content)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting IsoBox server...");

    // Create executor without deduplication
    let executor = Arc::new(CodeExecutor::new());

    // Check if Docker is available
    match std::process::Command::new("docker")
        .arg("--version")
        .output()
    {
        Ok(output) if output.status.success() => {
            let version = String::from_utf8_lossy(&output.stdout);
            log::info!("Docker available: {}", version.trim());
        }
        _ => {
            log::error!("Docker is not available or not running!");
            std::process::exit(1);
        }
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let grpc_port = std::env::var("GRPC_PORT").unwrap_or_else(|_| "50051".to_string());
    let bind_address = format!("0.0.0.0:{}", port);
    let grpc_address = format!("0.0.0.0:{}", grpc_port);

    log::info!("HTTP server starting on {}", bind_address);
    log::info!("gRPC server starting on {}", grpc_address);

    // Create gRPC service with authentication
    let grpc_service = CodeExecutionServiceImpl::new(executor.clone(), None); // No auth service for now

    // Start gRPC server in a separate task
    let grpc_service_clone = grpc_service.clone();
    let grpc_handle = tokio::spawn(async move {
        tonic::transport::Server::builder()
            .add_service(
                crate::generated::isobox::code_execution_service_server::CodeExecutionServiceServer::new(grpc_service_clone)
            )
            .serve(grpc_address.parse().unwrap())
            .await
    });

    // Start HTTP server
    let http_handle = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(executor.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .route("/execute", web::post().to(execute_code))
                    .route(
                        "/execute/test-cases",
                        web::post().to(execute_with_test_cases),
                    )
                    .route(
                        "/execute/test-files",
                        web::post().to(execute_with_test_files),
                    )
                    .route("/execute/test-urls", web::post().to(execute_with_test_urls)),
            )
            .service(web::scope("/auth").route("/status", web::get().to(auth_status)))
            .service(web::scope("/admin").route("/dedup/stats", web::get().to(dedup_stats)))
            .route("/health", web::get().to(health_check))
    })
    .bind(&bind_address)?
    .run();

    // Wait for both servers
    tokio::select! {
        result = http_handle => {
            if let Err(e) = result {
                log::error!("HTTP server error: {}", e);
            }
        }
        result = grpc_handle => {
            if let Err(e) = result {
                log::error!("gRPC server error: {}", e);
            }
        }
    }

    Ok(())
}
