use std::cell::RefCell;
use std::hint::black_box;

use criterion::{criterion_group, criterion_main, Criterion};

use array_range_query::SegTreeSum;

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
        b.iter(|| {
            // clone values so each iteration owns a fresh Vec
            let tree = SegTreeSum::<i64>::from_vec(values.clone());
            black_box(&tree);
        })
    });
}

fn bench_query_random_ranges_10(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();
    let tree = SegTreeSum::<i64>::from_slice(&values);

    // Use a RefCell-wrapped RNG so the same RNG state is used across iterations.
    let rng = RefCell::new(Lcg::new(0xC0FFEE));

    c.bench_function("seg_tree_query_random_10_1000", |b| {
        b.iter(|| {
            let mut acc: i64 = 0;
            for _ in 0..10 {
                let mut r = rng.borrow_mut();
                let a = r.next_usize(SIZE);
                let bidx = r.next_usize(SIZE);
                let (l, rgt) = if a <= bidx {
                    (a, bidx + 1)
                } else {
                    (bidx, a + 1)
                };
                // Query range [l, rgt)
                acc = acc.wrapping_add(tree.query(l..rgt));
            }
            black_box(acc);
        })
    });
}

fn bench_query_fixed_size_750_10(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();
    let tree = SegTreeSum::<i64>::from_slice(&values);

    let rng = RefCell::new(Lcg::new(0xDEADBEEF));
    let window = 750usize;
    assert!(window <= SIZE);

    c.bench_function("seg_tree_query_range_size_750_10_1000", |b| {
        b.iter(|| {
            let mut acc: i64 = 0;
            for _ in 0..10 {
                let mut r = rng.borrow_mut();
                let left = r.next_usize(SIZE - window + 1);
                let right = left + window;
                acc = acc.wrapping_add(tree.query(left..right));
            }
            black_box(acc);
        })
    });
}

fn bench_updates_random_elements_10(c: &mut Criterion) {
    let values: Vec<i64> = (1..=SIZE as i64).collect();

    // Construct once (as requested) and perform updates on the same tree.
    let mut tree = SegTreeSum::<i64>::from_vec(values.clone());

    let rng = RefCell::new(Lcg::new(0xFEED_FACE));

    c.bench_function("seg_tree_update_random_10_1000", |b| {
        b.iter(|| {
            for _ in 0..10 {
                let mut r = rng.borrow_mut();
                let idx = r.next_usize(SIZE);
                // produce a pseudo-random i64 value (may be negative)
                let val = (r.next_u64() as i64).wrapping_sub(0x4000_0000_0000_0000u64 as i64);
                tree.update(idx, val);
            }
            // keep the tree alive and prevent optimization
            black_box(&tree);
        })
    });
}

criterion_group!(
    benches,
    bench_constructors,
    bench_query_random_ranges_10,
    bench_query_fixed_size_750_10,
    bench_updates_random_elements_10
);
criterion_main!(benches);
