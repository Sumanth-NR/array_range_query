use core::hint::black_box;
use std::path::Path;

use array_range_query::LazySegTreeAddSum;

use criterion::{criterion_group, criterion_main, Criterion};
mod rng;

/// Size used for the benchmarks.
const SIZE: usize = 1000;

fn bench_constructors(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();

    c.bench_function("lazy_seg_tree_new_1000", |b| {
        b.iter(|| {
            let tree = LazySegTreeAddSum::<i64>::new(SIZE);
            black_box(&tree);
        })
    });

    c.bench_function("lazy_seg_tree_from_slice_1000", |b| {
        b.iter(|| {
            let tree = LazySegTreeAddSum::<i64>::from_slice(&values);
            black_box(&tree);
        })
    });

    c.bench_function("lazy_seg_tree_from_vec_1000", |b| {
        use criterion::BatchSize;
        b.iter_batched(
            || values.clone(), // cloned outside the timed closure
            |v| {
                let tree = LazySegTreeAddSum::<i64>::from_vec(v);
                black_box(&tree);
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_range_query(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();
    let tree = LazySegTreeAddSum::<i64>::from_vec(values);

    // Use a plain mutable RNG so the same RNG state is used across iterations.
    let mut rng = rng::Lcg::new(0xC0FFEE);

    c.bench_function("lazy_seg_tree_range_size_random_query_1000", |b| {
        b.iter_batched(
            || {
                let num1 = rng.next_usize(SIZE);
                let num2 = rng.next_usize(SIZE);
                if num1 <= num2 {
                    (num1, num2)
                } else {
                    (num2, num1)
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

    c.bench_function("lazy_seg_tree_range_size_750_query_1000", |b| {
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
        );
    });
}

fn bench_range_update(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();

    // Construct once and perform range updates on the same tree.
    let mut tree = LazySegTreeAddSum::<i64>::from_vec(values.clone());

    let mut rng = rng::Lcg::new(0xFEED_FACE);

    // We'll perform 10 random range-add updates per iteration.
    c.bench_function("lazy_seg_tree_range_size_random_update_1000", |b| {
        b.iter_batched(
            || {
                let a = rng.next_usize(SIZE);
                let bidx = rng.next_usize(SIZE);
                let (left, right) = if a <= bidx { (a, bidx) } else { (bidx, a) };
                let val = (rng.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
                (left, right, val)
            },
            |(left, right, val)| {
                // Update on range [left, right] with value val
                tree.update(left..=right, val);
                black_box(&tree);
            },
            criterion::BatchSize::SmallInput,
        );
    });

    let window = 750usize;
    assert!(window <= SIZE);

    c.bench_function("lazy_seg_tree_range_size_750_update_1000", |b| {
        b.iter_batched(
            || {
                let a = rng.next_usize(SIZE);
                let bidx = rng.next_usize(SIZE);
                let (left, right) = if a <= bidx { (a, bidx) } else { (bidx, a) };
                let val = (rng.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
                (left, right, val)
            },
            |(left, right, val)| {
                // Update on range [left, right] with value val
                tree.update(left..=right, val);
                black_box(&tree);
            },
            criterion::BatchSize::SmallInput,
        );
    });
}

fn criterion_config() -> Criterion {
    Criterion::default().output_directory(Path::new("target/criterion/lazy_seg_tree_1000"))
}

criterion_group! {
    name = benches;
    config = criterion_config();
    targets = bench_constructors,
              bench_range_query,
              bench_range_update
}
criterion_main!(benches);
