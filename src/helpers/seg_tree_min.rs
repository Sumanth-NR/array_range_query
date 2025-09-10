//! Segment tree specialization for minimum operations.
//!
//! This module provides a convenient wrapper around the generic `SegTree`
//! for minimum queries with an automatically chosen identity element.
//!
//! Examples
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! // Define the min monoid
//! struct MinSpec;
//! impl SegTreeSpec for MinSpec {
//!     type T = i32;
//!     const ID: Self::T = i32::MAX;
//!     fn op(a: &mut Self::T, b: &Self::T) { if *a > *b { *a = *b; } }
//! }
//!
//! // Example A: consume a Vec (cheap move)
//! let values_owned = vec![5, 2, 8, 1, 9, 3];
//! let mut tree_owned = SegTree::<MinSpec>::from_vec(values_owned);
//! assert_eq!(tree_owned.query(..), 1);
//!
//! // Example B: build from a slice (clones elements)
//! let values = vec![5, 2, 8, 1, 9, 3];
//! let tree_from_slice = SegTree::<MinSpec>::from_slice(&values);
//! assert_eq!(tree_from_slice.query(1..4), 1);
//! ```

use crate::{SegTree, SegTreeSpec};
use min_max_traits::Max as ConstUpperBound;
use std::marker::PhantomData;

/// Specification for segment trees that perform minimum operations.
///
/// This spec works with any type `T` that implements ordering and provides a
/// constant maximum via the `min_max_traits::Max` trait. The identity element
/// is set to the maximum constant of the type.
pub struct SegTreeMinSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeMinSpec<T>
where
    T: Clone + ConstUpperBound + Ord,
{
    type T = T;
    const ID: Self::T = <T as ConstUpperBound>::MAX;

    fn op(a: &mut Self::T, b: &Self::T) {
        if *a > *b {
            *a = b.clone();
        }
    }
}

/// Convenience alias: a `SegTree` specialized to perform minimum queries over `T`.
///
/// Usage notes:
/// - Prefer `SegTreeMin::<T>::from_vec(vec)` when you can give ownership of the
///   `Vec<T>` to the tree — this avoids unnecessary cloning.
/// - Use `SegTreeMin::<T>::from_slice(&slice)` when you only have a borrowed
///   slice and are OK with cloning each element.
pub type SegTreeMin<T> = SegTree<SegTreeMinSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_basic_operations() {
        let values = vec![5, 2, 8, 1, 9, 3];
        let tree = SegTreeMin::<i32>::from_slice(&values);

        // Test initial queries
        assert_eq!(tree.query(..), 1); // min(5,2,8,1,9,3) = 1
        assert_eq!(tree.query(1..4), 1); // min(2,8,1) = 1
        assert_eq!(tree.query(..1), 5); // Single element
        assert_eq!(tree.query(4..6), 3); // min(9, 3) = 3
        assert_eq!(tree.query(2..2), i32::MAX); // Empty range returns ID (MAX)
    }

    #[test]
    fn test_min_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = SegTreeMin::<i32>::from_slice(&values);

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
        let tree_i64 = SegTreeMin::<i64>::from_slice(&values_i64);
        assert_eq!(tree_i64.query(..), 500000000);

        // Test with u32
        let values_u32 = vec![15u32, 25, 5, 45];
        let tree_u32 = SegTreeMin::<u32>::from_slice(&values_u32);
        assert_eq!(tree_u32.query(..), 5);
        assert_eq!(tree_u32.query(1..3), 5);
    }

    #[test]
    fn test_min_edge_cases() {
        // Single element
        let single = vec![42];
        let tree_single = SegTreeMin::<i32>::from_slice(&single);
        assert_eq!(tree_single.query(..), 42);
        assert_eq!(tree_single.query(..0), i32::MAX);

        // All same values
        let same = vec![7, 7, 7, 7, 7];
        let tree_same = SegTreeMin::<i32>::from_slice(&same);
        assert_eq!(tree_same.query(..), 7);
        assert_eq!(tree_same.query(1..4), 7);
        assert_eq!(tree_same.query(2..3), 7);

        // Empty ranges in larger tree
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let tree = SegTreeMin::<i32>::from_slice(&values);
        assert_eq!(tree.query(3..3), i32::MAX); // Empty range
        assert_eq!(tree.query(..0), i32::MAX); // Empty range at start
        assert_eq!(tree.query(8..), i32::MAX); // Empty range at end
    }

    #[test]
    fn test_min_large_tree() {
        let size: i32 = 1000;
        let values: Vec<i32> = (1..=size).collect();
        let mut tree = SegTreeMin::<i32>::from_slice(&values);

        // Minimum of 1..=1000 is 1
        assert_eq!(tree.query(..), 1);

        // Minimum of the first half is 1
        assert_eq!(tree.query(..500), 1);

        // Minimum of second half (501..=1000) is 501
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
        let mut tree = SegTreeMin::<i32>::from_slice(&values);

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
        assert_eq!(tree.query(..1), i32::MAX); // Still MAX when excluding updated indices
    }

    #[test]
    fn test_min_extremes() {
        let values = vec![i32::MIN, i32::MAX, 0, -1, 1];
        let mut tree = SegTreeMin::<i32>::from_slice(&values);

        assert_eq!(tree.query(..), i32::MIN); // MIN is smallest
        assert_eq!(tree.query(1..), -1); // Excluding MIN: min(MAX, 0, -1, 1) = -1
        assert_eq!(tree.query(1..2), i32::MAX); // Just MAX

        tree.update(0, 0); // Change MIN to 0
        assert_eq!(tree.query(..), -1); // min(0, MAX, 0, -1, 1) = -1
    }

    #[test]
    fn test_consume_vec_constructor() {
        // Demonstrate using the consuming constructor when we don't need the Vec afterwards
        let values = vec![3, 1, 4, 1, 5];
        let tree = SegTreeMin::<i32>::from_vec(values); // moves `values` into the tree
        assert_eq!(tree.query(..), 1);
    }
}
