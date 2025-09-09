//! Basic usage examples for array_range_query crate.
//!
//! This example demonstrates how to use both SegTree and LazySegTree
//! for common operations like sum, min, max queries and range updates.

use array_range_query::{
    LazySegTree, LazySegTreeAddMin, LazySegTreeAddSum, LazySegTreeReplaceSum, LazySegTreeSpec,
    SegTree, SegTreeMax, SegTreeMin, SegTreeSpec, SegTreeSum,
};

fn main() {
    println!("=== Basic Segment Tree Examples ===\n");

    // Example 1: Custom SegTree for sum operations
    custom_sum_example();

    // Example 2: Helper types for common operations
    helper_types_example();

    // Example 3: Custom min operation with detailed output
    custom_min_example();

    println!("\n=== Lazy Segment Tree Examples ===\n");

    // Example 4: Custom LazySegTree for range add + range sum
    custom_lazy_example();

    // Example 5: Using LazySegTree helper types
    lazy_helper_example();

    // Example 6: Advanced lazy operations
    advanced_lazy_example();
}

/// Example 1: Custom SegTree implementation for sum queries
fn custom_sum_example() {
    println!("1. Custom SegTree for Sum Operations");
    println!("-----------------------------------");

    // Define a custom spec for sum operations
    struct SumSpec;

    impl SegTreeSpec for SumSpec {
        type T = i64;
        const ID: Self::T = 0; // Identity element for addition

        fn op(a: &Self::T, b: &Self::T) -> Self::T {
            a + b
        }
    }

    let values = vec![1, 2, 3, 4, 5];
    let mut seg_tree = SegTree::<SumSpec>::from_vec(&values);

    println!("Initial values: {:?}", values);
    println!("Sum of range [1, 4): {}", seg_tree.query(1..4)); // 2 + 3 + 4 = 9
    println!("Sum of entire array [0, 5): {}", seg_tree.query(..)); // 15

    // Update element at index 2 from 3 to 10
    seg_tree.update(2, 10);
    println!("\nAfter updating index 2 to 10:");
    println!("Sum of range [1, 4): {}", seg_tree.query(1..4)); // 2 + 10 + 4 = 16
    println!("Sum of entire array [0, 5): {}", seg_tree.query(..)); // 22
    println!();
}

/// Example 2: Using helper types for common operations
fn helper_types_example() {
    println!("2. Helper Types for Common Operations");
    println!("------------------------------------");

    let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("Values: {:?}", values);

    // Sum operations
    let mut sum_tree = SegTreeSum::<i32>::from_vec(&values);
    println!("Range sum [2, 6): {}", sum_tree.query(2..6)); // 4 + 1 + 5 + 9 = 19

    // Min operations
    let mut min_tree = SegTreeMin::<i32>::from_vec(&values);
    println!("Range min [2, 6): {}", min_tree.query(2..6)); // min(4, 1, 5, 9) = 1

    // Max operations
    let mut max_tree = SegTreeMax::<i32>::from_vec(&values);
    println!("Range max [2, 6): {}", max_tree.query(2..6)); // max(4, 1, 5, 9) = 9

    // Demonstrate updates
    println!("\nAfter updating index 3 to 20:");
    sum_tree.update(3, 20);
    min_tree.update(3, 20);
    max_tree.update(3, 20);

    println!("Range sum [2, 6): {}", sum_tree.query(2..6)); // 4 + 20 + 5 + 9 = 38
    println!("Range min [2, 6): {}", min_tree.query(2..6)); // min(4, 20, 5, 9) = 4
    println!("Range max [2, 6): {}", max_tree.query(2..6)); // max(4, 20, 5, 9) = 20
    println!();
}

/// Example 3: Custom min operation with detailed explanation
fn custom_min_example() {
    println!("3. Custom Min Operation with Details");
    println!("-----------------------------------");

    struct MinSpec;

    impl SegTreeSpec for MinSpec {
        type T = i32;
        const ID: Self::T = i32::MAX; // Identity for min is maximum possible value

        fn op(a: &Self::T, b: &Self::T) -> Self::T {
            (*a).min(*b)
        }
    }

    let values = vec![7, 3, 9, 1, 6, 2, 8, 4];
    let seg_tree = SegTree::<MinSpec>::from_vec(&values);

    println!("Values: {:?}", values);
    println!("Min of entire array: {}", seg_tree.query(..)); // 1
    println!("Min of [2, 5): {}", seg_tree.query(2..5)); // min(9, 1, 6) = 1
    println!("Min of [0, 3): {}", seg_tree.query(..3)); // min(7, 3, 9) = 3
    println!("Min of [5, 8): {}", seg_tree.query(5..)); // min(2, 8, 4) = 2
    println!();
}

