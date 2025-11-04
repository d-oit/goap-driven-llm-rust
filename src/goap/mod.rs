//! GOAP - Goal-Oriented Action Planning system
//!
//! This module provides the core GOAP functionality for strategic LLM reasoning.
//! It includes world state management, action planning, and execution.

pub mod actions;
pub mod cache;
pub mod goals;
pub mod metrics;
pub mod planning;
pub mod world;

// Re-export commonly used types
pub use actions::{Action, ActionType, PlanExecutor};
pub use cache::{IntelligentCache, SuccessPattern};
pub use goals::{Goal, GoalState};
pub use metrics::GOAPMetrics;
pub use planning::GOAPPlanner;
pub use world::WorldState;

// Main system type
mod system;

pub use system::{GOAPConfig, GOAPSystem};
