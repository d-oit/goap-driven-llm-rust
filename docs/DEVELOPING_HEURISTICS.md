# Developing Custom Heuristics

Learn how to create effective heuristic functions for A* planning.

## Overview

The heuristic guides A* search toward the goal efficiently. A good heuristic is critical for performance.

## Heuristic Fundamentals

### What is a Heuristic?

A heuristic estimates the remaining cost from a state to the goal:

```rust
fn heuristic(world_state: &WorldState, goal: &GoalState) -> f64 {
    // Estimate: "How many more tokens will we need?"
    estimate_remaining_cost(world_state, goal)
}
```

### Properties of Good Heuristics

#### 1. Admissibility (Must Have)
**Never overestimate**: `h(n) ≤ actual_cost(n, goal)`

```rust
// Good: Underestimates
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    let remaining_props = goal.required_properties.len();
    remaining_props as f64 * 50.0  // 50 tokens per property (conservative)
}

// Bad: Overestimates
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    let remaining_props = goal.required_properties.len();
    remaining_props as f64 * 10000.0  // 10k tokens = unrealistic!
}
```

#### 2. Consistency (Monotonic)
**Triangle inequality**: `h(n) ≤ cost(n, n') + h(n')`

If moving from A to B costs 100 tokens, then:
- Heuristic(A) ≤ 100 + Heuristic(B)

```rust
// Consistent heuristic example
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // This is consistent because it's based on minimum costs
    (goal.required_properties.len() as f64) * MIN_ACTION_COST
}
```

#### 3. Informative (Higher is Better)
**More precise estimates** = fewer nodes explored

```rust
// Less informative (explores more nodes)
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    0.0  // Always 0 = Dijkstra's algorithm
}

// More informative (explores fewer nodes)
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // Uses actual state information
    calculate_detailed_estimate(state, goal)
}
```

## Creating Custom Heuristics

### Step 1: Implement Heuristic Trait

```rust
use goap_llm::planning::Heuristic;

pub struct MyCustomHeuristic {
    // Configuration for the heuristic
    token_cost_per_property: f64,
    success_probability_weight: f64,
}

impl MyCustomHeuristic {
    pub fn new() -> Self {
        MyCustomHeuristic {
            token_cost_per_property: 100.0,
            success_probability_weight: 0.2,
        }
    }
}

impl Heuristic for MyCustomHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Calculate estimate
        self.calculate_estimate(state, goal)
    }
}
```

### Step 2: Implement Estimation Logic

```rust
impl MyCustomHeuristic {
    fn calculate_estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Step 1: Count unsatisfied properties
        let unsatisfied = self.count_unsatisfied_properties(state, goal);

        // Step 2: Estimate cost per property
        let base_cost = unsatisfied as f64 * self.token_cost_per_property;

        // Step 3: Adjust for success probability
        let success_prob = self.estimate_success_probability(state, goal);
        let adjusted_cost = base_cost / success_prob.max(0.1);  // Avoid division by zero

        // Step 4: Ensure admissibility (never overestimate)
        adjusted_cost.min(base_cost * 2.0)
    }

    fn count_unsatisfied_properties(&self, state: &WorldState, goal: &GoalState) -> usize {
        goal.required_properties
            .iter()
            .filter(|prop| !state.has_property(prop))
            .count()
    }

    fn estimate_success_probability(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Based on historical data, cache hits, etc.
        0.85  // 85% success rate (example)
    }
}
```

### Step 3: Add Configuration

```rust
pub struct HeuristicConfig {
    pub token_cost_weight: f64,      // Weight for token cost
    pub time_cost_weight: f64,       // Weight for time cost
    pub success_weight: f64,         // Weight for success probability
    pub max_estimate: f64,           // Cap estimates
    pub cache_enabled: bool,         // Enable caching
}

impl Default for HeuristicConfig {
    fn default() -> Self {
        HeuristicConfig {
            token_cost_weight: 0.5,
            time_cost_weight: 0.3,
            success_weight: 0.2,
            max_estimate: 10000.0,
            cache_enabled: true,
        }
    }
}
```

### Step 4: Implement Caching (Optional)

```rust
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct CachedHeuristic<H: Heuristic> {
    inner: H,
    cache: HashMap<u64, f64>,
    cache_hits: u64,
    cache_misses: u64,
}

impl<H: Heuristic> CachedHeuristic<H> {
    pub fn new(inner: H) -> Self {
        CachedHeuristic {
            inner,
            cache: HashMap::new(),
            cache_hits: 0,
            cache_misses: 0,
        }
    }

    fn calculate_hash(&self, state: &WorldState, goal: &GoalState) -> u64 {
        let mut hasher = DefaultHasher::new();
        state.hash(&mut hasher);
        goal.hash(&mut hasher);
        hasher.finish()
    }
}

impl<H: Heuristic> Heuristic for CachedHeuristic<H> {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let hash = self.calculate_hash(state, goal);

        if let Some(&cached_value) = self.cache.get(&hash) {
            self.cache_hits += 1;
            return cached_value;
        }

        self.cache_misses += 1;
        let estimate = self.inner.estimate(state, goal);

        if self.cache.len() < 1000 {  // Limit cache size
            self.cache.insert(hash, estimate);
        }

        estimate
    }
}
```

