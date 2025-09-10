//! Comprehensive Criterion benchmarks for `SegTree`.
//!
//! This benchmark suite exercises the non-lazy `SegTree` implementation across
//! multiple workloads and sizes to evaluate performance characteristics.
//!
//! ## Workloads
//!
//! For each size, we benchmark:
//! - **Construction**: `SegTree::new`, `SegTree::from_slice`, `SegTree::from_slice`
//! - **Updates**: 100 random point updates on a fresh tree
//! - **Queries**: 100 random range queries on a fresh tree
//! - **Mixed**: 100 mixed random operations (updates + queries)
//! - **Micro**: Repeated single operations for overhead analysis
//!
//! ## Configuration
//!
//! Sizes can be controlled via environment variables:
//! - `BENCH_SMALL_SIZE`: Default 100
//! - `BENCH_LARGE_SIZE`: Default 10,000
//! - `BENCH_SKIP_LARGE`: Set to "1" to skip large size benchmarks
//!
//! ## Usage
//!
//! ```bash
//! # Default run
//! cargo bench --bench seg_tree_old --release
//!
//! # CI-friendly run (small size only)
//! BENCH_SKIP_LARGE=1 cargo bench --bench seg_tree_old --release
//!
//! # Custom sizes
//! BENCH_SMALL_SIZE=5000 BENCH_LARGE_SIZE=500000 cargo bench --bench seg_tree_old --release
//! ```

use array_range_query::{SegTree, SegTreeSpec};
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};
use std::env;
use std::ops::Range;

/// Simple deterministic LCG for reproducible pseudo-random sequences.
#[derive(Clone)]
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        // Ensure non-zero state for proper LCG behavior
        Self {
            state: if seed == 0 { 1 } else { seed },
        }
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        // Well-tested LCG constants from Numerical Recipes
        self.state = self.state.wrapping_mul(1664525).wrapping_add(1013904223);
        self.state
    }

    #[inline]
    fn gen_range(&mut self, max: usize) -> usize {
        if max == 0 {
            0
        } else {
            (self.next_u64() as usize) % max
        }
    }

    #[inline]
    fn gen_i64(&mut self) -> i64 {
        self.next_u64() as i64
    }
}

/// Sum specification for benchmarking.
struct SumSpec;
impl SegTreeSpec for SumSpec {
    type T = i64;
    const ID: Self::T = 0;

    fn op(a: &mut Self::T, b: &Self::T) {
        *a += *b;
    }
}

/// Configuration for benchmark sizes.
struct BenchConfig {
    sizes: Vec<usize>,
}

