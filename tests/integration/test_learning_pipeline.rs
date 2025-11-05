//! Integration test for pattern learning pipeline
//!
//! Tests the system's ability to learn from experience and improve over time

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_learns_from_successful_plans() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - process a successful request
        let result = system.process_request(&world_state, &goals, &actions, 5000).await;
        assert!(result.is_ok(), "Request should succeed");

        // Then
        let metrics = system.get_metrics().await;
        assert!(metrics.learned_patterns > 0, "Should have learned at least one pattern");
    }

    #[tokio::test]
    async fn test_confidence_improvement_over_requests() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - process first request
        let initial_confidence = system.get_pattern_confidence(&goals.primary_goal().to_string()).await;

        // Process multiple similar requests to trigger learning
        for i in 0..5 {
            let result = system.process_request(&world_state, &goals, &actions, 5000).await;
            assert!(result.is_ok(), "Request {} should succeed", i);
        }

        // Then
        let final_confidence = system.get_pattern_confidence(&goals.primary_goal().to_string()).await;
        let improvement = final_confidence - initial_confidence;
        assert!(improvement >= 0.1, "Confidence should improve by at least 10%");
    }

    #[tokio::test]
    async fn test_learning_effectiveness() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let mut confidence_scores = Vec::new();
        for i in 0..10 {
            let result = system.process_request(&world_state, &goals, &actions, 5000).await;
            assert!(result.is_ok(), "Request {} should succeed", i);

            let confidence = system.get_pattern_confidence(&goals.primary_goal().to_string()).await;
            confidence_scores.push(confidence);
        }

        // Then
        let first_half_avg: f64 = confidence_scores[0..5].iter().sum::<f64>() / 5.0;
        let second_half_avg: f64 = confidence_scores[5..10].iter().sum::<f64>() / 5.0;
        let improvement = second_half_avg - first_half_avg;

        assert!(improvement >= 0.1, "Learning effectiveness should show 10-15% improvement");
        assert!(improvement <= 0.2, "Learning improvement should be realistic (< 20%)");
    }

    #[tokio::test]
    async fn test_adapts_to_failure_patterns() {
        // Given
        let system = GOAPSystem::new();
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - process requests that might fail
        let mut success_count = 0;
        for i in 0..20 {
            let result = system.process_request(&world_state, &goals, &actions, 5000).await;
            if result.is_ok() {
                success_count += 1;
            }

            // Simulate feedback about failures
            if i % 5 == 0 {
                system.record_failure(&goals, "simulated failure".to_string()).await;
            }
        }

        // Then
        let success_rate = success_count as f64 / 20.0;
        assert!(success_rate >= 0.8, "Success rate should improve with learning");
    }
}
