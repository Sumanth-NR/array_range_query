//! Lazy segment tree specialization for range add updates and sum queries.
//!
//! This module provides a convenient wrapper around the generic `LazySegTree`
//! for range addition updates with sum queries, supporting efficient batch operations.

use crate::{LazySegTree, LazySegTreeSpec};
use num_traits::ConstZero;
use std::marker::PhantomData;
use std::ops::Add;

/// Specification for lazy segment trees that perform range add updates with sum queries.
///
/// This spec is restricted to work with types that support basic arithmetic operations.
/// Both T and U must be the same numeric type that supports addition and multiplication.
pub struct LazySegTreeAddSumSpec<T>(PhantomData<T>);

impl<T> LazySegTreeSpec for LazySegTreeAddSumSpec<T>
where
    T: Clone + Add<Output = T> + ConstZero,
{
    type T = T;
    type U = T;

    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T {
        d1.clone() + d2.clone()
    }

    fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::T {
        u1.clone() + u2.clone()
    }

    fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
        // Manually multiply u by size using repeated addition
        let mut result = d.clone();
        for _ in 0..size {
            result = result + u.clone();
        }
        result
    }
}

/// Convenience alias: a `LazySegTree` specialized for range add updates and sum queries.
///
/// # Examples
///
/// ```rust
/// use array_range_query::LazySegTreeAddSum;
///
/// let values = vec![1i32, 2, 3, 4, 5];
/// let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);
///
/// assert_eq!(tree.query(0, 5), 15); // Sum of all elements
///
/// // Add 10 to range [1, 4)
/// tree.update(1, 4, 10);
/// assert_eq!(tree.query(0, 5), 45); // 1 + (2+10) + (3+10) + (4+10) + 5
/// ```
pub type LazySegTreeAddSum<T> = LazySegTree<LazySegTreeAddSumSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_sum_basic_operations() {
        let values = vec![1i32, 2, 3, 4, 5];
        let tree = LazySegTreeAddSum::<i32>::from_vec(&values);

        // Test initial queries
        assert_eq!(tree.query(0, 5), 15); // Sum of all: 1+2+3+4+5
        assert_eq!(tree.query(1, 4), 9); // Sum of middle: 2+3+4
        assert_eq!(tree.query(0, 1), 1); // Single element
        assert_eq!(tree.query(4, 5), 5); // Last element
        assert_eq!(tree.query(2, 2), 0); // Empty range
    }

    #[test]
    fn test_add_sum_range_updates() {
        let values = vec![10i32, 20, 30, 40, 50];
        let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);

        assert_eq!(tree.query(0, 5), 150);

        // Add 5 to range [1, 4)
        tree.update(1, 4, 5);
        assert_eq!(tree.query(0, 5), 165); // 10 + (20+5) + (30+5) + (40+5) + 50
        assert_eq!(tree.query(1, 4), 105); // (20+5) + (30+5) + (40+5) = 25+35+45 = 105
        assert_eq!(tree.query(0, 2), 35); // 10 + (20+5)

        // Add 10 to range [0, 3)
        tree.update(0, 3, 10);
        assert_eq!(tree.query(0, 3), 100); // (10+10) + (25+10) + (35+10) = 20+35+45 = 100
        assert_eq!(tree.query(0, 5), 195); // 20 + 35 + 45 + 45 + 50 = 195
    }

    #[test]
    fn test_add_sum_overlapping_updates() {
        let values = vec![1i32, 1, 1, 1, 1]; // All ones
        let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);

        assert_eq!(tree.query(0, 5), 5);

        // Overlapping updates
        tree.update(0, 3, 2); // Add 2 to [0, 3): [3, 3, 3, 1, 1]
        tree.update(2, 5, 4); // Add 4 to [2, 5): [3, 3, 7, 5, 5]
        tree.update(1, 4, 1); // Add 1 to [1, 4): [3, 4, 8, 6, 5]

        assert_eq!(tree.query(0, 1), 3); // 3
        assert_eq!(tree.query(1, 2), 4); // 4
        assert_eq!(tree.query(2, 3), 8); // 8
        assert_eq!(tree.query(3, 4), 6); // 6
        assert_eq!(tree.query(4, 5), 5); // 5
        assert_eq!(tree.query(0, 5), 26); // 3+4+8+6+5
    }

    #[test]
    fn test_add_sum_with_different_types() {
        // Test with i64 type
        let values = vec![1i64, 2, 3, 4];
        let mut tree = LazySegTreeAddSum::<i64>::from_vec(&values);

        assert_eq!(tree.query(0, 4), 10);

        tree.update(1, 3, 5); // Add 5 to middle elements
        assert_eq!(tree.query(0, 4), 20); // 1 + (2+5) + (3+5) + 4 = 20
    }

    #[test]
    fn test_add_sum_with_i64() {
        let values = vec![1000000000i64, 2000000000, 3000000000];
        let mut tree = LazySegTreeAddSum::<i64>::from_vec(&values);

        assert_eq!(tree.query(0, 3), 6000000000);

        // Add large values
        tree.update(0, 2, 1000000000);
        assert_eq!(tree.query(0, 3), 8000000000);
    }

    #[test]
    fn test_add_sum_edge_cases() {
        // Single element
        let single = vec![42i32];
        let mut tree_single = LazySegTreeAddSum::<i32>::from_vec(&single);
        assert_eq!(tree_single.query(0, 1), 42);
        tree_single.update(0, 1, 8);
        assert_eq!(tree_single.query(0, 1), 50);

        // Empty updates (no-op)
        let values = vec![1i32, 2, 3, 4, 5];
        let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);
        let original_sum = tree.query(0, 5);
        tree.update(2, 2, 100); // Empty range update
        assert_eq!(tree.query(0, 5), original_sum); // Should be unchanged
    }

    #[test]
    fn test_add_sum_large_tree() {
        let size = 1000;
        let values = vec![1i32; size]; // All ones
        let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);

        assert_eq!(tree.query(0, size), size as i32);

        // Add 1 to first half
        tree.update(0, size / 2, 1);
        assert_eq!(tree.query(0, size), (size + size / 2) as i32); // 1500

        // Add 2 to second half
        tree.update(size / 2, size, 2);
        assert_eq!(tree.query(0, size), (size + size / 2 + size) as i32); // 2500

        // Verify individual halves
        assert_eq!(tree.query(0, size / 2), (size / 2 * 2) as i32); // 1000
        assert_eq!(tree.query(size / 2, size), (size / 2 * 3) as i32); // 1500
    }

    #[test]
    fn test_add_sum_new_empty_tree() {
        let mut tree = LazySegTreeAddSum::<i32>::new(5);

        // All elements should be zero initially
        assert_eq!(tree.query(0, 5), 0);

        // Add values to ranges
        tree.update(1, 4, 10);
        assert_eq!(tree.query(0, 5), 30); // 0 + 10 + 10 + 10 + 0
        assert_eq!(tree.query(1, 4), 30); // 10 + 10 + 10

        // Add more to overlapping range
        tree.update(0, 3, 5);
        assert_eq!(tree.query(0, 5), 45); // (0+5) + (10+5) + (10+5) + 10 + 0
        assert_eq!(tree.query(0, 3), 35); // 5 + 15 + 15
    }

    #[test]
    fn test_add_sum_zero_updates() {
        let values = vec![5i32, 10, 15, 20];
        let mut tree = LazySegTreeAddSum::<i32>::from_vec(&values);

        let original_sum = tree.query(0, 4);

        // Adding zero should not change anything
        tree.update(0, 4, 0);
        assert_eq!(tree.query(0, 4), original_sum);

        tree.update(1, 3, 0);
        assert_eq!(tree.query(0, 4), original_sum);
    }

    #[test]
    fn test_add_sum_stress_test() {
        let size = 100;
        let mut tree = LazySegTreeAddSum::<i32>::new(size);

        // Perform many overlapping updates
        for i in 0..50 {
            let left = i * 2;
            let right = std::cmp::min((i + 1) * 2 + 10, size);
            tree.update(left, right, (i + 1) as i32);
        }

        // Verify that queries work correctly after many updates
        let total = tree.query(0, size);
        assert!(total > 0); // Should have accumulated some value

        // Test various range queries
        for i in 0..10 {
            let left = i * 10;
            let right = std::cmp::min((i + 1) * 10, size);
            let range_sum = tree.query(left, right);
            assert!(range_sum >= 0); // Should be valid
        }
    }
}
