# Performance Guide

Optimize GOAP for your use case and measure performance.

## Performance Characteristics

### Planning Performance

**Time Complexity**: O(b^d)
- b = branching factor (average actions per state)
- d = plan depth (number of steps to goal)

**Space Complexity**: O(b^d)
- Nodes stored in priority queue
- Maximum queue size during search

**Example Performance**:
```
Scenario 1: Simple (b=3, d=5)
- Nodes explored: ~243
- Time: <1ms
- Memory: ~5KB

Scenario 2: Medium (b=5, d=10)
- Nodes explored: ~9.7 million (worst case)
- Typical with good heuristic: ~100,000
- Time: 10-50ms
- Memory: ~50MB

Scenario 3: Complex (b=10, d=15)
- Nodes explored: Very large
- Requires: Strong heuristics, timeouts
- Time: 100-500ms
- Memory: 100-500MB
```

### Cache Performance

**Pattern Lookup**: O(log n)
- LSH hash → bucket → compare candidates
- Typical: 3-5 comparisons

**Memory Usage**:
- 1 pattern ≈ 1KB
- 1000 patterns ≈ 1MB
- 10,000 patterns ≈ 10MB

### Execution Performance

**Per Action**:
- Simple action: 1-10ms
- Complex action: 50-200ms
- LLM call: 500-3000ms

**End-to-End**:
- Pattern reuse: 200-800ms
- Full planning: 1000-3000ms
- Reactive replanning: +100-500ms

## Optimization Strategies

### 1. Improve Heuristic

**Good Heuristic** → Fewer nodes explored → Faster planning

```rust
pub struct OptimizedHeuristic {
    domain_knowledge: HashMap<String, f64>,  // Pre-computed costs
}

impl Heuristic for OptimizedHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let base = self.estimate_base_cost(state, goal);

        // Use domain knowledge
        if let Some(pattern) = self.domain_knowledge.get(&state.current_request) {
            return base * pattern;  // Use cached estimate
        }

        base
    }
}
```

**Benefits**:
- Reduces nodes explored by 50-82%
- Faster planning for similar requests
- Consistent performance

### 2. Cache Heuristic Values

```rust
struct HeuristicCache {
    cache: HashMap<StateHash, f64>,
    hits: u64,
    misses: u64,
}

impl HeuristicCache {
    fn new() -> Self {
        HeuristicCache {
            cache: HashMap::new(),
            hits: 0,
            misses: 0,
        }
    }

    fn get(&mut self, state: &WorldState, goal: &GoalState) -> Option<f64> {
        let hash = self.calculate_hash(state, goal);

        if let Some(value) = self.cache.get(&hash) {
            self.hits += 1;
            return Some(*value);
        }

        self.misses += 1;
        None
    }

    fn insert(&mut self, state: &WorldState, goal: &GoalState, value: f64) {
        let hash = self.calculate_hash(state, goal);
        self.cache.insert(hash, value);
    }

    fn hit_rate(&self) -> f64 {
        let total = self.hits + self.misses;
        if total == 0 { 0.0 } else { self.hits as f64 / total as f64 }
    }
}
```

### 3. Limit Plan Depth

```rust
let config = GOAPConfig {
    max_plan_depth: 10,  // Reduce from default 20

    // Benefits:
    // - Faster planning (less exploration)
    // - Lower memory usage
    // - Simpler, more predictable plans

    // Trade-offs:
    // - May not solve complex problems
    // - Might need to split complex requests
};
```

### 4. Use Pattern Reuse Aggressively

```rust
let config = GOAPConfig {
    pattern_confidence_threshold: 60,  // Lower threshold

    // Benefits:
    // - More patterns used
    // - Faster responses (patterns are fast)
    // - Lower token usage

    // Trade-offs:
    // - Lower quality responses (patterns may not fit perfectly)
    // - Risk of pattern hallucination
};
```

### 5. Enable Parallel Execution

```rust
let config = GOAPConfig {
    enable_parallel_execution: true,

    // Benefits:
    // - Faster execution (parallel actions)
    // - Better CPU utilization

    // Requirements:
    // - Actions must be independent
    // - No shared mutable state

    // Use case:
    let independent_actions = vec![
        Action::new(ActionType::DetectSchemaType),
        Action::new(ActionType::CheckPatternCache),
    ];
    // These can run in parallel!
```

