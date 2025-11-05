# Data Model: GOAP-Driven LLM

**Date**: 2025-11-03
**Feature**: GOAP-Driven LLM Strategic Reasoning
**Phase**: 1 - Design & Contracts

## Entity Definitions

### 1. WorldProperty (Enum)

Represents discrete properties of the world state that actions can check or modify.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorldProperty {
    // Schema tracking
    SchemaAvailable(String),           // Schema type identifier
    SchemaCached(String),             // Schema loaded in cache
    SchemaValidated(String),          // Schema validation passed

    // Pattern caching
    PatternAvailable(String),         // Pattern ID in cache
    PatternConfidence(String, u8),    // Pattern ID + confidence score (0-100)

    // Token management
    TokenBudgetRemaining(u32),        // Tokens left in budget
    TokenCompressionApplied(u8),      // Compression ratio (0-100%)

    // Request lifecycle
    RequestOptimized,                 // Request optimized
    RequestValidated,                 // Request syntax checked
    ResponseGenerated,                // LLM response created
    ResponseValidated,                // Response verified against schema

    // Learning
    PatternLearned(String),           // New pattern added
    MetricsUpdated,                   // Performance metrics recorded

    // Error states
    ValidationFailed(String),         // Error type that occurred
    RequiresHumanIntervention,        // Manual intervention needed
}
```

**Validation Rules**:
- SchemaAvailable and SchemaCached must use same schema type string
- PatternConfidence confidence must be 0-100
- TokenBudgetRemaining cannot be negative
- TokenCompressionApplied must be 0-100

### 2. WorldState

Tracks all system state during planning and execution.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorldState {
    pub properties: HashMap<WorldProperty, bool>,
    pub schema_cache: HashMap<String, Arc<Schema>>,
    pub pattern_cache: HashMap<String, SuccessPattern>,
    pub token_budget: u32,
    pub current_request: String,
}

impl WorldState {
    pub fn new(initial_budget: u32, request: String) -> Self

    pub fn has_property(&self, prop: &WorldProperty) -> bool

    pub fn set_property(&mut self, prop: WorldProperty, value: bool)

    pub fn get_available_schemas(&self) -> Vec<String>

    pub fn get_available_patterns(&self) -> Vec<(String, u8)>

    pub fn tokens_remaining(&self) -> u32

    pub fn tokens_available(&self) -> bool {
        self.token_budget > 100
    }
}
```

**State Invariants**:
- token_budget >= 0 always
- All properties in schema_cache must be marked as SchemaCached
- All properties in pattern_cache must be marked as PatternAvailable

**Transitions**:
- RequestOptimized → RequestValidated (via PreValidateRequest)
- RequestValidated → ResponseGenerated (via GenerateResponse/GenerateFromPattern)
- ResponseGenerated → ResponseValidated (via PostValidateResponse)

### 3. ActionType (Enum)

Defines all possible actions the system can perform.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    // Information gathering
    DetectSchemaType,
    FetchSchema(String),
    CheckPatternCache,

    // Optimization
    CompressRequest,
    OptimizeTokenUsage,

    // Validation
    PreValidateRequest,
    PostValidateResponse,
    QuickValidatePattern,

    // Generation
    GenerateResponse,
    GenerateFromPattern,
    GenerateFromTemplate,

    // Learning
    LearnSuccessPattern,
    UpdateMetrics,
    AdaptOptimizationRules,

    // Error handling
    FixValidationErrors,
    RequestClarification,
    Replan,
}
```

**Action Taxonomy**:
- **Information Gathering**: Collect required data before planning
- **Optimization**: Reduce token usage or improve efficiency
- **Validation**: Ensure request/response quality
- **Generation**: Create the actual LLM response
- **Learning**: Extract patterns and update metrics
- **Error Handling**: Recover from failures

### 4. Action

Defines a single operation with requirements and effects.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub preconditions: Vec<WorldProperty>,
    pub effects: Vec<WorldProperty>,
    pub token_cost: u32,
    pub confidence: u8,
    pub duration_ms: u32,
    pub retry_count: u32,
}

impl Action {
    pub fn new(action_type: ActionType) -> Self

    pub fn can_execute(&self, state: &WorldState) -> bool {
        self.preconditions.iter().all(|prop| state.has_property(prop))
    }

    pub fn estimate_cost(&self) -> u32 {
        self.token_cost + (self.duration_ms / 100)
    }
}
```

