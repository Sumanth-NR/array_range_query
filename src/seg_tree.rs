//! A generic, reusable Segment Tree implementation.
//!
//! This module provides a `SegTree` data structure, which is useful for efficient
//! range queries on a sequence of elements. A segment tree can answer queries
//! for any associative operation (like summation, minimum, maximum) on a range
//! in `O(log n)` time. Point updates are also supported in `O(log n)` time.
//!
//! ## Overview
//!
//! A segment tree is a binary tree data structure that allows efficient range queries
//! and point updates on an array. Each leaf represents an element of the array, and
//! each internal node represents the result of applying an associative operation
//! to its children.
//!
//! ## Design
//!
//! The implementation follows a common Rust design pattern that separates the generic
//! data structure logic from the specific user-defined operation:
//!
//! - [`SegTree<Spec>`]: The generic segment tree struct. It handles the tree structure,
//!   indexing, and the query/update algorithms.
//! - [`SegTreeSpec`]: A trait that you implement to define the behavior of the
//!   segment tree. It specifies the element type and the associative binary operation
//!   (a "monoid").
//!
//! ## Key Properties
//!
//! - **Time Complexity**: O(log n) for both queries and updates
//! - **Space Complexity**: O(n)
//! - **Generic**: Works with any associative operation
//! - **Memory Efficient**: Uses a flat array representation
//!
//! ## Example
//!
//! Here is how to create a segment tree for range sum queries:
//!
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! // 1. Define a struct to represent your operation
//! struct SumSpec;
//!
//! // 2. Implement the `SegTreeSpec` trait for it
//! impl SegTreeSpec for SumSpec {
//!     // The type of the elements in the tree
//!     type T = i64;
//!
//!     // The identity element for the operation (0 for addition)
//!     const ID: Self::T = 0;
//!
//!     // The associative binary operation, performed in-place
//!     fn op(a: &mut Self::T, b: &Self::T) {
//!         *a += *b;
//!     }
//! }
//!
//! // 3. Create the segment tree with your spec
//! let values = vec![1, 2, 3, 4, 5];
//! let mut seg_tree = SegTree::<SumSpec>::from_slice(&values);
//!
//! // Query the sum of the range [2, 5) -> sum of elements at indices 2, 3, and 4
//! assert_eq!(seg_tree.query(2..5), 12);
//! assert_eq!(seg_tree.query(..), 15);
//!
//! // 4. Update a value and see the query result change
//! seg_tree.update(3, 10); // Set the element at index 3 to 10
//! assert_eq!(seg_tree.query(..), 21);
//! ```

use crate::utils;
use core::marker::PhantomData;
use core::ops::RangeBounds;

/// Defines the monoid operation and element type for a `SegTree`.
///
/// A "monoid" in abstract algebra is a set equipped with an associative binary operation
/// and an identity element. This trait encapsulates that mathematical structure,
/// allowing `SegTree` to be generic over any valid monoid.
///
/// # Requirements
///
/// The implementing type must satisfy the monoid laws:
/// - **Identity**: For any element `a`, `op(a, ID) = a` and `op(ID, a) = a`
/// - **Associativity**: `op(a, op(b, c)) = op(op(a, b), c)`
///
/// # Examples
///
/// ## Sum Monoid
/// ```
/// use array_range_query::SegTreeSpec;
///
/// struct SumSpec;
/// impl SegTreeSpec for SumSpec {
///     type T = i32;
///     const ID: Self::T = 0;  // 0 is the identity for addition
///
///     fn op(a: &mut Self::T, b: &Self::T) {
///         *a += *b;
///     }
/// }
/// ```
///
/// ## Min Monoid
/// ```
/// use array_range_query::SegTreeSpec;
///
/// struct MinSpec;
/// impl SegTreeSpec for MinSpec {
///     type T = i32;
///     const ID: Self::T = i32::MAX;  // MAX is the identity for min
///
///     fn op(a: &mut Self::T, b: &Self::T) {
///         if *a > *b {
///             *a = *b;
///         }
///     }
/// }
/// ```
pub trait SegTreeSpec {
    /// The type of the elements stored and operated on in the segment tree.
    ///
    /// This type must implement `Clone` to allow efficient copying during tree operations.
    type T: Clone;

    /// The identity element for the monoid operation `op`.
    ///
    /// This element must satisfy the identity property: for any element `a` of type `T`,
    /// `op(a, ID)` must equal `a`. Common examples:
    /// - `0` for addition
    /// - `1` for multiplication
    /// - `i32::MIN` for maximum operations
    /// - `i32::MAX` for minimum operations
    const ID: Self::T;

    /// The associative binary operation of the monoid, performed in-place.
    ///
    /// This operation must be associative: `op(a, op(b, c))` must be equal
    /// to `op(op(a, b), c)` for all valid inputs.
    ///
    /// The operation mutates the first parameter `a` to store the result
    /// of combining `a` with `b`.
    ///
    /// # Parameters
    /// - `a`: The left operand (modified in-place to store the result)
    /// - `b`: The right operand (read-only)
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

