//! Integration test for planner-executor flow
//!
//! Tests the complete planning to execution workflow

#[cfg(test)]
mod tests {
    use crate::integration::create_test_request;
    use goap_llm::*;

    #[tokio::test]
    async fn test_system_processes_request() {
        // Given
        let mut system = GOAPSystem::new();
        let request = create_test_request();

        // When
        let result = system.process_request(request).await;

        // Then
        assert!(result.is_ok(), "System should process request successfully");
        let response = result.unwrap();
        assert!(response.contains("Processed request"), "Response should confirm processing");
    }

    #[tokio::test]
    async fn test_system_validates_request() {
        // Given
        let system = GOAPSystem::new();
        let request = create_test_request();

        // When
        let result = system.validate_request(request).await;

        // Then
        assert!(result.is_ok(), "System should validate request successfully");
        let validation = result.unwrap();
        assert!(validation.valid, "Request should be valid");
    }

    #[tokio::test]
    async fn test_system_pattern_operations() {
        // Given
        let system = GOAPSystem::new();
        let _request = "Create a GitHub workflow".to_string();

        // When
        let result = system.list_patterns().await;

        // Then
        assert!(result.is_ok(), "Should list patterns successfully");
        let _patterns = result.unwrap();
        // Patterns list can be empty initially, that's fine
    }
}
