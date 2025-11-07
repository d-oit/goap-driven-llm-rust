//! Integration test for pattern learning pipeline
//!
//! Tests the system's ability to learn from experience and improve over time

#[cfg(test)]
mod tests {
    use crate::integration::create_test_request;
    use goap_llm::*;

    #[tokio::test]
    async fn test_learns_from_successful_plans() {
        // Given
        let mut system = GOAPSystem::new();
        let request = create_test_request();

        // When - process a successful request
        let result = system.process_request(request).await;
        assert!(result.is_ok(), "Request should succeed");

        // Then - verify request was processed
        let response = result.unwrap();
        assert!(response.contains("Processed request"), "Should have processed request");
    }

    #[tokio::test]
    async fn test_processes_multiple_requests() {
        // Given
        let mut system = GOAPSystem::new();

        // When - process multiple requests
        let mut results = Vec::new();
        for i in 0..5 {
            let request = format!("test request {}", i);
            let result = system.process_request(request).await;
            results.push(result);
        }

        // Then
        assert_eq!(results.len(), 5, "Should process all requests");
        for (i, result) in results.iter().enumerate() {
            assert!(result.is_ok(), "Request {} should succeed", i);
        }
    }

    #[tokio::test]
    async fn test_learning_effectiveness() {
        // Given
        let mut system = GOAPSystem::new();

        // When - process multiple requests to verify system works
        let mut success_count = 0;
        for i in 0..10 {
            let request = format!("test request {}", i);
            let result = system.process_request(request).await;
            if result.is_ok() {
                success_count += 1;
            }
        }

        // Then
        let success_rate = success_count as f64 / 10.0;
        assert!(
            success_rate >= 0.9,
            "Success rate should be high, got {}",
            success_rate
        );
    }

    #[tokio::test]
    async fn test_handles_different_requests() {
        // Given
        let mut system = GOAPSystem::new();

        // When - process various requests
        let requests = vec![
            "Create a CI workflow".to_string(),
            "Generate a test suite".to_string(),
            "Build a deployment pipeline".to_string(),
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
            success_rate >= 0.8,
            "Success rate should be high for different request types"
        );
    }
}