## Heuristic Design Patterns

### Pattern 1: Weighted Sum

Combine multiple factors:

```rust
pub struct WeightedHeuristic {
    weights: Vec<(String, f64)>,  // (factor_name, weight)
}

impl WeightedHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let mut total = 0.0;

        // Token cost component
        let token_component = self.estimate_token_cost(state, goal);
        total += token_component * self.get_weight("token");

        // Time cost component
        let time_component = self.estimate_time_cost(state, goal);
        total += time_component * self.get_weight("time");

        // Success probability component
        let success_component = self.estimate_success_cost(state, goal);
        total += success_component * self.get_weight("success");

        total
    }
}
```

### Pattern 2: Pattern-Based

Use cached patterns to estimate:

```rust
pub struct PatternBasedHeuristic {
    pattern_cache: Arc<IntelligentCache>,
}

impl PatternBasedHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Find similar patterns
        if let Some(pattern) = self.pattern_cache.find_similar(state.current_request) {
            // Use pattern's historical cost as heuristic
            pattern.avg_tokens as f64 * 0.8  // Conservative (82% of average)
        } else {
            // Fallback to default heuristic
            self.default_estimate(state, goal)
        }
    }
}
```

### Pattern 3: Domain-Specific

Tailored to specific domains:

```rust
pub struct CodeGenerationHeuristic {
    language: String,
    complexity_estimator: ComplexityEstimator,
}

impl CodeGenerationHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let request = &state.current_request;

        // Estimate based on code complexity
        let complexity = self.complexity_estimator.estimate(request);

        // Base cost per line of code
        let cost_per_line = self.get_cost_per_line(&self.language);

        // Estimate total tokens
        let estimated_lines = complexity.estimated_lines();
        let base_cost = estimated_lines as f64 * cost_per_line;

        // Adjust for schema availability
        let schema_bonus = if state.has_schema() { 0.8 } else { 1.2 };

        base_cost * schema_bonus
    }
}
```

### Pattern 4: Adaptive

Learn from execution history:

```rust
pub struct AdaptiveHeuristic {
    base_heuristic: Box<dyn Heuristic>,
    learning_rate: f64,
    error_history: Vec<f64>,  // (actual - estimated) differences
}

impl AdaptiveHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let base_estimate = self.base_heuristic.estimate(state, goal);

        // Apply learned adjustment
        let adjustment = self.calculate_adjustment();

        let adjusted_estimate = base_estimate * (1.0 + adjustment);

        // Ensure admissibility
        adjusted_estimate.min(base_estimate * 1.5)
    }

    fn update(&mut self, state: &WorldState, goal: &GoalState, actual_cost: f64) {
        let estimated = self.estimate(state, goal);
        let error = actual_cost - estimated;
        self.error_history.push(error);

        // Keep only recent errors
        if self.error_history.len() > 100 {
            self.error_history.remove(0);
        }
    }

    fn calculate_adjustment(&self) -> f64 {
        if self.error_history.is_empty() {
            return 0.0;
        }

        // Simple moving average of errors
        let avg_error: f64 = self.error_history.iter().sum::<f64>()
            / self.error_history.len() as f64;

        // Adjust based on learning rate
        avg_error * self.learning_rate
    }
}
```

## Example Heuristics

### Heuristic 1: Token-Cost Based

```rust
pub struct TokenCostHeuristic {
    min_cost_per_action: f64,
    avg_cost_per_action: f64,
}

impl TokenCostHeuristic {
    pub fn new() -> Self {
        TokenCostHeuristic {
            min_cost_per_action: 50.0,   // Minimum cost per action
            avg_cost_per_action: 200.0,  // Average cost per action
        }
    }
}

impl Heuristic for TokenCostHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let unsatisfied = self.count_unsatisfied(state, goal);

        // Conservative estimate: minimum cost
        let min_estimate = unsatisfied as f64 * self.min_cost_per_action;

        // Optimistic estimate: average cost
        let avg_estimate = unsatisfied as f64 * self.avg_cost_per_action;

        // Choose: use conservative for admissibility
        min_estimate
    }
}
```

### Heuristic 2: Pattern-Aware

