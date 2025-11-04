# Tutorial: Pattern Reuse

Learn how to leverage cached patterns for 50-70% token savings and faster responses.

## Overview

Pattern reuse is GOAP's key efficiency feature:

```
Request A → Process → Cache Pattern → Request B (similar) → Reuse Pattern → 60% savings!
     ↓            ↓           ↓              ↓              ↓            ↓
   GitHub    Generate    Store in      GitHub        Retrieve     Faster &
  Workflow   500 tokens   Cache        Workflow      Pattern      cheaper
```

## How Pattern Reuse Works

### 1. Pattern Creation
When a request succeeds, GOAP extracts the pattern:

```rust
// Successful execution
let result = system.process_request(&mut ws, actions, goals).await?;

if result.success {
    // Pattern is automatically extracted
    // - Action sequence: [DetectSchema, CheckCache, Generate]
    // - Metadata: confidence, success rate, token usage
    // - Stored in IntelligentCache
}
```

### 2. Pattern Storage
Patterns are stored with metadata:

```rust
SuccessPattern {
    id: "github-workflow-v1",
    action_sequence: [DetectSchema, GenerateResponse],
    confidence: 85,           // Likelihood of success
    success_rate: 0.92,       // Historical success rate
    avg_tokens: 180,          // Average token usage
    usage_count: 45,          // Times used
    created_at: Timestamp,
    last_used: Timestamp,
}
```

### 3. Pattern Detection
For new requests, GOAP finds similar patterns:

```rust
// LSH (Locality Sensitive Hashing) for similarity
let similarity = calculate_similarity(
    new_request_hash,
    cached_pattern.request_hash
);

// If similarity > threshold (e.g., 0.7), use pattern
```

### 4. Pattern Reuse
Benefits of using cached patterns:

- **Fewer tokens**: 150 vs 500 tokens (70% savings)
- **Faster execution**: 800ms vs 2500ms (68% faster)
- **Higher success rate**: 92% vs 85%
- **Lower cost**: Direct path, no planning overhead

## Step-by-Step Guide

### Step 1: Enable Pattern Caching

```rust
use goap_llm::prelude::*;

let system = GOAPSystem::new();  // Pattern cache enabled by default
```

### Step 2: Process Initial Requests

```rust
// First request - no patterns available
let mut ws1 = WorldState::new(5000, "Create GitHub Actions workflow".to_string());
let actions1 = vec![
    Action::new(ActionType::DetectSchemaType).with_cost(50),
    Action::new(ActionType::GenerateResponse).with_cost(400),
];

let result1 = system.process_request(&mut ws1, actions1, GoalState::primary_goal()).await?;
println!("Tokens used: {}", 5000 - ws1.tokens_remaining());  // ~450 tokens
```

### Step 3: Verify Pattern Storage

```rust
let cache = system.get_cache();
let patterns = cache.get_patterns();

for pattern in patterns {
    println!("Pattern: {}", pattern.id);
    println!("  Confidence: {}%", pattern.confidence);
    println!("  Usage: {} times", pattern.usage_count);
}
```

### Step 4: Reuse Patterns

```rust
// Similar request - pattern should be reused
let mut ws2 = WorldState::new(
    5000,
    "Create GitHub Actions workflow for Node.js deployment".to_string()
);

let result2 = system.process_request(&mut ws2, actions1, GoalState::pattern_reuse_goal()).await?;
println!("Tokens used: {}", 5000 - ws2.tokens_remaining());  // ~180 tokens!
```

## Pattern Confidence

Confidence determines when to use patterns:

```rust
// Confidence calculation
let confidence = (
    similarity * 0.4 +           // How similar is the request?
    success_rate * 0.4 +         // Historical success rate?
    usage_frequency * 0.2        // How often used?
) * 100.0;

// Thresholds
if confidence >= 80.0 {
    // High confidence: Use pattern immediately
    use_pattern()
} else if confidence >= 60.0 {
    // Medium confidence: Use with caution
    consider_pattern()
} else {
    // Low confidence: Skip pattern
    full_generation()
}
```

### Adjusting Confidence

Increase confidence by:
- Successful pattern matches
- High usage frequency
- Consistent success

Decreases when:
- Pattern fails
- Not used for long time
- Request differs significantly

## Tuning Pattern Reuse

### Set Confidence Threshold

```rust
// In GOAPConfig
let config = GOAPConfig {
    pattern_confidence_threshold: 70,  // Use patterns >70%
    ..Default::default()
};
```

### Adjust Similarity Detection

```rust
// Tune LSH parameters for your domain
let similarity_config = SimilarityConfig {
    hash_size: 64,        // Larger = more precise
    bucket_count: 100,    // More buckets = better distribution
    threshold: 0.75,      // Similarity threshold
};
```

### Pattern Learning Rate

```rust
let learning_config = LearningConfig {
    confidence_increment: 5.0,    // +5% per successful use
    decay_rate: 0.95,             // 5% decay per period
    min_confidence: 30.0,         // Don't use below 30%
};
```

## Pattern Types

### 1. Template Patterns
Generic patterns for common tasks:

```rust
// "Create a deployment workflow"
SuccessPattern {
    action_sequence: [DetectSchema, Validate, Generate],
    confidence: 85,
    applicable_to: ["github-workflow", "gitlab-ci", "jenkins"],
}
```

### 2. Domain Patterns
Specific to particular domains:

```rust
// "Create Kubernetes deployment"
SuccessPattern {
    action_sequence: [DetectK8sSchema, CheckK8sPatterns, Generate],
    domain: "kubernetes",
    confidence: 90,
}
```

### 3. User Patterns
Tailored to specific users:

