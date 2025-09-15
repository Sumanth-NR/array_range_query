//! Segment tree for efficient range queries and point updates.
//!
//! A segment tree supports range queries and point updates in O(log n) time
//! for any associative operation. Define operations by implementing [`SegTreeSpec`].
//!
//! # Example
//!
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! struct SumSpec;
//! impl SegTreeSpec for SumSpec {
//!     type T = i64;
//!     const ID: Self::T = 0;
//!     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
//! }
//!
//! let values = vec![1, 2, 3, 4, 5];
//! let mut tree = SegTree::<SumSpec>::from_slice(&values);
//! assert_eq!(tree.query(2..5), 12); // sum of indices 2, 3, 4
//! tree.update(3, 10);
//! assert_eq!(tree.query(..), 21);
//! ```

use crate::utils;
use core::marker::PhantomData;
use core::ops::RangeBounds;

/// Specification for segment tree operations.
///
/// Defines an associative operation (monoid) with identity element.
/// Must satisfy: `op(a, ID) = a` and `op(a, op(b, c)) = op(op(a, b), c)`.
///
/// # Example
/// ```rust
/// use array_range_query::SegTreeSpec;
///
/// struct SumSpec;
/// impl SegTreeSpec for SumSpec {
///     type T = i32;
///     const ID: Self::T = 0;
///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
/// }
/// ```
pub trait SegTreeSpec {
    /// Element type stored in the segment tree.
    type T: Clone;

    /// Identity element for the operation.
    const ID: Self::T;

    /// Associative binary operation, performed in-place.
    ///
    /// Modifies `a` to store the result of combining `a` with `b`.
    fn op(a: &mut Self::T, b: &Self::T);
}

/// A generic Segment Tree data structure.
///
/// A segment tree is a complete binary tree stored in a flat array that enables
/// efficient range queries and point updates on sequences of elements. The tree
/// supports any associative operation defined by the `SegTreeSpec` trait.
///
/// # Internal Structure
///
/// - Uses 1-based indexing where the root is at index 1
/// - Leaf nodes start at index `max_size` (next power of 2 ≥ `size`)
/// - For any node at index `i`, its children are at `2*i` and `2*i+1`
/// - Total space used is `2 * max_size`
///
/// # Type Parameters
///
/// * `Spec` - A type implementing `SegTreeSpec` that defines the operation and element type
///
/// # Examples
///
/// ```
/// use array_range_query::{SegTree, SegTreeSpec};
///
/// struct MaxSpec;
/// impl SegTreeSpec for MaxSpec {
///     type T = i32;
///     const ID: Self::T = i32::MIN;
///     fn op(a: &mut Self::T, b: &Self::T) { *a = (*a).max(*b); }
/// }
///
/// let values = vec![3, 1, 4, 1, 5, 9, 2];
/// let tree = SegTree::<MaxSpec>::from_vec(values);
/// assert_eq!(tree.query(2..5), 5); // max(4, 1, 5) = 5
/// ```
pub struct SegTree<Spec: SegTreeSpec> {
    /// The logical size of the array (as provided by the user)
    size: usize,
    /// The number of leaf nodes in the internal tree (next power of 2 ≥ size)
    max_size: usize,
    /// Tree data stored as a flat boxed slice using 1-based indexing
    data: Box<[Spec::T]>,
    /// Zero-sized marker to associate the `Spec` type with the struct
    _spec: PhantomData<Spec>,
}

impl<Spec: SegTreeSpec> SegTree<Spec> {
    // ===== CONSTRUCTORS =====

    /// Creates a new segment tree with all elements initialized to `Spec::ID`.
    ///
    /// # Time Complexity
    /// O(n)
    pub fn new(size: usize) -> Self {
        let max_size = size.next_power_of_two();
        Self {
            size,
            max_size,
            data: vec![Spec::ID; max_size * 2].into_boxed_slice(),
            _spec: PhantomData,
        }
    }

