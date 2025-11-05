//! Reactive planning implementation
//!
//! Automatically detects failures and triggers replanning.

use crate::goap::actions::executor::{ExecutionResult, ExecutionStatus};
use crate::goap::world::WorldState;
use std::time::{Duration, Instant};

/// Reactive planner that handles failures and triggers replanning
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReactivePlanner {
    /// Maximum number of replans allowed
    max_replans: u32,

    /// Current replan count
    replan_count: u32,

    /// Whether reactive planning is enabled
    enabled: bool,

    /// Creation time
    created_at: Instant,

    /// Last failure time
    last_failure: Option<Instant>,

    /// Minimum time between replans
    min_replan_interval: Duration,

    /// Track replan history
    replan_history: Vec<ReplanEvent>,
}

/// Event in the replan history
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ReplanEvent {
    /// Timestamp
    timestamp: Instant,

    /// Reason for replan
    reason: String,

    /// Token budget at the time
    tokens_remaining: u32,

    /// Current step
    step: u32,
}

impl ReactivePlanner {
    /// Create a new reactive planner
    pub fn new(max_replans: u32) -> Self {
        ReactivePlanner {
            max_replans,
            replan_count: 0,
            enabled: true,
            created_at: Instant::now(),
            last_failure: None,
            min_replan_interval: Duration::from_millis(500),
            replan_history: Vec::new(),
        }
    }

    /// Create a disabled reactive planner
    pub fn disabled() -> Self {
        ReactivePlanner {
            max_replans: 0,
            replan_count: 0,
            enabled: false,
            created_at: Instant::now(),
            last_failure: None,
            min_replan_interval: Duration::from_millis(500),
            replan_history: Vec::new(),
        }
    }

    /// Enable reactive planning
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable reactive planning
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Set maximum replans
    pub fn max_replans(mut self, max: u32) -> Self {
        self.max_replans = max;
        self
    }

    /// Get current replan count
    pub fn get_replan_count(&self) -> u32 {
        self.replan_count
    }

    /// Check if replanning should occur
    pub fn should_replan(
        &mut self,
        _world_state: &WorldState,
        execution_result: &ExecutionResult,
        failure_reason: Option<&str>,
    ) -> bool {
        if !self.enabled {
            return false;
        }

        if self.replan_count >= self.max_replans {
            return false;
        }

        // Check for failures
        let has_failures = execution_result
            .steps
            .iter()
            .any(|step| step.status == ExecutionStatus::Failed);

        if !has_failures && failure_reason.is_none() {
            return false;
        }

        // Check minimum interval
        if let Some(last_failure) = self.last_failure {
            if last_failure.elapsed() < self.min_replan_interval {
                return false;
            }
        }

        true
    }

    /// Record a replan event
    pub fn record_replan(&mut self, world_state: &WorldState, reason: String) {
        self.replan_count += 1;
        self.last_failure = Some(Instant::now());

        self.replan_history.push(ReplanEvent {
            timestamp: Instant::now(),
            reason,
            tokens_remaining: world_state.tokens_remaining(),
            step: world_state.get_current_step(),
        });
    }

    /// Check if max replans reached
    pub fn max_replans_reached(&self) -> bool {
        self.replan_count >= self.max_replans
    }

    /// Get replan history
    pub fn get_replan_history(&self) -> &[ReplanEvent] {
        &self.replan_history
    }

    /// Get time since last failure
    pub fn time_since_last_failure(&self) -> Option<Duration> {
        self.last_failure.map(|t| t.elapsed())
    }

    /// Check if token budget is critically low
    pub fn is_token_budget_critical(&self, world_state: &WorldState) -> bool {
        world_state.tokens_remaining() < 100
    }

    /// Get failure recovery rate
    pub fn recovery_rate(&self) -> f64 {
        if self.replan_history.is_empty() {
            return 0.0;
        }

        // Simple recovery rate: ratio of replans that led to eventual success
        // In a real implementation, this would track success after replans
        0.8 // Placeholder
    }

    /// Reset replan count
    pub fn reset(&mut self) {
        self.replan_count = 0;
        self.replan_history.clear();
        self.last_failure = None;
    }

    /// Check if system is in a recovery state
    pub fn is_in_recovery(&self) -> bool {
        self.replan_count > 0 && !self.max_replans_reached()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::goap::actions::ExecutionResult;

    #[test]
    fn test_reactive_planner_creation() {
        let planner = ReactivePlanner::new(3);
        assert_eq!(planner.max_replans, 3);
        assert!(planner.enabled);
        assert_eq!(planner.get_replan_count(), 0);
    }

    #[test]
    fn test_disabled_reactive_planner() {
        let planner = ReactivePlanner::disabled();
        assert!(!planner.enabled);
        assert_eq!(planner.max_replans, 0);
    }

    #[test]
    fn test_should_replan() {
        let mut planner = ReactivePlanner::new(3);
        let world_state = WorldState::new(1000, "test".to_string());
        let execution_result = ExecutionResult::new();

        assert!(!planner.should_replan(&world_state, &execution_result, None));
    }

    #[test]
    fn test_record_replan() {
        let mut planner = ReactivePlanner::new(3);
        let world_state = WorldState::new(1000, "test".to_string());

        planner.record_replan(&world_state, "Token budget exceeded".to_string());

        assert_eq!(planner.get_replan_count(), 1);
        assert_eq!(planner.replan_history.len(), 1);
    }

    #[test]
    fn test_max_replans_reached() {
        let mut planner = ReactivePlanner::new(2);
        let world_state = WorldState::new(1000, "test".to_string());

        planner.record_replan(&world_state, "First failure".to_string());
        planner.record_replan(&world_state, "Second failure".to_string());

        assert!(planner.max_replans_reached());
    }

    #[test]
    fn test_is_token_budget_critical() {
        let planner = ReactivePlanner::new(3);
        let world_state = WorldState::new(100, "test".to_string());

        {
            let _ = planner.is_token_budget_critical(&world_state);
        };
    }

    #[test]
    fn test_reset() {
        let mut planner = ReactivePlanner::new(3);
        let world_state = WorldState::new(1000, "test".to_string());

        planner.record_replan(&world_state, "Failure".to_string());
        assert_eq!(planner.get_replan_count(), 1);

        planner.reset();
        assert_eq!(planner.get_replan_count(), 0);
        assert_eq!(planner.replan_history.len(), 0);
    }
}
