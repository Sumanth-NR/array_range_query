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
# use crate::lazy_seg_tree::{LazySegTree, LazySegTreeSpec};
# struct RangeAddSum;
# impl LazySegTreeSpec for RangeAddSum {
#     type T = i64;
#     type U = i64;
#     const ID: Self::T = 0;
#     fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T { d1 + d2 }
#     fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U { u1 + u2 }
#     fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
#         d + (u * size as i64)
#     }
# }
let mut tree = LazySegTree::<RangeAddSum>::from_vec(&vec![1,2,3,4,5]);
assert_eq!(tree.query(1, 4), 9); // 2 + 3 + 4
tree.update(1, 4, 10); // add 10 to indices 1..4
assert_eq!(tree.query(0, 5), 45);
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

use std::cell::RefCell;
use std::fmt::Display;
use std::marker::PhantomData;

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
    type T: Clone;
    type U: Clone;

    /// Identity element for `T`.
    const ID: Self::T;

    /// Combine two child values into a parent value.
    fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T;

    /// Compose two updates. If update `a` is applied before `b`, then the
    /// composed update should be `op_on_update(a, b)` (document the intended
    /// order in non-commutative cases).
    fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U;

    /// Apply an update `u` to a node's stored aggregate `d` which represents
    /// `size` leaves. For example, for range-add + range-sum:
    /// `op_update_on_data(u, d, size) = d + u * size`.
    fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T;

    /// Helper to combine an existing optional tag with a new tag.
    ///
    /// Default behavior: if there is an existing tag, compose them using
    /// `op_on_update`; otherwise, install `new_tag`.
    fn op_on_update_option(existing_tag: &Option<Self::U>, new_tag: &Self::U) -> Option<Self::U> {
        if let Some(existing) = existing_tag {
            Some(Self::op_on_update(existing, new_tag))
        } else {
            Some(new_tag.clone())
        }
    }
}

/// Generic lazy segment tree.
///
/// The tree stores aggregates of type `Spec::T` and lazy tags of `Spec::U`.
/// Public operations:
/// - `new(size)` and `from_vec(values)` to construct,
/// - `query(&self, left, right)` to query `[left, right)`,
/// - `update(&mut self, left, right, value)` to apply a lazy update to `[left, right)`.
#[derive(Clone, Debug)]
pub struct LazySegTree<Spec: LazySegTreeSpec> {
    size: usize,
    max_size: usize,
    data: RefCell<Vec<Spec::T>>,
    tags: RefCell<Vec<Option<Spec::U>>>,
    _spec: PhantomData<Spec>,
}

impl<Spec: LazySegTreeSpec> LazySegTree<Spec> {
    /// Create a new tree of `size` elements, all initialized to `Spec::ID`.
    ///
    /// `max_size` (the number of leaves used internally) will be the next power
    /// of two greater than or equal to `size`.
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