    /// Creates a new segment tree from a slice of values.
    ///
    /// # Time Complexity
    /// O(n)
    pub fn from_slice(values: &[Spec::T]) -> Self {
        let size = values.len();
        let max_size = size.next_power_of_two();
        let mut data = vec![Spec::ID; 2 * max_size];

        // Copy initial values to the leaf nodes
        data[max_size..(max_size + size)].clone_from_slice(values);

        // Build the tree by combining children up to the root
        for i in (1..max_size).rev() {
            let mut v = data[i * 2].clone();
            Spec::op(&mut v, &data[i * 2 + 1]);
            data[i] = v;
        }

        Self {
            size,
            max_size,
            data: data.into_boxed_slice(),
            _spec: PhantomData,
        }
    }

    /// Creates a new segment tree from a vector of values.
    ///
    /// # Time Complexity
    /// O(n)
    pub fn from_vec(vec: Vec<Spec::T>) -> Self {
        let size = vec.len();
        let max_size = size.next_power_of_two();
        // Allocate full tree storage (internal nodes + leaves)
        let mut data = vec![Spec::ID; 2 * max_size];

        // Move owned values directly into the leaf slots to avoid cloning
        for (i, v) in vec.into_iter().enumerate() {
            data[max_size + i] = v;
        }

        // Build the tree by combining children up to the root
        for i in (1..max_size).rev() {
            let mut v = data[i * 2].clone();
            Spec::op(&mut v, &data[i * 2 + 1]);
            data[i] = v;
        }

        Self {
            size,
            max_size,
            data: data.into_boxed_slice(),
            _spec: PhantomData,
        }
    }

    // ===== PUBLIC INTERFACE =====

    /// Queries the aggregated value over the given range.
    ///
    /// # Example
    ///
    /// ```
    /// use array_range_query::helpers::SegTreeMax;
    ///
    /// let mut tree = SegTreeMax::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(tree.query(..), 5);
    /// ```
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// Panics if the range is invalid or out of bounds.
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> Spec::T {
        let (left, right) = utils::parse_range(range, self.size);
        utils::validate_range(left, right, self.size);

        if left == right {
            return Spec::ID;
        }

        // Map the logical range to the internal array indices
        let mut left = left + self.max_size;
        let mut right = right + self.max_size;

        // Initialize accumulators for the left and right sides of the range
        let mut result_left = Spec::ID;
        let mut result_right = Spec::ID;

        // Standard segment tree range query algorithm
        while left < right {
            // If left is odd (right child), include it and move to next
            if left & 1 == 1 {
                Spec::op(&mut result_left, &self.data[left]);
                left += 1;
            }
            // If right is odd (right child), include the left sibling and move back
            if right % 2 == 1 {
                right -= 1;
                Spec::op(&mut result_right, &self.data[right]);
            }
            // Move up to parent level
            left /= 2;
            right /= 2;
        }

        // Combine the left and right results
        Spec::op(&mut result_left, &result_right);
        result_left
    }

    /// Updates the value at the given index.
    ///
    /// # Example
    ///
    /// ```
    /// use array_range_query::helpers::SegTreeMax;
    ///
    /// let mut tree = SegTreeMax::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(tree.query(..), 5);
    /// tree.update(2, 6);
    /// assert_eq!(tree.query(..), 6);
    /// ```
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// Panics if `index` is out of bounds.
    pub fn update(&mut self, index: usize, value: Spec::T) {
        assert!(index < self.size, "update index out of bounds");

        let leaf_index = index + self.max_size;
        self.data[leaf_index] = value;
        self.recompute(leaf_index);
    }

    // ===== PRIVATE HELPER METHODS =====

    /// Recomputes parent nodes from a leaf up to the root.
    fn recompute(&mut self, mut index: usize) {
        // Move up the tree level by level
        while index > 1 {
            index /= 2; // Move to parent

            // Recompute parent value from its two children
            let mut v = self.data[index * 2].clone();
            Spec::op(&mut v, &self.data[index * 2 + 1]);
            self.data[index] = v;
        }
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    /// Test specification for sum operations.
    struct SumSpec;
    impl SegTreeSpec for SumSpec {
        type T = i64;
        const ID: Self::T = 0;

        fn op(a: &mut Self::T, b: &Self::T) {
            *a += *b;
        }
    }

    /// Test specification for maximum operations.
    struct MaxSpec;
    impl SegTreeSpec for MaxSpec {
        type T = i32;
        const ID: Self::T = i32::MIN;

        fn op(a: &mut Self::T, b: &Self::T) {
            if *a < *b {
                *a = *b;
            }
        }
    }

    #[test]
    fn test_new_empty() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        assert_eq!(seg_tree.query(..), 0);
    }

