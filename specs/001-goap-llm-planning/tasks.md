# Tasks: GOAP-Driven LLM Strategic Reasoning

**Input**: Design documents from `/specs/001-goap-llm-planning/`
**Prerequisites**: plan.md (✅), spec.md (✅), research.md (✅), data-model.md (✅), contracts/ (✅)

**Tests**: TDD approach strongly recommended for GOAP algorithms - tests included for all user stories

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2, US3)
- Include exact file paths in descriptions

## Path Conventions

- Library-first architecture: `src/`, `tests/` at repository root
- Core library: `src/goap/` with modules for world, actions, planning, goals, cache, metrics
- CLI wrapper: `src/cli/`
- Paths shown below follow repository structure from plan.md

---

## Phase 1: Setup (Shared Infrastructure) 🚀

**Purpose**: Project initialization and basic structure

- [X] T001 Create project structure per implementation plan in src/, tests/, examples/, benches/
- [X] T002 Initialize Rust project with Cargo.toml and dependencies (tokio, async-trait, thiserror, anyhow, dashmap, lru, serde, jsonschema)
- [X] T003 [P] Configure rustfmt, clippy, and cargo-watch for development
- [X] T004 [P] Setup workspace structure with lib crate "goap-llm" and CLI crate
- [X] T005 Create basic module structure in src/goap/ with mod.rs files for all submodules

---

## Phase 2: Foundational (Blocking Prerequisites) ⚠️ CRITICAL

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [X] T006 Implement core error types in src/error.rs (PlanningError, ExecutionError, CacheError, ValidationError)
- [X] T007 [P] Create WorldProperty enum in src/goap/world/property.rs
- [X] T008 [P] Create ActionType enum in src/goap/actions/action.rs
- [X] T009 [P] Create Goal enum in src/goap/goals/goal.rs
- [X] T010 Create basic Action struct in src/goap/actions/action.rs (without execution logic)
- [X] T011 Create basic GoalState struct in src/goap/goals/goal.rs (without orchestration logic)
- [X] T012 Setup testing infrastructure with tokio-test in tests/unit/mod.rs
- [X] T013 [P] Create test helpers and fixtures in tests/fixtures/
- [X] T014 [P] Setup criterion benchmarking infrastructure in benches/

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - GOAP Planning and Execution (Priority: P1) 🎯 MVP

**Goal**: Implement core GOAP functionality - system analyzes requests, creates optimal action plans via A* search, and executes them step-by-step to generate valid responses

**Independent Test**: Submit a single request and verify that a plan is generated with multiple ordered actions, actions are executed sequentially, a valid response is produced, and the system tracks world state changes throughout execution

### Tests for User Story 1 (TDD - write first!) ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T015 [P] [US1] Unit tests for WorldState in tests/unit/test_world_state.rs
- [X] T016 [P] [US1] Property-based tests for world state transitions using proptest
- [X] T017 [P] [US1] Unit tests for Action precondition/effect validation
- [X] T018 [P] [US1] Unit tests for A* planner correctness in tests/unit/test_planning.rs
- [X] T019 [P] [US1] Integration test for planner-executor flow in tests/integration/test_planner_exec.rs
- [X] T020 [US1] Performance benchmark for A* search in benches/planning_bench.rs

### Implementation for User Story 1

- [X] T021 [P] [US1] Implement WorldState struct and methods in src/goap/world/state.rs
- [X] T022 [P] [US1] Complete Action struct with preconditions/effects in src/goap/actions/action.rs
- [X] T023 [P] [US1] Implement ActionGraph in src/goap/planning/graph.rs
- [X] T024 [P] [US1] Implement Heuristic function in src/goap/planning/heuristic.rs
- [X] T025 [US1] Implement PlanNode with BinaryHeap for A* search in src/goap/planning/planner.rs
- [X] T026 [US1] Implement GOAPPlanner with A* algorithm in src/goap/planning/planner.rs
- [X] T027 [US1] Implement GoalState with satisfaction checking in src/goap/goals/goal.rs
- [X] T028 [US1] Implement PlanExecutor for sequential action execution in src/goap/actions/executor.rs
- [X] T029 [US1] Create ExecutionResult and ExecutionStep structs in src/goap/actions/executor.rs
- [X] T030 [US1] Wire planner and executor in src/goap/lib.rs
- [X] T031 [US1] Add structured logging for planning and execution

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Pattern Reuse for Efficiency (Priority: P2)

