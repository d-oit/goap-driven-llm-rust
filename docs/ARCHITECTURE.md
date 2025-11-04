# Architecture Overview

Understand the GOAP system architecture, component interactions, and design decisions.

## System Overview

GOAP (Goal-Oriented Action Planning) transforms reactive LLM processing into proactive strategic planning:

```
┌─────────────────────────────────────────────────────────────────────┐
│                          GOAP System                                │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Request → ┌─────────┐ → Plan → ┌──────────┐ → Execute → Result    │
│            │ Planner │         │ Executor │                       │
│            │  (A*)   │         │          │                       │
│            └─────────┘         └──────────┘                       │
│                ↓                   ↓                                │
│            ┌─────────┐         ┌──────────┐                       │
│            │ Cache   │         │ Metrics  │                       │
│            │ Pattern │         │ Monitor  │                       │
│            └─────────┘         └──────────┘                       │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Architecture

### 1. Core Components

#### GOAPSystem (Orchestrator)
**Purpose**: Main entry point and coordinator
**Responsibilities**:
- Process requests end-to-end
- Coordinate between planner, executor, and cache
- Manage system configuration
- Collect metrics

```rust
pub struct GOAPSystem {
    planner: GOAPPlanner,
    executor: PlanExecutor,
    cache: IntelligentCache,
    metrics: GOAPMetrics,
    config: GOAPConfig,
}
```

**Interactions**:
```
GOAPSystem
  ├─→ GOAPPlanner (planning)
  ├─→ PlanExecutor (execution)
  ├─→ IntelligentCache (patterns)
  └─→ GOAPMetrics (monitoring)
```

#### GOAPPlanner (A* Search)
**Purpose**: Find optimal action sequences
**Algorithm**: A* with weighted heuristic

```rust
pub struct GOAPPlanner {
    actions: Vec<Action>,
    heuristic: Box<dyn Heuristic>,
}

pub struct PlanNode {
    world_state: WorldState,
    action_sequence: Vec<Action>,
    g_cost: f64,  // Actual cost
    h_cost: f64,  // Heuristic estimate
    f_cost: f64,  // Total: g + h
}
```

**Planning Flow**:
```
Initial State ──[Expand]──→ Neighbors ──[Select Best]──→ Goal Found
     ↓                 ↓              ↓              ↓
  Start Node     Generate Plan    Priority Queue   Optimal Path
                 (Apply Actions)   (Binary Heap)
```

#### PlanExecutor (Sequential Execution)
**Purpose**: Execute action plans
**Features**: Reactive replanning, error handling

```rust
pub struct PlanExecutor {
    max_retries: u32,
    reactive_enabled: bool,
}

pub struct ExecutionResult {
    pub success: bool,
    pub steps_completed: usize,
    pub execution_steps: Vec<ExecutionStep>,
    pub goals_satisfied: Vec<Goal>,
}
```

**Execution Flow**:
```
For each action in plan:
    ├─→ Check preconditions
    ├─→ Execute action
    ├─→ Apply effects
    ├─→ Update metrics
    └─→ Handle failures
         ├─→ Trigger replanning (if enabled)
         └─→ Continue or abort
```

### 2. State Management

#### WorldState
**Purpose**: Track current system state
**Structure**:

```rust
pub struct WorldState {
    pub properties: HashMap<WorldProperty, bool>,
    pub schema_cache: HashMap<String, Arc<Schema>>,
    pub pattern_cache: HashMap<String, SuccessPattern>,
    pub token_budget: u32,
    pub current_request: String,
}
```

**State Transitions**:
```
RequestReceived
       ↓
  SchemaDetected
       ↓
  PatternChecked ────┐
       ↓              │
  RequestValidated   │
       ↓              │
  ResponseGenerated  │
       ↓              │
  PatternLearned ←───┘
       ↓
    Complete
```

#### WorldProperty (Enum)
**Purpose**: Discrete state properties

```rust
pub enum WorldProperty {
    // Information
    SchemaAvailable(String),
    PatternAvailable(String),
    RequestValidated,

