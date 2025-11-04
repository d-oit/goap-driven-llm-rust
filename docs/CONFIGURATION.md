# Configuration Guide

Configure GOAP for your specific use case and performance requirements.

## Configuration Overview

GOAP is configurable through the `GOAPConfig` struct:

```rust
use goap_llm::prelude::*;

let config = GOAPConfig {
    // Planning parameters
    max_plan_depth: 20,
    planning_timeout_ms: 5000,

    // Pattern cache settings
    pattern_cache_size: 1000,
    pattern_confidence_threshold: 70,

    // Token budget management
    token_budget_default: 5000,
    token_budget_critical: 100,

    // Reactive replanning
    max_replans: 3,
    replan_threshold: ReplanThreshold::OnCriticalFailure,

    // Performance tuning
    enable_parallel_execution: true,
    cache_ttl_hours: 24,
};
```

## Planning Configuration

### Plan Depth Limit

```rust
let config = GOAPConfig {
    max_plan_depth: 20,  // Maximum 20 actions in a plan

    // Default: 20
    // Range: 1-100
    // Impact: Higher = more thorough, slower
};
```

**Trade-offs:**
- **Too low**: Can't solve complex problems
- **Too high**: Slow planning, unnecessary complexity

### Planning Timeout

```rust
let config = GOAPConfig {
    planning_timeout_ms: 5000,  // Give up after 5 seconds

    // Default: 5000ms
    // Range: 100-30000ms
    // Impact: Lower = faster failures, higher = better plans
};
```

### Heuristic Configuration

```rust
let heuristic_config = HeuristicConfig {
    token_weight: 0.5,      // 50% weight on token cost
    time_weight: 0.3,       // 30% weight on execution time
    success_weight: 0.2,    // 20% weight on success probability

    // Weights should sum to 1.0
    // total = token_weight + time_weight + success_weight
};
```

## Pattern Cache Configuration

### Cache Size

```rust
let config = GOAPConfig {
    pattern_cache_size: 1000,  // Maximum 1000 patterns

    // Default: 1000
    // Range: 10-10000
    // Impact: Higher = more patterns, more memory
};
```

### Pattern Confidence Threshold

```rust
let config = GOAPConfig {
    pattern_confidence_threshold: 70,  // Use patterns >70%

    // Default: 70
    // Range: 0-100
    // Impact: Higher = only use high-quality patterns
};
```

**Thresholds by use case:**
- **Safety-critical**: 85-90% (only proven patterns)
- **General use**: 70-82% (balanced)
- **Experimentation**: 50-60% (more patterns, lower quality)

### Cache Time-to-Live

```rust
let config = GOAPConfig {
    cache_ttl_hours: 24,  // Patterns expire after 24 hours

    // Default: 24 hours
    // Range: 1-720 hours (30 days)
    // Impact: Longer = stale patterns, shorter = lose good patterns
};
```

### Cache Eviction Policy

```rust
enum EvictionPolicy {
    LRU,        // Least Recently Used
    LFU,        // Least Frequently Used
    FIFO,       // First In, First Out
    Random,     // Random eviction
}

let config = EvictionConfig {
    policy: EvictionPolicy::LRU,
    max_memory_mb: 256,  // Memory limit in MB
};
```

## Token Budget Configuration

### Default Budget

```rust
let config = GOAPConfig {
    token_budget_default: 5000,  // Default 5000 tokens

    // Default: 5000
    // Range: 100-100000
    // Impact: Higher = more expensive, better results
};
```

**Budgets by request type:**
- **Simple queries**: 500-1000 tokens
- **Code generation**: 2000-5000 tokens
- **Complex analysis**: 5000-10000 tokens
- **Creative tasks**: 10000+ tokens

### Critical Threshold

```rust
let config = GOAPConfig {
    token_budget_critical: 100,  // Trigger optimization at 100 tokens

    // Default: 100
    // Range: 10-1000
    // Impact: Lower = more aggressive optimization
};
```

### Compression Settings

```rust
let compression_config = CompressionConfig {
    enabled: true,
    max_compression_ratio: 0.5,  // Can compress up to 50%
    compression_threshold: 200,   // Compress when <200 tokens left
};
```

## Reactive Replanning Configuration

### Maximum Replans

```rust
let config = GOAPConfig {
    max_replans: 3,  // Allow up to 3 replans per request

    // Default: 3
    // Range: 0-10
    // Impact: Higher = more recovery, slower
};
```

### Replan Threshold

