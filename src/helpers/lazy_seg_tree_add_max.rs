//! Lazy segment tree specialization for range add updates and maximum queries.
//!
//! This module provides a convenient wrapper around the generic `LazySegTree`
//! for range addition updates with maximum queries, supporting efficient batch operations.

use crate::{LazySegTree, LazySegTreeSpec};
use min_max_traits::Min as ConstLowerBound;
use std::marker::PhantomData;
use std::ops::Add;

/// Specification for lazy segment trees that perform range add updates with maximum queries.
///
/// This spec works with data type `T` where:
/// - `T` supports addition and multiplication by usize
/// - `T` has a zero constant and supports ordering
/// - Updates are applied uniformly to all elements in a range
pub struct LazySegTreeAddMaxSpec<T>(PhantomData<T>);

impl<T> LazySegTreeSpec for LazySegTreeAddMaxSpec<T>
where
    T: Clone + Add<Output = T> + ConstLowerBound + Ord,
{
    type T = T;
    type U = T;

    const ID: Self::T = <T as ConstLowerBound>::MIN;

    fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
        if *d1 < *d2 {
            *d1 = d2.clone();
        }
    }

    fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
        *u1 = u1.clone() + u2.clone();
    }

    fn op_update_on_data(u: &Self::U, d: &mut Self::T, _size: usize) {
        *d = d.clone() + u.clone();
    }
}