    // Generation
    ResponseGenerated,
    ResponseValidated,

    // Optimization
    TokenCompressionApplied(u8),
    TokenBudgetRemaining(u32),

    // Learning
    PatternLearned(String),
    MetricsUpdated,

    // Errors
    ValidationFailed(String),
    RequiresHumanIntervention,
}
```

### 3. Action System

#### Action
**Purpose**: Define operations with preconditions and effects

```rust
pub struct Action {
    pub action_type: ActionType,
    pub preconditions: HashSet<WorldProperty>,
    pub effects: HashSet<WorldProperty>,
    pub estimated_cost: u32,
    pub estimated_duration_ms: u64,
}
```

**Action Taxonomy**:

```
ActionType
├─ Information Gathering
│   ├─ DetectSchemaType
│   ├─ FetchSchema
│   └─ CheckPatternCache
│
├─ Validation
│   ├─ PreValidateRequest
│   ├─ PostValidateResponse
│   └─ QuickValidatePattern
│
├─ Generation
│   ├─ GenerateResponse
│   ├─ GenerateFromPattern
│   └─ GenerateFromTemplate
│
├─ Optimization
│   ├─ CompressRequest
│   └─ OptimizeTokenUsage
│
├─ Learning
│   ├─ LearnSuccessPattern
│   └─ UpdateMetrics
│
└─ Error Handling
    ├─ FixValidationErrors
    ├─ Replan
    └─ RequestClarification
```

### 4. Pattern Caching

#### IntelligentCache
**Purpose**: Store and retrieve successful patterns
**Features**: LSH similarity, confidence scoring, LRU eviction

```rust
pub struct IntelligentCache {
    patterns: DashMap<String, SuccessPattern>,
    lru_cache: LruCache<String, SuccessPattern>,
    similarity_engine: LSHEngine,
}

pub struct SuccessPattern {
    pub id: String,
    pub request_hash: Vec<u8>,      // LSH signature
    pub action_sequence: Vec<ActionType>,
    pub confidence: u8,              // 0-100
    pub success_rate: f64,           // 0.0-1.0
    pub avg_tokens: u32,
    pub usage_count: u32,
    pub created_at: Instant,
    pub last_used: Instant,
}
```

**Pattern Flow**:
```
Successful Execution
       ↓
  Extract Pattern
       ↓
  Calculate Similarity Hash (LSH)
       ↓
  Store in Cache with Metadata
       ↓
  On New Request
       ↓
  Calculate Similarity
       ↓
  If Similar (≥threshold): Reuse Pattern
  Else: Full Planning
```

### 5. Goal System

#### GoalState
**Purpose**: Define objectives to achieve

```rust
pub struct GoalState {
    pub goals: Vec<Goal>,
    pub required_properties: Vec<WorldProperty>,
    pub priority_level: u8,
    pub timeout_ms: u32,
}

