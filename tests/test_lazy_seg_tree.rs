//! Comprehensive tests for a LazySegTree with a complex TreeSpec.
//!
//! The TreeSpec tracks (sum, min, max) and supports Range Add and Range Replace updates.
//! This file contains a specific sequential test and a powerful randomized,
//! property-based test to ensure correctness.

#[cfg(test)]
mod comprehensive_test_lazy_seg_tree {
    use array_range_query::{LazySegTree, LazySegTreeSpec};
    use rand::Rng;

    #[derive(Clone, Debug, PartialEq)]
    enum UpdateType {
        Add(i32),
        Replace(i32),
    }

    struct TreeSpec;
    type Type = (i64, i32, i32);
    impl LazySegTreeSpec for TreeSpec {
        type T = Type;
        const ID: Self::T = (0, i32::MAX, i32::MIN);
        type U = UpdateType;

        fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
            d1.0 += d2.0;
            d1.1 = d1.1.min(d2.1);
            d1.2 = d1.2.max(d2.2);
        }

        fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
            match u2 {
                UpdateType::Replace(x) => *u1 = UpdateType::Replace(*x),
                UpdateType::Add(x) => match u1 {
                    UpdateType::Replace(y) => *u1 = UpdateType::Replace(*y + *x),
                    UpdateType::Add(y) => *u1 = UpdateType::Add(*y + *x),
                },
            }
        }

        fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
            match u {
                UpdateType::Replace(x) => {
                    d.0 = *x as i64 * size as i64;
                    d.1 = *x;
                    d.2 = *x;
                }
                UpdateType::Add(x) => {
                    d.0 += *x as i64 * size as i64;
                    d.1 += *x;
                    d.2 += *x;
                }
            }
        }
    }

    // A simple "brute force" implementation to check our results against.
    // It calculates the (sum, min, max) by iterating over a slice.
    fn brute_force_query(slice: &[i32]) -> (i64, i32, i32) {
        if slice.is_empty() {
            return (0, i32::MAX, i32::MIN); // Identity
        }
        let mut sum = 0i64;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        for &x in slice {
            sum += x as i64;
            min = min.min(x);
            max = max.max(x);
        }
        (sum, min, max)
    }

    /// Verifies queries on a tree built from a predictable sequence.
    #[test]
    fn test_small_range_queries_on_sequence() {
        let size = 10;
        // Create the sequence: -200, -197, -194, ...
        let vec: Vec<i32> = (0..size).map(|i| -200 + (i as i32 * 3)).collect();
        let tree_data: Vec<Type> = vec.iter().map(|&x| (x as i64, x, x)).collect();

        let tree = LazySegTree::<TreeSpec>::from_vec(tree_data);

        // Query all ranges of sizes 1 through 5
        for start in 0..=(size - 5) {
            for length in 1..=5 {
                let end = start + length;
                let range = start..end;
                println!("Querying range {:#?}", range);

                let expected = brute_force_query(&vec[range.clone()]);
                let result = tree.query(range.clone());

                assert_eq!(
                    result, expected,
                    "Query failed for range {:#?} on initial sequence",
                    range
                );
            }
        }
    }

    /// A powerful randomized test that performs thousands of operations.
    #[test]
    fn test_randomized_updates_and_queries() {
        let mut rng = rand::rng();
        let tree_size = 500;

        // Run 10 independent trials
        for trial in 0..10 {
            // 1. Create a random initial state
            let mut vec: Vec<i32> = (0..tree_size)
                .map(|_| rng.random_range(-1000..=1000))
                .collect();

            let tree_data: Vec<(i64, i32, i32)> = vec.iter().map(|&x| (x as i64, x, x)).collect();
            let mut tree = LazySegTree::<TreeSpec>::from_vec(tree_data);

            // 2. Perform 100 random operations on this state
            for op in 0..100 {
                let l = rng.random_range(0..tree_size);
                let r = rng.random_range(l..=tree_size);
                let range = l..r;

                // 50% chance to query, 50% chance to update
                if rng.random_bool(0.5) {
                    // --- QUERY ---
                    println!("Querying range {:#?}...", range);
                    let expected = brute_force_query(&vec[range.clone()]);
                    let result = tree.query(range.clone());
                    assert_eq!(
                        result, expected,
                        "Randomized Query failed. Trial {}, Op {}. Range: {:#?}",
                        trial, op, range
                    );
                } else {
                    // --- UPDATE ---
                    let value = rng.random_range(-100..=100);
                    // 50% chance to Add, 50% to Replace
                    if rng.random_bool(0.5) {
                        // Apply to our simple vector model
                        println!("Adding range {:#?} with value {}", range, value);
                        for item in vec[range.clone()].iter_mut() {
                            *item += value;
                        }
                        // Apply to the tree
                        tree.update(range.clone(), UpdateType::Add(value));
                    } else {
                        // Apply to our simple vector model
                        println!("Replacing range {:#?} with value {}", range, value);
                        for item in vec[range.clone()].iter_mut() {
                            *item = value;
                        }
                        // Apply to the tree
                        tree.update(range.clone(), UpdateType::Replace(value));
                    }
                }
            }
        }
    }
}
