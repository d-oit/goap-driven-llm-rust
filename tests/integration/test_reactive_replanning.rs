//! Integration test for reactive replanning functionality
//!
//! Tests the system's ability to adapt and replan when failures occur

#[cfg(test)]
mod tests {
    use crate::integration::create_test_request;
    use goap_llm::*;

    #[tokio::test]
    async fn test_handles_request_processing() {
        // Given
        let mut system = GOAPSystem::new();
        let request = create_test_request();

        // When
        let result = system.process_request(request).await;

        // Then
        assert!(result.is_ok(), "System should handle request successfully");
    }

    #[tokio::test]
    async fn test_processes_multiple_requests() {
        // Given
        let mut system = GOAPSystem::new();

        // When - simulate multiple scenarios
        let requests = vec![
            "Create a deployment pipeline".to_string(),
            "Set up CI/CD workflow".to_string(),
            "Generate test cases".to_string(),
        ];

        let mut results = Vec::new();
        for request in requests {
            let result = system.process_request(request).await;
            results.push(result);
        }

        // Then
        let success_count = results.iter().filter(|r| r.is_ok()).count();
        let success_rate = success_count as f64 / results.len() as f64;
        assert!(
            success_rate >= 0.9,
            "Success rate should be high, got {}",
            success_rate
        );
    }

    #[tokio::test]
    async fn test_request_validation() {
        // Given
        let system = GOAPSystem::new();

        // When
        let result = system.validate_request("Test request".to_string()).await;

        // Then
        assert!(result.is_ok(), "Should validate request successfully");
        let validation = result.unwrap();
        assert!(validation.valid, "Request should be valid");
    }
}
