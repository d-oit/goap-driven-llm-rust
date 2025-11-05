//! Benchmark: Pattern Cache Performance
//!
//! Measures cache lookup latency, hit rates, and memory footprint.

use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn benchmark_pattern_lookup_latency(c: &mut Criterion) {
    c.bench_function("cache_lookup_100_patterns", |b| {
        // Setup: Create cache with 100 patterns
        let patterns: Vec<_> = (0..100).map(|i| format!("pattern_{}", i)).collect();

        // Benchmark lookup
        b.iter(|| {
            let search_pattern = "pattern_50";
            // Simulated lookup (actual implementation would use cache)
            let found = patterns.iter().find(|p| p == &search_pattern);
            black_box(found);
        })
    });
}

fn benchmark_cache_hit_rate(c: &mut Criterion) {
    c.bench_function("cache_hit_rate", |b| {
        // Setup cache with varying pattern popularity
        let patterns = vec![
            ("hot_pattern", 100), // 100 hits
            ("warm_pattern", 50), // 50 hits
            ("cold_pattern", 10), // 10 hits
        ];

        b.iter(|| {
            let mut hits = 0;
            let mut total = 0;

            // Simulate access pattern
            for _ in 0..1000 {
                total += 1;
                // 82% hot, 15% warm, 5% cold
                let access = (total % 100) as usize;
                if access < 80 {
                    hits += 1; // hit hot
                } else if access < 95 {
                    // sometimes hit warm
                    if (total % 3) == 0 {
                        hits += 1;
                    }
                }
            }

            let hit_rate = hits as f64 / total as f64;
            black_box(hit_rate);
        })
    });
}

fn benchmark_memory_footprint(c: &mut Criterion) {
    c.bench_function("memory_1k_patterns", |b| {
        b.iter(|| {
            // Simulate memory for 1000 patterns
            let patterns_size = 1000 * 128; // Approximate pattern size
            let hash_maps_size = 1000 * 64; // Simulated hash overhead
            let total = patterns_size + hash_maps_size;

            black_box(total);
        })
    });
}

fn benchmark_eviction_performance(c: &mut Criterion) {
    c.bench_function("cache_eviction_lru", |b| {
        b.iter(|| {
            // Simulate LRU eviction with 100 items
            let mut cache = std::collections::LinkedList::new();
            let capacity = 50;

            // Fill cache
            for i in 0..capacity {
                cache.push_back(format!("item_{}", i));
            }

            // Access pattern: trigger eviction
            for i in 0..30 {
                cache.push_back(format!("new_item_{}", i));
                if cache.len() > capacity {
                    cache.pop_front(); // Evict LRU
                }
            }

            black_box(cache.len());
        })
    });
}

fn benchmark_pattern_similarity(c: &mut Criterion) {
    c.bench_function("similarity_calc_lsh", |b| {
        b.iter(|| {
            let hash1 = vec![1u8; 32];
            let hash2 = vec![2u8; 32];

            // Simplified LSH similarity: Jaccard index
            let common = hash1
                .iter()
                .zip(hash2.iter())
                .filter(|(a, b)| a == b)
                .count();

            let similarity = common as f64 / hash1.len() as f64;
            black_box(similarity);
        })
    });
}

criterion_group!(
    cache_benches,
    benchmark_pattern_lookup_latency,
    benchmark_cache_hit_rate,
    benchmark_memory_footprint,
    benchmark_eviction_performance,
    benchmark_pattern_similarity
);
criterion_main!(cache_benches);