**Goal**: Detect similar requests from cache and reuse successful patterns to generate responses faster with fewer tokens

**Independent Test**: Process a request successfully (creating a pattern), submit a similar request, verify the pattern is detected and reused, and confirm reduced token usage and faster response

### Tests for User Story 2 (TDD - write first!) ⚠️

- [X] T032 [P] [US2] Unit tests for SuccessPattern in tests/unit/test_cache.rs
- [X] T033 [P] [US2] Unit tests for pattern similarity/confidence calculation
- [X] T034 [P] [US2] Integration test for pattern reuse in tests/integration/test_pattern_reuse.rs
- [X] T035 [US2] Performance benchmark for pattern lookup in benches/cache_bench.rs

### Implementation for User Story 2

- [X] T036 [P] [US2] Implement SuccessPattern struct in src/goap/cache/pattern.rs
- [X] T037 [P] [US2] Implement LSH-based similarity detection in src/goap/cache/pattern.rs
- [X] T038 [P] [US2] Implement confidence scoring algorithm in src/goap/cache/pattern.rs
- [X] T039 [P] [US2] Create LRU cache for schemas in src/goap/cache/schema.rs
- [X] T040 [P] [US2] Create Dashmap-based pattern cache in src/goap/cache/intelligent.rs
- [X] T041 [US2] Implement IntelligentCache with pattern lookup in src/goap/cache/intelligent.rs
- [X] T042 [US2] Add pattern detection to ActionType::GenerateFromPattern
- [X] T043 [US2] Update PlanExecutor to prioritize pattern reuse when confidence > 70%
- [X] T044 [US2] Integrate pattern cache with GOAPSystem in src/goap/lib.rs
### Performance Benchmarks for User Story 2

- [X] T045 [US2] Benchmark token usage with pattern reuse vs full generation (target: 50-70% reduction) in benches/token_efficiency_bench.rs
- [X] T046 [US2] Benchmark response time with pattern reuse (target: 25-35% improvement) in benches/response_time_bench.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Reactive Replanning on Failure (Priority: P3)

**Goal**: Automatically detect failures, trigger reactive replanning, and generate new action sequences to achieve goals through alternative means

**Independent Test**: Simulate a failure during plan execution, verify the system detects the failure, confirm a new plan is generated, and validate the new plan achieves the original goal

### Tests for User Story 3 (TDD - write first!) ⚠️

- [X] T045 [P] [US3] Unit tests for ReactivePlanner state machine
- [X] T046 [P] [US3] Unit tests for error detection and recovery logic
- [X] T047 [P] [US3] Integration test for reactive replanning in tests/integration/test_reactive_replan.rs
- [X] T048 [US3] Load test for replanning under failure scenarios

### Implementation for User Story 3

- [X] T049 [P] [US3] Implement GoalOrchestrator in src/goap/goals/orchestrator.rs
- [X] T050 [P] [US3] Implement dynamic goal priority adjustment in src/goap/goals/orchestrator.rs
- [X] T051 [P] [US3] Implement ReactivePlanner in src/goap/goals/reactive.rs
- [X] T052 [P] [US3] Implement should_replan logic (token budget, failures, timeouts)
- [X] T053 [P] [US3] Implement execute_next with reactive triggers
- [X] T054 [US3] Add replan tracking and metrics to ExecutionResult
- [X] T055 [US3] Integrate ReactivePlanner with PlanExecutor
- [X] T056 [US3] Add bounded retry logic (max 3 replans)
- [X] T057 [US3] Add failure recovery for schema fetch errors
- [X] T058 [US3] Benchmark reactive replanning recovery rate (SC-005: 82%+ recovery) in benches/reactive_replan_bench.rs

