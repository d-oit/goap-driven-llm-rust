//! Integration test for token budget management
//!
//! Tests that the system respects configured token limits

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_enforces_token_budget() {
        // Given
        let max_tokens = 5000;
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let system = GOAPSystem::new();
        let result = system.process_request(&world_state, &goals, &actions, max_tokens).await;

        // Then
        assert!(result.is_ok(), "Request should complete within budget");
        let response = result.unwrap();
        assert!(response.tokens_used() <= max_tokens, "Should not exceed token budget");
    }

    #[tokio::test]
    async fn test_token_budget_compliance_rate() {
        // Given
        let max_tokens = 5000;
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();
        let system = GOAPSystem::new();

        // When - process multiple requests
        let mut compliant_count = 0;
        for i in 0..20 {
            let result = system.process_request(&world_state, &goals, &actions, max_tokens).await;
            if result.is_ok() && result.unwrap().tokens_used() <= max_tokens {
                compliant_count += 1;
            }
        }

        // Then
        let compliance_rate = compliant_count as f64 / 20.0;
        assert!(compliance_rate >= 0.95, "Token budget compliance should be >= 95%, got {}", compliance_rate);
    }

    #[tokio::test]
    async fn test_optimizes_token_usage() {
        // Given
        let max_tokens = 5000;
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let system = GOAPSystem::new();
        let result = system.process_request(&world_state, &goals, &actions, max_tokens).await;

        // Then
        assert!(result.is_ok(), "Request should complete");
        let response = result.unwrap();
        let usage_rate = response.tokens_used() as f64 / max_tokens as f64;
        assert!(usage_rate <= 0.9, "Should use <= 90% of budget for safety margin");
    }

    #[tokio::test]
    async fn test_token_usage_patterns() {
        // Given
        let max_tokens = 5000;
        let system = GOAPSystem::new();

        // When
        let metrics = system.get_metrics().await;
        let total_tokens = metrics.total_tokens_processed;
        let budgeted_tokens = metrics.total_budgeted_tokens;

        // Then
        if budgeted_tokens > 0 {
            let efficiency = total_tokens as f64 / budgeted_tokens as f64;
            assert!(efficiency >= 0.85, "Token efficiency should be >= 85%, got {}", efficiency);
        }
    }
}
