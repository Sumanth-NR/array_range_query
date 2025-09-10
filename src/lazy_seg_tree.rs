/*!
Lazy Segment Tree (range-update, range-query) — generic implementation.

## Overview

This module implements a generic lazy segment tree: a binary tree stored in
an array that supports efficient range updates and range queries.

The implementation is deliberately generic and configurable via the
`LazySegTreeSpec` trait. The trait allows you to define:
- the stored value type `T` and the lazy-update type `U`,
- how two values `T` are combined (aggregation),
- how two updates `U` are composed, and
- how a lazy update affects a node's stored aggregate (taking into account
  the number of leaves covered by that node).

## Design notes

- API distinction between read and write:
  - `query(&self, ...)` is provided as `&self`. Internally it uses
    `RefCell` to push lazy tags while preserving a read-only public API.
  - `update(&mut self, ...)` requires `&mut self` and uses direct mutable
    access to the internal buffers for better performance and simpler
    borrowing semantics.

- Storage layout:
  - A complete binary tree is stored in a `Vec` using 1-based indexing
    (i.e. root at index `1`). The number of leaves is `max_size`, the next
    power of two >= logical `size`. Total storage uses `max_size * 2` slots.

- Ranges are half-open: `[left, right)`. This is consistent across
  `query` and `update`.

## Usage summary

1. Implement `LazySegTreeSpec` for your problem domain (range add / range
   sum, range assign / range min, etc.).
2. Construct a tree:
   - `LazySegTree::new(size)` creates a tree with all values set to `Spec::ID`.
   - `LazySegTree::from_vec(values)` builds the tree from an initial slice.
3. Use `query` and `update` to perform operations. `query` will return the
   aggregate over a half-open interval, and `update` will apply a lazy
   update to every element in the interval.

## Example (Range Add + Range Sum)

```rust
use array_range_query::{LazySegTree, LazySegTreeSpec};

struct RangeAddSum;

impl LazySegTreeSpec for RangeAddSum {
    type T = i64;
    type U = i64;
    const ID: Self::T = 0;

    fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
        *d1 += *d2;
    }

    fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
        *u1 += *u2;
    }

    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
        *d += u * size as i64;
    }
}

let mut tree = LazySegTree::<RangeAddSum>::from_vec(vec![1,2,3,4,5]);
assert_eq!(tree.query(1..4), 9); // 2 + 3 + 4
tree.update(1..4, 10); // add 10 to indices 1..4
assert_eq!(tree.query(..), 45);
```

## Panics and safety

- `validate_range` asserts that `left <= right` and `right <= size`. If the
  caller violates these preconditions the library panics with a helpful
  message.
- Because `query(&self, ..)` uses interior mutability (`RefCell`), a
  runtime panic will occur if external code causes conflicting borrows that
  violate `RefCell` rules. Typical use does not trigger this, but it is a
  potential runtime failure mode to be aware of.
*/

use crate::utils;
use core::marker::PhantomData;
use core::ops::RangeBounds;

use core::cell::RefCell;
use core::fmt::Display;

/// Specification trait for `LazySegTree`.
///
/// Implement this trait to define the concrete behavior of the tree:
/// - `T`: data stored in nodes (the aggregate type),
/// - `U`: lazy-update type (the tag type),
/// - `ID`: identity element for `T`.
///
/// Required operations:
/// - `op_on_data`          — combine two `T`s into one (e.g., sum or min),
/// - `op_on_update`        — compose two updates `U` into a single update,
/// - `op_update_on_data`   — apply an update `U` to `T` representing `size` leaves.
pub trait LazySegTreeSpec {
    /// The type of elements stored and operated on in the tree nodes.
    type T: Clone;

    /// The type of lazy updates applied to ranges.
    type U: Clone;

    /// Identity element for `T`.
    ///
    /// This is the neutral element for the `op_on_data` operation.
    /// For sum operations, this would be 0; for min operations, this would be the maximum value.
    const ID: Self::T;

    /// Combine two child values into a parent value, performed in-place.
    ///
    /// This operation must be associative. Mutates `d1` to be the result of combining
    /// `d1` with `d2`. For example, for range sum queries: `*d1 += *d2`.
    fn op_on_data(d1: &mut Self::T, d2: &Self::T);

