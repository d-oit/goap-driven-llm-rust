# Tutorial: Reactive Replanning

Learn how GOAP handles failures and automatically finds alternative paths.

## Overview

Reactive replanning enables robust execution through automatic recovery:

```
Plan Execution ──[Failure]──→ Detect ──[Replan]──→ New Plan ──[Success]
      ↓                 ↓           ↓           ↓           ↓
  Action 3          Failure     Alternative  Execute      Done
  Fails             Found       Path Found    Path
```

## Why Reactive Replanning?

Real-world systems encounter:
- **Validation errors**: Schema mismatches, invalid inputs
- **Token budget exceeded**: Ran out of tokens mid-execution
- **Schema fetch failures**: Can't retrieve required schemas
- **Pattern mismatches**: Cached patterns don't fit new request
- **Timeouts**: Actions take too long to complete

GOAP handles these gracefully through automatic replanning.

## How It Works

### 1. Failure Detection
Monitor execution for problems:

```rust
fn execute_action(&self, action: &Action, state: &mut WorldState) -> Result<()> {
    let result = action.execute().await?;

    if !result.success {
        // Failure detected!
        self.metrics.increment_failures();
        state.set_property(WorldProperty::ValidationFailed(result.error), true);

        // Trigger replanning check
        if self.should_replan(state) {
            self.trigger_replanning(state)?;
        }
    }

    Ok(())
}
```

### 2. Replanning Triggers

GOAP replans when:

```rust
fn should_replan(&self, state: &WorldState) -> bool {
    // Token budget critical
    state.tokens_remaining() < 100 ||
    // Validation failed
    state.has_property(&WorldProperty::ValidationFailed("".to_string())) ||
    // Execution timeout
    self.execution_time > MAX_TIMEOUT ||
    // Schema fetch failed
    state.has_property(&WorldProperty::SchemaNotFound("".to_string()))
}
```

### 3. Alternative Path Discovery
Find new action sequences:

```rust
fn replan(&self, state: &WorldState, failed_actions: Vec<Action>) -> Plan {
    // Remove failed actions
    let available_actions = self.actions
        .iter()
        .filter(|a| !failed_actions.contains(a))
        .cloned()
        .collect();

    // Find alternative paths
    self.planner.find_alternative_plan(
        state,
        available_actions,
        self.goals
    )
}
```

### 4. Bounded Retry Logic
Limit replanning attempts:

```rust
struct RetryState {
    attempts: u32,
    max_attempts: 3,  // Maximum 3 replans
    backoff_ms: u64,
}

impl RetryState {
    fn should_retry(&self) -> bool {
        self.attempts < self.max_attempts
    }

    fn next_backoff(&self) -> u64 {
        self.backoff_ms * (2_u64.pow(self.attempts))
    }
}
```

## Step-by-Step Guide

### Step 1: Enable Reactive Planning

```rust
let system = GOAPSystem::new();

// Reactive planning is enabled by default
// Configure if needed
let config = GOAPConfig {
    max_replans: 3,
    replan_threshold: ReplanThreshold::TokenBudgetCritical,
    ..Default::default()
};
```

### Step 2: Simulate Failure Scenario

```rust
let mut world_state = WorldState::new(5000, "Complex request".to_string());

// Simulate validation failure
world_state.set_property(
    WorldProperty::ValidationFailed("Schema mismatch".to_string()),
    true
);

let actions = vec![
    Action::new(ActionType::GenerateResponse)  // Will fail
        .with_precondition(WorldProperty::RequestValidated),
    Action::new(ActionType::GenerateFromTemplate)  // Alternative
        .with_precondition(WorldProperty::SchemaAvailable("template".to_string())),
];
```

### Step 3: Execute with Failure Handling

```rust
let result = system.process_request(&mut world_state, actions, goals).await?;

println!("Execution completed");
println!("  Steps attempted: {}", result.total_steps);
println!("  Steps completed: {}", result.steps_completed);
println!("  Replans triggered: {}", result.replans_triggered);
```

### Step 4: Examine Recovery Path

