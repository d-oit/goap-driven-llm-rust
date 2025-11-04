# Research Phase Output

**Date**: 2025-11-03 | **Feature**: GOAP-Driven LLM Strategic Reasoning | **Status**: Complete

## Architecture Decisions

### 1. Storage Layer: redb Embedded Database

**Decision**: Use redb (Redis-compatible embedded database) for persistent pattern and goal storage

**Rationale**:
- Zero external dependencies (perfect for Claude Code skill/MCP)
- ACID compliance ensures data integrity for pattern cache
- Skill-scoped isolation provides logical boundaries
- High-performance embedded storage without Redis server overhead
- Battle-tested (https://github.com/cberner/redb) with proven reliability

**Alternatives Considered**:
- SQLite: More complex API, larger binary size
- JSON file storage: No ACID guarantees, concurrent access issues
- In-memory only: No persistence across sessions
- Redis server: External dependency, not suitable for skill architecture

**Implementation Notes**:
- Store patterns and goals as JSON-serialized structs
- Use skill-scoped namespace: "goap:patterns:*" and "goap:goals:*"
- Implement proper transaction handling for pattern updates

### 2. Runtime Model: Single-Threaded Async

**Decision**: Single-threaded async runtime using tokio

**Rationale**:
- Matches Claude Code/MCP integration model (one request at a time)
- Async enables efficient I/O for redb operations
- Simpler than multi-threaded (no data race concerns)
- Sufficient for skill architecture patterns

**Alternatives Considered**:
- Multi-threaded runtime: Unnecessary complexity for single-request model
- Synchronous blocking I/O: Poor performance, blocks the runtime
- Thread-per-request: Excessive overhead for skill context

**Implementation Notes**:
- Use `tokio::main` with appropriate feature flags
- Async functions for all redb operations
- Proper async error handling with `?` operator

### 3. A* Search Heuristic: Weighted Cost Function

**Decision**: Cost-based heuristic combining token cost + execution time + success probability

**Rationale**:
- Maintains admissibility (never overestimates true cost)
- Allows tuning for different priorities (cost vs speed vs reliability)
- Aligns with user requirements for token optimization
- Enables reactive replanning based on resource constraints

**Formula**: `h(n) = α(token_cost) + β(execution_time) + γ(success_probability)`

Where α, β, γ are configurable weights based on current goals

**Alternatives Considered**:
- Manhattan/Euclidean distance: Not applicable for abstract goal space
- Pure token cost: Ignores execution time and reliability
- Uniform cost: No optimization, explores all paths equally

**Implementation Notes**:
- Validate admissibility: h(n) ≤ actual cost to goal
- Store heuristic data in pattern cache
- Dynamic weight adjustment based on current token budget

### 4. Goal Representation: Struct + HashMap Properties

**Decision**: Struct-based goals with HashMap for flexible properties, stored in redb as JSON

**Rationale**:
- Type safety for known properties (task type, complexity, language)
- Flexibility for domain-specific properties via HashMap
- JSON serialization allows easy inspection and debugging
- Compatible with redb's binary storage format
- Queryable via pattern matching on struct fields

**Alternatives Considered**:
- Pure HashMap: Loses type safety
- Static enum-based: Too rigid for diverse goal space
- Trait objects: Over-engineering for this use case

**Implementation Notes**:
```rust
struct Goal {
    task_type: String,
    properties: HashMap<String, Value>,
    priority: Priority,
    satisfaction_criteria: Vec<Condition>,
}
```

### 5. Pattern Caching Strategy

**Decision**: Store successful execution paths with confidence metrics in redb

**Rationale**:
- Enables 50-70% token reduction through reuse
- Confidence-based selection prevents low-quality reuse
- Skill-scoped sharing improves efficiency across users
- Metadata enables continuous learning

**Pattern Structure**:
```rust
struct SuccessPattern {
    goal_signature: String,  // Hash of goal properties
    action_sequence: Vec<Action>,
    confidence: f64,
    success_count: u64,
    avg_token_usage: u32,
    avg_execution_time: Duration,
}
```

**Alternatives Considered**:
- Simple string matching: Too rigid, no partial matching
- Pure machine learning: Black box, hard to debug
- No caching: Fails performance requirements

**Implementation Notes**:
- Pattern confidence threshold: 70% (from spec)
- Extract patterns from successful executions automatically
- Update confidence based on reuse success rate
- Implement pattern similarity matching

### 6. World State Management

**Decision**: Track comprehensive world state including schemas, patterns, tokens, satisfied properties

**Rationale**:
- A* search requires accurate current state
- Enables reactive replanning when state changes
- Validates preconditions before action execution
- Tracks effects during plan execution

**World State Structure**:
```rust
struct WorldState {
    available_schemas: HashSet<String>,
    pattern_cache: HashMap<String, SuccessPattern>,
    token_budget: u32,
    satisfied_properties: HashSet<Property>,
    current_context: Context,
}
```

**Implementation Notes**:
- Immutable snapshots for A* search
- Apply effects as diffs
- Validate preconditions against current state
- Support state rollback for failed plans

### 7. Error Handling Strategy

**Decision**: Custom Error enum with Result<T> propagation, no unwrap()

**Rationale**:
- Matches Rust best practices (from constitution)
- Clear error types for different failure modes
- Integrates with anyhow for context
- Enables graceful degradation

**Error Variants**:
- `PlanningFailed`: A* search couldn't find solution
- `ExecutionFailed`: Action execution failed
- `TokenBudgetExceeded`: Budget exceeded during execution
- `PatternNotFound`: No suitable pattern for goal
- `InvalidGoal`: Goal malformed or missing properties

**Implementation Notes**:
- All public APIs return Result<T>
- Use `?` operator for propagation
- Contextual errors with anyhow
- User-friendly error messages

### 8. Testing Approach

**Decision**: TDD with 82% unit coverage, 100% integration for GOAP planner, property-based testing for A*

**Rationale**:
- GOAP algorithms are complex, need comprehensive testing
- Property-based testing catches edge cases in A* search
- Integration tests validate end-to-end planning
- Performance tests ensure benchmarks are met

**Test Structure**:
- Unit tests: Each component in isolation
- Integration tests: GOAP planner end-to-end
- Property tests: A* algorithm correctness
- Performance tests: Token reduction, response time
- Fuzz tests: Input parsing and validation

**Implementation Notes**:
- tokio-test for async tests
- mockall for external dependencies
- criterion for benchmarking
- proptest for property-based testing

## Open Research Items

All critical decisions resolved in this phase. No pending clarifications.

## Next Steps

Proceed to Phase 1: Design & Contracts
- Generate data model documentation (COMPLETE)
- Define API contracts
- Create quickstart guide
- Update agent context
