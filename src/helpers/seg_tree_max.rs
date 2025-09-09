//! Segment tree specialization for maximum operations.
//!
//! This module provides a convenient wrapper around the generic `SegTree`
//! for maximum queries with automatic minimum identity element.

use crate::{SegTree, SegTreeSpec};
use min_max_traits::Min as ConstLowerBound;
use std::marker::PhantomData;

/// Specification for segment trees that perform maximum operations.
///
/// This spec works with any type `T` that implements ordering and has
/// a minimum constant. The identity element is automatically set to the
/// minimum value of the type.
pub struct SegTreeMaxSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeMaxSpec<T>
where
    T: Clone + ConstLowerBound + Ord,
{
    type T = T;
    const ID: Self::T = <T as ConstLowerBound>::MIN;

    fn op(a: &Self::T, b: &Self::T) -> Self::T {
        a.clone().max(b.clone())
    }
}

/// Convenience alias: a `SegTree` specialized to perform maximum queries over `T`.
///
/// # Examples
///
/// ```rust
/// use array_range_query::SegTreeMax;
///
/// let values = vec![5, 2, 8, 1, 9, 3];
/// let mut tree = SegTreeMax::<i32>::from_vec(&values);
///
/// assert_eq!(tree.query(..), 9); // Maximum of all elements
/// assert_eq!(tree.query(1..4), 8); // Maximum of elements 2, 8, 1
///
/// tree.update(3, 15); // Change element at index 3 to 15
/// assert_eq!(tree.query(..), 15); // Updated maximum
/// ```
pub type SegTreeMax<T> = SegTree<SegTreeMaxSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_basic_operations() {
        let values = vec![5, 2, 8, 1, 9, 3];
        let tree = SegTreeMax::<i32>::from_vec(&values);

        // Test initial queries
        assert_eq!(tree.query(..), 9); // Max of all: max(5,2,8,1,9,3) = 9
        assert_eq!(tree.query(1..4), 8); // Max of middle: max(2,8,1) = 8
        assert_eq!(tree.query(..1), 5); // Single element
        assert_eq!(tree.query(4..6), 9); // max(9, 3) = 9
        assert_eq!(tree.query(2..2), i32::MIN); // Empty range returns MIN
    }

    #[test]
    fn test_max_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = SegTreeMax::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 50);

        // Update last element to smaller value
        tree.update(4, 5);
        assert_eq!(tree.query(..), 40); // max(10,20,30,40,5) = 40
        assert_eq!(tree.query(4..5), 5); // Just the updated element
        assert_eq!(tree.query(..4), 40); // max(10,20,30,40) = 40

        // Update middle element to largest value
        tree.update(2, 100);
        assert_eq!(tree.query(..), 100); // max(10,20,100,40,5) = 100
        assert_eq!(tree.query(2..3), 100); // Just the updated element
        assert_eq!(tree.query(..3), 100); // max(10,20,100) = 100
    }

    #[test]
    fn test_max_with_different_types() {
        // Test with i64
        let values_i64 = vec![1000000000_i64, 2000000000, 500000000];
        let tree_i64 = SegTreeMax::<i64>::from_vec(&values_i64);
        assert_eq!(tree_i64.query(..), 2000000000);

        // Test with u32
        let values_u32 = vec![15u32, 25, 45, 5];
        let tree_u32 = SegTreeMax::<u32>::from_vec(&values_u32);
        assert_eq!(tree_u32.query(..), 45);
        assert_eq!(tree_u32.query(1..3), 45);
    }

    #[test]
    fn test_max_edge_cases() {
        // Single element
        let single = vec![42];
        let tree_single = SegTreeMax::<i32>::from_vec(&single);
        assert_eq!(tree_single.query(..), 42);
        assert_eq!(tree_single.query(..0), i32::MIN);

        // All same values
        let same = vec![7, 7, 7, 7, 7];
        let tree_same = SegTreeMax::<i32>::from_vec(&same);
        assert_eq!(tree_same.query(..), 7);
        assert_eq!(tree_same.query(1..4), 7);
        assert_eq!(tree_same.query(2..3), 7);

        // Empty ranges in larger tree
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let tree = SegTreeMax::<i32>::from_vec(&values);
        assert_eq!(tree.query(3..3), i32::MIN); // Empty range
        assert_eq!(tree.query(..0), i32::MIN); // Empty range at start
        assert_eq!(tree.query(8..), i32::MIN); // Empty range at end
    }

    #[test]
    fn test_max_large_tree() {
        let size: i32 = 1000;
        let values: Vec<i32> = (1..=size).collect();
        let mut tree = SegTreeMax::<i32>::from_vec(&values);

        // Maximum of 1 to 1000 is 1000
        assert_eq!(tree.query(..), 1000);

        // Maximum of first half (1 to 500) is 500
        assert_eq!(tree.query(..500), 500);

        // Maximum of second half (501 to 1000) is 1000
        assert_eq!(tree.query(500..), 1000);

        // Test update - change the maximum
        tree.update(999, 0); // Change 1000 to 0
        assert_eq!(tree.query(..), 999); // New maximum is 999
        assert_eq!(tree.query(500..), 999); // Second half maximum is now 999
        assert_eq!(tree.query(999..1000), 0); // Just the updated element
    }

    #[test]
    fn test_max_negative_values() {
        let values = vec![-5, -3, -1, 2, 4];
        let mut tree = SegTreeMax::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 4); // max(-5, -3, -1, 2, 4) = 4
        assert_eq!(tree.query(..3), -1); // max(-5, -3, -1) = -1
        assert_eq!(tree.query(3..5), 4); // max(2, 4) = 4
        assert_eq!(tree.query(1..4), 2); // max(-3, -1, 2) = 2

        tree.update(4, -10); // Change 4 to -10
        assert_eq!(tree.query(..), 2); // New maximum is 2
        assert_eq!(tree.query(..4), 2); // Excluding last element
    }

    #[test]
    fn test_max_new_empty_tree() {
        let mut tree = SegTreeMax::<i32>::new(5);

        // All elements should be MIN initially
        assert_eq!(tree.query(..), i32::MIN);
        assert_eq!(tree.query(2..4), i32::MIN);

        // Update some elements
        tree.update(1, 10);
        tree.update(3, 20);

        assert_eq!(tree.query(..), 20); // max(MIN, 10, MIN, 20, MIN) = 20
        assert_eq!(tree.query(1..4), 20); // max(10, MIN, 20) = 20
        assert_eq!(tree.query(..2), 10); // max(MIN, 10) = 10
        assert_eq!(tree.query(4..5), i32::MIN); // Still MIN
    }

    #[test]
    fn test_max_extremes() {
        let values = vec![i32::MIN, i32::MAX, 0, -1, 1];
        let mut tree = SegTreeMax::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), i32::MAX); // MAX is largest
        assert_eq!(tree.query(..1), i32::MIN); // Just MIN
        assert_eq!(tree.query(2..5), 1); // Excluding MIN and MAX: max(0, -1, 1) = 1

        tree.update(1, 0); // Change MAX to 0
        assert_eq!(tree.query(..), 1); // max(MIN, 0, 0, -1, 1) = 1
    }
}
