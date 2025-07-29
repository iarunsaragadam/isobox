use super::{ApiKeyExtractor, AuthError, AuthResult, AuthStrategy};
use crate::auth::config::ApiKeyConfig;
use actix_web::HttpRequest;
use async_trait::async_trait;

/// API Key authentication strategy
pub struct ApiKeyAuthStrategy {
    config: ApiKeyConfig,
}

impl ApiKeyAuthStrategy {
    pub fn new(config: ApiKeyConfig) -> Self {
        Self { config }
    }

    fn extract_api_key_from_header(&self, request: &HttpRequest) -> Result<String, AuthError> {
        let api_key = request
            .headers()
            .get(&self.config.header_name)
            .ok_or_else(|| {
                AuthError::MissingCredentials(format!("No {} header", self.config.header_name))
            })?
            .to_str()
            .map_err(|_| AuthError::MissingCredentials("Invalid API key header".to_string()))?;

        if api_key.trim().is_empty() {
            return Err(AuthError::MissingCredentials("Empty API key".to_string()));
        }

        Ok(api_key.trim().to_string())
    }

    fn validate_api_key(&self, api_key: &str) -> Result<(), AuthError> {
        if self.config.api_keys.contains(&api_key.to_string()) {
            Ok(())
        } else {
            Err(AuthError::Authentication("Invalid API key".to_string()))
        }
    }
}

#[async_trait]
impl AuthStrategy for ApiKeyAuthStrategy {
    async fn authenticate(&self, request: &HttpRequest) -> Result<AuthResult, AuthError> {
        // Extract API key from request
        let api_key = self.extract_api_key_from_header(request)?;

        // Validate the API key
        self.validate_api_key(&api_key)?;

        // For API key auth, we use the key itself as a user identifier
        // In a real system, you might want to map API keys to specific users
        let user_id = format!("apikey:{}", api_key);

        // API keys typically have full permissions
        let permissions = vec![
            "execute".to_string(),
            "read".to_string(),
            "write".to_string(),
        ];

        Ok(AuthResult::authenticated(user_id, permissions)
            .with_metadata("auth_type".to_string(), "apikey".to_string())
            .with_metadata(
                "api_key_prefix".to_string(),
                api_key[..8.min(api_key.len())].to_string(),
            ))
    }

    fn name(&self) -> &'static str {
        "apikey"
    }
}

impl ApiKeyExtractor for ApiKeyAuthStrategy {
    fn extract_api_key(&self, request: &HttpRequest) -> Result<String, AuthError> {
        self.extract_api_key_from_header(request)
    }
}

impl ApiKeyAuthStrategy {
    /// Add a new API key to the allowed list
    pub fn add_api_key(&mut self, api_key: String) {
        if !self.config.api_keys.contains(&api_key) {
            self.config.api_keys.push(api_key);
        }
    }

    /// Remove an API key from the allowed list
    pub fn remove_api_key(&mut self, api_key: &str) -> bool {
        if let Some(index) = self.config.api_keys.iter().position(|k| k == api_key) {
            self.config.api_keys.remove(index);
            true
        } else {
            false
        }
    }

    /// Check if an API key is valid without performing full authentication
    pub fn is_valid_api_key(&self, api_key: &str) -> bool {
        self.config.api_keys.contains(&api_key.to_string())
    }

    /// Get the number of configured API keys
    pub fn api_key_count(&self) -> usize {
        self.config.api_keys.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    fn create_test_config() -> ApiKeyConfig {
        ApiKeyConfig {
            api_keys: vec![
                "test-key-1".to_string(),
                "test-key-2".to_string(),
                "secret-key-123".to_string(),
            ],
            header_name: "X-API-Key".to_string(),
        }
    }

    #[actix_web::test]
    async fn test_apikey_auth_strategy_success() {
        let strategy = ApiKeyAuthStrategy::new(create_test_config());
        let req = test::TestRequest::default()
            .insert_header(("X-API-Key", "test-key-1"))
            .to_http_request();

        let result = strategy.authenticate(&req).await;
        assert!(result.is_ok());

        let auth_result = result.unwrap();
        assert!(auth_result.authenticated);
        assert_eq!(auth_result.user_id, Some("apikey:test-key-1".to_string()));
        assert!(auth_result.has_permission("execute"));
        assert!(auth_result.has_permission("read"));
        assert!(auth_result.has_permission("write"));
    }

    #[actix_web::test]
    async fn test_apikey_auth_strategy_missing_header() {
        let strategy = ApiKeyAuthStrategy::new(create_test_config());
        let req = test::TestRequest::default().to_http_request();

        let result = strategy.authenticate(&req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::MissingCredentials(_) => {}
            _ => panic!("Expected MissingCredentials error"),
        }
    }

    #[actix_web::test]
    async fn test_apikey_auth_strategy_invalid_key() {
        let strategy = ApiKeyAuthStrategy::new(create_test_config());
        let req = test::TestRequest::default()
            .insert_header(("X-API-Key", "invalid-key"))
            .to_http_request();

        let result = strategy.authenticate(&req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::Authentication(_) => {}
            _ => panic!("Expected Authentication error"),
        }
    }

    #[actix_web::test]
    async fn test_apikey_auth_strategy_empty_key() {
        let strategy = ApiKeyAuthStrategy::new(create_test_config());
        let req = test::TestRequest::default()
            .insert_header(("X-API-Key", ""))
            .to_http_request();

        let result = strategy.authenticate(&req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::MissingCredentials(_) => {}
            _ => panic!("Expected MissingCredentials error"),
        }
    }

    #[test]
    fn test_apikey_auth_strategy_name() {
        let strategy = ApiKeyAuthStrategy::new(create_test_config());
        assert_eq!(strategy.name(), "apikey");
    }

    #[test]
    fn test_apikey_management() {
        let mut strategy = ApiKeyAuthStrategy::new(create_test_config());

        // Test initial count
        assert_eq!(strategy.api_key_count(), 3);

        // Test adding new key
        strategy.add_api_key("new-key".to_string());
        assert_eq!(strategy.api_key_count(), 4);
        assert!(strategy.is_valid_api_key("new-key"));

        // Test adding duplicate key
        strategy.add_api_key("new-key".to_string());
        assert_eq!(strategy.api_key_count(), 4);

        // Test removing key
        assert!(strategy.remove_api_key("test-key-1"));
        assert_eq!(strategy.api_key_count(), 3);
        assert!(!strategy.is_valid_api_key("test-key-1"));

        // Test removing non-existent key
        assert!(!strategy.remove_api_key("non-existent"));
        assert_eq!(strategy.api_key_count(), 3);
    }

    #[test]
    fn test_custom_header_name() {
        let config = ApiKeyConfig {
            api_keys: vec!["custom-key".to_string()],
            header_name: "X-Custom-Key".to_string(),
        };

        let strategy = ApiKeyAuthStrategy::new(config);
        let req = test::TestRequest::default()
            .insert_header(("X-Custom-Key", "custom-key"))
            .to_http_request();

        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(strategy.authenticate(&req));
        assert!(result.is_ok());
    }
}