pub enum Goal {
    GenerateValidResponse,
    OptimizeTokenUsage,
    MaximizeConfidence,
    EnsureSchemaAvailable,
    ReuseSuccessfulPattern,
}
```

**Goal Hierarchy**:
```
BestEffort (Priority 10)
├─ EnsureSchemaAvailable (9)
├─ ReuseSuccessfulPattern (8)
├─ GenerateValidResponse (7)
└─ LearnFromSuccess (6)
```

### 6. Metrics & Monitoring

#### GOAPMetrics
**Purpose**: Track performance and effectiveness

```rust
pub struct GOAPMetrics {
    pub total_plans_generated: u64,
    pub successful_plans: u64,
    pub failed_plans: u64,
    pub replans_triggered: u64,
    pub average_plan_depth: f64,
    pub average_execution_time_ms: f64,
    pub goal_success_rates: HashMap<Goal, f64>,
    pub action_success_rates: HashMap<ActionType, f64>,
    pub tokens_saved_by_planning: u64,
    pub cache_hit_rate: f64,
}
```

## Design Decisions

### 1. A* Search Algorithm

**Why A***:
- Finds optimal solutions
- Complete (finds solution if exists)
- Efficient (uses heuristic to guide search)

**Alternative Considered**: Dijkstra's algorithm
- **Rejected**: Slower, explores more nodes
- **Chosen**: A* balances optimality and speed

### 2. Weighted Heuristic

**Heuristic**: `h(n) = w₁ * token_cost + w₂ * time_cost + w₃ * (1 - success_prob)`

**Weights**:
- Token cost: 50% (primary concern)
- Time cost: 30% (secondary)
- Success probability: 20% (tertiary)

**Why Weighted**:
- Optimizes for token efficiency (business cost)
- Considers execution time (user experience)
- Factors in success likelihood (reliability)

### 3. Pattern Similarity (LSH)

**Why LSH**:
- Fast similarity search: O(log n)
- Scalable to large pattern sets
- Handles high-dimensional data (text embeddings)

**Alternative Considered**: Vector search (FAISS)
- **Rejected**: More complex, heavier dependencies
- **Chosen**: LSH simpler, adequate performance

### 4. Reactive Replanning

**Triggers**:
- Token budget critical (<100)
- Validation failure
- Execution timeout
- Schema fetch failure

**Why Reactive**:
- Real-world systems fail
- Better than fail-fast
- Improves robustness

### 5. Embedded Database (redb)

**Why redb**:
- No external dependencies
- ACID transactions
- Simple API
- Good performance for pattern caching

**Alternative Considered**: SQLite
- **Rejected**: Heavier, more features than needed
- **Chosen**: redb lighter, sufficient for GOAP

## Data Flow

### Request Processing Flow

```
1. Request Received
   ↓
2. Create WorldState
   - token_budget = configured default
   - current_request = request text
   - properties = empty
   ↓
3. Check Pattern Cache
   - Calculate request hash
   - Find similar patterns (LSH)
   - Evaluate confidence
   ↓
4. If High-Confidence Pattern:
   - Use pattern
   - Skip to execution
   ↓
5. If No Pattern / Low Confidence:
   - Run A* planner
   - Find optimal action sequence
   - Generate plan
   ↓
6. Execute Plan
   - For each action:
     * Check preconditions
     * Execute action
     * Apply effects
     * Update metrics
     * Handle failures (replanning)
   ↓
7. Collect Results
   - success = all actions completed?
   - steps_completed = actions executed
   - tokens_used = budget - remaining
   - goals_satisfied = achieved goals
   ↓
8. Learn Pattern
   - If successful:
     * Extract action sequence
     * Calculate similarity hash
     * Store in cache
     * Update metadata
   ↓
9. Return Result
```

## Performance Characteristics

### Planning Complexity

```
Time Complexity: O(b^d)
- b = branching factor (avg actions per state)
- d = depth (plan length)

Space Complexity: O(b^d)
- Stores nodes in priority queue
- Maximum queue size during search

Example:
- b = 5, d = 10
- Nodes explored: ~9.7 million (worst case)
- Average: ~100,000 nodes (with good heuristic)
```

### Cache Lookup

```
Time Complexity: O(log n)
- LSH: hash → bucket → compare candidates
- Typical: 3-5 comparisons per lookup

Space Complexity: O(n)
- n = number of patterns stored
- Each pattern: ~1KB
- 1000 patterns ≈ 1MB memory
```

### Pattern Similarity

```
Time Complexity: O(k)
- k = hash size (typically 64-256 bits)
- Bitwise operations: very fast

Space Complexity: O(1)
- Fixed-size hash per request
- No additional memory per comparison
```

## Extension Points

### 1. Custom Actions

```rust
// Define custom action type
#[derive(Debug, Clone)]
pub enum CustomActionType {
    MyCustomAction(String),
}

// Implement Action trait
impl Action for CustomAction {
    fn execute(&self, state: &mut WorldState) -> Result<ActionResult> {
        // Custom logic
        Ok(ActionResult::Success)
    }
}
```

### 2. Custom Heuristics

```rust
pub struct MyHeuristic;