```rust
enum ReplanThreshold {
    Never,                    // Don't replan
    OnCriticalFailure,        // Only on critical errors
    OnAnyFailure,             // Replan on any failure
    Predictive,               // Replan before failure
}

let config = ReplanConfig {
    threshold: ReplanThreshold::OnCriticalFailure,
};
```

### Recovery Strategies

```rust
let recovery_config = RecoveryConfig {
    strategies: vec![
        RecoveryStrategy::PatternReuse,
        RecoveryStrategy::Compression,
        RecoveryStrategy::AlternativeSchema,
        RecoveryStrategy::FallbackTemplate,
    ],
    parallel_attempts: true,  // Try strategies in parallel
};
```

## Performance Tuning

### Parallel Execution

```rust
let config = GOAPConfig {
    enable_parallel_execution: true,

    // Default: true
    // When: Multiple independent actions
    // Impact: Faster execution, more CPU usage
};
```

### Batch Size

```rust
let config = BatchConfig {
    batch_size: 10,  // Process 10 requests in batch

    // Default: 10
    // Range: 1-100
    // Impact: Higher = better throughput, higher latency
};
```

### Concurrency Limit

```rust
let config = ConcurrencyConfig {
    max_concurrent_requests: 100,

    // Default: 100
    // Range: 1-1000
    // Impact: Higher = better throughput, more memory
};
```

## Metrics Configuration

### Metrics Collection

```rust
let config = MetricsConfig {
    enabled: true,
    collection_interval: Duration::from_secs(60),  // Collect every minute
    retention_period: Duration::from_secs(86400),  // Keep for 24 hours
};
```

### Metric Types

```rust
enum MetricType {
    ExecutionTime,
    TokenUsage,
    SuccessRate,
    CacheHitRate,
    ReplanCount,
    Custom(String),
}

let config = MetricsConfig {
    enabled_metrics: vec![
        MetricType::ExecutionTime,
        MetricType::TokenUsage,
        MetricType::SuccessRate,
        MetricType::CacheHitRate,
    ],
};
```

## Environment-Specific Configuration

### Development

```rust
// Development environment
let config = GOAPConfig {
    planning_timeout_ms: 10000,  // Longer timeout for debugging
    pattern_cache_size: 100,     // Small cache
    enable_detailed_logging: true,
    enable_metrics: true,
};
```

### Production

```rust
// Production environment
let config = GOAPConfig {
    planning_timeout_ms: 2000,   // Faster timeout
    pattern_cache_size: 5000,    // Large cache
    enable_detailed_logging: false,
    enable_metrics: true,
    max_memory_mb: 1024,
};
```

### Testing

```rust
// Testing environment
let config = GOAPConfig {
    planning_timeout_ms: 500,    // Fast timeout
    pattern_cache_size: 10,      // Minimal cache
    max_replans: 1,              // Limited retries
    enable_metrics: false,       // Disable for speed
};
```

## Configuration Loading

### From File

```rust
use serde_json;

let config_str = std::fs::read_to_string("goap_config.json")?;
let config: GOAPConfig = serde_json::from_str(&config_str)?;
```

Example `goap_config.json`:
```json
{
  "token_budget_default": 5000,
  "pattern_confidence_threshold": 75,
  "max_replans": 3,
  "cache_ttl_hours": 48,
  "enable_parallel_execution": true
}
```

### From Environment

```rust
let config = GOAPConfig {
    token_budget_default: std::env::var("GOAP_BUDGET")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()?,
    pattern_confidence_threshold: std::env::var("GOAP_PATTERN_THRESHOLD")
        .unwrap_or_else(|_| "70".to_string())
        .parse()?,
    ..Default::default()
};
```

### From CLI

```rust
let matches = clap::App::new("goap")
    .arg(clap::Arg::with_name("budget")
        .long("budget")
        .value_name("TOKENS")
        .help("Token budget"))
    .get_matches();

let config = GOAPConfig {
    token_budget_default: matches.value_of("budget")
        .unwrap_or("5000")
        .parse()?,
    ..Default::default()
};
```

## Configuration Validation

### Validate Config

```rust
fn validate_config(config: &GOAPConfig) -> Result<(), ConfigError> {
    // Check weights sum to 1.0
    let heuristic_total = config.token_weight
        + config.time_weight
        + config.success_weight;

    if (heuristic_total - 1.0).abs() > 0.01 {
        return Err(ConfigError::InvalidHeuristicWeights);
    }

    // Check ranges
    if config.max_plan_depth == 0 {
        return Err(ConfigError::InvalidPlanDepth);
    }

    if config.pattern_confidence_threshold > 100 {
        return Err(ConfigError::InvalidConfidenceThreshold);
    }

    Ok(())
}
```