**Checkpoint**: At this point, User Stories 1, 2, AND 3 should all work independently

---

## Phase 6: User Story 4 - Token Budget Optimization (Priority: P3)

**Goal**: Monitor token consumption in real-time, apply compression techniques, and make real-time decisions to minimize token usage while maintaining response quality

**Independent Test**: Set a token budget constraint, process requests with varying complexity, verify the system stays within budget, and confirm response quality remains acceptable

### Tests for User Story 4 (TDD - write first!) ⚠️

- [X] T058 [P] [US4] Unit tests for token budget tracking in WorldState
- [X] T059 [P] [US4] Unit tests for token compression logic
- [X] T060 [P] [US4] Integration test for token budget enforcement
- [X] T061 [US4] Performance test for real-time budget monitoring

### Implementation for User Story 4

- [X] T062 [P] [US4] Implement TokenCompressionApplied property handler
- [X] T063 [P] [US4] Implement token budget monitoring in WorldState
- [X] T064 [P] [US4] Implement compression action (ActionType::CompressRequest)
- [X] T065 [P] [US4] Add token cost estimation to Action::estimate_cost()
- [X] T066 [P] [US4] Add budget-aware action selection in A* planner
- [X] T067 [US4] Implement real-time token consumption tracking during execution
- [X] T068 [US4] Add token budget threshold checks (<100 tokens triggers replan)
- [X] T069 [US4] Integrate token optimization with pattern reuse
- [X] T070 [US4] Add token efficiency metrics to GOAPMetrics
- [X] T071 [US4] Benchmark token budget adherence (SC-007: 95%+ compliance) in benches/token_budget_bench.rs

**Checkpoint**: At this point, User Stories 1-4 should all work independently

---

## Phase 7: User Story 5 - Learning from Success Patterns (Priority: P4)

**Goal**: Analyze successful execution paths, extract patterns, and store them with metadata for future reuse and optimization

**Independent Test**: Process multiple successful requests, verify patterns are extracted and stored, check pattern metadata (confidence, success rate, etc.), and confirm patterns are available for future reuse

### Tests for User Story 5 (TDD - write first!) ⚠️

- [X] T071 [P] [US5] Unit tests for pattern extraction algorithm
- [X] T072 [P] [US5] Unit tests for metadata update logic
- [X] T073 [P] [US5] Integration test for learning pipeline in tests/integration/
- [X] T074 [US5] Test pattern confidence improvement over time

### Implementation for User Story 5

- [X] T075 [P] [US5] Implement LearnSuccessPattern action in src/goap/actions/action.rs
- [X] T076 [P] [US5] Implement pattern extraction from ExecutionResult
- [X] T077 [P] [US5] Implement metadata recording (confidence, success_rate, avg_tokens)
- [X] T078 [P] [US5] Implement pattern learning in IntelligentCache
- [X] T079 [P] [US5] Add pattern decay/update mechanism
- [X] T080 [P] [US5] Implement heuristic adaptation based on learning
- [X] T081 [US5] Complete learning pipeline in PlanExecutor
- [X] T082 [US5] Add learning metrics to GOAPMetrics
- [X] T083 [US5] Integrate learning with pattern cache updates
- [X] T084 [US5] Benchmark pattern confidence improvement (SC-008: 10-15% per cycle) in benches/learning_effectiveness_bench.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 8: Polish & Cross-Cutting Concerns ✨

**Purpose**: Improvements that affect multiple user stories

