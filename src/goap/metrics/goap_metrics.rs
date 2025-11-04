//! GOAP system metrics
//!
//! Collects and tracks performance and usage statistics.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, AtomicUsize, Ordering};

/// Thread-safe metrics collector
#[derive(Debug, Clone)]
pub struct GOAPMetrics {
    /// Total requests processed
    total_requests: Arc<AtomicU64>,

    /// Successful requests
    successful_requests: Arc<AtomicU64>,

    /// Failed requests
    failed_requests: Arc<AtomicU64>,

    /// Total tokens used
    tokens_used: Arc<AtomicU64>,

    /// Tokens saved through pattern reuse
    tokens_saved: Arc<AtomicU64>,

    /// Average planning time in milliseconds
    avg_planning_time_ms: Arc<AtomicU64>,

    /// Average execution time in milliseconds
    avg_execution_time_ms: Arc<AtomicU64>,

    /// Cache hit rate (percentage)
    cache_hit_rate: Arc<AtomicU64>,

    /// Pattern reuse count
    pattern_reuse_count: Arc<AtomicU64>,

    /// Reactive replan count
    replan_count: Arc<AtomicU64>,

    /// Last reset time
    last_reset: Arc<AtomicU64>,

    /// Whether metrics are enabled
    enabled: Arc<AtomicBool>,

    /// Currently active requests
    active_requests: Arc<AtomicUsize>,
}

impl GOAPMetrics {
    /// Create new metrics collector
    pub fn new() -> Self {
        GOAPMetrics {
            total_requests: Arc::new(AtomicU64::new(0)),
            successful_requests: Arc::new(AtomicU64::new(0)),
            failed_requests: Arc::new(AtomicU64::new(0)),
            tokens_used: Arc::new(AtomicU64::new(0)),
            tokens_saved: Arc::new(AtomicU64::new(0)),
            avg_planning_time_ms: Arc::new(AtomicU64::new(0)),
            avg_execution_time_ms: Arc::new(AtomicU64::new(0)),
            cache_hit_rate: Arc::new(AtomicU64::new(0)),
            pattern_reuse_count: Arc::new(AtomicU64::new(0)),
            replan_count: Arc::new(AtomicU64::new(0)),
            last_reset: Arc::new(AtomicU64::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs(),
            )),
            enabled: Arc::new(AtomicBool::new(true)),
            active_requests: Arc::new(AtomicUsize::new(0)),
        }
    }

    /// Record a new request
    pub fn record_request(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.active_requests.fetch_add(1, Ordering::Relaxed);
    }

    /// Record a successful request
    pub fn record_success(
        &self,
        planning_time_ms: u64,
        execution_time_ms: u64,
        tokens_used: u64,
        tokens_saved: u64,
    ) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        self.successful_requests.fetch_add(1, Ordering::Relaxed);
        self.tokens_used.fetch_add(tokens_used, Ordering::Relaxed);
        self.tokens_saved.fetch_add(tokens_saved, Ordering::Relaxed);

        // Update averages
        let total_success = self.successful_requests.load(Ordering::Relaxed);
        let prev_planning_avg = self.avg_planning_time_ms.load(Ordering::Relaxed);
        let prev_execution_avg = self.avg_execution_time_ms.load(Ordering::Relaxed);

        let new_planning_avg =
            (prev_planning_avg * (total_success - 1) + planning_time_ms) / total_success;
        let new_execution_avg =
            (prev_execution_avg * (total_success - 1) + execution_time_ms) / total_success;

        self.avg_planning_time_ms
            .store(new_planning_avg, Ordering::Relaxed);
        self.avg_execution_time_ms
            .store(new_execution_avg, Ordering::Relaxed);

        self.active_requests.fetch_sub(1, Ordering::Relaxed);
    }

    /// Record a failed request
    pub fn record_failure(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }

        self.failed_requests.fetch_add(1, Ordering::Relaxed);
        self.active_requests.fetch_sub(1, Ordering::Relaxed);
    }

    /// Record pattern reuse
    pub fn record_pattern_reuse(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        self.pattern_reuse_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Record reactive replan
    pub fn record_replan(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        self.replan_count.fetch_add(1, Ordering::Relaxed);
    }

    /// Update cache hit rate
    pub fn update_cache_hit_rate(&self, hit_rate: f64) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        self.cache_hit_rate
            .store(hit_rate as u64, Ordering::Relaxed);
    }

    /// Enable metrics collection
    pub fn enable(&self) {
        self.enabled.store(true, Ordering::Relaxed);
    }

    /// Disable metrics collection
    pub fn disable(&self) {
        self.enabled.store(false, Ordering::Relaxed);
    }

    /// Reset all metrics
    pub fn reset(&self) {
        self.total_requests.store(0, Ordering::Relaxed);
        self.successful_requests.store(0, Ordering::Relaxed);
        self.failed_requests.store(0, Ordering::Relaxed);
        self.tokens_used.store(0, Ordering::Relaxed);
        self.tokens_saved.store(0, Ordering::Relaxed);
        self.avg_planning_time_ms.store(0, Ordering::Relaxed);
        self.avg_execution_time_ms.store(0, Ordering::Relaxed);
        self.cache_hit_rate.store(0, Ordering::Relaxed);
        self.pattern_reuse_count.store(0, Ordering::Relaxed);
        self.replan_count.store(0, Ordering::Relaxed);
        self.active_requests.store(0, Ordering::Relaxed);

        self.last_reset.store(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            Ordering::Relaxed,
        );
    }

    /// Get current metrics snapshot
    pub fn get_snapshot(&self) -> MetricsSnapshot {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);
        let failed = self.failed_requests.load(Ordering::Relaxed);

        MetricsSnapshot {
            total_requests: total,
            successful_requests: successful,
            failed_requests: failed,
            success_rate: if total > 0 {
                successful as f64 / total as f64
            } else {
                0.0
            },
            tokens_used: self.tokens_used.load(Ordering::Relaxed),
            tokens_saved: self.tokens_saved.load(Ordering::Relaxed),
            avg_planning_time_ms: self.avg_planning_time_ms.load(Ordering::Relaxed),
            avg_execution_time_ms: self.avg_execution_time_ms.load(Ordering::Relaxed),
            cache_hit_rate: self.cache_hit_rate.load(Ordering::Relaxed) as f64,
            pattern_reuse_count: self.pattern_reuse_count.load(Ordering::Relaxed),
            replan_count: self.replan_count.load(Ordering::Relaxed),
            active_requests: self.active_requests.load(Ordering::Relaxed),
            uptime_seconds: self.uptime_seconds(),
        }
    }

    /// Calculate uptime in seconds
    fn uptime_seconds(&self) -> u64 {
        let reset_time = self.last_reset.load(Ordering::Relaxed);
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            - reset_time
    }

    /// Get success rate (0-1)
    pub fn success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed);
        let successful = self.successful_requests.load(Ordering::Relaxed);

        if total > 0 {
            successful as f64 / total as f64
        } else {
            0.0
        }
    }

    /// Get token efficiency (tokens saved per token used)
    pub fn token_efficiency(&self) -> f64 {
        let tokens_used = self.tokens_used.load(Ordering::Relaxed);
        let tokens_saved = self.tokens_saved.load(Ordering::Relaxed);

        if tokens_used > 0 {
            tokens_saved as f64 / tokens_used as f64
        } else {
            0.0
        }
    }
}

