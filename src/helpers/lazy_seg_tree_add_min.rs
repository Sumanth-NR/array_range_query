use crate::{LazySegTree, LazySegTreeSpec};
use min_max_traits::Max as ConstUpperBound;
use std::marker::PhantomData;
use std::ops::Add;

/// Specification for lazy segment trees that perform range add updates with minimum queries.
///
/// This spec works with data type `T` where:
/// - `T` supports addition, subtraction, and ordering
/// - `T` has a maximum constant (for min aggregation)
/// - Updates are applied uniformly to all elements in a range
pub struct LazySegTreeAddMinSpec<T>(PhantomData<T>);

impl<T> LazySegTreeSpec for LazySegTreeAddMinSpec<T>
where
    T: Clone + Add<Output = T> + ConstUpperBound + Ord,
{
    type T = T;
    type U = T;

    const ID: Self::T = <T as ConstUpperBound>::MAX;

    fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
        if *d1 > *d2 {
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

/// Convenience alias: a `LazySegTree` specialized for range add updates and min queries.
///
/// # Examples
///
/// ```
/// use array_range_query::LazySegTreeAddMin;
///
/// let values = vec![5, 2, 8, 1, 9, 3];
/// let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);
///
/// assert_eq!(tree.query(..), 1); // Minimum of all elements
///
/// // Add 2 to range [1, 4)
/// tree.update(1..4, 2);
/// assert_eq!(tree.query(..), 3); // min(5, 4, 10, 3, 9, 3)
/// ```
pub type LazySegTreeAddMin<T> = LazySegTree<LazySegTreeAddMinSpec<T>>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_min_basic_operations() {
        let values = vec![5, 2, 8, 1, 9, 3];
        let tree = LazySegTreeAddMin::<i32>::from_vec(&values);

        // Test initial queries
        assert_eq!(tree.query(..), 1); // min(5,2,8,1,9,3) = 1
        assert_eq!(tree.query(1..4), 1); // min(2,8,1) = 1
        assert_eq!(tree.query(..1), 5); // Single element
        assert_eq!(tree.query(4..6), 3); // min(9,3) = 3
        assert_eq!(tree.query(2..2), i32::MAX); // Empty range returns MAX
    }

    #[test]
    fn test_add_min_range_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 10);

        // Add 5 to range [1, 4)
        tree.update(1..4, 5);
        assert_eq!(tree.query(..), 10); // min(10,25,35,45,50) = 10
        assert_eq!(tree.query(1..4), 25); // min(25,35,45) = 25

        // Add -15 to range [0, 3)
        tree.update(..3, -15);
        assert_eq!(tree.query(..), -5); // min(-5,10,20,45,50)
        assert_eq!(tree.query(..3), -5); // min(-5,10,20)
    }

    #[test]
    fn test_add_min_overlapping_updates() {
        let values = vec![1, 1, 1, 1, 1];
        let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 1);

        // Overlapping updates
        tree.update(..3, 2); // [3,3,3,1,1]
        tree.update(2..5, -1); // [3,3,2,0,0]
        tree.update(1..4, 1); // [3,4,3,1,0]

        assert_eq!(tree.query(..1), 3);
        assert_eq!(tree.query(1..2), 4);
        assert_eq!(tree.query(2..3), 3);
        assert_eq!(tree.query(3..4), 1);
        assert_eq!(tree.query(4..5), 0);
        assert_eq!(tree.query(..), 0);
    }

    #[test]
    fn test_add_min_negative_updates() {
        let values = vec![10, 20, 30, 40, 50];
        let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 10);

        // Subtract (negative add) from range
        tree.update(1..4, -5);
        assert_eq!(tree.query(..), 10); // min(10,15,25,35,50) = 10
        assert_eq!(tree.query(1..4), 15); // min(15,25,35) = 15

        // Mix positive and negative updates
        tree.update(..2, 15); // [25,30,25,35,50]
        assert_eq!(tree.query(..2), 25); // min(25,30)
        assert_eq!(tree.query(..), 25); // min(25,30,25,35,50)
    }

    #[test]
    fn test_add_min_edge_cases() {
        // Single element
        let single = vec![42];
        let mut tree_single = LazySegTreeAddMin::<i32>::from_vec(&single);
        assert_eq!(tree_single.query(..), 42);
        tree_single.update(..1, 8);
        assert_eq!(tree_single.query(..), 50);

        // Empty updates (no-op)
        let values = vec![1, 2, 3, 4, 5];
        let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);
        let original_min = tree.query(..);
        tree.update(2..2, 100); // Empty range update
        assert_eq!(tree.query(..), original_min); // Should be unchanged
    }

    #[test]
    fn test_add_min_large_tree() {
        let size = 1000;
        let values = vec![1i32; size]; // All ones
        let mut tree = LazySegTreeAddMin::<i32>::from_vec(&values);

        assert_eq!(tree.query(..), 1);

        // Add 1 to first half
        tree.update(..size / 2, 1);
        assert_eq!(tree.query(..), 1); // Second half is still 1

        // Subtract 2 from second half
        tree.update(size / 2.., -2);
        assert_eq!(tree.query(..), -1); // Now min is -1
        assert_eq!(tree.query(size / 2..), -1);
    }

    #[test]
    #[should_panic(expected = "overflow")]
    fn test_add_min_new_empty_tree_should_panic() {
        let mut tree = LazySegTreeAddMin::<i32>::new(5);

        // All elements should be MAX initially
        assert_eq!(tree.query(..), i32::MAX);

        // Add 10 to [1, 4)
        // This step should panic
        tree.update(1..4, 10);
    }
}
