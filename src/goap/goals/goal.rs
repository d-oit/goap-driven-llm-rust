//! Goal definitions and state management
//!
//! Goals define what the GOAP system wants to achieve.

use crate::goap::world::property::WorldProperty;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Types of goals that can be achieved
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Goal {
    /// Generate a valid response
    GenerateValidResponse,

    /// Optimize token usage
    OptimizeTokenUsage,

    /// Maximize confidence in the response
    MaximizeConfidence,

    /// Ensure schema is available
    EnsureSchemaAvailable,

    /// Reuse successful patterns
    ReuseSuccessfulPattern,

    /// Minimize token cost
    MinimizeTokenCost,

    /// Validate output
    ValidateOutput,

    /// Learn from success
    LearnFromSuccess,

    /// Best effort completion
    BestEffort,
}

impl Goal {
    /// Get a human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Goal::GenerateValidResponse => "Generate a valid response",
            Goal::OptimizeTokenUsage => "Optimize token usage",
            Goal::MaximizeConfidence => "Maximize confidence",
            Goal::EnsureSchemaAvailable => "Ensure schema is available",
            Goal::ReuseSuccessfulPattern => "Reuse successful patterns",
            Goal::MinimizeTokenCost => "Minimize token cost",
            Goal::ValidateOutput => "Validate output",
            Goal::LearnFromSuccess => "Learn from success",
            Goal::BestEffort => "Best effort completion",
        }
    }
}

/// Goal state with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GoalState {
    /// List of goals to achieve
    pub goals: Vec<Goal>,

    /// Required world properties to satisfy goals
    pub required_properties: Vec<WorldProperty>,

    /// Priority level (1=low, 10=critical)
    pub priority_level: u8,

    /// Maximum time to achieve goals in milliseconds
    pub timeout_ms: u64,

    /// Time when goal was created
    pub created_at: Option<u64>,
}

impl GoalState {
    /// Create a new goal state
    pub fn new(
        goals: Vec<Goal>,
        required_properties: Vec<WorldProperty>,
        priority_level: u8,
        timeout_ms: u64,
    ) -> Self {
        GoalState {
            goals,
            required_properties,
            priority_level,
            timeout_ms,
            created_at: Some(std::time::Instant::now().elapsed().as_millis() as u64),
        }
    }

    /// Create a goal state focused on efficiency
    pub fn efficiency_focused() -> Self {
        GoalState::new(
            vec![
                Goal::GenerateValidResponse,
                Goal::MinimizeTokenCost,
                Goal::OptimizeTokenUsage,
            ],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
            ],
            5,
            30000,
        )
    }

    /// Create a goal state for pattern reuse
    pub fn pattern_reuse_goal() -> Self {
        GoalState::new(
            vec![Goal::ReuseSuccessfulPattern, Goal::MinimizeTokenCost],
            vec![
                WorldProperty::PatternAvailable("found".to_string()),
                WorldProperty::ResponseValidated,
            ],
            8,
            15000,
        )
    }

    /// Create a goal state for maximum quality
    pub fn quality_focused() -> Self {
        GoalState::new(
            vec![
                Goal::GenerateValidResponse,
                Goal::MaximizeConfidence,
                Goal::ValidateOutput,
            ],
            vec![
                WorldProperty::ResponseGenerated,
                WorldProperty::ResponseValidated,
            ],
            10,
            45000,
        )
    }

    /// Check if goals have been satisfied
    pub fn is_satisfied(&self, world_state: &crate::goap::world::WorldState) -> bool {
        self.required_properties
            .iter()
            .all(|prop| world_state.has_property(prop))
    }

    /// Check if timeout has been exceeded
    pub fn is_timeout(&self) -> bool {
        self.created_at
            .map(|t| std::time::Instant::now().elapsed().as_millis() > t as u128)
            .unwrap_or(false)
    }

    /// Get required properties
    pub fn get_required_properties(&self) -> &[WorldProperty] {
        &self.required_properties
    }

    /// Get priority level
    pub fn get_priority(&self) -> u8 {
        self.priority_level
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(self.created_at.unwrap_or(0))
    }
}

impl Default for GoalState {
    fn default() -> Self {
        Self::efficiency_focused()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_creation() {
        let goal = Goal::GenerateValidResponse;
        assert_eq!(goal.description(), "Generate a valid response");
    }

    #[test]
    fn test_goal_state_creation() {
        let goal_state = GoalState::new(
            vec![Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );
        assert_eq!(goal_state.goals.len(), 1);
        assert_eq!(goal_state.priority_level, 5);
    }

    #[test]
    fn test_goal_state_default() {
        let goal_state = GoalState::default();
        assert!(goal_state.goals.contains(&Goal::GenerateValidResponse));
        assert!(goal_state.goals.contains(&Goal::MinimizeTokenCost));
    }

    #[test]
    fn test_efficiency_focused() {
        let goal_state = GoalState::efficiency_focused();
        assert!(goal_state.goals.contains(&Goal::MinimizeTokenCost));
    }
}