```rust
pub struct PatternAwareHeuristic {
    cache: Arc<IntelligentCache>,
}

impl PatternAwareHeuristic {
    pub fn new(cache: Arc<IntelligentCache>) -> Self {
        PatternAwareHeuristic { cache }
    }
}

impl Heuristic for PatternAwareHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Check for cached patterns
        if let Some(pattern) = self.cache.find_best_pattern(&state.current_request) {
            // Use pattern cost, adjusted for confidence
            let pattern_cost = pattern.avg_tokens as f64;
            let confidence_factor = pattern.confidence as f64 / 100.0;

            // Lower confidence = more conservative estimate
            pattern_cost * (2.0 - confidence_factor)
        } else {
            // No pattern: use default
            self.default_estimate(state, goal)
        }
    }
}
```

### Heuristic 3: Multi-Objective

```rust
pub struct MultiObjectiveHeuristic {
    token_weight: f64,
    time_weight: f64,
    success_weight: f64,
}

impl MultiObjectiveHeuristic {
    pub fn new(token_weight: f64, time_weight: f64, success_weight: f64) -> Self {
        // Ensure weights sum to 1.0
        let total = token_weight + time_weight + success_weight;
        MultiObjectiveHeuristic {
            token_weight: token_weight / total,
            time_weight: time_weight / total,
            success_weight: success_weight / total,
        }
    }
}

impl Heuristic for MultiObjectiveHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let unsatisfied = self.count_unsatisfied(state, goal);

        // Token component
        let token_cost = unsatisfied as f64 * 200.0;

        // Time component
        let time_cost = unsatisfied as f64 * 1000.0;  // ms

        // Success component
        let success_rate = self.estimate_success_rate(state, goal);
        let success_cost = if success_rate > 0.0 {
            token_cost / success_rate
        } else {
            token_cost * 2.0  // Penalize low success
        };

        // Weighted sum
        (token_cost * self.token_weight)
            + (time_cost * self.time_weight)
            + (success_cost * self.success_weight)
    }
}
```

## Testing Heuristics

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heuristic_is_admissible() {
        let heuristic = TokenCostHeuristic::new();
        let state = create_test_state();
        let goal = create_test_goal();

        let estimate = heuristic.estimate(&state, &goal);
        let actual_cost = execute_plan_and_measure(&state, &goal);

        // Heuristic should not overestimate
        assert!(estimate <= actual_cost,
            "Heuristic {} > Actual {}", estimate, actual_cost);
    }

    #[test]
    fn test_heuristic_consistency() {
        let heuristic = TokenCostHeuristic::new();
        let state_a = create_state_a();
        let state_b = create_state_b();
        let goal = create_test_goal();

        let cost_a_to_b = calculate_actual_cost(&state_a, &state_b);
        let h_a = heuristic.estimate(&state_a, &goal);
        let h_b = heuristic.estimate(&state_b, &goal);

        // h(a) ≤ cost(a,b) + h(b)
        assert!(h_a <= cost_a_to_b + h_b);
    }

    #[test]
    fn test_heuristic_more_informative() {
        let simple_heuristic = SimpleHeuristic::new();
        let detailed_heuristic = DetailedHeuristic::new();

        let state = create_complex_state();
        let goal = create_complex_goal();

        let simple_estimate = simple_heuristic.estimate(&state, &goal);
        let detailed_estimate = detailed_heuristic.estimate(&state, &goal);

        // Detailed heuristic should be closer to actual cost
        assert!(detailed_estimate > simple_estimate);
    }
}
```

### Property-Based Tests

```rust
#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_heuristic_never_negative(
            state_props in prop::collection::vec(any::<String>(), 0..10),
            goal_props in prop::collection::vec(any::<String>(), 0..10),
        ) {
            let state = create_state_with_props(state_props);
            let goal = create_goal_with_props(goal_props);
            let heuristic = TokenCostHeuristic::new();

            let estimate = heuristic.estimate(&state, &goal);

            prop_assert!(estimate >= 0.0,
                "Heuristic should never be negative");
        }

        #[test]
        fn test_heuristic_increases_with_unsatisfied(
            satisfied_count in 0usize..10,
            total_count in 0usize..20,
        ) {
            prop_assume!(total_count >= satisfied_count);

            let state = create_state_with_count(satisfied_count);
            let goal = create_goal_with_count(total_count);
            let heuristic = TokenCostHeuristic::new();

            let estimate = heuristic.estimate(&state, &goal);

            // More unsatisfied properties = higher estimate
            let unsatisfied = total_count - satisfied_count;
            prop_assert!(estimate >= unsatisfied as f64 * 50.0);
        }
    }
}
```

### Benchmark Tests

```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, criterion_group, criterion_main, Criterion};

    fn benchmark_heuristic(c: &mut Criterion) {
        let heuristic = TokenCostHeuristic::new();
        let state = create_random_state(1000);
        let goal = create_random_goal(50);

        c.bench_function("heuristic_estimate", |b| {
            b.iter(|| {
                let _estimate = heuristic.estimate(
                    black_box(&state),
                    black_box(&goal)
                );
            })
        });
    }
}
```

## Heuristic Best Practices

### 1. Ensure Admissibility
```rust
// Good: Conservative estimate
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    goal.required_properties.len() as f64 * MIN_ACTION_COST
}