    /// Build a tree from a slice of initial values. Complexity: O(n).
    ///
    /// The slice length determines the logical `size`.
    pub fn from_vec(values: &[Spec::T]) -> Self {
        let size = values.len();
        let max_size = size.next_power_of_two();
        let mut data = vec![Spec::ID; max_size * 2];

        // Copy leaves and build internal nodes bottom-up.
        if size > 0 {
            data[max_size..(max_size + size)].clone_from_slice(values);
            for i in (1..max_size).rev() {
                data[i] = Spec::op_on_data(&data[i * 2], &data[i * 2 + 1]);
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

    /// Query the half-open interval `[left, right)`.
    ///
    /// Returns `Spec::ID` for empty ranges (`left == right`).
    /// Panics if the range is invalid (see `validate_range`).
    pub fn query(&self, left: usize, right: usize) -> Spec::T {
        self.validate_range(left, right);
        if left == right {
            return Spec::ID;
        }
        self.query_internal(1, 0, left, right, self.max_size)
    }

    /// Apply `value` lazily to the half-open interval `[left, right)`.
    ///
    /// Requires `&mut self`. Panics if range is invalid.
    pub fn update(&mut self, left: usize, right: usize, value: Spec::U) {
        self.validate_range(left, right);
        if left == right {
            return;
        }
        self.update_internal(1, 0, left, right, self.max_size, value);
    }

    /// Push a pending tag at `index` down to the node's data and to its children.
    ///
    /// This version is used from `&self` paths (e.g. `query`) and therefore uses
    /// `RefCell` borrows.
    ///
    /// `node_size` is the number of leaves covered by this node.
    ///
    /// # Panics
    ///
    /// Panics if `RefCell` borrow rules are violated (e.g., a conflicting borrow
    /// exists when this method is called).
    fn push(&self, index: usize, node_size: usize) {
        // Borrow tags mutably. This may panic if a conflicting borrow exists.
        let mut tags = self.tags.borrow_mut();
        if let Some(tag) = tags[index].take() {
            let mut data = self.data.borrow_mut();
            let old_data = data[index].clone();
            data[index] = Spec::op_update_on_data(&tag, &old_data, node_size);

            if index < self.max_size {
                tags[index * 2] = Spec::op_on_update_option(&tags[index * 2], &tag);
                tags[index * 2 + 1] = Spec::op_on_update_option(&tags[index * 2 + 1], &tag);
            }
        }
    }

    /// Push a pending tag at `index` down using direct mutable access.
    ///
    /// Called from `&mut self` update paths; avoids `RefCell` overhead.
    fn push_mut(&mut self, index: usize, node_size: usize) {
        let tags = self.tags.get_mut();
        if let Some(tag) = tags[index].take() {
            let data = self.data.get_mut();
            let old_data = data[index].clone();
            data[index] = Spec::op_update_on_data(&tag, &old_data, node_size);

            if index < self.max_size {
                tags[index * 2] = Spec::op_on_update_option(&tags[index * 2], &tag);
                tags[index * 2 + 1] = Spec::op_on_update_option(&tags[index * 2 + 1], &tag);
            }
        }
    }

    /// Recompute the value at `index` from its children.
    ///
    /// This uses direct mutable access via `get_mut()` because `pull_mut` is
    /// only called from `&mut self` paths.
    fn pull_mut(&mut self, index: usize) {
        let data = self.data.get_mut();
        data[index] = Spec::op_on_data(&data[index * 2], &data[index * 2 + 1]);
    }

    /// Internal recursive query implementation.
    ///
    /// - `index`: current node index in the array.
    /// - `node_left..node_right`: interval covered by this node (half-open).
    fn query_internal(
        &self,
        index: usize,
        node_left: usize,
        left: usize,
        right: usize,
        node_right: usize,
    ) -> Spec::T {
        // No overlap.
        if node_right <= left || right <= node_left {
            return Spec::ID;
        }

        // Ensure current node's pending tag (if any) is applied before reading.
        self.push(index, node_right - node_left);
        if left <= node_left && node_right <= right {
            // Full cover.
            return self.data.borrow()[index].clone();
        } else {
            // Partial cover — combine children.
            let mid = (node_left + node_right) / 2;
            let left_result = self.query_internal(index * 2, node_left, left, right, mid);
            let right_result = self.query_internal(index * 2 + 1, mid, left, right, node_right);
            Spec::op_on_data(&left_result, &right_result)
        }
    }

    /// Internal recursive update implementation.
    ///
    /// Applies `value` to `[left, right)` within node covering `node_left..node_right`.
    fn update_internal(
        &mut self,
        index: usize,
        node_left: usize,
        left: usize,
        right: usize,
        node_right: usize,
        value: Spec::U,
    ) {
        // No overlap: ensure the node's pending tag (if any) is applied so that
        // parent invariants remain correct.
        if left >= node_right || right <= node_left {
            self.push_mut(index, node_right - node_left);
        } else if left <= node_left && node_right <= right {
            // Fully covered: compose the new tag with any existing tag.
            {
                let tags = self.tags.get_mut();
                tags[index] = Spec::op_on_update_option(&tags[index], &value);
            }
            // Apply it immediately to the node's stored data (and propagate to children).
            self.push_mut(index, node_right - node_left);
        } else {
            // Partial overlap: push pending tag before recurring.
            self.push_mut(index, node_right - node_left);
            let mid = (node_left + node_right) / 2;
            self.update_internal(index * 2, node_left, left, right, mid, value.clone());
            self.update_internal(index * 2 + 1, mid, left, right, node_right, value);
            // Recompute this node's aggregate after children updates.
            self.pull_mut(index);
        }
    }

    /// Validate the half-open range `[left, right)`.
    ///
    /// Panics with a descriptive message if the range is invalid.
    fn validate_range(&self, left: usize, right: usize) {
        assert!(
            left <= right,
            "Invalid range: `left` must be less than or equal to `right`. Got left: {}, right: {}",
            left,
            right
        );
        assert!(
            right <= self.size,
            "Out of bounds: `right` must be within the structure's size. Got right: {}, size: {}",
            right,
            self.size
        );
    }
}

/// Helper to pretty-print optional values arranged as a binary tree.
///
/// This helper is used by `Display` to render non-identity node values and
/// pending tags for inspection/debugging.
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
        // Header: show types and logical size.
        write!(f, "Type: {}", std::any::type_name::<Spec::T>())?;
        write!(f, ", Update Type: {}", std::any::type_name::<Spec::U>())?;
        write!(f, ", Size: {}", self.size)?;

        // Inspect buffers.
        let data = self.data.borrow();
        let tags = self.tags.borrow();

        // Convert `data` into Option<T> so we can show only non-identity entries.
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
        let tag_values = tags.iter().map(|x| x).collect::<Vec<_>>();

        writeln!(f, "\nData: [")?;
        print_tree_option(f, &data_values, 1, 1, 0, self.max_size)?;
        writeln!(f, "]")?;

        writeln!(f, "Tags: [")?;
        print_tree_option(f, &tag_values, 1, 1, 0, self.max_size)?;
        writeln!(f, "]")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct RangeAddSum;
    impl LazySegTreeSpec for RangeAddSum {
        type T = i64;
        type U = i64;
        const ID: Self::T = 0;

        fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T {
            d1 + d2
        }
        fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U {
            u1 + u2
        }
        fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
            d + (u * size as i64)
        }
    }

    #[test]
    fn test_from_vec_and_initial_query() {
        let tree = LazySegTree::<RangeAddSum>::from_vec(&vec![1, 2, 3, 4, 5]);
        assert_eq!(tree.query(0, 5), 15);
        assert_eq!(tree.query(2, 4), 7);
    }

    #[test]
    fn test_update_and_query() {
        let mut tree = LazySegTree::<RangeAddSum>::new(7);
        tree.update(0, 5, 10); // Add 10 to first 5 elements
        assert_eq!(tree.query(0, 7), 50);
        tree.update(2, 7, -5); // Subtract 5 from elements 2,3,4,5,6
        assert_eq!(tree.query(0, 2), 20); // 10 + 10
        assert_eq!(tree.query(2, 5), 15); // (10-5) + (10-5) + (10-5)
        assert_eq!(tree.query(0, 7), 25); // 20 + 15 + (-5 * 2)
    }

    #[test]
    fn test_overlapping_updates() {
        let mut tree = LazySegTree::<RangeAddSum>::new(10);
        tree.update(0, 6, 5);
        assert_eq!(tree.query(0, 10), 30);
        tree.update(4, 8, 10);
        let expected = (5 * 4) + ((5 + 10) * 2) + (10 * 2);
        assert_eq!(tree.query(0, 10), expected);
        assert_eq!(tree.query(4, 6), 30);
    }

    #[test]
    #[should_panic]
    fn test_panic_invalid_range() {
        let tree = LazySegTree::<RangeAddSum>::new(10);
        tree.query(5, 4);
    }
}
