use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Missing required configuration: {0}")]
    MissingConfig(String),
    #[error("Invalid configuration value: {0}")]
    InvalidValue(String),
    #[error("Configuration error: {0}")]
    Other(String),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuthConfig {
    pub auth_type: AuthType,
    pub jwt_config: Option<JwtConfig>,
    pub mtls_config: Option<MtlsConfig>,
    pub apikey_config: Option<ApiKeyConfig>,
    pub oauth2_config: Option<OAuth2Config>,
    pub dedup_config: Option<DedupConfig>,
    pub cache_config: CacheConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum AuthType {
    None,
    Jwt,
    Mtls,
    ApiKey,
    OAuth2,
}

impl std::str::FromStr for AuthType {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(AuthType::None),
            "jwt" => Ok(AuthType::Jwt),
            "mtls" => Ok(AuthType::Mtls),
            "apikey" => Ok(AuthType::ApiKey),
            "oauth2" => Ok(AuthType::OAuth2),
            _ => Err(ConfigError::InvalidValue(format!(
                "Unknown auth type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub issuer_url: String,
    pub audience: String,
    pub public_key_url: String,
    pub cache_ttl: Duration,
    pub algorithms: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MtlsConfig {
    pub ca_cert_path: String,
    pub client_cert_required: bool,
    pub verify_hostname: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKeyConfig {
    pub api_keys: Vec<String>,
    pub header_name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OAuth2Config {
    pub provider: OAuth2Provider,
    pub client_id: String,
    pub client_secret: String,
    pub token_url: String,
    pub userinfo_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum OAuth2Provider {
    Google,
    Meta,
    GitHub,
    Firebase,
    Cognito,
}

impl std::str::FromStr for OAuth2Provider {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "google" => Ok(OAuth2Provider::Google),
            "meta" => Ok(OAuth2Provider::Meta),
            "github" => Ok(OAuth2Provider::GitHub),
            "firebase" => Ok(OAuth2Provider::Firebase),
            "cognito" => Ok(OAuth2Provider::Cognito),
            _ => Err(ConfigError::InvalidValue(format!(
                "Unknown OAuth2 provider: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DedupConfig {
    pub enabled: bool,
    pub cache_ttl: Duration,
    pub cache_type: CacheType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum CacheType {
    Memory,
    Redis,
}

impl std::str::FromStr for CacheType {
    type Err = ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "memory" => Ok(CacheType::Memory),
            "redis" => Ok(CacheType::Redis),
            _ => Err(ConfigError::InvalidValue(format!(
                "Unknown cache type: {}",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub auth_cache_ttl: Duration,
    pub max_cache_size: usize,
}

impl AuthConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        let auth_type = std::env::var("AUTH_TYPE")
            .unwrap_or_else(|_| "none".to_string())
            .parse()?;

        let jwt_config = if auth_type == AuthType::Jwt {
            Some(JwtConfig::from_env()?)
        } else {
            None
        };

        let mtls_config = if auth_type == AuthType::Mtls {
            Some(MtlsConfig::from_env()?)
        } else {
            None
        };

        let apikey_config = if auth_type == AuthType::ApiKey {
            Some(ApiKeyConfig::from_env()?)
        } else {
            None
        };

        let oauth2_config = if auth_type == AuthType::OAuth2 {
            Some(OAuth2Config::from_env()?)
        } else {
            None
        };

        let dedup_config = if std::env::var("DEDUP_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false)
        {
            Some(DedupConfig::from_env()?)
        } else {
            None
        };

        let cache_config = CacheConfig::from_env()?;

        Ok(AuthConfig {
            auth_type,
            jwt_config,
            mtls_config,
            apikey_config,
            oauth2_config,
            dedup_config,
            cache_config,
        })
    }

    pub fn default() -> Self {
        Self {
            auth_type: AuthType::None,
            jwt_config: None,
            mtls_config: None,
            apikey_config: None,
            oauth2_config: None,
            dedup_config: None,
            cache_config: CacheConfig {
                auth_cache_ttl: Duration::from_secs(3600),
                max_cache_size: 1000,
            },
        }
    }
}

impl JwtConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let issuer_url = std::env::var("JWT_ISSUER_URL")
            .map_err(|_| ConfigError::MissingConfig("JWT_ISSUER_URL".to_string()))?;

        let audience = std::env::var("JWT_AUDIENCE")
            .map_err(|_| ConfigError::MissingConfig("JWT_AUDIENCE".to_string()))?;

        let public_key_url = std::env::var("JWT_PUBLIC_KEY_URL")
            .map_err(|_| ConfigError::MissingConfig("JWT_PUBLIC_KEY_URL".to_string()))?;

        let cache_ttl = std::env::var("JWT_CACHE_TTL")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<u64>()
            .map_err(|_| ConfigError::InvalidValue("JWT_CACHE_TTL must be a number".to_string()))?;

        Ok(JwtConfig {
            issuer_url,
            audience,
            public_key_url,
            cache_ttl: Duration::from_secs(cache_ttl),
            algorithms: vec!["RS256".to_string(), "ES256".to_string()],
        })
    }
}

impl MtlsConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let ca_cert_path = std::env::var("MTLS_CA_CERT_PATH")
            .map_err(|_| ConfigError::MissingConfig("MTLS_CA_CERT_PATH".to_string()))?;

        let client_cert_required = std::env::var("MTLS_CLIENT_CERT_REQUIRED")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        let verify_hostname = std::env::var("MTLS_VERIFY_HOSTNAME")
            .unwrap_or_else(|_| "true".to_string())
            .parse::<bool>()
            .unwrap_or(true);

        Ok(MtlsConfig {
            ca_cert_path,
            client_cert_required,
            verify_hostname,
        })
    }
}

impl ApiKeyConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let api_keys = std::env::var("API_KEYS")
            .map_err(|_| ConfigError::MissingConfig("API_KEYS".to_string()))?
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        if api_keys.is_empty() {
            return Err(ConfigError::InvalidValue(
                "At least one API key must be provided".to_string(),
            ));
        }

        let header_name =
            std::env::var("API_KEY_HEADER").unwrap_or_else(|_| "X-API-Key".to_string());

        Ok(ApiKeyConfig {
            api_keys,
            header_name,
        })
    }
}

impl OAuth2Config {
    fn from_env() -> Result<Self, ConfigError> {
        let provider = std::env::var("OAUTH2_PROVIDER")
            .map_err(|_| ConfigError::MissingConfig("OAUTH2_PROVIDER".to_string()))?
            .parse()?;

        let client_id = std::env::var("OAUTH2_CLIENT_ID")
            .map_err(|_| ConfigError::MissingConfig("OAUTH2_CLIENT_ID".to_string()))?;

        let client_secret = std::env::var("OAUTH2_CLIENT_SECRET")
            .map_err(|_| ConfigError::MissingConfig("OAUTH2_CLIENT_SECRET".to_string()))?;

        let token_url = std::env::var("OAUTH2_TOKEN_URL")
            .map_err(|_| ConfigError::MissingConfig("OAUTH2_TOKEN_URL".to_string()))?;

        let userinfo_url = std::env::var("OAUTH2_USERINFO_URL")
            .map_err(|_| ConfigError::MissingConfig("OAUTH2_USERINFO_URL".to_string()))?;

        Ok(OAuth2Config {
            provider,
            client_id,
            client_secret,
            token_url,
            userinfo_url,
        })
    }
}

impl DedupConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let cache_ttl = std::env::var("DEDUP_CACHE_TTL")
            .unwrap_or_else(|_| "3600".to_string())
            .parse::<u64>()
            .map_err(|_| {
                ConfigError::InvalidValue("DEDUP_CACHE_TTL must be a number".to_string())
            })?;

        let cache_type = std::env::var("DEDUP_CACHE_TYPE")
            .unwrap_or_else(|_| "memory".to_string())
            .parse()?;

        Ok(DedupConfig {
            enabled: true,
            cache_ttl: Duration::from_secs(cache_ttl),
            cache_type,
        })
    }
}

impl CacheConfig {
    fn from_env() -> Result<Self, ConfigError> {
        let auth_cache_ttl = std::env::var("AUTH_CACHE_TTL")
            .unwrap_or_else(|_| "300".to_string())
            .parse::<u64>()
            .map_err(|_| {
                ConfigError::InvalidValue("AUTH_CACHE_TTL must be a number".to_string())
            })?;

        let max_cache_size = std::env::var("MAX_CACHE_SIZE")
            .unwrap_or_else(|_| "10000".to_string())
            .parse::<usize>()
            .map_err(|_| {
                ConfigError::InvalidValue("MAX_CACHE_SIZE must be a number".to_string())
            })?;

        Ok(CacheConfig {
            auth_cache_ttl: Duration::from_secs(auth_cache_ttl),
            max_cache_size,
        })
    }
}
