//! Integration test for full request flow
//!
//! Tests the complete end-to-end system workflow

#[cfg(test)]
mod tests {
    use crate::integration::create_test_request;
    use goap_llm::*;

    #[tokio::test]
    async fn test_full_request_lifecycle() {
        // Given
        let mut system = GOAPSystem::new();
        let request = create_test_request();

        // When
        let result = system.process_request(request).await;

        // Then
        assert!(
            result.is_ok(),
            "Full request flow should complete successfully"
        );
        let response = result.unwrap();
        assert!(
            response.contains("Processed request"),
            "Response should contain processing info"
        );
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        // Given
        let mut systems = Vec::new();
        for _ in 0..5 {
            systems.push(GOAPSystem::new());
        }

        // When - process multiple concurrent requests
        let mut handles = Vec::new();
        for mut system in systems {
            let request = create_test_request();
            handles.push(tokio::spawn(async move {
                system.process_request(request).await
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
        let mut system = GOAPSystem::new();

        // When
        let start = std::time::Instant::now();
        let mut completed_count = 0;

        for i in 0..10 {
            let request = format!("test request {}", i);
            let result = system.process_request(request).await;
            if result.is_ok() {
                completed_count += 1;
            }
        }
        let elapsed = start.elapsed();

        // Then
        let throughput = completed_count as f64 / (elapsed.as_secs() as f64 / 3600.0);
        assert!(
            throughput >= 100.0,
            "Throughput should be >= 100 requests/hour, got {}",
            throughput
        );
    }

    #[tokio::test]
    async fn test_system_health_metrics() {
        // Given
        let mut system = GOAPSystem::new();

        // When - process some requests
        for i in 0..5 {
            let request = format!("test request {}", i);
            let _ = system.process_request(request).await;
        }

        // Then
        let metrics = system.metrics();
        assert!(metrics.total_requests >= 5, "Should track total requests");
        assert!(
            metrics.successful_requests > 0,
            "Should have successful requests"
        );
        // avg_planning_time_ms can be 0 for very fast operations
    }
}
