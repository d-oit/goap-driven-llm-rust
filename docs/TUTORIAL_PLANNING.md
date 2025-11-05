# Tutorial: GOAP Planning

This tutorial walks you through planning with GOAP step-by-step.

## Overview

GOAP (Goal-Oriented Action Planning) transforms reactive request processing into proactive strategic planning:

```
Request → Analyze → Plan → Execute → Learn
           ↓         ↓        ↓        ↓
        World    Action   Action   Pattern
        State    Graph    Sequence  Cache
```

## Step 1: Understanding World State

WorldState represents everything the system knows:

```rust
let mut world_state = WorldState::new(
    5000,  // Token budget
    "Create a deployment pipeline".to_string(),  // Request
);

// Check properties
assert!(world_state.tokens_available());
assert!(!world_state.has_property(&WorldProperty::ResponseGenerated));

// Set properties
world_state.set_property(WorldProperty::RequestValidated, true);
```

### WorldState Properties

| Property | Purpose | When Set |
|----------|---------|----------|
| `SchemaAvailable(id)` | Schema loaded | After detecting schema type |
| `PatternAvailable(id)` | Pattern cached | After pattern lookup |
| `RequestValidated` | Request checked | After validation |
| `ResponseGenerated` | Output created | After generation |
| `TokenBudgetRemaining(n)` | Tokens left | Continuously tracked |

## Step 2: Creating Actions

Actions define what can be done:

```rust
let action = Action::new(ActionType::GenerateResponse)
    .with_precondition(WorldProperty::RequestValidated)
    .with_effect(WorldProperty::ResponseGenerated)
    .with_cost(400)
    .with_duration(2000);
```

### Action Anatomy

```
┌─────────────────────┐
│   Action            │
├─────────────────────┤
│ Type: GenerateResp  │
│                     │
│ Preconditions:      │  Must be true
│  - RequestValid     │  before execution
│                     │
│ Effects:            │  Becomes true
│  - ResponseGen      │  after execution
│                     │
│ Cost: 400 tokens    │  Token estimate
│ Duration: 2000ms    │  Time estimate
└─────────────────────┘
```

### Common Action Patterns

#### Pattern 1: Information Gathering
```rust
Action::new(ActionType::DetectSchemaType)
    .with_effect(WorldProperty::SchemaAvailable("github-workflow".to_string()))
    .with_cost(50)
```

#### Pattern 2: Validation
```rust
Action::new(ActionType::PreValidateRequest)
    .with_effect(WorldProperty::RequestValidated)
    .with_cost(50)
```

#### Pattern 3: Generation
```rust
Action::new(ActionType::GenerateResponse)
    .with_precondition(WorldProperty::RequestValidated)
    .with_effect(WorldProperty::ResponseGenerated)
    .with_cost(400)
```

#### Pattern 4: Pattern Reuse
```rust
Action::new(ActionType::GenerateFromPattern)
    .with_precondition(WorldProperty::PatternAvailable("template".to_string()))
    .with_effect(WorldProperty::ResponseGenerated)
    .with_cost(150)  // Cheaper than full generation!
```

## Step 3: Defining Goals

Goals define what we want to achieve:

```rust
// Primary goal: Generate valid response
let goals = GoalState::primary_goal();

// Efficiency goal: Minimize tokens
let goals = GoalState::efficiency_focused();

// Pattern goal: Reuse cached patterns
let goals = GoalState::pattern_reuse_goal();
```

### Goal Priority Levels

| Priority | Use Case | Characteristics |
|----------|----------|----------------|
| 9-10 | Critical | Must achieve |
| 7-8 | High | Important for efficiency |
| 4-6 | Medium | Normal operations |
| 1-3 | Low | Nice-to-have |

## Step 4: Planning Process

The planner finds the optimal action sequence:

```
Initial State ──→ [Plan Generation] ──→ Action Sequence ──→ Execution
     ↓                   ↓                      ↓              ↓
  5000 tokens      A* Search             Optimized        Valid
  Empty state      Heuristic             Path             Response
                   Cost-based
```

### A* Search

GOAP uses A* search to find optimal plans:

```rust
// The planner evaluates:
// 1. Path cost (sum of action costs)
// 2. Heuristic estimate (remaining cost to goal)
// 3. Total: g(n) + h(n) - minimize total
```

### Heuristic Function

Heuristic estimates remaining cost:

```rust
fn heuristic(world_state: &WorldState, goal: &GoalState) -> f64 {
    let remaining_properties = goal.required_properties.len()
        - world_state.satisfied_properties();
    remaining_properties as f64 * 100.0  // Estimated cost
}
```

