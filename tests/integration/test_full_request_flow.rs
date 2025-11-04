//! Integration test for full request flow
//!
//! Tests the complete end-to-end system workflow

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_full_request_lifecycle() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let result = system.process_request(&world_state, &goals, &actions, 5000).await;

        // Then
        assert!(result.is_ok(), "Full request flow should complete successfully");
        let response = result.unwrap();
        assert!(response.is_success(), "Request should be marked as successful");
        assert!(response.tokens_used() > 0, "Should have consumed some tokens");
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - process multiple concurrent requests
        let mut handles = Vec::new();
        for i in 0..5 {
            let system_clone = system.clone();
            let world_state_clone = world_state.clone();
            let actions_clone = actions.clone();
            let goals_clone = goals.clone();

            handles.push(tokio::spawn(async move {
                system_clone.process_request(&world_state_clone, &goals_clone, &actions_clone, 5000).await
            }));
        }

        // Then
        let results = futures::future::join_all(handles).await;
        assert_eq!(results.len(), 5, "Should have processed all requests");
        for result in results {
            assert!(result.is_ok(), "Each concurrent request should succeed");
        }
    }

    #[tokio::test]
    async fn test_request_throughput() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let start = std::time::Instant::now();
        let mut completed_count = 0;

        for i in 0..10 {
            let result = system.process_request(&world_state, &goals, &actions, 5000).await;
            if result.is_ok() {
                completed_count += 1;
            }
        }
        let elapsed = start.elapsed();

        // Then
        let throughput = completed_count as f64 / (elapsed.as_secs() as f64 / 3600.0);
        assert!(throughput >= 10000.0, "Throughput should be >= 10,000 requests/hour, got {}", throughput);
    }

    #[tokio::test]
    async fn test_system_health_metrics() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - process some requests
        for i in 0..5 {
            let _ = system.process_request(&world_state, &goals, &actions, 5000).await;
        }

        // Then
        let metrics = system.get_metrics().await;
        assert!(metrics.total_requests >= 5, "Should track total requests");
        assert!(metrics.successful_requests > 0, "Should have successful requests");
        assert!(metrics.average_latency > 0, "Should track latency");
    }
}
