use actix_web::HttpRequest;
use async_trait::async_trait;
use super::{AuthStrategy, AuthResult, AuthError};

/// No authentication strategy - allows all requests
pub struct NoneAuthStrategy;

impl NoneAuthStrategy {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl AuthStrategy for NoneAuthStrategy {
    async fn authenticate(&self, _request: HttpRequest) -> Result<AuthResult, AuthError> {
        // Always return success with no user information
        Ok(AuthResult::new())
    }

    fn name(&self) -> &'static str {
        "none"
    }
}

impl Default for NoneAuthStrategy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_none_auth_strategy() {
        let strategy = NoneAuthStrategy::new();
        let req = test::TestRequest::default().to_http_request();
        
        let result = strategy.authenticate(&req).await;
        assert!(result.is_ok());
        
        let auth_result = result.unwrap();
        assert!(!auth_result.authenticated);
        assert!(auth_result.user_id.is_none());
        assert!(auth_result.permissions.is_empty());
    }

    #[test]
    fn test_none_auth_strategy_name() {
        let strategy = NoneAuthStrategy::new();
        assert_eq!(strategy.name(), "none");
    }
} 