### 6. Tune Batch Processing

```rust
let config = BatchConfig {
    batch_size: 50,  // Process 50 requests together

    // Benefits:
    // - Higher throughput
    // - Better resource utilization
    // - Reduced overhead per request

    // Trade-offs:
    // - Higher latency (wait for batch)
    // - More memory (batch state)

    // Use case: High-throughput scenarios
};
```

## Performance Monitoring

### Key Metrics

```rust
struct PerformanceMetrics {
    planning_time_ms: f64,
    execution_time_ms: f64,
    cache_hit_rate: f64,
    tokens_used: u32,
    memory_usage_mb: f64,
    nodes_explored: u64,
    plan_depth: u32,
}
```

### Collecting Metrics

```rust
impl GOAPSystem {
    async fn process_request_with_metrics(
        &self,
        world_state: &mut WorldState,
        actions: Vec<Action>,
        goals: GoalState,
    ) -> Result<ExecutionResult, Error> {
        let start_time = std::time::Instant::now();
        let initial_memory = self.get_memory_usage();

        // Track planning separately
        let planning_start = std::time::Instant::now();
        let plan = self.planner.find_plan(world_state, &actions, &goals).await?;
        let planning_time = planning_start.elapsed();

        // Execute
        let result = self.executor.execute(&plan, world_state).await?;

        // Calculate metrics
        let total_time = start_time.elapsed();
        let memory_delta = self.get_memory_usage() - initial_memory;

        let metrics = PerformanceMetrics {
            planning_time_ms: planning_time.as_millis() as f64,
            execution_time_ms: (total_time - planning_time).as_millis() as f64,
            cache_hit_rate: self.cache.hit_rate(),
            tokens_used: result.total_tokens_used,
            memory_usage_mb: memory_delta as f64 / 1024.0 / 1024.0,
            nodes_explored: plan.nodes_explored,
            plan_depth: plan.actions.len() as u32,
        };

        self.metrics.record(metrics);
        Ok(result)
    }
}
```

### Benchmarking

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_planning(c: &mut Criterion) {
    c.bench_function("planning_simple", |b| {
        let (state, actions, goals) = create_simple_scenario();

        b.iter(|| {
            let result = GOAPPlanner::new()
                .plan(black_box(&state), black_box(&actions), black_box(&goals));
            black_box(result);
        })
    });

    c.bench_function("planning_complex", |b| {
        let (state, actions, goals) = create_complex_scenario();

        b.iter(|| {
            let result = GOAPPlanner::new()
                .plan(black_box(&state), black_box(&actions), black_box(&goals));
            black_box(result);
        })
    });

    c.bench_function("pattern_lookup", |b| {
        let cache = create_populated_cache();

        b.iter(|| {
            let pattern = cache.lookup(black_box("test request"));
            black_box(pattern);
        })
    });
}

criterion_group!(benches, benchmark_planning);
criterion_main!(benches);
```

### Performance Profiling

```rust
// Use tracing for performance profiling
use tracing::{span, Level};

impl GOAPSystem {
    async fn process_request(
        &self,
        world_state: &mut WorldState,
        actions: Vec<Action>,
        goals: GoalState,
    ) -> Result<ExecutionResult, Error> {
        let _span = span!(Level::INFO, "process_request").entered();

        // Planning phase
        let _planning_span = span!(Level::INFO, "planning").entered();
        let plan = self.planner.find_plan(world_state, &actions, &goals).await?;
        _planning_span.exit();

        // Execution phase
        let _execution_span = span!(Level::INFO, "execution").entered();
        let result = self.executor.execute(&plan, world_state).await?;
        _execution_span.exit();

        Ok(result)
    }
}