    #[test]
    fn test_from_slice_with_query() {
        let values = vec![1, 2, 3];
        let seg_tree = SegTree::<SumSpec>::from_slice(&values);

        // Comprehensively test if querying works correctly for any range
        assert_eq!(seg_tree.query(0..1), 1);
        assert_eq!(seg_tree.query(1..2), 2);
        assert_eq!(seg_tree.query(2..3), 3);
        assert_eq!(seg_tree.query(..2), 3);
        assert_eq!(seg_tree.query(1..), 5);
        assert_eq!(seg_tree.query(..), 6);
    }

    #[test]
    fn test_from_vec_with_query() {
        let values = vec![1, 2, 3];
        let seg_tree = SegTree::<SumSpec>::from_vec(values);

        // Comprehensively test if querying works correctly for any range
        assert_eq!(seg_tree.query(0..1), 1);
        assert_eq!(seg_tree.query(1..2), 2);
        assert_eq!(seg_tree.query(2..3), 3);
        assert_eq!(seg_tree.query(..2), 3);
        assert_eq!(seg_tree.query(1..), 5);
        assert_eq!(seg_tree.query(..), 6);
    }

    #[test]
    fn test_query_sub_ranges() {
        let seg_tree = SegTree::<SumSpec>::from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8]);

        assert_eq!(seg_tree.query(0..3), 6); // 1+2+3
        assert_eq!(seg_tree.query(2..5), 12); // 3+4+5
        assert_eq!(seg_tree.query(4..), 26); // 5+6+7+8
        assert_eq!(seg_tree.query(..=6), 28); // 1+2+3+4+5+6+7
        assert_eq!(seg_tree.query(7..8), 8); // just 8
    }

    #[test]
    fn test_query_empty_range() {
        let seg_tree = SegTree::<SumSpec>::from_vec(vec![1, 2, 3]);

        assert_eq!(seg_tree.query(1..1), 0);
        assert_eq!(seg_tree.query(3..3), 0);
    }

    #[test]
    fn test_update() {
        let mut seg_tree = SegTree::<SumSpec>::from_vec(vec![1, 2, 3, 4, 5]);

        assert_eq!(seg_tree.query(..), 15);

        // Update index 2 (value 3) to 10
        seg_tree.update(2, 10);
        assert_eq!(seg_tree.query(..), 1 + 2 + 10 + 4 + 5);
        assert_eq!(seg_tree.query(2..3), 10);
        assert_eq!(seg_tree.query(..2), 3);
    }

    #[test]
    fn test_large_tree() {
        let mut seg_tree = SegTree::<SumSpec>::from_vec((1..=1000).collect());

        // Sum of 1 to 1000 = 1000 * 1001 / 2 = 500500
        assert_eq!(seg_tree.query(..), 500500);

        // Sum of first 500 numbers = 500 * 501 / 2 = 125250
        assert_eq!(seg_tree.query(..500), 125250);

        // Update index 499 (value 500) to 1000
        seg_tree.update(499, 1000);

        assert_eq!(seg_tree.query(..), 500500 + 500);
        assert_eq!(seg_tree.query(..500), 125250 + 500);
    }

    #[test]
    #[should_panic(expected = "update index out of bounds")]
    fn test_panic_update_out_of_bounds() {
        let mut seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.update(10, 5);
    }

    #[test]
    #[should_panic]
    fn test_panic_query_out_of_bounds() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.query(..11);
    }

    #[test]
    #[should_panic]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_panic_query_invalid_range() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.query(5..4);
    }
}
