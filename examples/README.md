# GOAP Examples

This directory contains runnable example programs demonstrating the GOAP (Goal-Oriented Action Planning) library for LLM strategic reasoning.

## Basic Examples

### 1. Basic Planning (`basic_planning.rs`)
Minimal setup showing:
- Creating a world state
- Defining actions with preconditions and effects
- Setting up goals
- Running the planner and executor

```bash
cargo run --example basic_planning
```

### 2. Pattern Reuse (`pattern_reuse.rs`)
Demonstrates:
- Pattern caching from successful executions
- Reusing patterns for similar requests
- Measuring token and time savings

```bash
cargo run --example pattern_reuse
```

### 3. Reactive Replanning (`reactive_replanning.rs`)
Shows:
- Failure detection during execution
- Automatic replanning with alternative paths
- Recovery metrics and success rates

```bash
cargo run --example reactive_replanning
```

## Advanced Examples

### 4. Token Optimization (`token_optimization.rs`)
Illustrates:
- Token budget management
- Real-time consumption tracking
- Compression and optimization strategies

```bash
cargo run --example token_optimization
```

### 5. Metrics Collection (`metrics_collection.rs`)
Demonstrates:
- Performance monitoring over time
- Learning effect measurement
- Success criteria validation

```bash
cargo run --example metrics_collection
```

### 6. CLI Wrapper (`cli_wrapper.rs`)
Shows:
- Command-line interface usage
- JSON input/output format
- Configuration options

```bash
cargo run --example cli_wrapper -- --help
```

### 7. Custom Actions (`custom_actions.rs`)
Explains:
- Creating domain-specific action types
- Integrating custom actions with GOAP
- Complex precondition/effect chains

```bash
cargo run --example custom_actions
```

## Running Examples

### Run All Examples
```bash
for example in basic_planning pattern_reuse reactive_replanning token_optimization metrics_collection cli_wrapper custom_actions; do
    echo "=== Running $example ==="
    cargo run --example $example
    echo ""
done
```

### Run with Custom Configuration
```bash
# Token budget example
cargo run --example token_optimization -- --budget 2000

# Speed-optimized example
cargo run --example cli_wrapper -- --optimize speed --format json

# Verbose output
cargo run --example basic_planning -- --verbose
```

## Example Output

Each example produces:
- **Success/failure status**
- **Performance metrics** (time, tokens)
- **World state changes**
- **Goal satisfaction**
- **Success criteria validation**

## Common Use Cases

| Example | Use Case |
|---------|----------|
| basic_planning | Learning GOAP fundamentals |
| pattern_reuse | Optimizing repeated similar requests |
| reactive_replanning | Handling failures gracefully |
| token_optimization | Managing token budgets |
| metrics_collection | Monitoring performance |
| cli_wrapper | Building command-line tools |
| custom_actions | Extending for specific domains |

## Next Steps

1. Run `basic_planning.rs` to understand the basics
2. Try `pattern_reuse.rs` to see efficiency gains
3. Experiment with `token_optimization.rs` for cost control
4. Use `metrics_collection.rs` to measure your improvements
5. Build your own solution using these patterns

## Troubleshooting

### Examples Don't Compile
```bash
# Ensure all dependencies are installed
cargo update

# Check example paths
ls examples/
```

### Runtime Errors
- Check token budget (default: 5000)
- Verify action preconditions are met
- Review world state properties

### Performance Issues
- Monitor token usage with `metrics_collection`
- Check pattern cache hit rates
- Analyze replanning frequency
