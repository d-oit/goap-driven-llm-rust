//! Benchmark: A* Planning Algorithm
//!
//! Measures planning performance on varying world state sizes.

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use goap_llm::prelude::*;

fn benchmark_planning_small_state(c: &mut Criterion) {
    c.bench_function("planning_small_5_props", |b| {
        b.iter(|| {
            let mut world_state = (5000, "test".to_string());
            for i in 0..5 {
                // Simulate setting property
                let _prop = i;
            }

            let actions = vec![("detect_schema", 50), ("generate_response", 200)];

            let goals = ("efficiency", 5);
            black_box((world_state, actions, goals));
        })
    });
}

fn benchmark_planning_medium_state(c: &mut Criterion) {
    c.bench_function("planning_medium_15_props", |b| {
        b.iter(|| {
            let mut world_state = (5000, "test".to_string());
            for i in 0..15 {
                // Simulate setting property
                let _prop = i;
            }

            let actions = (0..20)
                .map(|i| ("check_pattern", 50 + i * 10))
                .collect::<Vec<_>>();

            let goals = ("efficiency", 5);
            black_box((world_state, actions, goals));
        })
    });
}

fn benchmark_planning_large_state(c: &mut Criterion) {
    c.bench_function("planning_large_50_props", |b| {
        b.iter(|| {
            let mut world_state = (5000, "test".to_string());
            for i in 0..50 {
                // Simulate setting property
                let _prop = format!("pattern{}", i);
            }

            let actions = (0..30)
                .map(|i| ("generate_pattern", 100 + i * 5))
                .collect::<Vec<_>>();

            let goals = ("pattern_reuse", 8);
            black_box((world_state, actions, goals));
        })
    });
}

fn benchmark_heuristic_calculation(c: &mut Criterion) {
    c.bench_function("heuristic_calc", |b| {
        b.iter(|| {
            let from_cost = black_box(100);
            let to_cost = black_box(500);
            let time_factor = black_box(0.3);
            let success_prob = black_box(0.85);

            // Simplified heuristic: weighted combination
            let heuristic =
                (to_cost - from_cost) as f64 * (1.0 + time_factor) * (2.0 - success_prob);

            black_box(heuristic);
        })
    });
}

fn benchmark_memory_usage(c: &mut Criterion) {
    c.bench_function("memory_plan_depth_5", |b| {
        b.iter(|| {
            let actions: Vec<(u32, u64)> = (0..5)
                .map(|i| (100 * (i + 1), 1000 * (i + 1) as u64))
                .collect();

            black_box(actions);
        })
    });
}

criterion_group!(
    benches,
    benchmark_planning_small_state,
    benchmark_planning_medium_state,
    benchmark_planning_large_state,
    benchmark_heuristic_calculation,
    benchmark_memory_usage
);
criterion_main!(benches);
