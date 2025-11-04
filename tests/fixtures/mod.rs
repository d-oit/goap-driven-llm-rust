//! Test fixtures and utilities
//!
//! Shared test helpers, mock data, and fixtures used across test modules.

use goap_llm::prelude::*;

/// Helper to create a test world state
pub fn create_test_world_state() -> WorldState {
    WorldState::new(
        5000,
        "test request: Create a Node.js GitHub Actions workflow".to_string(),
    )
}

/// Helper to create test actions
pub fn create_test_actions() -> Vec<Action> {
    vec![
        Action::new(ActionType::DetectSchemaType)
            .with_effect(WorldProperty::SchemaAvailable("github-workflow".to_string()))
            .with_cost(50),
        Action::new(ActionType::CheckPatternCache)
            .with_effect(WorldProperty::PatternAvailable("workflow-template".to_string()))
            .with_cost(30),
        Action::new(ActionType::GenerateFromPattern)
            .with_precondition(WorldProperty::PatternAvailable("workflow-template".to_string()))
            .with_effect(WorldProperty::ResponseGenerated)
            .with_cost(200),
    ]
}

/// Helper to create test goals
pub fn create_test_goals() -> GoalState {
    GoalState::primary_goal()
}

/// Token count helper
pub fn assert_token_budget_under(world_state: &WorldState, max_tokens: u32) {
    assert!(world_state.tokens_remaining() <= max_tokens,
        "Token budget exceeded: {} > {}",
        world_state.tokens_remaining(),
        max_tokens);
}
