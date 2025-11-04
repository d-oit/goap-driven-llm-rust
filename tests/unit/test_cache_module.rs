//! Unit tests for the cache module
//!
//! This module tests the caching mechanisms for patterns, goals, and
//! learning data to improve performance and enable reuse.

use goap_llm::cache::Cache;
use goap_llm::error::{Error, Result};
use goap_llm::goap::GoapPattern;
use crate::fixtures::TestFixtures;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_cache_creation() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap();
        assert!(cache.is_ok());
    }

    #[test]
    fn test_cache_pattern_storage() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let pattern = GoapPattern {
            id: "test_pattern".to_string(),
            name: "Test Pattern".to_string(),
            // Additional pattern fields would be here
            preconditions: serde_json::json!({}),
            effects: serde_json::json!({}),
            actions: vec![],
            metadata: serde_json::json!({}),
        };

        let result = cache.store_pattern(&pattern);
        assert!(result.is_ok());
    }

    #[test]
    fn test_pattern_retrieval() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let pattern = GoapPattern {
            id: "test_pattern_2".to_string(),
            name: "Test Pattern 2".to_string(),
            preconditions: serde_json::json!({}),
            effects: serde_json::json!({}),
            actions: vec![],
            metadata: serde_json::json!({}),
        };

        // Store and retrieve
        cache.store_pattern(&pattern).unwrap();
        let retrieved = cache.get_pattern("test_pattern_2");
        assert!(retrieved.is_ok());
        assert_eq!(retrieved.unwrap().id, "test_pattern_2");
    }

    #[test]
    fn test_pattern_not_found() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let result = cache.get_pattern("non_existent_pattern");
        assert!(result.is_err());
    }

    #[test]
    fn test_cache_goal_storage() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let goal = serde_json::json!({
            "id": "test_goal",
            "description": "Test goal description",
            "priority": 1,
            "requirements": {}
        });

        let result = cache.store_goal("test_goal", &goal);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cache_goal_retrieval() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let goal = serde_json::json!({
            "id": "test_goal_2",
            "description": "Test goal 2",
            "priority": 2,
            "requirements": {}
        });

        cache.store_goal("test_goal_2", &goal).unwrap();
        let retrieved = cache.get_goal("test_goal_2");
        assert!(retrieved.is_ok());
    }

    #[test]
    fn test_cache_invalidation() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let pattern = GoapPattern {
            id: "pattern_to_delete".to_string(),
            name: "Pattern to Delete".to_string(),
            preconditions: serde_json::json!({}),
            effects: serde_json::json!({}),
            actions: vec![],
            metadata: serde_json::json!({}),
        };

        cache.store_pattern(&pattern).unwrap();
        assert!(cache.get_pattern("pattern_to_delete").is_ok());

        let result = cache.invalidate_pattern("pattern_to_delete");
        assert!(result.is_ok());
        assert!(cache.get_pattern("pattern_to_delete").is_err());
    }

    #[test]
    fn test_cache_list_patterns() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        // Store multiple patterns
        for i in 0..5 {
            let pattern = GoapPattern {
                id: format!("pattern_{}", i),
                name: format!("Pattern {}", i),
                preconditions: serde_json::json!({}),
                effects: serde_json::json!({}),
                actions: vec![],
                metadata: serde_json::json!({}),
            };
            cache.store_pattern(&pattern).unwrap();
        }

        let patterns = cache.list_patterns();
        assert!(patterns.is_ok());
        assert_eq!(patterns.unwrap().len(), 5);
    }

    #[test]
    fn test_cache_with_ttl() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        // Store with TTL
        let pattern = GoapPattern {
            id: "ttl_pattern".to_string(),
            name: "TTL Pattern".to_string(),
            preconditions: serde_json::json!({}),
            effects: serde_json::json!({}),
            actions: vec![],
            metadata: serde_json::json!({}),
        };

        cache
            .store_pattern_with_ttl(&pattern, Duration::from_millis(100))
            .unwrap();

        // Should be available immediately
        assert!(cache.get_pattern("ttl_pattern").is_ok());

        // Wait for expiration
        std::thread::sleep(Duration::from_millis(150));

        // Should no longer be available
        assert!(cache.get_pattern("ttl_pattern").is_err());
    }

    #[test]
    fn test_cache_hit_miss_stats() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        let pattern = GoapPattern {
            id: "stats_pattern".to_string(),
            name: "Stats Pattern".to_string(),
            preconditions: serde_json::json!({}),
            effects: serde_json::json!({}),
            actions: vec![],
            metadata: serde_json::json!({}),
        };

        cache.store_pattern(&pattern).unwrap();

        // Multiple hits
        cache.get_pattern("stats_pattern").unwrap();
        cache.get_pattern("stats_pattern").unwrap();

        // Miss
        cache.get_pattern("non_existent");

        let stats = cache.get_stats();
        assert!(stats.is_ok());
        let stats = stats.unwrap();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
    }

    #[test]
    fn test_cache_memory_usage() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        // Store several items
        for i in 0..10 {
            let pattern = GoapPattern {
                id: format!("memory_pattern_{}", i),
                name: format!("Memory Pattern {}", i),
                preconditions: serde_json::json!({}),
                effects: serde_json::json!({}),
                actions: vec![],
                metadata: serde_json::json!({}),
            };
            cache.store_pattern(&pattern).unwrap();
        }

        let usage = cache.get_memory_usage();
        assert!(usage.is_ok());
        assert!(usage.unwrap() > 0);
    }

    #[test]
    fn test_cache_clear() {
        let fixtures = TestFixtures::new();
        let cache = Cache::new(fixtures.mock_config()).unwrap().unwrap();

        // Store some items
        for i in 0..3 {
            let pattern = GoapPattern {
                id: format!("clear_pattern_{}", i),
                name: format!("Clear Pattern {}", i),
                preconditions: serde_json::json!({}),
                effects: serde_json::json!({}),
                actions: vec![],
                metadata: serde_json::json!({}),
            };
            cache.store_pattern(&pattern).unwrap();
        }

        assert_eq!(cache.list_patterns().unwrap().len(), 3);

        let result = cache.clear_all();
        assert!(result.is_ok());
        assert_eq!(cache.list_patterns().unwrap().len(), 0);
    }
}
