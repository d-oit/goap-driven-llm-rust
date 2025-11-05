# Error Handling Guide

Best practices for handling errors in the GOAP system.

## Error Types

GOAP uses a hierarchical error system:

```
Error (top-level)
├─ PlanningError
├─ ExecutionError
├─ CacheError
└─ ValidationError
```

## Error Definitions

### PlanningError

Errors related to plan generation:

```rust
#[derive(Error, Debug)]
pub enum PlanningError {
    #[error("No valid path found to goal")]
    NoPathFound,

    #[error("Token budget exceeded")]
    TokenBudgetExceeded,

    #[error("Planning timeout after {0}ms")]
    Timeout(u64),

    #[error("Invalid goal state: {0}")]
    InvalidGoal(String),

    #[error("Maximum plan depth exceeded: {0}")]
    MaxDepthExceeded(u32),

    #[error("No actions available")]
    NoActionsAvailable,

    #[error("Heuristic calculation failed: {0}")]
    HeuristicFailed(String),
}
```

**When**: Occurs during A* planning phase
**Impact**: Prevents plan generation
**Recovery**: Can try with different parameters or simplified goal

### ExecutionError

Errors during plan execution:

```rust
#[derive(Error, Debug)]
pub enum ExecutionError {
    #[error("Execution timeout")]
    Timeout,

    #[error("Action failed: {0}")]
    ActionFailed(String),

    #[error("Invalid action sequence")]
    InvalidSequence,

    #[error("World state error: {0}")]
    WorldState(String),

    #[error("Maximum retries exceeded")]
    MaxRetriesExceeded,

    #[error("Reactive replanning failed")]
    ReplanFailed,

    #[error("Goal not satisfied: {0:?}")]
    GoalNotSatisfied(Vec<String>),
}
```

**When**: Occurs during action execution
**Impact**: Single action or entire plan fails
**Recovery**: May trigger reactive replanning

### CacheError

Errors related to pattern caching:

```rust
#[derive(Error, Debug)]
pub enum CacheError {
    #[error("Pattern not found: {0}")]
    PatternNotFound(String),

    #[error("Schema not found: {0}")]
    SchemaNotFound(String),

    #[error("Cache full")]
    CacheFull,

    #[error("Cache eviction error")]
    EvictionError,

    #[error("Pattern validation failed")]
    PatternValidationFailed,

    #[error("Cache corruption detected")]
    CorruptionDetected,

    #[error("Serialization error: {0}")]
    Serialization(String),
}
```

**When**: Occurs during pattern cache operations
**Impact**: Pattern reuse unavailable, falls back to full planning
**Recovery**: Clear cache, restart system

### ValidationError

Errors during input/output validation:

```rust
#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Invalid request: {0}")]
    InvalidRequest(String),

    #[error("Invalid schema: {0}")]
    InvalidSchema(String),

    #[error("Schema validation failed: {0}")]
    SchemaValidationFailed(String),

    #[error("Response validation failed")]
    ResponseValidationFailed,

    #[error("Pattern confidence too low: {0}")]
    LowConfidence(f64),

    #[error("Request too long: {0} bytes")]
    RequestTooLong(usize),
}
```

**When**: Occurs during validation
**Impact**: Request rejected before processing
**Recovery**: Fix input, adjust validation rules

## Handling Errors

### Basic Error Handling

```rust
use goap_llm::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let system = GOAPSystem::new();

    match system.process_request(&mut world_state, actions, goals).await {
        Ok(result) => {
            println!("Success: {}", result.success);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            return Err(error.into());
        }
    }

    Ok(())
}
```

### Specific Error Handling

