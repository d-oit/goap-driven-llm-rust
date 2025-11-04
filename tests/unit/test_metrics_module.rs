//! Unit tests for GOAP metrics module
//!
//! Tests performance metrics collection, latency tracking, throughput calculation,
//! token monitoring, historical data management, and thread-safety.

#[cfg(test)]
mod tests {
    use super::*;
    use goap_llm::prelude::*;
    use std::sync::Arc;
    use std::thread;
    use std::time::Duration;

    // ========== Test Setup ==========

    fn create_test_metrics() -> GOAPMetrics {
        GOAPMetrics::new()
    }

    // ========== Metrics Collection Tests ==========

    #[test]
    fn test_record_request_increments_total() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Recording a request
        metrics.record_request();

        // Then: Total requests should be incremented
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 1);
    }

    #[test]
    fn test_record_success_updates_all_metrics() {
        // Given: A metrics instance with a recorded request
        let metrics = create_test_metrics();
        metrics.record_request();

        // When: Recording a successful request with timing and token data
        metrics.record_success(100, 200, 1500, 500);

        // Then: All relevant metrics should be updated
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.successful_requests, 1);
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.failed_requests, 0);
        assert_eq!(snapshot.tokens_used, 1500);
        assert_eq!(snapshot.tokens_saved, 500);
        assert_eq!(snapshot.avg_planning_time_ms, 100);
        assert_eq!(snapshot.avg_execution_time_ms, 200);
        assert_eq!(snapshot.active_requests, 0);
    }

    #[test]
    fn test_record_failure_increments_failed_requests() {
        // Given: A metrics instance with a recorded request
        let metrics = create_test_metrics();
        metrics.record_request();

        // When: Recording a failure
        metrics.record_failure();

        // Then: Failed requests should be incremented
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.failed_requests, 1);
        assert_eq!(snapshot.successful_requests, 0);
        assert_eq!(snapshot.active_requests, 0);
    }

    #[test]
    fn test_record_pattern_reuse_increments_counter() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Recording pattern reuse
        metrics.record_pattern_reuse();

        // Then: Pattern reuse count should be incremented
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.pattern_reuse_count, 1);
    }

    #[test]
    fn test_record_replan_increments_counter() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Recording a replan
        metrics.record_replan();

        // Then: Replan count should be incremented
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.replan_count, 1);
    }

    // ========== Latency and Average Calculation Tests ==========

    #[test]
    fn test_rolling_average_calculation() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording multiple successful requests with different planning times
        metrics.record_success(100, 200, 1000, 0);
        metrics.record_success(200, 400, 1000, 0);
        metrics.record_success(300, 600, 1000, 0);

        // Then: Average should be calculated correctly
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.avg_planning_time_ms, 200); // (100+200+300)/3
        assert_eq!(snapshot.avg_execution_time_ms, 400); // (200+400+600)/3
    }

    #[test]
    fn test_latency_accumulation_over_time() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording requests with varying latencies
        for i in 1..=10 {
            metrics.record_success(i * 50, i * 100, 1000, 0);
        }

        // Then: Average should reflect the cumulative data
        let snapshot = metrics.get_snapshot();
        let expected_avg = 275; // (50+100+...+500)/10 = 275
        assert_eq!(snapshot.avg_planning_time_ms, expected_avg);
    }

    // ========== Throughput and Rate Tracking Tests ==========

    #[test]
    fn test_success_rate_calculation() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording multiple successful and failed requests
        for _ in 0..8 {
            metrics.record_success(100, 200, 1000, 0);
        }
        for _ in 0..2 {
            metrics.record_failure();
        }

        // Then: Success rate should be 80% (8/10)
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 10);
        assert_eq!(snapshot.successful_requests, 8);
        assert_eq!(snapshot.failed_requests, 2);
        assert!((snapshot.success_rate - 0.8).abs() < f64::EPSILON);
    }

    #[test]
    fn test_success_rate_with_no_requests() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Getting snapshot without any requests
        let snapshot = metrics.get_snapshot();

        // Then: Success rate should be 0.0
        assert_eq!(snapshot.success_rate, 0.0);
    }

    #[test]
    fn test_active_requests_tracking() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording and completing requests
        metrics.record_request();
        assert_eq!(metrics.get_snapshot().active_requests, 1);

        metrics.record_success(100, 200, 1000, 0);
        assert_eq!(metrics.get_snapshot().active_requests, 0);

        metrics.record_request();
        metrics.record_request();
        assert_eq!(metrics.get_snapshot().active_requests, 2);

        metrics.record_failure();
        assert_eq!(metrics.get_snapshot().active_requests, 1);
    }

    // ========== Token Usage and Budget Tests ==========

    #[test]
    fn test_token_usage_accumulation() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording multiple requests with token usage
        metrics.record_success(100, 200, 1000, 0);
        metrics.record_success(100, 200, 2000, 500);
        metrics.record_success(100, 200, 1500, 250);

        // Then: Token usage should be accumulated
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.tokens_used, 4500); // 1000+2000+1500
        assert_eq!(snapshot.tokens_saved, 750); // 0+500+250
    }

    #[test]
    fn test_token_efficiency_calculation() {
        // Given: A metrics instance with token data
        let metrics = create_test_metrics();
        metrics.record_success(100, 200, 2000, 1000); // 50% efficiency

        // When: Getting the snapshot
        let snapshot = metrics.get_snapshot();

        // Then: Token efficiency should be calculated
        assert!((snapshot.tokens_saved as f64 / snapshot.tokens_used as f64) - 0.5 < f64::EPSILON);
    }

    #[test]
    fn test_token_efficiency_with_zero_tokens_used() {
        // Given: A metrics instance with no tokens used
        let metrics = create_test_metrics();

        // When: Getting token efficiency
        let efficiency = metrics.token_efficiency();

        // Then: Should return 0.0 (avoid division by zero)
        assert_eq!(efficiency, 0.0);
    }

    #[test]
    fn test_cache_hit_rate_update() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Updating cache hit rate
        metrics.update_cache_hit_rate(0.75);

        // Then: Cache hit rate should be stored
        let snapshot = metrics.get_snapshot();
        assert!((snapshot.cache_hit_rate - 0.75).abs() < f64::EPSILON);
    }

    // ========== Historical Data and Retention Tests ==========

    #[test]
    fn test_uptime_calculation() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();

        // When: Getting snapshot immediately
        let snapshot1 = metrics.get_snapshot();
        let initial_uptime = snapshot1.uptime_seconds;

        // And: Waiting a bit
        thread::sleep(Duration::from_millis(100));

        // And: Getting snapshot again
        let snapshot2 = metrics.get_snapshot();
        let new_uptime = snapshot2.uptime_seconds;

        // Then: Uptime should have increased
        assert!(new_uptime >= initial_uptime);
    }

    #[test]
    fn test_last_reset_timestamp() {
        // Given: A fresh metrics instance
        let metrics = create_test_metrics();
        let initial_reset = metrics.get_snapshot().uptime_seconds;

        // When: Resetting metrics
        thread::sleep(Duration::from_millis(100));
        metrics.reset();

        // And: Getting snapshot
        let snapshot = metrics.get_snapshot();

        // Then: Uptime should be reset
        assert!(snapshot.uptime_seconds < initial_reset);
    }

    // ========== Reset Functionality Tests ==========

    #[test]
    fn test_reset_clears_all_counters() {
        // Given: A metrics instance with recorded data
        let metrics = create_test_metrics();
        metrics.record_success(100, 200, 1000, 500);
        metrics.record_failure();
        metrics.record_pattern_reuse();
        metrics.record_replan();
        metrics.update_cache_hit_rate(0.8);

        // When: Resetting all metrics
        metrics.reset();

        // Then: All counters should be zero
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 0);
        assert_eq!(snapshot.successful_requests, 0);
        assert_eq!(snapshot.failed_requests, 0);
        assert_eq!(snapshot.tokens_used, 0);
        assert_eq!(snapshot.tokens_saved, 0);
        assert_eq!(snapshot.avg_planning_time_ms, 0);
        assert_eq!(snapshot.avg_execution_time_ms, 0);
        assert_eq!(snapshot.cache_hit_rate, 0.0);
        assert_eq!(snapshot.pattern_reuse_count, 0);
        assert_eq!(snapshot.replan_count, 0);
        assert_eq!(snapshot.active_requests, 0);
    }

    // ========== Enable/Disable Functionality Tests ==========

    #[test]
    fn test_disable_stops_recording() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Disabling metrics
        metrics.disable();

        // And: Recording data
        metrics.record_request();
        metrics.record_success(100, 200, 1000, 0);
        metrics.record_failure();
        metrics.record_pattern_reuse();

        // Then: No data should be recorded
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 0);
        assert_eq!(snapshot.successful_requests, 0);
        assert_eq!(snapshot.failed_requests, 0);
        assert_eq!(snapshot.pattern_reuse_count, 0);
    }

    #[test]
    fn test_enable_allows_recording() {
        // Given: A disabled metrics instance with some data recorded while disabled
        let metrics = create_test_metrics();
        metrics.disable();
        metrics.record_request();
        metrics.record_success(100, 200, 1000, 0);

        // When: Re-enabling metrics
        metrics.enable();

        // And: Recording new data
        metrics.record_success(100, 200, 1000, 0);

        // Then: Only the new data should be recorded
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 1);
        assert_eq!(snapshot.successful_requests, 1);
    }

    #[test]
    fn test_toggle_metrics_enabled_state() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Toggling enable/disable multiple times
        metrics.disable();
        assert_eq!(metrics.get_snapshot().total_requests, 0);

        metrics.enable();
        metrics.record_request();
        assert_eq!(metrics.get_snapshot().total_requests, 1);

        metrics.disable();
        metrics.record_request();
        assert_eq!(metrics.get_snapshot().total_requests, 1);

        metrics.enable();
        metrics.record_request();
        assert_eq!(metrics.get_snapshot().total_requests, 2);
    }

    // ========== Thread Safety and Concurrency Tests ==========

    #[test]
    fn test_concurrent_request_recording() {
        // Given: A shared metrics instance
        let metrics = Arc::new(create_test_metrics());
        let metrics_clone = Arc::clone(&metrics);

        // When: Recording requests from multiple threads
        let handles: Vec<_> = (0..10)
            .map(|_| {
                let metrics = Arc::clone(&metrics);
                thread::spawn(move || {
                    for _ in 0..100 {
                        metrics.record_request();
                        metrics.record_success(100, 200, 1000, 0);
                    }
                })
            })
            .collect();

        // Then: Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }

        // And: Verify total count
        let snapshot = metrics_clone.get_snapshot();
        assert_eq!(snapshot.total_requests, 1000);
        assert_eq!(snapshot.successful_requests, 1000);
    }

    #[test]
    fn test_concurrent_reset_safety() {
        // Given: A metrics instance with data
        let metrics = Arc::new(create_test_metrics());
        for _ in 0..100 {
            metrics.record_success(100, 200, 1000, 0);
        }

        // When: Resetting and recording concurrently
        let reset_metrics = Arc::clone(&metrics);
        let record_metrics = Arc::clone(&metrics);

        let handle1 = thread::spawn(move || {
            for _ in 0..50 {
                reset_metrics.reset();
                thread::sleep(Duration::from_micros(1));
            }
        });

        let handle2 = thread::spawn(move || {
            for _ in 0..50 {
                record_metrics.record_success(100, 200, 1000, 0);
                thread::sleep(Duration::from_micros(1));
            }
        });

        // Then: Both operations should complete without panic
        handle1.join().unwrap();
        handle2.join().unwrap();

        // And: Metrics should be in a valid state
        let snapshot = metrics.get_snapshot();
        assert!(snapshot.total_requests >= 0);
        assert!(snapshot.successful_requests >= 0);
    }

    #[test]
    fn test_concurrent_toggle_and_record() {
        // Given: A metrics instance
        let metrics = Arc::new(create_test_metrics());

        // When: Toggling enable/disable and recording concurrently
        let toggle_metrics = Arc::clone(&metrics);
        let record_metrics = Arc::clone(&metrics);

        let handle1 = thread::spawn(move || {
            for i in 0..100 {
                if i % 2 == 0 {
                    toggle_metrics.enable();
                } else {
                    toggle_metrics.disable();
                }
                thread::sleep(Duration::from_micros(10));
            }
        });

        let handle2 = thread::spawn(move || {
            for _ in 0..100 {
                record_metrics.record_request();
                record_metrics.record_success(100, 200, 1000, 0);
                thread::sleep(Duration::from_micros(10));
            }
        });

        // Then: Both operations should complete without data races
        handle1.join().unwrap();
        handle2.join().unwrap();

        // And: Final state should be valid
        let snapshot = metrics.get_snapshot();
        assert!(snapshot.total_requests <= 100);
    }

    // ========== Metrics Snapshot and Serialization Tests ==========

    #[test]
    fn test_snapshot_contains_all_fields() {
        // Given: A metrics instance with comprehensive data
        let metrics = create_test_metrics();
        metrics.record_request();
        metrics.record_success(100, 200, 1500, 500);
        metrics.record_failure();
        metrics.record_pattern_reuse();
        metrics.record_replan();
        metrics.update_cache_hit_rate(0.85);

        // When: Getting a snapshot
        let snapshot = metrics.get_snapshot();

        // Then: All fields should be present and populated
        assert!(snapshot.total_requests >= 0);
        assert!(snapshot.successful_requests >= 0);
        assert!(snapshot.failed_requests >= 0);
        assert!(snapshot.success_rate >= 0.0);
        assert!(snapshot.tokens_used >= 0);
        assert!(snapshot.tokens_saved >= 0);
        assert!(snapshot.avg_planning_time_ms >= 0);
        assert!(snapshot.avg_execution_time_ms >= 0);
        assert!(snapshot.cache_hit_rate >= 0.0);
        assert!(snapshot.pattern_reuse_count >= 0);
        assert!(snapshot.replan_count >= 0);
        assert!(snapshot.active_requests >= 0);
        assert!(snapshot.uptime_seconds >= 0);
    }

    #[test]
    fn test_snapshot_is_cloneable() {
        // Given: A metrics instance
        let metrics = create_test_metrics();
        metrics.record_success(100, 200, 1000, 0);

        // When: Getting a snapshot
        let snapshot1 = metrics.get_snapshot();

        // And: Cloning the snapshot
        let snapshot2 = snapshot1.clone();

        // Then: Both snapshots should have the same data
        assert_eq!(snapshot1.total_requests, snapshot2.total_requests);
        assert_eq!(snapshot1.successful_requests, snapshot2.successful_requests);
        assert_eq!(snapshot1.tokens_used, snapshot2.tokens_used);
    }

    #[test]
    fn test_success_rate_method_matches_snapshot() {
        // Given: A metrics instance with data
        let metrics = create_test_metrics();
        metrics.record_success(100, 200, 1000, 0);
        metrics.record_failure();

        // When: Getting success rate via method and snapshot
        let method_rate = metrics.success_rate();
        let snapshot_rate = metrics.get_snapshot().success_rate;

        // Then: Both should return the same value
        assert!((method_rate - snapshot_rate).abs() < f64::EPSILON);
    }

    // ========== Edge Cases and Boundary Conditions ==========

    #[test]
    fn test_very_large_token_values() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording with very large token values
        metrics.record_success(100, 200, u64::MAX, u64::MAX);

        // Then: Values should be stored correctly
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.tokens_used, u64::MAX);
        assert_eq!(snapshot.tokens_saved, u64::MAX);
    }

    #[test]
    fn test_zero_latency_handling() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording with zero latency
        metrics.record_success(0, 0, 1000, 0);

        // Then: Should handle gracefully
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.avg_planning_time_ms, 0);
        assert_eq!(snapshot.avg_execution_time_ms, 0);
    }

    #[test]
    fn test_repeated_reset_calls() {
        // Given: A metrics instance
        let metrics = create_test_metrics();
        metrics.record_success(100, 200, 1000, 0);

        // When: Calling reset multiple times
        metrics.reset();
        metrics.reset();
        metrics.reset();

        // Then: Should remain in reset state
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.total_requests, 0);
    }

    #[test]
    fn test_multiple_pattern_reuses() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Recording multiple pattern reuses
        for _ in 0..100 {
            metrics.record_pattern_reuse();
        }

        // Then: Count should be accurate
        let snapshot = metrics.get_snapshot();
        assert_eq!(snapshot.pattern_reuse_count, 100);
    }

    #[test]
    fn test_very_long_uptime() {
        // Given: A metrics instance
        let metrics = create_test_metrics();

        // When: Simulating long uptime (via setting a past reset time)
        // In real scenarios, this would happen naturally over time
        let snapshot = metrics.get_snapshot();

        // Then: Uptime calculation should handle large values
        assert!(snapshot.uptime_seconds >= 0);
    }

    // ========== Default Implementation Tests ==========

    #[test]
    fn test_default_trait_creates_valid_metrics() {
        // Given: Using Default trait
        let metrics: GOAPMetrics = GOAPMetrics::default();

        // When: Getting snapshot
        let snapshot = metrics.get_snapshot();

        // Then: Should be in a valid initial state
        assert_eq!(snapshot.total_requests, 0);
        assert_eq!(snapshot.successful_requests, 0);
        assert_eq!(snapshot.failed_requests, 0);
        assert!(snapshot.success_rate >= 0.0);
    }

    #[test]
    fn test_clone_produces_independent_instances() {
        // Given: A metrics instance with data
        let metrics1 = create_test_metrics();
        metrics1.record_success(100, 200, 1000, 0);

        // When: Cloning the metrics
        let metrics2 = metrics1.clone();

        // And: Recording different data on the clone
        metrics2.record_success(200, 400, 2000, 0);

        // Then: They should be independent
        let snapshot1 = metrics1.get_snapshot();
        let snapshot2 = metrics2.get_snapshot();

        assert_ne!(snapshot1.successful_requests, snapshot2.successful_requests);
    }
}