    /// Compose two updates, performed in-place.
    ///
    /// If update `u1` is applied before `u2`, then the composed update should be
    /// the result of `op_on_update(u1, u2)`. This operation must be associative.
    /// Mutates `u1` to be the result of the composition.
    fn op_on_update(u1: &mut Self::U, u2: &Self::U);

    /// Apply an update `u` to a node's stored aggregate `d` which represents `size` leaves.
    ///
    /// This operation is performed in-place and mutates `d` to be the result.
    /// For example, for range-add + range-sum: `*d += u * size`.
    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize);
}

/// Generic lazy segment tree.
///
/// The tree stores aggregates of type `Spec::T` and lazy tags of `Spec::U`.
/// Supports efficient range queries and range updates in O(log n) time.
///
/// # Type Parameters
///
/// * `Spec` - A type implementing `LazySegTreeSpec` that defines the operations
#[derive(Clone, Debug)]
pub struct LazySegTree<Spec: LazySegTreeSpec> {
    /// The logical size of the array (as provided by the user)
    size: usize,
    /// The number of leaf nodes in the internal tree (next power of 2 >= size)
    max_size: usize,
    /// Tree data stored as a flat vector with 1-based indexing
    data: RefCell<Vec<Spec::T>>,
    /// Lazy propagation tags for pending updates
    tags: RefCell<Vec<Option<Spec::U>>>,
    /// Zero-sized marker to associate the `Spec` type with the struct
    _spec: PhantomData<Spec>,
}

impl<Spec: LazySegTreeSpec> LazySegTree<Spec> {
    // ===== CONSTRUCTORS =====

    /// Create a new tree of `size` elements, all initialized to `Spec::ID`.
    ///
    /// The internal tree size (`max_size`) will be the next power of two
    /// greater than or equal to `size` for efficient tree operations.
    ///
    /// # Time Complexity
    /// O(n) where n is the next power of two >= `size`.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{LazySegTree, LazySegTreeSpec};
    /// # struct RangeAddSum;
    /// # impl LazySegTreeSpec for RangeAddSum {
    /// #     type T = i64; type U = i64; const ID: Self::T = 0;
    /// #     fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    /// #     fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    /// #     fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) { *d += u * size as i64; }
    /// # }
    /// let tree = LazySegTree::<RangeAddSum>::new(100);
    /// assert_eq!(tree.query(..), 0); // All elements start as identity (0)
    /// ```
    pub fn new(size: usize) -> Self {
        let max_size = size.next_power_of_two();
        Self {
            size,
            max_size,
            data: RefCell::new(vec![Spec::ID; max_size * 2]),
            tags: RefCell::new(vec![None; max_size * 2]),
            _spec: PhantomData,
        }
    }

    /// Build a tree from a slice of initial values.
    ///
    /// The tree is constructed bottom-up in O(n) time, which is more efficient
    /// than creating an empty tree and updating each element individually.
    ///
    /// # Time Complexity
    /// O(n) where n is the length of `values`.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{LazySegTree, LazySegTreeSpec};
    /// # struct RangeAddSum;
    /// # impl LazySegTreeSpec for RangeAddSum {
    /// #     type T = i64; type U = i64; const ID: Self::T = 0;
    /// #     fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    /// #     fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    /// #     fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) { *d += u * size as i64; }
    /// # }
    /// let values = vec![1, 2, 3, 4, 5];
    /// let tree = LazySegTree::<RangeAddSum>::from_slice(&values);
    /// assert_eq!(tree.query(..), 15); // Sum of all elements
    /// ```
    pub fn from_slice(values: &[Spec::T]) -> Self {
        let size = values.len();
        let max_size = size.next_power_of_two();
        let mut data = vec![Spec::ID; max_size * 2];

        // Copy leaves and build internal nodes bottom-up
        if size > 0 {
            data[max_size..(max_size + size)].clone_from_slice(values);
            for i in (1..max_size).rev() {
                let mut v = data[i * 2].clone();
                Spec::op_on_data(&mut v, &data[i * 2 + 1]);
                data[i] = v;
            }
        }

        Self {
            size,
            max_size,
            data: RefCell::new(data),
            tags: RefCell::new(vec![None; max_size * 2]),
            _spec: PhantomData,
        }
    }

