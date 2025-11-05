//! Intelligent caching with pattern detection
//!
//! Combines pattern and schema caching with intelligent similarity detection.

#[allow(unused_imports)]
use crate::goap::actions::ActionType;
#[allow(unused_imports)]
use crate::goap::cache::pattern::{PatternSimilarity, SuccessPattern};
use crate::goap::cache::schema::{CachedSchema, SchemaCache};
use dashmap::DashMap;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Intelligent cache that manages patterns and schemas
#[derive(Debug, Clone)]
pub struct IntelligentCache {
    /// Pattern cache (thread-safe)
    patterns: Arc<DashMap<String, SuccessPattern>>,

    /// Schema cache (thread-safe)
    schemas: Arc<Mutex<SchemaCache>>,

    /// Pattern lookup index for fast similarity search
    pattern_index: Arc<DashMap<String, Vec<String>>>,

    /// Cache statistics
    stats: Arc<DashMap<String, u64>>,
}

impl IntelligentCache {
    /// Create a new intelligent cache
    pub fn new(_pattern_cache_size: usize, schema_cache_size: usize) -> Self {
        IntelligentCache {
            patterns: Arc::new(DashMap::new()),
            schemas: Arc::new(Mutex::new(SchemaCache::new(schema_cache_size))),
            pattern_index: Arc::new(DashMap::new()),
            stats: Arc::new(DashMap::new()),
        }
    }

    /// Store a success pattern
    pub fn store_pattern(&self, pattern: SuccessPattern) -> String {
        let pattern_id = pattern.id.clone();

        self.patterns.insert(pattern_id.clone(), pattern);

        // Update index
        let signature = self
            .patterns
            .get(&pattern_id)
            .map(|p| p.signature.clone())
            .unwrap_or_default();

        if let Some(mut index_entry) = self.pattern_index.get_mut(&signature) {
            index_entry.push(pattern_id.clone());
        } else {
            self.pattern_index
                .insert(signature, vec![pattern_id.clone()]);
        }

        self.increment_stat("patterns_stored");
        pattern_id
    }

