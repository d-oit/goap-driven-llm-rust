//! Unit tests for core GOAP module public APIs
//!
//! Tests WorldState, Action, GoalState, and GOAPSystem functionality.

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;
    use std::time::Duration;

    // ============ WorldState Tests ============

    #[test]
    fn test_world_state_creation() {
        // Given
        let token_budget = 5000;
        let request = "test request".to_string();

        // When
        let world_state = WorldState::new(token_budget, request.clone());

        // Then
        assert_eq!(world_state.get_token_budget(), token_budget);
        assert_eq!(world_state.get_tokens_used(), 0);
        assert_eq!(world_state.get_request(), request.as_str());
        assert_eq!(world_state.tokens_remaining(), token_budget);
        assert_eq!(world_state.get_current_step(), 0);
    }

    #[test]
    fn test_world_state_default() {
        // Given
        let world_state = WorldState::default();

        // Then
        assert_eq!(world_state.get_token_budget(), 10000);
        assert_eq!(world_state.get_tokens_used(), 0);
    }

    #[test]
    fn test_world_state_property_management() {
        // Given
        let mut world_state = WorldState::new(1000, "test".to_string());
        let property = WorldProperty::RequestValidated;

        // When - set property
        world_state.set_property(property.clone());

        // Then
        assert!(world_state.has_property(&property));
        assert!(!world_state.get_properties().is_empty());
    }

    #[test]
    fn test_world_state_property_satisfaction() {
        // Given
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::RequestValidated);
        let properties = vec![WorldProperty::RequestValidated];

        // When
        let satisfied = world_state.satisfies(&properties);

        // Then
        assert!(satisfied);
    }

    #[test]
    fn test_world_state_property_not_satisfied() {
        // Given
        let world_state = WorldState::new(1000, "test".to_string());
        let properties = vec![WorldProperty::RequestValidated];

        // When
        let satisfied = world_state.satisfies(&properties);

        // Then
        assert!(!satisfied);
    }

    #[test]
    fn test_world_state_difference() {
        // Given
        let mut world_state1 = WorldState::new(1000, "test".to_string());
        let mut world_state2 = WorldState::new(1000, "test".to_string());

        world_state1.set_property(WorldProperty::RequestValidated);
        world_state2.set_property(WorldProperty::RequestValidated);
        world_state2.set_property(WorldProperty::ResponseGenerated);

        // When
        let diff = world_state1.difference(&world_state2);

        // Then
        assert!(diff.contains(&WorldProperty::ResponseGenerated));
    }

    #[test]
    fn test_world_state_elapsed_time() {
        // Given
        let world_state = WorldState::new(1000, "test".to_string());

        // When
        let elapsed = world_state.elapsed();

        // Then
        assert!(elapsed.as_millis() >= 0);
    }

    #[test]
    fn test_world_state_step_increment() {
        // Given
        let mut world_state = WorldState::new(1000, "test".to_string());

        // When
        world_state.increment_step();

        // Then
        assert_eq!(world_state.get_current_step(), 1);

        // When
        world_state.increment_step();

        // Then
        assert_eq!(world_state.get_current_step(), 2);
    }

    // ============ Action Tests ============

    #[test]
    fn test_action_creation() {
        // Given
        let action_type = ActionType::GenerateResponse;

        // When
        let action = Action::new(action_type.clone());

        // Then
        assert_eq!(action.action_type, action_type);
        assert!(action.preconditions.is_empty());
        assert!(action.effects.is_empty());
        assert_eq!(action.get_cost(), 100);
        assert_eq!(action.get_duration(), 100);
    }

    #[test]
    fn test_action_with_precondition() {
        // Given
        let action = Action::new(ActionType::GenerateResponse);

        // When
        let action = action.with_precondition(WorldProperty::RequestValidated);

        // Then
        assert!(action.preconditions.contains(&WorldProperty::RequestValidated));
    }

    #[test]
    fn test_action_with_effect() {
        // Given
        let action = Action::new(ActionType::GenerateResponse);

        // When
        let action = action.with_effect(WorldProperty::ResponseGenerated);

        // Then
        assert!(action.effects.contains(&WorldProperty::ResponseGenerated));
    }

    #[test]
    fn test_action_with_cost() {
        // Given
        let action = Action::new(ActionType::GenerateResponse);

        // When
        let action = action.with_cost(250);

        // Then
        assert_eq!(action.get_cost(), 250);
    }

    #[test]
    fn test_action_with_duration() {
        // Given
        let action = Action::new(ActionType::GenerateResponse);

        // When
        let action = action.with_duration(500);

        // Then
        assert_eq!(action.get_duration(), 500);
    }

    #[test]
    fn test_action_can_execute_with_satisfied_preconditions() {
        // Given
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::RequestValidated);

        let action = Action::new(ActionType::GenerateResponse)
            .with_precondition(WorldProperty::RequestValidated);

        // When
        let can_execute = action.can_execute(&world_state);

        // Then
        assert!(can_execute);
    }

    #[test]
    fn test_action_cannot_execute_without_preconditions() {
        // Given
        let world_state = WorldState::new(1000, "test".to_string());

        let action = Action::new(ActionType::GenerateResponse)
            .with_precondition(WorldProperty::RequestValidated);

        // When
        let can_execute = action.can_execute(&world_state);

        // Then
        assert!(!can_execute);
    }

    #[test]
    fn test_action_type_name() {
        // Given/When/Then
        assert_eq!(ActionType::GenerateResponse.name(), "Generate Response");
        assert_eq!(ActionType::CheckPatternCache.name(), "Check Pattern Cache");
        assert_eq!(ActionType::LearnSuccessPattern.name(), "Learn Success Pattern");
    }

    // ============ GoalState Tests ============

    #[test]
    fn test_goal_state_creation() {
        // Given
        let goals = vec![Goal::GenerateValidResponse];
        let properties = vec![WorldProperty::ResponseGenerated];
        let priority = 5;
        let timeout = 30000;

        // When
        let goal_state = GoalState::new(goals.clone(), properties.clone(), priority, timeout);

        // Then
        assert_eq!(goal_state.goals, goals);
        assert_eq!(goal_state.priority_level, priority);
        assert_eq!(goal_state.timeout_ms, timeout);
        assert!(goal_state.created_at.is_some());
    }

    #[test]
    fn test_goal_state_efficiency_focused() {
        // When
        let goal_state = GoalState::efficiency_focused();

        // Then
        assert!(goal_state.goals.contains(&Goal::GenerateValidResponse));
        assert!(goal_state.goals.contains(&Goal::MinimizeTokenCost));
        assert!(goal_state.goals.contains(&Goal::OptimizeTokenUsage));
        assert_eq!(goal_state.priority_level, 5);
    }

    #[test]
    fn test_goal_state_pattern_reuse() {
        // When
        let goal_state = GoalState::pattern_reuse_goal();

        // Then
        assert!(goal_state.goals.contains(&Goal::ReuseSuccessfulPattern));
        assert!(goal_state.goals.contains(&Goal::MinimizeTokenCost));
        assert_eq!(goal_state.priority_level, 8);
    }

    #[test]
    fn test_goal_state_quality_focused() {
        // When
        let goal_state = GoalState::quality_focused();

        // Then
        assert!(goal_state.goals.contains(&Goal::GenerateValidResponse));
        assert!(goal_state.goals.contains(&Goal::MaximizeConfidence));
        assert!(goal_state.goals.contains(&Goal::ValidateOutput));
        assert_eq!(goal_state.priority_level, 10);
    }

    #[test]
    fn test_goal_state_is_satisfied() {
        // Given
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::ResponseGenerated);
        world_state.set_property(WorldProperty::ResponseValidated);

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated, WorldProperty::ResponseValidated],
            5,
            30000,
        );

        // When
        let satisfied = goal_state.is_satisfied(&world_state);

        // Then
        assert!(satisfied);
    }

    #[test]
    fn test_goal_state_not_satisfied() {
        // Given
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let satisfied = goal_state.is_satisfied(&world_state);

        // Then
        assert!(!satisfied);
    }

    #[test]
    fn test_goal_state_get_required_properties() {
        // Given
        let properties = vec![WorldProperty::ResponseGenerated, WorldProperty::ResponseValidated];
        let goal_state = GoalState::new(vec![], properties.clone(), 5, 30000);

        // When
        let required = goal_state.get_required_properties();

        // Then
        assert_eq!(required.len(), 2);
        assert!(required.contains(&WorldProperty::ResponseGenerated));
        assert!(required.contains(&WorldProperty::ResponseValidated));
    }

    #[test]
    fn test_goal_state_get_priority() {
        // Given
        let goal_state = GoalState::new(vec![], vec![], 8, 30000);

        // When
        let priority = goal_state.get_priority();

        // Then
        assert_eq!(priority, 8);
    }

    #[test]
    fn test_goal_state_default() {
        // When
        let goal_state = GoalState::default();

        // Then
        assert!(goal_state.goals.contains(&Goal::GenerateValidResponse));
    }

    // ============ GOAPSystem Tests ============

    #[test]
    fn test_system_creation() {
        // When
        let system = GOAPSystem::new();

        // Then
        assert!(system.elapsed().as_millis() >= 0);
    }

    #[test]
    fn test_system_with_custom_config() {
        // Given
        let config = GOAPConfig {
            pattern_cache_size: 5000,
            schema_cache_size: 500,
            pattern_confidence_threshold: 80,
            max_plan_depth: 30,
            max_replans: 5,
            action_timeout_ms: 10000,
            default_token_budget: 15000,
            min_token_threshold: 200,
            learning_rate: 0.2,
            pattern_decay_rate: 0.1,
        };

        // When
        let system = GOAPSystem::with_config(config.clone());

        // Then
        assert_eq!(system.config.default_token_budget, config.default_token_budget);
        assert_eq!(system.config.pattern_confidence_threshold, config.pattern_confidence_threshold);
        assert_eq!(system.config.max_plan_depth, config.max_plan_depth);
    }

    #[test]
    fn test_system_builder() {
        // When
        let system = GOAPSystem::builder()
            .pattern_cache_size(5000)
            .pattern_confidence_threshold(80)
            .max_plan_depth(30)
            .build();

        // Then
        assert_eq!(system.config.pattern_cache_size, 5000);
        assert_eq!(system.config.pattern_confidence_threshold, 80);
        assert_eq!(system.config.max_plan_depth, 30);
    }

    #[test]
    fn test_system_with_pattern_threshold() {
        // Given
        let system = GOAPSystem::new();

        // When
        let system = system.with_pattern_threshold(85);

        // Then
        assert_eq!(system.config.pattern_confidence_threshold, 85);
    }

    #[test]
    fn test_system_with_token_budget() {
        // Given
        let system = GOAPSystem::new();

        // When
        let system = system.with_token_budget(20000);

        // Then
        assert_eq!(system.config.default_token_budget, 20000);
    }

    #[test]
    fn test_system_max_replans() {
        // Given
        let system = GOAPSystem::new();

        // When
        let system = system.max_replans(5);

        // Then
        assert_eq!(system.config.max_replans, 5);
    }

    #[tokio::test]
    async fn test_system_process_request() {
        // Given
        let mut system = GOAPSystem::new();
        let request = "Test request".to_string();

        // When
        let result = system.process_request(request.clone()).await;

        // Then
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(response.contains("Processed request"));
        assert!(response.contains(&request));
    }

    #[tokio::test]
    async fn test_system_validate_request() {
        // Given
        let system = GOAPSystem::new();
        let request = "Test request".to_string();

        // When
        let result = system.validate_request(request.clone()).await;

        // Then
        assert!(result.is_ok());
        let validation = result.unwrap();
        assert!(validation.valid);
        assert!(validation.estimated_tokens > 0);
    }

    #[test]
    fn test_system_uptime() {
        // Given
        let system = GOAPSystem::new();

        // When
        let uptime1 = system.elapsed();

        // Simulate some time passing
        std::thread::sleep(Duration::from_millis(10));

        let uptime2 = system.elapsed();

        // Then
        assert!(uptime2 >= uptime1);
    }

    // ============ WorldProperty Tests ============

    #[test]
    fn test_world_property_description() {
        // Given/When/Then
        assert_eq!(
            WorldProperty::RequestValidated.description(),
            "Request has been validated"
        );
        assert_eq!(
            WorldProperty::ResponseGenerated.description(),
            "Response has been generated"
        );
        assert_eq!(
            WorldProperty::PatternAvailable("test".to_string()).description(),
            "Pattern is available"
        );
    }

    // ============ Goal Tests ============

    #[test]
    fn test_goal_description() {
        // Given/When/Then
        assert_eq!(Goal::GenerateValidResponse.description(), "Generate a valid response");
        assert_eq!(Goal::OptimizeTokenUsage.description(), "Optimize token usage");
        assert_eq!(Goal::MaximizeConfidence.description(), "Maximize confidence");
    }

    // ============ GOAPConfig Tests ============

    #[test]
    fn test_goap_config_default() {
        // When
        let config = GOAPConfig::default();

        // Then
        assert_eq!(config.pattern_cache_size, 10_000);
        assert_eq!(config.schema_cache_size, 1_000);
        assert_eq!(config.pattern_confidence_threshold, 70);
        assert_eq!(config.max_plan_depth, 20);
        assert_eq!(config.max_replans, 3);
        assert_eq!(config.action_timeout_ms, 5000);
        assert_eq!(config.default_token_budget, 10_000);
        assert_eq!(config.min_token_threshold, 100);
        assert_eq!(config.learning_rate, 0.1);
        assert_eq!(config.pattern_decay_rate, 0.05);
    }
}