// Run with tracing enabled:
// RUST_LOG=trace cargo run
```

## Configuration for Performance

### High Throughput

```rust
let config = GOAPConfig {
    // Faster planning
    planning_timeout_ms: 1000,
    max_plan_depth: 10,

    // Aggressive caching
    pattern_cache_size: 10000,
    pattern_confidence_threshold: 50,
    cache_ttl_hours: 72,

    // Parallel execution
    enable_parallel_execution: true,
    max_concurrent_requests: 100,

    // Batch processing
    batch_size: 50,
};
```

### Low Latency

```rust
let config = GOAPConfig {
    // Very fast planning
    planning_timeout_ms: 500,
    max_plan_depth: 5,

    // Use patterns aggressively
    pattern_confidence_threshold: 60,
    enable_pattern_prefetch: true,

    // No retries
    max_replans: 0,

    // Smaller cache (faster lookup)
    pattern_cache_size: 100,
};
```

### High Quality

```rust
let config = GOAPConfig {
    // Thorough planning
    planning_timeout_ms: 10000,
    max_plan_depth: 30,

    // Strict caching
    pattern_confidence_threshold: 85,

    // More retries
    max_replans: 5,

    // Larger cache (more patterns)
    pattern_cache_size: 10000,
};
```

### Low Memory

```rust
let config = GOAPConfig {
    // Smaller caches
    pattern_cache_size: 100,  // 100 patterns = ~100KB
    max_memory_mb: 256,       // Limit total memory

    // Shallow plans
    max_plan_depth: 10,

    // No caching
    cache_ttl_hours: 1,       // Expire quickly
};
```

## Common Performance Issues

### Issue 1: Slow Planning

**Symptoms**: Planning takes >1 second
**Causes**:
- Weak heuristic
- Deep plans (high depth)
- Many actions to choose from

**Solutions**:
```rust
// 1. Improve heuristic
config.heuristic = Box::new(OptimizedHeuristic::new());

// 2. Limit depth
config.max_plan_depth = 10;

// 3. Reduce action count
actions.retain(|a| a.get_cost() < 500);
```

### Issue 2: High Memory Usage

**Symptoms**: Memory usage grows unbounded
**Causes**:
- Large pattern cache
- Deep plans
- No cache cleanup

**Solutions**:
```rust
// 1. Limit cache size
config.pattern_cache_size = 1000;

// 2. Set TTL
config.cache_ttl_hours = 24;

// 3. Enable eviction
config.eviction_policy = EvictionPolicy::LRU;
```

### Issue 3: Low Cache Hit Rate

**Symptoms**: Cache hit rate <50%
**Causes**:
- High confidence threshold
- Request diversity
- Cache too small

**Solutions**:
```rust
// 1. Lower threshold
config.pattern_confidence_threshold = 60;

// 2. Increase cache size
config.pattern_cache_size = 5000;

// 3. Check similarity calculation
config.similarity_threshold = 0.7;  // Lower = more matches
```

### Issue 4: Timeouts

**Symptoms**: Frequent timeouts
**Causes**:
- Timeout too short
- Complex planning
- Slow actions

**Solutions**:
```rust
// 1. Increase timeout
config.planning_timeout_ms = 10000;

// 2. Simplify problem
config.max_plan_depth = 10;

// 3. Use patterns aggressively
config.pattern_confidence_threshold = 70;
```

### Issue 5: Token Budget Exceeded

**Symptoms**: Frequent budget violations
**Causes**:
- Budget too low
- Inefficient plans
- No pattern reuse

**Solutions**:
```rust
// 1. Increase budget
config.token_budget_default = 10000;

// 2. Enable compression
config.compression_enabled = true;
config.compression_threshold = 200;

