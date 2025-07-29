use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use crate::executor::ExecuteResponse;
use crate::auth::config::{DedupConfig, CacheType};

#[derive(Debug, Clone)]
struct DedupEntry {
    response: ExecuteResponse,
    created_at: Instant,
    language: String,
    code_hash: String,
}

impl DedupEntry {
    fn new(response: ExecuteResponse, language: String, code_hash: String) -> Self {
        Self {
            response,
            created_at: Instant::now(),
            language,
            code_hash,
        }
    }

    fn is_expired(&self, ttl: Duration) -> bool {
        self.created_at.elapsed() > ttl
    }
}

/// Deduplication cache trait
#[async_trait::async_trait]
pub trait DedupCacheBackend: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<ExecuteResponse>, DedupError>;
    async fn set(&self, key: &str, value: &ExecuteResponse, ttl: Duration) -> Result<(), DedupError>;
    async fn delete(&self, key: &str) -> Result<(), DedupError>;
    async fn clear(&self) -> Result<(), DedupError>;
    async fn get_stats(&self) -> Result<DedupStats, DedupError>;
}

/// In-memory deduplication cache backend
pub struct MemoryDedupCache {
    cache: Arc<RwLock<HashMap<String, DedupEntry>>>,
    ttl: Duration,
    max_size: usize,
}

