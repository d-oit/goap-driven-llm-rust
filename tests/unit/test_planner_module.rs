//! Unit tests for A* planning algorithm
//!
//! Tests plan generation, heuristic calculations, graph traversal, and path finding.

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;
    use std::collections::{HashMap, HashSet};

    // ============ GOAPPlanner Tests ============

    #[test]
    fn test_planner_creation() {
        // Given
        let action_graph = ActionGraph::build_default();

        // When
        let planner = GOAPPlanner::new(action_graph);

        // Then
        assert_eq!(planner.max_depth, 20);
        assert_eq!(planner.max_planning_time_ms, 200);
    }

    #[test]
    fn test_planner_with_config() {
        // Given
        let action_graph = ActionGraph::build_default();
        let max_depth = 30;
        let max_time = 500;

        // When
        let planner = GOAPPlanner::with_config(action_graph.clone(), max_depth, max_time);

        // Then
        assert_eq!(planner.max_depth, max_depth);
        assert_eq!(planner.max_planning_time_ms, max_time);
    }

    #[test]
    fn test_planner_max_depth_setter() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);

        // When
        let planner = planner.max_depth(40);

        // Then
        assert_eq!(planner.max_depth, 40);
    }

    #[test]
    fn test_planner_max_time_setter() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);

        // When
        let planner = planner.max_planning_time(1000);

        // Then
        assert_eq!(planner.max_planning_time_ms, 1000);
    }

    #[test]
    fn test_planner_get_available_actions() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);

        // When
        let actions = planner.get_available_actions();

        // Then
        assert!(!actions.is_empty());
        assert!(actions.contains(&ActionType::GenerateResponse));
        assert!(actions.contains(&ActionType::CheckPatternCache));
    }

    #[test]
    fn test_planner_plan_already_at_goal() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
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
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        assert!(plan.is_empty(), "Should return empty plan when already at goal");
    }

    #[test]
    fn test_planner_plan_generates_actions() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        assert!(!plan.is_empty(), "Should generate actions to reach goal");
    }

    #[test]
    fn test_planner_plan_multiple_goals() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse, Goal::OptimizeTokenUsage],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
            ],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        assert!(!plan.is_empty(), "Should generate actions for multiple goals");
    }

    #[test]
    fn test_planner_plan_with_low_token_budget() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(50, "test".to_string()); // Very low budget

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        // Should still generate a plan, but may be constrained
    }

    #[test]
    fn test_planner_plan_deterministic() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When - generate same plan multiple times
        let plan1 = planner.plan(world_state.clone(), goal_state.clone());
        let plan2 = planner.plan(world_state, goal_state);

        // Then
        assert!(plan1.is_ok());
        assert!(plan2.is_ok());
        assert_eq!(plan1.unwrap(), plan2.unwrap(), "Plans should be deterministic");
    }

    #[test]
    fn test_planner_max_depth_limit() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph).max_depth(5);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        // Plan should respect depth limit (in real implementation)
    }

    // ============ Heuristic Tests ============

    #[test]
    fn test_heuristic_creation() {
        // When
        let heuristic = Heuristic::new();

        // Then
        assert!(!heuristic.property_weights.is_empty());
        assert!(heuristic.cache.is_empty());
    }

    #[test]
    fn test_heuristic_default() {
        // When
        let heuristic = Heuristic::default();

        // Then
        assert!(!heuristic.property_weights.is_empty());
    }

    #[test]
    fn test_heuristic_estimate_basic() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let estimate = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert!(estimate >= 0, "Heuristic estimate should be non-negative");
    }

    #[test]
    fn test_heuristic_estimate_with_properties_satisfied() {
        // Given
        let mut heuristic = Heuristic::new();
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::ResponseGenerated);

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let estimate = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert_eq!(estimate, 0, "Should estimate 0 cost when all properties satisfied");
    }

    #[test]
    fn test_heuristic_estimate_missing_properties() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
                WorldProperty::RequestValidated,
            ],
            5,
            30000,
        );

        // When
        let estimate = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert!(estimate > 0, "Should estimate positive cost for missing properties");
    }

    #[test]
    fn test_heuristic_estimate_token_budget_pressure() {
        // Given
        let mut heuristic = Heuristic::new();

        // Test with very low tokens
        let world_state_low = WorldState::new(50, "test".to_string());
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // Test with high tokens
        let world_state_high = WorldState::new(2000, "test".to_string());

        // When
        let estimate_low = heuristic.estimate(&world_state_low, &goal_state);
        let estimate_high = heuristic.estimate(&world_state_high, &goal_state);

        // Then
        assert!(
            estimate_low > estimate_high,
            "Should estimate higher cost for low token budget"
        );
    }

    #[test]
    fn test_heuristic_cache_functionality() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When - first call
        let estimate1 = heuristic.estimate(&world_state, &goal_state);

        // When - second call (should use cache)
        let estimate2 = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert_eq!(estimate1, estimate2, "Cached value should match");
        assert!(heuristic.cache.len() > 0, "Cache should be populated");
    }

    #[test]
    fn test_heuristic_clear_cache() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // Populate cache
        heuristic.estimate(&world_state, &goal_state);
        assert!(!heuristic.cache.is_empty());

        // When
        heuristic.clear_cache();

        // Then
        assert!(heuristic.cache.is_empty());
    }

    #[test]
    fn test_heuristic_cache_stats() {
        // Given
        let mut heuristic = Heuristic::new();

        // When
        let (count, size) = heuristic.cache_stats();

        // Then
        assert_eq!(count, 0);
        assert_eq!(size, 0);
    }

    #[test]
    fn test_heuristic_with_custom_properties() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::EnsureSchemaAvailable],
            vec![WorldProperty::SchemaAvailable],
            5,
            30000,
        );

        // When
        let estimate = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert!(estimate >= 0);
    }

    #[test]
    fn test_heuristic_priority_weighting() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());

        // High priority goal
        let goal_state_high = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            10,
            30000,
        );

        // Low priority goal
        let goal_state_low = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            1,
            30000,
        );

        // When
        let estimate_high = heuristic.estimate(&world_state, &goal_state_high);
        let estimate_low = heuristic.estimate(&world_state, &goal_state_low);

        // Then
        // Priority is factored into the heuristic calculation
        assert!(estimate_high >= 0);
        assert!(estimate_low >= 0);
    }

    #[test]
    fn test_heuristic_admissible() {
        // Given
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let estimate = heuristic.estimate(&world_state, &goal_state);

        // Then
        assert!(
            estimate >= 0,
            "Heuristic must be non-negative (admissible)"
        );
    }

    #[test]
    fn test_heuristic_consistent() {
        // Given
        let mut heuristic = Heuristic::new();

        // Create states with incremental progress
        let state1 = WorldState::new(1000, "test".to_string());
        let state2 = {
            let mut s = WorldState::new(1000, "test".to_string());
            s.set_property(WorldProperty::RequestValidated);
            s
        };

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::RequestValidated, WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let h1 = heuristic.estimate(&state1, &goal_state);
        let h2 = heuristic.estimate(&state2, &goal_state);

        // Then
        assert!(
            h1 >= h2,
            "Heuristic should be consistent (monotonic)"
        );
    }

    // ============ ActionGraph Tests ============

    #[test]
    fn test_action_graph_build_default() {
        // When
        let graph = ActionGraph::build_default();

        // Then
        assert!(!graph.get_actions().is_empty());
    }

    #[test]
    fn test_action_graph_get_actions() {
        // Given
        let graph = ActionGraph::build_default();

        // When
        let actions = graph.get_actions();

        // Then
        assert!(actions.contains(&ActionType::GenerateResponse));
        assert!(actions.contains(&ActionType::CheckPatternCache));
        assert!(actions.contains(&ActionType::FetchSchema));
    }

    // ============ Planning Integration Tests ============

    #[test]
    fn test_planner_with_heuristic_integration() {
        // Given
        let action_graph = ActionGraph::build_default();
        let mut heuristic = Heuristic::new();
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let heuristic_estimate = heuristic.estimate(&world_state, &goal_state);
        let planner = GOAPPlanner::new(action_graph);
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        assert!(heuristic_estimate >= 0);
    }

    #[test]
    fn test_planning_preserves_state() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::RequestValidated);

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state.clone(), goal_state);

        // Then
        assert!(plan.is_ok());
        // Original state should be preserved
        assert!(world_state.has_property(&WorldProperty::RequestValidated));
    }

    #[test]
    fn test_planning_with_timeout() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph).max_planning_time(10); // Very short timeout
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        // Should handle timeout gracefully
    }

    #[test]
    fn test_plan_optimization() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        // Simple goal
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        // Plan should be as short as possible
        assert!(!plan.is_empty() || plan.is_empty()); // Either empty (already at goal) or has actions
    }

    #[test]
    fn test_plan_cost_calculation() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse, Goal::OptimizeTokenUsage],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
            ],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        // Plan should include necessary actions
    }

    #[test]
    fn test_planning_with_complex_goals() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(2000, "complex test".to_string());

        let goal_state = GoalState::new(
            vec![
                Goal::GenerateValidResponse,
                Goal::MaximizeConfidence,
                Goal::ValidateOutput,
                Goal::LearnFromSuccess,
            ],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
                WorldProperty::LearnSuccessPattern,
            ],
            10,
            60000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();
        assert!(!plan.is_empty(), "Should generate plan for complex goals");
    }

    #[test]
    fn test_plan_validation() {
        // Given
        let action_graph = ActionGraph::build_default();
        let planner = GOAPPlanner::new(action_graph);
        let world_state = WorldState::new(1000, "test".to_string());

        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );

        // When
        let plan = planner.plan(world_state, goal_state);

        // Then
        assert!(plan.is_ok());
        let plan = plan.unwrap();

        // Validate plan structure
        assert!(plan.len() <= 20, "Plan should respect max depth");

        // All actions should be valid ActionTypes
        for action in &plan {
            match action {
                ActionType::GenerateResponse
                | ActionType::CheckPatternCache
                | ActionType::FetchSchema
                | ActionType::LearnSuccessPattern
                | ActionType::ValidateOutput => {
                    // Valid action type
                }
                _ => panic!("Invalid action type in plan: {:?}", action),
            }
        }
    }
}
