//! A generic, reusable Segment Tree implementation.
//!
//! This module provides a `SegTree` data structure, which is useful for efficient
//! range queries on a sequence of elements. A segment tree can answer queries
//! for any associative operation (like summation, minimum, maximum) on a range
//! in `O(log n)` time. Point updates are also supported in `O(log n)` time.
//!
//! ## Design
//!
//! The implementation follows a common Rust design pattern that separates the generic
//! data structure logic from the specific user-defined operation.
//!
//! - [`SegTree<Spec>`]: The generic segment tree struct. It handles the tree structure,
//!   indexing, and the query/update algorithms.
//! - [`SegTreeSpec`]: A trait that you implement to define the behavior of the
//!   segment tree. It specifies the element type and the associative binary operation
//!   (a "monoid").
//!
//! ## Example
//!
//! Here is how to create a segment tree for range sum queries.
//!
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! // 1. Define a struct to represent your operation.
//! struct SumSpec;
//!
//! // 2. Implement the `SegTreeSpec` trait for it.
//! impl SegTreeSpec for SumSpec {
//!     // The type of the elements in the tree.
//!     type T = i64;
//!     // The identity element for the operation (0 for addition).
//!     const ID: Self::T = 0;
//!
//!     // The associative binary operation.
//!     fn op(a: &Self::T, b: &Self::T) -> Self::T {
//!         a + b
//!     }
//! }
//!
//! // 3. Create the segment tree with your spec.
//! let mut values = vec![1, 2, 3, 4, 5];
//! let mut seg_tree = SegTree::<SumSpec>::from_vec(&values);
//!
//! // Query the sum of the range [2, 5) -> sum of elements at index 2, 3, and 4.
//! assert_eq!(seg_tree.query(2, 5), 3 + 4 + 5);
//! assert_eq!(seg_tree.query(0, 5), 15);
//!
//! // 4. Update a value and see the query result change.
//! seg_tree.update(3, 10); // Set the element at index 3 to 10.
//! assert_eq!(seg_tree.query(0, 5), 1 + 2 + 3 + 10 + 5);
//! ```

use std::marker::PhantomData;

/// Defines the monoid operation and element type for a `SegTree`.
///
/// A "monoid" is a set of elements with an identity element and an associative
/// binary operation. This trait encapsulates that definition, allowing `SegTree`
/// to be generic over any valid monoid.
pub trait SegTreeSpec {
    /// The type of the elements stored and operated on in the segment tree.
    type T: Clone;

    /// The identity element for the monoid operation `op`.
    ///
    /// For any element `a`, `op(a, ID)` must be equal to `a`.
    /// Examples: 0 for addition, 1 for multiplication, `infinity` for minimum.
    const ID: Self::T;

    /// The associative binary operation of the monoid.
    ///
    /// This operation must be associative: `op(a, op(b, c))` must be equal
    /// to `op(op(a, b), c)`.
    fn op(a: &Self::T, b: &Self::T) -> Self::T;
}

/// A generic Segment Tree data structure.
///
/// See [SegTree] for a detailed explanation and examples.
pub struct SegTree<Spec: SegTreeSpec> {
    /// The user-provided size of the array.
    size: usize,
    /// The number of leaf nodes in the tree (a power of 2 >= size).
    max_size: usize,
    /// The tree data, stored as a flat vector.
    data: Vec<Spec::T>,
    /// Zero-sized marker to associate the `Spec` type with the struct.
    _spec: PhantomData<Spec>,
}

impl<Spec: SegTreeSpec> SegTree<Spec> {
    /// Creates a new `SegTree` of a given `size`, initialized with identity elements.
    ///
    /// The internal size of the tree will be the smallest power of two
    /// greater than or equal to `size`.
    ///
    /// Time complexity: `O(n)` where n is the smallest power of two >= `size`.
    pub fn new(size: usize) -> Self {
        let max_size = size.next_power_of_two();
        Self {
            size,
            max_size,
            data: vec![Spec::ID; max_size * 2],
            _spec: PhantomData,
        }
    }

    /// Creates a new `SegTree` from a vector of initial values.
    ///
    /// The tree is built in `O(n)` time, which is more efficient than creating an
    /// empty tree and updating each element individually.
    ///
    /// Time complexity: `O(n)` where n is the length of `values`.
    pub fn from_vec(values: &[Spec::T]) -> Self {
        let size = values.len();
        let max_size = size.next_power_of_two();
        let mut data = vec![Spec::ID; 2 * max_size];

        // Copy initial values to the leaf nodes.
        data[max_size..(max_size + size)].clone_from_slice(values);

        // Build the tree by combining children up to the root.
        for i in (1..max_size).rev() {
            data[i] = Spec::op(&data[i * 2], &data[i * 2 + 1]);
        }

        Self {
            size,
            max_size,
            data,
            _spec: PhantomData,
        }
    }

    /// Performs a point update, replacing the value at `index` with a new `value`.
    ///
    /// After updating the leaf node, the change is propagated up the tree by
    /// recalculating the parent nodes.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds (`index >= self.size`).
    pub fn update(&mut self, index: usize, value: Spec::T) {
        assert!(index < self.size, "update index out of bounds");
        let leaf_index = index + self.max_size;
        self.data[leaf_index] = value;
        self.recompute(leaf_index);
    }

