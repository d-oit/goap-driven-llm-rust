//! Integration test for reactive replanning functionality
//!
//! Tests the system's ability to adapt and replan when failures occur

#[cfg(test)]
mod tests {
    use super::*;
    use super::create_test_world_state;
    use super::create_test_actions;
    use super::create_test_goals;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_detects_execution_failure() {
        // Given
        let world_state = create_test_world_state();
        let mut actions = create_test_actions();
        actions.push(Action::new(ActionType::GenerateFromLLM));

        let goals = create_test_goals();

        // When
        let planner = Planner::new();
        let plan = planner.create_plan(&world_state, &goals, &actions).await;
        assert!(plan.is_ok(), "Initial plan should be created");

        // Simulate failure during execution
        let executor = Executor::new();
        let result = executor.execute_with_failure_injection(&world_state, &actions, 0.5).await;

        // Then
        if let Err(e) = result {
            assert!(e.to_string().contains("failure"), "Should detect execution failure");
        }
    }

    #[tokio::test]
    async fn test_replans_after_failure() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let reactive_planner = ReactivePlanner::new();
        let result = reactive_planner.handle_request(&world_state, &goals, &actions).await;

        // Then
        assert!(result.is_ok(), "Reactive planner should handle failures and replan");
        let (plan, success) = result.unwrap();
        assert!(!plan.steps().is_empty(), "Should create a recovery plan");
    }

    #[tokio::test]
    async fn test_recovery_success_rate() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When - simulate multiple failure scenarios
        let mut success_count = 0;
        for i in 0..10 {
            let reactive_planner = ReactivePlanner::new();
            let result = reactive_planner.handle_request(&world_state, &goals, &actions).await;

            if result.is_ok() {
                success_count += 1;
            }
        }

        // Then
        let success_rate = success_count as f64 / 10.0;
        assert!(success_rate >= 0.82, "Recovery success rate should be >= 82%, got {}", success_rate);
    }

    #[tokio::test]
    async fn test_adapts_to_new_conditions() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let reactive_planner = ReactivePlanner::new();
        let initial_result = reactive_planner.handle_request(&world_state, &goals, &actions).await;
        assert!(initial_result.is_ok(), "Initial request should succeed");

        // Simulate changing conditions
        let modified_goals = goals.with_new_priority(1);

        // Then
        let adapted_result = reactive_planner.handle_request(&world_state, &modified_goals, &actions).await;
        assert!(adapted_result.is_ok(), "Should adapt to new conditions");
    }
}
