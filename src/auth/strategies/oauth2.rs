use actix_web::HttpRequest;
use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::{AuthStrategy, AuthResult, AuthError, TokenExtractor};
use crate::auth::config::{OAuth2Config, OAuth2Provider};

#[derive(Debug, Serialize, Deserialize)]
struct OAuth2TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
    scope: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OAuth2UserInfo {
    id: Option<String>,
    sub: Option<String>,
    email: Option<String>,
    name: Option<String>,
    picture: Option<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_json::Value>,
}

/// OAuth2 authentication strategy
pub struct OAuth2AuthStrategy {
    config: OAuth2Config,
    http_client: Client,
}

impl OAuth2AuthStrategy {
    pub fn new(config: OAuth2Config) -> Self {
        Self {
            config,
            http_client: Client::new(),
        }
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

    async fn verify_token_with_provider(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        match &self.config.provider {
            OAuth2Provider::Google => self.verify_google_token(token).await,
            OAuth2Provider::Meta => self.verify_meta_token(token).await,
            OAuth2Provider::GitHub => self.verify_github_token(token).await,
            OAuth2Provider::Firebase => self.verify_firebase_token(token).await,
            OAuth2Provider::Cognito => self.verify_cognito_token(token).await,
        }
    }

    async fn verify_google_token(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        let response = self.http_client
            .get("https://www.googleapis.com/oauth2/v2/userinfo")
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to verify Google token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidToken(format!("Google token verification failed: HTTP {}", response.status())));
        }

        let user_info: OAuth2UserInfo = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse Google user info: {}", e)))?;

        Ok(user_info)
    }

    async fn verify_meta_token(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        let response = self.http_client
            .get("https://graph.facebook.com/me")
            .query(&[("access_token", token), ("fields", "id,name,email,picture")])
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to verify Meta token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidToken(format!("Meta token verification failed: HTTP {}", response.status())));
        }

        let user_info: OAuth2UserInfo = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse Meta user info: {}", e)))?;

        Ok(user_info)
    }

    async fn verify_github_token(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        let response = self.http_client
            .get("https://api.github.com/user")
            .header("Authorization", format!("Bearer {}", token))
            .header("User-Agent", "isobox-auth")
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to verify GitHub token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidToken(format!("GitHub token verification failed: HTTP {}", response.status())));
        }

        let user_info: OAuth2UserInfo = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse GitHub user info: {}", e)))?;

        Ok(user_info)
    }

    async fn verify_firebase_token(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        // Firebase tokens are JWTs, so we could verify them locally
        // For now, we'll use the Firebase Admin SDK approach via HTTP
        let response = self.http_client
            .post("https://identitytoolkit.googleapis.com/v1/accounts:lookup")
            .query(&[("key", &self.config.client_id)])
            .json(&serde_json::json!({
                "idToken": token
            }))
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to verify Firebase token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidToken(format!("Firebase token verification failed: HTTP {}", response.status())));
        }

        // Parse Firebase response and extract user info
        let firebase_response: serde_json::Value = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse Firebase response: {}", e)))?;

        // Extract user info from Firebase response
        if let Some(users) = firebase_response.get("users").and_then(|u| u.as_array()) {
            if let Some(user) = users.first() {
                let user_info = OAuth2UserInfo {
                    id: user.get("localId").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    sub: user.get("localId").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    email: user.get("email").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    name: user.get("displayName").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    picture: user.get("photoUrl").and_then(|v| v.as_str()).map(|s| s.to_string()),
                    extra: HashMap::new(),
                };
                return Ok(user_info);
            }
        }

        Err(AuthError::InvalidToken("Invalid Firebase response format".to_string()))
    }

    async fn verify_cognito_token(&self, token: &str) -> Result<OAuth2UserInfo, AuthError> {
        // AWS Cognito tokens are JWTs, so we could verify them locally
        // For now, we'll use a simplified approach
        let response = self.http_client
            .get(&self.config.userinfo_url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await
            .map_err(|e| AuthError::Network(format!("Failed to verify Cognito token: {}", e)))?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidToken(format!("Cognito token verification failed: HTTP {}", response.status())));
        }

        let user_info: OAuth2UserInfo = response.json().await
            .map_err(|e| AuthError::Internal(format!("Failed to parse Cognito user info: {}", e)))?;

        Ok(user_info)
    }
}

