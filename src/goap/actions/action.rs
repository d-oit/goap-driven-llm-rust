//! Action definitions
//!
//! Actions represent operations that can be performed in the world.
//! Each action has preconditions (what must be true) and effects (what becomes true).

use crate::goap::world::WorldState;
use crate::goap::world::property::WorldProperty;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Types of actions available in the GOAP system
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    /// Detect the schema type for a request
    DetectSchemaType,

    /// Fetch validation schema
    FetchSchema,

    /// Check pattern cache for similar requests
    CheckPatternCache,

    /// Compress the request to save tokens
    CompressRequest,

    /// Pre-validate the request
    PreValidateRequest,

    /// Post-validate the response
    PostValidateResponse,

    /// Quick validation for pattern
    QuickValidatePattern,

    /// Generate full response via LLM
    GenerateResponse,

    /// Generate response from cached pattern
    GenerateFromPattern,

    /// Generate response from template
    GenerateFromTemplate,

    /// Learn and cache successful patterns
    LearnSuccessPattern,

    /// Update system metrics
    UpdateMetrics,

    /// Adapt optimization rules
    AdaptOptimizationRules,

    /// Fix validation errors
    FixValidationErrors,

    /// Request clarification from user
    RequestClarification,

    /// Trigger replanning
    Replan,
}

impl ActionType {
    /// Get a human-readable name for this action
    pub fn name(&self) -> &'static str {
        match self {
            ActionType::DetectSchemaType => "Detect Schema Type",
            ActionType::FetchSchema => "Fetch Schema",
            ActionType::CheckPatternCache => "Check Pattern Cache",
            ActionType::CompressRequest => "Compress Request",
            ActionType::PreValidateRequest => "Pre-Validate Request",
            ActionType::PostValidateResponse => "Post-Validate Response",
            ActionType::QuickValidatePattern => "Quick Validate Pattern",
            ActionType::GenerateResponse => "Generate Response",
            ActionType::GenerateFromPattern => "Generate From Pattern",
            ActionType::GenerateFromTemplate => "Generate From Template",
            ActionType::LearnSuccessPattern => "Learn Success Pattern",
            ActionType::UpdateMetrics => "Update Metrics",
            ActionType::AdaptOptimizationRules => "Adapt Optimization Rules",
            ActionType::FixValidationErrors => "Fix Validation Errors",
            ActionType::RequestClarification => "Request Clarification",
            ActionType::Replan => "Replan",
        }
    }
}

/// Represents a GOAP action with preconditions and effects
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    /// Type of this action
    pub action_type: ActionType,

    /// Properties that must be true before executing
    pub preconditions: HashSet<WorldProperty>,

    /// Properties that become true after executing
    pub effects: HashSet<WorldProperty>,

    /// Estimated token cost
    pub estimated_cost: u32,

    /// Estimated time in milliseconds
    pub estimated_duration_ms: u64,
}

impl Action {
    /// Create a new action
    pub fn new(action_type: ActionType) -> Self {
        Action {
            action_type,
            preconditions: HashSet::new(),
            effects: HashSet::new(),
            estimated_cost: 100,
            estimated_duration_ms: 100,
        }
    }

    /// Add a precondition
    pub fn with_precondition(mut self, property: WorldProperty) -> Self {
        self.preconditions.insert(property);
        self
    }

    /// Add an effect
    pub fn with_effect(mut self, property: WorldProperty) -> Self {
        self.effects.insert(property);
        self
    }

    /// Set estimated cost
    pub fn with_cost(mut self, cost: u32) -> Self {
        self.estimated_cost = cost;
        self
    }

    /// Set estimated duration
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.estimated_duration_ms = duration_ms;
        self
    }

    /// Check if this action can execute given the world state
    pub fn can_execute(&self, world_state: &WorldState) -> bool {
        self.preconditions
            .iter()
            .all(|prop| world_state.has_property(prop))
    }

    /// Get the estimated token cost
    pub fn get_cost(&self) -> u32 {
        self.estimated_cost
    }

    /// Get the estimated duration
    pub fn get_duration(&self) -> u64 {
        self.estimated_duration_ms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_creation() {
        let action = Action::new(ActionType::GenerateResponse);
        assert_eq!(action.action_type, ActionType::GenerateResponse);
        assert!(action.preconditions.is_empty());
        assert!(action.effects.is_empty());
    }

    #[test]
    fn test_action_with_precondition() {
        let action = Action::new(ActionType::GenerateResponse)
            .with_precondition(WorldProperty::RequestValidated);

        assert!(
            action
                .preconditions
                .contains(&WorldProperty::RequestValidated)
        );
    }

    #[test]
    fn test_action_with_effect() {
        let action =
            Action::new(ActionType::GenerateResponse).with_effect(WorldProperty::ResponseGenerated);

        assert!(action.effects.contains(&WorldProperty::ResponseGenerated));
    }

    #[test]
    fn test_can_execute() {
        let mut world_state = WorldState::new(1000, "test request".to_string());
        world_state.set_property(WorldProperty::RequestValidated);

        let action = Action::new(ActionType::GenerateResponse)
            .with_precondition(WorldProperty::RequestValidated);

        assert!(action.can_execute(&world_state));
    }
}