```rust
for step in &result.execution_steps {
    match step.status {
        StepStatus::Success => {
            println!("✓ {}: Success", step.action.name());
        }
        StepStatus::RecoveredError => {
            println!("⚡ {}: Recovered from error", step.action.name());
            println!("   Original error: {}", step.error);
        }
        StepStatus::Failed => {
            println!("✗ {}: Failed", step.action.name());
        }
    }
}
```

## Common Failure Scenarios

### Scenario 1: Token Budget Exceeded

```rust
// Start with tight budget
let mut world_state = WorldState::new(200, "Large request".to_string());

// Action 1 succeeds
// Action 2 fails (out of tokens)

// GOAP detects: tokens_remaining < 100
// Trigger: Replanning
// Alternative: Use pattern or compress request
```

### Scenario 2: Validation Failure

```rust
let mut world_state = WorldState::new(5000, "Request".to_string());

// Set up invalid request
world_state.set_property(
    WorldProperty::ValidationFailed("Invalid schema".to_string()),
    true
);

// Primary path: GenerateResponse (requires Validated)
// → Fails
// → Alternative: FixValidationErrors → GenerateResponse
```

### Scenario 3: Schema Fetch Failure

```rust
// Try to fetch schema
let action = Action::new(ActionType::FetchSchema)
    .with_effect(WorldProperty::SchemaAvailable("remote".to_string()));

// Network error → SchemaNotFound
// → Alternative: Use cached schema
// → Or: Use template instead
```

### Scenario 4: Pattern Mismatch

```rust
// Try to use cached pattern
let action = Action::new(ActionType::GenerateFromPattern)
    .with_precondition(WorldProperty::PatternAvailable("pattern".to_string()));

// Pattern doesn't match well enough
// → Alternative: Full generation
// → Or: Learn new pattern
```

## Replanning Strategies

### Strategy 1: Skip Failed Action

```rust
// Original plan: [A, B, C, D]
// B fails
// New plan: [A, C, D]  // Skip B
```

### Strategy 2: Alternative Action

```rust
// Original plan: [A, GenerateResponse, C]
// GenerateResponse fails
// New plan: [A, GenerateFromTemplate, C]  // Alternative
```

### Strategy 3: Recovery Action

```rust
// Original plan: [A, B, C]
// B fails
// New plan: [A, FixB, B, C]  // Recover then retry
```

### Strategy 4: Complete Replan

```rust
// Original plan: [A, B, C]
// Multiple failures
// New plan: [D, E, F]  // Completely different approach
```

## Configuring Reactive Behavior

### Set Max Replans

```rust
let config = GOAPConfig {
    max_replans: 3,  // Allow up to 3 replans
    ..Default::default()
};
```

### Adjust Replan Triggers

```rust
enum ReplanThreshold {
    Never,                    // Don't replan
    OnCriticalFailure,        // Only on critical errors
    OnAnyFailure,             // Replan on any failure
    Predictive,               // Replan before failure (predictive)
}

let config = ReplanConfig {
    threshold: ReplanThreshold::OnCriticalFailure,
    token_critical_threshold: 100,  // Trigger at 100 tokens
    timeout_threshold_ms: 5000,     // Trigger at 5 second timeout
};
```

### Custom Recovery Actions

```rust
let custom_recovery = vec![
    Action::new(ActionType::RequestClarification),
    Action::new(ActionType::ReduceComplexity),
    Action::new(ActionType::UseFallback),
];
```

## Measuring Recovery Effectiveness

### Recovery Rate

```rust
let total_failures = 10;
let successful_recoveries = 8;

let recovery_rate = (successful_recoveries as f64 / total_failures as f64) * 100.0;
println!("Recovery rate: {:.1}%", recovery_rate);
// Target: 82%+ (SC-005)
```

### Replanning Overhead

```rust
let initial_plan_time = 200;  // ms
let replan_time = 300;        // ms
let overhead = replan_time - initial_plan_time;

println!("Replanning overhead: {}ms", overhead);
```

### Success After Replan