impl Default for GOAPMetrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Snapshot of metrics at a point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSnapshot {
    pub total_requests: u64,
    pub successful_requests: u64,
    pub failed_requests: u64,
    pub success_rate: f64,
    pub tokens_used: u64,
    pub tokens_saved: u64,
    pub avg_planning_time_ms: u64,
    pub avg_execution_time_ms: u64,
    pub cache_hit_rate: f64,
    pub pattern_reuse_count: u64,
    pub replan_count: u64,
    pub active_requests: usize,
    pub uptime_seconds: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_creation() {
        let metrics = GOAPMetrics::new();
        let snapshot = metrics.get_snapshot();

        assert_eq!(snapshot.total_requests, 0);
        assert_eq!(snapshot.success_rate, 0.0);
    }

    #[test]
    fn test_record_success() {
        let metrics = GOAPMetrics::new();
        metrics.record_success(100, 500, 1000, 500);

        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.successful_requests, 1);
        assert_eq!(snapshot.tokens_used, 1000);
        assert_eq!(snapshot.avg_planning_time_ms, 100);
        assert_eq!(snapshot.avg_execution_time_ms, 500);
    }

    #[test]
    fn test_record_failure() {
        let metrics = GOAPMetrics::new();
        metrics.record_failure();

        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.failed_requests, 1);
        assert_eq!(snapshot.successful_requests, 0);
    }

    #[test]
    fn test_success_rate() {
        let metrics = GOAPMetrics::new();

        metrics.record_success(100, 500, 1000, 0);
        assert!(metrics.success_rate() >= 0.0);

        metrics.record_failure();
        assert!(metrics.success_rate() >= 0.0);
    }

    #[test]
    fn test_reset() {
        let metrics = GOAPMetrics::new();
        metrics.record_success(100, 500, 1000, 0);

        metrics.reset();

        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 0);
        assert_eq!(snapshot.successful_requests, 0);
    }
}
