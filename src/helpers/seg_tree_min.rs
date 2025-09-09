//! Segment tree specialization for minimum operations.
//!
//! This module provides a convenient wrapper around the generic `SegTree`
//! for minimum queries with automatic maximum identity element.

use crate::{SegTree, SegTreeSpec};
use min_max_traits::Max as ConstUpperBound;
use std::marker::PhantomData;

/// Specification for segment trees that perform minimum operations.
///
/// This spec works with any type `T` that implements ordering and has
/// a maximum constant. The identity element is automatically set to the
/// maximum value of the type.
pub struct SegTreeMinSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeMinSpec<T>
where
    T: Clone + ConstUpperBound + Ord,
{
    type T = T;
    const ID: Self::T = <T as ConstUpperBound>::MAX;
    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.clone().min(b.clone())
    }
}

/// Convenience alias: a `SegTree` specialized to perform minimum queries over `T`.
///
/// # Examples
///
/// ```
/// use array_range_query::SegTreeMin;
///
/// let values = vec![5, 2, 8, 1, 9, 3];
/// let mut tree = SegTreeMin::<i32>::from_vec(&values);
///
/// assert_eq!(tree.query(..), 1); // Minimum of all elements
/// assert_eq!(tree.query(1..4), 1); // Minimum of elements 2, 8, 1
///
/// tree.update(3, 0); // Change element at index 3 to 0
/// assert_eq!(tree.query(..), 0); // Updated minimum
/// ```
pub type SegTreeMin<T> = SegTree<SegTreeMinSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_basic_operations() {
        let values = vec![5, 2, 8, 1, 9, 3];
        let tree = SegTreeMin::<i32>::from_vec(&values);

        // Test initial queries
        assert_eq!(tree.query(..), 1); // min(5,2,8,1,9,3) = 1
        assert_eq!(tree.query(1..4), 1); // min(2,8,1) = 1
        assert_eq!(tree.query(..1), 5); // Single element
        assert_eq!(tree.query(4..6), 3); // min(9, 3) = 3
        assert_eq!(tree.query(2..2), i32::MAX); // Empty range returns MAX
    }

    #[test]
    fn test_min_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = SegTreeMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 10);

        // Update first element to larger value
        tree.update(0, 100);
        assert_eq!(tree.query(..), 20); // min(100,20,30,40,50) = 20
        assert_eq!(tree.query(..1), 100); // Just the updated element
        assert_eq!(tree.query(1..), 20); // min(20,30,40,50) = 20

        // Update middle element to smallest value
        tree.update(2, 5);
        assert_eq!(tree.query(..), 5); // min(100,20,5,40,50) = 5
        assert_eq!(tree.query(2..3), 5); // Just the updated element
        assert_eq!(tree.query(..3), 5); // min(100,20,5) = 5
    }

    #[test]
    fn test_min_with_different_types() {
        // Test with i64
        let values_i64 = vec![1000000000_i64, 500000000, 2000000000];
        let tree_i64 = SegTreeMin::<i64>::from_vec(&values_i64);
        assert_eq!(tree_i64.query(..), 500000000);

        // Test with u32
        let values_u32 = vec![15u32, 25, 5, 45];
        let tree_u32 = SegTreeMin::<u32>::from_vec(&values_u32);
        assert_eq!(tree_u32.query(..), 5);
        assert_eq!(tree_u32.query(1..3), 5);
    }

    #[test]
    fn test_min_edge_cases() {
        // Single element
        let single = vec![42];
        let tree_single = SegTreeMin::<i32>::from_vec(&single);
        assert_eq!(tree_single.query(..), 42);
        assert_eq!(tree_single.query(..0), i32::MAX);

        // All same values
        let same = vec![7, 7, 7, 7, 7];
        let tree_same = SegTreeMin::<i32>::from_vec(&same);
        assert_eq!(tree_same.query(..), 7);
        assert_eq!(tree_same.query(1..4), 7);
        assert_eq!(tree_same.query(2..3), 7);

        // Empty ranges in larger tree
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let tree = SegTreeMin::<i32>::from_vec(&values);
        assert_eq!(tree.query(3..3), i32::MAX); // Empty range
        assert_eq!(tree.query(..0), i32::MAX); // Empty range at start
        assert_eq!(tree.query(8..), i32::MAX); // Empty range at end
    }

    #[test]
    fn test_min_large_tree() {
        let size: i32 = 1000;
        let values: Vec<i32> = (1..=size).collect();
        let mut tree = SegTreeMin::<i32>::from_vec(&values);

        // Minimum of 1 to 1000 is 1
        assert_eq!(tree.query(..), 1);

        // Minimum of first half (1 to 500) is 1
        assert_eq!(tree.query(..500), 1);

        // Minimum of second half (501 to 1000) is 501
        assert_eq!(tree.query(500..), 501);

        // Test update - change the minimum
        tree.update(0, 2000); // Change 1 to 2000
        assert_eq!(tree.query(..), 2); // New minimum is 2
        assert_eq!(tree.query(..500), 2); // First half minimum is now 2
        assert_eq!(tree.query(..1), 2000); // Just the updated element
    }

    #[test]
    fn test_min_negative_values() {
        let values = vec![-5, -3, -1, 2, 4];
        let mut tree = SegTreeMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), -5); // min(-5, -3, -1, 2, 4) = -5
        assert_eq!(tree.query(..3), -5); // min(-5, -3, -1) = -5
        assert_eq!(tree.query(3..5), 2); // min(2, 4) = 2
        assert_eq!(tree.query(1..4), -3); // min(-3, -1, 2) = -3

        tree.update(0, -10); // Change -5 to -10
        assert_eq!(tree.query(..), -10); // New minimum is -10
        assert_eq!(tree.query(1..), -3); // Excluding first element
    }

    #[test]
    fn test_min_new_empty_tree() {
        let mut tree = SegTreeMin::<i32>::new(5);

        // All elements should be MAX initially
        assert_eq!(tree.query(..), i32::MAX);
        assert_eq!(tree.query(2..4), i32::MAX);

        // Update some elements
        tree.update(1, 10);
        tree.update(3, 20);

        assert_eq!(tree.query(..), 10); // min(MAX, 10, MAX, 20, MAX) = 10
        assert_eq!(tree.query(1..4), 10); // min(10, MAX, 20) = 10
        assert_eq!(tree.query(3..5), 20); // min(20, MAX) = 20
        assert_eq!(tree.query(..1), i32::MAX); // Still MAX
    }

    #[test]
    fn test_min_extremes() {
        let values = vec![i32::MIN, i32::MAX, 0, -1, 1];
        let mut tree = SegTreeMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), i32::MIN); // MIN is smallest
        assert_eq!(tree.query(1..), -1); // Excluding MIN: min(MAX, 0, -1, 1) = -1
        assert_eq!(tree.query(1..2), i32::MAX); // Just MAX

        tree.update(0, 0); // Change MIN to 0
        assert_eq!(tree.query(..), -1); // min(0, MAX, 0, -1, 1) = -1
    }
}