- [X] T084 [P] Add comprehensive rustdoc documentation for all public APIs
- [X] T085 [P] Create example programs in examples/ (basic_planning.rs, pattern_reuse.rs, reactive_replanning.rs)
- [X] T086 [P] Add integration tests for end-to-end workflows in tests/integration/
- [X] T087 [P] Optimize A* search performance with heuristic caching
- [X] T088 [P] Add concurrent access benchmarks in benches/
- [X] T089 [P] Implement optional pattern persistence (serialize/deserialize)
- [X] T090 [P] Add CLI wrapper in src/cli/ for testing and demonstration
- [X] T091 [P] Create systemd service file or Docker setup (if applicable)
- [X] T092 [P] Add API documentation with openapi.yaml
- [X] T093 [P] Performance tuning - profile and optimize bottlenecks
- [X] T094 [P] Security review - input validation, error handling
- [X] T095 Run full test suite with cargo test --all-features
- [X] T096 Run clippy with cargo clippy --all-targets --all-features -- -D warnings
- [X] T097 Run benchmarks to verify performance goals
- [X] T098 Update quickstart.md with any new insights
- [X] T099 Final code review and cleanup
- [X] T100 Benchmark edge case handling (SC-009: 85%+ scenarios) in benches/edge_case_bench.rs
- [X] T101 Benchmark system throughput (SC-010: 10,000+ requests/hour) in benches/throughput_bench.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 9: Test Suite Implementation 🎯

**Purpose**: Create comprehensive test suite to achieve 82% unit coverage and 100% integration coverage

**✅ Status**: 58 inline unit tests already exist in source files (all passing)
**Missing**: External integration tests, property-based tests, and contract tests

### Testing Infrastructure Setup

- [X] T102 [P] **COMPLETED**: Inline unit tests already exist in source files (58 tests across 14 modules)
- [X] T103 [P] Create tests/integration/mod.rs with integration test modules
- [X] T104 [P] Create tests/fixtures/mod.rs and test utilities in tests/fixtures/
- [X] T105 [P] Setup test configuration in Cargo.toml for tokio-test, mockall, proptest

### Unit Tests (Inline - Rust Best Practice)

**✅ ALREADY COMPLETE**: All unit tests are in `#[cfg(test)]` blocks in source files
- Benefits: Test private APIs directly, faster development, industry standard
- Coverage: WorldState, Actions, Planning, Cache, Goals, Metrics, System
- Status: All 58 tests passing

### External Integration Tests (COMPLETED)

- [X] T106 [US1] Create tests/integration/test_planner_executor_flow.rs
  - End-to-end planning and execution via public API
  - GOAPSystem orchestration flow
  - Goal satisfaction verification
  - Metrics collection validation

- [X] T107 [US2] Create tests/integration/test_pattern_reuse.rs
  - Pattern storage and retrieval via public API
  - Similarity detection accuracy
  - Token savings measurement
  - Response time improvement validation

- [X] T108 [US3] Create tests/integration/test_reactive_replanning.rs
  - Failure injection and detection
  - Replanning trigger verification
  - Alternative path discovery
  - Recovery rate measurement

- [X] T109 [US4] Create tests/integration/test_token_budget.rs
  - Budget enforcement across execution
  - Compression triggers and effects
  - Optimization strategy selection
  - Budget compliance validation

- [X] T110 [US5] Create tests/integration/test_learning_pipeline.rs
  - Pattern extraction from successful executions
  - Metadata update and confidence adjustment
  - Learning impact on future planning
  - Pattern decay and cleanup

### Property-Based Tests (COMPLETED)

- [X] T111 [P] Create tests/unit/test_property_based.rs with proptest
  - World state transitions remain valid
  - Action sequences don't violate invariants
  - Heuristic is admissible for A*
  - Pattern confidence calculations are consistent

### Contract Tests (COMPLETED)

- [X] T112 Create tests/contract/test_api_contract.rs
  - Public API stability
  - Backward compatibility
  - Error type consistency
  - Serialization format validation

### End-to-End Workflow Tests (COMPLETED)

- [X] T113 Create tests/integration/test_full_request_flow.rs
  - Complete request → response cycle
  - All user story scenarios
  - System-wide performance validation
  - Error recovery across components

**Checkpoint**: Integration and property-based tests created and passing

---

## Phase 10: Benchmark Suite Implementation ⚡

**Purpose**: Create performance benchmarks to verify SC-002 through SC-010 targets

### Benchmark Infrastructure

- [X] T120 [P] Create benches/planning_bench.rs for A* algorithm benchmarks
  - Plan generation time vs world state size
  - Heuristic calculation performance
  - Memory usage for varying plan depths
  - Regression detection baseline

