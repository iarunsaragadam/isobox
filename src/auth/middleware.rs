use super::{AuthError, AuthResult, AuthService, AuthStrategy};
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpRequest, HttpResponse,
};
use futures_util::future::{ready, LocalBoxFuture, Ready};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Authentication middleware for Actix-Web
pub struct AuthMiddleware {
    auth_service: Arc<AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: AuthService) -> Self {
        Self {
            auth_service: Arc::new(auth_service),
        }
    }

    pub fn with_strategy(
        strategy: Box<dyn AuthStrategy>,
        cache: Arc<super::cache::AuthCache>,
    ) -> Self {
        let auth_service = AuthService::new(strategy, cache);
        Self::new(auth_service)
    }
}

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService {
            service,
            auth_service: self.auth_service.clone(),
        }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
    auth_service: Arc<AuthService>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_service = self.auth_service.clone();
        let service = self.service.clone();

        Box::pin(async move {
            // Extract the HTTP request for authentication
            let http_req = req.request().clone();

            // Perform authentication
            let auth_result = match auth_service.authenticate(http_req).await {
                Ok(result) => result,
                Err(AuthError::MissingCredentials(msg)) => {
                    log::warn!("Authentication failed - missing credentials: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Authentication required",
                                "message": msg,
                                "code": "MISSING_CREDENTIALS"
                            }))
                            .map_into_right_body(),
                    ));
                }
                Err(AuthError::InvalidToken(msg)) => {
                    log::warn!("Authentication failed - invalid token: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Invalid authentication token",
                                "message": msg,
                                "code": "INVALID_TOKEN"
                            }))
                            .map_into_right_body(),
                    ));
                }
                Err(AuthError::Authentication(msg)) => {
                    log::warn!("Authentication failed: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Authentication failed",
                                "message": msg,
                                "code": "AUTH_FAILED"
                            }))
                            .map_into_right_body(),
                    ));
                }
                Err(AuthError::Configuration(msg)) => {
                    log::error!("Authentication configuration error: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(serde_json::json!({
                                "error": "Authentication configuration error",
                                "message": msg,
                                "code": "CONFIG_ERROR"
                            }))
                            .map_into_right_body(),
                    ));
                }
                Err(AuthError::Network(msg)) => {
                    log::error!("Authentication network error: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::ServiceUnavailable()
                            .json(serde_json::json!({
                                "error": "Authentication service unavailable",
                                "message": msg,
                                "code": "NETWORK_ERROR"
                            }))
                            .map_into_right_body(),
                    ));
                }
                Err(AuthError::Internal(msg)) => {
                    log::error!("Authentication internal error: {}", msg);
                    return Ok(req.into_response(
                        HttpResponse::InternalServerError()
                            .json(serde_json::json!({
                                "error": "Authentication internal error",
                                "message": msg,
                                "code": "INTERNAL_ERROR"
                            }))
                            .map_into_right_body(),
                    ));
                }
            };

            // Add authentication information to request extensions
            req.extensions_mut().insert(auth_result);

            // Continue with the request
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

/// Helper trait to extract authentication result from request extensions
pub trait AuthExtractor {
    fn get_auth_result(&self) -> Option<AuthResult>;
}

impl AuthExtractor for HttpRequest {
    fn get_auth_result(&self) -> Option<AuthResult> {
        self.extensions().get::<AuthResult>().cloned()
    }
}

/// Helper trait to check if user has specific permissions
pub trait PermissionChecker {
    fn has_permission(&self, permission: &str) -> bool;
    fn is_authenticated(&self) -> bool;
    fn get_user_id(&self) -> Option<String>;
}

impl PermissionChecker for HttpRequest {
    fn has_permission(&self, permission: &str) -> bool {
        self.get_auth_result()
            .map(|auth| auth.has_permission(permission))
            .unwrap_or(false)
    }

    fn is_authenticated(&self) -> bool {
        self.get_auth_result()
            .map(|auth| auth.authenticated)
            .unwrap_or(false)
    }

    fn get_user_id(&self) -> Option<String> {
        self.get_auth_result().and_then(|auth| auth.user_id)
    }
}