```rust
// Adapted to user's coding style
SuccessPattern {
    action_sequence: [DetectStyle, ApplyUserPrefs, Generate],
    user_id: "user123",
    confidence: 88,
}
```

## Advanced Pattern Techniques

### Pattern Chaining

Combine multiple patterns:

```rust
// Pattern 1: GitHub workflow
let pattern1 = cache.find("github-workflow")?;

// Pattern 2: Node.js deployment
let pattern2 = cache.find("nodejs-deploy")?;

// Chain them for complex requests
let combined_actions = pattern1.actions + pattern2.actions;
```

### Pattern Composition

Build patterns from components:

```rust
// Base pattern
let base_pattern = Pattern::new("base-validation");

// Add components
base_pattern.add_step(ActionType::PreValidateRequest);
base_pattern.add_step(ActionType::QuickValidatePattern);
base_pattern.add_step(ActionType::GenerateFromTemplate);
```

### Dynamic Pattern Adaptation

Modify patterns based on context:

```rust
// Adapt pattern for low-budget scenario
if tokens_remaining < 200 {
    pattern.with_compression_enabled(true)
        .with_skip_validation(true)
        .with_minimal_generation(true)
}
```

## Measuring Pattern Effectiveness

### Track Token Savings

```rust
let baseline_tokens = 500;  // Full generation
let pattern_tokens = 180;   // With pattern

let savings = ((baseline_tokens - pattern_tokens) / baseline_tokens) * 100.0;
println!("Token savings: {:.1}%", savings);  // 64%
```

### Monitor Success Rate

```rust
let total_pattern_uses = 100;
let successful_uses = 92;

let success_rate = (successful_uses as f64 / total_pattern_uses) * 100.0;
println!("Pattern success rate: {:.1}%", success_rate);  // 92%
```

### Measure Response Time

```rust
let baseline_time = Duration::from_millis(2500);  // Full generation
let pattern_time = Duration::from_millis(800);    // With pattern

let speedup = (baseline_time.as_millis() - pattern_time.as_millis()) as f64
    / baseline_time.as_millis() * 100.0;
println!("Speed improvement: {:.1}%", speedup);  // 68%
```

## Pattern Cache Management

### Cache Size Limits

```rust
let cache_config = CacheConfig {
    max_patterns: 1000,     // Limit cache size
    max_memory_mb: 256,     // Memory limit
    eviction_policy: EvictionPolicy::LRU,  // Remove least recently used
};
```

### Pattern Decay

Patterns become less reliable over time:

```rust
// Decay calculation
let age_weeks = (now - pattern.created_at).num_weeks();
let decay_factor = 0.95f64.powf(age_weeks as f64);
let decayed_confidence = pattern.confidence * decay_factor;

// Remove if confidence drops too low
if decayed_confidence < 30.0 {
    cache.remove(pattern.id);
}
```

### Pattern Invalidation

Invalidate patterns when:
- They consistently fail
- Schema changes
- User feedback indicates poor quality

```rust
// Invalidate failed pattern
cache.invalidate_pattern(
    "pattern_id",
    InvalidationReason::ConsistentFailure
);
```

## Best Practices

### 1. Start with High-Quality Patterns
- Only cache successful executions
- Set confidence based on success rate
- Monitor pattern quality over time

### 2. Use Appropriate Thresholds
- Don't use low-confidence patterns (waste tokens)
- Adjust threshold based on domain (70% for safety, 82% for strict)

### 3. Monitor Pattern Health
```rust
// Track these metrics
let metrics = system.get_metrics();
println!("Cache hit rate: {:.1}%", metrics.cache_hit_rate * 100.0);
println!("Patterns active: {}", cache.get_patterns().len());
```

### 4. Clean Up Stale Patterns
```rust
// Remove old, unused patterns
cache.cleanup(|pattern| {
    pattern.last_used < cutoff_date && pattern.confidence < 50.0
});
```

### 5. Validate Pattern Quality
```rust
// Test patterns before trusting
for pattern in cache.get_patterns() {
    let test_result = validate_pattern(pattern);
    if !test_result.is_valid {
        cache.remove(pattern.id);
    }
}
```

## Common Pitfalls

### Pitfall 1: Using Low-Confidence Patterns
```rust
// Bad: Using 40% confidence pattern
let pattern = cache.find("similar")?;  // May fail!

// Good: Use high-confidence patterns
let pattern = cache.find_high_confidence("similar", 80.0)?;
```

### Pitfall 2: Not Updating Confidence
```rust
// Bad: Confidence never changes
static_pattern.confidence = 70;  // Always 70%

// Good: Update based on outcomes
if pattern.successful() {
    pattern.confidence += 5;
} else {
    pattern.confidence -= 10;
}
```

### Pitfall 3: Caching Failed Patterns
```rust
// Bad: Cache everything
cache.store(execution_result);  // Even failures!

// Good: Only cache successes
if execution_result.success {
    cache.store(execution_result);
}
```

## Troubleshooting

### "Pattern not found"
- Increase similarity threshold tolerance
- Add more training examples
- Check if pattern was invalidated

### "Low confidence despite reuse"
- Verify pattern quality
- Check success rate calculation
- Adjust confidence increment

### "No token savings"
- Verify pattern is actually being used
- Check action costs are realistic
- Monitor actual vs estimated costs

### "Pattern成功率下降"
- Review pattern invalidation logic
- Check if schema changed
- Verify pattern is still applicable

## Next Steps

- Learn [Reactive Replanning](TUTORIAL_REACTIVE.md) for failure handling
- Review [Performance Tuning](PERFORMANCE.md) for optimization
- Check [Error Handling](ERROR_HANDLING.md) for best practices

## Example

See `examples/pattern_reuse.rs` for a complete working example with metrics.
