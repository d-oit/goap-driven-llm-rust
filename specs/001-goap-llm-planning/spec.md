# Feature Specification: GOAP-Driven LLM Strategic Reasoning

**Feature Branch**: `001-goap-llm-planning`
**Created**: 2025-11-03
**Status**: Draft
**Input**: Complete GOAP-Driven Clever LLM Architecture (Rust)

## Clarifications

### Session 2025-11-03

- Q: Security & Privacy Requirements → A: Used as Claude Code skill/MCP, no external API keys needed, integrated directly into Claude context

- Q: Data Persistence & Learning → A: Use redb embedded database with skill-scoped isolation (patterns shared within GOAP skill across all users, logically separated from other skills)

- Q: Runtime Model & Concurrency → A: Single-threaded with async await (suitable for Claude integration, one request at a time)

- Q: Goal Representation Format → A: Struct-based with HashMap properties, stored in redb database with JSON serialization (consistent storage layer, flexible goal specification)

- Q: A* Search Heuristic → A: Cost-based with token权重 (weighted combination of token cost + execution time + success probability)

## User Scenarios & Testing *(mandatory)*

<!--
  IMPORTANT: User stories should be PRIORITIZED as user journeys ordered by importance.
  Each user story/journey must be INDEPENDENTLY TESTABLE - meaning if you implement just ONE of them,
  you should still have a viable MVP (Minimum Viable Product) that delivers value.

  Assign priorities (P1, P2, P3, etc.) to each story, where P1 is the most critical.
  Think of each story as a standalone slice of functionality that can be:
  - Developed independently
  - Tested independently
  - Deployed independently
  - Demonstrated to users independently
-->

### User Story 1 - GOAP Planning and Execution (Priority: P1)

A user submits a request to the LLM system (e.g., "Create a GitHub Actions workflow for Node.js"). The system analyzes the request, creates an optimal action plan using A* search, and executes the plan step-by-step to generate a valid response.

**Why this priority**: This is the core GOAP functionality that transforms the system from reactive to proactive. Without this, the system cannot perform strategic planning or optimize its actions.

**Independent Test**: Can be fully tested by submitting a single request and verifying that:
- A plan is generated with multiple ordered actions
- Actions are executed sequentially
- A valid response is produced
- The system tracks world state changes throughout execution

**Acceptance Scenarios**:

1. **Given** a user request for code generation, **When** the system processes it, **Then** it creates a world state with properties (schema available, token budget, etc.), selects appropriate goals, and generates an action plan via A* search

2. **Given** an action plan with multiple steps, **When** each action is executed, **Then** the system validates preconditions before execution, applies effects to update world state, and tracks execution results

3. **Given** successful plan execution, **When** the system completes all actions, **Then** it produces a validated response that satisfies all goal requirements

---

### User Story 2 - Pattern Reuse for Efficiency (Priority: P2)

When a user submits a request similar to one processed successfully before, the system detects the similarity, retrieves the cached pattern, and reuses the successful approach to generate a response faster and with fewer tokens.

**Why this priority**: Pattern reuse is a key efficiency driver that reduces cost and improves performance. It's one of the main benefits of the GOAP approach - learning from past successes.

**Independent Test**: Can be fully tested by:
- Processing a request successfully (creating a pattern)
- Submitting a similar request
- Verifying the pattern is detected and reused
- Confirming reduced token usage and faster response

**Acceptance Scenarios**:

1. **Given** a previously successful request pattern, **When** a similar request arrives, **Then** the system detects high pattern confidence (>70%) and prioritizes pattern reuse over full generation

2. **Given** pattern reuse is selected, **When** the response is generated, **Then** token usage is significantly lower (150 vs 600 tokens) and response time is faster (500ms vs 3000ms)

3. **Given** pattern reuse produces a valid response, **When** it satisfies the request, **Then** the system updates pattern confidence and usage statistics

---

### User Story 3 - Reactive Replanning on Failure (Priority: P3)

When plan execution fails or world state changes unexpectedly, the system automatically detects the deviation, triggers reactive replanning, and generates a new action sequence to achieve the goal through alternative means.

**Why this priority**: Real-world systems encounter failures and changing conditions. The ability to adapt and replan is critical for robust operation and differentiates GOAP from rigid linear processing.

**Independent Test**: Can be fully tested by:
- Simulating a failure during plan execution
- Verifying the system detects the failure
- Confirming a new plan is generated
- Validating the new plan achieves the original goal

**Acceptance Scenarios**:

1. **Given** plan execution in progress, **When** an action fails or token budget becomes critically low (<100 tokens), **Then** the system triggers reactive replanning and generates an alternative action sequence

2. **Given** reactive replanning is triggered, **When** a new plan is generated, **Then** it achieves the same goal with adjusted constraints (e.g., using pattern reuse instead of full generation)

3. **Given** the new plan succeeds, **When** the goal is achieved, **Then** the system records the replan event and updates heuristics for future planning

---

### User Story 4 - Token Budget Optimization (Priority: P3)

The system continuously monitors token consumption throughout plan execution, applies compression techniques when appropriate, and makes real-time decisions to minimize token usage while maintaining response quality.

**Why this priority**: Token costs are a primary concern for LLM systems. GOAP's ability to optimize token usage through smart planning provides direct business value through cost reduction.

**Independent Test**: Can be fully tested by:
- Setting a token budget constraint
- Processing requests with varying complexity
- Verifying the system stays within budget
- Confirming response quality remains acceptable

**Acceptance Scenarios**:

1. **Given** a token budget constraint (e.g., 5000 tokens), **When** planning actions, **Then** the system accounts for token costs in action selection and prefers low-cost actions when possible

