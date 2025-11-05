//! Pattern Reuse Example
//!
//! Demonstrates pattern caching and reuse for efficiency improvements.

use goap_llm::prelude::*;

#[tokio::main]
async fn main() -> goap_llm::Result<()> {
    println!("=== GOAP Pattern Reuse Example ===\n");

    // 1. Create GOAP system with pattern cache
    let mut system = GOAPSystem::new();

    // 2. First request: Create a pattern
    println!("--- Request 1: Initial Request (No Pattern) ---");
    let request1 = "Create a GitHub Actions workflow for deploying a Node.js app".to_string();

    let start1 = std::time::Instant::now();
    let result1 = system.process_request(request1.clone()).await;
    let duration1 = start1.elapsed();

    match result1 {
        Ok(response) => {
            println!("✓ Request 1 completed");
            println!("  Response: {}\n", response);
        }
        Err(e) => {
            println!("✗ Request 1 failed: {}\n", e);
        }
    }

    // 3. Second request: Reuse pattern
    println!("--- Request 2: Similar Request (With Pattern Reuse) ---");
    let request2 = "Create GitHub Actions workflow for Node.js deployment".to_string();

    let start2 = std::time::Instant::now();
    let result2 = system.process_request(request2.clone()).await;
    let duration2 = start2.elapsed();

    match result2 {
        Ok(response) => {
            println!("✓ Request 2 completed");
            println!("  Response: {}\n", response);
        }
        Err(e) => {
            println!("✗ Request 2 failed: {}\n", e);
        }
    }

    // 4. Display efficiency comparison
    println!("=== Efficiency Comparison ===");

    println!("Duration comparison:");
    println!("  Request 1: {:?}ms", duration1.as_millis());
    println!("  Request 2: {:?}ms", duration2.as_millis());

    if duration1.as_millis() > 0 {
        let time_improvement = ((duration1.as_millis() - duration2.as_millis()) as f64
            / duration1.as_millis() as f64)
            * 100.0;
        println!("Time improvement: {:.1}%", time_improvement);

        if time_improvement >= 25.0 {
            println!("✓ Target achieved: 25-35% improvement (SC-003)");
        } else {
            println!("⚠ Target not yet reached: 25-35%");
        }
    }

    // 5. Display pattern cache status
    println!("\n=== Pattern Cache Status ===");
    println!("Patterns cached: 1 (simulated for demo)");
    println!("  - Pattern 'nodejs-workflow': confidence=85%, usage=1");

    println!("\n=== Example Complete ===");
    Ok(())
}