    /// Queries the segment tree for the aggregated value in the range `[left, right)`.
    ///
    /// The range is half-open, including `left` and excluding `right`.
    ///
    /// Time complexity: `O(log n)`.
    ///
    /// # Panics
    ///
    /// This function panics if `left > right` or if `right` is out of the bounds
    /// of the original array size (`right > self.size`).
    pub fn query(&self, left: usize, right: usize) -> Spec::T {
        assert!(
            left <= right,
            "query range start cannot be greater than end"
        );
        assert!(right <= self.size, "query range end is out of bounds");

        if left == right {
            return Spec::ID;
        }

        // Map the logical range to the internal array indices.
        let mut left = left + self.max_size;
        let mut right = right + self.max_size;

        // Initialize accumulators for the left and right sides of the range.
        let mut result_left = Spec::ID;
        let mut result_right = Spec::ID;

        while left < right {
            if left & 1 == 1 {
                result_left = Spec::op(&result_left, &self.data[left]);
                left += 1;
            }
            if right % 2 == 1 {
                right -= 1;
                result_right = Spec::op(&self.data[right], &result_right);
            }
            left /= 2;
            right /= 2;
        }

        Spec::op(&result_left, &result_right)
    }

    /// Private helper to recompute parent nodes from a leaf up to the root.
    fn recompute(&mut self, mut index: usize) {
        // `index` is the leaf index in the `data` vector.
        // We move up to the parent and recompute its value.
        while index > 1 {
            index /= 2;
            self.data[index] = Spec::op(&self.data[index * 2], &self.data[index * 2 + 1]);
        }
    }
}

// Unit tests are placed in a submodule and only compiled when running `cargo test`.
#[cfg(test)]
mod tests {
    use super::*;

    // Define a spec for summation for testing purposes.
    struct SumSpec;
    impl SegTreeSpec for SumSpec {
        type T = i64;
        const ID: Self::T = 0;
        fn op(a: &Self::T, b: &Self::T) -> Self::T {
            a + b
        }
    }

    // Define a spec for maximum for testing purposes.
    struct MaxSpec;
    impl SegTreeSpec for MaxSpec {
        type T = i32;
        const ID: Self::T = i32::MIN;
        fn op(a: &Self::T, b: &Self::T) -> Self::T {
            *a.max(b)
        }
    }

    #[test]
    fn test_new_empty() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        assert_eq!(seg_tree.query(0, 10), 0);
    }

    #[test]
    fn test_from_vec_and_query_all() {
        let values = vec![1, 2, 3, 4, 5];
        let seg_tree = SegTree::<SumSpec>::from_vec(&values);
        assert_eq!(seg_tree.query(0, 5), 15);
    }

    #[test]
    fn test_query_sub_ranges() {
        let values = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let seg_tree = SegTree::<SumSpec>::from_vec(&values);
        assert_eq!(seg_tree.query(0, 3), 6); // 1+2+3
        assert_eq!(seg_tree.query(2, 5), 12); // 3+4+5
        assert_eq!(seg_tree.query(4, 8), 26); // 5+6+7+8
        assert_eq!(seg_tree.query(7, 8), 8); // just 8
    }

    #[test]
    fn test_query_empty_range() {
        let values = vec![1, 2, 3];
        let seg_tree = SegTree::<SumSpec>::from_vec(&values);
        assert_eq!(seg_tree.query(1, 1), 0);
        assert_eq!(seg_tree.query(3, 3), 0);
    }

    #[test]
    fn test_update() {
        let values = vec![1, 2, 3, 4, 5];
        let mut seg_tree = SegTree::<SumSpec>::from_vec(&values);

        assert_eq!(seg_tree.query(0, 5), 15);

        // Update index 2 (value 3) to 10
        seg_tree.update(2, 10);
        assert_eq!(seg_tree.query(0, 5), 1 + 2 + 10 + 4 + 5);
        assert_eq!(seg_tree.query(2, 3), 10);
        assert_eq!(seg_tree.query(0, 2), 3);
    }

    #[test]
    fn test_max_spec() {
        let values = vec![1, 10, 3, 8, 5];
        let mut seg_tree = SegTree::<MaxSpec>::from_vec(&values);

        assert_eq!(seg_tree.query(0, 5), 10);
        assert_eq!(seg_tree.query(2, 4), 8); // max(3, 8)

        seg_tree.update(1, 2); // update 10 to 2
        assert_eq!(seg_tree.query(0, 5), 8); // max(1, 2, 3, 8, 5)
    }

    #[test]
    #[should_panic]
    fn test_panic_query_invalid_range() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.query(5, 4);
    }

    #[test]
    #[should_panic]
    fn test_panic_query_out_of_bounds() {
        let seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.query(0, 11);
    }

    #[test]
    #[should_panic]
    fn test_panic_update_out_of_bounds() {
        let mut seg_tree = SegTree::<SumSpec>::new(10);
        seg_tree.update(10, 5);
    }
}