**Heuristic must be admissible**: Never overestimate actual cost.

## Step 5: Execution Flow

Actions execute sequentially:

```rust
for action in plan.actions {
    // 1. Check preconditions
    if !action.can_execute(&world_state) {
        // Trigger reactive replanning
        break;
    }

    // 2. Execute action
    execute_action(&action, &mut world_state).await?;

    // 3. Apply effects
    for effect in action.effects {
        world_state.set_property(effect, true);
    }

    // 4. Update metrics
    metrics.tokens_used += action.get_cost();
}
```

### Execution States

```
Planning → Ready → Executing → Complete
    ↓         ↓         ↓          ↓
  Generate  Precond   Apply       Record
  Sequence  Check     Effects     Metrics
```

## Step 6: Advanced Planning

### Multiple Goals

```rust
let mut goals = GoalState::primary_goal();

// Add additional goals
goals.goals.push(Goal::OptimizeTokenUsage);
goals.goals.push(Goal::ValidateOutput);

// Set priorities
goals.priority_level = 8;
```

### Parallel Action Execution

Some actions can run in parallel:

```rust
let actions = vec![
    // These can run in parallel
    Action::new(ActionType::DetectSchemaType),
    Action::new(ActionType::CheckPatternCache),
];

// Planner groups independent actions
// Executor runs them concurrently where possible
```

### Conditional Actions

Use conditional logic in action effects:

```rust
// Action that adapts based on state
if world_state.tokens_remaining() < 500 {
    // Low budget: use pattern
    action.with_precondition(WorldProperty::PatternAvailable("fast".to_string()))
} else {
    // High budget: full generation
    action.with_precondition(WorldProperty::RequestValidated)
}
```

## Step 7: Monitoring Planning

Track planning performance:

```rust
let metrics = system.get_metrics();

println!("Plans generated: {}", metrics.total_plans_generated);
println!("Success rate: {:.1}%", metrics.success_rate() * 100.0);
println!("Average plan depth: {:.1}", metrics.average_plan_depth);
```

## Best Practices

### 1. Keep Actions Atomic
```rust
// Good: One clear purpose
Action::new(ActionType::ValidateSyntax)

// Avoid: Multiple purposes
Action::new(ActionType::ValidateAndOptimizeAndGenerate)  // Too complex!
```

### 2. Set Realistic Costs
```rust
// Base costs (rough estimates):
DetectSchemaType:     50 tokens
CheckPatternCache:    30 tokens
GenerateFromPattern: 150 tokens
GenerateResponse:    400 tokens
```

### 3. Define Clear Effects
```rust
// Good: Specific effect
.with_effect(WorldProperty::ResponseGenerated)

// Avoid: Vague effect
.with_effect(WorldProperty::SomethingHappened)
```

### 4. Use Proper Preconditions
```rust
// Good: Enforce order
GenerateResponse
    .with_precondition(RequestValidated)

// Avoid: No dependencies
GenerateResponse
    // Can run anytime - may cause errors!
```

## Common Patterns

### Pipeline Pattern
```
DetectSchema → ValidateRequest → GenerateResponse → PostValidate
      ↓              ↓                ↓              ↓
  Available      Validated        Generated      Complete
```

### Parallel Pattern
```
DetectSchema ────┐
                 ├─→ Merge → GenerateResponse
CheckPattern ────┘
```

### Retry Pattern
```
Attempt1 ──[fail]──→ Replan ──[fail]──→ Replan ──[success]──→ Done
   ↓                    ↓              ↓              ↓
Try direct         Try pattern     Try template    Succeed
```

## Troubleshooting

### No Path Found
```
Error: No valid path found to goal

Solutions:
1. Check action preconditions can be satisfied
2. Add alternative action paths
3. Verify world state can reach goal
4. Increase plan depth limit
```

### Suboptimal Plans
```
Problem: Plan uses 600 tokens, but 200-token alternative exists

Solutions:
1. Adjust action costs to reflect reality
2. Improve heuristic function
3. Add more action options
4. Enable pattern reuse
```

### Execution Failures
```
Problem: Action fails during execution

Solutions:
1. Add reactive replanning
2. Improve precondition checking
3. Add retry logic
4. Handle errors gracefully
```

## Next Steps

- Learn [Pattern Reuse](TUTORIAL_PATTERNS.md) for efficiency
- Explore [Reactive Planning](TUTORIAL_REACTIVE.md) for robustness
- Review [Configuration Guide](CONFIGURATION.md) for tuning
- Check [Performance Tips](PERFORMANCE.md) for optimization

## Example

See `examples/basic_planning.rs` for a complete working example.
