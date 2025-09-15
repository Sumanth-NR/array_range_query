//! Segment tree for sum operations.
//!
//! Provides `SegTreeSum<T>` for efficient range sum queries.

use crate::{SegTree, SegTreeSpec};
use num_traits::ConstZero;
use std::marker::PhantomData;
use std::ops::AddAssign;

/// Specification for sum operations.
pub struct SegTreeSumSpec<T>(PhantomData<T>);

impl<T> SegTreeSpec for SegTreeSumSpec<T>
where
    T: Clone + ConstZero + AddAssign<T>,
{
    type T = T;
    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op(a: &mut Self::T, b: &Self::T) {
        *a += b.clone();
    }
}

/// Segment tree specialized for sum operations.
pub type SegTreeSum<T> = SegTree<SegTreeSumSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_basic_operations() {
        let values = vec![1, 2, 3, 4, 5];
        let tree = SegTreeSum::<i32>::from_slice(&values);

        // Test initial queries
        assert_eq!(tree.query(..), 15); // 1+2+3+4+5
        assert_eq!(tree.query(1..4), 9); // 2+3+4
        assert_eq!(tree.query(..1), 1); // single element
        assert_eq!(tree.query(4..5), 5); // last element
        assert_eq!(tree.query(2..2), 0); // empty range
    }

    #[test]
    fn test_sum_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = SegTreeSum::<i32>::from_slice(&values);

        assert_eq!(tree.query(..), 150);

        // Update middle element
        tree.update(2, 100);
        assert_eq!(tree.query(..), 220); // 10+20+100+40+50
        assert_eq!(tree.query(2..3), 100); // just the updated element
        assert_eq!(tree.query(1..4), 160); // 20+100+40

        // Update first element
        tree.update(0, 5);
        assert_eq!(tree.query(..), 215); // 5+20+100+40+50
        assert_eq!(tree.query(..2), 25); // 5+20
    }

    #[test]
    fn test_sum_with_different_types() {
        // Test with i64
        let values_i64 = vec![1000000000_i64, 2000000000, 3000000000];
        let tree_i64 = SegTreeSum::<i64>::from_slice(&values_i64);
        assert_eq!(tree_i64.query(..), 6000000000);

        // Test with f64 (approximate comparison)
        let values_f64 = vec![1.5, 2.5, 3.5, 4.5];
        let tree_f64 = SegTreeSum::<f64>::from_slice(&values_f64);
        assert!((tree_f64.query(..) - 12.0).abs() < 1e-10);
        assert!((tree_f64.query(1..3) - 6.0).abs() < 1e-10);
    }

    #[test]
    fn test_sum_edge_cases() {
        // Single element
        let single = vec![42];
        let tree_single = SegTreeSum::<i32>::from_slice(&single);
        assert_eq!(tree_single.query(..), 42);
        assert_eq!(tree_single.query(..0), 0);

        // Empty ranges in larger tree
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let tree = SegTreeSum::<i32>::from_slice(&values);
        assert_eq!(tree.query(3..3), 0); // empty range
        assert_eq!(tree.query(..0), 0); // empty at start
        assert_eq!(tree.query(8..), 0); // empty at end
    }

    #[test]
    fn test_sum_large_tree() {
        let size: i32 = 1000;
        let values: Vec<i32> = (1..=size).collect();
        let mut tree = SegTreeSum::<i32>::from_slice(&values);

        // Sum of 1..=1000 = 1000 * 1001 / 2 = 500500
        assert_eq!(tree.query(..), 500500);

        // Sum of first half (1..500)
        assert_eq!(tree.query(..500), 125250);

        // Sum of second half
        assert_eq!(tree.query(500..), 375250);

        // Test update
        tree.update(499, 0); // change 500 to 0
        assert_eq!(tree.query(..), 500000); // 500500 - 500
        assert_eq!(tree.query(..500), 124750); // 125250 - 500
    }

    #[test]
    fn test_sum_zero_values() {
        let values = vec![0, 0, 0, 0, 0];
        let mut tree = SegTreeSum::<i32>::from_slice(&values);

        assert_eq!(tree.query(..), 0);
        assert_eq!(tree.query(1..4), 0);

        // Update with non-zero
        tree.update(2, 10);
        assert_eq!(tree.query(..), 10);
        assert_eq!(tree.query(2..3), 10);
        assert_eq!(tree.query(..2), 0);
        assert_eq!(tree.query(3..5), 0);
    }

    #[test]
    fn test_sum_negative_values() {
        let values = vec![-5, -3, -1, 2, 4];
        let mut tree = SegTreeSum::<i32>::from_slice(&values);

        assert_eq!(tree.query(..), -3); // -5 + -3 + -1 + 2 + 4 = -3
        assert_eq!(tree.query(..3), -9); // -5 + -3 + -1 = -9
        assert_eq!(tree.query(3..5), 6); // 2 + 4 = 6

        tree.update(0, 10); // change -5 to 10
        assert_eq!(tree.query(..), 12); // 10 + -3 + -1 + 2 + 4 = 12
    }

    #[test]
    fn test_sum_new_empty_tree() {
        let mut tree = SegTreeSum::<i32>::new(5);

        // All elements should be zero initially
        assert_eq!(tree.query(..), 0);
        assert_eq!(tree.query(2..4), 0);

        // Update some elements
        tree.update(1, 10);
        tree.update(3, 20);

        assert_eq!(tree.query(..), 30); // 0 + 10 + 0 + 20 + 0
        assert_eq!(tree.query(1..4), 30);
        assert_eq!(tree.query(..2), 10);
    }
}
