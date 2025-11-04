//! API contract tests for public interface stability
//!
//! Tests API signatures, error types, and serialization formats

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;
    use serde_json;

    #[test]
    fn test_goapsystem_public_api_signature() {
        // Given
        let system = GOAPSystem::new();

        // When - verify method signatures
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // Then - verify methods exist and have correct signatures
        assert!(system.process_request(&world_state, &goals, &actions, 5000).is_ok());
    }

    #[test]
    fn test_planner_public_api_signature() {
        // Given
        let planner = Planner::new();

        // When
        let world_state = create_test_world_state();
        let goals = create_test_goals();
        let actions = create_test_actions();

        // Then
        assert!(planner.create_plan(&world_state, &goals, &actions).is_ok());
    }

    #[test]
    fn test_cache_public_api_signature() {
        // Given
        let cache = IntelligentCache::new();

        // When
        let pattern = Pattern {
            id: uuid::Uuid::new_v4(),
            goal: "test".to_string(),
            actions: vec!["action1".to_string()],
            confidence: 0.8,
        };

        // Then
        assert!(cache.store_pattern(pattern).is_ok());
    }

    #[test]
    fn test_error_type_contracts() {
        // Given
        let system = GOAPSystem::new();

        // When
        let result = system.simulate_error();

        // Then - verify error type is correct
        match result {
            Err(e) => {
                assert!(e.to_string().len() > 0, "Error should have a message");
            }
            Ok(_) => panic!("Should return an error"),
        }
    }

    #[test]
    fn test_world_state_serialization() {
        // Given
        let world_state = create_test_world_state();

        // When
        let serialized = serde_json::to_string(&world_state);
        let deserialized: WorldState = serde_json::from_str(&serialized.unwrap());

        // Then
        assert_eq!(world_state.tokens_remaining(), deserialized.tokens_remaining());
        assert_eq!(world_state.request_id(), deserialized.request_id());
    }

    #[test]
    fn test_pattern_serialization() {
        // Given
        let pattern = Pattern {
            id: uuid::Uuid::new_v4(),
            goal: "test pattern".to_string(),
            actions: vec!["action1".to_string(), "action2".to_string()],
            confidence: 0.85,
        };

        // When
        let serialized = serde_json::to_string(&pattern);
        let deserialized: Pattern = serde_json::from_str(&serialized.unwrap());

        // Then
        assert_eq!(pattern.goal, deserialized.goal);
        assert_eq!(pattern.actions, deserialized.actions);
        assert_eq!(pattern.confidence, deserialized.confidence);
    }

    #[test]
    fn test_action_serialization() {
        // Given
        let action = Action::new(ActionType::DetectSchemaType)
            .with_cost(50);

        // When
        let serialized = serde_json::to_string(&action);
        let deserialized: Action = serde_json::from_str(&serialized.unwrap());

        // Then
        assert_eq!(action.action_type(), deserialized.action_type());
        assert_eq!(action.cost(), deserialized.cost());
    }

    #[test]
    fn test_metrics_serialization() {
        // Given
        let metrics = GOAPMetrics::new();

        // When
        let serialized = serde_json::to_string(&metrics);
        let deserialized: GOAPMetrics = serde_json::from_str(&serialized.unwrap());

        // Then
        assert_eq!(metrics.total_requests, deserialized.total_requests);
    }

    #[test]
    fn test_api_compatibility_across_versions() {
        // Given
        let world_state = create_test_world_state();

        // When - simulate API usage
        let json = serde_json::to_string(&world_state).unwrap();
        let restored: WorldState = serde_json::from_str(&json).unwrap();

        // Then
        assert!(restored.tokens_remaining() > 0, "Deserialized object should be valid");
    }

    #[test]
    fn test_error_handling_contract() {
        // Given
        let system = GOAPSystem::new();

        // When
        let result = system.simulate_various_failures();

        // Then - verify errors are properly typed
        if let Err(e) = result {
            assert!(e.is::<anyhow::Error>() || e.is::<goap_llm::error::Error>());
        }
    }

    #[test]
    fn test_async_api_contracts() {
        // Given
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let cache = IntelligentCache::new();

            // When
            let pattern = Pattern {
                id: uuid::Uuid::new_v4(),
                goal: "async test".to_string(),
                actions: vec!["action".to_string()],
                confidence: 0.9,
            };

            // Then
            assert!(cache.store_pattern(pattern).await.is_ok());
        });
    }

    #[test]
    fn test_struct_field_contracts() {
        // Given
        let world_state = WorldState::new(5000, "test request".to_string());

        // When - verify required fields exist
        let _request_id = world_state.request_id();
        let _tokens_remaining = world_state.tokens_remaining();
        let _properties = world_state.properties();

        // Then - fields should be accessible
        assert!(!_request_id.is_empty());
        assert!(_tokens_remaining > 0);
    }
}