**Constraints**:
- confidence: 0-100 (percentage likelihood of success)
- duration_ms: > 0 for all actions
- retry_count: typically 0-3
- Preconditions must be satisfied before execution
- Effects applied atomically on success

### 5. Goal (Enum)

Represents objectives the system aims to achieve.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Goal {
    // Primary goals
    GenerateValidResponse,
    OptimizeTokenUsage,
    MaximizeConfidence,

    // Sub-goals
    EnsureSchemaAvailable,
    ReuseSuccessfulPattern,
    MinimizeTokenCost,
    ValidateOutput,
    LearnFromSuccess,

    // Complex goals
    BestEffort,
}
```

**Goal Relationships**:
- BestEffort decomposes to: EnsureSchemaAvailable → ReuseSuccessfulPattern → GenerateValidResponse → LearnFromSuccess
- GenerateValidResponse requires: ResponseGenerated + ResponseValidated
- OptimizeTokenUsage requires: TokenCompressionApplied OR PatternAvailable

### 6. GoalState

Container for goals with metadata.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoalState {
    pub goals: Vec<Goal>,
    pub required_properties: Vec<WorldProperty>,
    pub priority_level: u8,
    pub timeout_ms: u32,
}

impl GoalState {
    pub fn primary_goal() -> Self

    pub fn efficiency_focused() -> Self

    pub fn pattern_reuse_goal() -> Self

    pub fn is_satisfied(&self, state: &WorldState) -> bool {
        self.required_properties.iter().all(|prop| state.has_property(prop))
    }
}
```

**Priority Levels**:
- 1-3: Low priority (nice-to-have)
- 4-6: Medium priority (normal operations)
- 7-8: High priority (important for efficiency)
- 9-10: Critical priority (must achieve)

### 7. SuccessPattern

Cached representation of a successful execution.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SuccessPattern {
    pub id: String,
    pub request_hash: Vec<u8>,          // LSH signature for similarity
    pub action_sequence: Vec<ActionType>,
    pub confidence: u8,                 // 0-100
    pub success_rate: f64,              // 0.0-1.0
    pub avg_tokens: u32,
    pub avg_duration_ms: u32,
    pub usage_count: u32,
    pub created_at: Instant,
    pub last_used: Instant,
    pub schema_type: Option<String>,
}
```

**Learning Algorithm**:
- confidence = weighted_average(similarity, success_rate, usage_frequency)
- similarity: LSH-based Jaccard similarity
- success_rate: Historical success / total attempts
- usage_frequency: log(usage_count) / age_weeks

**Cache Management**:
- LRU eviction when cache size exceeds limit
- Confidence threshold for activation (>70%)
- Stale pattern cleanup (unused > 30 days)

### 8. ExecutionResult

Records the outcome of plan execution.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionResult {
    pub success: bool,
    pub steps_completed: usize,
    pub total_steps: usize,
    pub total_tokens_used: u32,
    pub execution_steps: Vec<ExecutionStep>,
    pub goals_satisfied: Vec<Goal>,
    pub error_message: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExecutionStep {
    pub action: ActionType,
    pub status: StepStatus,
    pub duration_ms: u32,
    pub tokens_used: u32,
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum StepStatus {
    Success,
    RecoveredError,
    Failed,
}
```

**Metrics Derived**:
- Success rate: successful_executions / total_executions
- Average tokens: sum(tokens_used) / successful_executions
- Recovery rate: recovered_errors / total_errors
- Goal achievement rate: goals_satisfied / goals_attempted

### 9. Schema

Represents validation schemas for different request types.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Schema {
    pub id: String,
    pub name: String,
    pub schema_type: String,           // "github-workflow", "docker-compose", etc.
    pub definition: Value,             // JSON Schema definition
    pub validator: Option<Validator>,  // Compiled validator
    pub created_at: Instant,
}

