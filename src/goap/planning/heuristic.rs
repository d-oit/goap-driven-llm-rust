//! Planning heuristics
//!
//! Heuristics estimate the cost to reach a goal from a given state.

use crate::goap::goals::GoalState;
use crate::goap::world::{WorldProperty, WorldState};
use std::collections::HashMap;

/// Heuristic function for A* search
#[derive(Debug, Clone)]
pub struct Heuristic {
    /// Cached heuristic values
    cache: HashMap<String, u32>,

    /// Weights for different property types
    property_weights: HashMap<WorldProperty, u32>,
}

impl Heuristic {
    /// Create a new heuristic calculator
    pub fn new() -> Self {
        let mut property_weights = HashMap::new();

        // Set weights for different properties
        property_weights.insert(WorldProperty::RequestValidated, 1);
        property_weights.insert(WorldProperty::SchemaAvailable, 2);
        property_weights.insert(WorldProperty::PatternAvailable("".to_string()), 1);
        property_weights.insert(WorldProperty::ResponseGenerated, 3);
        property_weights.insert(WorldProperty::ResponseValidated, 2);

        Heuristic {
            cache: HashMap::new(),
            property_weights,
        }
    }

    /// Calculate heuristic estimate for reaching a goal
    pub fn estimate(&mut self, current_state: &WorldState, goal_state: &GoalState) -> u32 {
        // Check cache first
        let cache_key = self.generate_cache_key(current_state, goal_state);
        if let Some(&cached_value) = self.cache.get(&cache_key) {
            return cached_value;
        }

        // Calculate heuristic
        let heuristic = self.calculate_heuristic(current_state, goal_state);

        // Cache the result
        self.cache.insert(cache_key, heuristic);

        heuristic
    }

    /// Internal heuristic calculation
    fn calculate_heuristic(&self, current_state: &WorldState, goal_state: &GoalState) -> u32 {
        let required_props = goal_state.get_required_properties();

        // Count missing properties and sum their weights
        let mut estimate = 0u32;
        for prop in required_props {
            if !current_state.has_property(prop) {
                let weight = self.property_weights.get(prop).cloned().unwrap_or(1);
                estimate += weight * 100; // Base cost
            }
        }

        // Add penalty for missing critical properties
        let critical_missing = required_props
            .iter()
            .filter(|prop| !current_state.has_property(prop))
            .count();

        estimate += critical_missing as u32 * 50;

        // Add token budget heuristic
        let remaining_tokens = current_state.tokens_remaining();
        let budget_pressure = if remaining_tokens < 100 {
            200 // High penalty for low tokens
        } else if remaining_tokens < 500 {
            100
        } else {
            0
        };

        estimate += budget_pressure;

        estimate
    }

    /// Generate a cache key for the state/goal combination
    fn generate_cache_key(&self, current_state: &WorldState, goal_state: &GoalState) -> String {
        format!(
            "{:?}:{:?}:{}",
            current_state.get_properties(),
            goal_state.get_required_properties(),
            current_state.tokens_remaining()
        )
    }

    /// Clear the heuristic cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache statistics
    pub fn cache_stats(&self) -> (usize, usize) {
        (
            self.cache.len(),
            self.cache.len() * std::mem::size_of::<(String, u32)>(),
        )
    }
}

impl Default for Heuristic {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristic_creation() {
        let heuristic = Heuristic::new();
        assert!(!heuristic.property_weights.is_empty());
    }

    #[test]
    fn test_heuristic_estimation() {
        let mut heuristic = Heuristic::new();
        let state = WorldState::new(1000, "test".to_string());
        let goal = GoalState::default();

        let _estimate = heuristic.estimate(&state, &goal);
    }
}
