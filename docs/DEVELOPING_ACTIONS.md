# Developing Custom Actions

Learn how to create domain-specific actions for your GOAP application.

## Overview

Actions are the fundamental building blocks of GOAP. This guide shows how to create custom actions tailored to your domain.

## Action Anatomy

Every action has three key components:

```rust
pub struct Action {
    pub action_type: ActionType,           // WHAT to do
    pub preconditions: HashSet<WorldProperty>,  // WHEN it can execute
    pub effects: HashSet<WorldProperty>,        // WHAT changes after
    pub estimated_cost: u32,               // HOW EXPENSIVE (tokens)
    pub estimated_duration_ms: u64,        // HOW LONG (time)
}
```

## Step-by-Step Guide

### Step 1: Define Action Type

Add to the `ActionType` enum:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    // ... existing actions ...

    // Add your custom action
    MyCustomAction(String),  // Parameterized for flexibility
}
```

**Best Practices**:
- Use parameterized actions for flexibility (e.g., `MyAction(String)`)
- Avoid over-specific actions (e.g., don't create `GenerateNodeJSWorkflow`)
- Group related actions with same prefix for organization

### Step 2: Implement Action Name (Optional)

```rust
impl ActionType {
    pub fn name(&self) -> &'static str {
        match self {
            // ... existing matches ...

            ActionType::MyCustomAction(param) => {
                match param.as_str() {
                    "option1" => "Custom Action Option 1",
                    "option2" => "Custom Action Option 2",
                    _ => "Custom Action",
                }
            }
        }
    }
}
```

### Step 3: Create Action Builder

```rust
pub struct MyCustomActionBuilder {
    preconditions: Vec<WorldProperty>,
    effects: Vec<WorldProperty>,
    cost: u32,
    duration: u64,
    parameter: String,
}

impl MyCustomActionBuilder {
    pub fn new(param: String) -> Self {
        MyCustomActionBuilder {
            preconditions: vec![],
            effects: vec![],
            cost: 100,  // Default cost
            duration: 1000,  // Default duration (ms)
            parameter: param,
        }
    }

    pub fn with_precondition(mut self, property: WorldProperty) -> Self {
        self.preconditions.push(property);
        self
    }

    pub fn with_effect(mut self, property: WorldProperty) -> Self {
        self.effects.push(property);
        self
    }

    pub fn with_cost(mut self, cost: u32) -> Self {
        self.cost = cost;
        self
    }

    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.duration = duration_ms;
        self
    }

    pub fn build(self) -> Action {
        Action::new(ActionType::MyCustomAction(self.parameter))
            .with_preconditions(self.preconditions)
            .with_effects(self.effects)
            .with_cost(self.cost)
            .with_duration(self.duration)
    }
}
```

### Step 4: Add Helper Methods to Action (Optional)

```rust
impl Action {
    // Add these methods to Action impl

    pub fn with_preconditions(mut self, preconditions: Vec<WorldProperty>) -> Self {
        self.preconditions.extend(preconditions);
        self
    }

    pub fn with_effects(mut self, effects: Vec<WorldProperty>) -> Self {
        self.effects.extend(effects);
        self
    }
}
```

### Step 5: Create Convenience Constructor

```rust
impl ActionType {
    /// Create a new custom action with default settings
    pub fn my_custom_action(param: String) -> Action {
        Action::new(ActionType::MyCustomAction(param))
            .with_cost(100)
            .with_duration(1000)
    }
}
```

## Example: Domain-Specific Actions

### Example 1: Database Action

```rust
// Action to execute database query
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DatabaseActionType {
    ConnectToDatabase(String),      // Connect to specific DB
    ExecuteQuery(String),           // Execute SQL query
    ValidateSchema(String),         // Validate DB schema
    BackupDatabase(String),         // Backup database
}

// Builder
impl DatabaseActionType {
    pub fn execute_query(query: String) -> Action {
        Action::new(DatabaseActionType::ExecuteQuery(query))
            .with_precondition(WorldProperty::DatabaseConnected("primary".to_string()))
            .with_effect(WorldProperty::QueryExecuted)
            .with_cost(200)  // Tokens for LLM to understand/execute
            .with_duration(2000)
    }
}

// World properties
impl WorldProperty {
    pub fn database_connected(db_name: String) -> Self {
        WorldProperty::DatabaseAvailable(db_name)
    }
}
```

### Example 2: API Action

```rust
// Action to interact with external APIs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ApiActionType {
    FetchFromApi(String),      // GET request
    PostToApi(String),         // POST request
    ValidateApiKey(String),    // Validate API credentials
    RateLimitCheck(String),    // Check rate limits
}

