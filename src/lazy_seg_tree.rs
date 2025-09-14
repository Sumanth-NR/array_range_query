/*!
Lazy Segment Tree (range-update, range-query) — generic implementation.

## Overview

This module implements a generic lazy segment tree: a binary tree stored in
an array that supports efficient range updates and range queries.

The implementation is deliberately generic and configurable via the
`LazySegTreeSpec` trait. The trait allows you to define:
- the stored value type `T` and the lazy-update type `U`,
- how two values `T` are combined (aggregation),
- how two updates `U` are composed,
- and how a lazy update affects a node's stored aggregate (taking into account
  the number of leaves covered by that node).

## Design notes

- API distinction between read and write:
  - `query(&self, ...)` is provided as `&self`. Internally it uses
    `RefCell` to push lazy tags while preserving a read-only public API.
  - `update(&mut self, ...)` requires `&mut self` and uses direct mutable
    access to the internal buffers for better performance and simpler
    borrowing semantics.

- Storage layout:
  - A complete binary tree is stored in a `Box<[]>` (a boxed slice) using
    1-based indexing (i.e. root at index `1`). The number of leaves is `max_size`,
    the next power of two >= logical `size`. Total storage uses `max_size * 2` slots.

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

use crate::{utils, SegTreeNode};
use core::marker::PhantomData;
use core::ops::RangeBounds;

use core::cell::RefCell;
use core::fmt::Display;

pub trait LazySegTreeSpec {
    type T: Clone;
    type U: Clone;
    const ID: Self::T;
    fn op_on_data(d1: &mut Self::T, d2: &Self::T);
    fn op_on_update(u1: &mut Self::U, u2: &Self::U);
    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize);
}

#[derive(Clone, Debug)]
pub struct LazySegTree<Spec: LazySegTreeSpec> {
    size: usize,
    max_size: usize,
    max_depth: u32,
    data: RefCell<Box<[Spec::T]>>,
    tags: RefCell<Box<[Option<Spec::U>]>>,
    _spec: PhantomData<Spec>,
}

impl<Spec: LazySegTreeSpec> LazySegTree<Spec> {
    // ===== CONSTRUCTORS =====

    fn size_to_max_size_and_depth(size: usize) -> (usize, u32) {
        if size == 0 {
            panic!("LazySegTree must have a positive size");
        }
        let max_size = size.next_power_of_two();
        let max_depth = max_size.trailing_zeros();
        (max_size, max_depth)
    }

    pub fn new(size: usize) -> Self {
        let (max_size, max_depth) = Self::size_to_max_size_and_depth(size);
        Self {
            size,
            max_size,
            max_depth,
            data: RefCell::new(vec![Spec::ID; max_size * 2].into_boxed_slice()),
            tags: RefCell::new(vec![None; max_size * 2].into_boxed_slice()),
            _spec: PhantomData,
        }
    }

    pub fn from_slice(values: &[Spec::T]) -> Self {
        let size = values.len();
        let (max_size, max_depth) = Self::size_to_max_size_and_depth(size);
        let mut data = vec![Spec::ID; max_size * 2];

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
            max_depth,
            max_size,
            data: RefCell::new(data.into_boxed_slice()),
            tags: RefCell::new(vec![None; max_size * 2].into_boxed_slice()),
            _spec: PhantomData,
        }
    }

    pub fn from_vec(values: Vec<Spec::T>) -> Self {
        let size = values.len();
        let (max_size, max_depth) = Self::size_to_max_size_and_depth(size);
        let mut data = vec![Spec::ID; max_size * 2];

        if size > 0 {
            for (i, v) in values.into_iter().enumerate() {
                data[max_size + i] = v;
            }
            for i in (1..max_size).rev() {
                let mut v = data[i * 2].clone();
                Spec::op_on_data(&mut v, &data[i * 2 + 1]);
                data[i] = v;
            }
        }

        Self {
            size,
            max_size,
            max_depth,
            data: RefCell::new(data.into_boxed_slice()),
            tags: RefCell::new(vec![None; max_size * 2].into_boxed_slice()),
            _spec: PhantomData,
        }
    }

    // ===== PUBLIC INTERFACE =====

    pub fn query<R: RangeBounds<usize>>(&self, range: R) -> Spec::T {
        let (left_inp, right_inp) = utils::parse_range(range, self.size);
        utils::validate_range(left_inp, right_inp, self.size);
        if left_inp == right_inp {
            return Spec::ID;
        }

        let mut l = self.max_size + left_inp;
        let mut r = self.max_size + right_inp;

        for i in (1..=self.max_depth).rev() {
            // Checks if the node is not a left bound
            if ((l >> i) << i) != l {
                self.push_node(SegTreeNode(l >> i));
            }
            if ((r >> i) << i) != r {
                self.push_node(SegTreeNode((r - 1) >> i));
            }
        }

        let mut res = Spec::ID;

        while l < r {
            if l & 1 != 0 {
                Spec::op_on_data(&mut res, &self.eval(SegTreeNode(l)));
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                Spec::op_on_data(&mut res, &self.eval(SegTreeNode(r)));
            }
            l >>= 1;
            r >>= 1;
        }

        res
    }

    pub fn update<R: RangeBounds<usize>>(&mut self, range: R, value: Spec::U) {
        let (left_inp, right_inp) = utils::parse_range(range, self.size);
        utils::validate_range(left_inp, right_inp, self.size);
        if left_inp == right_inp {
            return;
        }

        let mut l = self.max_size + left_inp;
        let mut r = self.max_size + right_inp;

        for i in (1..=self.max_depth).rev() {
            if ((l >> i) << i) != l {
                self.push_node_mut(SegTreeNode(l >> i));
            }
            if ((r >> i) << i) != r {
                self.push_node_mut(SegTreeNode((r - 1) >> i));
            }
        }

        let l0 = l;
        let r0 = r;

        while l < r {
            if l & 1 != 0 {
                Self::combine_tag_option(&mut self.tags.get_mut()[l], &value);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                Self::combine_tag_option(&mut self.tags.get_mut()[r], &value);
            }
            l >>= 1;
            r >>= 1;
        }

        for i in 1..=self.max_depth {
            if ((l0 >> i) << i) != l0 {
                self.pull_node(SegTreeNode(l0 >> i));
            }
            if ((r0 >> i) << i) != r0 {
                self.pull_node(SegTreeNode((r0 - 1) >> i));
            }
        }
    }

    // ===== PRIVATE HELPER METHODS =====

    fn pull_node(&mut self, node: SegTreeNode) {
        if node.is_leaf(self.max_depth) {
            return;
        }
        let mut res = self.eval_mut(node.left_child());
        let right_val = self.eval_mut(node.right_child());
        Spec::op_on_data(&mut res, &right_val);
        self.data.get_mut()[node.0] = res;
    }

    fn eval(&self, node: SegTreeNode) -> Spec::T {
        let data = self.data.borrow();
        let tags = self.tags.borrow();
        let mut d = data[node.0].clone();
        if let Some(tag) = &tags[node.0] {
            Spec::op_update_on_data(tag, &mut d, node.size(self.max_depth));
        }
        d
    }

    fn eval_mut(&mut self, node: SegTreeNode) -> Spec::T {
        let tag = self.tags.get_mut()[node.0].clone();
        let mut d = self.data.get_mut()[node.0].clone();
        if let Some(tag) = &tag {
            Spec::op_update_on_data(tag, &mut d, node.size(self.max_depth));
        }
        d
    }

    /// Pushes the tag of the current node to its children after consuming it.
    #[inline]
    fn push_node(&self, node: SegTreeNode) {
        let mut tags = self.tags.borrow_mut();
        if let Some(tag) = tags[node.0].take() {
            let mut data = self.data.borrow_mut();
            Spec::op_update_on_data(&tag, &mut data[node.0], node.size(self.max_depth));
            if !node.is_leaf(self.max_depth) {
                Self::combine_tag_option(&mut tags[node.left_child().0], &tag);
                Self::combine_tag_option(&mut tags[node.right_child().0], &tag);
            }
        }
    }

    #[inline]
    fn push_node_mut(&mut self, node: SegTreeNode) {
        if let Some(tag) = self.tags.get_mut()[node.0].take() {
            let node_size = node.size(self.max_depth);
            Spec::op_update_on_data(&tag, &mut self.data.get_mut()[node.0], node_size);
            if !node.is_leaf(self.max_depth) {
                let left_child_idx = node.left_child().0;
                let right_child_idx = node.right_child().0;
                let tags = self.tags.get_mut();
                Self::combine_tag_option(&mut tags[left_child_idx], &tag);
                Self::combine_tag_option(&mut tags[right_child_idx], &tag);
            }
        }
    }

    #[inline]
    fn combine_tag_option(existing_tag: &mut Option<Spec::U>, new_tag: &Spec::U) {
        if let Some(existing) = existing_tag {
            Spec::op_on_update(existing, new_tag);
        } else {
            *existing_tag = Some(new_tag.clone());
        }
    }
}

// ===== DISPLAY IMPLEMENTATION =====

fn print_tree_option<T: Display>(
    f: &mut std::fmt::Formatter<'_>,
    tree: &[&Option<T>],
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
        writeln!(f, "LazySegTree {{")?;
        writeln!(f, "  Data Type: {}", std::any::type_name::<Spec::T>())?;
        writeln!(f, "  Update Type: {}", std::any::type_name::<Spec::U>())?;
        writeln!(f, "  Size: {} (Internal: {})", self.size, self.max_size)?;

        let data = self.data.borrow();
        let tags = self.tags.borrow();

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
        let data_values_slice = data_values.iter().collect::<Vec<_>>();
        let tag_values_slice = tags.iter().collect::<Vec<_>>();

        writeln!(f, "  Data:")?;
        print_tree_option(f, &data_values_slice, 1, 2, 0, self.max_size)?;

        writeln!(f, "  Lazy Tags:")?;
        print_tree_option(f, &tag_values_slice, 1, 2, 0, self.max_size)?;

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
        // Add 5 to first 6 elements
        expected.iter_mut().take(6).for_each(|v| *v += 5);
        // Add 10 to indices 4..8 -> skip first 4, take next 4
        expected.iter_mut().skip(4).take(4).for_each(|v| *v += 10);
        // Add -2 to indices 2..5 -> skip first 2, take next 3
        expected.iter_mut().skip(2).take(3).for_each(|v| *v += -2);

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
