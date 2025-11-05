# Contributing to Project Name

First off, thank you for considering contributing to this project! It's people like you that make this project better.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Guidelines](#coding-guidelines)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Pull Request Process](#pull-request-process)
- [Agent Coding Tools](#agent-coding-tools)

## 📜 Code of Conduct

This project adheres to the principles of open collaboration and respect. Please be professional and courteous in all interactions.

## 🚀 Getting Started

### Prerequisites

- Rust 1.91+ ([install](https://rustup.rs/))
- Git
- Basic familiarity with Rust tooling

### Setting Up Your Development Environment

```bash
# 1. Fork the repository on GitHub

# 2. Clone your fork
git clone https://github.com/your-username/repo.git
cd repo

# 3. Add the upstream repository as a remote
git remote add upstream https://github.com/original/repo.git

# 4. Verify your setup
cargo test
cargo build
```

## 🔄 Development Workflow

We follow a simplified GitFlow:

1. **Create a feature branch** from `main`
2. **Make your changes** following our coding guidelines
3. **Write/update tests** for your changes
4. **Run the test suite** to ensure everything works
5. **Update documentation** if needed
6. **Submit a pull request** to the `main` branch

### Branch Naming Convention

- Feature branches: `feature/descriptive-name`
- Bug fixes: `fix/issue-number-description`
- Documentation: `docs/descriptive-name`
- Performance: `perf/improvement-description`

Example:
```bash
git checkout -b feature/add-async-http-client
```

## 📝 Coding Guidelines

### Code Style

We follow the standard Rust formatting guidelines enforced by `rustfmt`:

```bash
# Format your code before committing
cargo fmt

# Run clippy for additional linting
cargo clippy --all-targets --all-features -- -D warnings
```

### General Guidelines

1. **Write expressive code**: Use clear, descriptive names
2. **Keep functions small**: Aim for < 50 lines per function
3. **Follow DRY**: Don't Repeat Yourself
4. **Prefer composition over inheritance**
5. **Handle errors gracefully**: Use `Result` and `Option` appropriately
6. **Document public APIs**: Use `///` doc comments
7. **Add examples**: Include usage examples in doc comments

### Example Code Structure

```rust
/// Brief description of what this function does.
///
/// # Arguments
///
/// * `param1` - Description of the first parameter
/// * `param2` - Description of the second parameter
///
/// # Returns
///
/// Description of what is returned.
///
/// # Examples
///
/// ```
/// let result = my_function(42, "example");
/// assert_eq!(result.is_ok(), true);
/// ```
pub fn my_function(param1: i32, param2: &str) -> Result<String, Error> {
    // Implementation
    todo!("Implement the function")
}
```

### Error Handling

Use the `?` operator and `Result` for error handling:

```rust
fn process_data(data: &[u8]) -> Result<ProcessedData, ProcessingError> {
    let parsed = parse_input(data)?;
    let validated = validate_data(parsed)?;
    Ok(validated)
}
```

### Async/Await

For async code, follow these patterns:

```rust
/// Spawns an async task with proper error handling
pub async fn async_operation() -> Result<(), Error> {
    // Use tokio or async-std based on project preference
    perform_operation().await?;
    Ok(())
}
```

## 🧪 Testing Guidelines

### Test Organization

1. **Unit tests**: In the same file as the code they're testing
2. **Integration tests**: In the `tests/` directory
3. **Documentation tests**: In doc comments with `/// ```rust`

### Writing Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_functionality() {
        // Arrange
        let input = setup_test_data();

        // Act
        let result = function_under_test(input);

        // Assert
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_value);
    }

    #[test]
    fn test_error_handling() {
        let invalid_input = create_invalid_input();
        let result = function_under_test(invalid_input);
        assert!(result.is_err());
    }
}
```

### Running Tests

```bash
# All tests
cargo test

# With output
cargo test -- --nocapture

# Specific test
cargo test test_name

# With coverage
tarpaulin --verbose --all-features --workspace --out Html
```

### Property-Based Testing

For complex logic, use property-based testing:

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn test_reverse_involutive(
        input in prop::collection::vec(any::<i32>(), 0..100)
    ) {
        let reversed: Vec<i32> = input.clone().into_iter().rev().collect();
        let double_reversed: Vec<i32> = reversed.into_iter().rev().collect();
        assert_eq!(input, double_reversed);
    }
}
```

## 📚 Documentation

### Documentation Requirements

1. **Public APIs** must have documentation
2. **Complex algorithms** should have detailed explanations
3. **Usage examples** should be included where helpful
4. **Architecture decisions** should be documented in `docs/`

### Building Documentation

```bash
# Generate local documentation
cargo doc --open

# Generate docs without building
cargo doc --no-deps --open
```

### Example Documentation