    /// Creates a new `LazySegTree` from a vector of initial values.
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
    /// use array_range_query::{LazySegTree, LazySegTreeSpec};
    /// # struct RangeAddSum;
    /// # impl LazySegTreeSpec for RangeAddSum {
    /// #     type T = i64; type U = i64; const ID: Self::T = 0;
    /// #     fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    /// #     fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    /// #     fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) { *d += u * size as i64; }
    /// # }
    /// let values = vec![1, 2, 3, 4, 5];
    /// let tree = LazySegTree::<RangeAddSum>::from_vec(values);
    /// assert_eq!(tree.query(..), 15); // Sum of all elements
    /// ```
    pub fn from_vec(values: Vec<Spec::T>) -> Self {
        let size = values.len();
        let max_size = size.next_power_of_two();
        let mut data = vec![Spec::ID; max_size * 2];

        // Move owned values directly into the leaf slots to avoid cloning
        if size > 0 {
            for (i, v) in values.into_iter().enumerate() {
                data[max_size + i] = v;
            }
            // Build internal nodes bottom-up
            for i in (1..max_size).rev() {
                let mut v = data[i * 2].clone();
                Spec::op_on_data(&mut v, &data[i * 2 + 1]);
                data[i] = v;
            }
        }

        Self {
            size,
            max_size,
            data: RefCell::new(data),
            tags: RefCell::new(vec![None; max_size * 2]),
            _spec: PhantomData,
        }
    }

    // ===== PUBLIC INTERFACE =====

    /// Query the range specified by `range`.
    ///
    /// Returns the aggregate value over the specified range using the `op_on_data` operation.
    /// The range can be any type that implements `RangeBounds<usize>`, such as
    /// `a..b`, `a..=b`, `..b`, `a..`, or `..`.
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// Panics if the range is invalid (start > end) or out of bounds.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{LazySegTree, LazySegTreeSpec};
    /// # struct RangeAddSum;
    /// # impl LazySegTreeSpec for RangeAddSum {
    /// #     type T = i64; type U = i64; const ID: Self::T = 0;
    /// #     fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    /// #     fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    /// #     fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) { *d += u * size as i64; }
    /// # }
    /// let tree = LazySegTree::<RangeAddSum>::from_vec(vec![1, 2, 3, 4, 5]);
    /// assert_eq!(tree.query(1..4), 9);  // Sum of elements [2, 3, 4]
    /// assert_eq!(tree.query(..), 15);   // Sum of all elements
    /// ```
    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> Spec::T {
        let (left, right) = utils::parse_range(range, self.size);
        utils::validate_range(left, right, self.size);

        if left == right {
            return Spec::ID;
        }

        self.query_internal(1, 0, left, right, self.max_size)
    }

    /// Apply `value` lazily to the range specified by `range`.
    ///
    /// Updates all elements in the specified range by applying the given update value.
    /// The range can be any type that implements `RangeBounds<usize>`.
    ///
    /// # Time Complexity
    /// O(log n)
    ///
    /// # Panics
    /// Panics if the range is invalid (start > end) or out of bounds.
    ///
    /// # Examples
    /// ```
    /// use array_range_query::{LazySegTree, LazySegTreeSpec};
    /// # struct RangeAddSum;
    /// # impl LazySegTreeSpec for RangeAddSum {
    /// #     type T = i64; type U = i64; const ID: Self::T = 0;
    /// #     fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    /// #     fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    /// #     fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) { *d += u * size as i64; }
    /// # }
    /// let mut tree = LazySegTree::<RangeAddSum>::from_vec(vec![1, 2, 3, 4, 5]);
    /// tree.update(1..4, 10); // Add 10 to elements at indices 1, 2, 3
    /// assert_eq!(tree.query(..), 45); // 1 + 12 + 13 + 14 + 5
    /// ```
    pub fn update<R: RangeBounds<usize>>(&mut self, range: R, value: Spec::U) {
        let (left, right) = utils::parse_range(range, self.size);
        utils::validate_range(left, right, self.size);

        if left == right {
            return;
        }

        self.update_internal(1, 0, left, right, self.max_size, value);
    }

