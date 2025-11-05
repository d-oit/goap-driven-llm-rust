//! World property definitions
//!
//! Properties represent the state of the world at any given time.
//! They are used as preconditions and effects for actions.

use serde::{Deserialize, Serialize};

/// Properties that can exist in the world state
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldProperty {
    /// Request has been validated
    RequestValidated,

    /// Response has been generated
    ResponseGenerated,

    /// Response has been validated
    ResponseValidated,

    /// Schema has been fetched and is available
    SchemaAvailable,

    /// Schema type has been detected
    SchemaTypeDetected,

    /// Pattern cache has been checked
    PatternCacheChecked,

    /// Pattern has been found with sufficient confidence
    PatternAvailable(String), // pattern ID

    /// Token compression has been applied
    TokenCompressionApplied(u32), // compression ratio

    /// Request has been pre-validated
    PreValidated,

    /// Request has been post-validated
    PostValidated,

    /// Quick validation for pattern has been completed
    QuickValidatedPattern,

    /// Token optimization has been applied
    OptimizeTokenUsage,

    /// Response has been generated from template
    GenerateFromTemplate,

    /// Success pattern has been learned
    LearnSuccessPattern,

    /// Metrics have been updated
    UpdateMetrics,

    /// Optimization rules have been adapted
    AdaptOptimizationRules,

    /// Validation errors have been fixed
    FixValidationErrors,

    /// Clarification has been requested
    RequestClarification,

    /// Replanning has been triggered
    Replan,

    /// Token budget check passed
    TokenBudgetCheckPassed(u32), // tokens available

    /// LLM call completed successfully
    LLMCallCompleted,

    /// Token budget exceeded
    TokenBudgetExceeded,

    /// Max retries reached
    MaxRetriesReached,
}

impl WorldProperty {
    /// Get a human-readable description of this property
    pub fn description(&self) -> &'static str {
        match self {
            WorldProperty::RequestValidated => "Request has been validated",
            WorldProperty::ResponseGenerated => "Response has been generated",
            WorldProperty::ResponseValidated => "Response has been validated",
            WorldProperty::SchemaAvailable => "Schema is available",
            WorldProperty::SchemaTypeDetected => "Schema type has been detected",
            WorldProperty::PatternCacheChecked => "Pattern cache has been checked",
            WorldProperty::PatternAvailable(_) => "Pattern is available",
            WorldProperty::TokenCompressionApplied(_) => "Token compression has been applied",
            WorldProperty::PreValidated => "Request has been pre-validated",
            WorldProperty::PostValidated => "Request has been post-validated",
            WorldProperty::QuickValidatedPattern => "Quick validation for pattern completed",
            WorldProperty::OptimizeTokenUsage => "Token optimization has been applied",
            WorldProperty::GenerateFromTemplate => "Response generated from template",
            WorldProperty::LearnSuccessPattern => "Success pattern has been learned",
            WorldProperty::UpdateMetrics => "Metrics have been updated",
            WorldProperty::AdaptOptimizationRules => "Optimization rules have been adapted",
            WorldProperty::FixValidationErrors => "Validation errors have been fixed",
            WorldProperty::RequestClarification => "Clarification has been requested",
            WorldProperty::Replan => "Replanning has been triggered",
            WorldProperty::TokenBudgetCheckPassed(_) => "Token budget check passed",
            WorldProperty::LLMCallCompleted => "LLM call completed successfully",
            WorldProperty::TokenBudgetExceeded => "Token budget exceeded",
            WorldProperty::MaxRetriesReached => "Max retries reached",
        }
    }
}
