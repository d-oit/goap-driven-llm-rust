# Implementation Plan: GOAP-Driven LLM Strategic Reasoning

**Branch**: `001-goap-llm-planning` | **Date**: 2025-11-03 | **Spec**: [link]
**Input**: Feature specification from `/specs/001-goap-llm-planning/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Build a GOAP (Goal-Oriented Action Planning) system for LLM strategic reasoning that optimizes token usage through A* search, pattern reuse, and reactive replanning. Implemented as a Claude Code skill/MCP with single-threaded async architecture using Rust 1.91+ and redb for persistent pattern storage.

## Technical Context

<!--
  ACTION REQUIRED: Replace the content in this section with the technical details
  for the project. The structure here is presented in advisory capacity to guide
  the iteration process.
-->

**Language/Version**: Rust 1.91+ (Edition 2021)
**Primary Dependencies**: redb (embedded database), serde (serialization), tokio (async runtime), tracing (logging)
**Storage**: redb embedded database for patterns and goals with skill-scoped isolation
**Testing**: cargo test, tokio-test (async tests), mockall (mocks), criterion (benchmarking)
**Target Platform**: Claude Code skill/MCP (embedded integration)
**Project Type**: Single library with optional CLI wrapper
**Performance Goals**: 50-70% token reduction, 25-35% response time improvement, 90%+ success rate, 10k+ requests/hour
**Constraints**: Single-threaded with async await, <5000 token budget, 95%+ budget adherence
**Scale/Scope**: Pattern cache with A* search over world state, reactive replanning, goal orchestration

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

**GATE EVALUATION** ✅ PASS

| Principle | Implementation Strategy | Verification Method |
|-----------|------------------------|---------------------|
| I. Code Quality Standards | Rust 1.91+ Edition 2021, clippy warnings resolved, rustfmt formatting, comprehensive docs for public APIs | `cargo clippy --all-targets --all-features`, `cargo fmt --check` |
| II. Testing Standards | TDD approach, 82% unit coverage, 100% integration for GOAP planner, property-based tests for A* algorithm | `cargo test --all-features`, coverage reporting, criterion benchmarks |
| III. User Experience Consistency | Library-first with optional CLI, subcommands pattern, JSON output, structured error messages | CLI interface tests, output format validation |
| IV. Performance Requirements | O(n²) A* complexity, bounded memory for world state, pattern cache with proper invalidation, automated benchmarks | Performance benchmarks, memory profiling |
| V. Observability & Documentation | Structured JSON logging, metrics for latency/throughput/error rates, comprehensive README with quickstart | Log validation, metrics exposition, documentation completeness check |

**Compliance Status**: All gates satisfied with defined implementation strategies

## Phase 0: Research ✅ COMPLETE

**Output**: `research.md` with 8 architecture decisions resolved:
1. ✅ Storage Layer: redb embedded database
2. ✅ Runtime Model: Single-threaded async with tokio
3. ✅ A* Heuristic: Weighted cost function (token + time + probability)
4. ✅ Goal Representation: Struct + HashMap stored as JSON in redb
5. ✅ Pattern Caching: Confidence-based with skill-scoped isolation
6. ✅ World State: Comprehensive tracking with property management
7. ✅ Error Handling: Custom Error enum with Result<T>
8. ✅ Testing: TDD with 82% unit, 100% integration coverage

All NEEDS CLARIFICATION items resolved. No pending research.

## Phase 1: Design & Contracts ✅ COMPLETE

**Outputs Generated**:
- ✅ `data-model.md` - Complete entity definitions with validation rules
- ✅ `contracts/api.yaml` - OpenAPI 3.0 specification
- ✅ `contracts/README.md` - API integration guide
- ✅ `quickstart.md` - Getting started guide
- ✅ Agent context updated in `CLAUDE.md`

**Design Decisions Documented**:
- 10 core entities defined (WorldState, Action, Goal, Pattern, etc.)
- 9 API endpoints specified with request/response schemas
- State transitions and validation rules defined
- Error types and handling strategies documented

**Constitution Re-Check**: ✅ PASS - All principles still satisfied with detailed design

## Project Structure

### Documentation (this feature)

```text
specs/[###-feature]/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (/speckit.plan command)
├── data-model.md        # Phase 1 output (/speckit.plan command)
├── quickstart.md        # Phase 1 output (/speckit.plan command)
├── contracts/           # Phase 1 output (/speckit.plan command)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)
<!--
  ACTION REQUIRED: Replace the placeholder tree below with the concrete layout
  for this feature. Delete unused options and expand the chosen structure with
  real paths (e.g., apps/admin, packages/something). The delivered plan must
  not include Option labels.
-->

```text
# [REMOVE IF UNUSED] Option 1: Single project (DEFAULT)
src/
├── models/
├── services/
├── cli/
└── lib/

tests/
├── contract/
├── integration/
└── unit/

# [REMOVE IF UNUSED] Option 2: Web application (when "frontend" + "backend" detected)
backend/
├── src/
│   ├── models/
│   ├── services/
│   └── api/
└── tests/

frontend/
├── src/
│   ├── components/
│   ├── pages/
│   └── services/
└── tests/

# [REMOVE IF UNUSED] Option 3: Mobile + API (when "iOS/Android" detected)
api/
└── [same as backend above]

ios/ or android/
└── [platform-specific structure: feature modules, UI flows, platform tests]
```

**Structure Decision**: Single project structure (library-first with optional CLI wrapper)

This is a Claude Code skill/MCP that provides GOAP planning capabilities as a library.
All components are modular and independently testable.

**Source Structure**:
src/
├── goap/                    # Main GOAP module
│   ├── mod.rs              # Module root
│   ├── actions/            # Action definitions and execution
│   │   ├── mod.rs
│   │   ├── action.rs       # Action, ActionType, ActionBuilder
│   │   └── executor.rs     # PlanExecutor, ExecutionResult, ExecutionStep
│   ├── cache/              # Pattern and schema caching
│   │   ├── mod.rs
│   │   ├── pattern.rs      # SuccessPattern, LSH similarity
│   │   ├── schema.rs       # SchemaCache, CachedSchema
│   │   └── intelligent.rs  # IntelligentCache with lookup
│   ├── goals/              # Goal management and orchestration
│   │   ├── mod.rs
│   │   ├── goal.rs         # Goal, GoalState with satisfaction
│   │   ├── orchestrator.rs # GoalOrchestrator
│   │   └── reactive.rs     # ReactivePlanner
│   ├── metrics/            # Performance metrics
│   │   ├── mod.rs
│   │   └── goap_metrics.rs # GOAPMetrics, MetricsSnapshot
│   ├── planning/           # A* search and planning
│   │   ├── mod.rs
│   │   ├── graph.rs        # ActionGraph
│   │   ├── heuristic.rs    # Heuristic functions
│   │   └── planner.rs      # GOAPPlanner, PlanNode
│   ├── world/              # World state management
│   │   ├── mod.rs
│   │   ├── property.rs     # WorldProperty enum
│   │   └── state.rs        # WorldState struct and methods
│   └── system.rs           # GOAPSystem orchestrator
├── error.rs                # Error types (Planning, Execution, Cache, Validation)
└── lib.rs                  # Library entry point
tests/
├── unit/           # Unit tests for components
├── integration/    # End-to-end tests
└── contract/       # API contract tests
```

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| N/A | No constitution violations | All principles satisfied |

## Implementation Phases

### Phase 1: Project Foundation
**Duration**: 1-2 weeks | **Priority**: P0 (Critical)

1. **Testing Infrastructure**
   - Set up tokio-test for async tests
   - Configure mockall for mocking external dependencies
   - Set up proptest for property-based testing
   - Create test fixtures in `tests/fixtures/`

2. **Benchmarking Infrastructure**
   - Configure criterion with HTML reports
   - Set up benchmark structure in `benches/`
   - Create benchmark patterns for A*, planning, and cache operations

3. **Documentation Structure**
   - Set up rustdoc configuration
   - Create API documentation structure
   - Set up README and user guides

4. **Example Programs**
   - Create basic planning example
   - Create pattern reuse example
   - Create reactive replanning example

### Phase 2: Core Testing
**Duration**: 2-3 weeks | **Priority**: P0 (Critical)

**Goal**: Achieve 82% unit test coverage and 100% integration coverage for GOAP planner

**Test Categories**:
- **Unit Tests** (`tests/unit/`): Test each module in isolation
  - World state management
  - Action definitions and validation
  - A* planning algorithm
  - Pattern cache operations
  - Goal orchestration
  - Error handling

- **Integration Tests** (`tests/integration/`): Test end-to-end workflows
  - Planner-executor flow
  - Pattern reuse pipeline
  - Reactive replanning on failure
  - Token budget optimization
  - Learning from success patterns

- **Property-Based Tests**: Test invariants across many inputs
  - World state transitions
  - Action precondition/effect validation
  - Heuristic admissibility for A*
  - Pattern confidence calculations

### Phase 3: Benchmarking Suite
**Duration**: 1-2 weeks | **Priority**: P1 (High)

**Goal**: Establish performance baselines and regression detection

**Benchmark Categories**:
- **Planning Performance**
  - A* search on varying world state sizes
  - Heuristic calculation performance
  - Plan depth vs execution time

- **Caching Performance**
  - Pattern lookup latency
  - Cache hit/miss ratios
  - Memory usage patterns

- **End-to-End Performance**
  - Token efficiency (target: 50-70% reduction)
  - Response time (target: 25-35% improvement)
  - System throughput (target: 10,000+ requests/hour)
  - Reactive replanning recovery rate (target: 82%+)

### Phase 4: Documentation
**Duration**: 1-2 weeks | **Priority**: P1 (High)

**Goal**: Comprehensive user and developer documentation

**Documentation Types**:
- **API Documentation**: Generated via rustdoc
  - All public types and functions
  - Usage examples in doc comments
  - Type relationships and traits

- **User Guides**
  - Quickstart guide
  - Pattern reuse tutorial
  - Reactive planning guide
  - Configuration options

- **Developer Guides**
  - Architecture overview
  - Adding new action types
  - Extending the planner
  - Contributing guidelines

### Phase 5: Example Programs
**Duration**: 1 week | **Priority**: P2 (Medium)

**Goal**: Demonstrate real-world usage patterns

**Examples**:
1. **`examples/basic_planning.rs`**: Simple GOAP planning
   - Create world state
   - Define actions and goals
   - Run planner and executor
   - Display results

2. **`examples/pattern_reuse.rs`**: Pattern caching and reuse
   - Process initial request
   - Cache successful pattern
   - Reuse pattern for similar request
   - Measure efficiency gains

3. **`examples/reactive_replanning.rs`**: Failure handling
   - Create scenario with potential failures
   - Execute plan with simulated failure
   - Demonstrate reactive replanning
   - Show recovery metrics

4. **`examples/token_optimization.rs`**: Budget management
   - Set token budget constraints
   - Process complex request
   - Show token consumption tracking
   - Demonstrate optimization decisions

5. **`examples/metrics_collection.rs`**: Performance monitoring
   - Enable metrics collection
   - Execute multiple requests
   - Display performance statistics
   - Show learning effects

## Risk Assessment

| Risk | Impact | Likelihood | Mitigation |
|------|--------|------------|------------|
| Performance targets not met | High | Medium | Optimize A* heuristic, add caching layers |
| Test coverage below 82% | High | Low | Focus on edge cases, add integration tests early |
| Pattern detection accuracy low | Medium | Medium | Tune LSH parameters, adjust confidence thresholds |
| Documentation incomplete | Medium | Low | Generate from code, prioritize API docs |
| Examples don't compile | Medium | Low | Test examples in CI, keep them simple |

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Unit test coverage | 82%+ | `cargo test --coverage` |
| Integration test coverage | 100% | Manual verification + CI |
| Documentation completeness | 100% | All public APIs documented |
| Example compilation | 100% | `cargo test --examples` |
| Benchmark completion | All 10 benchmarks | `cargo bench` |
| Performance regression detection | Enabled | Nightly benchmark comparison |

## Deliverables

### Code Deliverables
- [x] Source implementation (completed in tasks.md)
- [ ] Test suite (82% unit, 100% integration coverage)
- [ ] Benchmark suite (10+ benchmarks)
- [ ] Documentation (API + user guides)
- [ ] Example programs (5 examples)

### Verification Deliverables
- [ ] Coverage reports
- [ ] Benchmark reports
- [ ] Documentation builds
- [ ] CI/CD pipeline configuration
- [ ] Performance regression tracking |
