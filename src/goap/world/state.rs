//! World state implementation
//!
//! The WorldState tracks all relevant information during planning and execution.

use crate::goap::world::property::WorldProperty;
use std::collections::HashSet;
use std::time::{Duration, Instant};

/// Represents the current state of the world
#[derive(Debug, Clone)]
pub struct WorldState {
    /// Set of true properties in the world
    properties: HashSet<WorldProperty>,

    /// Total token budget for this request
    token_budget: u32,

    /// Tokens used so far
    tokens_used: u32,

    /// Start time of planning/execution
    start_time: Instant,

    /// Current step in the execution
    current_step: u32,

    /// Request being processed
    request: String,
}

impl WorldState {
    /// Create a new world state
    pub fn new(token_budget: u32, request: String) -> Self {
        WorldState {
            properties: HashSet::new(),
            token_budget,
            tokens_used: 0,
            start_time: Instant::now(),
            current_step: 0,
            request,
        }
    }

    /// Check if a property is true
    pub fn has_property(&self, property: &WorldProperty) -> bool {
        self.properties.contains(property)
    }

    /// Set a property to true
    pub fn set_property(&mut self, property: WorldProperty) {
        self.properties.insert(property);
    }

    /// Remove a property
    pub fn remove_property(&self, _property: &WorldProperty) -> bool {
        false // TODO: Implement if needed
    }

    /// Get all current properties
    pub fn get_properties(&self) -> &HashSet<WorldProperty> {
        &self.properties
    }

    /// Check if token budget is available
    pub fn tokens_available(&self) -> bool {
        self.tokens_used < self.token_budget
    }

    /// Get remaining tokens
    pub fn tokens_remaining(&self) -> u32 {
        self.token_budget.saturating_sub(self.tokens_used)
    }

    /// Add tokens used
    pub fn add_tokens_used(&mut self, tokens: u32) {
        self.tokens_used = self.tokens_used.saturating_add(tokens);
    }

    /// Get total token budget
    pub fn get_token_budget(&self) -> u32 {
        self.token_budget
    }

    /// Get tokens used
    pub fn get_tokens_used(&self) -> u32 {
        self.tokens_used
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Get current step
    pub fn get_current_step(&self) -> u32 {
        self.current_step
    }

    /// Increment step
    pub fn increment_step(&mut self) {
        self.current_step += 1;
    }

    /// Get the request
    pub fn get_request(&self) -> &str {
        &self.request
    }

    /// Check if a state satisfies a goal
    pub fn satisfies(&self, properties: &[WorldProperty]) -> bool {
        properties.iter().all(|prop| self.has_property(prop))
    }

    /// Get the difference between this state and another
    pub fn difference(&self, other: &WorldState) -> Vec<WorldProperty> {
        other
            .properties
            .difference(&self.properties)
            .cloned()
            .collect()
    }
}

impl Default for WorldState {
    fn default() -> Self {
        Self::new(10000, String::new())
    }
}