    // ===== PRIVATE HELPER METHODS =====

    /// Private helper to combine an existing optional tag with a new tag.
    ///
    /// If there's an existing tag, compose them using `op_on_update`.
    /// Otherwise, install the new tag.
    fn combine_tag_option(existing_tag: &mut Option<Spec::U>, new_tag: &Spec::U) {
        if let Some(existing) = existing_tag {
            Spec::op_on_update(existing, new_tag);
        } else {
            *existing_tag = Some(new_tag.clone());
        }
    }

    /// Push a pending tag at `index` down to the node's data and to its children.
    ///
    /// This version is used from `&self` query paths and uses `RefCell` borrows.
    /// The `node_size` parameter represents the number of leaves covered by this node.
    ///
    /// # Panics
    /// Panics if `RefCell` borrow rules are violated (e.g., conflicting borrows exist).
    fn push(&self, index: usize, node_size: usize) {
        let mut tags = self.tags.borrow_mut();
        if let Some(tag) = tags[index].take() {
            let mut data = self.data.borrow_mut();
            Spec::op_update_on_data(&tag, &mut data[index], node_size);

            if index < self.max_size {
                Self::combine_tag_option(&mut tags[index * 2], &tag);
                Self::combine_tag_option(&mut tags[index * 2 + 1], &tag);
            }
        }
    }

    /// Push a pending tag at `index` down using direct mutable access.
    ///
    /// Called from `&mut self` update paths; avoids `RefCell` overhead for better performance.
    fn push_mut(&mut self, index: usize, node_size: usize) {
        let tags = self.tags.get_mut();
        if let Some(tag) = tags[index].take() {
            let data = self.data.get_mut();
            Spec::op_update_on_data(&tag, &mut data[index], node_size);

            if index < self.max_size {
                Self::combine_tag_option(&mut tags[index * 2], &tag);
                Self::combine_tag_option(&mut tags[index * 2 + 1], &tag);
            }
        }
    }

    /// Recompute the value at `index` from its children.
    ///
    /// This method pulls up values from child nodes and combines them using `op_on_data`.
    /// Only called from `&mut self` paths, so uses direct mutable access.
    fn pull_mut(&mut self, index: usize) {
        let data = self.data.get_mut();
        let mut v = data[index * 2].clone();
        Spec::op_on_data(&mut v, &data[index * 2 + 1]);
        data[index] = v;
    }

    /// Internal recursive query implementation.
    ///
    /// Traverses the tree to find nodes that overlap with the query range [left, right)
    /// and combines their values.
    ///
    /// # Parameters
    /// - `index`: Current node index in the tree array
    /// - `node_left`, `node_right`: Half-open interval [node_left, node_right) covered by this node
    /// - `left`, `right`: Query range [left, right)
    fn query_internal(
        &self,
        index: usize,
        node_left: usize,
        left: usize,
        right: usize,
        node_right: usize,
    ) -> Spec::T {
        // No overlap between node range and query range
        if node_right <= left || right <= node_left {
            return Spec::ID;
        }

        // Ensure current node's pending tag (if any) is applied before reading
        self.push(index, node_right - node_left);

        if left <= node_left && node_right <= right {
            // Node is fully covered by the query range
            return self.data.borrow()[index].clone();
        } else {
            // Partial overlap - combine results from children
            let mid = (node_left + node_right) / 2;
            let mut left_result = self.query_internal(index * 2, node_left, left, right, mid);
            let right_result = self.query_internal(index * 2 + 1, mid, left, right, node_right);
            Spec::op_on_data(&mut left_result, &right_result);
            left_result
        }
    }

