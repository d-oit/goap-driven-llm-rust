//! GOAP-driven LLM Strategic Reasoning System
//!
//! This library provides a Goal-Oriented Action Planning (GOAP) system that transforms
//! LLM request processing from reactive to proactive through strategic planning,
//! pattern reuse, and adaptive replanning.

pub mod goap;

// Re-export public API
pub use goap::{
    Action, ActionType, GOAPConfig, GOAPMetrics, GOAPPlanner, GOAPSystem, Goal, GoalState,
    IntelligentCache, PlanExecutor, SuccessPattern, WorldState,
};

// Error types
mod error;
pub use error::{Error, Result};

// Re-export commonly used types
pub mod prelude {
    pub use crate::error::{Error, Result};
    pub use crate::goap::{Action, ActionType, GOAPSystem, GoalState, WorldState};
}

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Get library information
pub fn info() -> LibraryInfo {
    LibraryInfo {
        name: env!("CARGO_PKG_NAME"),
        version: env!("CARGO_PKG_VERSION"),
        description: env!("CARGO_PKG_DESCRIPTION"),
    }
}

/// Information about the library
#[derive(Debug, Clone)]
pub struct LibraryInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_info() {
        let info = info();
        assert_eq!(info.name, "goap-llm");
        assert!(!info.version.is_empty());
    }

    #[test]
    fn test_prelude_import() {
        // Test that prelude types can be imported
        let system = GOAPSystem::new();
        let _elapsed = system.elapsed();
    }
}
