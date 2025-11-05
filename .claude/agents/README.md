# Claude Code Agents

This directory contains Claude Code sub-agents that provide specialized expertise for various development tasks. Each agent is designed to handle specific workflows and can be triggered automatically based on the task context.

## Agent Structure

Each agent is a markdown file with YAML frontmatter containing:
- **name**: Unique identifier for the agent (in hyphen-case)
- **description**: What the agent does and when to use it
- **trigger**: Keywords or patterns that activate the agent

The markdown content describes the agent's expertise, capabilities, usage patterns, and implementation guidelines.

## Available Agents

### 1. Git Pushing Agent
**File:** `git-pushing-agent.md`

Automates Git workflows with conventional commit messages. Stages all changes, creates properly formatted commits with Claude Code attribution, and pushes to remote branches.

**Triggers:**
- "push changes", "commit and push"
- "push to github", "save to github"
- "git push", "conventional commit"

### 2. MCP Builder Agent
**File:** `mcp-builder-agent.md`

Guides development of high-quality MCP (Model Context Protocol) servers. Provides comprehensive 4-phase workflow from research to evaluation, supporting Python (FastMCP) and TypeScript implementations.

**Triggers:**
- "build mcp server"
- "create mcp integration"
- "mcp protocol", "model context protocol"
- "mcp server development"

### 3. Rust Clean Code Agent
**File:** `rust-clean-code-agent.md`

Enforces Rust best practices and clean code principles. Guides code organization, error handling with Result/thiserror/anyhow, async patterns with Tokio, testing with tokio-test and mockall.

**Triggers:**
- "rust code review"
- "clean rust code"
- "rust best practices"
- "refactor rust", "rust error handling"
- "rust testing"

### 4. Skill Creator Agent
**File:** `skill-creator-agent.md`

Guides the creation of new Claude skills following best practices. Provides templates, validation tools, and step-by-step workflows for skill development.

**Triggers:**
- "create skill", "new skill"
- "skill template", "build skill"
- "skill development"

### 5. Theme Factory Agent
**File:** `theme-factory-agent.md`

Creates and applies professional themes to artifacts and documents. Provides 10 pre-set themes (Arctic Frost, Botanical Garden, Desert Rose, etc.) and custom theme generation.

**Triggers:**
- "create theme", "apply theme"
- "style document"
- "custom theme", "theme design"

### 6. Verification Agent
**File:** `verification-agent.md`

Implements comprehensive verification workflows before task completion. Ensures all requirements are met, tests pass, documentation is complete, and code quality standards are followed.

**Triggers:**
- "verify before completion"
- "quality check"
- "pre-completion verification"
- "ensure completion"
- "final verification"

### 7. Web App Testing Agent
**File:** `webapp-testing-agent.md`

Provides comprehensive toolkit for testing local web applications using Playwright. Supports verifying frontend functionality, debugging UI behavior, capturing screenshots, and viewing browser logs.

**Triggers:**
- "test webapp"
- "automate browser"
- "playwright testing"
- "web application testing"
- "ui automation"
- "browser testing"

## GOAP-Specific Specialist Agents

### 8. GOAP Planning Specialist Agent
**File:** `goap-planning-specialist-agent.md`

Expert in GOAP (Goal-Oriented Action Planning) system development, A* search algorithms, world state management, action planning, and strategic reasoning for LLM systems. Use when implementing GOAP planners, heuristic functions, action graphs, or reactive replanning systems.

**Triggers:**
- "goap planning"
- "goal-oriented action planning"
- "a* search"
- "action planner"
- "strategic reasoning"
- "world state"
- "action graph"
- "reactive replanning"
- "heuristic function"
- "plan optimization"

### 9. LLM Integration Specialist Agent
**File:** `llm-integration-specialist-agent.md`

Expert in LLM (Large Language Model) integration, token optimization, prompt engineering, schema validation, compression techniques, and cost management for AI systems. Use when implementing LLM clients, token tracking, response generation, or optimization strategies.

**Triggers:**
- "llm integration"
- "token optimization"
- "prompt engineering"
- "schema validation"
- "llm client"
- "token tracking"
- "response generation"
- "compression"
- "cost management"
- "ai system"

### 10. Async Rust Performance Agent
**File:** `async-rust-performance-agent.md`

