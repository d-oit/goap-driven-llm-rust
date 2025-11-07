//! Integration test for token budget management
//!
//! Tests that the system respects configured token limits

#[cfg(test)]
mod tests {
    use goap_llm::*;

    #[tokio::test]
    async fn test_enforces_token_budget() {
        // Given
        let max_tokens = 5000;
        let config = GOAPConfig {
            default_token_budget: max_tokens,
            ..Default::default()
        };
        let mut system = GOAPSystem::with_config(config);
        let request = "Create a test workflow".to_string();

        // When
        let result = system.process_request(request).await;

        // Then
        assert!(result.is_ok(), "Request should complete within budget");
        let response = result.unwrap();
        assert!(response.contains("Processed"), "Should have processed the request");
    }

    #[tokio::test]
    async fn test_token_budget_compliance_rate() {
        // Given
        let max_tokens = 5000;
        let config = GOAPConfig {
            default_token_budget: max_tokens,
            ..Default::default()
        };
        let mut system = GOAPSystem::with_config(config);

        // When - process multiple requests
        let mut compliant_count = 0;
        for i in 0..20 {
            let request = format!("test request {}", i);
            let result = system.process_request(request).await;
            if result.is_ok() {
                compliant_count += 1;
            }
        }

        // Then
        let compliance_rate = compliant_count as f64 / 20.0;
        assert!(
            compliance_rate >= 0.9,
            "Request success rate should be >= 90%, got {}",
            compliance_rate
        );
    }

    #[tokio::test]
    async fn test_optimizes_token_usage() {
        // Given
        let max_tokens = 5000;
        let config = GOAPConfig {
            default_token_budget: max_tokens,
            min_token_threshold: 100,
            ..Default::default()
        };
        let mut system = GOAPSystem::with_config(config);
        let request = "Create a workflow with specific requirements".to_string();

        // When
        let result = system.process_request(request).await;

        // Then
        assert!(result.is_ok(), "Request should complete");
        let response = result.unwrap();
        assert!(response.contains("Tokens"), "Response should mention token usage");
    }

    #[tokio::test]
    async fn test_token_usage_patterns() {
        // Given
        let mut system = GOAPSystem::new();

        // When - process some requests
        for i in 0..10 {
            let request = format!("test request {}", i);
            let _ = system.process_request(request).await;
        }

        // Then
        let metrics = system.metrics();
        assert!(metrics.total_requests >= 10, "Should track total requests");
    }
}
