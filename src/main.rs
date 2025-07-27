mod executor;

use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer, Result};
use executor::{CodeExecutor, ExecuteRequest};
use std::sync::Arc;

async fn execute_code(
    executor: web::Data<Arc<CodeExecutor>>,
    request: web::Json<ExecuteRequest>,
) -> Result<HttpResponse> {
    log::info!("Received execution request for language: {}", request.language);
    
    match executor.execute(request.into_inner()).await {
        Ok(response) => {
            log::info!("Execution successful");
            Ok(HttpResponse::Ok().json(response))
        }
        Err(error) => {
            log::error!("Execution failed: {}", error);
            Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": error
            })))
        }
    }
}

async fn health_check() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "isobox",
        "version": env!("CARGO_PKG_VERSION")
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    log::info!("Starting isobox server...");
    
    // Create executor
    let executor = Arc::new(CodeExecutor::new());
    
    // Check if Docker is available
    match std::process::Command::new("docker").arg("--version").output() {
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
    let bind_address = format!("0.0.0.0:{}", port);
    
    log::info!("Server starting on {}", bind_address);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(executor.clone()))
            .wrap(Logger::default())
            .route("/health", web::get().to(health_check))
            .route("/execute", web::post().to(execute_code))
    })
    .bind(&bind_address)?
    .run()
    .await
}