- [X] T121 [P] Create benches/cache_bench.rs for caching benchmarks
  - Pattern lookup latency (target: <1ms)
  - Cache hit rate measurement
  - Memory footprint analysis
  - Eviction performance

- [X] T122 [P] Create benches/token_efficiency_bench.rs for efficiency benchmarks
  - Token usage with/without pattern reuse (target: 50-70% reduction)
  - Response time improvement (target: 25-35%)
  - End-to-end performance metrics

- [X] T123 [P] Create benches/reactive_replan_bench.rs for replanning benchmarks
  - Failure recovery rate (target: 82%+)
  - Replanning overhead measurement
  - Alternative path discovery time
  - Success rate after replanning

### End-to-End Benchmarks

- [X] T124 Create benches/throughput_bench.rs
  - Requests per hour (target: 10,000+)
  - Concurrent request handling
  - Resource utilization
  - Sustained performance over time

- [X] T125 Create benches/edge_case_bench.rs
  - Edge case handling (target: 85%+ success)
  - Stress testing with extreme inputs
  - Degradation under load
  - Recovery after failures

- [X] T126 Create benches/learning_effectiveness_bench.rs
  - Pattern confidence improvement (target: 10-15% per cycle)
  - Learning curve measurement
  - Knowledge transfer between patterns
  - Adaptive heuristic tuning

- [X] T127 Create benches/comparison_bench.rs
  - GOAP vs naive approach comparison
  - Baseline performance establishment
  - Optimization impact measurement
  - Scalability analysis

### Benchmark Reporting

- [X] T128 Create benches/comparison.rs for baseline tracking
  - Historical performance tracking
  - Regression detection alerts
  - Performance trend analysis
  - CI integration for automated checks

**Checkpoint**: All benchmarks operational with HTML reports

---

## Phase 11: Documentation Implementation 📚

**Purpose**: Create comprehensive API docs, user guides, and developer documentation

### API Documentation

- [X] T129 [P] Add comprehensive rustdoc to all public types in src/lib.rs
  - Library overview with usage examples
  - Module hierarchy documentation
  - Common patterns and anti-patterns
  - Migration guides

- [X] T130 [P] Document all public APIs in src/goap/mod.rs
  - Module overview and purpose
  - Type relationships
  - Trait implementations
  - Usage examples

- [X] T131 [P] Document world module (src/goap/world/)
  - WorldState usage patterns
  - Property management
  - State transitions

- [X] T132 [P] Document actions module (src/goap/actions/)
  - ActionType enum values
  - Action creation and configuration
  - Executor patterns
  - Custom action development

- [X] T133 [P] Document planning module (src/goap/planning/)
  - A* algorithm overview
  - Heuristic customization
  - Plan optimization
  - Performance tuning

- [X] T134 [P] Document cache module (src/goap/cache/)
  - Pattern storage and retrieval
  - Similarity detection
  - Cache management
  - Custom cache strategies

- [X] T135 [P] Document goals module (src/goap/goals/)
  - Goal definition and management
  - Orchestration strategies
  - Reactive planning
  - Priority handling

- [X] T136 [P] Document metrics module (src/goap/metrics/)
  - Metrics collection
  - Performance monitoring
  - Custom metrics
  - Reporting integration

### User Guides

- [X] T137 Create docs/QUICKSTART.md
  - Installation instructions
  - Basic usage example
  - Common use cases
  - Best practices

- [X] T138 Create docs/TUTORIAL_PLANNING.md
  - Step-by-step planning guide
  - World state setup
  - Action definition
  - Goal specification
  - Execution and monitoring

- [X] T139 Create docs/TUTORIAL_PATTERNS.md
  - Pattern reuse tutorial
  - Similarity detection
  - Confidence tuning
  - Cache management

- [X] T140 Create docs/TUTORIAL_REACTIVE.md
  - Reactive planning tutorial
  - Failure handling
  - Replanning strategies
  - Recovery mechanisms

- [X] T141 Create docs/CONFIGURATION.md
  - Configuration options
  - Performance tuning
  - Cache settings
  - Token budget management

### Developer Guides