impl ApiActionType {
    pub fn fetch_from_api(endpoint: String) -> Action {
        Action::new(ApiActionType::FetchFromApi(endpoint))
            .with_precondition(WorldProperty::ApiKeyValid)
            .with_precondition(WorldProperty::RateLimitOk)
            .with_effect(WorldProperty::DataFetched)
            .with_cost(150)
            .with_duration(1500)
    }
}
```

### Example 3: ML Model Action

```rust
// Action to run ML inference
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MlActionType {
    LoadModel(String),         // Load ML model
    RunInference(String),      // Run inference
    ValidateModel(String),     // Validate model output
    FineTuneModel(String),     // Fine-tune model
}

impl MlActionType {
    pub fn run_inference(model_name: String, input_data: String) -> Action {
        Action::new(MlActionType::RunInference(model_name))
            .with_precondition(WorldProperty::ModelLoaded(model_name))
            .with_effect(WorldProperty::InferenceCompleted)
            .with_effect(WorldProperty::OutputGenerated)
            .with_cost(300)  // Higher cost for ML operations
            .with_duration(5000)
    }
}
```

## Advanced Patterns

### Pattern 1: Conditional Actions

```rust
// Action that adapts based on context
pub struct ConditionalAction {
    if_branch: Action,
    else_branch: Action,
}

impl ConditionalAction {
    pub fn new(condition: WorldProperty) -> Self {
        ConditionalAction {
            if_branch: Action::new(ActionType::GenerateResponse)
                .with_precondition(condition.clone())
                .with_effect(WorldProperty::ResponseGenerated),

            else_branch: Action::new(ActionType::GenerateFromTemplate)
                .with_precondition(!condition)
                .with_effect(WorldProperty::ResponseGenerated),
        }
    }
}
```

### Pattern 2: Chained Actions

```rust
// Action that executes a sequence
pub struct ActionChain {
    actions: Vec<Action>,
}

impl ActionChain {
    pub fn new() -> Self {
        ActionChain { actions: vec![] }
    }

    pub fn add_action(mut self, action: Action) -> Self {
        self.actions.push(action);
        self
    }

    pub fn build(self) -> Action {
        // Combine all preconditions
        let all_preconditions: HashSet<_> = self.actions
            .iter()
            .flat_map(|a| a.preconditions.clone())
            .collect();

        // Combine all effects
        let all_effects: HashSet<_> = self.actions
            .iter()
            .flat_map(|a| a.effects.clone())
            .collect();

        // Sum costs and durations
        let total_cost = self.actions.iter().map(|a| a.get_cost()).sum();
        let total_duration = self.actions.iter().map(|a| a.get_duration()).sum();

        Action::new(ActionType::ActionChain)
            .with_preconditions(all_preconditions.into_iter().collect())
            .with_effects(all_effects.into_iter().collect())
            .with_cost(total_cost)
            .with_duration(total_duration)
    }
}
```

### Pattern 3: Retryable Actions

```rust
// Action with built-in retry logic
pub struct RetryableAction {
    action: Action,
    max_retries: u32,
    backoff_ms: u64,
}

impl RetryableAction {
    pub fn new(action: Action, max_retries: u32) -> Self {
        RetryableAction {
            action,
            max_retries,
            backoff_ms: 100,  // Start with 100ms backoff
        }
    }

    pub async fn execute(&self, world_state: &mut WorldState) -> Result<ActionResult> {
        let mut attempt = 0;

        loop {
            match self.action.execute(world_state).await {
                Ok(result) => return Ok(result),
                Err(e) if attempt < self.max_retries => {
                    attempt += 1;
                    tokio::time::sleep(
                        std::time::Duration::from_millis(
                            self.backoff_ms * (2_u64.pow(attempt))
                        )
                    ).await;
                    continue;
                }
                Err(e) => return Err(e),
            }
        }
    }
}
```

## Testing Custom Actions

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_action_creation() {
        let action = Action::new(ActionType::MyCustomAction("test".to_string()));

        assert_eq!(
            action.action_type,
            ActionType::MyCustomAction("test".to_string())
        );
        assert_eq!(action.get_cost(), 100);  // Default cost
    }

    #[test]
    fn test_custom_action_with_preconditions() {
        let action = Action::new(ActionType::MyCustomAction("test".to_string()))
            .with_precondition(WorldProperty::RequestValidated);

        assert!(action.preconditions.contains(&WorldProperty::RequestValidated));
    }

    #[test]
    fn test_custom_action_can_execute() {
        let mut world_state = WorldState::new(5000, "test".to_string());
        world_state.set_property(WorldProperty::RequestValidated, true);

        let action = Action::new(ActionType::MyCustomAction("test".to_string()))
            .with_precondition(WorldProperty::RequestValidated);

        assert!(action.can_execute(&world_state));
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
        fn test_action_cost_is_positive(cost in 1u32..10000) {
            let action = Action::new(ActionType::MyCustomAction("test".to_string()))
                .with_cost(cost);

            prop_assert!(action.get_cost() > 0);
        }

        #[test]
        fn test_action_duration_is_positive(duration in 1u64..60000) {
            let action = Action::new(ActionType::MyCustomAction("test".to_string()))
                .with_duration(duration);

            prop_assert!(action.get_duration() > 0);
        }
    }
}
```