```rust
match system.process_request(&mut world_state, actions, goals).await {
    Ok(result) => {
        // Success
        handle_success(result)
    }
    Err(Error::Planning(PlanningError::NoPathFound)) => {
        // Planning failed - try simpler goal
        eprintln!("No path found, trying simplified goal");
        let simplified_goals = simplify_goals(goals);
        system.process_request(&mut world_state, actions, simplified_goals).await?;
    }
    Err(Error::Planning(PlanningError::TokenBudgetExceeded)) => {
        // Token budget exceeded - increase budget or optimize
        eprintln!("Budget exceeded, increasing budget");
        world_state.token_budget = 10000;
        system.process_request(&mut world_state, actions, goals).await?;
    }
    Err(Error::Execution(ExecutionError::Timeout)) => {
        // Execution timeout - try with shorter plan
        eprintln!("Timeout, using faster path");
        let faster_actions = optimize_for_speed(actions);
        system.process_request(&mut world_state, faster_actions, goals).await?;
    }
    Err(Error::Cache(CacheError::PatternNotFound(_))) => {
        // No pattern - full planning (expected in many cases)
        eprintln!("No pattern cache hit, using full planning");
    }
    Err(error) => {
        // Other errors - log and handle generically
        eprintln!("Unexpected error: {}", error);
        return Err(error.into());
    }
}
```

### Propagating Errors

```rust
fn process_with_retry(
    system: &GOAPSystem,
    world_state: &mut WorldState,
    actions: Vec<Action>,
    goals: GoalState,
    max_retries: u32,
) -> Result<ExecutionResult, Error> {
    let mut attempt = 0;

    loop {
        match system.process_request(world_state, actions.clone(), goals.clone()).await {
            Ok(result) => return Ok(result),
            Err(error) if attempt < max_retries => {
                attempt += 1;
                eprintln!("Attempt {} failed: {}", attempt, error);
                tokio::time::sleep(std::time::Duration::from_millis(100 * attempt)).await;
                continue;
            }
            Err(error) => return Err(error),
        }
    }
}
```

### Adding Context

```rust
use anyhow::{Context, Result};

fn load_and_process(request: &str) -> Result<Response> {
    let world_state = WorldState::new(5000, request.to_string())
        .context("Failed to create world state")?;

    let system = GOAPSystem::new()
        .context("Failed to initialize GOAP system")?;

    let actions = create_actions()
        .context("Failed to create actions")?;

    let goals = GoalState::primary_goal()
        .context("Failed to create goals")?;

    system.process_request(&mut world_state, actions, goals)
        .await
        .context("Failed to process request")?
        .context("Request processing returned error")
}
```

## Error Recovery Strategies

### 1. Retry with Backoff

```rust
async fn process_with_backoff(
    system: &GOAPSystem,
    world_state: &mut WorldState,
    actions: Vec<Action>,
    goals: GoalState,
) -> Result<ExecutionResult, Error> {
    let mut attempt = 0;
    let max_attempts = 3;

    loop {
        match system.process_request(world_state, actions.clone(), goals.clone()).await {
            Ok(result) => return Ok(result),
            Err(Error::Cache(CacheError::PatternNotFound(_))) => {
                // No pattern - try again (may find pattern on retry)
                attempt += 1;
                if attempt >= max_attempts {
                    return Err(Error::general("Max attempts reached"));
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
            Err(Error::Planning(_)) => {
                // Planning errors may succeed on retry
                attempt += 1;
                if attempt >= max_attempts {
                    return Err(Error::general("Max attempts reached"));
                }
                tokio::time::sleep(std::time::Duration::from_millis(200)).await;
            }
            Err(error) => {
                // Non-retryable error
                return Err(error);
            }
        }
    }
}
```

### 2. Fallback Strategies

```rust
async fn process_with_fallback(
    system: &GOAPSystem,
    world_state: &mut WorldState,
    mut actions: Vec<Action>,
    goals: GoalState,
) -> Result<ExecutionResult, Error> {
    // Try with pattern reuse
    goals.priority_level = 8;  // Prioritize pattern reuse

    match system.process_request(world_state, actions.clone(), goals.clone()).await {
        Ok(result) if result.success => return Ok(result),
        _ => {
            // Pattern reuse failed, try full generation
            actions = create_fallback_actions();
            goals = GoalState::primary_goal();  // Reset priority

            match system.process_request(world_state, actions, goals).await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    // All strategies failed
                    Err(Error::general(format!(
                        "All strategies failed: {}",
                        error
                    )))
                }
            }
        }
    }
}
```

