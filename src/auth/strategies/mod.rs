pub mod apikey;
pub mod jwt;
pub mod mtls;
pub mod none;
pub mod oauth2;

use actix_web::HttpRequest;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

pub use apikey::ApiKeyAuthStrategy;
pub use jwt::JwtAuthStrategy;
pub use mtls::MtlsAuthStrategy;
pub use none::NoneAuthStrategy;
pub use oauth2::OAuth2AuthStrategy;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Authentication failed: {0}")]
    Authentication(String),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Missing credentials: {0}")]
    MissingCredentials(String),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    pub user_id: Option<String>,
    pub permissions: Vec<String>,
    pub metadata: HashMap<String, String>,
    pub authenticated: bool,
}

impl AuthResult {
    pub fn new() -> Self {
        Self {
            user_id: None,
            permissions: Vec::new(),
            metadata: HashMap::new(),
            authenticated: false,
        }
    }

    pub fn authenticated(user_id: String, permissions: Vec<String>) -> Self {
        Self {
            user_id: Some(user_id),
            permissions,
            metadata: HashMap::new(),
            authenticated: true,
        }
    }

    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }

    pub fn has_permission(&self, permission: &str) -> bool {
        self.permissions.contains(&permission.to_string())
    }
}

impl Default for AuthResult {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
pub trait AuthStrategy: Send + Sync {
    async fn authenticate(&self, request: &HttpRequest) -> Result<AuthResult, AuthError>;
    fn name(&self) -> &'static str;
}

/// Helper trait for extracting tokens from requests
pub trait TokenExtractor {
    fn extract_token(&self, request: &HttpRequest) -> Result<String, AuthError>;
}

impl TokenExtractor for () {
    fn extract_token(&self, _request: &HttpRequest) -> Result<String, AuthError> {
        Err(AuthError::MissingCredentials(
            "No token extractor configured".to_string(),
        ))
    }
}

/// Helper trait for extracting API keys from requests
pub trait ApiKeyExtractor {
    fn extract_api_key(&self, request: &HttpRequest) -> Result<String, AuthError>;
}

impl ApiKeyExtractor for () {
    fn extract_api_key(&self, _request: &HttpRequest) -> Result<String, AuthError> {
        Err(AuthError::MissingCredentials(
            "No API key extractor configured".to_string(),
        ))
    }
}

/// Helper trait for extracting client certificates from requests
pub trait CertificateExtractor {
    fn extract_certificate(&self, request: &HttpRequest) -> Result<Vec<u8>, AuthError>;
}

impl CertificateExtractor for () {
    fn extract_certificate(&self, _request: &HttpRequest) -> Result<Vec<u8>, AuthError> {
        Err(AuthError::MissingCredentials(
            "No certificate extractor configured".to_string(),
        ))
    }
}
