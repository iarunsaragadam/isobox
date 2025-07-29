pub mod cache;
pub mod config;
pub mod dedup;
pub mod middleware;
pub mod strategies;

use actix_web::{Error, HttpRequest, HttpResponse};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

pub use cache::AuthCache;
pub use config::AuthConfig;
pub use dedup::DedupCache;
pub use middleware::AuthMiddleware;
pub use strategies::{AuthError, AuthResult, AuthStrategy};

/// Main authentication service that orchestrates all authentication strategies
pub struct AuthService {
    strategy: Box<dyn AuthStrategy>,
    cache: Arc<AuthCache>,
}

impl AuthService {
    pub fn new(strategy: Box<dyn AuthStrategy>, cache: Arc<AuthCache>) -> Self {
        Self { strategy, cache }
    }

    pub async fn new_with_config(config: &AuthConfig) -> Result<Self, AuthError> {
        let strategy = AuthStrategyFactory::create_strategy(config).await?;
        let cache = Arc::new(AuthCache::new(&config.cache_config).await?);
        Ok(Self { strategy, cache })
    }

    pub async fn authenticate(&self, request: HttpRequest) -> Result<AuthResult, AuthError> {
        // Check cache first
        if let Some(cached_result) = self.cache.get_cached_auth(&request).await? {
            return Ok(cached_result);
        }

        // Perform authentication
        let result = self.strategy.authenticate(request).await?;

        // Cache the result - we need to recreate the request for caching
        // This is a limitation of the current design
        // In a real implementation, you might want to cache based on headers only

        Ok(result)
    }
}

/// Factory for creating authentication strategies
pub struct AuthStrategyFactory;

impl AuthStrategyFactory {
    pub async fn create_strategy(config: &AuthConfig) -> Result<Box<dyn AuthStrategy>, AuthError> {
        match &config.auth_type {
            config::AuthType::None => Ok(Box::new(strategies::NoneAuthStrategy::new())),
            config::AuthType::Jwt => {
                let jwt_config = config.jwt_config.as_ref().ok_or(AuthError::Configuration(
                    "JWT config required for JWT auth".to_string(),
                ))?;
                Ok(Box::new(strategies::JwtAuthStrategy::new(
                    jwt_config.clone(),
                )))
            }
            config::AuthType::Mtls => {
                let mtls_config = config.mtls_config.as_ref().ok_or(AuthError::Configuration(
                    "mTLS config required for mTLS auth".to_string(),
                ))?;
                let mtls_strategy = strategies::MtlsAuthStrategy::new(mtls_config.clone())?;
                Ok(Box::new(mtls_strategy))
            }
            config::AuthType::ApiKey => {
                let apikey_config =
                    config
                        .apikey_config
                        .as_ref()
                        .ok_or(AuthError::Configuration(
                            "API key config required for API key auth".to_string(),
                        ))?;
                Ok(Box::new(strategies::ApiKeyAuthStrategy::new(
                    apikey_config.clone(),
                )))
            }
            config::AuthType::OAuth2 => {
                let oauth2_config =
                    config
                        .oauth2_config
                        .as_ref()
                        .ok_or(AuthError::Configuration(
                            "OAuth2 config required for OAuth2 auth".to_string(),
                        ))?;
                Ok(Box::new(strategies::OAuth2AuthStrategy::new(
                    oauth2_config.clone(),
                )))
            }
        }
    }
}
