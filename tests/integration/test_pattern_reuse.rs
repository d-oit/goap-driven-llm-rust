//! Integration test for pattern reuse functionality
//!
//! Tests the intelligent caching and pattern reuse system

#[cfg(test)]
mod tests {
    use super::*;
    use super::create_test_world_state;
    use super::create_test_actions;
    use super::create_test_goals;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_cache_stores_patterns() {
        // Given
        let cache = IntelligentCache::new();
        let pattern = Pattern {
            id: uuid::Uuid::new_v4(),
            goal: "Create workflow".to_string(),
            actions: vec!["detect".to_string(), "generate".to_string()],
            confidence: 0.9,
        };

        // When
        let stored = cache.store_pattern(pattern.clone()).await;
        let retrieved = cache.get_pattern(&pattern.goal).await;

        // Then
        assert!(stored.is_ok(), "Should store pattern successfully");
        assert!(retrieved.is_some(), "Should retrieve pattern from cache");
    }

    #[tokio::test]
    async fn test_pattern_reuse_improves_performance() {
        // Given
        let cache = IntelligentCache::new();
        let request = "Create a CI workflow".to_string();

        // When - first request (no cache)
        let start1 = std::time::Instant::now();
        let result1 = cache.process_with_pattern(&request, |req, pattern| {
            async move {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                Ok("response 1".to_string())
            }
        }).await;
        let time1 = start1.elapsed();

        // When - second request (with cache)
        let start2 = std::time::Instant::now();
        let result2 = cache.process_with_pattern(&request, |req, pattern| {
            async move {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                Ok("response 2".to_string())
            }
        }).await;
        let time2 = start2.elapsed();

        // Then
        assert!(result1.is_ok(), "First request should succeed");
        assert!(result2.is_ok(), "Second request should succeed");
        assert!(time2 < time1, "Cached request should be faster");
    }

    #[tokio::test]
    async fn test_cache_hit_rate() {
        // Given
        let cache = IntelligentCache::new();

        // When
        for i in 0..10 {
            let request = format!("request {}", i);
            if i % 2 == 0 {
                cache.store_pattern(Pattern {
                    id: uuid::Uuid::new_v4(),
                    goal: request.clone(),
                    actions: vec!["action1".to_string()],
                    confidence: 0.8,
                }).await;
            }
        }

        // Then
        let metrics = cache.get_metrics().await;
        assert!(metrics.hit_rate > 0.5, "Cache hit rate should be > 50%");
    }
}
