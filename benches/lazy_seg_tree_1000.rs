use core::cell::RefCell;
use core::hint::black_box;
use std::path::Path;

use criterion::{criterion_group, criterion_main, Criterion};

use array_range_query::LazySegTreeAddSum;

/// Size used for the benchmarks.
const SIZE: usize = 1000;

/// A tiny deterministic linear congruential generator so we don't need external crates.
/// Not cryptographically secure — only for reproducible pseudo-random inputs in benchmarks.
#[derive(Clone)]
struct Lcg(u64);

impl Lcg {
    fn new(seed: u64) -> Self {
        Self(seed)
    }

    fn next_u64(&mut self) -> u64 {
        // Parameters from Numerical Recipes (common LCG)
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }

    fn next_usize(&mut self, max: usize) -> usize {
        (self.next_u64() as usize) % max
    }
}

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

    // Use a RefCell-wrapped RNG so the same RNG state is used across iterations.
    let rng = RefCell::new(Lcg::new(0xC0FFEE));

    c.bench_function("lazy_seg_tree_range_size_random_query_1000", |b| {
        b.iter_batched(
            || {
                let mut r = rng.borrow_mut();
                let num1 = r.next_usize(SIZE);
                let num2 = r.next_usize(SIZE);
                if num1 <= num2 {
                    (num1, num2)
                } else {
                    (num2, num1)
                }
            },
            |(left, right)| {
                // Perform exactly one query for range [l, rgt)
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
                let mut r = rng.borrow_mut();
                let left = r.next_usize(SIZE - window);
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

    let rng = RefCell::new(Lcg::new(0xFEED_FACE));

    // We'll perform 10 random range-add updates per iteration.
    c.bench_function("lazy_seg_tree_range_size_random_update_1000", |b| {
        b.iter_batched(
            || {
                let mut r = rng.borrow_mut();
                let a = r.next_usize(SIZE);
                let bidx = r.next_usize(SIZE);
                let (left, right) = if a <= bidx { (a, bidx) } else { (bidx, a) };
                let val = (r.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
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
                let mut r = rng.borrow_mut();
                let a = r.next_usize(SIZE);
                let bidx = r.next_usize(SIZE);
                let (left, right) = if a <= bidx { (a, bidx) } else { (bidx, a) };
                let val = (r.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
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
