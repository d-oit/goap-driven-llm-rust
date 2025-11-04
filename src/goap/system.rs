//! Main GOAP system
//!
//! Orchestrates all components: planning, execution, caching, and metrics.

use crate::error::{CacheError, Error, ExecutionError, PlanningError, Result};
use crate::goap::actions::PlanExecutor;
use crate::goap::cache::IntelligentCache;
use crate::goap::cache::pattern::SuccessPattern;
use crate::goap::goals::GoalState;
use crate::goap::metrics::GOAPMetrics;
use crate::goap::metrics::goap_metrics::MetricsSnapshot;
use crate::goap::planning::{ActionGraph, GOAPPlanner};
use crate::goap::world::{WorldProperty, WorldState};
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tracing::info;

/// Configuration for the GOAP system
#[derive(Debug, Clone)]
pub struct GOAPConfig {
    /// Pattern cache size
    pub pattern_cache_size: usize,

    /// Schema cache size
    pub schema_cache_size: usize,

    /// Pattern confidence threshold
    pub pattern_confidence_threshold: u8,

    /// Maximum plan depth
    pub max_plan_depth: u32,

    /// Maximum replans
    pub max_replans: u32,

    /// Action timeout in milliseconds
    pub action_timeout_ms: u64,

    /// Default token budget
    pub default_token_budget: u32,

    /// Minimum token threshold
    pub min_token_threshold: u32,

    /// Learning rate
    pub learning_rate: f64,

    /// Pattern decay rate
    pub pattern_decay_rate: f64,
}

impl Default for GOAPConfig {
    fn default() -> Self {
        GOAPConfig {
            pattern_cache_size: 10_000,
            schema_cache_size: 1_000,
            pattern_confidence_threshold: 70,
            max_plan_depth: 20,
            max_replans: 3,
            action_timeout_ms: 5000,
            default_token_budget: 10_000,
            min_token_threshold: 100,
            learning_rate: 0.1,
            pattern_decay_rate: 0.05,
        }
    }
}

/// Main GOAP system orchestrator
#[derive(Debug)]
pub struct GOAPSystem {
    /// Configuration
    config: GOAPConfig,

    /// Planner component
    planner: GOAPPlanner,

    /// Executor component
    executor: PlanExecutor,

    /// Pattern and schema cache
    cache: IntelligentCache,

    /// Metrics collector
    metrics: GOAPMetrics,

    /// Creation time
    created_at: Instant,
}

impl GOAPSystem {
    /// Create a new GOAP system with default configuration
    pub fn new() -> Self {
        let config = GOAPConfig::default();
        Self::with_config(config)
    }

    /// Create a GOAP system with custom configuration
    pub fn with_config(config: GOAPConfig) -> Self {
        let action_graph = ActionGraph::build_default();

        GOAPSystem {
            planner: GOAPPlanner::new(action_graph).max_depth(config.max_plan_depth),
            executor: PlanExecutor::with_max_time(config.action_timeout_ms),
            cache: IntelligentCache::new(config.pattern_cache_size, config.schema_cache_size),
            metrics: GOAPMetrics::new(),
            config,
            created_at: Instant::now(),
        }
    }

    /// Create a GOAP system with builder pattern
    pub fn builder() -> GOAPSystemBuilder {
        GOAPSystemBuilder::new()
    }

    /// Process a request through the GOAP planner (TODO: Implement full logic)
    pub async fn process_request(&mut self, request: String) -> Result<String> {
        info!("Processing request: {}", request);

        self.metrics.record_request();

        let start_time = Instant::now();

        // Create initial world state
        let mut world_state = WorldState::new(self.config.default_token_budget, request.clone());

        // Create goal state
        let goal_state = GoalState::efficiency_focused();

        // Set initial property
        world_state.set_property(WorldProperty::RequestValidated);

        // Plan the actions
        let planning_start = Instant::now();
        let plan = self
            .planner
            .plan(world_state.clone(), goal_state.clone())
            .map_err(|e| Error::Planning(PlanningError::General(e.to_string())))?;
        let planning_duration = planning_start.elapsed();

        // Execute the plan
        let execution_result = self
            .executor
            .execute_plan(plan, &mut world_state)
            .await
            .map_err(|e| Error::Execution(ExecutionError::ActionFailed(e.to_string())))?;

        // Record metrics
        self.metrics.record_success(
            planning_duration.as_millis() as u64,
            execution_result.total_duration_ms,
            execution_result.total_tokens_used as u64,
            0, // TODO: Calculate token savings
        );

        // Generate response (placeholder)
        let response = format!(
            "Processed request: {}\nSteps: {}\nTokens: {}\nDuration: {}ms",
            request,
            execution_result.steps.len(),
            execution_result.total_tokens_used,
            execution_result.total_duration_ms
        );

        info!(
            "Request processed successfully in {}ms",
            start_time.elapsed().as_millis()
        );

        Ok(response)
    }

    /// Validate a request without full execution
    pub async fn validate_request(&self, request: String) -> Result<ValidationResponse> {
        info!("Validating request: {}", request);

        let estimated_tokens = self.estimate_token_usage(&request);

        // Check for similar patterns
        let pattern_match = self.check_for_patterns(&request);

        Ok(ValidationResponse {
            valid: true,
            estimated_tokens,
            pattern_match,
        })
    }