### Default Configuration

```rust
impl Default for GOAPConfig {
    fn default() -> Self {
        GOAPConfig {
            // Planning
            max_plan_depth: 20,
            planning_timeout_ms: 5000,
            token_weight: 0.5,
            time_weight: 0.3,
            success_weight: 0.2,

            // Cache
            pattern_cache_size: 1000,
            pattern_confidence_threshold: 70,
            cache_ttl_hours: 24,
            eviction_policy: EvictionPolicy::LRU,
            max_memory_mb: 256,

            // Budget
            token_budget_default: 5000,
            token_budget_critical: 100,

            // Replanning
            max_replans: 3,
            replan_threshold: ReplanThreshold::OnCriticalFailure,

            // Performance
            enable_parallel_execution: true,
            batch_size: 10,
            max_concurrent_requests: 100,

            // Metrics
            enable_metrics: true,
            metrics_interval: Duration::from_secs(60),
        }
    }
}
```

## Configuration Best Practices

### 1. Start with Defaults
```rust
// Good: Start with reasonable defaults
let config = GOAPConfig::default();

// Then adjust based on needs
config.pattern_confidence_threshold = 80;  // Higher for safety
```

### 2. Tune Incrementally
```rust
// Bad: Change everything at once
let config = GOAPConfig {
    max_plan_depth: 100,
    pattern_cache_size: 10000,
    max_replans: 10,
};

// Good: Change one parameter at a time
let config = GOAPConfig::default();
config.max_plan_depth = 25;  // Increase from 20
```

### 3. Document Changes
```rust
// Good: Document why you changed it
let config = GOAPConfig {
    // Increased to handle more complex workflows
    max_plan_depth: 30,

    // Increased to cache more patterns for repeated use
    pattern_cache_size: 2000,

    // Default is fine for our use case
    // (no change needed)
};
```

### 4. Monitor Impact
```rust
// Track metrics after configuration changes
let metrics_before = system.get_metrics();
apply_new_config(&system, new_config);
let metrics_after = system.get_metrics();

compare_metrics(&metrics_before, &metrics_after);
```

## Common Configurations

### High-Throughput

```rust
let config = GOAPConfig {
    batch_size: 50,
    max_concurrent_requests: 500,
    enable_parallel_execution: true,
    planning_timeout_ms: 1000,  // Fast timeout
    pattern_cache_size: 5000,
};
```

### Low-Latency

```rust
let config = GOAPConfig {
    planning_timeout_ms: 500,   // Very fast
    max_plan_depth: 10,         // Shallow plans
    enable_parallel_execution: true,
    max_replans: 1,             // Limited retries
};
```

### High-Reliability

```rust
let config = GOAPConfig {
    max_replans: 5,             // More recovery attempts
    replan_threshold: ReplanThreshold::OnAnyFailure,
    pattern_confidence_threshold: 85,  // Only proven patterns
    token_budget_default: 10000, // Higher budget
};
```

### Cost-Optimized

```rust
let config = GOAPConfig {
    token_budget_default: 2000,  // Lower budget
    pattern_confidence_threshold: 60,  // More patterns
    compression_enabled: true,
    cache_ttl_hours: 72,         // Keep patterns longer
};
```

## Troubleshooting

### Configuration Not Applied
```rust
// Check: Did you use the config?
let system = GOAPSystem::new_with_config(config);  // Must use this!

// Not this:
let system = GOAPSystem::new();  // Uses default config!
```

### Performance Degradation
```rust
// Monitor after config changes
let metrics = system.get_metrics();
if metrics.avg_response_time > expected {
    // Revert problematic changes
    system.update_config(old_config);
}
```

### Out of Memory
```rust
// Set memory limits
let config = GOAPConfig {
    max_memory_mb: 512,  // Limit memory usage
    pattern_cache_size: 1000,  // Reduce cache size
};
```

## Next Steps

- Review [Performance Guide](PERFORMANCE.md) for optimization strategies
- Study [Tutorial: Planning](TUTORIAL_PLANNING.md) for planning details
- Check [Error Handling Guide](ERROR_HANDLING.md) for best practices

## Reference

Full configuration reference available in the [API documentation](../docs/api/index.html).