#[async_trait]
impl AuthStrategy for OAuth2AuthStrategy {
    async fn authenticate(&self, request: HttpRequest) -> Result<AuthResult, AuthError> {
        // Extract token from request
        let token = self.extract_token_from_header(&request)?;

        // Verify token with the appropriate provider
        let user_info = self.verify_token_with_provider(&token).await?;

        // Extract user information
        let user_id = user_info.id
            .or(user_info.sub.clone())
            .or(user_info.email.clone())
            .ok_or_else(|| AuthError::Authentication("No user ID in OAuth2 response".to_string()))?;

        // Build permissions based on provider
        let permissions = match &self.config.provider {
            OAuth2Provider::Google => vec!["execute".to_string(), "read".to_string()],
            OAuth2Provider::Meta => vec!["execute".to_string(), "read".to_string()],
            OAuth2Provider::GitHub => vec!["execute".to_string(), "read".to_string(), "write".to_string()],
            OAuth2Provider::Firebase => vec!["execute".to_string(), "read".to_string()],
            OAuth2Provider::Cognito => vec!["execute".to_string(), "read".to_string(), "admin".to_string()],
        };

        // Build metadata
        let mut metadata = HashMap::new();
        if let Some(email) = user_info.email {
            metadata.insert("email".to_string(), email);
        }
        if let Some(name) = user_info.name {
            metadata.insert("name".to_string(), name);
        }
        if let Some(picture) = user_info.picture {
            metadata.insert("picture".to_string(), picture);
        }
        metadata.insert("provider".to_string(), format!("{:?}", self.config.provider));

        Ok(AuthResult::authenticated(user_id, permissions)
            .with_metadata("auth_type".to_string(), "oauth2".to_string()))
    }

    fn name(&self) -> &'static str {
        "oauth2"
    }
}

impl TokenExtractor for OAuth2AuthStrategy {
    fn extract_token(&self, request: &HttpRequest) -> Result<String, AuthError> {
        self.extract_token_from_header(request)
    }
}

impl OAuth2AuthStrategy {
    /// Get the OAuth2 provider name
    pub fn provider_name(&self) -> &'static str {
        match &self.config.provider {
            OAuth2Provider::Google => "google",
            OAuth2Provider::Meta => "meta",
            OAuth2Provider::GitHub => "github",
            OAuth2Provider::Firebase => "firebase",
            OAuth2Provider::Cognito => "cognito",
        }
    }

    /// Get the client ID
    pub fn client_id(&self) -> &str {
        &self.config.client_id
    }

    /// Check if the strategy is configured for a specific provider
    pub fn is_provider(&self, provider: OAuth2Provider) -> bool {
        std::mem::discriminant(&self.config.provider) == std::mem::discriminant(&provider)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    fn create_test_config() -> OAuth2Config {
        OAuth2Config {
            provider: OAuth2Provider::Google,
            client_id: "test-client-id".to_string(),
            client_secret: "test-client-secret".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            userinfo_url: "https://www.googleapis.com/oauth2/v2/userinfo".to_string(),
        }
    }

    #[actix_web::test]
    async fn test_oauth2_auth_strategy_missing_header() {
        let strategy = OAuth2AuthStrategy::new(create_test_config());
        let req = test::TestRequest::default().to_http_request();
        
        let result = strategy.authenticate(req).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            AuthError::MissingCredentials(_) => {},
            _ => panic!("Expected MissingCredentials error"),
        }
    }

    #[test]
    fn test_oauth2_auth_strategy_name() {
        let strategy = OAuth2AuthStrategy::new(create_test_config());
        assert_eq!(strategy.name(), "oauth2");
    }

    #[test]
    fn test_oauth2_provider_methods() {
        let strategy = OAuth2AuthStrategy::new(create_test_config());
        
        assert_eq!(strategy.provider_name(), "google");
        assert_eq!(strategy.client_id(), "test-client-id");
        assert!(strategy.is_provider(OAuth2Provider::Google));
        assert!(!strategy.is_provider(OAuth2Provider::GitHub));
    }

    #[test]
    fn test_oauth2_provider_parsing() {
        assert_eq!("google".parse::<OAuth2Provider>().unwrap(), OAuth2Provider::Google);
        assert_eq!("meta".parse::<OAuth2Provider>().unwrap(), OAuth2Provider::Meta);
        assert_eq!("github".parse::<OAuth2Provider>().unwrap(), OAuth2Provider::GitHub);
        assert_eq!("firebase".parse::<OAuth2Provider>().unwrap(), OAuth2Provider::Firebase);
        assert_eq!("cognito".parse::<OAuth2Provider>().unwrap(), OAuth2Provider::Cognito);
        
        assert!("invalid".parse::<OAuth2Provider>().is_err());
    }
} 