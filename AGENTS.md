### 📌 Purpose

This guide ensures all AI agents generate **consistent, high-quality Rust code** across modules: architecture, testing, patterns, tooling, and constraints.

---

## 📚 Table of Contents

* [Project Standards](#project-standards)
* [Architecture & Code Structure](#architecture--code-structure)
* [Development Workflow for Agents](#development-workflow-for-agents)
* [Core Code Patterns](#core-code-patterns)
* [Testing Patterns](#testing-patterns)
* [Common Tasks (+ Templates)](#common-tasks--templates)
* [Quality Rules for Generated Code](#quality-rules-for-generated-code)
* [Key Files to Read](#key-files-to-read)
* [Help Checklist for Agents](#help-checklist-for-agents)
* [Learning Resources](#learning-resources)

---

## 🧱 Project Standards

### Language & Tooling

| Category       | Standard                                 |
| -------------- | ---------------------------------------- |
| Rust           | ≥ 1.91 (Edition 2021)                    |
| Async runtime  | Tokio                                    |
| Error Handling | `thiserror`, `anyhow`, contextual errors |
| Logging        | `tracing`, `tracing-subscriber`          |
| Tests          | `tokio-test`, `mockall`                  |

### Required Dependencies (short)

```toml
tokio = { version = "1", features = ["full"] }
anyhow = "1"
thiserror = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
tokio-test = "0.4"
mockall = "0.12"
```

---

## 🧩 Architecture & Code Structure

```
src/
├── lib.rs            # Exports modules, core types
├── main.rs           # Bin entry (if any)
├── error.rs          # Custom error + Result
├── config.rs         # Config loading & validation
└── module/
    ├── mod.rs        # Module public surface
    ├── public_api.rs # Public API only
    ├── internal.rs   # Private logic
    └── tests.rs      # Unit tests for module
```

### Error Handling Pattern

* Use `thiserror` for typed errors
* Use `anyhow::Context` to add context when calling external systems
* Never `unwrap()` or `panic!()` inside library code

```rust
pub type Result<T> = std::result::Result<T, Error>;
```

---

## 🔄 Development Workflow for Agents

### When starting a task

1. **Explore structure**

   ```bash
   find src -name "*.rs"
   ```
2. **Read: `lib.rs`, `error.rs`, similar modules**
3. **Check tests before coding**
4. **Follow patterns only – never invent new architecture**

### Implementation Flow (for agents)

> **Pattern, not invention. Match existing style.**

1. Find similar code → replicate pattern
2. Write tests first (TDD preferred)
3. Implement with `Result<T, Error>`
4. Add docs (`///`) on public APIs
5. Run tests before producing final output

```bash
cargo test --all --workspace
```

---

## 🧠 Core Code Patterns

### Module Skeleton

```rust
// mod.rs
pub mod public_api;
mod internal;

pub use public_api::*;
```

### Public API Rules

* Validate inputs
* Return `Result<T, Error>`
* Async only when needed

```rust
pub fn new(param: String) -> Result<Self> {
    if param.trim().is_empty() {
        return Err(Error::InvalidInput { message: "param empty".into() });
    }
    Ok(Self { /* ... */ })
}
```

### Internal Implementation

* No public exports
* Keep logic cohesive and isolated
* Unit test through public API unless internal behavior is complex

---

## 🧪 Testing Patterns

### Unit Tests

```rust
#[test]
fn test_sync() {
    let res = function("input");
    assert!(res.is_ok());
}

#[tokio::test]
async fn test_async() {
    let res = async_fn("x").await;
    assert!(res.is_ok());
}
```

### Integration Tests

Create in `tests/`:

```rust
#[tokio::test]
async fn full_flow() {
    let svc = PublicStruct::new("x".into()).unwrap();
    assert_eq!(svc.async_method().await.unwrap(), "done");
}
```

### Mocking External Calls

```rust
mock_service.expect_call()
    .with(eq("x"))
    .times(1)
    .returning(|_| Ok("ok".into()));
```

---

## 🧰 Common Tasks & Templates

### Add a New Module

1. `src/my_module/`
2. Add: `mod.rs`, `public_api.rs`, `internal.rs`, `tests.rs`
3. Export from `lib.rs`

### Configuration Loading

* Never hardcode values
* Read env or config file
* Validate before returning

---

## 🧬 Quality Rules for Generated Code

### Hard Requirements

| Category       | Rule                                |
| -------------- | ----------------------------------- |
| File size      | ≤ 500 LOC per file                  |
| Config         | No secrets, no env-specific values  |
| Architecture   | Modular, pattern-aligned            |
| Testing        | Must include tests for all new code |
| Error Handling | No panic/unwrap/silent failures     |

**If >500 LOC** → Agent must **auto-split into multiple modules** and provide a file tree first.

---

## 📁 Key Files to Read

Before coding, agents must **read or summarize**:

1. `src/lib.rs`
2. `src/error.rs`
3. `src/config.rs`
4. `Cargo.toml`
5. `tests/`
6. `examples/` (if present)

---

## 🆘 Help Checklist for Agents

If stuck:

* Read tests → find expected behavior
* Search for similar module → copy pattern
* Trace error types before adding new ones
* Keep code minimal & cohesive

---

## 🎓 Learning Resources (Shortlist)

* [https://doc.rust-lang.org/book/](https://doc.rust-lang.org/book/)
* [https://rust-lang.github.io/async-book/](https://rust-lang.github.io/async-book/)
* [https://doc.rust-lang.org/error-handling/](https://doc.rust-lang.org/error-handling/)
* [https://doc.rust-lang.org/rust-by-example/](https://doc.rust-lang.org/rust-by-example/)
