# GOAP-Driven LLM Rust Constitution

<!-- Sync Impact Report:
Version: 1.0.0 (NEW)
Modified: Initial creation with focus on code quality, testing standards, user experience, and performance
Added: 5 core principles, 2 additional sections, governance framework
Removed: N/A
Templates requiring updates: ✅ All templates aligned (plan-template.md, spec-template.md, tasks-template.md, commands/*.md)
Follow-up TODOs: None
-->

## Core Principles

### I. Code Quality Standards (NON-NEGOTIABLE)

All code MUST follow Rust best practices and maintain high quality standards. Code reviews are mandatory for all changes. Complexity MUST be justified and documented. The codebase MUST be self-documenting through clear naming conventions and extensive documentation. Every public API MUST have documentation. Clippy warnings MUST be resolved - the codebase compiles with `cargo clippy --all-targets --all-features -- -D warnings`. Code formatting MUST be consistent using `rustfmt`. Unsafe code blocks MUST be avoided unless absolutely necessary and MUST be thoroughly documented with safety guarantees. Error handling MUST use Result types with proper error propagation. Dependencies MUST be kept minimal and regularly audited for security and necessity.

**Rationale**: Rust's safety guarantees are only effective when the codebase adheres to best practices. High code quality prevents bugs, improves maintainability, and ensures system reliability especially critical for LLM/GOAP systems.

### II. Testing Standards (NON-NEGOTIABLE)

All features MUST be tested before implementation. Test-Driven Development (TDD) is STRONGLY RECOMMENDED. Unit tests MUST cover all public APIs with minimum 82% code coverage. Integration tests are REQUIRED for all system interactions, API boundaries, and GOAP planner functionality. Property-based testing MUST be used for critical algorithms and data transformations. Performance regression tests MUST be maintained for performance-critical paths. All tests MUST run with `cargo test --all-features`. Test suites MUST be deterministic and isolated. Test data MUST be generated or fixture-based, not hardcoded where possible. Fuzz testing MUST be implemented for parsing and input handling. Documentation examples MUST be tested as part of the test suite.

**Rationale**: GOAP and LLM systems involve complex state machines and algorithms. Comprehensive testing ensures correctness, prevents regressions, and validates that the planning algorithms work correctly across diverse scenarios.

### III. User Experience Consistency (NON-NEGOTIABLE)

All user-facing interfaces MUST be consistent in terminology, behavior, and error handling. The CLI interface MUST follow consistent patterns: single binary with subcommands, standardized flags (`--help`, `--version`, `--verbose`, `--quiet`), and clear error messages. JSON is the primary structured output format. Output formatting MUST be stable and versioned - breaking changes require MAJOR version bumps. Error messages MUST be actionable and user-friendly with clear guidance on resolution. All tools MUST support both interactive use and automation. Progress indicators and feedback MUST be provided for long-running operations. Configuration MUST use consistent patterns (env vars, config files, CLI flags). The API surface MUST be minimal yet complete - follow the Unix philosophy of doing one thing well.

**Rationale**: Consistent UX reduces cognitive load for users and makes the system predictable. For LLM/GOAP systems used in various contexts, predictable interfaces are essential for integration and automation.

### IV. Performance Requirements (NON-NEGOTIABLE)

The system MUST meet defined performance benchmarks and maintain them across releases. The planning algorithms MUST execute within O(n²) complexity for typical scenarios. Memory usage MUST be bounded and predictable - no unbounded growth. The system MUST handle concurrent operations safely using appropriate Rust concurrency primitives. Lazy evaluation and streaming MUST be used for large data processing. Performance profiling MUST be integrated into the development workflow. Cache layers MUST be implemented for expensive computations with proper invalidation strategies. The system MUST degrade gracefully under load. Benchmarks MUST be automated and tracked over time. Cold start times MUST be minimized through proper initialization strategies. Performance regression tests MUST fail builds that degrade beyond acceptable thresholds.

**Rationale**: LLM and GOAP systems are computationally intensive. Performance requirements ensure the system remains usable at scale and meets user expectations for response times and resource usage.

### V. Observability & Documentation (NON-NEGOTIABLE)

The system MUST provide comprehensive logging with structured logging (JSON format). Log levels MUST be used appropriately (ERROR for failures, WARN for concerning patterns, INFO for significant operations, DEBUG for detailed tracing). Metrics MUST be exposed for all critical operations (latency, throughput, error rates, resource usage). Tracing MUST be implemented for complex operations and cross-service interactions. Documentation MUST be kept in sync with code - TODOs are NOT allowed. Every public API MUST have comprehensive documentation including examples. Architecture decision records (ADRs) MUST document significant technical decisions. Performance characteristics MUST be documented for all public interfaces. The README MUST include quickstart examples, common use cases, and troubleshooting guides. Breaking changes MUST be documented in CHANGELOG with migration guides.

**Rationale**: Observability is critical for debugging complex GOAP algorithms and understanding LLM behavior. Documentation ensures the system is accessible and maintainable over time.

## Development Workflow

**All code contributions MUST follow this process**:
1. Feature specification approved (use spec template)
2. Implementation plan created (use plan template)
3. Test coverage achieved (minimum 82% unit, 100% integration for new features)
4. Performance benchmarks maintained or improved
5. Code review completed by at least one maintainer
6. Documentation updated to reflect changes
7. All CI/CD checks passing

**Quality Gates**:
- No clippy warnings (enforced via CI)
- Test coverage thresholds met (enforced via CI)
- Performance benchmarks pass (enforced via CI)
- Documentation completeness verified (manual review)

**Branch Strategy**:
- `main`: Production-ready code only
- `feature/*`: Feature development branches
- Release tags follow semantic versioning (MAJOR.MINOR.PATCH)

## Architecture & Design Constraints

**Core Constraints**:
- The system MUST be designed as a library-first architecture with optional CLI wrapper
- All components MUST be modular and independently testable
- Configuration-driven behavior over hardcoded logic
- Fail-fast on invalid inputs with descriptive errors
- Memory safety guaranteed through Rust's ownership model - NO unsafe code without explicit justification
- Concurrency-safe design using appropriate sync primitives (Arc, Mutex, channels, etc.)
- Backward compatibility maintained within MAJOR version bounds

**Technology Stack**:
- Language: Rust (latest stable)
- Testing: cargo test, criterion for benchmarking, proptest for property-based testing
- Documentation: rustdoc, mdbook for user guides
- Build: cargo (no custom build scripts unless necessary)

## Governance

**Constitution Authority**: This constitution supersedes all other development practices and decisions. Any conflicts between this constitution and other documentation MUST be resolved in favor of this document.

**Amendment Process**:
1. Propose changes via pull request with detailed justification
2. Update VERSION following semantic versioning rules (MAJOR for breaking principles, MINOR for additions, PATCH for clarifications)
3. Update LAST_AMENDED_DATE to current date
4. Review period: minimum 7 days for MINOR/PATCH, 14 days for MAJOR
5. Approval: requires at least 2 maintainers or 100% of current maintainers (whichever is smaller)
6. Migration plan: required for MAJOR changes with backward-incompatible impact

**Compliance Verification**:
- All code reviews MUST verify constitution compliance
- CI/CD pipeline MUST enforce quality gates defined in this constitution
- Performance regressions trigger alerts and blocking failures
- Documentation gaps trigger review failures
- Maintainers MAY NOT approve commits violating constitution without explicit variance request and approval

**Version Compatibility**:
- MAJOR versions MAY break backward compatibility but MUST provide migration guides
- MINOR versions add features while maintaining backward compatibility
- PATCH versions fix bugs without changing functionality
- All PRs MUST specify whether they constitute MAJOR/MINOR/PATCH changes

**Version**: 1.0.0 | **Ratified**: 2025-11-03 | **Last Amended**: 2025-11-03