    /// Internal recursive update implementation.
    ///
    /// Applies the update `value` to all elements in the range [left, right).
    /// Uses lazy propagation to efficiently handle range updates.
    ///
    /// # Parameters
    /// - `index`: Current node index in the tree array
    /// - `node_left`, `node_right`: Half-open interval [node_left, node_right) covered by this node
    /// - `left`, `right`: Update range [left, right)
    /// - `value`: Update value to apply
    fn update_internal(
        &mut self,
        index: usize,
        node_left: usize,
        left: usize,
        right: usize,
        node_right: usize,
        value: Spec::U,
    ) {
        if left >= node_right || right <= node_left {
            // No overlap: ensure the node's pending tag is applied to maintain invariants
            self.push_mut(index, node_right - node_left);
        } else if left <= node_left && node_right <= right {
            // Node is fully covered: apply lazy update
            {
                let tags = self.tags.get_mut();
                Self::combine_tag_option(&mut tags[index], &value);
            }
            // Apply the update immediately to this node's stored data
            self.push_mut(index, node_right - node_left);
        } else {
            // Partial overlap: push pending updates before recursing
            self.push_mut(index, node_right - node_left);
            let mid = (node_left + node_right) / 2;
            self.update_internal(index * 2, node_left, left, right, mid, value.clone());
            self.update_internal(index * 2 + 1, mid, left, right, node_right, value);
            // Recompute this node's aggregate after updating children
            self.pull_mut(index);
        }
    }
}

// ===== DISPLAY IMPLEMENTATION =====

/// Helper function to pretty-print optional values arranged as a binary tree.
///
/// Used by the `Display` implementation to render non-identity node values and
/// pending tags for debugging and inspection purposes.
fn print_tree_option<T: Display>(
    f: &mut std::fmt::Formatter<'_>,
    tree: &Vec<&Option<T>>,
    index: usize,
    depth: usize,
    l: usize,
    r: usize,
) -> std::fmt::Result {
    if index >= tree.len() {
        return Ok(());
    }

    if let Some(value) = &tree[index] {
        for _ in 0..depth {
            write!(f, "  ")?;
        }
        writeln!(f, "{} (Index: {}, Covers [{}, {}))", value, index, l, r)?;
    }

    if index * 2 + 1 < tree.len() {
        print_tree_option(f, tree, index * 2, depth + 1, l, (l + r) / 2)?;
        print_tree_option(f, tree, index * 2 + 1, depth + 1, (l + r) / 2, r)?;
    }

    Ok(())
}

