use actix_web::HttpRequest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use super::strategies::AuthResult;
use crate::auth::config::CacheConfig;

#[derive(Debug, Clone)]
struct CachedAuthResult {
    auth_result: AuthResult,
    created_at: Instant,
    ttl: Duration,
}

impl CachedAuthResult {
    fn new(auth_result: AuthResult, ttl: Duration) -> Self {
        Self {
            auth_result,
            created_at: Instant::now(),
            ttl,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }
}

/// Authentication cache trait
#[async_trait::async_trait]
pub trait AuthCacheBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<AuthResult>, CacheError>;
    async fn set(&self, key: &str, value: &AuthResult, ttl: Duration) -> Result<(), CacheError>;
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
    async fn clear(&self) -> Result<(), CacheError>;
}

/// In-memory authentication cache backend
pub struct MemoryAuthCache {
    cache: Arc<RwLock<HashMap<String, CachedAuthResult>>>,
    max_size: usize,
}

impl MemoryAuthCache {
    pub fn new(max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            max_size,
        }
    }

    async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, cached_result| !cached_result.is_expired());
    }

    async fn enforce_size_limit(&self) {
        let mut cache = self.cache.write().await;
        if cache.len() > self.max_size {
            // Remove oldest entries (simple FIFO approach)
            let mut entries: Vec<_> = cache.drain().collect();
            entries.sort_by(|a, b| a.1.created_at.cmp(&b.1.created_at));
            
            // Keep only the newest entries
            let to_keep = self.max_size;
            for (key, value) in entries.into_iter().take(to_keep) {
                cache.insert(key, value);
            }
        }
    }
}

#[async_trait::async_trait]
impl AuthCacheBackend for MemoryAuthCache {
    async fn get(&self, key: &str) -> Result<Option<AuthResult>, CacheError> {
        let cache = self.cache.read().await;
        
        if let Some(cached_result) = cache.get(key) {
            if cached_result.is_expired() {
                // Return None for expired entries, they'll be cleaned up later
                return Ok(None);
            }
            return Ok(Some(cached_result.auth_result.clone()));
        }
        
        Ok(None)
    }

    async fn set(&self, key: &str, value: &AuthResult, ttl: Duration) -> Result<(), CacheError> {
        let cached_result = CachedAuthResult::new(value.clone(), ttl);
        
        {
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), cached_result);
        }
        
        // Cleanup and enforce size limit
        self.cleanup_expired().await;
        self.enforce_size_limit().await;
        
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    async fn clear(&self) -> Result<(), CacheError> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }
}

/// Main authentication cache that uses in-memory storage
pub struct AuthCache {
    backend: Box<dyn AuthCacheBackend>,
    ttl: Duration,
}

impl AuthCache {
    pub async fn new(config: &CacheConfig) -> Result<Self, CacheError> {
        let backend: Box<dyn AuthCacheBackend> = Box::new(MemoryAuthCache::new(config.max_cache_size));

        Ok(Self {
            backend,
            ttl: config.auth_cache_ttl,
        })
    }

    pub async fn get_cached_auth(&self, request: &HttpRequest) -> Result<Option<AuthResult>, CacheError> {
        let key = self.generate_cache_key(request);
        self.backend.get(&key).await
    }

    pub async fn cache_auth_result(&self, request: &HttpRequest, auth_result: &AuthResult) -> Result<(), CacheError> {
        let key = self.generate_cache_key(request);
        self.backend.set(&key, auth_result, self.ttl).await
    }

    pub async fn invalidate_auth(&self, request: &HttpRequest) -> Result<(), CacheError> {
        let key = self.generate_cache_key(request);
        self.backend.delete(&key).await
    }

    pub async fn clear_all(&self) -> Result<(), CacheError> {
        self.backend.clear().await
    }

    fn generate_cache_key(&self, request: &HttpRequest) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        
        // Include authentication-related headers in the cache key
        if let Some(auth_header) = request.headers().get("Authorization") {
            if let Ok(auth_value) = auth_header.to_str() {
                hasher.update(auth_value.as_bytes());
            }
        }
        
        if let Some(api_key_header) = request.headers().get("X-API-Key") {
            if let Ok(api_key_value) = api_key_header.to_str() {
                hasher.update(api_key_value.as_bytes());
            }
        }
        
        if let Some(client_cert_header) = request.headers().get("X-Client-Certificate") {
            if let Ok(cert_value) = client_cert_header.to_str() {
                hasher.update(cert_value.as_bytes());
            }
        }
        
        // Include user agent and IP for additional uniqueness
        if let Some(user_agent) = request.headers().get("User-Agent") {
            if let Ok(ua_value) = user_agent.to_str() {
                hasher.update(ua_value.as_bytes());
            }
        }
        
        // Include connection info if available
        if let Some(connection_info) = request.connection_info() {
            hasher.update(connection_info.peer_addr().unwrap_or("unknown").as_bytes());
        }
        
        format!("auth:{}", hex::encode(hasher.finalize()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CacheError {
    #[error("Cache connection error: {0}")]
    Connection(String),
    #[error("Cache operation failed: {0}")]
    Operation(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[tokio::test]
    async fn test_memory_cache() {
        let cache = MemoryAuthCache::new(100);
        let auth_result = AuthResult::authenticated("user1".to_string(), vec!["read".to_string()]);
        
        // Test set and get
        cache.set("key1", &auth_result, Duration::from_secs(60)).await.unwrap();
        let result = cache.get("key1").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().user_id, Some("user1".to_string()));
        
        // Test delete
        cache.delete("key1").await.unwrap();
        let result = cache.get("key1").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_auth_cache() {
        let config = CacheConfig {
            max_cache_size: 100,
            auth_cache_ttl: Duration::from_secs(60),
            redis_url: None,
        };
        
        let cache = AuthCache::new(&config).await.unwrap();
        let auth_result = AuthResult::authenticated("user1".to_string(), vec!["read".to_string()]);
        
        let req = test::TestRequest::default()
            .insert_header(("Authorization", "Bearer token123"))
            .to_http_request();
        
        // Test caching
        cache.cache_auth_result(&req, &auth_result).await.unwrap();
        let cached = cache.get_cached_auth(&req).await.unwrap();
        assert!(cached.is_some());
        
        // Test invalidation
        cache.invalidate_auth(&req).await.unwrap();
        let cached = cache.get_cached_auth(&req).await.unwrap();
        assert!(cached.is_none());
    }

    #[test]
    fn test_cache_key_generation() {
        let config = CacheConfig {
            max_cache_size: 100,
            auth_cache_ttl: Duration::from_secs(60),
            redis_url: None,
        };
        
        let cache = AuthCache::new(&config).block_on().unwrap();
        let req = test::TestRequest::default()
            .insert_header(("Authorization", "Bearer token123"))
            .insert_header(("X-API-Key", "key456"))
            .to_http_request();
        
        let key1 = cache.generate_cache_key(&req);
        let key2 = cache.generate_cache_key(&req);
        assert_eq!(key1, key2); // Same request should generate same key
    }
} 