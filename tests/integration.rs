//! Integration tests
//!
//! These tests verify end-to-end functionality of the GOAP system

#[cfg(test)]
mod integration {
    use goap_llm::*;

    // Re-export integration test modules
    mod test_full_request_flow;
    mod test_learning_pipeline;
    mod test_pattern_reuse;
    mod test_planner_executor_flow;
    mod test_reactive_replanning;
    mod test_token_budget;

    // Helper functions for creating test data
    #[allow(dead_code)]
    pub fn create_test_request() -> String {
        "Create a test workflow".to_string()
    }

    #[allow(dead_code)]
    pub fn create_test_system() -> GOAPSystem {
        GOAPSystem::new()
    }

    #[allow(dead_code)]
    pub fn create_custom_system(pattern_cache_size: usize, schema_cache_size: usize) -> GOAPSystem {
        let config = GOAPConfig {
            pattern_cache_size,
            schema_cache_size,
            ..Default::default()
        };
        GOAPSystem::with_config(config)
    }
}