impl Schema {
    pub fn validate(&self, data: &Value) -> Result<(), ValidationError>

    pub fn from_json(name: String, definition: Value) -> Result<Self, SchemaError>
}
```

**Schema Sources**:
- Local cache (LRU)
- Remote fetch (schemastore.org)
- User-provided schemas
- Auto-generated from examples

### 10. GOAPMetrics

Performance and effectiveness metrics.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GOAPMetrics {
    pub total_plans_generated: u64,
    pub successful_plans: u64,
    pub failed_plans: u64,
    pub replans_triggered: u64,

    pub average_plan_depth: f64,
    pub average_plan_cost: f64,
    pub average_execution_time_ms: f64,

    pub goal_success_rates: HashMap<Goal, f64>,
    pub action_success_rates: HashMap<ActionType, f64>,

    pub tokens_saved_by_planning: u64,
    pub cache_hit_rate: f64,
}
```

**Metric Collection**:
- Incremented during plan execution
- Aggregated per time window (hourly, daily)
- Used for heuristic tuning
- Exposed via metrics API

## Validation Rules

### Cross-Entity Validation

1. **Action → WorldState**:
   - All preconditions must be checkable (property defined)
   - Effects must be meaningful (not contradictory)
   - Cost estimates must be realistic

2. **Pattern → SuccessPattern**:
   - confidence <= 100
   - usage_count > 0 implies success_rate > 0
   - avg_tokens > 0 for non-zero usage_count

3. **Goal → GoalState**:
   - required_properties must be achievable via action effects
   - timeout_ms must be sufficient for action sequence
   - priority_level 1-10

### State Transitions

**Valid Transitions**:
```
Initial → [DetectSchemaType] → [FetchSchema] → [CheckPatternCache] →
[PreValidateRequest] → [GenerateResponse/GenerateFromPattern] →
[PostValidateRequest] → [LearnSuccessPattern] → Success

Initial → [DetectSchemaType] → [FetchSchema] → [CheckPatternCache] →
[PreValidateRequest] → [GenerateResponse] → [ValidationFailed] →
[FixValidationErrors] → [Replan] → Success
```

**Invalid Transitions** (Return Error):
- GenerateResponse without RequestValidated
- PostValidateResponse without ResponseGenerated
- LearnSuccessPattern without ResponseValidated

### Constraints

1. **Token Budget**:
   - Initial budget >= 200 (minimum for basic response)
   - Running total never negative
   - Compression reduces budget consumption

2. **Concurrency**:
   - PatternCache: Read-heavy, lock-free via Dashmap
   - SchemaCache: Write-rare, exclusive lock via Mutex
   - WorldState: Per-request, no sharing

3. **Performance**:
   - Plan depth <= 20 actions
   - Heuristic admissibility: h(n) <= actual_cost(n, goal)
   - Cache size bounded (configurable, default 10,000 patterns)

## Serialization

All entities support JSON serialization via serde:

```rust
// Example serialization
let world_state = WorldState::new(10000, request.clone());
let json = serde_json::to_string(&world_state)?;
let deserialized: WorldState = serde_json::from_str(&json)?;
```

**Format Examples**:

```json
{
  "properties": {
    "SchemaAvailable(\"github-workflow\")": true,
    "RequestValidated": true,
    "TokenBudgetRemaining(8500)": true
  },
  "token_budget": 8500,
  "current_request": "Create GitHub Actions workflow for Node.js"
}
```

```json
{
  "id": "pattern_abc123",
  "action_sequence": ["DetectSchemaType", "CheckPatternCache", "GenerateFromPattern"],
  "confidence": 85,
  "success_rate": 0.92,
  "avg_tokens": 150,
  "usage_count": 45
}
```

## Error Types

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Planning error: {0}")]
    Planning(#[from] PlanningError),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),

    #[error("Cache error: {0}")]
    Cache(#[from] CacheError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

---

## Next Steps

Proceed to API contract definition in `contracts/` directory with:
1. OpenAPI specification for public API
2. JSON schemas for validation
3. Error response formats
4. Integration examples