impl BenchConfig {
    fn from_env() -> Self {
        let small_size = env::var("BENCH_SMALL_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(100);

        let large_size = env::var("BENCH_LARGE_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1000);

        let skip_large = env::var("BENCH_SKIP_LARGE")
            .map(|s| s == "1")
            .unwrap_or(false);

        let mut sizes = vec![small_size];
        if !skip_large {
            sizes.push(large_size);
        }

        Self { sizes }
    }
}

const OPS: usize = 100;
const MAX_QUERY_LEN: usize = 100;

fn benchmark_constructors(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
) {
    let base_values: Vec<i64> = (0..size).map(|i| i as i64).collect();

    // SegTree::new
    group.bench_with_input(BenchmarkId::new("constructor_new", size), &size, |b, &s| {
        b.iter(|| {
            let tree = SegTree::<SumSpec>::new(s);
            black_box(tree);
        })
    });

    // SegTree::from_vec (provide owned Vec to avoid cloning inside)
    group.bench_with_input(
        BenchmarkId::new("constructor_from_vec", size),
        &base_values,
        |b, vals| {
            b.iter_batched(
                || vals.clone(),
                |data| {
                    // data is an owned Vec<T>, so use `from_vec`
                    let tree = SegTree::<SumSpec>::from_vec(data);
                    black_box(tree);
                },
                BatchSize::LargeInput, // Cloning is expensive for large sizes
            )
        },
    );

    // SegTree::from_slice (borrow base_values)
    group.bench_with_input(
        BenchmarkId::new("constructor_from_slice", size),
        &base_values,
        |b, vals| {
            b.iter(|| {
                let tree = SegTree::<SumSpec>::from_slice(vals);
                black_box(tree);
            })
        },
    );
}

fn benchmark_updates(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
) {
    let base_values: Vec<i64> = (0..size).map(|i| i as i64).collect();

    // Pre-generate random updates
    let mut rng = Lcg::new(0xDEADBEEF_u64.wrapping_add(size as u64));
    let updates: Vec<(usize, i64)> = (0..OPS)
        .map(|_| (rng.gen_range(size), rng.gen_i64()))
        .collect();

    group.bench_with_input(
        BenchmarkId::new("updates_random_100", size),
        &size,
        |b, &_| {
            b.iter_batched(
                || {
                    // Setup: create tree (borrow base_values) and prepare update list (not measured)
                    let tree = SegTree::<SumSpec>::from_slice(&base_values);
                    (tree, updates.clone())
                },
                |(mut tree, ops)| {
                    // Measurement: perform updates
                    for (idx, val) in ops {
                        tree.update(idx, val);
                    }
                    black_box(tree.query(..)); // Prevent optimization
                },
                BatchSize::SmallInput,
            )
        },
    );
}

fn benchmark_queries(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
) {
    let base_values: Vec<i64> = (0..size).map(|i| i as i64).collect();

    // Pre-generate random query ranges
    let mut rng = Lcg::new(0xCAFEBABE_u64.wrapping_add(size as u64));
    let ranges: Vec<Range<usize>> = (0..OPS)
        .map(|_| {
            let start = rng.gen_range(size);
            let max_len = MAX_QUERY_LEN.min(size);
            let len = rng.gen_range(max_len).saturating_add(1); // At least length 1
            let end = (start + len).min(size);
            start..end
        })
        .collect();

    group.bench_with_input(
        BenchmarkId::new("queries_random_100", size),
        &size,
        |b, &_| {
            b.iter_batched(
                || {
                    // Setup: create tree (borrow base_values) and prepare ranges (not measured)
                    let tree = SegTree::<SumSpec>::from_slice(&base_values);
                    (tree, ranges.clone())
                },
                |(tree, ranges)| {
                    // Measurement: perform queries
                    let mut sum = 0i64;
                    for range in ranges {
                        sum = sum.wrapping_add(tree.query(range));
                    }
                    black_box(sum);
                },
                BatchSize::SmallInput,
            )
        },
    );
}

fn benchmark_mixed_workload(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
) {
    let base_values: Vec<i64> = (0..size).map(|i| i as i64).collect();

    // Pre-generate mixed operations (50% updates, 50% queries)
    let mut rng = Lcg::new(0xFEEDFACE_u64.wrapping_add(size as u64));
    let mut operations = Vec::with_capacity(OPS);

    for _ in 0..OPS {
        if rng.next_u64() % 2 == 0 {
            // Update operation
            let idx = rng.gen_range(size);
            let val = rng.gen_i64();
            operations.push(Operation::Update(idx, val));
        } else {
            // Query operation
            let start = rng.gen_range(size);
            let max_len = MAX_QUERY_LEN.min(size);
            let len = rng.gen_range(max_len).saturating_add(1);
            let end = (start + len).min(size);
            operations.push(Operation::Query(start..end));
        }
    }

    group.bench_with_input(
        BenchmarkId::new("mixed_workload_100", size),
        &size,
        |b, &_| {
            b.iter_batched(
                || {
                    // Setup: create fresh tree (borrow base_values) and prepare operations (not measured)
                    let tree = SegTree::<SumSpec>::from_slice(&base_values);
                    (tree, operations.clone())
                },
                |(mut tree, ops)| {
                    // Measurement: perform mixed operations
                    let mut query_sum = 0i64;
                    for op in ops {
                        match op {
                            Operation::Update(idx, val) => tree.update(idx, val),
                            Operation::Query(range) => {
                                query_sum = query_sum.wrapping_add(tree.query(range));
                            }
                        }
                    }
                    black_box(query_sum);
                },
                BatchSize::SmallInput,
            )
        },
    );
}

fn benchmark_micro_operations(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    size: usize,
) {
    let base_values: Vec<i64> = (0..size).map(|i| i as i64).collect();
    // Borrow base_values for read-only micro benchmarks
    let tree = SegTree::<SumSpec>::from_slice(&base_values);

    // Pick a deterministic index and range for micro-benchmarks
    let mut rng = Lcg::new(size as u64);
    let test_idx = rng.gen_range(size);
    let test_range = {
        let start = test_idx;
        let end = (start + 100).min(size); // Small fixed range
        start..end
    };

    // Micro-benchmark: single update (measure just the update operation)
    group.bench_with_input(
        BenchmarkId::new("micro_single_update", size),
        &size,
        |b, &_| {
            b.iter_batched(
                || {
                    // Setup: create fresh tree for each measurement (borrow base_values)
                    let tree = SegTree::<SumSpec>::from_slice(&base_values);
                    (tree, 0i64)
                },
                |(mut tree, counter)| {
                    // Measurement: single update operation
                    tree.update(test_idx, counter);
                    black_box(&tree);
                },
                BatchSize::SmallInput,
            )
        },
    );

    // Micro-benchmark: single query (measure just the query operation)
    group.bench_with_input(
        BenchmarkId::new("micro_single_query", size),
        &test_range,
        |b, range| {
            b.iter(|| {
                let result = tree.query(range.clone());
                black_box(result);
            })
        },
    );

    // Full-range query (common operation)
    group.bench_with_input(
        BenchmarkId::new("micro_full_range_query", size),
        &size,
        |b, &_| {
            b.iter(|| {
                let result = tree.query(..);
                black_box(result);
            })
        },
    );
}

fn seg_tree_benchmarks(c: &mut Criterion) {
    let config = BenchConfig::from_env();
    let mut group = c.benchmark_group("seg_tree_comprehensive");

    // Configure sampling for faster benchmarks on large inputs
    group.sample_size(30); // Reduce samples for large benchmarks
    group.measurement_time(std::time::Duration::from_secs(10)); // 10s per benchmark

    for &size in &config.sizes {
        eprintln!("Benchmarking SegTree with size: {}", size);

        benchmark_constructors(&mut group, size);
        benchmark_updates(&mut group, size);
        benchmark_queries(&mut group, size);
        benchmark_mixed_workload(&mut group, size);
        benchmark_micro_operations(&mut group, size);
    }

    group.finish();
}

#[derive(Clone)]
enum Operation {
    Update(usize, i64),
    Query(Range<usize>),
}

criterion_group!(benches, seg_tree_benchmarks);
criterion_main!(benches);
