# Contributing to GOAP

Thank you for your interest in contributing! This guide will help you get started.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Documentation](#documentation)
- [Submitting Changes](#submitting-changes)
- [Review Process](#review-process)

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## Getting Started

### Ways to Contribute

1. **Bug Reports**: Report bugs through GitHub Issues
2. **Feature Requests**: Suggest new features or improvements
3. **Code Contributions**: Fix bugs, add features, improve performance
4. **Documentation**: Improve docs, tutorials, examples
5. **Tests**: Add or improve test coverage
6. **Reviews**: Help review pull requests

### Development Setup

1. **Fork the Repository**
   ```bash
   git clone https://github.com/yourusername/goap-driven-llm-rust.git
   cd goap-driven-llm-rust
   ```

2. **Add Upstream Remote**
   ```bash
   git remote add upstream https://github.com/original/goap-driven-llm-rust.git
   ```

3. **Install Rust**
   ```bash
   # Install Rust toolchain
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env

   # Verify installation
   rustc --version  # Should be 1.91+
   ```

4. **Install Additional Tools**
   ```bash
   # Install development tools
   rustup component add rustfmt clippy llvm-tools-preview

   # Install cargo plugins
   cargo install cargo-watch cargo-audit cargo-expand
   ```

5. **Build Project**
   ```bash
   cargo build
   cargo test
   ```

## Development Setup

### Project Structure

```
goap-driven-llm-rust/
├── src/
│   ├── goap/              # Core GOAP modules
│   │   ├── actions/       # Action definitions
│   │   ├── cache/         # Pattern caching
│   │   ├── goals/         # Goal management
│   │   ├── planning/      # A* planning
│   │   ├── metrics/       # Metrics collection
│   │   └── world/         # World state
│   ├── cli/               # CLI interface
│   ├── error.rs           # Error types
│   └── lib.rs             # Library root
├── tests/                 # Test suite
│   ├── integration/       # Integration tests
│   ├── unit/             # Unit tests
│   ├── fixtures/         # Test fixtures
│   └── contract/         # API contract tests
├── benches/              # Performance benchmarks
├── examples/             # Usage examples
├── docs/                 # Documentation
└── specs/                # Specifications
```

### Recommended Workflow

1. Create a feature branch
   ```bash
   git checkout -b feature/your-feature-name
   ```

2. Make changes
   - Follow coding standards
   - Write tests
   - Update documentation

3. Run quality checks
   ```bash
   cargo fmt --all
   cargo clippy --all-targets --all-features -- -D warnings
   cargo test --all-features
   cargo bench
   ```

4. Commit changes
   ```bash
   git add .
   git commit -m "feat: add feature description"
   ```

5. Push and create PR
   ```bash
   git push origin feature/your-feature-name
   ```

## Coding Standards

### Rust Style Guide

We follow standard Rust formatting:

```rust
// Use rustfmt for formatting
cargo fmt --all

// Check formatting
cargo fmt --all -- --check
```

### Naming Conventions

```rust
// Types: PascalCase
pub struct WorldState;
pub enum ActionType;
pub struct GOAPSystem;

// Functions and variables: snake_case
pub fn process_request()
let world_state = WorldState::new();

// Constants: SCREAMING_SNAKE_CASE
const MAX_PLAN_DEPTH: u32 = 20;
const DEFAULT_TOKEN_BUDGET: u32 = 5000;

// Modules: snake_case
mod planning;
mod actions;
```

### Error Handling

```rust
// Use custom error types
#[derive(Error, Debug)]
pub enum Error {
    #[error("Planning error: {0}")]
    Planning(#[from] PlanningError),

    #[error("Execution error: {0}")]
    Execution(#[from] ExecutionError),
}

// Use Result<T, E>
fn risky_operation() -> Result<String, Error> {
    // Use ? for error propagation
    let data = read_file("config.txt")?;
    Ok(data)
}
```

### Async Code

```rust
// Use async/await consistently
#[tokio::main]
async fn main() -> Result<(), Error> {
    let result = async_operation().await?;
    Ok(())
}

// Document Send/Sync bounds
fn concurrent_processing<T: Send + Sync>(items: Vec<T>) {
    // Implementation
}
```

### Documentation

```rust
/// Public-facing API with comprehensive documentation
///
/// # Arguments
///
/// * `request` - The request to process
/// * `budget` - Token budget for the request
///
/// # Returns
///
/// Returns `Ok(response)` on success, or `Err(Error)` on failure.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let system = GOAPSystem::new();
/// let result = system.process_request(request, 5000);
/// assert!(result.is_ok());
/// ```
pub fn process_request(request: &str, budget: u32) -> Result<Response, Error> {
    // Implementation
}
```

## Testing Requirements

### Test Coverage

- **Target**: 82%+ code coverage
- **Unit Tests**: Test private functions inline
- **Integration Tests**: Test public API in `tests/`
- **Property-Based Tests**: Test invariants with proptest

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[tokio::test]
    async fn test_async_function() {
        // Test async functions
        let result = async_function().await;
        assert!(result.is_ok());
    }

    // Property-based tests
    #[test]
    fn test_invariant(prop in any::<String>()) {
        // Test invariants across many inputs
        let result = process_input(prop);
        assert!(is_valid(result));
    }
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test module
cargo test module_name

# Run tests with output
cargo test -- --nocapture

# Run doctests
cargo test --doc

# Run integration tests
cargo test --test integration_test_name

# Check coverage
cargo install cargo-tarpaulin
cargo tarpaulin --out html
```

### Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_function(c: &mut Criterion) {
    c.bench_function("my_function", |b| {
        b.iter(|| {
            let result = my_function(black_box(input));
            black_box(result)
        })
    });
}

criterion_group!(benches, benchmark_function);
criterion_main!(benches);
```

Run benchmarks:
```bash
cargo bench
```

## Documentation

### Types of Documentation

1. **API Documentation**: Rustdoc in source code
2. **User Guides**: Tutorials and how-tos in `docs/`
3. **Examples**: Runnable examples in `examples/`
4. **README**: Project overview

### Writing Documentation

```markdown
# Documentation Title

Brief description of the feature or concept.

## Overview

Detailed explanation with context.

## Usage Examples

```rust
// Code examples
let result = do_something();
```

## Best Practices

- Tip 1
- Tip 2

## Troubleshooting

Common issues and solutions.
```

### Building Docs

```bash
# Generate documentation
cargo doc --no-deps --all-features

# Build with private items
cargo doc --no-deps --all-features --document-private-items

# Serve locally
cargo doc --no-deps --open
```

## Submitting Changes

### Pull Request Process

1. **Create Descriptive PR**
   - Clear title
   - Detailed description
   - Link related issues

2. **PR Template**

   ```markdown
   ## Description
   Brief description of changes

   ## Type of Change
   - [ ] Bug fix
   - [ ] New feature
   - [ ] Breaking change
   - [ ] Documentation update

   ## Testing
   - [ ] Added tests
   - [ ] All tests pass
   - [ ] Added integration tests

   ## Checklist
   - [ ] Code follows style guide
   - [ ] Self-reviewed code
   - [ ] Comments added for complex code
   - [ ] Documentation updated
   - [ ] No new clippy warnings
   ```

3. **Maintain Linear History**
   ```bash
   # Use rebase, not merge
   git rebase upstream/main
   ```

### Commit Message Format

```
type(scope): short description

Longer description if needed.

Breaking changes or other notes.
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes
- `refactor`: Code refactoring
- `test`: Adding tests
- `chore`: Maintenance tasks

**Examples**:
```
feat(planner): add support for custom heuristics

Implement Heuristic trait allowing custom cost functions.
Update A* planner to use custom heuristics.

Closes #123
```

```
fix(cache): handle cache misses gracefully

When pattern cache returns None, fall back to full planning
instead of panicking.

Fixes #456
```

## Review Process

### Code Review Checklist

**Functionality**:
- [ ] Code does what it claims
- [ ] Edge cases handled
- [ ] Error handling appropriate
- [ ] No panics or unwraps

**Quality**:
- [ ] Code is readable and clear
- [ ] Comments explain why, not what
- [ ] Names are descriptive
- [ ] No duplication

**Testing**:
- [ ] Tests cover the changes
- [ ] Tests are meaningful
- [ ] Integration tests included
- [ ] Edge cases tested

**Performance**:
- [ ] No obvious performance issues
- [ ] Appropriate algorithms used
- [ ] Memory usage reasonable

**Documentation**:
- [ ] API documented
- [ ] Examples included
- [ ] README updated if needed

### Reviewing Code

1. **Be Respectful**: Provide constructive feedback
2. **Be Specific**: Point to exact issues with examples
3. **Be Thorough**: Check all aspects of the change
4. **Be Timely**: Review promptly

```markdown
## Review Comments

### Major Issues

The cache lookup on line 45 could panic if the key doesn't exist. Consider using
`.get()` instead of direct indexing.

```rust
// Current (problematic)
let value = cache[key];

// Better
if let Some(value) = cache.get(key) {
    // Use value
} else {
    // Handle missing case
}
```

### Suggestions

Consider adding a unit test for the error case in `process_request` to ensure
it's handled correctly.

### Nits

- Line 23: Missing space after comma
- Line 67: Type annotation is redundant here

Overall looks good! Just fix the panic issue and I'll approve.
```

### Addressing Feedback

1. **Respond to All Comments**: Acknowledge each piece of feedback
2. **Make Changes**: Update code as requested
3. **Ask Questions**: Clarify if feedback is unclear
4. **Explain Decisions**: If you disagree, explain why

```markdown
Thanks for the review!

**Addressed Issues**:
- Fixed panic on line 45 ✅
- Added unit test for error case ✅
- Fixed formatting nits ✅

**Regarding Cache Structure**:
I kept the HashMap structure because:
1. Simpler for this use case (no pattern matching needed)
2. Better performance for exact lookups
3. Can be refactored later if needed

Happy to discuss further if you feel strongly about it!
```

## Release Process

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):
- `MAJOR.MINOR.PATCH`
- `MAJOR`: Breaking changes
- `MINOR`: New features (backward compatible)
- `PATCH`: Bug fixes (backward compatible)

### Release Checklist

- [ ] Update version in `Cargo.toml`
- [ ] Update `CHANGELOG.md`
- [ ] Run full test suite
- [ ] Run benchmarks
- [ ] Create release PR
- [ ] Tag release: `git tag v0.1.0`
- [ ] Push tags: `git push origin v0.1.0`
- [ ] Create GitHub release

## Getting Help

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests
- **GitHub Discussions**: General questions, ideas
- **Documentation**: Check `docs/` directory

### Asking Questions

When asking for help:

1. **Search First**: Check docs, issues, discussions
2. **Provide Context**: What are you trying to do?
3. **Show Code**: Include relevant code snippets
4. **Include Errors**: Paste full error messages
5. **Environment**: OS, Rust version, GOAP version

Example:
```
I'm trying to create a custom action but it never executes.

Rust version: 1.91
GOAP version: 0.1.0
OS: Ubuntu 22.04

Code:
```rust
let action = Action::new(ActionType::MyCustomAction);
let result = system.process_request(&mut state, vec![action], goals).await;
```

Error:
```
Error: No valid path found to goal
```

What I've tried:
- Added preconditions
- Checked world state
- Read docs

Expected: Action should execute
Actual: No path found

What am I missing?
```

## Recognition

Contributors are recognized in:
- `AUTHORS.md` file
- Release notes
- Contributors page on website

Thank you for contributing! 🎉