```rust
let total_replans = 10;
let successes_after_replan = 7;

let success_rate = (successes_after_replan as f64 / total_replans as f64) * 100.0;
println!("Success after replan: {:.1}%", success_rate);
```

## Advanced Reactive Techniques

### Predictive Replanning

Replan before failure occurs:

```rust
fn predict_failure(&self, plan: &Plan, state: &WorldState) -> bool {
    // Predict if remaining tokens will be exceeded
    let remaining_cost = plan.estimated_cost();
    let buffer = state.tokens_remaining() * 0.2;  // 20% buffer

    remaining_cost > (state.tokens_remaining() - buffer)
}
```

### Learning from Failures

```rust
// Record failure pattern
system.learn_failure_pattern(
    failed_action,
    failure_reason,
    successful_alternative
);

// Update heuristic to avoid similar failures
system.adjust_heuristic(failure_pattern);
```

### Parallel Recovery

Attempt multiple recovery strategies:

```rust
// Try alternatives in parallel
let recovery_strategies = vec![
    attempt_pattern_reuse(),
    attempt_compression(),
    attempt_alternative_schema(),
];

// Wait for first to succeed
let successful_recovery = futures::future::select_all(recovery_strategies).await;
```

## Error Types and Handling

### Transient Errors (Retry)

```rust
// Temporary failures - can retry
TransientError::NetworkTimeout => retry_with_backoff(),
TransientError::SchemaCacheMiss => fetch_and_retry(),
```

### Persistent Errors (Replan)

```rust
// Persistent failures - need new plan
PersistentError::InvalidRequest => replan_with_validation(),
PersistentError::ResourceExhausted => replan_with_constraints(),
```

### Fatal Errors (Fail)

```rust
// Can't recover
FatalError::SecurityViolation => fail_immediately(),
FatalError::CorruptedState => restart_system(),
```

## Best Practices

### 1. Set Realistic Expectations
```rust
// Not all failures can be recovered
let recovery_rate = 0.80;  // 82% is excellent
assert!(recovery_rate < 1.0, "100% recovery is unrealistic");
```

### 2. Limit Replanning Attempts
```rust
// Prevent infinite loops
max_replans = 3;
assert!(max_replans > 0, "Need at least 1 retry");
```

### 3. Track Recovery Patterns
```rust
// Learn from recoveries
system.record_recovery(
    original_failure,
    recovery_action,
    success_outcome
);
```

### 4. Balance Speed vs Robustness
```rust
// Fast: Fail fast on errors
if should_fail_fast() {
    return Err(error);
}

// Robust: Replan on errors
if should_replan() {
    return replan();
}
```

## Common Pitfalls

### Pitfall 1: Infinite Replanning Loop
```rust
// Bad: No max attempts
loop {
    if failed { replan(); }
}

// Good: Bounded retries
for attempt in 0..MAX_ATTEMPTS {
    if failed && attempt < MAX_ATTEMPTS {
        replan();
    }
}
```

### Pitfall 2: Always Replanning
```rust
// Bad: Replan on every minor failure
if minor_error { replan(); }  // Too aggressive!

// Good: Only replan on critical failures
if critical_error { replan(); }
```

### Pitfall 3: Not Tracking Recovery
```rust
// Bad: Don't learn from failures
let _ = execute_action(action);  // Ignore failures

// Good: Learn and improve
let result = execute_action(action)?;
system.learn_from_failure(&result);
```

## Troubleshooting

### "Too many replans"
- Check max_replans configuration
- Review failure root causes
- Add more robust action definitions

### "Replanning makes it worse"
- Verify alternative paths are valid
- Check heuristic accuracy
- Monitor success rate after replan

### "Slow recovery"
- Enable parallel recovery strategies
- Optimize alternative path discovery
- Use predictive replanning

## Next Steps

- Review [Token Optimization](TUTORIAL_PATTERNS.md) for budget management
- Study [Performance Tuning](PERFORMANCE.md) for optimization
- Check [Error Handling Guide](ERROR_HANDLING.md) for best practices

## Example

See `examples/reactive_replanning.rs` for a complete working example with recovery metrics.
