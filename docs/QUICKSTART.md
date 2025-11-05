# Quickstart Guide

Welcome to GOAP-driven LLM! This guide will get you up and running in 5 minutes.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
goap-llm = "0.1.0"
```

## Your First GOAP Request

```rust
use goap_llm::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Create world state
    let mut world_state = WorldState::new(
        5000,  // Token budget
        "Create a GitHub Actions workflow".to_string(),
    );

    // 2. Define actions
    let actions = vec![
        Action::new(ActionType::DetectSchemaType)
            .with_effect(WorldProperty::SchemaAvailable("github-workflow".to_string()))
            .with_cost(50),
        Action::new(ActionType::GenerateResponse)
            .with_precondition(WorldProperty::RequestValidated)
            .with_effect(WorldProperty::ResponseGenerated)
            .with_cost(400),
    ];

    // 3. Set goals
    let goals = GoalState::primary_goal();

    // 4. Process request
    let system = GOAPSystem::new();
    let result = system.process_request(&mut world_state, actions, goals).await?;

    // 5. Check results
    println!("Success: {}", result.success);
    println!("Tokens used: {}", result.total_tokens_used);

    Ok(())
}
```

Run it:
```bash
cargo run --example basic_planning
```

## Common Use Cases

### 1. Enable Pattern Reuse

```rust
// Process first request (creates pattern)
let mut ws1 = WorldState::new(5000, "Request A".to_string());
let _ = system.process_request(&mut ws1, actions.clone(), goals.clone()).await?;

// Process similar request (reuses pattern)
let mut ws2 = WorldState::new(5000, "Similar request A".to_string());
let result = system.process_request(&mut ws2, actions.clone(), goals.clone()).await?;

// See efficiency gains
println!("Tokens saved: {}", 5000 - ws2.tokens_remaining());
```

### 2. Handle Failures with Reactive Replanning

```rust
let mut world_state = WorldState::new(5000, request);

// Simulate failure condition
world_state.set_property(
    WorldProperty::ValidationFailed("Error".to_string()),
    true
);

let result = system.process_request(&mut world_state, actions, goals).await?;

// System automatically tries alternative paths
println!("Replans triggered: {}", result.steps_completed);
```

### 3. Manage Token Budget

```rust
// Tight budget scenario
let mut world_state = WorldState::new(1000, "Complex request".to_string());

// System will automatically:
// - Use patterns when available
// - Apply compression if needed
// - Optimize action selection

let result = system.process_request(&mut world_state, actions, goals).await?;

println!("Budget respected: {}", world_state.tokens_remaining() >= 0);
```

## Key Concepts

### WorldState
Tracks the current state:
- Token budget
- Available schemas
- Cached patterns
- Satisfied properties

### Actions
Define what can be done:
- **Preconditions**: What must be true before execution
- **Effects**: What becomes true after execution
- **Cost**: Token cost estimate

### Goals
Define what to achieve:
- Primary goal: Generate valid response
- Efficiency goal: Minimize token usage
- Pattern reuse goal: Leverage cached patterns

### GOAPSystem
Orchestrates everything:
- Plans action sequences
- Executes plans
- Handles failures
- Learns patterns

## Configuration

### Adjust Token Budget
```rust
let world_state = WorldState::new(
    10000,  // Increase budget
    request,
);
```

### Set Optimization Level
```rust
let goals = match optimization_level {
    "speed" => GoalState::efficiency_focused(),
    "quality" => GoalState::primary_goal(),
    "balanced" => GoalState::pattern_reuse_goal(),
};
```

## Best Practices

### 1. Start with Simple Actions
```rust
// Good: Clear preconditions and effects
Action::new(ActionType::GenerateResponse)
    .with_precondition(WorldProperty::RequestValidated)
    .with_effect(WorldProperty::ResponseGenerated)
    .with_cost(300)

// Avoid: Unclear dependencies
Action::new(ActionType::GenerateResponse)
    // Missing preconditions!
```

### 2. Set Realistic Token Budgets
- Simple requests: 1000-2000 tokens
- Complex requests: 5000-10000 tokens
- Include buffer: Add 20% to estimate

### 3. Monitor Performance
```rust
let metrics = system.get_metrics();
println!("Success rate: {:.1}%", metrics.success_rate() * 100.0);
println!("Cache hit rate: {:.1}%", metrics.cache_hit_rate * 100.0);
```

### 4. Handle Errors Gracefully
```rust
match system.process_request(&mut ws, actions, goals).await {
    Ok(result) => {
        if result.success {
            println!("Request succeeded!");
        } else {
            println!("Partial success: {}/{} steps completed",
                result.steps_completed,
                result.total_steps
            );
        }
    }
    Err(e) => {
        eprintln!("Error: {}", e);
        // Implement fallback logic
    }
}
```

## Examples

See the `examples/` directory for complete implementations:

- `basic_planning.rs` - Minimal setup
- `pattern_reuse.rs` - Efficiency optimization
- `reactive_replanning.rs` - Failure handling
- `token_optimization.rs` - Budget management
- `metrics_collection.rs` - Performance monitoring

## Troubleshooting

### "No valid path found"
- Check action preconditions are achievable
- Verify world state can satisfy requirements
- Add alternative action paths

### "Token budget exceeded"
- Increase initial budget
- Enable pattern reuse
- Use compression actions

### Low success rate
- Review action definitions
- Check precondition/effect logic
- Monitor metrics for insights

## Next Steps

1. ✅ Run your first example
2. 📚 Read [Tutorial: Planning](TUTORIAL_PLANNING.md)
3. 🚀 Explore [Pattern Reuse](TUTORIAL_PATTERNS.md)
4. 🔧 Review [Configuration](CONFIGURATION.md)
5. 📊 Check [Performance Guide](PERFORMANCE.md)

## Need Help?

- 📖 See [Architecture Overview](ARCHITECTURE.md)
- 🐛 Check [Error Handling Guide](ERROR_HANDLING.md)
- 💬 Review examples in `examples/`
- 📊 Monitor with [Metrics Collection example](../examples/metrics_collection.rs)