    /// List all cached patterns
    pub async fn list_patterns(&self) -> Result<Vec<SuccessPattern>> {
        Ok(self.cache.get_all_patterns())
    }

    /// Get a specific pattern
    pub async fn get_pattern(&self, id: &str) -> Result<SuccessPattern> {
        self.cache
            .get_pattern(id)
            .ok_or_else(|| Error::Cache(CacheError::PatternNotFound(id.to_string())))
    }

    /// Delete a pattern
    pub fn delete_pattern(&self, id: &str) -> Result<()> {
        if self.cache.delete_pattern(id) {
            Ok(())
        } else {
            Err(Error::Cache(CacheError::PatternNotFound(id.to_string())))
        }
    }

    /// Get system uptime
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }

    pub fn metrics(&self) -> MetricsSnapshot {
        self.metrics.get_snapshot()
    }

    /// Configure with pattern confidence threshold
    pub fn with_pattern_threshold(mut self, threshold: u8) -> Self {
        self.config.pattern_confidence_threshold = threshold;
        self
    }

    /// Configure with token budget
    pub fn with_token_budget(mut self, budget: u32) -> Self {
        self.config.default_token_budget = budget;
        self
    }

    /// Enable reactive replanning
    pub fn enable_replanning(self, _enabled: bool) -> Self {
        // Would configure reactive planner here
        self
    }

    /// Set maximum replans
    pub fn max_replans(mut self, max_replans: u32) -> Self {
        self.config.max_replans = max_replans;
        self
    }

    /// Estimate token usage for a request
    fn estimate_token_usage(&self, request: &str) -> u32 {
        // Simple estimation based on request length
        (request.len() as f64 * 1.5) as u32
    }

    /// Check for existing patterns
    fn check_for_patterns(&self, request: &str) -> Option<PatternMatchSummary> {
        // Create a simple signature from the request
        let signature = self.create_signature(request);

        let matches = self.cache.find_similar_patterns(&signature, 0.0);

        if !matches.is_empty() {
            let best_match = &matches[0];
            Some(PatternMatchSummary {
                confidence: best_match.confidence,
                estimated_tokens: best_match.estimated_tokens,
                usage_count: best_match.usage_count,
            })
        } else {
            None
        }
    }

    /// Create a simple signature from a request
    fn create_signature(&self, request: &str) -> String {
        // Simple signature based on keywords
        let keywords = ["github", "workflow", "docker", "compose", "kubernetes"];
        let found: Vec<&str> = keywords
            .iter()
            .filter(|&&keyword| request.to_lowercase().contains(keyword))
            .copied()
            .collect();

        if !found.is_empty() {
            found.join("_")
        } else {
            "generic_request".to_string()
        }
    }
}

impl Default for GOAPSystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from request validation
#[derive(Debug, Clone)]
pub struct ValidationResponse {
    /// Whether the request is valid
    pub valid: bool,

    /// Estimated token usage
    pub estimated_tokens: u32,

    /// Pattern match information
    pub pattern_match: Option<PatternMatchSummary>,
}

/// Summary of pattern match
#[derive(Debug, Clone)]
pub struct PatternMatchSummary {
    /// Confidence score
    pub confidence: f64,

    /// Estimated tokens
    pub estimated_tokens: u32,

    /// Usage count
    pub usage_count: u32,
}

/// Builder for GOAPSystem
#[derive(Debug)]
pub struct GOAPSystemBuilder {
    config: GOAPConfig,
}

impl GOAPSystemBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        GOAPSystemBuilder {
            config: GOAPConfig::default(),
        }
    }

    /// Set pattern cache size
    pub fn pattern_cache_size(mut self, size: usize) -> Self {
        self.config.pattern_cache_size = size;
        self
    }

    /// Set pattern confidence threshold
    pub fn pattern_confidence_threshold(mut self, threshold: u8) -> Self {
        self.config.pattern_confidence_threshold = threshold;
        self
    }

    /// Set maximum plan depth
    pub fn max_plan_depth(mut self, depth: u32) -> Self {
        self.config.max_plan_depth = depth;
        self
    }

    /// Build the GOAPSystem
    pub fn build(self) -> GOAPSystem {
        GOAPSystem::with_config(self.config)
    }
}

impl Default for GOAPSystemBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_creation() {
        let system = GOAPSystem::new();
        let _elapsed = system.created_at.elapsed();
    }

    #[test]
    fn test_system_with_config() {
        let config = GOAPConfig::default();
        let system = GOAPSystem::with_config(config);
        assert_eq!(system.config.default_token_budget, 10_000);
    }

    #[test]
    fn test_builder() {
        let system = GOAPSystem::builder()
            .pattern_cache_size(5000)
            .pattern_confidence_threshold(80)
            .build();

        assert_eq!(system.config.pattern_cache_size, 5000);
        assert_eq!(system.config.pattern_confidence_threshold, 80);
    }

    #[tokio::test]
    async fn test_process_request() {
        let mut system = GOAPSystem::new();
        let result = system.process_request("Test request".to_string()).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_request() {
        let system = GOAPSystem::new();
        let result = system.validate_request("Test request".to_string()).await;
        assert!(result.is_ok());
        assert!(result.unwrap().valid);
    }
}
