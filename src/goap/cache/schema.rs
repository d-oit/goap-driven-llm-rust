//! Schema caching
//!
//! Provides LRU caching for validation schemas.

use lru::LruCache;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::num::NonZeroUsize;

/// Cache for validation schemas
#[derive(Debug)]
pub struct SchemaCache {
    /// LRU cache for schemas
    cache: LruCache<String, CachedSchema>,

    /// Maximum cache size
    max_size: usize,
}

/// Cached schema with metadata
#[derive(Debug, Clone)]
pub struct CachedSchema {
    /// Schema content
    pub content: String,

    /// Schema type (e.g., "github_workflow", "dockerfile")
    pub schema_type: String,

    /// Creation time
    pub created_at: std::time::Instant,

    /// Number of times accessed
    pub access_count: u32,

    /// Schema version
    pub version: String,
}

impl SchemaCache {
    /// Create a new schema cache
    pub fn new(max_size: usize) -> Self {
        let cache = LruCache::new(
            NonZeroUsize::new(max_size).unwrap_or_else(|| NonZeroUsize::new(100).unwrap()),
        );
        SchemaCache { cache, max_size }
    }

    /// Get a schema from cache
    pub fn get(&mut self, schema_type: &str) -> Option<&CachedSchema> {
        if let Some(schema) = self.cache.get(schema_type) {
            // Increment access count (need to clone and reinsert)
            let mut schema = schema.clone();
            schema.access_count += 1;
            self.cache.put(schema_type.to_string(), schema.clone());
            Some(Box::leak(Box::new(schema)))
        } else {
            None
        }
    }

    /// Put a schema in the cache
    pub fn put(&mut self, schema_type: String, schema: CachedSchema) {
        self.cache.put(schema_type, schema);
    }

    /// Check if a schema type is cached
    pub fn contains(&mut self, schema_type: &str) -> bool {
        self.cache.contains(schema_type)
    }

    /// Remove a schema from cache
    pub fn remove(&mut self, schema_type: &str) -> Option<CachedSchema> {
        self.cache.pop(schema_type)
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if cache is empty
    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }

    /// Get maximum cache size
    pub fn capacity(&self) -> usize {
        self.max_size
    }

    /// Get all cached schema types
    pub fn get_cached_types(&self) -> Vec<String> {
        self.cache.iter().map(|(k, _)| k.clone()).collect()
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let total_accesses: u32 = self.cache.iter().map(|(_, v)| v.access_count).sum();

        CacheStats {
            size: self.cache.len(),
            capacity: self.max_size,
            total_accesses,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    /// Current number of items
    pub size: usize,

    /// Maximum capacity
    pub capacity: usize,

    /// Total number of accesses
    pub total_accesses: u32,
}

impl Default for SchemaCache {
    fn default() -> Self {
        Self::new(1000)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_cache_creation() {
        let cache = SchemaCache::new(100);
        assert_eq!(cache.capacity(), 100);
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_put_and_get() {
        let mut cache = SchemaCache::new(100);
        let schema = CachedSchema {
            content: "schema content".to_string(),
            schema_type: "test".to_string(),
            created_at: std::time::Instant::now(),
            access_count: 0,
            version: "1.0".to_string(),
        };

        cache.put("test".to_string(), schema.clone());
        assert_eq!(cache.len(), 1);
        assert!(cache.contains("test"));
    }

    #[test]
    fn test_remove() {
        let mut cache = SchemaCache::new(100);
        let schema = CachedSchema {
            content: "schema content".to_string(),
            schema_type: "test".to_string(),
            created_at: std::time::Instant::now(),
            access_count: 0,
            version: "1.0".to_string(),
        };

        cache.put("test".to_string(), schema);
        assert_eq!(cache.len(), 1);

        let removed = cache.remove("test");
        assert!(removed.is_some());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_clear() {
        let mut cache = SchemaCache::new(100);
        let schema = CachedSchema {
            content: "schema content".to_string(),
            schema_type: "test".to_string(),
            created_at: std::time::Instant::now(),
            access_count: 0,
            version: "1.0".to_string(),
        };

        cache.put("test".to_string(), schema);
        assert_eq!(cache.len(), 1);

        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_stats() {
        let mut cache = SchemaCache::new(100);
        let schema = CachedSchema {
            content: "schema content".to_string(),
            schema_type: "test".to_string(),
            created_at: std::time::Instant::now(),
            access_count: 0,
            version: "1.0".to_string(),
        };

        cache.put("test".to_string(), schema);
        let stats = cache.stats();
        assert_eq!(stats.size, 1);
        assert_eq!(stats.capacity, 100);
    }
}
