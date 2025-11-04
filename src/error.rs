//! Error types for the GOAP system
//!
//! Custom error types using thiserror for domain-specific errors
//! and anyhow for contextful error handling.

use thiserror::Error;

/// Main error type for the GOAP system
#[derive(Error, Debug)]
pub enum Error {
    /// Planning-related errors
    #[error("Planning error: {0}")]
    Planning(#[from] PlanningError),

    /// Execution-related errors
    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),

    /// Cache-related errors
    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    /// Validation errors
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Serialization/deserialization errors
    #[error("Serialization error")]
    Serialization,

    /// General error with message
    #[error("{0}")]
    General(String),
}

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// Planning-specific errors
#[derive(Error, Debug)]
pub enum PlanningError {
    #[error("No valid path found to goal")]
    NoPathFound,

    #[error("Token budget exceeded")]
    TokenBudgetExceeded,

    #[error("Planning timeout after {0}ms")]
    Timeout(u64),

    #[error("Invalid goal state: {0}")]
    InvalidGoal(String),

    #[error("Maximum plan depth exceeded: {0}")]
    MaxDepthExceeded(u32),

    #[error("No actions available")]
    NoActionsAvailable,

    #[error("Heuristic calculation failed: {0}")]
    HeuristicFailed(String),
    #[error("{0}")]
    General(String),
}

/// Execution-specific errors
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Execution timeout")]
    Timeout,

    #[error("Action failed: {0}")]
    ActionFailed(String),

    #[error("Invalid action sequence")]
    InvalidSequence,

    #[error("World state error: {0}")]
    WorldState(String),

    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    #[error("Reactive replanning failed")]
    ReplanFailed,

    #[error("Goal not satisfied: {0:?}")]
    GoalNotSatisfied(Vec<String>),
}

/// Cache-specific errors
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Schema not found: {0}")]
    SchemaNotFound(String),

    #[error("Cache full")]
    CacheFull,

    #[error("Cache eviction error")]
    EvictionError,

    #[error("Pattern validation failed")]
    PatternValidationFailed,

    #[error("Cache corruption detected")]
    CorruptionDetected,

    #[error("Serialization error: {0}")]
    Serialization(String),
}

/// Validation-specific errors
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidationFailed(String),

    #[error("Response validation failed")]
    ResponseValidationFailed,

    #[error("Pattern confidence too low: {0}")]
    LowConfidence(f64),

    #[error("Request too long: {0} bytes")]
    RequestTooLong(usize),
}

/// Convenience functions for creating errors
impl Error {
    /// Create a general error with a message
    pub fn general<T: std::fmt::Display>(msg: T) -> Self {
        Error::General(msg.to_string())
    }

    /// Create a planning error
    pub fn planning<T: std::fmt::Display>(msg: T) -> Self {
        Error::Planning(PlanningError::General(msg.to_string()))
    }

    /// Create an execution error
    pub fn execution<T: std::fmt::Display>(msg: T) -> Self {
        Error::Execution(ExecutionError::ActionFailed(msg.to_string()))
    }

    /// Create a cache error
    pub fn cache<T: std::fmt::Display>(msg: T) -> Self {
        Error::Cache(CacheError::Serialization(msg.to_string()))
    }

    /// Create a validation error
    pub fn validation<T: std::fmt::Display>(msg: T) -> Self {
        Error::Validation(ValidationError::InvalidRequest(msg.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = Error::general("test error");
        assert!(matches!(error, Error::General(_)));
    }

    #[test]
    fn test_planning_error() {
        let error = Error::Planning(PlanningError::NoPathFound);
        assert!(matches!(error, Error::Planning(_)));
    }

    #[test]
    fn test_convenience_methods() {
        let error = Error::general("test");
        assert!(matches!(error, Error::General(_)));

        let error = Error::planning("planning failed");
        assert!(matches!(error, Error::Planning(_)));

        let error = Error::execution("action failed");
        assert!(matches!(error, Error::Execution(_)));

        let error = Error::cache("cache error");
        assert!(matches!(error, Error::Cache(_)));

        let error = Error::validation("invalid");
        assert!(matches!(error, Error::Validation(_)));
    }
}