/// Example 4: Custom LazySegTree for range add + range sum
fn custom_lazy_example() {
    println!("4. Custom LazySegTree for Range Add + Range Sum");
    println!("----------------------------------------------");

    // Define a spec for range add operations with sum queries
    struct RangeAddSum;

    impl LazySegTreeSpec for RangeAddSum {
        type T = i64; // Data type (stores sums)
        type U = i64; // Update type (add values)
        const ID: Self::T = 0;

        // Combine two sum values
        fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T {
            d1 + d2
        }

        // Compose two add operations
        fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U {
            u1 + u2
        }

        // Apply add operation to sum (multiply by range size)
        fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
            d + u * (size as i64)
        }
    }

    let values = vec![1i64, 2, 3, 4, 5];
    let mut lazy_tree = LazySegTree::<RangeAddSum>::from_vec(&values);

    println!("Initial values: {:?}", values);
    println!("Initial sum [1, 4): {}", lazy_tree.query(1..4)); // 2 + 3 + 4 = 9
    println!("Initial sum [0, 5): {}", lazy_tree.query(..)); // 15

    // Add 10 to all elements in range [1, 4)
    lazy_tree.update(1..4, 10);
    println!("\nAfter adding 10 to range [1, 4):");
    println!("Sum [1, 4): {}", lazy_tree.query(1..4)); // (2+10) + (3+10) + (4+10) = 39
    println!("Sum [0, 5): {}", lazy_tree.query(..)); // 1 + 12 + 13 + 14 + 5 = 45

    // Add 5 to range [0, 2)
    lazy_tree.update(..2, 5);
    println!("\nAfter adding 5 to range [0, 2):");
    println!("Sum [0, 2): {}", lazy_tree.query(..2)); // (1+5) + (12+5) = 23
    println!("Sum [0, 5): {}", lazy_tree.query(..)); // 6 + 17 + 13 + 14 + 5 = 55
    println!();
}

/// Example 5: Using LazySegTree helper types
fn lazy_helper_example() {
    println!("5. LazySegTree Helper Types");
    println!("--------------------------");

    // Range add + range sum
    let values = vec![2i32, 4, 6, 8, 10];
    let mut sum_tree = LazySegTreeAddSum::<i32>::from_vec(&values);

    println!("Initial values: {:?}", values);
    println!("Sum [0, 5): {}", sum_tree.query(..)); // 30
    println!("Sum [1, 4): {}", sum_tree.query(1..4)); // 4 + 6 + 8 = 18

    // Add 3 to range [1, 4)
    sum_tree.update(1..4, 3);
    println!("\nAfter adding 3 to range [1, 4):");
    println!("Sum [0, 5): {}", sum_tree.query(..)); // 2 + 7 + 9 + 11 + 10 = 39
    println!("Sum [1, 4): {}", sum_tree.query(1..4)); // 7 + 9 + 11 = 27

    // Add -2 to range [0, 3)
    sum_tree.update(..3, -2);
    println!("\nAfter subtracting 2 from range [0, 3):");
    println!("Sum [0, 5): {}", sum_tree.query(..)); // 0 + 5 + 7 + 11 + 10 = 33
    println!("Sum [0, 3): {}", sum_tree.query(..3)); // 0 + 5 + 7 = 12

    // Range add + range min
    let min_values = vec![5, 2, 8, 1, 9, 3];
    let mut min_tree = LazySegTreeAddMin::<i32>::from_vec(&min_values);
    println!("\nInitial min values: {:?}", min_values);
    println!("Min [0, 6): {}", min_tree.query(..)); // 1
    min_tree.update(1..4, 2);
    println!(
        "After adding 2 to [1, 4): Min [0, 6): {}",
        min_tree.query(..)
    ); // min(5, 4, 10, 3, 9, 3) = 3

    // Range assignment (replace) + range sum
    let rep_values = vec![1, 2, 3, 4, 5];
    let mut rep_tree = LazySegTreeReplaceSum::<i32>::from_vec(&rep_values);
    println!("\nInitial replace values: {:?}", rep_values);
    println!("Sum [0, 5): {}", rep_tree.query(..)); // 15
    rep_tree.update(1..4, 10); // Replace [1, 4) with 10
    println!(
        "After replacing [1, 4) with 10: Sum [0, 5): {}",
        rep_tree.query(..)
    ); // 1 + 10 + 10 + 10 + 5 = 36
    println!();
}

/// Example 6: Advanced lazy operations with overlapping updates
fn advanced_lazy_example() {
    println!("6. Advanced Lazy Operations");
    println!("--------------------------");

    let values = vec![1i32, 1, 1, 1, 1, 1, 1, 1]; // 8 ones
    let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);

    println!("Initial: 8 ones, sum = {}", tree.query(..));

    // Perform overlapping updates
    tree.update(..4, 2); // Add 2 to first half
    tree.update(2..6, 3); // Add 3 to middle section (overlaps)
    tree.update(4.., 1); // Add 1 to second half

    println!("After overlapping updates:");
    println!("Range [0, 2): sum = {}", tree.query(..2)); // (1+2) + (1+2) = 6
    println!("Range [2, 4): sum = {}", tree.query(2..4)); // (1+2+3) + (1+2+3) = 12
    println!("Range [4, 6): sum = {}", tree.query(4..6)); // (1+3+1) + (1+3+1) = 10
    println!("Range [6, 8): sum = {}", tree.query(6..)); // (1+1) + (1+1) = 4
    println!("Total sum: {}", tree.query(..)); // 6 + 12 + 10 + 4 = 32

    // Verify by querying individual ranges
    let mut total = 0;
    for i in 0..4 {
        let range_sum = tree.query(i * 2..(i + 1) * 2);
        total += range_sum;
        println!("  Range [{}, {}): {}", i * 2, (i + 1) * 2, range_sum);
    }
    println!("Sum verification: {}", total);
    println!();
}