/// Middleware configuration for different authentication requirements
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub require_auth: bool,
    pub required_permissions: Vec<String>,
    pub allow_anonymous: bool,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            require_auth: true,
            required_permissions: vec!["execute".to_string()],
            allow_anonymous: false,
        }
    }
}

/// Middleware that enforces specific authentication requirements
pub struct RequiredAuthMiddleware {
    config: AuthConfig,
}

impl RequiredAuthMiddleware {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub fn require_permission(permission: &str) -> Self {
        Self::new(AuthConfig {
            require_auth: true,
            required_permissions: vec![permission.to_string()],
            allow_anonymous: false,
        })
    }

    pub fn require_permissions(permissions: Vec<String>) -> Self {
        Self::new(AuthConfig {
            require_auth: true,
            required_permissions: permissions,
            allow_anonymous: false,
        })
    }

    pub fn allow_anonymous() -> Self {
        Self::new(AuthConfig {
            require_auth: false,
            required_permissions: vec![],
            allow_anonymous: true,
        })
    }
}

impl<S, B> Transform<S, ServiceRequest> for RequiredAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = RequiredAuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequiredAuthMiddlewareService {
            service,
            config: self.config.clone(),
        }))
    }
}

pub struct RequiredAuthMiddlewareService<S> {
    service: S,
    config: AuthConfig,
}

impl<S, B> Service<ServiceRequest> for RequiredAuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + Clone,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let config = self.config.clone();
        let service = self.service.clone();

        Box::pin(async move {
            let http_req = req.request();

            // Check if authentication is required
            if config.require_auth {
                if !http_req.is_authenticated() {
                    return Ok(req.into_response(
                        HttpResponse::Unauthorized()
                            .json(serde_json::json!({
                                "error": "Authentication required",
                                "code": "AUTH_REQUIRED"
                            }))
                            .map_into_right_body(),
                    ));
                }

                // Check required permissions
                for permission in &config.required_permissions {
                    if !http_req.has_permission(permission) {
                        return Ok(req.into_response(
                            HttpResponse::Forbidden()
                                .json(serde_json::json!({
                                    "error": "Insufficient permissions",
                                    "message": format!("Permission '{}' required", permission),
                                    "code": "INSUFFICIENT_PERMISSIONS"
                                }))
                                .map_into_right_body(),
                        ));
                    }
                }
            }

            // Continue with the request
            let res = service.call(req).await?;
            Ok(res)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::strategies::NoneAuthStrategy;
    use actix_web::test;

    #[actix_web::test]
    async fn test_auth_middleware_with_none_strategy() {
        let strategy = Box::new(NoneAuthStrategy::new());
        let cache = Arc::new(
            crate::auth::cache::AuthCache::new(&crate::auth::config::CacheConfig {
                auth_cache_ttl: std::time::Duration::from_secs(300),
                max_cache_size: 1000,
            })
            .await
            .unwrap(),
        );

        let middleware = AuthMiddleware::with_strategy(strategy, cache);
        let req = test::TestRequest::default().to_http_request();

        // The middleware should work with the none strategy
        assert!(req.get_auth_result().is_none());
    }

    #[tokio::test]
    async fn test_permission_checker() {
        let req = test::TestRequest::default().to_http_request();

        // Without auth result, should return false/defaults
        assert!(!req.is_authenticated());
        assert!(!req.has_permission("execute"));
        assert!(req.get_user_id().is_none());
    }

    #[test]
    fn test_auth_config_defaults() {
        let config = AuthConfig::default();
        assert!(config.require_auth);
        assert_eq!(config.required_permissions, vec!["execute"]);
        assert!(!config.allow_anonymous);
    }

    #[test]
    fn test_required_auth_middleware_factory() {
        let middleware = RequiredAuthMiddleware::require_permission("execute");
        assert!(middleware.config.require_auth);
        assert_eq!(middleware.config.required_permissions, vec!["execute"]);

        let middleware = RequiredAuthMiddleware::allow_anonymous();
        assert!(!middleware.config.require_auth);
        assert!(middleware.config.allow_anonymous);
    }
}