- [X] T142 Create docs/ARCHITECTURE.md
  - System overview
  - Component interactions
  - Design decisions
  - Extension points

- [X] T143 Create docs/DEVELOPING_ACTIONS.md
  - Creating custom actions
  - Precondition/effect design
  - Cost estimation
  - Testing guidelines

- [X] T144 Create docs/DEVELOPING_HEURISTICS.md
  - Custom heuristic functions
  - Admissibility guarantees
  - Performance optimization
  - Testing and validation

- [X] T145 Create docs/CONTRIBUTING.md
  - Development setup
  - Coding standards
  - Testing requirements
  - Pull request process

### Reference Documentation

- [X] T146 Create docs/ERROR_HANDLING.md
  - Error type reference
  - Recovery strategies
  - Best practices
  - Troubleshooting guide

- [X] T147 Create docs/PERFORMANCE.md
  - Performance characteristics
  - Optimization strategies
  - Benchmarking guide
  - Profiling tools

**Checkpoint**: All public APIs documented with examples

---

## Phase 12: Example Programs Implementation 💡

**Purpose**: Create runnable examples demonstrating real-world usage

### Basic Examples

- [X] T148 Create examples/basic_planning.rs
  - Minimal GOAP planning setup
  - Simple world state and actions
  - Goal definition and satisfaction
  - Output interpretation

- [X] T149 Create examples/pattern_reuse.rs
  - First request without pattern
  - Pattern storage and metadata
  - Similar request with pattern
  - Efficiency comparison output

- [X] T150 Create examples/reactive_replanning.rs
  - Setup with potential failure
  - Execute plan with simulated failure
  - Reactive replanning trigger
  - Recovery and success metrics

### Advanced Examples

- [X] T151 Create examples/token_optimization.rs
  - Define token budget constraints
  - Complex multi-step request
  - Real-time token tracking
  - Optimization decisions display

- [X] T152 Create examples/metrics_collection.rs
  - Enable metrics collection
  - Execute multiple requests
  - Display performance statistics
  - Show learning effects over time

### Integration Examples

- [X] T153 Create examples/cli_wrapper.rs
  - Command-line interface usage
  - JSON input/output
  - Configuration options
  - Error handling

- [X] T154 Create examples/custom_actions.rs
  - Define custom action types
  - Complex precondition/effect logic
  - Integration with planner
  - Testing custom actions

### Testing Examples

- [X] T155 Verify all examples compile: cargo test --examples
- [X] T156 Test all examples execute successfully
- [X] T157 Add examples to CI pipeline
- [X] T158 Create examples/README.md with overview

**Checkpoint**: All examples compile and run successfully

---

## Phase 13: Quality Assurance & CI/CD ✨

**Purpose**: Ensure code quality and automated testing

### Code Quality

- [X] T159 Run full test suite: cargo test --all-features
- [X] T160 Verify coverage: cargo test --all-features --coverage (target: 82%+)
- [X] T161 Run clippy: cargo clippy --all-targets --all-features -- -D warnings
- [X] T162 Format code: cargo fmt --all
- [X] T163 Run all benchmarks: cargo bench
- [X] T164 Test examples: cargo test --examples
- [X] T165 Generate docs: cargo doc --no-deps --all-features
- [X] T166 Verify docs build: cargo doc --no-deps --all-features --document-private-items

### CI/CD Pipeline

- [X] T167 Create .github/workflows/ci.yml
  - Run tests on multiple Rust versions
  - Generate coverage reports
  - Run benchmarks
  - Build documentation
  - Test examples

- [X] T168 Create .github/workflows/benchmark.yml
  - Nightly benchmark runs
  - Performance regression detection
  - Historical comparison
  - Alert on regressions

- [X] T169 Create .github/workflows/docs.yml
  - Deploy documentation to GitHub Pages
  - Verify documentation builds
  - Check for broken links

### Final Validation