impl Heuristic for MyHeuristic {
    fn estimate(&self, state: &WorldState, goal: &GoalState) -> f64 {
        // Custom estimation logic
        100.0
    }
}
```

### 3. Custom Similarity

```rust
pub struct MySimilarityEngine;

impl SimilarityEngine for MySimilarityEngine {
    fn calculate(&self, hash1: &[u8], hash2: &[u8]) -> f64 {
        // Custom similarity calculation
        0.85
    }
}
```

### 4. Custom Learning

```rust
pub struct MyLearningStrategy;

impl LearningStrategy for MyLearningStrategy {
    fn extract_pattern(&self, execution: &ExecutionResult) -> Option<SuccessPattern> {
        // Custom pattern extraction
        Some(pattern)
    }
}
```

## Threading Model

### Async/Await Architecture

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let system = GOAPSystem::new();

    // Process requests concurrently
    let requests = vec![req1, req2, req3];
    let results = futures::future::join_all(
        requests.into_iter().map(|req| system.process_request(req))
    ).await;

    Ok(())
}
```

**Concurrency**:
- Each request: Independent world state
- No shared mutable state during execution
- Cache: Read-heavy (DashMap for lock-free reads)
- Safe concurrent access to patterns

## Error Handling

### Error Hierarchy

```
Error (trait object)
├─ PlanningError
│   ├─ NoPathFound
│   ├─ TokenBudgetExceeded
│   ├─ Timeout
│   └─ InvalidGoal
│
├─ ExecutionError
│   ├─ ActionFailed
│   ├─ MaxRetriesExceeded
│   └─ ReplanFailed
│
├─ CacheError
│   ├─ PatternNotFound
│   ├─ CacheFull
│   └─ CorruptionDetected
│
└─ ValidationError
    ├─ InvalidRequest
    ├─ SchemaValidationFailed
    └─ ResponseValidationFailed
```

**Error Flow**:
```
Error Occurs
   ↓
Classify Error Type
   ↓
Determine Recovery Strategy
   ├─ Transient → Retry
   ├─ Persistent → Replan
   └─ Fatal → Fail
   ↓
Execute Recovery
   ↓
Update Metrics
```

## Security Considerations

### Input Validation

```rust
fn validate_request(request: &str) -> Result<()> {
    if request.is_empty() {
        return Err(Error::Validation(ValidationError::InvalidRequest(
            "Request cannot be empty".to_string()
        )));
    }

    if request.len() > MAX_REQUEST_SIZE {
        return Err(Error::Validation(ValidationError::RequestTooLong(
            request.len()
        )));
    }

    Ok(())
}
```

### Schema Validation

```rust
fn validate_schema(schema: &Schema) -> Result<()> {
    // Verify schema is from trusted source
    if !schema.is_whitelisted() {
        return Err(Error::Validation(ValidationError::InvalidSchema(
            "Untrusted schema source".to_string()
        )));
    }

    Ok(())
}
```

### Pattern Security

```rust
fn validate_pattern(pattern: &SuccessPattern) -> Result<()> {
    // Validate action sequence doesn't contain dangerous actions
    for action in &pattern.action_sequence {
        if action.is_dangerous() {
            return Err(Error::Cache(CacheError::PatternValidationFailed));
        }
    }

    Ok(())
}
```

## Performance Optimization

### 1. Heuristic Caching
Cache heuristic values for similar states:

```rust
struct HeuristicCache {
    cache: HashMap<StateHash, f64>,
}

impl Heuristic {
    fn estimate(&self, state: &WorldState) -> f64 {
        let hash = state.hash();
        if let Some(cached) = self.cache.get(&hash) {
            return *cached;
        }

        let estimate = self.calculate(state);
        self.cache.insert(hash, estimate);
        estimate
    }
}
```

### 2. Parallel Action Execution
Execute independent actions in parallel:

```rust
async fn execute_parallel(actions: &[Action]) -> Vec<Result<ActionResult>> {
    futures::future::join_all(
        actions.iter().map(|action| action.execute())
    ).await
}
```