/// Convenience alias: a `LazySegTree` specialized for range add updates and maximum queries.
///
/// # Examples
///
/// ```rust
/// use array_range_query::LazySegTreeAddMax;
///
/// let values = vec![1, 3, 2, 5, 4];
/// let mut tree = LazySegTreeAddMax::<i64>::from_vec(values);
///
/// assert_eq!(tree.query(..), 5); // Maximum of all elements
///
/// // Add 10 to range [1, 4)
/// tree.update(1..4, 10);
/// assert_eq!(tree.query(..), 15); // 1, max(13, 12, 15), 4 = 15
/// ```
pub type LazySegTreeAddMax<T> = LazySegTree<LazySegTreeAddMaxSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_max_basic_operations() {
        let values = vec![1i32, 3, 2, 5, 4];
        let tree = LazySegTreeAddMax::<i32>::from_vec(values);

        // Test initial queries
        assert_eq!(tree.query(..), 5); // Max of all: max(1,3,2,5,4) = 5
        assert_eq!(tree.query(1..4), 5); // Max of middle: max(3,2,5) = 5
        assert_eq!(tree.query(..1), 1); // Single element
        assert_eq!(tree.query(3..4), 5); // Single element
        assert_eq!(tree.query(2..2), i32::MIN); // Empty range returns MIN (ID)
    }

    #[test]
    fn test_add_max_range_updates() {
        let values = vec![10i32, 20, 30, 40, 50];
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);

        assert_eq!(tree.query(..), 50);

        // Add 5 to range [1, 4)
        tree.update(1..4, 5);
        // Values become: [10, 25, 35, 45, 50]
        assert_eq!(tree.query(..), 50); // max(10, 25, 35, 45, 50) = 50
        assert_eq!(tree.query(1..4), 45); // max(25, 35, 45) = 45
        assert_eq!(tree.query(..2), 25); // max(10, 25) = 25

        // Add 20 to range [0, 3)
        tree.update(..3, 20);
        // Values become: [30, 45, 55, 45, 50]
        assert_eq!(tree.query(..3), 55); // max(30, 45, 55) = 55
        assert_eq!(tree.query(..), 55); // max(30, 45, 55, 45, 50) = 55
    }

    #[test]
    fn test_add_max_overlapping_updates() {
        let values = vec![1i32, 1, 1, 1, 1]; // All ones
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);

        assert_eq!(tree.query(..), 1);

        // Overlapping updates
        tree.update(..3, 2); // Add 2 to [0, 3): [3, 3, 3, 1, 1]
        tree.update(2..5, 4); // Add 4 to [2, 5): [3, 3, 7, 5, 5]
        tree.update(1..4, 1); // Add 1 to [1, 4): [3, 4, 8, 6, 5]

        assert_eq!(tree.query(..1), 3); // 3
        assert_eq!(tree.query(1..2), 4); // 4
        assert_eq!(tree.query(2..3), 8); // 8
        assert_eq!(tree.query(3..4), 6); // 6
        assert_eq!(tree.query(4..5), 5); // 5
        assert_eq!(tree.query(..), 8); // max(3,4,8,6,5) = 8
    }

    #[test]
    fn test_add_max_negative_updates() {
        let values = vec![10i32, 20, 30, 40, 50];
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);

        assert_eq!(tree.query(..), 50);

        // Subtract (negative add) from range
        tree.update(1..4, -5);
        // Values become: [10, 15, 25, 35, 50]
        assert_eq!(tree.query(..), 50); // max(10, 15, 25, 35, 50) = 50
        assert_eq!(tree.query(1..4), 35); // max(15, 25, 35) = 35

        // Mix positive and negative updates
        tree.update(..2, -15); // Subtract 15 from first two
                               // Values become: [-5, 0, 25, 35, 50]
        assert_eq!(tree.query(..2), 0); // max(-5, 0) = 0
        assert_eq!(tree.query(..), 50); // max(-5, 0, 25, 35, 50) = 50
    }

    #[test]
    fn test_add_max_edge_cases() {
        // Single element
        let single = vec![42i32];
        let mut tree_single = LazySegTreeAddMax::<i32>::from_vec(single);
        assert_eq!(tree_single.query(..), 42);
        tree_single.update(..1, 8);
        assert_eq!(tree_single.query(..), 50);

        // Empty updates (no-op)
        let values = vec![1i32, 2, 3, 4, 5];
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);
        let original_max = tree.query(..);
        tree.update(2..2, 100); // Empty range update
        assert_eq!(tree.query(..), original_max); // Should be unchanged
    }

    #[test]
    fn test_add_max_large_tree() {
        let size = 1000;
        let values: Vec<i64> = (1..=size as i64).collect(); // 1 to 1000
        let mut tree = LazySegTreeAddMax::<i64>::from_vec(values);

        assert_eq!(tree.query(..), size as i64); // Maximum is 1000

        // Add 1000 to first half
        tree.update(..size / 2, 1000);
        // First half becomes: 1001 to 1500, second half remains: 501 to 1000
        assert_eq!(tree.query(..), 1500); // New maximum
        assert_eq!(tree.query(..size / 2), 1500); // First half max
        assert_eq!(tree.query(size / 2..), 1000); // Second half max
    }

    #[test]
    fn test_add_max_new_empty_tree() {
        let mut tree = LazySegTreeAddMax::<i32>::new(5);

        // All elements should be MIN initially
        assert_eq!(tree.query(..), i32::MIN);

        // Add values to ranges
        tree.update(1..4, 10);
        assert_eq!(tree.query(..), i32::MIN + 10);
        assert_eq!(tree.query(4..5), i32::MIN);

        // Add more to overlapping range
        tree.update(..3, 5);
        assert_eq!(tree.query(..), i32::MIN + 15);
        assert_eq!(tree.query(3..), i32::MIN + 10);
    }

    #[test]
    fn test_add_max_zero_updates() {
        let values = vec![5i32, 10, 15, 20];
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);

        let original_max = tree.query(..4);

        // Adding zero should not change anything
        tree.update(..4, 0);
        assert_eq!(tree.query(..4), original_max);

        tree.update(1..3, 0);
        assert_eq!(tree.query(..4), original_max);
    }

    #[test]
    fn test_add_max_all_negative() {
        let values = vec![-10i32, -5, -15, -3, -8];
        let mut tree = LazySegTreeAddMax::<i32>::from_vec(values);

        assert_eq!(tree.query(..), -3); // max(-10, -5, -15, -3, -8) = -3

        // Add positive value to make some elements positive
        tree.update(1..4, 10);
        // Values become: [-10, 5, -5, 7, -8]
        assert_eq!(tree.query(..), 7); // max(-10, 5, -5, 7, -8) = 7
        assert_eq!(tree.query(1..4), 7); // max(5, -5, 7) = 7
    }

    #[test]
    fn test_add_max_stress_test() {
        let size = 100;
        let mut tree = LazySegTreeAddMax::<i32>::new(size);
        let mut vec = vec![i32::MIN; size];

        // Perform many overlapping updates
        for i in 0..50 {
            let left = i * 2;
            let right = std::cmp::min((i + 1) * 2 + 10, size);
            tree.update(left..right, (i + 1) as i32);
            for item in &mut vec[left..right] {
                *item += (i + 1) as i32;
            }
        }

        // Verify that queries work correctly after many updates
        let total_max = tree.query(..);
        let expected_max = vec.iter().max().unwrap_or(&i32::MIN);
        assert_eq!(
            total_max, *expected_max,
            "Expected max value: {}",
            expected_max
        );

        // Test various range queries
        for i in 0..10 {
            let left = i * 10;
            let right = std::cmp::min((i + 1) * 10, size);
            let range_max = tree.query(left..right);
            let expected_max = vec[left..right].iter().max().unwrap_or(&i32::MIN);
            assert_eq!(range_max, *expected_max);
        }
    }
}