// Bad: Overestimate
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    goal.required_properties.len() as f64 * MAX_ACTION_COST
}
```

### 2. Use Domain Knowledge
```rust
// Good: Uses domain-specific info
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    if state.has_schema() {
        // Faster with schema
        estimate_with_schema(goal)
    } else {
        // Slower without schema
        estimate_without_schema(goal)
    }
}
```

### 3. Cache Expensive Computations
```rust
// Good: Cache results
use std::collections::HashMap;

struct CachedHeuristic {
    cache: HashMap<StateHash, f64>,
}

// Bad: Recalculate every time
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // Expensive calculation every time
    calculate_complex_estimate(state, goal)
}
```

### 4. Balance Speed vs Accuracy
```rust
// Good: Adjustable accuracy
pub struct ConfigurableHeuristic {
    accuracy_level: u8,  // 1-10, higher = more accurate, slower
}

impl Heuristic for ConfigurableHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        match self.accuracy_level {
            1..=3 => self.fast_estimate(state, goal),      // Quick, rough
            4..=7 => self.medium_estimate(state, goal),    // Balanced
            8..=10 => self.detailed_estimate(state, goal), // Slow, precise
        }
    }
}
```

### 5. Monitor Performance
```rust
impl Heuristic for MyHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        let start = std::time::Instant::now();

        let estimate = self.calculate(state, goal);

        let duration = start.elapsed();
        if duration.as_millis() > 10 {
            warn!("Slow heuristic: {}ms", duration.as_millis());
        }

        estimate
    }
}
```

## Common Heuristic Functions

### 1. Manhattan Distance (Grid-Based)

```rust
fn manhattan_distance(current: (i32, i32), goal: (i32, i32)) -> f64 {
    ((goal.0 - current.0).abs() + (goal.1 - current.1).abs()) as f64
}
```

### 2. Euclidean Distance (Continuous)

```rust
fn euclidean_distance(current: (f64, f64), goal: (f64, f64)) -> f64 {
    let dx = goal.0 - current.0;
    let dy = goal.1 - current.1;
    (dx * dx + dy * dy).sqrt()
}
```

### 3. Pattern Match Score

```rust
fn pattern_match_score(request: &str, patterns: &[SuccessPattern]) -> f64 {
    let mut best_score = 0.0;

    for pattern in patterns {
        let similarity = calculate_similarity(request, &pattern.request_hash);
        best_score = best_score.max(similarity);
    }

    best_score
}
```

### 4. Token Budget Remaining

```rust
fn budget_remaining_estimate(state: &WorldState, goal: &GoalState) -> f64 {
    let remaining_budget = state.tokens_remaining();
    let estimated_needed = goal.required_properties.len() as f64 * 100.0;

    remaining_budget.max(estimated_needed)
}
```

## Troubleshooting

### Slow Planning
```rust
// Problem: Heuristic too slow
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // Complex calculation taking 100ms
    complex_calculation(state, goal)
}

// Solution: Cache or simplify
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // Check cache first
    if let Some(cached) = cache.get(&(state.hash(), goal.hash())) {
        return cached;
    }

    let estimate = simple_calculation(state, goal);
    cache.insert((state.hash(), goal.hash()), estimate);
    estimate
}
```

### Poor Performance
```rust
// Problem: Heuristic not informative enough
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    0.0  // Always 0 = Dijkstra
}

// Solution: More informative
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    // Use actual state information
    calculate_detailed_estimate(state, goal)
}
```

### Not Admissible
```rust
// Problem: Overestimates
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    goal.required_properties.len() as f64 * 10000.0  // Way too high!
}

// Solution: Conservative estimate
fn heuristic(state: &WorldState, goal: &GoalState) -> f64 {
    goal.required_properties.len() as f64 * 50.0  // Realistic minimum
}
```

## Next Steps

- Review [Tutorial: Planning](TUTORIAL_PLANNING.md) for planning integration
- Study [Performance Guide](PERFORMANCE.md) for optimization strategies
- Check [Configuration Guide](CONFIGURATION.md) for tuning heuristics

## Reference

- [Heuristic Trait API](../docs/api/heuristic/index.html)
- [A* Algorithm Documentation](../docs/algorithms/astar.html)
- [Example Heuristics](../examples/custom_heuristics.rs)
