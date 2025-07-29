use isobox::generated::isobox::code_execution_service_client::CodeExecutionServiceClient;
use isobox::generated::isobox::{
    ExecuteCodeRequest, GetSupportedLanguagesRequest, HealthCheckRequest,
};
use tonic::transport::Channel;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the gRPC server
    let channel = Channel::from_static("http://127.0.0.1:50051")
        .connect()
        .await?;

    let mut client = CodeExecutionServiceClient::new(channel);

    println!("🚀 isobox gRPC Client Example");
    println!("==============================\n");

    // Health check
    println!("1. Health Check");
    let health_response = client.health_check(HealthCheckRequest {}).await?;

    println!("   Status: {}", health_response.get_ref().status);
    println!("   Service: {}", health_response.get_ref().service);
    println!("   Version: {}", health_response.get_ref().version);
    println!(
        "   Uptime: {} seconds",
        health_response.get_ref().uptime_seconds
    );
    println!();

    // Get supported languages
    println!("2. Supported Languages");
    let languages_response = client
        .get_supported_languages(GetSupportedLanguagesRequest {})
        .await?;

    for lang in &languages_response.get_ref().languages {
        println!(
            "   - {} ({}) - {}",
            lang.display_name,
            lang.name,
            if lang.requires_compilation {
                "compiled"
            } else {
                "interpreted"
            }
        );
    }
    println!();

    // Execute Python code
    println!("3. Execute Python Code");
    let mut python_request = tonic::Request::new(ExecuteCodeRequest {
        language: "python".to_string(),
        code: r#"
import math
print("Python Math Test")
print(f"π = {math.pi}")
print(f"√16 = {math.sqrt(16)}")
print(f"2^10 = {2**10}")
"#
        .to_string(),
        resource_limits: None,
    });

    // Add authentication metadata
    python_request
        .metadata_mut()
        .insert("authorization", "test-key-123".parse()?);

    let python_response = client.execute_code(python_request).await?;

    let response = python_response.get_ref();
    println!("   Exit Code: {}", response.exit_code);
    println!("   Time Taken: {:.3}s", response.time_taken);
    println!("   Status: {:?}", response.status);
    println!("   Output:");
    for line in response.stdout.lines() {
        println!("     {}", line);
    }
    if !response.stderr.is_empty() {
        println!("   Errors:");
        for line in response.stderr.lines() {
            println!("     {}", line);
        }
    }
    println!();

    // Execute Rust code with custom resource limits
    println!("4. Execute Rust Code with Custom Limits");
    let mut rust_request = tonic::Request::new(ExecuteCodeRequest {
        language: "rust".to_string(),
        code: r#"
fn main() {
    println!("Rust Program Test");
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum of {:?} = {}", numbers, sum);
    
    // Demonstrate some Rust features
    let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled);
}
"#
        .to_string(),
        resource_limits: Some(isobox::generated::isobox::ResourceLimits {
            cpu_time_limit_seconds: 10,
            wall_time_limit_seconds: 15,
            memory_limit_bytes: 256 * 1024 * 1024, // 256 MB
            stack_limit_bytes: 64 * 1024 * 1024,   // 64 MB
            max_processes: 100,
            max_files: 200,
            enable_network: false,
        }),
    });

    // Add authentication metadata
    rust_request
        .metadata_mut()
        .insert("authorization", "test-key-123".parse()?);

    let rust_response = client.execute_code(rust_request).await?;

    let response = rust_response.get_ref();
    println!("   Exit Code: {}", response.exit_code);
    println!("   Time Taken: {:.3}s", response.time_taken);
    println!("   Status: {:?}", response.status);
    println!("   Output:");
    for line in response.stdout.lines() {
        println!("     {}", line);
    }
    if !response.stderr.is_empty() {
        println!("   Errors:");
        for line in response.stderr.lines() {
            println!("     {}", line);
        }
    }
    println!();

    // Test error handling with unsupported language
    println!("5. Test Error Handling (Unsupported Language)");
    let mut error_request = tonic::Request::new(ExecuteCodeRequest {
        language: "unsupported_language".to_string(),
        code: "print('test')".to_string(),
        resource_limits: None,
    });

    // Add authentication metadata
    error_request
        .metadata_mut()
        .insert("authorization", "test-key-123".parse()?);

    let error_response = client.execute_code(error_request).await?;

    let response = error_response.get_ref();
    println!("   Exit Code: {}", response.exit_code);
    println!("   Status: {:?}", response.status);
    println!("   Error: {}", response.error_message);
    println!();

    println!("✅ All tests completed successfully!");

    Ok(())
}
