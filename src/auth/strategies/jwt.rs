use actix_web::HttpRequest;
use async_trait::async_trait;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use super::{AuthStrategy, AuthResult, AuthError, TokenExtractor};
use crate::auth::config::JwtConfig;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: Option<String>,
    iss: Option<String>,
    aud: Option<String>,
    exp: Option<u64>,
    iat: Option<u64>,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKeys {
    keys: Vec<GooglePublicKey>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GooglePublicKey {
    kid: String,
    n: String,
    e: String,
    alg: String,
    kty: String,
    use_: Option<String>,
    #[serde(rename = "use")]
    use_field: Option<String>,
}

/// JWT authentication strategy
pub struct JwtAuthStrategy {
    config: JwtConfig,
    http_client: Client,
    public_keys: Arc<RwLock<HashMap<String, DecodingKey>>>,
}

impl JwtAuthStrategy {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            config,
            http_client: Client::new(),
            public_keys: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    async fn fetch_public_keys(&self) -> Result<HashMap<String, DecodingKey>, AuthError> {
        let response = self.http_client
            .get(&self.config.public_key_url)
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to fetch public keys: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::Network(format!(
                "Failed to fetch public keys: HTTP {}", 
                response.status()
            )));
        }

        let keys: GooglePublicKeys = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse public keys: {}", e)))?;

        let mut public_keys = HashMap::new();
        
        for key in keys.keys {
            if let Ok(decoding_key) = self.create_decoding_key(&key) {
                public_keys.insert(key.kid, decoding_key);
            }
        }

        Ok(public_keys)
    }

    fn create_decoding_key(&self, key: &GooglePublicKey) -> Result<DecodingKey, AuthError> {
        // For Google's JWKS, we need to construct the RSA public key
        // This is a simplified implementation - in production, you'd want to use a proper JWKS library
        if key.kty == "RSA" && key.alg == "RS256" {
            // For now, we'll use a placeholder approach
            // In a real implementation, you'd reconstruct the RSA public key from n and e
            Err(AuthError::Internal("RSA key reconstruction not implemented".to_string()))
        } else {
            Err(AuthError::Internal(format!("Unsupported key type: {} or algorithm: {}", key.kty, key.alg)))
        }
    }

    async fn verify_token(&self, token: &str) -> Result<Claims, AuthError> {
        // Decode header to get key ID
        let header = decode_header(token)
            .map_err(|e| AuthError::InvalidToken(format!("Invalid token header: {}", e)))?;

        let kid = header.kid
            .ok_or_else(|| AuthError::InvalidToken("No key ID in token header".to_string()))?;

        // Get the appropriate public key
        let public_keys = self.public_keys.read().await;
        let decoding_key = public_keys.get(&kid)
            .ok_or_else(|| AuthError::InvalidToken("Unknown key ID".to_string()))?;

        // Validate and decode the token
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[&self.config.issuer_url]);

        let token_data = decode::<Claims>(token, decoding_key, &validation)
            .map_err(|e| AuthError::InvalidToken(format!("Token validation failed: {}", e)))?;

        Ok(token_data.claims)
    }

    fn extract_token_from_header(&self, request: &HttpRequest) -> Result<String, AuthError> {
        let auth_header = request.headers()
            .get("Authorization")
            .ok_or_else(|| AuthError::MissingCredentials("No Authorization header".to_string()))?
            .to_str()
            .map_err(|_| AuthError::MissingCredentials("Invalid Authorization header".to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err(AuthError::MissingCredentials("Invalid Authorization header format".to_string()));
        }

        Ok(auth_header[7..].to_string())
    }
}

#[async_trait]
impl AuthStrategy for JwtAuthStrategy {
    async fn authenticate(&self, request: &HttpRequest) -> Result<AuthResult, AuthError> {
        // Extract token from request
        let token = self.extract_token_from_header(&request)?;

        // Verify the token
        let claims = self.verify_token(&token).await?;

        // Extract user information
        let user_id = claims.sub
            .or(claims.email.clone())
            .ok_or_else(|| AuthError::Authentication("No user ID in token".to_string()))?;

        let permissions = vec!["execute".to_string()];

        // Build metadata
        let mut metadata = HashMap::new();
        if let Some(email) = claims.email {
            metadata.insert("email".to_string(), email);
        }
        if let Some(name) = claims.name {
            metadata.insert("name".to_string(), name);
        }
        if let Some(picture) = claims.picture {
            metadata.insert("picture".to_string(), picture);
        }
        if let Some(iss) = claims.iss {
            metadata.insert("issuer".to_string(), iss);
        }

        Ok(AuthResult::authenticated(user_id, permissions)
            .with_metadata("token_type".to_string(), "jwt".to_string()))
    }

    fn name(&self) -> &'static str {
        "jwt"
    }
}

impl TokenExtractor for JwtAuthStrategy {
    fn extract_token(&self, request: &HttpRequest) -> Result<String, AuthError> {
        self.extract_token_from_header(request)
    }
}

impl JwtAuthStrategy {
    /// Initialize the strategy by fetching public keys
    pub async fn initialize(&self) -> Result<(), AuthError> {
        let keys = self.fetch_public_keys().await?;
        let mut public_keys = self.public_keys.write().await;
        *public_keys = keys;
        Ok(())
    }

    /// Refresh public keys (useful for key rotation)
    pub async fn refresh_keys(&self) -> Result<(), AuthError> {
        self.initialize().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_web::test]
    async fn test_jwt_auth_strategy_missing_header() {
        let config = JwtConfig {
            issuer_url: "https://accounts.google.com".to_string(),
            audience: "test-audience".to_string(),
            public_key_url: "https://www.googleapis.com/oauth2/v1/certs".to_string(),
            cache_ttl: std::time::Duration::from_secs(3600),
            algorithms: vec!["RS256".to_string()],
        };

        let strategy = JwtAuthStrategy::new(config);
        let req = test::TestRequest::default().to_http_request();
        
        let result = strategy.authenticate(req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::MissingCredentials(_) => {},
            _ => panic!("Expected MissingCredentials error"),
        }
    }

    #[test]
    fn test_jwt_auth_strategy_name() {
        let config = JwtConfig {
            issuer_url: "https://accounts.google.com".to_string(),
            audience: "test-audience".to_string(),
            public_key_url: "https://www.googleapis.com/oauth2/v1/certs".to_string(),
            cache_ttl: std::time::Duration::from_secs(3600),
            algorithms: vec!["RS256".to_string()],
        };

        let strategy = JwtAuthStrategy::new(config);
        assert_eq!(strategy.name(), "jwt");
    }
} 