### 3. Graceful Degradation

```rust
async fn process_with_degradation(
    system: &GOAPSystem,
    world_state: &mut WorldState,
    actions: Vec<Action>,
    goals: GoalState,
) -> Result<ExecutionResult, Error> {
    // Try optimal approach
    let result = system.process_request(world_state, actions.clone(), goals.clone()).await;

    match result {
        Ok(mut result) if result.success => Ok(result),
        _ => {
            // Degrade: reduce budget, use simpler actions
            eprintln!("Optimal approach failed, degrading gracefully");

            world_state.token_budget = world_state.token_budget / 2;
            let simpler_actions = actions
                .into_iter()
                .map(|mut a| {
                    a.estimated_cost = a.estimated_cost / 2;
                    a
                })
                .collect();

            let degraded_goals = GoalState {
                priority_level: 5,  // Lower priority
                ..goals
            };

            system.process_request(world_state, simpler_actions, degraded_goals).await
        }
    }
}
```

## Logging Errors

### Structured Logging

```rust
use tracing::{info, warn, error, debug};

async fn process_request_with_logging(
    system: &GOAPSystem,
    world_state: &mut WorldState,
    actions: Vec<Action>,
    goals: GoalState,
) -> Result<ExecutionResult, Error> {
    info!("Starting request processing";
        "request" => world_state.current_request,
        "budget" => world_state.token_budget,
        "actions" => actions.len(),
        "goals" => goals.goals.len(),
    );

    match system.process_request(world_state, actions, goals).await {
        Ok(result) => {
            info!("Request processed successfully";
                "success" => result.success,
                "steps" => result.steps_completed,
                "tokens_used" => result.total_tokens_used,
            );
            Ok(result)
        }
        Err(error) => {
            error!("Request processing failed";
                "error" => %error,
                "error_type" => format!("{:?}", error),
            );
            Err(error)
        }
    }
}
```

### Error Categories for Logging

```rust
#[derive(Debug)]
enum ErrorSeverity {
    Info,
    Warn,
    Error,
    Critical,
}

fn classify_error(error: &Error) -> ErrorSeverity {
    match error {
        Error::Validation(_) => ErrorSeverity::Warn,
        Error::Cache(_) => ErrorSeverity::Warn,
        Error::Planning(_) => ErrorSeverity::Error,
        Error::Execution(_) => ErrorSeverity::Critical,
        _ => ErrorSeverity::Error,
    }
}
```

## Custom Error Types

### Domain-Specific Errors

```rust
#[derive(Error, Debug)]
pub enum MyError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Authentication failed: {0}")]
    Auth(String),
}

// Implement conversion
impl From<goap_llm::Error> for MyError {
    fn from(error: goap_llm::Error) -> Self {
        MyError::Database(error.to_string())
    }
}
```

### Error with Context

```rust
use thiserror::Error;

#[derive(Error, Debug)]
#[error("Failed to process request {request_id}: {source}")]
pub struct RequestError {
    pub request_id: String,
    #[from]
    pub source: goap_llm::Error,
}

impl RequestError {
    pub fn new(request_id: String, error: goap_llm::Error) -> Self {
        RequestError {
            request_id,
            source: error,
        }
    }
}
```

## Best Practices

### 1. Use Specific Error Types
```rust
// Good: Specific error
if budget_exceeded {
    return Err(Error::Planning(PlanningError::TokenBudgetExceeded));
}

// Bad: Generic error
if budget_exceeded {
    return Err(Error::general("Budget problem"));
}
```

### 2. Provide Context
```rust
// Good: Descriptive error
return Err(Error::planning(format!(
    "Failed to find path from state with {} properties to goal requiring {}",
    current_count, goal_count
)));

// Bad: Unclear error
return Err(Error::planning("No path"));
```

