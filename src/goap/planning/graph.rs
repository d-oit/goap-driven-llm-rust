//! Action graph implementation
//!
//! Represents the graph of available actions and their relationships.

use crate::goap::actions::{Action, ActionType};
use crate::goap::world::property::WorldProperty;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// Represents the action graph for planning
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionGraph {
    /// Map of action types to their definitions
    actions: HashMap<ActionType, Action>,

    /// Adjacency list: which actions can follow which
    transitions: HashMap<ActionType, HashSet<ActionType>>,

    /// Available actions in the graph
    available_actions: Vec<ActionType>,
}

impl ActionGraph {
    /// Create a new empty action graph
    pub fn new() -> Self {
        ActionGraph {
            actions: HashMap::new(),
            transitions: HashMap::new(),
            available_actions: Vec::new(),
        }
    }

    /// Add an action to the graph
    pub fn add_action(&mut self, action: Action) -> &mut Self {
        let action_type = action.action_type.clone();
        self.actions.insert(action_type.clone(), action);
        if !self.available_actions.contains(&action_type) {
            self.available_actions.push(action_type);
        }
        self
    }

    /// Add a transition between actions
    pub fn add_transition(&mut self, from: ActionType, to: ActionType) -> &mut Self {
        self.transitions.entry(from).or_default().insert(to);
        self
    }

    /// Get an action by type
    pub fn get_action(&self, action_type: &ActionType) -> Option<&Action> {
        self.actions.get(action_type)
    }

    /// Get all available actions
    pub fn get_actions(&self) -> &[ActionType] {
        &self.available_actions
    }

    /// Get actions that can follow the given action
    pub fn get_next_actions(&self, action: &ActionType) -> Vec<ActionType> {
        self.transitions
            .get(action)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all actions in the graph
    pub fn get_all_actions(&self) -> HashMap<ActionType, Action> {
        self.actions.clone()
    }

    /// Build a default action graph with standard actions
    pub fn build_default() -> Self {
        let mut graph = ActionGraph::new();

        // Add all standard actions with their preconditions and effects

        // 1. Detect schema type
        graph.add_action(
            Action::new(ActionType::DetectSchemaType)
                .with_precondition(WorldProperty::RequestValidated)
                .with_effect(WorldProperty::SchemaTypeDetected)
                .with_cost(10)
                .with_duration(50),
        );

        // 2. Fetch schema
        graph.add_action(
            Action::new(ActionType::FetchSchema)
                .with_precondition(WorldProperty::SchemaTypeDetected)
                .with_effect(WorldProperty::SchemaAvailable)
                .with_cost(20)
                .with_duration(100),
        );

        // 3. Check pattern cache
        graph.add_action(
            Action::new(ActionType::CheckPatternCache)
                .with_precondition(WorldProperty::RequestValidated)
                .with_effect(WorldProperty::PatternCacheChecked)
                .with_cost(5)
                .with_duration(20),
        );

        // 4. Generate response
        graph.add_action(
            Action::new(ActionType::GenerateResponse)
                .with_precondition(WorldProperty::RequestValidated)
                .with_effect(WorldProperty::ResponseGenerated)
                .with_cost(100)
                .with_duration(500),
        );

        // 5. Generate from pattern (if available)
        graph.add_action(
            Action::new(ActionType::GenerateFromPattern)
                .with_precondition(WorldProperty::PatternAvailable("found".to_string()))
                .with_effect(WorldProperty::ResponseGenerated)
                .with_cost(30)
                .with_duration(150),
        );

        // 6. Post-validate response
        graph.add_action(
            Action::new(ActionType::PostValidateResponse)
                .with_precondition(WorldProperty::ResponseGenerated)
                .with_effect(WorldProperty::ResponseValidated)
                .with_cost(15)
                .with_duration(75),
        );

        // 7. Learn success pattern
        graph.add_action(
            Action::new(ActionType::LearnSuccessPattern)
                .with_precondition(WorldProperty::ResponseValidated)
                .with_effect(WorldProperty::LearnSuccessPattern)
                .with_cost(10)
                .with_duration(50),
        );

        // Define transitions
        graph.add_transition(ActionType::DetectSchemaType, ActionType::FetchSchema);
        graph.add_transition(ActionType::FetchSchema, ActionType::GenerateResponse);
        graph.add_transition(
            ActionType::CheckPatternCache,
            ActionType::GenerateFromPattern,
        );
        graph.add_transition(
            ActionType::GenerateResponse,
            ActionType::PostValidateResponse,
        );
        graph.add_transition(
            ActionType::GenerateFromPattern,
            ActionType::PostValidateResponse,
        );
        graph.add_transition(
            ActionType::PostValidateResponse,
            ActionType::LearnSuccessPattern,
        );

        graph
    }
}

impl Default for ActionGraph {
    fn default() -> Self {
        Self::build_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_graph_creation() {
        let graph = ActionGraph::new();
        assert!(graph.get_all_actions().is_empty());
    }

    #[test]
    fn test_add_action() {
        let mut graph = ActionGraph::new();
        let action = Action::new(ActionType::GenerateResponse);
        graph.add_action(action);

        assert_eq!(graph.get_actions().len(), 1);
        assert!(graph.get_action(&ActionType::GenerateResponse).is_some());
    }

    #[test]
    fn test_add_transition() {
        let mut graph = ActionGraph::new();
        graph.add_transition(
            ActionType::GenerateResponse,
            ActionType::PostValidateResponse,
        );

        let next_actions = graph.get_next_actions(&ActionType::GenerateResponse);
        assert!(next_actions.contains(&ActionType::PostValidateResponse));
    }

    #[test]
    fn test_build_default() {
        let graph = ActionGraph::build_default();
        assert!(!graph.get_actions().is_empty());
        assert!(graph.get_actions().contains(&ActionType::GenerateResponse));
    }
}