    /// Find similar patterns
    pub fn find_similar_patterns(&self, signature: &str, min_confidence: f64) -> Vec<PatternMatch> {
        self.increment_stat("pattern_lookups");

        // Get patterns with similar signatures
        let similar_patterns = if let Some(index_entry) = self.pattern_index.get(signature) {
            index_entry.value().clone()
        } else {
            Vec::new()
        };

        let mut matches = Vec::new();

        for pattern_id in similar_patterns {
            if let Some(pattern_ref) = self.patterns.get(&pattern_id) {
                let pattern = pattern_ref.value();

                if pattern.confidence >= min_confidence {
                    // Calculate similarity
                    let similarity = self.calculate_similarity(signature, &pattern.signature);

                    if similarity.score >= 0.7 {
                        matches.push(PatternMatch {
                            pattern_id: pattern.id.clone(),
                            confidence: pattern.confidence,
                            similarity_score: similarity.score,
                            estimated_tokens: pattern.avg_tokens,
                            usage_count: pattern.usage_count,
                        });

                        self.increment_stat("pattern_hits");
                    }
                }
            }
        }

        // Sort by confidence and similarity
        matches.sort_by(|a, b| {
            b.confidence
                .partial_cmp(&a.confidence)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches
    }

    /// Get a pattern by ID
    pub fn get_pattern(&self, pattern_id: &str) -> Option<SuccessPattern> {
        self.increment_stat("pattern_gets");

        self.patterns.get(pattern_id).map(|r| r.value().clone())
    }

    /// Delete a pattern
    pub fn delete_pattern(&self, pattern_id: &str) -> bool {
        if let Some((_, pattern_ref)) = self.patterns.remove(pattern_id) {
            let pattern = pattern_ref;

            // Remove from index
            let signature = pattern.signature;
            if let Some(mut index_entry) = self.pattern_index.get_mut(&signature) {
                index_entry.retain(|id| id != pattern_id);
            }

            self.increment_stat("patterns_deleted");
            true
        } else {
            false
        }
    }

    /// Store a schema
    pub fn store_schema(&self, schema_type: String, content: String, version: String) {
        let cached_schema = CachedSchema {
            content,
            schema_type: schema_type.clone(),
            created_at: std::time::Instant::now(),
            access_count: 0,
            version,
        };

        {
            let mut schemas = self.schemas.lock().unwrap();
            schemas.put(schema_type, cached_schema);
        }
        self.increment_stat("schemas_stored");
    }

    /// Get a schema
    pub fn get_schema(&self, schema_type: &str) -> Option<CachedSchema> {
        self.increment_stat("schema_lookups");

        // Clone the schema since LruCache requires ownership
        let mut schemas = self.schemas.lock().unwrap();
        schemas.get(schema_type).cloned()
    }

    /// Get all patterns
    pub fn get_all_patterns(&self) -> Vec<SuccessPattern> {
        self.patterns.iter().map(|r| r.value().clone()).collect()
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> CacheStatistics {
        let schemas_count = {
            let schemas = self.schemas.lock().unwrap();
            schemas.len() as u64
        };

        CacheStatistics {
            patterns_count: self.patterns.len() as u64,
            schemas_count,
            pattern_lookups: self.get_stat("pattern_lookups"),
            pattern_hits: self.get_stat("pattern_hits"),
            schema_lookups: self.get_stat("schema_lookups"),
            cache_hit_rate: if self.get_stat("pattern_lookups") > 0 {
                (self.get_stat("pattern_hits") as f64 / self.get_stat("pattern_lookups") as f64)
                    * 100.0
            } else {
                0.0
            },
        }
    }

    /// Calculate similarity between two signatures
    fn calculate_similarity(&self, sig1: &str, sig2: &str) -> PatternSimilarity {
        // Simple similarity algorithm - can be enhanced with LSH
        let sig1_words: std::collections::HashSet<&str> = sig1.split('_').collect();
        let sig2_words: std::collections::HashSet<&str> = sig2.split('_').collect();

        let intersection = sig1_words.intersection(&sig2_words).count();
        let union = sig1_words.union(&sig2_words).count();

        let jaccard = if union > 0 {
            intersection as f64 / union as f64
        } else {
            0.0
        };

        PatternSimilarity::new(jaccard, jaccard)
    }

    /// Increment a statistic counter
    fn increment_stat(&self, key: &str) {
        let mut counter = self.stats.entry(key.to_string()).or_insert(0);
        *counter += 1;
    }

    /// Get a statistic value
    fn get_stat(&self, key: &str) -> u64 {
        self.stats.get(key).map(|v| *v).unwrap_or(0)
    }

    /// Clear all caches
    pub fn clear(&self) {
        self.patterns.clear();
        self.pattern_index.clear();
        {
            let mut schemas = self.schemas.lock().unwrap();
            schemas.clear();
        }
        self.stats.clear();
    }
}

/// Pattern match result
#[derive(Debug, Clone)]
pub struct PatternMatch {
    /// Pattern ID
    pub pattern_id: String,

    /// Confidence score
    pub confidence: f64,

    /// Similarity score
    pub similarity_score: f64,

    /// Estimated token usage
    pub estimated_tokens: u32,

    /// Number of times used
    pub usage_count: u32,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStatistics {
    /// Number of cached patterns
    pub patterns_count: u64,

    /// Number of cached schemas
    pub schemas_count: u64,

    /// Total pattern lookups
    pub pattern_lookups: u64,

    /// Total pattern hits
    pub pattern_hits: u64,

    /// Total schema lookups
    pub schema_lookups: u64,

    /// Cache hit rate percentage
    pub cache_hit_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_and_get_pattern() {
        let cache = IntelligentCache::new(100, 50);
        let pattern = SuccessPattern::new(
            "test_sig".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );

        let pattern_id = cache.store_pattern(pattern);
        let retrieved = cache.get_pattern(&pattern_id);

        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().id, pattern_id);
    }

    #[test]
    fn test_find_similar_patterns() {
        let cache = IntelligentCache::new(100, 50);

        let pattern = SuccessPattern::new(
            "github_workflow_nodejs_ci".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );

        cache.store_pattern(pattern);

        let matches = cache.find_similar_patterns("github_workflow_nodejs_ci", 0.0);
        assert!(!matches.is_empty());
    }

    #[test]
    fn test_delete_pattern() {
        let cache = IntelligentCache::new(100, 50);
        let pattern = SuccessPattern::new(
            "test".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );

        let pattern_id = cache.store_pattern(pattern);
        assert!(cache.get_pattern(&pattern_id).is_some());

        assert!(cache.delete_pattern(&pattern_id));
        assert!(cache.get_pattern(&pattern_id).is_none());
    }

    #[test]
    fn test_store_and_get_schema() {
        let cache = IntelligentCache::new(100, 50);
        cache.store_schema(
            "github_workflow".to_string(),
            "schema content".to_string(),
            "1.0".to_string(),
        );

        let schema = cache.get_schema("github_workflow");
        assert!(schema.is_some());
        assert_eq!(schema.unwrap().schema_type, "github_workflow");
    }

    #[test]
    fn test_cache_statistics() {
        let cache = IntelligentCache::new(100, 50);

        let stats = cache.get_stats();
        assert_eq!(stats.patterns_count, 0);
        assert_eq!(stats.schemas_count, 0);
    }
}