### 3. Log at Appropriate Level
```rust
// Good: Match severity to impact
match error {
    Error::Validation(_) => warn!("Validation failed: {}", error),
    Error::Cache(_) => debug!("Cache miss: {}", error),
    Error::Execution(_) => error!("Execution failed: {}", error),
}

// Bad: Log everything as error
match error {
    Error::Validation(_) => error!("Validation failed: {}", error),
    Error::Cache(_) => error!("Cache miss: {}", error),  // Too severe!
}
```

### 4. Handle All Variants
```rust
// Good: Handle all error types
match error {
    Error::Planning(e) => handle_planning_error(e),
    Error::Execution(e) => handle_execution_error(e),
    Error::Cache(e) => handle_cache_error(e),
    Error::Validation(e) => handle_validation_error(e),
}

// Bad: Unhandled variants
match error {
    Error::Planning(e) => handle_planning_error(e),
    // Missing other variants!
}
```

### 5. Don't Panic
```rust
// Good: Return error
fn risky_operation() -> Result<String, Error> {
    if failed {
        return Err(Error::general("Operation failed"));
    }
    Ok("success".to_string())
}

// Bad: Panic
fn risky_operation() -> String {
    if failed {
        panic!("Operation failed");  // Never do this!
    }
    "success".to_string()
}
```

## Testing Errors

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_planning_error() {
        let error = Error::Planning(PlanningError::NoPathFound);
        assert!(matches!(error, Error::Planning(_)));
        assert_eq!(error.to_string(), "No valid path found to goal");
    }

    #[test]
    fn test_error_conversion() {
        let planning_error = PlanningError::Timeout(5000);
        let error: Error = planning_error.into();
        assert!(matches!(error, Error::Planning(_)));
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_error_handling() {
    let system = GOAPSystem::new();
    let mut world_state = WorldState::new(0, "test".to_string());  // Zero budget

    let actions = vec![Action::new(ActionType::GenerateResponse)];
    let goals = GoalState::primary_goal();

    let result = system.process_request(&mut world_state, actions, goals).await;

    assert!(result.is_err());
    if let Err(Error::Planning(PlanningError::TokenBudgetExceeded)) = result {
        // Expected error
    } else {
        panic!("Expected TokenBudgetExceeded error");
    }
}
```

### Property-Based Error Tests

```rust
#[cfg(test)]
mod prop_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_error_never_none(error in any::<Error>()) {
            // Errors should always be Some variant
            prop_assert!(!matches!(error, Error::General(_)) ||
                         !error.to_string().is_empty());
        }
    }
}
```

## Common Pitfalls

### Pitfall 1: Swallowing Errors
```rust
// Bad: Silently ignore
if let Err(error) = system.process_request(...) {
    // Do nothing!
}

// Good: Handle or propagate
if let Err(error) = system.process_request(...) {
    log::warn!("Request failed: {}", error);
    return Err(error);
}
```

### Pitfall 2: Using unwrap/expect
```rust
// Bad: Can panic
let result = system.process_request(...).unwrap();

// Good: Handle error
let result = system.process_request(...)?;
// or
match system.process_request(...) {
    Ok(result) => result,
    Err(error) => {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}
```

### Pitfall 3: Inconsistent Error Levels
```rust
// Bad: All errors are errors
Error::Cache(CacheError::PatternNotFound(_)) => {
    error!("Pattern not found");  // Too severe!
}

// Good: Appropriate level
Error::Cache(CacheError::PatternNotFound(_)) => {
    debug!("Pattern not found (expected on first request)");
}
```

## Troubleshooting

### "No error context"
- Use `anyhow::Context` to add context
- Include request ID or relevant IDs in error

### "Error handling too verbose"
- Create helper functions for common patterns
- Use `?` operator to simplify

### "Unclear error messages"
- Add specific error types
- Include relevant data in error message
- Use `#[error("...")]` attribute

## Next Steps

- Review [Tutorial: Reactive Replanning](TUTORIAL_REACTIVE.md) for recovery strategies
- Check [Configuration Guide](CONFIGURATION.md) for error handling configuration
- Study [Performance Guide](PERFORMANCE.md) for error handling performance

## Reference

- [Error Type API](../docs/api/error/index.html)
- [Recovery Strategies Documentation](../docs/recovery/)
