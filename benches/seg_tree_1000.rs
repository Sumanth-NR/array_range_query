// RNG captured mutably by closures; no RefCell needed
use core::hint::black_box;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

use array_range_query::SegTreeSum;

/// Size used for the benchmarks.
const SIZE: usize = 1000;

mod rng;

fn bench_constructors(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();

    c.bench_function("seg_tree_new_1000", |b| {
        b.iter(|| {
            let tree = SegTreeSum::<i64>::new(SIZE);
            // prevent the compiler from optimizing this away
            black_box(&tree);
        })
    });

    c.bench_function("seg_tree_from_slice_1000", |b| {
        b.iter(|| {
            let tree = SegTreeSum::<i64>::from_slice(&values);
            black_box(&tree);
        })
    });

    c.bench_function("seg_tree_from_vec_1000", |b| {
        b.iter_batched(
            || values.clone(), // cloned outside the timed closure
            |v| {
                let tree = SegTreeSum::<i64>::from_vec(v);
                black_box(&tree);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_range_query(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();
    let tree = SegTreeSum::<i64>::from_slice(&values);

    // Use a plain mutable RNG so the same RNG state is used across iterations.
    let mut rng = rng::Lcg::new(0xC0FFEE);

    c.bench_function("seg_tree_range_size_random_query_1000", |b| {
        b.iter_batched(
            || {
                let left = rng.next_usize(SIZE);
                let right = rng.next_usize(SIZE);
                if left <= right {
                    (left, right)
                } else {
                    (right, left)
                }
            },
            |(left, right)| {
                // Perform exactly one query for range [left, right]
                let res = tree.query(left..=right);
                black_box(res);
            },
            criterion::BatchSize::SmallInput,
        )
    });

    let window = 750usize;
    assert!(window <= SIZE);

    c.bench_function("seg_tree_range_size_750_query_1000", |b| {
        b.iter_batched(
            || {
                let left = rng.next_usize(SIZE - window);
                let right = left + window;
                (left, right)
            },
            |(left, right)| {
                // Perform exactly one query for range [left, right]
                let res = tree.query(left..=right);
                black_box(res);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn bench_point_update(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();

    // Construct once (as requested) and perform updates on the same tree.
    let mut tree = SegTreeSum::<i64>::from_vec(values.clone());

    let mut rng = rng::Lcg::new(0xFEED_FACE);

    c.bench_function("seg_tree_point_update_1000", |b| {
        b.iter_batched(
            || {
                let idx = rng.next_usize(SIZE);
                // produce a pseudo-random i64 value (may be negative)
                let val = (rng.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
                (idx, val)
            },
            |(idx, val)| {
                tree.update(idx, val);
                // keep the tree alive and prevent optimization
                black_box(&tree);
            },
            criterion::BatchSize::SmallInput,
        )
    });
}

fn criterion_config() -> Criterion {
    Criterion::default().output_directory(Path::new("target/criterion/seg_tree_1000"))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_constructors,
              bench_range_query,
              bench_point_update,
}
criterion_main!(benches);
