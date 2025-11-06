//! Integration test for planner-executor flow
//!
//! Tests the complete planning to execution workflow

#[cfg(test)]
mod tests {
    use super::*;
    use super::create_test_world_state;
    use super::create_test_actions;
    use super::create_test_goals;
    use goap_llm::prelude::*;

    #[tokio::test]
    async fn test_planner_creates_valid_plan() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let planner = Planner::new();
        let plan = planner.create_plan(&world_state, &goals, &actions).await;

        // Then
        assert!(plan.is_ok(), "Planner should create a valid plan");
        let plan = plan.unwrap();
        assert!(!plan.steps().is_empty(), "Plan should have at least one step");
    }

    #[tokio::test]
    async fn test_executor_executes_plan() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();

        // When
        let executor = Executor::new();
        let result = executor.execute(&world_state, &actions).await;

        // Then
        assert!(result.is_ok(), "Executor should successfully execute the plan");
    }

    #[tokio::test]
    async fn test_planner_executor_integration() {
        // Given
        let world_state = create_test_world_state();
        let actions = create_test_actions();
        let goals = create_test_goals();

        // When
        let planner = Planner::new();
        let plan = planner.create_plan(&world_state, &goals, &actions).await;
        assert!(plan.is_ok(), "Planning should succeed");

        let executor = Executor::new();
        let result = executor.execute(&world_state, &actions).await;

        // Then
        assert!(result.is_ok(), "Full planner-executor flow should succeed");
    }
}