```rust
/// A high-performance data structure for storing key-value pairs.
///
/// This structure provides O(1) average-case time complexity for insertion,
/// lookup, and deletion operations. It uses a hash table with open addressing
/// to minimize memory overhead.
///
/// # Examples
///
/// Basic usage:
///
/// ```
/// let mut map = HashMap::new();
/// map.insert("key", "value");
/// assert_eq!(map.get("key"), Some(&"value"));
/// ```
///
/// Using with custom types:
///
/// ```
/// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
/// struct User {
///     id: u64,
///     name: String,
/// }
///
/// let mut users = HashMap::new();
/// users.insert(User { id: 1, name: "Alice".to_string() }, "admin");
/// ```
pub struct HashMap<K, V> {
    // Internal implementation
}
```

## 🔀 Pull Request Process

### Before Submitting

1. ✅ Run all tests: `cargo test`
2. ✅ Format code: `cargo fmt`
3. ✅ Lint code: `cargo clippy --all-targets --all-features -- -D warnings`
4. ✅ Check security: `cargo audit`
5. ✅ Update documentation if needed
6. ✅ Add/Update tests for your changes
7. ✅ Update CHANGELOG.md if applicable

### Pull Request Template

When submitting a PR, please fill out the following:

```markdown
## 📝 Description
Brief description of the changes

## ✅ Changes Made
- [ ] Feature
- [ ] Bug fix
- [ ] Documentation
- [ ] Performance improvement

## 🧪 Testing
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests pass locally

## 📚 Documentation
- [ ] Documentation updated
- [ ] Examples added/updated

## 🔍 Checklist
- [ ] Code follows project style guidelines
- [ ] Self-review completed
- [ ] Code is properly commented
- [ ] No compiler warnings
```

### Review Process

1. **Automated checks** must pass (CI, tests, linting)
2. **At least one reviewer** must approve
3. **Address all feedback** before merging
4. **Squash and merge** preferred for clean history

### PR Size Guidelines

- **Small PRs** (< 300 lines): Encouraged, faster review
- **Medium PRs** (300-1000 lines): Normal review time
- **Large PRs** (> 1000 lines): Consider splitting into smaller PRs

## 🤖 Agent Coding Tools

This project is optimized for AI coding assistants including **Claude Code**, **OpenCode**, and **Codex**.

### For AI Assistants

#### Getting Context

```bash
# View project structure
tree -L 3 -I 'target'

# View recent commits
git log --oneline -10

# View open issues
gh issue list

# View documentation
find docs -name "*.md" | head -5
```

#### Best Practices for AI Agents

1. **Understand the architecture** before making changes
2. **Read existing tests** to understand expected behavior
3. **Follow established patterns** in the codebase
4. **Run tests frequently** to verify changes
5. **Ask for clarification** when uncertain
6. **Document reasoning** in pull requests

#### Working with Rust

Key considerations for AI agents:

```rust
// ✅ GOOD: Proper error handling
fn process(data: &[u8]) -> Result<String, ProcessingError> {
    if data.is_empty() {
        return Err(ProcessingError::InvalidInput(
            "Data cannot be empty".to_string()
        ));
    }
    // ... process data
    Ok(result)
}

// ❌ BAD: Ignoring potential errors
fn process(data: &[u8]) -> String {
    let result = risky_operation(); // Could panic!
    result
}
```

#### Testing with AI

AI agents should write comprehensive tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_case() {
        // Test typical usage
    }

    #[test]
    fn test_edge_cases() {
        // Test boundary conditions
    }

    #[test]
    fn test_error_cases() {
        // Test error handling
    }

    #[test]
    fn test_with_fuzzing() {
        // Property-based tests for robustness
    }
}
```

### Agent-Optimized File Structure

For AI-friendly project organization:

```
/src
  /module_name         # Clear, descriptive names
    mod.rs
    public_api.rs      # Public interfaces
    internal.rs        # Private implementation
    tests.rs           # Module tests

/examples
  basic_usage.rs      # Simple examples
  advanced_usage.rs   # Complex examples
  real_world.rs       # Production scenarios

/tests
  integration_test1.rs
  integration_test2.rs
```

## 🎯 Common Contribution Scenarios

### Adding a New Feature

1. Create feature branch
2. Implement feature with tests
3. Update documentation
4. Run full test suite
5. Submit PR

### Fixing a Bug

1. Create issue or find existing one
2. Create fix branch
3. Add test that reproduces the bug
4. Implement fix
5. Verify fix with test
6. Submit PR

### Improving Documentation

1. Create docs branch
2. Make documentation changes
3. Build and verify docs render correctly
4. Submit PR

## 📊 Performance Considerations

When making changes, consider:

- **Algorithmic complexity**: Use appropriate data structures
- **Memory allocation**: Minimize unnecessary allocations
- **Cache efficiency**: Consider cache-friendly data layouts
- **Benchmarks**: Run benchmarks for performance-sensitive code

```bash
# Run benchmarks
cargo bench

# Profile with criterion
cargo criterion --message-format=json
```

## ❓ Questions?

- **GitHub Issues**: [Open an issue](https://github.com/username/repo/issues)
- **Discussions**: Use GitHub Discussions
- **Discord**: [Join our Discord](https://discord.gg/your-project)

## 🙏 Recognition

Contributors will be recognized in:

- [CONTRIBUTORS.md](CONTRIBUTORS.md)
- Release notes for significant contributions
- Project README (for major contributors)

## 📄 License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing! 🎉**

Every contribution, no matter how small, helps make this project better.
