//! Integration tests
//!
//! These tests verify end-to-end functionality of the GOAP system

#[cfg(test)]
mod integration {
    use super::*;
    use goap_llm::prelude::*;

    // Re-export integration test modules
    mod test_full_request_flow;
    mod test_learning_pipeline;
    mod test_pattern_reuse;
    mod test_planner_executor_flow;
    mod test_reactive_replanning;
    mod test_token_budget;

    // Helper functions for creating test data
    pub fn create_test_world_state() -> WorldState {
        WorldState::new()
    }

    pub fn create_test_actions() -> Vec<Action> {
        // Create some basic test actions
        vec![
            Action::new(ActionType::GenerateResponse),
            Action::new(ActionType::CheckPatternCache),
        ]
    }

    pub fn create_test_goals() -> Vec<Goal> {
        // Create some basic test goals - keep them simple for now
        // Since the actual Goal constructor isn't critical for tests
        vec![]
    }
}
