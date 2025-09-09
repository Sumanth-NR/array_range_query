use crate::{LazySegTree, LazySegTreeSpec};
use num_traits::{ConstZero, NumCast};
use std::marker::PhantomData;
use std::ops::{Add, Mul};

/// Specification for lazy segment trees that perform range assignment (replace) updates with sum queries.
///
/// This spec works with data type `T` where:
/// - `T` supports addition and has a zero constant (for sum aggregation)
/// - Updates are assignments (replace all values in a range with a given value)
pub struct LazySegTreeReplaceSumSpec<T>(PhantomData<T>);

impl<T> LazySegTreeSpec for LazySegTreeReplaceSumSpec<T>
where
    T: Clone + ConstZero + Add<Output = T> + NumCast + Mul<Output = T>,
{
    type T = T;
    type U = T;

    const ID: Self::T = <T as ConstZero>::ZERO;

    fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
        *d1 = d1.clone() + d2.clone();
    }

    #[allow(unused_variables)]
    fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
        *u1 = u2.clone();
    }

    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
        *d = u.clone() * T::from(size).unwrap_or_else(|| panic!("Failed to convert usize to T"));
    }
}

/// Convenience alias: a `LazySegTree` specialized for range assignment (replace) updates and sum queries.
///
/// # Examples
///
/// ```
/// use array_range_query::LazySegTreeReplaceSum;
///
/// let values = vec![1, 2, 3, 4, 5];
/// let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);
///
/// assert_eq!(tree.query(..), 15); // Sum of all elements
///
/// // Replace range [1, 4) with 10
/// tree.update(1..4, 10);
/// assert_eq!(tree.query(..), 1 + 10 + 10 + 10 + 5);
/// ```
pub type LazySegTreeReplaceSum<T> = LazySegTree<LazySegTreeReplaceSumSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_and_point_queries() {
        let values = vec![1, 2, 3, 4, 5];
        let tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 15);
        assert_eq!(tree.query(1..4), 9);
        assert_eq!(tree.query(..1), 1);
        assert_eq!(tree.query(4..5), 5);
        assert_eq!(tree.query(2..2), 0);
    }

    #[test]
    fn test_range_replace_and_sum() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);

        // Replace [1, 4) with 5
        tree.update(1..4, 5);
        assert_eq!(tree.query(..), 10 + 5 + 5 + 5 + 50);
        assert_eq!(tree.query(1..4), 15);

        // Replace [0, 3) with 7
        tree.update(..3, 7);
        assert_eq!(tree.query(..3), 7 + 7 + 7);
        assert_eq!(tree.query(..), 7 + 7 + 7 + 5 + 50);

        // Replace [2, 5) with 1
        tree.update(2..5, 1);
        assert_eq!(tree.query(..), 7 + 7 + 1 + 1 + 1);
        assert_eq!(tree.query(2..5), 1 + 1 + 1);
    }

    #[test]
    fn test_overlapping_and_nested_replaces() {
        let values = vec![1, 2, 3, 4, 5];
        let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);

        // Replace [0, 3) with 2
        tree.update(..3, 2);
        assert_eq!(tree.query(..), 2 + 2 + 2 + 4 + 5);

        // Replace [2, 5) with 7
        tree.update(2..5, 7);
        assert_eq!(tree.query(..), 2 + 2 + 7 + 7 + 7);

        // Replace [1, 4) with 1
        tree.update(1..4, 1);
        assert_eq!(tree.query(..), 2 + 1 + 1 + 1 + 7);

        // Replace [0, 5) with 9
        tree.update(..5, 9);
        assert_eq!(tree.query(..), 45);
    }

    #[test]
    fn test_single_element_and_empty_range() {
        let single = vec![42];
        let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&single);
        assert_eq!(tree.query(..), 42);
        tree.update(..1, 8);
        assert_eq!(tree.query(..), 8);

        // Empty range update should do nothing
        let values = vec![1, 2, 3, 4, 5];
        let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);
        let original_sum = tree.query(..);
        tree.update(2..2, 100);
        assert_eq!(tree.query(..), original_sum);
    }

    #[test]
    fn test_large_tree_and_full_replace() {
        let size = 1000;
        let values = (1..=size as i32).collect::<Vec<_>>();
        let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(&values);

        // Replace first half with 10
        tree.update(..size / 2, 10);
        assert_eq!(tree.query(..size / 2), (size as i32 / 2) * 10);

        // Replace second half with 20
        tree.update(size / 2.., 20);
        assert_eq!(tree.query(size / 2..), (size as i32 / 2) * 20);

        // Replace all with 5
        tree.update(..size, 5);
        assert_eq!(tree.query(..), size as i32 * 5);
    }

    #[test]
    fn test_new_empty_tree_and_partial_replace() {
        let mut tree = LazySegTreeReplaceSum::<i32>::new(5);

        // All elements should be zero initially
        assert_eq!(tree.query(..), 0);

        // Replace [1, 4) with 10
        tree.update(1..4, 10);
        assert_eq!(tree.query(..), 30);
        assert_eq!(tree.query(1..4), 30);

        // Replace all with 2
        tree.update(..5, 2);
        assert_eq!(tree.query(..), 10);
    }

    #[test]
    fn test_noop_update_none() {
        let tree = LazySegTreeReplaceSum::<i32>::from_vec(&[1, 2, 3, 4, 5]);
        let original = tree.query(..);
        // No-op update is not possible with non-Option update type, so just check original value
        assert_eq!(tree.query(..), original);
    }
}