2. **Given** token budget monitoring during execution, **When** consumption approaches limits, **Then** the system applies compression or switches to efficient patterns to stay within budget

3. **Given** successful completion within budget, **When** metrics are recorded, **Then** token efficiency metrics are updated and used to optimize future planning

---

### User Story 5 - Learning from Success Patterns (Priority: P4)

After successfully completing a request, the system analyzes the execution path, extracts the successful pattern, and stores it in the cache with associated metadata for future reuse and optimization.

**Why this priority**: Continuous learning enables the system to improve over time, making each subsequent request more efficient. This creates compounding value as the pattern library grows.

**Independent Test**: Can be fully tested by:
- Processing multiple successful requests
- Verifying patterns are extracted and stored
- Checking pattern metadata (confidence, success rate, etc.)
- Confirming patterns are available for future reuse

**Acceptance Scenarios**:

1. **Given** successful plan execution, **When** the response is validated, **Then** the system extracts the successful action sequence and stores it as a learnable pattern

2. **Given** a new pattern is stored, **When** metadata is recorded, **Then** it includes success confidence, token usage, execution time, and goal satisfaction details

3. **Given** pattern statistics accumulate, **When** making future planning decisions, **Then** the system uses this data to adjust action costs and success probability estimates

---

### Edge Cases

- What happens when no valid plan can be found (goal unreachable)?
- How does the system handle when all actions fail (system cannot recover)?
- What occurs when token budget is exceeded mid-execution?
- How does the system behave when pattern cache is empty?
- What happens during concurrent requests with shared resources?
- How does the system handle schema fetch failures?
- What occurs when validation repeatedly fails (unrecoverable errors)?
- How does the system prioritize multiple competing goals?

## Requirements *(mandatory)*

<!--
  ACTION REQUIRED: The content in this section represents placeholders.
  Fill them out with the right functional requirements.
-->

### Functional Requirements

- **FR-001**: System MUST define and maintain a world state that tracks schema availability, pattern cache, token budget, and request properties throughout execution

- **FR-002**: System MUST support action definition with preconditions, effects, token costs, confidence levels, and duration estimates for each possible action type

- **FR-003**: System MUST implement A* search algorithm with cost-based heuristic (weighted combination of token cost + execution time + success probability) to find optimal action sequences that achieve specified goals while minimizing composite cost

- **FR-004**: System MUST execute action plans sequentially, validating preconditions before each action and applying effects to update world state

- **FR-005**: System MUST support reactive replanning when execution fails, token budget is exceeded, or world state diverges from expectations

- **FR-006**: System MUST detect and cache successful execution patterns using redb embedded database, storing them with confidence metrics for future reuse and optimization within the GOAP skill scope

- **FR-007**: System MUST prioritize pattern reuse when high-confidence patterns are available, reducing token usage and response time

- **FR-008**: System MUST monitor token consumption in real-time and apply compression or adjust plans when approaching budget limits

- **FR-009**: System MUST learn from successful executions by extracting patterns, updating confidence scores, and improving planning heuristics

- **FR-010**: System MUST validate responses against schemas and goals, providing post-execution feedback for learning and optimization

- **FR-011**: System MUST support multiple goal orchestration with dynamic priority adjustment based on execution results and resource constraints

- **FR-012**: System MUST handle action failures gracefully with recovery mechanisms, limiting retry attempts and triggering replanning when appropriate

### Key Entities

- **World State**: Represents the current system state including available schemas, cached patterns, token budget, and satisfied properties

- **Action**: Defines a single operation with preconditions (requirements), effects (state changes), token cost, confidence, and execution duration

- **Goal**: Represents objectives the system aims to achieve, stored in redb as JSON-serialized struct with required properties, priority level, and satisfaction criteria

- **Action Plan**: An ordered sequence of actions generated by A* search to transition from current world state to goal state

- **Success Pattern**: A cached representation of a successful execution path, persisted in redb database, including metadata on confidence, efficiency, and applicability

- **Execution Result**: Records the outcome of plan execution including success/failure, token usage, duration, and goal satisfaction

## Success Criteria *(mandatory)*

<!--
  ACTION REQUIRED: Define measurable success criteria.
  These must be technology-agnostic and measurable.
-->

### Measurable Outcomes

- **SC-001**: Users receive valid responses for 90%+ of requests without manual intervention or correction

- **SC-002**: System reduces token consumption by 50-70% when pattern reuse is applicable, compared to full generation

- **SC-003**: Average response time improves by 25-35% through optimized action ordering and pattern reuse

- **SC-004**: Pattern reuse achieves 85%+ confidence score on successfully matched patterns, validating detection accuracy

- **SC-005**: Reactive replanning successfully recovers from 82%+ of actionable failures without user-visible errors

- **SC-006**: System maintains goal satisfaction rate above 90% even when adjusting to failures or constraints

- **SC-007**: Token budget adherence exceeds 95%, staying within specified limits for all requests

- **SC-008**: Learning effectiveness improves pattern confidence by 10-15% per successful reuse cycle

- **SC-009**: Edge case handling succeeds for 85%+ of documented error scenarios without escalation

- **SC-010**: System successfully processes 10,000+ requests per hour with consistent performance and reliability
- **SC-011**: System achieves minimum 82% code coverage in unit tests for all public APIs, verified with `cargo test --all-features --coverage`

- **SC-012**: Integration tests achieve 100% coverage for GOAP planner functionality, reactive replanning, and pattern reuse flows, verified with `cargo test --all-features`

- **SC-013**: Edge case handling succeeds for 85%+ of documented error scenarios without escalation, verified with dedicated edge case test suite


