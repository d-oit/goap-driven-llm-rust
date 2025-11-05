//! Goal orchestration
//!
//! The GoalOrchestrator manages multiple goals and adjusts priorities dynamically.

use crate::goap::goals::GoalState;
#[allow(unused_imports)]
use crate::goap::world::WorldProperty;
use crate::goap::world::WorldState;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

/// Manages and orchestrates multiple goals
#[derive(Debug, Clone)]
pub struct GoalOrchestrator {
    /// Active goals
    active_goals: HashMap<String, GoalState>,

    /// Completed goals
    completed_goals: HashSet<String>,

    /// Failed goals
    failed_goals: HashSet<String>,

    /// Current focus goal ID
    current_focus: Option<String>,

    /// Creation time
    created_at: Instant,
}

impl GoalOrchestrator {
    /// Create a new goal orchestrator
    pub fn new() -> Self {
        GoalOrchestrator {
            active_goals: HashMap::new(),
            completed_goals: HashSet::new(),
            failed_goals: HashSet::new(),
            current_focus: None,
            created_at: Instant::now(),
        }
    }

    /// Add a goal
    pub fn add_goal(&mut self, id: String, goal: GoalState) -> &mut Self {
        self.active_goals.insert(id.clone(), goal);
        if self.current_focus.is_none() {
            self.current_focus = Some(id.clone());
        }
        self
    }

    /// Remove a goal
    pub fn remove_goal(&mut self, id: &str) -> Option<GoalState> {
        self.completed_goals.remove(id);
        self.failed_goals.remove(id);
        self.active_goals.remove(id)
    }

    /// Mark a goal as completed
    pub fn complete_goal(&mut self, id: &str) -> bool {
        if self.active_goals.remove(id).is_some() {
            self.completed_goals.insert(id.to_string());
            true
        } else {
            false
        }
    }

    /// Mark a goal as failed
    pub fn fail_goal(&mut self, id: &str) -> bool {
        if self.active_goals.remove(id).is_some() {
            self.failed_goals.insert(id.to_string());
            true
        } else {
            false
        }
    }

    /// Get current focus goal
    pub fn get_current_focus(&self) -> Option<&GoalState> {
        self.current_focus
            .as_ref()
            .and_then(|id| self.active_goals.get(id))
    }

    /// Set current focus goal
    pub fn set_focus(&mut self, id: &str) {
        if self.active_goals.contains_key(id) {
            self.current_focus = Some(id.to_string());
        }
    }

    /// Get all active goals
    pub fn get_active_goals(&self) -> &HashMap<String, GoalState> {
        &self.active_goals
    }

    /// Get completed goals
    pub fn get_completed_goals(&self) -> &HashSet<String> {
        &self.completed_goals
    }

    /// Get failed goals
    pub fn get_failed_goals(&self) -> &HashSet<String> {
        &self.failed_goals
    }

    /// Check if all goals are satisfied
    pub fn all_satisfied(&self, world_state: &WorldState) -> bool {
        self.active_goals
            .values()
            .all(|goal| goal.is_satisfied(world_state))
    }

    /// Check if any goal has timed out
    pub fn check_timeouts(&mut self) -> Vec<String> {
        let mut timed_out = Vec::new();

        for (id, goal) in self.active_goals.iter() {
            if goal.is_timeout() {
                timed_out.push(id.clone());
            }
        }

        for id in &timed_out {
            self.fail_goal(id);
        }

        timed_out
    }

    /// Get the highest priority active goal
    pub fn get_highest_priority(&self) -> Option<(&String, &GoalState)> {
        self.active_goals
            .iter()
            .max_by_key(|(_, goal)| goal.get_priority())
    }

    /// Get progress metrics
    pub fn get_progress(&self) -> (usize, usize, usize) {
        (
            self.active_goals.len(),
            self.completed_goals.len(),
            self.failed_goals.len(),
        )
    }

    /// Adjust goal priorities based on context
    pub fn adjust_priorities(&mut self, world_state: &WorldState) {
        // Simple priority adjustment logic
        // Could be enhanced with more sophisticated algorithms

        for goal in self.active_goals.values_mut() {
            // Boost priority if we're close to timeout
            if goal.is_timeout() && goal.priority_level < 10 {
                goal.priority_level = 10;
            }

            // Adjust based on token budget pressure
            if world_state.tokens_remaining() < 100 && goal.priority_level < 9 {
                // Favor efficiency goals under token pressure
                if goal
                    .goals
                    .contains(&crate::goap::goals::Goal::MinimizeTokenCost)
                    || goal
                        .goals
                        .contains(&crate::goap::goals::Goal::OptimizeTokenUsage)
                {
                    goal.priority_level = 9;
                }
            }
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.created_at.elapsed()
    }
}

impl Default for GoalOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goal_orchestrator_creation() {
        let orchestrator = GoalOrchestrator::new();
        assert_eq!(orchestrator.get_active_goals().len(), 0);
        assert_eq!(orchestrator.get_completed_goals().len(), 0);
    }

    #[test]
    fn test_add_goal() {
        let mut orchestrator = GoalOrchestrator::new();
        let goal = GoalState::default();
        orchestrator.add_goal("goal1".to_string(), goal);

        assert_eq!(orchestrator.get_active_goals().len(), 1);
        assert!(orchestrator.get_active_goals().contains_key("goal1"));
    }

    #[test]
    fn test_complete_goal() {
        let mut orchestrator = GoalOrchestrator::new();
        let goal = GoalState::default();
        orchestrator.add_goal("goal1".to_string(), goal);

        assert!(orchestrator.complete_goal("goal1"));
        assert_eq!(orchestrator.get_active_goals().len(), 0);
        assert!(orchestrator.get_completed_goals().contains("goal1"));
    }

    #[test]
    fn test_all_satisfied() {
        let mut orchestrator = GoalOrchestrator::new();
        let mut world_state = WorldState::new(1000, "test".to_string());
        world_state.set_property(WorldProperty::ResponseGenerated);
        world_state.set_property(WorldProperty::ResponseValidated);

        let goal = GoalState::efficiency_focused();
        orchestrator.add_goal("goal1".to_string(), goal);

        assert!(orchestrator.all_satisfied(&world_state));
    }

    #[test]
    fn test_highest_priority() {
        let mut orchestrator = GoalOrchestrator::new();
        let goal1 = GoalState::new(
            vec![crate::goap::goals::Goal::GenerateValidResponse],
            vec![WorldProperty::ResponseGenerated],
            5,
            30000,
        );
        let goal2 = GoalState::new(
            vec![crate::goap::goals::Goal::MinimizeTokenCost],
            vec![WorldProperty::ResponseGenerated],
            8,
            20000,
        );

        orchestrator.add_goal("goal1".to_string(), goal1);
        orchestrator.add_goal("goal2".to_string(), goal2);

        let (id, _) = orchestrator.get_highest_priority().unwrap();
        assert_eq!(id, "goal2");
    }
}