    /// Creates a new `SegTree` of a given `size`, initialized with identity elements.
    ///
    /// All elements in the tree are initialized to `Spec::ID`. The internal tree size
    /// (`max_size`) will be the smallest power of two greater than or equal to `size`
    /// for optimal tree structure and performance.
    ///
    /// # Time Complexity
    /// O(n) where n is the smallest power of two ≥ `size`.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{SegTree, SegTreeSpec};
    ///
    /// struct SumSpec;
    /// impl SegTreeSpec for SumSpec {
    ///     type T = i64;
    ///     const ID: Self::T = 0;
    ///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
    /// }
    ///
    /// let tree = SegTree::<SumSpec>::new(100);
    /// assert_eq!(tree.query(..), 0); // All elements are 0 (identity)
    /// ```
    pub fn new(size: usize) -> Self {
        let max_size = size.next_power_of_two();
        Self {
            size,
            max_size,
            data: vec![Spec::ID; max_size * 2].into_boxed_slice(),
            _spec: PhantomData,
        }
    }

    /// Creates a new `SegTree` from a slice of initial values.
    ///
    /// The tree is built in O(n) time using a bottom-up approach, which is more
    /// efficient than creating an empty tree and updating each element individually.
    /// This is the recommended way to initialize a segment tree with known values.
    ///
    /// # Time Complexity
    /// O(n) where n is the length of `values`.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{SegTree, SegTreeSpec};
    ///
    /// struct SumSpec;
    /// impl SegTreeSpec for SumSpec {
    ///     type T = i64;
    ///     const ID: Self::T = 0;
    ///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
    /// }
    ///
    /// let values = vec![1, 2, 3, 4, 5];
    /// let tree = SegTree::<SumSpec>::from_slice(&values);
    /// assert_eq!(tree.query(..), 15); // Sum of all elements
    /// assert_eq!(tree.query(1..4), 9); // Sum of elements [2, 3, 4]
    /// ```
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

    /// Creates a new `SegTree` from a vector of initial values
    ///
    /// The tree is built in O(n) time using a bottom-up approach, which is more
    /// efficient than creating an empty tree and updating each element individually.
    /// This is the recommended way to initialize a segment tree with known values.
    ///
    /// # Time Complexity
    /// O(n) where n is the length of `values`.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{SegTree, SegTreeSpec};
    ///
    /// struct SumSpec;
    /// impl SegTreeSpec for SumSpec {
    ///     type T = i64;
    ///     const ID: Self::T = 0;
    ///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
    /// }
    ///
    /// let values = vec![1, 2, 3, 4, 5];
    /// let tree = SegTree::<SumSpec>::from_vec(values);
    /// assert_eq!(tree.query(..), 15); // Sum of all elements
    /// assert_eq!(tree.query(1..4), 9); // Sum of elements [2, 3, 4]
    /// ```
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

    /// Queries the segment tree for the aggregated value in the given `range`.
    ///
    /// Returns the result of applying the monoid operation across all elements
    /// in the specified range. The range can be any type that implements
    /// `RangeBounds<usize>`, such as `a..b`, `a..=b`, `..b`, `a..`, or `..`.
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// - If the start of the range is greater than the end
    /// - If the end of the range is out of bounds (> `self.size`)
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{SegTree, SegTreeSpec};
    ///
    /// struct SumSpec;
    /// impl SegTreeSpec for SumSpec {
    ///     type T = i64;
    ///     const ID: Self::T = 0;
    ///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
    /// }
    ///
    /// let values = vec![1, 2, 3, 4, 5];
    /// let tree = SegTree::<SumSpec>::from_slice(&values);
    ///
    /// assert_eq!(tree.query(..), 15);      // All elements: 1+2+3+4+5
    /// assert_eq!(tree.query(1..4), 9);     // Elements [1,4): 2+3+4
    /// assert_eq!(tree.query(2..=4), 12);   // Elements [2,4]: 3+4+5
    /// assert_eq!(tree.query(..3), 6);      // Elements [0,3): 1+2+3
    /// assert_eq!(tree.query(2..), 12);     // Elements [2,∞): 3+4+5
    /// ```
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

    /// Performs a point update, replacing the value at `index` with a new `value`.
    ///
    /// After updating the leaf node, the change is propagated up the tree by
    /// recalculating all ancestor nodes. This maintains the tree invariant that
    /// each internal node contains the aggregate of its subtree.
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// Panics if `index >= self.size` (index out of bounds).
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{SegTree, SegTreeSpec};
    ///
    /// struct SumSpec;
    /// impl SegTreeSpec for SumSpec {
    ///     type T = i64;
    ///     const ID: Self::T = 0;
    ///     fn op(a: &mut Self::T, b: &Self::T) { *a += *b; }
    /// }
    ///
    /// let values = vec![1, 2, 3, 4, 5];
    /// let mut tree = SegTree::<SumSpec>::from_vec(values);
    ///
    /// assert_eq!(tree.query(..), 15);  // Original sum
    ///
    /// tree.update(2, 10);              // Change 3 to 10
    /// assert_eq!(tree.query(..), 22);  // New sum: 1+2+10+4+5
    /// assert_eq!(tree.query(2..3), 10); // Just the updated element
    /// ```
    pub fn update(&mut self, index: usize, value: Spec::T) {
        assert!(index < self.size, "update index out of bounds");

        let leaf_index = index + self.max_size;
        self.data[leaf_index] = value;
        self.recompute(leaf_index);
    }

    // ===== PRIVATE HELPER METHODS =====

    /// Private helper to recompute parent nodes from a leaf up to the root.
    ///
    /// Starting from the given `index` (which should be a leaf node), this method
    /// walks up the tree to the root, updating each parent node by combining
    /// the values of its two children using the monoid operation.
    ///
    /// # Parameters
    /// - `index`: The leaf index in the `data` vector (should be >= `max_size`)
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
