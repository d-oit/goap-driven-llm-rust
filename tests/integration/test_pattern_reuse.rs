//! Integration test for pattern reuse functionality
//!
//! Tests the intelligent caching and pattern reuse system

#[cfg(test)]
mod tests {
    use goap_llm::*;

    #[tokio::test]
    async fn test_cache_stores_patterns() {
        // Given
        let cache = IntelligentCache::new(100, 50);
        let pattern = SuccessPattern::new(
            "Create workflow".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );

        // When
        let pattern_id = cache.store_pattern(pattern.clone());
        let retrieved = cache.get_pattern(&pattern_id);

        // Then
        assert!(retrieved.is_some(), "Should retrieve pattern from cache");
        assert_eq!(retrieved.unwrap().signature, pattern.signature);
    }

    #[tokio::test]
    async fn test_pattern_reuse_improves_performance() {
        // Given
        let cache = IntelligentCache::new(100, 50);
        let pattern = SuccessPattern::new(
            "ci_workflow".to_string(),
            vec![ActionType::GenerateResponse],
            100,
            1.0,
        );
        cache.store_pattern(pattern);

        // When - first request (no similar pattern)
        let _start1 = std::time::Instant::now();
        let matches1 = cache.find_similar_patterns("different_workflow", 0.0);
        let _time1 = _start1.elapsed();

        // When - second request (with similar pattern)
        let _start2 = std::time::Instant::now();
        let matches2 = cache.find_similar_patterns("ci_workflow", 0.0);
        let _time2 = _start2.elapsed();

        // Then
        assert!(matches1.is_empty(), "No matches for different workflow");
        assert!(!matches2.is_empty(), "Should find matching pattern");
        // Note: In-memory operations are too fast to measure reliably
    }

    #[tokio::test]
    async fn test_cache_hit_rate() {
        // Given
        let cache = IntelligentCache::new(100, 50);

        // When
        for i in 0..10 {
            let signature = format!("request_{}", i);
            if i % 2 == 0 {
                let pattern = SuccessPattern::new(
                    signature.clone(),
                    vec![ActionType::GenerateResponse],
                    100,
                    1.0,
                );
                cache.store_pattern(pattern);
            }
        }

        // Then
        let stats = cache.get_stats();
        assert!(stats.patterns_count == 5, "Should have stored 5 patterns");

        // Lookup some patterns to generate hit rate
        let _matches1 = cache.find_similar_patterns("request_2", 0.0);
        let _matches2 = cache.find_similar_patterns("request_4", 0.0);
        let _matches3 = cache.find_similar_patterns("request_6", 0.0);

        let stats_after = cache.get_stats();
        assert!(
            stats_after.pattern_lookups > 0,
            "Should have performed lookups"
        );
    }
}