Expert in Rust async/await patterns, Tokio runtime optimization, concurrent data structures (DashMap, LRU), single-threaded async performance, and efficient I/O operations. Use when optimizing async code, managing concurrent access, or tuning Tokio performance for GOAP systems.

**Triggers:**
- "async rust"
- "tokio performance"
- "concurrent data structures"
- "async optimization"
- "tokio runtime"
- "dashmap"
- "lru cache"
- "async I/O"
- "single-threaded async"
- "performance tuning"

### 11. Benchmarking Specialist Agent
**File:** `benchmarking-specialist-agent.md`

Expert in Rust benchmarking with Criterion, performance testing, regression detection, throughput measurement, and automated performance validation. Use when setting up benchmarks, measuring performance, analyzing bottlenecks, or creating performance reports for GOAP systems.

**Triggers:**
- "benchmarking"
- "performance testing"
- "criterion"
- "throughput"
- "regression detection"
- "performance analysis"
- "load testing"
- "profiling"
- "performance metrics"

### 12. redb Database Specialist Agent
**File:** `redb-database-specialist-agent.md`

Expert in redb embedded database, ACID transactions, data persistence, pattern storage, schema caching, and skill-scoped isolation for GOAP systems. Use when implementing redb operations, managing data persistence, optimizing database performance, or handling concurrent access to pattern caches.

**Triggers:**
- "redb database"
- "embedded database"
- "data persistence"
- "acidity transactions"
- "pattern storage"
- "schema cache"
- "database optimization"
- "concurrent access"
- "skill isolation"
- "redb operations"

### 13. Testing Specialist Agent
**File:** `testing-specialist-agent.md`

Expert in Rust testing strategies, tokio-test for async tests, mockall for mocking, proptest for property-based testing, integration testing, and comprehensive test coverage for GOAP systems. Use when writing unit tests, integration tests, property-based tests, or achieving 82% code coverage requirements.

**Triggers:**
- "rust testing"
- "unit tests"
- "integration tests"
- "property-based testing"
- "tokio-test"
- "mockall"
- "proptest"
- "test coverage"
- "async testing"
- "mocking"

### 14. Documentation Specialist Agent
**File:** `documentation-specialist-agent.md`

Expert in Rust API documentation (rustdoc), markdown guides, README files, architectural documentation, user tutorials, and comprehensive documentation systems. Use when creating API docs, user guides, developer documentation, or improving documentation quality for GOAP systems.

**Triggers:**
- "documentation"
- "rustdoc"
- "api documentation"
- "user guide"
- "developer guide"
- "readme"
- "markdown"
- "tutorial"
- "architectural documentation"

### 15. Examples/CLI Specialist Agent
**File:** `examples-cli-specialist-agent.md`

Expert in creating Rust CLI applications, example programs, command-line interfaces, JSON I/O, configuration management, and user-friendly CLI tools for GOAP systems. Use when building CLI wrappers, creating runnable examples, or implementing command-line interfaces.

**Triggers:**
- "cli application"
- "command line interface"
- "example programs"
- "clap"
- "argparse"
- "json i/o"
- "configuration"
- "cli design"
- "command-line tool"

## Agent Activation

Agents activate automatically when you mention relevant keywords or request specific workflows. They provide:

1. **Specialized Expertise**: Deep knowledge in their domain
2. **Best Practices**: Industry-standard approaches and patterns
3. **Tool Integration**: Seamless use with existing development tools
4. **Quality Assurance**: Built-in verification and validation
5. **Documentation**: Comprehensive guides and examples

## Usage

Simply mention your task naturally. For example:
- "Commit these changes" → Git Pushing Agent activates
- "Help me test my webapp" → Web App Testing Agent activates
- "Create a new skill" → Skill Creator Agent activates

Agents will:
1. Analyze the task requirements
2. Apply domain-specific expertise
3. Follow established best practices
4. Provide comprehensive solutions
5. Ensure quality and completeness

## Development

To add new agents:
1. Create a new markdown file with `-agent.md` suffix
2. Include YAML frontmatter with name, description, and triggers
3. Add comprehensive documentation and examples
4. Follow the established pattern from existing agents

## License

These agents are derived from the Claude Skills framework and are available under the Apache 2.0 License. See individual skill licenses for specific terms.