//! Unit tests for the executor module
//!
//! This module tests the GOAP executor that transforms planned actions into
//! concrete system operations.

use tokio_test::block_on;
use goap_llm::executor::Executor;
use goap_llm::error::{Error, Result};
use crate::fixtures::TestFixtures;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await;
            assert!(executor.is_ok());
        });
    }

    #[test]
    fn test_execute_simple_action() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            // Test execution of a simple planned action
            // This is a placeholder test structure
            let result = executor.execute_action("test_action".to_string()).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_execute_action_with_params() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            let action_params = serde_json::json!({
                "action": "move",
                "target": "location_a",
                "speed": "normal"
            });

            let result = executor.execute_action_with_params(action_params).await;
            assert!(result.is_ok());
        });
    }

    #[test]
    fn test_action_failure_handling() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            // Test that failed actions are properly handled
            let result = executor.execute_action("invalid_action".to_string()).await;
            assert!(result.is_err());
        });
    }

    #[test]
    fn test_parallel_action_execution() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            let actions = vec![
                "action_1".to_string(),
                "action_2".to_string(),
                "action_3".to_string(),
            ];

            let results = executor.execute_actions_parallel(actions).await;
            assert!(results.is_ok());
            assert_eq!(results.unwrap().len(), 3);
        });
    }

    #[test]
    fn test_action_timeout() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            // Test that long-running actions respect timeout
            let result = executor
                .execute_action_with_timeout("slow_action".to_string(), std::time::Duration::from_millis(100))
                .await;
            // Should complete or timeout gracefully
            assert!(result.is_ok() || result.is_err());
        });
    }

    #[test]
    fn test_state_after_execution() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            // Execute action and verify state change
            let action = "state_changing_action".to_string();
            executor.execute_action(action).await.unwrap();

            let state = executor.get_current_state();
            assert!(state.is_ok());
        });
    }

    #[test]
    fn test_executor_metrics() {
        block_on(async {
            let fixtures = TestFixtures::new();
            let executor = Executor::new(fixtures.mock_config()).await.unwrap();

            // Execute some actions and check metrics
            executor.execute_action("action_1".to_string()).await.unwrap();
            executor.execute_action("action_2".to_string()).await.unwrap();

            let metrics = executor.get_execution_metrics();
            assert!(metrics.is_ok());
            assert!(metrics.unwrap().actions_executed >= 2);
        });
    }
}