impl<Spec: LazySegTreeSpec> Display for LazySegTree<Spec>
where
    Spec::T: Display + PartialEq,
    Spec::U: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Header: show types and logical size
        writeln!(f, "LazySegTree {{")?;
        writeln!(f, "  Data Type: {}", std::any::type_name::<Spec::T>())?;
        writeln!(f, "  Update Type: {}", std::any::type_name::<Spec::U>())?;
        writeln!(f, "  Size: {} (Internal: {})", self.size, self.max_size)?;

        // Inspect internal data structures
        let data = self.data.borrow();
        let tags = self.tags.borrow();

        // Show only non-identity data values
        let data_values: Vec<Option<Spec::T>> = data
            .iter()
            .map(|x| {
                if *x != Spec::ID {
                    Some(x.clone())
                } else {
                    None
                }
            })
            .collect();
        let data_values = data_values.iter().collect::<Vec<_>>();
        let tag_values = tags.iter().collect::<Vec<_>>();

        writeln!(f, "  Data:")?;
        print_tree_option(f, &data_values, 1, 2, 0, self.max_size)?;

        writeln!(f, "  Lazy Tags:")?;
        print_tree_option(f, &tag_values, 1, 2, 0, self.max_size)?;

        writeln!(f, "}}")?;

        Ok(())
    }
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    /// Test specification for range add updates with sum queries.
    #[derive(Debug)]
    struct RangeAddSum;

    impl LazySegTreeSpec for RangeAddSum {
        type T = i64;
        type U = i64;
        const ID: Self::T = 0;

        fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
            *d1 += *d2;
        }

        fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
            *u1 += *u2;
        }

        fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
            *d += u * size as i64;
        }
    }

    #[test]
    fn constructors() {
        // `new` should create an identity-filled tree
        let tree = LazySegTree::<RangeAddSum>::new(8);
        assert_eq!(tree.query(..), 0);

        // `from_slice` should build from a slice
        let arr = [1i64, 2, 3, 4, 5, 6, 7, 8];
        let tree_slice = LazySegTree::<RangeAddSum>::from_slice(&arr);
        assert_eq!(tree_slice.query(..), 36);
        assert_eq!(tree_slice.query(3..6), 4 + 5 + 6);

        // `from_vec` should consume an owned vector
        let tree_vec = LazySegTree::<RangeAddSum>::from_vec(vec![1i64, 2, 3]);
        assert_eq!(tree_vec.query(..), 6);
        assert_eq!(tree_vec.query(1..2), 2);
    }

    #[test]
    fn querying() {
        let tree = LazySegTree::<RangeAddSum>::from_vec(vec![1i64, 2, 3, 4, 5, 6, 7, 8]);

        // full range
        assert_eq!(tree.query(..), 36);

        // single elements and small ranges
        assert_eq!(tree.query(0..1), 1);
        assert_eq!(tree.query(7..8), 8);
        assert_eq!(tree.query(2..5), 3 + 4 + 5);

        // prefix / suffix
        assert_eq!(tree.query(..3), 1 + 2 + 3);
        assert_eq!(tree.query(3..), 4 + 5 + 6 + 7 + 8);

        // inclusive ranges and empty range
        assert_eq!(tree.query(..=6), 1 + 2 + 3 + 4 + 5 + 6 + 7);
        assert_eq!(tree.query(3..=5), 4 + 5 + 6);
        assert_eq!(tree.query(4..4), 0);
    }

    #[test]
    fn updating() {
        let mut tree = LazySegTree::<RangeAddSum>::from_vec(vec![1i64, 2, 3, 4, 5]);

        // Range update: add 10 to indices [1,4)
        tree.update(1..4, 10);
        assert_eq!(tree.query(1..4), (2 + 10) + (3 + 10) + (4 + 10));
        assert_eq!(tree.query(..), 1 + (2 + 10) + (3 + 10) + (4 + 10) + 5);

        // Point-like update via single-element range
        tree.update(2..3, -3); // modify index 2
        assert_eq!(tree.query(2..3), (3 + 10) - 3);

        // Empty-range update should be a no-op
        let before = tree.query(..);
        tree.update(2..2, 999);
        assert_eq!(tree.query(..), before);
    }

    #[test]
    fn combination_overlapping_updates() {
        let mut tree = LazySegTree::<RangeAddSum>::from_vec((1..=10).collect::<Vec<_>>());

        // Apply several overlapping updates
        tree.update(..6, 5); // add 5 to indices 0..6
        tree.update(4..8, 10); // add 10 to indices 4..8
        tree.update(2..5, -2); // add -2 to indices 2..5

        // Build expected array by applying same updates
        let mut expected: Vec<i64> = (1..=10).collect();
        for i in 0..6 {
            expected[i] += 5;
        }
        for i in 4..8 {
            expected[i] += 10;
        }
        for i in 2..5 {
            expected[i] += -2;
        }

        // Verify totals and several subranges
        let total_expected: i64 = expected.iter().sum();
        assert_eq!(tree.query(..), total_expected);

        // A few targeted queries
        assert_eq!(tree.query(0..3), expected[0] + expected[1] + expected[2]);
        assert_eq!(tree.query(4..6), expected[4] + expected[5]);
        assert_eq!(tree.query(7..10), expected[7] + expected[8] + expected[9]);
    }

    #[test]
    fn test_overlapping_updates() {
        let mut tree = LazySegTree::<RangeAddSum>::new(10);
        tree.update(..6, 5);
        assert_eq!(tree.query(..), 30);
        tree.update(4..8, 10);
        let expected = (5 * 4) + ((5 + 10) * 2) + (10 * 2);
        assert_eq!(tree.query(..), expected);
        assert_eq!(tree.query(4..6), 30);
    }

    #[test]
    #[should_panic(expected = "Invalid range: got")]
    #[allow(clippy::reversed_empty_ranges)]
    fn test_panic_invalid_range() {
        let tree = LazySegTree::<RangeAddSum>::new(10);
        tree.query(5..4);
    }
}