### 3. Batch Processing
Process multiple requests together:

```rust
async fn process_batch(requests: Vec<Request>) -> Vec<Result<Response>> {
    let chunks = requests.chunks(10);
    let results = futures::future::join_all(
        chunks.map(|chunk| process_chunk(chunk.to_vec()))
    ).await;
    results.concat()
}
```

### 4. Lazy Loading
Load schemas and patterns on-demand:

```rust
impl WorldState {
    fn get_schema(&self, id: &str) -> Result<Arc<Schema>> {
        // Check cache first
        if let Some(schema) = self.schema_cache.get(id) {
            return Ok(Arc::clone(schema));
        }

        // Lazy load from storage
        let schema = load_schema_from_storage(id)?;
        self.schema_cache.insert(id, Arc::clone(&schema));
        Ok(schema)
    }
}
```

## Monitoring & Observability

### Structured Logging

```rust
info!("Processing request"; "request_id" => id, "budget" => budget);

debug!("Executing action";
    "action" => action.name(),
    "cost" => action.get_cost(),
    "preconditions" => action.preconditions.len()
);

warn!("Token budget critical"; "remaining" => tokens, "threshold" => 100);

error!("Action failed";
    "action" => action.name(),
    "error" => error,
    "request_id" => id
);
```

### Metrics Export

```rust
// Prometheus metrics
let metrics = vec![
    counter!("goap_requests_total").inc(),
    histogram!("goap_request_duration").observe(duration),
    gauge!("goap_cache_size").set(cache.len() as f64),
];

// Export to monitoring system
monitoring.export(metrics);
```

## Deployment Architecture

### Single Process (Default)

```
┌─────────────────────────────────────────┐
│             GOAP Process                 │
│  ┌─────────┐  ┌──────────┐  ┌────────┐ │
│  │ Planner │  │ Executor │  │ Cache  │ │
│  └─────────┘  └──────────┘  └────────┘ │
│  ┌─────────┐  ┌──────────┐  ┌────────┐ │
│  │ Metrics │  │ Storage  │  │ Config │ │
│  └─────────┘  └──────────┘  └────────┘ │
└─────────────────────────────────────────┘
```

### Distributed (Future)

```
┌──────────┐  ┌──────────┐  ┌──────────┐
│  Client  │→ │   Load   │→ │  GOAP    │
│          │  │ Balancer │  │ Cluster  │
└──────────┘  └──────────┘  └──────────┘
                              │
                    ┌─────────┴─────────┐
                    │                   │
              ┌──────────┐       ┌──────────┐
              │  Planner │       │ Executor │
              │  Pool    │       │   Pool   │
              └──────────┘       └──────────┘
                    │                   │
              ┌──────────┐       ┌──────────┐
              │   Cache  │       │  Storage │
              │ Cluster  │       │ Cluster  │
              └──────────┘       └──────────┘
```

## Technology Stack

### Core
- **Language**: Rust 1.91+
- **Async Runtime**: Tokio
- **Error Handling**: thiserror + anyhow

### Data
- **Pattern Storage**: redb (embedded database)
- **Caching**: DashMap + LRU cache
- **Serialization**: serde + serde_json

### Search
- **Planning**: A* algorithm
- **Similarity**: LSH (Locality Sensitive Hashing)
- **Heuristics**: Weighted cost function

### Monitoring
- **Logging**: tracing + tracing-subscriber
- **Metrics**: custom metrics + Prometheus (optional)

## Next Steps

- Review [Tutorial: Planning](TUTORIAL_PLANNING.md) for detailed planning walkthrough
- Study [Tutorial: Patterns](TUTORIAL_PATTERNS.md) for caching strategies
- Read [Configuration Guide](CONFIGURATION.md) for tuning
- Check [Performance Guide](PERFORMANCE.md) for optimization

## Reference

- [API Documentation](../docs/api/index.html)
- [Source Code](../src/goap/)
- [Examples](../examples/)