## Action Design Guidelines

### 1. Single Responsibility
```rust
// Good: One clear purpose
Action::new(ActionType::ValidateSchema)

// Bad: Multiple purposes
Action::new(ActionType::ValidateSchemaAndOptimizeAndGenerate)
```

### 2. Clear Preconditions
```rust
// Good: Specific requirement
.with_precondition(WorldProperty::SchemaAvailable("github-workflow".to_string()))

// Bad: Vague requirement
.with_precondition(WorldProperty::SomethingReady)
```

### 3. Meaningful Effects
```rust
// Good: Specific result
.with_effect(WorldProperty::ResponseGenerated)

// Bad: Vague result
.with_effect(WorldProperty::WorkDone)
```

### 4. Realistic Costs
```rust
// Good: Based on actual token usage
.with_cost(400)  // Actual LLM call costs ~400 tokens

// Bad: Random numbers
.with_cost(12345)  // Unrealistic
```

### 5. Appropriate Duration
```rust
// Good: Based on actual execution time
.with_duration(2000)  // 2 seconds typical for this action

// Bad: Too fast or too slow
.with_duration(1)  // Unrealistic for complex action
```

## Common Action Patterns

### Validation Pattern
```rust
Action::new(ActionType::ValidateSomething)
    .with_effect(WorldProperty::SomethingValid)
    .with_cost(50)  // Cheap validation
    .with_duration(500)  // Fast validation
```

### Generation Pattern
```rust
Action::new(ActionType::GenerateSomething)
    .with_precondition(WorldProperty::SomethingValid)
    .with_effect(WorldProperty::SomethingGenerated)
    .with_cost(400)  // Expensive generation
    .with_duration(3000)  // Slower generation
```

### Cache Pattern
```rust
Action::new(ActionType::GenerateFromCache)
    .with_precondition(WorldProperty::CacheAvailable)
    .with_effect(WorldProperty::SomethingGenerated)
    .with_cost(100)  // Cheap from cache
    .with_duration(500)  // Fast from cache
```

### Recovery Pattern
```rust
Action::new(ActionType::RecoverFromError)
    .with_precondition(WorldProperty::ErrorOccurred)
    .with_effect(WorldProperty::ErrorResolved)
    .with_cost(200)  // Moderate cost
    .with_duration(1500)  // Medium recovery time
```

## Integrating Custom Actions

### 1. Add to ActionType Enum

```rust
// In src/goap/actions/action.rs
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    // ... existing variants ...
    MyCustomAction(String),
}
```

### 2. Add Name Implementation

```rust
impl ActionType {
    pub fn name(&self) -> &'static str {
        match self {
            // ... existing matches ...

            ActionType::MyCustomAction(_) => "My Custom Action",
        }
    }
}
```

### 3. Update Examples

```rust
// Add to examples/custom_actions.rs
let custom_action = Action::new(ActionType::MyCustomAction("param".to_string()))
    .with_precondition(WorldProperty::RequestValidated)
    .with_effect(WorldProperty::ResponseGenerated)
    .with_cost(300);
```

## Best Practices Checklist

- [ ] Action has single responsibility
- [ ] Preconditions are specific and checkable
- [ ] Effects are meaningful and observable
- [ ] Cost estimates are realistic (based on actual usage)
- [ ] Duration estimates are accurate
- [ ] Action includes unit tests
- [ ] Action includes property-based tests
- [ ] Documentation explains purpose and usage
- [ ] Examples demonstrate the action

## Troubleshooting

### "Action never executes"
- Check preconditions are set in world state
- Verify precondition logic in `can_execute()`
- Ensure actions are added to available actions list

### "Unexpected state changes"
- Review effects are correctly applied
- Check for conflicting actions
- Verify order of action execution

### "Slow planning"
- Reduce number of actions
- Simplify preconditions/effects
- Increase planning timeout

### "High token usage"
- Review action cost estimates
- Enable pattern reuse
- Add caching for expensive actions

## Next Steps

- Review [Tutorial: Planning](TUTORIAL_PLANNING.md) for planning integration
- Study [Tutorial: Patterns](TUTORIAL_PATTERNS.md) for caching strategies
- Check [Error Handling Guide](ERROR_HANDLING.md) for failure management

## Reference

- [Action API Documentation](../docs/api/action/index.html)
- [WorldState API](../docs/api/world_state/index.html)
- [Examples Repository](../examples/custom_actions.rs)