// 3. Use patterns
config.pattern_confidence_threshold = 60;
```

## Performance Testing

### Load Testing

```rust
#[tokio::test]
async fn test_high_load() {
    let system = GOAPSystem::new();
    let mut handles = vec![];

    // Launch 100 concurrent requests
    for i in 0..100 {
        let mut world_state = WorldState::new(5000, format!("request {}", i));
        let actions = create_actions();
        let goals = GoalState::primary_goal();

        let handle = tokio::spawn(async move {
            system.process_request(&mut world_state, actions, goals).await
        });
        handles.push(handle);
    }

    // Wait for all to complete
    let results = futures::future::join_all(handles).await;

    // Check success rate
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    let success_rate = success_count as f64 / results.len() as f64;

    assert!(success_rate >= 0.9, "Success rate should be >90%");
}
```

### Stress Testing

```rust
#[tokio::test]
async fn test_stress() {
    let system = GOAPSystem::new();

    // Gradually increase load
    for load in [10, 50, 100, 500, 1000] {
        let mut handles = vec![];

        for _ in 0..load {
            let mut world_state = WorldState::new(5000, "stress test".to_string());
            let actions = create_complex_actions();
            let goals = GoalState::primary_goal();

            let handle = tokio::spawn(async move {
                system.process_request(&mut world_state, actions, goals).await
            });
            handles.push(handle);
        }

        let start = std::time::Instant::now();
        let results = futures::future::join_all(handles).await;
        let duration = start.elapsed();

        let throughput = load as f64 / duration.as_secs_f64();
        println!("Load {}: {:.2} req/sec", load, throughput);

        // Verify performance doesn't degrade too much
        assert!(duration.as_secs() < load as u64 / 10, "Performance degraded");
    }
}
```

### Memory Leak Testing

```rust
#[tokio::test]
async fn test_memory_leak() {
    let system = GOAPSystem::new();
    let initial_memory = system.get_memory_usage();

    // Process 10000 requests
    for i in 0..10000 {
        let mut world_state = WorldState::new(5000, format!("request {}", i));
        let actions = create_actions();
        let goals = GoalState::primary_goal();

        let _ = system.process_request(&mut world_state, actions, goals).await;

        // Check memory every 1000 requests
        if i % 1000 == 0 {
            let current_memory = system.get_memory_usage();
            let growth = current_memory - initial_memory;
            assert!(growth < 100_000_000, "Memory growth too high"); // <100MB
        }
    }
}
```

## Benchmarking Guidelines

### 1. Use Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench planning

# Generate HTML report
cargo bench -- --output-format html
```

### 2. Track Performance Over Time

```rust
// Use benchmarking to track regressions
#[bench]
fn planning_performance(b: &mut Bencher) {
    b.iter(|| {
        let result = run_planning_benchmark();
        assert!(result.is_ok());
    })
}

// CI should fail if performance degrades >10%
```

### 3. Profile Real Workloads

```rust
// Collect real performance data
struct PerformanceCollector {
    samples: Vec<PerformanceMetrics>,
}

impl PerformanceCollector {
    fn record(&mut self, metrics: PerformanceMetrics) {
        self.samples.push(metrics);

        // Keep only recent samples
        if self.samples.len() > 1000 {
            self.samples.remove(0);
        }
    }

    fn average_planning_time(&self) -> f64 {
        self.samples.iter()
            .map(|m| m.planning_time_ms)
            .sum::<f64>() / self.samples.len() as f64
    }
}
```

## Performance Tuning Checklist

### Before Optimization
- [ ] Identify bottlenecks with profiling
- [ ] Set performance goals
- [ ] Establish baseline metrics

### During Optimization
- [ ] Change one parameter at a time
- [ ] Measure impact of each change
- [ ] Verify improvements with benchmarks

### After Optimization
- [ ] Run full test suite
- [ ] Verify no regressions
- [ ] Update documentation
- [ ] Monitor in production

## Common Benchmarks

### Planning Benchmarks
```rust
// benchmarks/planning_bench.rs
fn benchmark_simple_planning(c: &mut Criterion) { /* ... */ }
fn benchmark_complex_planning(c: &mut Criterion) { /* ... */ }
fn benchmark_heuristic_calculation(c: &mut Criterion) { /* ... */ }
```

### Cache Benchmarks
```rust
// benchmarks/cache_bench.rs
fn benchmark_pattern_lookup(c: &mut Criterion) { /* ... */ }
fn benchmark_cache_hit_rate(c: &mut Criterion) { /* ... */ }
fn benchmark_memory_usage(c: &mut Criterion) { /* ... */ }
```

### End-to-End Benchmarks
```rust
// benchmarks/e2e_bench.rs
fn benchmark_request_processing(c: &mut Criterion) { /* ... */ }
fn benchmark_throughput(c: &mut Criterion) { /* ... */ }
fn benchmark_latency(c: &mut Criterion) { /* ... */ }
```

## Next Steps

- Review [Configuration Guide](CONFIGURATION.md) for tuning parameters
- Study [Tutorial: Patterns](TUTORIAL_PATTERNS.md) for caching strategies
- Check [Error Handling Guide](ERROR_HANDLING.md) for error performance

## Reference

- [Performance Metrics API](../docs/api/metrics/index.html)
- [Benchmark Results](../benchmarks/report.html)
- [Profiling Guide](../docs/profiling.html)