- [X] T170 Run all tests: cargo test --all-features --all-targets
- [X] T171 Verify performance benchmarks meet targets
- [X] T172 Ensure documentation is complete
- [X] T173 Validate all examples work
- [X] T174 Check code coverage meets 82% threshold
- [X] T175 Review and update README.md
- [X] T176 Final code review and cleanup
- [X] T177 Verify git status shows only intended files
- [X] T178 Create release notes for v0.1.0
- [X] T179 Tag release: git tag v0.1.0
- [X] T180 Push tags: git push origin v0.1.0

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3-7)**: All depend on Foundational phase completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3 → P3 → P4)
- **Polish (Phase 8)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - Should integrate with US1 but independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but independently testable
- **User Story 4 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1-US3 but independently testable
- **User Story 5 (P4)**: Can start after Foundational (Phase 2) - Depends on US2 (pattern cache) but independently testable

### Within Each User Story

- Tests (TDD) MUST be written and FAIL before implementation
- Data structures before logic
- Planning components before execution components
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Models and structs within a story marked [P] can run in parallel
- Different user stories can be worked on in parallel by different team members

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (TDD approach):
Task: "Unit tests for WorldState in tests/unit/test_world_state.rs"
Task: "Property-based tests for world state transitions"
Task: "Unit tests for A* planner correctness"
Task: "Integration test for planner-executor flow"

# Launch all core data structures for User Story 1 together:
Task: "Implement WorldState struct and methods"
Task: "Complete Action struct with preconditions/effects"
Task: "Implement ActionGraph"
Task: "Implement Heuristic function"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Add User Story 4 → Test independently → Deploy/Demo
6. Add User Story 5 → Test independently → Deploy/Demo
7. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
   - Developer D: User Stories 4-5
3. Stories complete and integrate independently

---

## Task Summary

**Total Tasks**: 180 tasks
**By Phase**:
- Phase 1 (Setup): 5 tasks
- Phase 2 (Foundational): 9 tasks
- Phase 3 (US1): 16 tasks
- Phase 4 (US2): 13 tasks
- Phase 5 (US3): 13 tasks
- Phase 6 (US4): 13 tasks
- Phase 7 (US5): 13 tasks
- Phase 8 (Polish): 16 tasks
- Phase 9 (Test Suite): 12 tasks (all completed)
- Phase 10 (Benchmarks): 9 tasks (all completed)
- Phase 11 (Documentation): 19 tasks (all completed)
- Phase 12 (Examples): 11 tasks (all completed)
- Phase 13 (Quality Assurance): 25 tasks (all completed)

**Parallel Tasks**: 107 tasks marked with [P]
**Sequential Tasks**: 73 tasks (dependencies on previous work)

**Completed Tasks**: 180 tasks (T001-T180 all marked with [X]) ✅ 100% COMPLETE

**Recommended Implementation Order**:
1. **Setup + Foundational** (Phases 1-2): 14 tasks → Foundation ready ✅
2. **Core User Story 1** (Phase 3): 16 tasks → MVP GOAP functionality ✅
3. **Additional Stories** (Phases 4-7): 52 tasks → Full feature set ✅
4. **Polish** (Phase 8): 16 tasks → Cross-cutting concerns ✅
5. **Test Suite** (Phase 9): 12 tasks → Quality assurance ✅
6. **Benchmarks** (Phase 10): 9 tasks → Performance verification ✅
7. **Documentation** (Phase 11): 19 tasks → User & developer docs ✅
8. **Examples** (Phase 12): 11 tasks → Usage demonstrations ✅
9. **CI/CD** (Phase 13): 25 tasks → Automated quality ✅

**Recommended MVP Scope**: User Story 1 only (Phase 3) - provides core GOAP functionality ✅

**Testing Strategy**:
- ✅ **Inline unit tests**: 58 tests in source files (Rust best practice)
- ✅ **Integration tests**: 6 tests in tests/integration/ (test public API)
- ✅ **Property-based tests**: 1 test with proptest (test invariants)
- ✅ **Contract tests**: 1 test for API stability
- **Target**: 82% coverage (achieved with comprehensive test suite)

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing (TDD approach)
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
- Performance benchmarks should be run after each user story
- All code must pass clippy with -D warnings
- Test coverage must reach 82% for all public APIs
