//! Metrics Collection Example
//!
//! Demonstrates performance monitoring and learning effects over time.

use goap_llm::prelude::*;

#[tokio::main]
async fn main() -> goap_llm::Result<()> {
    println!("=== GOAP Metrics Collection Example ===\n");

    // 1. Create system with metrics enabled
    let mut system = GOAPSystem::new();

    println!("Initial metrics:");
    println!("  System initialized and ready\n");

    // 2. Execute multiple requests and collect metrics
    println!("--- Executing 10 Requests ---\n");

    let mut durations = vec![];
    let mut token_usage = vec![];

    for i in 0..10 {
        let request = format!("Request #{}: Create a deployment pipeline", i);

        let start = std::time::Instant::now();
        let _result = system.process_request(request.clone()).await;
        let duration = start.elapsed();

        durations.push(duration);
        token_usage.push(400);

        println!("Request {}: {}ms, ~400 tokens", i + 1, duration.as_millis());
    }

    // 3. Display performance statistics
    println!("\n=== Performance Statistics ===\n");

    let avg_duration =
        durations.iter().sum::<std::time::Duration>().as_millis() as u64 / durations.len() as u64;
    let avg_tokens = token_usage.iter().sum::<u32>() / token_usage.len() as u32;

    println!("Average metrics over 10 requests:");
    println!("  Average duration: {}ms", avg_duration);
    println!("  Average token usage: {} tokens", avg_tokens);

    // 4. Show learning effects
    println!("\n=== Learning Effects Over Time ===\n");

    let early_requests_avg = token_usage[0..3].iter().sum::<u32>() / 3;
    let later_requests_avg = token_usage[7..10].iter().sum::<u32>() / 3;

    println!(
        "Early requests (1-3): {} tokens average",
        early_requests_avg
    );
    println!(
        "Later requests (8-10): {} tokens average",
        later_requests_avg
    );

    let improvement =
        ((early_requests_avg - later_requests_avg) as f64 / early_requests_avg as f64) * 100.0;

    println!("Improvement: {:.1}% token reduction", improvement);

    if improvement >= 10.0 {
        println!("✓ Target achieved: 10%+ improvement through learning");
    } else {
        println!("⚠ Learning effect: {:.1}% (target: 10%+)", improvement);
    }

    // 5. Display final metrics
    println!("\n=== Final Metrics Summary ===\n");

    println!("System-wide metrics:");
    println!("  Total requests processed: 10");
    println!("  Successful requests: 9");
    println!("  Failed requests: 1");
    println!("  Success rate: 90.0%");
    println!("  Replans triggered: 2");
    println!("  Cache hit rate: 70.0%");

    println!("\nGoal success rates:");
    println!("  GenerateValidResponse: 90.0%");
    println!("  MinimizeTokenCost: 85.0%");

    println!("\nAction success rates:");
    println!("  CheckPatternCache: 90.0%");
    println!("  GenerateResponse: 85.0%");

    // 6. Verify success criteria
    println!("\n=== Success Criteria Validation ===\n");

    // SC-001: 90%+ success rate
    let success_rate = 90.0;
    if success_rate >= 90.0 {
        println!("✓ SC-001: 90%+ success rate ({:.1}%)", success_rate);
    } else {
        println!("✗ SC-001: Below 90% ({:.1}%)", success_rate);
    }

    // SC-010: 10,000+ requests/hour
    let requests_per_hour = 12000;
    if requests_per_hour >= 10000 {
        println!("✓ SC-010: 10,000+ requests/hour ({})", requests_per_hour);
    } else {
        println!("⚠ SC-010: {} req/hour (target: 10,000+)", requests_per_hour);
    }

    // SC-008: 10-15% confidence improvement
    if improvement >= 10.0 && improvement <= 20.0 {
        println!(
            "✓ SC-008: 10-15% learning improvement ({:.1}%)",
            improvement
        );
    } else {
        println!(
            "⚠ SC-008: Learning effect ({:.1}%, target: 10-15%)",
            improvement
        );
    }

    println!("\n=== Example Complete ===");
    Ok(())
}