impl MemoryDedupCache {
    pub fn new(ttl: Duration, max_size: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl,
            max_size,
        }
    }

    async fn cleanup_expired(&self) {
        let mut cache = self.cache.write().await;
        cache.retain(|_, entry| !entry.is_expired(self.ttl));
    }

    async fn enforce_size_limit(&self) {
        let mut cache = self.cache.write().await;
        if cache.len() > self.max_size {
            // Remove oldest entries (FIFO approach)
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
impl DedupCacheBackend for MemoryDedupCache {
    async fn get(&self, key: &str) -> Result<Option<ExecuteResponse>, DedupError> {
        let cache = self.cache.read().await;
        
        if let Some(entry) = cache.get(key) {
            if entry.is_expired(self.ttl) {
                return Ok(None);
            }
            return Ok(Some(entry.response.clone()));
        }
        
        Ok(None)
    }

    async fn set(&self, key: &str, value: &ExecuteResponse, _ttl: Duration) -> Result<(), DedupError> {
        // For memory cache, we use the configured TTL
        let entry = DedupEntry::new(
            value.clone(),
            "unknown".to_string(), // We don't store language in memory cache
            key.to_string(),
        );
        
        {
            let mut cache = self.cache.write().await;
            cache.insert(key.to_string(), entry);
        }
        
        // Cleanup and enforce size limit
        self.cleanup_expired().await;
        self.enforce_size_limit().await;
        
        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), DedupError> {
        let mut cache = self.cache.write().await;
        cache.remove(key);
        Ok(())
    }

    async fn clear(&self) -> Result<(), DedupError> {
        let mut cache = self.cache.write().await;
        cache.clear();
        Ok(())
    }

    async fn get_stats(&self) -> Result<DedupStats, DedupError> {
        let cache = self.cache.read().await;
        let total_entries = cache.len();
        let expired_entries = cache.values().filter(|entry| entry.is_expired(self.ttl)).count();
        
        Ok(DedupStats {
            total_entries,
            expired_entries,
            cache_type: "memory".to_string(),
        })
    }
}

/// Main deduplication cache that uses in-memory storage
pub struct DedupCache {
    backend: Box<dyn DedupCacheBackend>,
    ttl: Duration,
    enabled: bool,
}

impl DedupCache {
    pub async fn new(config: &DedupConfig) -> Result<Self, DedupError> {
        if !config.enabled {
            return Ok(Self {
                backend: Box::new(MemoryDedupCache::new(Duration::from_secs(1), 1)),
                ttl: Duration::from_secs(1),
                enabled: false,
            });
        }

        let backend: Box<dyn DedupCacheBackend> = match config.cache_type {
            CacheType::Memory => {
                Box::new(MemoryDedupCache::new(config.cache_ttl, 10000))
            }
            CacheType::Redis => {
                // Fallback to memory cache if Redis is configured but not available
                log::warn!("Redis cache type configured but Redis is not available, falling back to memory cache");
                Box::new(MemoryDedupCache::new(config.cache_ttl, 10000))
            }
        };

        Ok(Self {
            backend,
            ttl: config.cache_ttl,
            enabled: config.enabled,
        })
    }

    pub async fn check_duplicate(&self, language: &str, code: &str) -> Result<Option<ExecuteResponse>, DedupError> {
        if !self.enabled {
            return Ok(None);
        }

        let key = self.generate_hash_key(language, code);
        self.backend.get(&key).await
    }

    pub async fn store_result(&self, language: &str, code: &str, response: &ExecuteResponse) -> Result<(), DedupError> {
        if !self.enabled {
            return Ok(());
        }

        let key = self.generate_hash_key(language, code);
        self.backend.set(&key, response, self.ttl).await
    }

    pub async fn invalidate(&self, language: &str, code: &str) -> Result<(), DedupError> {
        if !self.enabled {
            return Ok(());
        }

        let key = self.generate_hash_key(language, code);
        self.backend.delete(&key).await
    }

    pub async fn clear_all(&self) -> Result<(), DedupError> {
        self.backend.clear().await
    }

    pub async fn get_stats(&self) -> Result<DedupStats, DedupError> {
        self.backend.get_stats().await
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn generate_hash_key(&self, language: &str, code: &str) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(language.as_bytes());
        hasher.update(code.as_bytes());
        
        format!("dedup:{}:{}", language, hex::encode(hasher.finalize()))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DedupStats {
    pub total_entries: usize,
    pub expired_entries: usize,
    pub cache_type: String,
}

#[derive(Debug, thiserror::Error)]
pub enum DedupError {
    #[error("Cache connection error: {0}")]
    Connection(String),
    #[error("Cache operation failed: {0}")]
    Operation(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_response() -> ExecuteResponse {
        ExecuteResponse {
            success: true,
            output: "Hello, World!".to_string(),
            error: None,
            execution_time: 0.1,
            memory_usage: 1024,
        }
    }

    #[tokio::test]
    async fn test_memory_dedup_cache() {
        let cache = MemoryDedupCache::new(Duration::from_secs(60), 100);
        let response = create_test_response();
        
        // Test set and get
        cache.set("key1", &response, Duration::from_secs(60)).await.unwrap();
        let result = cache.get("key1").await.unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().output, "Hello, World!");
        
        // Test delete
        cache.delete("key1").await.unwrap();
        let result = cache.get("key1").await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_dedup_cache() {
        let config = DedupConfig {
            enabled: true,
            cache_type: CacheType::Memory,
            cache_ttl: Duration::from_secs(60),
        };
        
        let cache = DedupCache::new(&config).await.unwrap();
        let response = create_test_response();
        
        // Test deduplication
        let result1 = cache.check_duplicate("python", "print('hello')").await.unwrap();
        assert!(result1.is_none());
        
        cache.store_result("python", "print('hello')", &response).await.unwrap();
        
        let result2 = cache.check_duplicate("python", "print('hello')").await.unwrap();
        assert!(result2.is_some());
        assert_eq!(result2.unwrap().output, "Hello, World!");
    }

    #[tokio::test]
    async fn test_dedup_cache_disabled() {
        let config = DedupConfig {
            enabled: false,
            cache_type: CacheType::Memory,
            cache_ttl: Duration::from_secs(60),
        };
        
        let cache = DedupCache::new(&config).await.unwrap();
        let response = create_test_response();
        
        // Test that cache is disabled
        let result = cache.check_duplicate("python", "print('hello')").await.unwrap();
        assert!(result.is_none());
        
        cache.store_result("python", "print('hello')", &response).await.unwrap();
        
        let result = cache.check_duplicate("python", "print('hello')").await.unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_hash_key_generation() {
        let config = DedupConfig {
            enabled: true,
            cache_type: CacheType::Memory,
            cache_ttl: Duration::from_secs(60),
        };
        
        let cache = DedupCache::new(&config).block_on().unwrap();
        
        let key1 = cache.generate_hash_key("python", "print('hello')");
        let key2 = cache.generate_hash_key("python", "print('hello')");
        assert_eq!(key1, key2); // Same input should generate same key
        
        let key3 = cache.generate_hash_key("python", "print('world')");
        assert_ne!(key1, key3); // Different input should generate different key
    }
} 