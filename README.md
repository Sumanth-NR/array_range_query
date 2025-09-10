# array_range_query

[![Crates.io](https://img.shields.io/crates/v/array_range_query.svg)](https://crates.io/crates/array_range_query)
[![Documentation](https://docs.rs/array_range_query/badge.svg)](https://docs.rs/array_range_query)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

A high-performance, generic implementation of segment trees and lazy segment trees in Rust for efficient range queries and range updates.

Includes helpers for:
- Range sum, min, max queries
- Range add, range assignment (replace), and lazy propagation for sum/min/max

## Features

- **Generic Segment Tree (`SegTree`)**: Supports any associative operation (monoid) with O(log n) point updates and range queries
- **Generic Lazy Segment Tree (`LazySegTree`)**: Supports range updates and range queries in O(log n) time with lazy propagation
- **Type-safe design**: Uses the type system to ensure correctness and prevent misuse
- **Comprehensive helper types**: Pre-built implementations for common operations (sum, min, max)
- **Zero-cost abstractions**: Generic design with no runtime overhead
- **Well-tested**: Extensive test coverage including edge cases

## Quick Start

Use `cargo add` (from the `cargo-edit` tool) to add the crate and automatically insert the correct version into your `Cargo.toml`:

```bash
cargo add array_range_query
```

If you prefer to pin a specific version, you can specify it when adding:

```bash
cargo add array_range_query@0.1.2
```

## Basic Usage

### Segment Tree for Range Sum Queries

```rust
use array_range_query::{SegTree, SegTreeSpec};

// Define a spec for sum operations
struct SumSpec;

impl SegTreeSpec for SumSpec {
    type T = i64;
    const ID: Self::T = 0; // Identity element for addition

    fn op(a: &mut Self::T, b: &Self::T) {
        *a += *b;
    }
}

let values = vec![1, 2, 3, 4, 5];
let mut seg_tree = SegTree::<SumSpec>::from_vec(values);

// Query sum of range [1, 4) -> elements at indices 1, 2, 3
assert_eq!(seg_tree.query(1..4), 9); // 2 + 3 + 4

// Update element at index 2 to 10
seg_tree.update(2, 10);

// Query again - sum should reflect the change
assert_eq!(seg_tree.query(1..4), 16); // 2 + 10 + 4
assert_eq!(seg_tree.query(..), 22); // 1 + 2 + 10 + 4 + 5
```

### Using Helper Types

For common operations, you can use the provided helper types:

```rust
use array_range_query::{SegTreeSum, SegTreeMin, SegTreeMax};

let values = vec![3, 1, 4, 1, 5, 9, 2, 6];

// Range sum queries
let mut sum_tree = SegTreeSum::<i32>::from_slice(&values);
assert_eq!(sum_tree.query(2..6), 19); // 4 + 1 + 5 + 9

// Range minimum queries
let mut min_tree = SegTreeMin::<i32>::from_slice(&values);
assert_eq!(min_tree.query(2..6), 1); // min(4, 1, 5, 9)

// Range maximum queries
let mut max_tree = SegTreeMax::<i32>::from_slice(&values);
assert_eq!(max_tree.query(2..6), 9); // max(4, 1, 5, 9)
```

### Lazy Segment Tree for Range Updates

```rust
use array_range_query::{LazySegTree, LazySegTreeSpec};

// Define a spec for range add + range sum
struct RangeAddSum;

impl LazySegTreeSpec for RangeAddSum {
    type T = i64; // Data type (sum values)
    type U = i64; // Update type (add values)
    const ID: Self::T = 0;

    // Combine two data values, performed in-place
    fn op_on_data(d1: &mut Self::T, d2: &Self::T) {
        *d1 += *d2;
    }

    // Compose two updates, performed in-place
    fn op_on_update(u1: &mut Self::U, u2: &Self::U) {
        *u1 += *u2;
    }

    // Apply update to data (accounting for range size), performed in-place
    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
        *d += u * size as i64;
    }
}

let mut lazy_tree = LazySegTree::<RangeAddSum>::from_vec(vec![1, 2, 3, 4, 5]);

// Initial sum of range [1, 4)
assert_eq!(lazy_tree.query(1..4), 9); // 2 + 3 + 4

// Add 10 to all elements in range [1, 4)
lazy_tree.update(1..4, 10);

// Query the updated range
assert_eq!(lazy_tree.query(1..4), 39); // (2+10) + (3+10) + (4+10)

// Total sum should be updated too
assert_eq!(lazy_tree.query(..), 45); // 1 + 12 + 13 + 14 + 5
```

### Using Lazy Segment Tree Helpers

```rust
use array_range_query::{LazySegTreeAddSum, LazySegTreeAddMin, LazySegTreeReplaceSum};

 // Range add + range sum
let values = vec![1, 2, 3, 4, 5];
let mut tree = LazySegTreeAddSum::<i64>::from_vec(values);
tree.update(1..3, 5);
assert_eq!(tree.query(0..2), 8);  // 1 + (2+5)
assert_eq!(tree.query(1..4), 19); // 7 + 8 + 4 = 19

// Range add + range min
let values = vec![5, 2, 8, 1, 9, 3];
let mut min_tree = LazySegTreeAddMin::<i32>::from_vec(values);
min_tree.update(1..4, 2);
assert_eq!(min_tree.query(..), 3); // min(5, 4, 10, 3, 9, 3)

// Range assignment (replace) + range sum
let values = vec![1, 2, 3, 4, 5];
let mut replace_tree = LazySegTreeReplaceSum::<i32>::from_vec(values);
replace_tree.update(1..4, 10); // Replace [1, 4) with 10
assert_eq!(replace_tree.query(..), 36); // 1 + 10 + 10 + 10 + 5
```

## Advanced Usage

### Custom Data Types

You can use segment trees with any type that implements the required traits:

```rust
use array_range_query::{SegTree, SegTreeSpec};

#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

struct PointMaxSpec;

impl SegTreeSpec for PointMaxSpec {
    type T = Point;
    const ID: Self::T = Point { x: i32::MIN, y: i32::MIN };

    fn op(a: &mut Self::T, b: &Self::T) {
        a.x = a.x.max(b.x);
        a.y = a.y.max(b.y);
    }
}

let points = vec![
    Point { x: 1, y: 2 },
    Point { x: 3, y: 1 },
    Point { x: 2, y: 4 },
];

let tree = SegTree::<PointMaxSpec>::from_vec(points);
let max_point = tree.query(..);
assert_eq!(max_point.x, 3);
assert_eq!(max_point.y, 4);
```

## API Reference

### SegTree

- `SegTree::new(size: usize)` — Create empty tree with given size.
- `SegTree::from_slice(values: &[T])` — Build tree from slice of values.
- `SegTree::from_vec(values: Vec<T>)` — Build tree from owned vector of values.
- `query(&self, range: impl RangeBounds<usize>) -> T` — Query a range (half-open, e.g. `2..5`).
- `update(&mut self, index: usize, value: T)` — Update a single element.

#### Trait: `SegTreeSpec`
```rust
trait SegTreeSpec {
    type T: Clone;
    const ID: Self::T;
    fn op(a: &mut Self::T, b: &Self::T);
}
```

### LazySegTree

- `LazySegTree::new(size: usize)` — Create empty lazy tree.
- `LazySegTree::from_slice(values: &[T])` — Build from slice of values.
- `LazySegTree::from_vec(values: Vec<T>)` — Build from owned vector of values.
- `query(&self, range: impl RangeBounds<usize>) -> T` — Query a range (half-open).
- `update(&mut self, range: impl RangeBounds<usize>, value: U)` — Update a range.

#### Trait: `LazySegTreeSpec`
```rust
trait LazySegTreeSpec {
    type T: Clone;
    type U: Clone;
    const ID: Self::T;
    fn op_on_data(d1: &mut Self::T, d2: &Self::T);
    fn op_on_update(u1: &mut Self::U, u2: &Self::U);
    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize);
}
```

### Helper Types

**Regular Segment Trees:**
- `SegTreeSum<T>` — Range sum queries
- `SegTreeMin<T>` — Range minimum queries
- `SegTreeMax<T>` — Range maximum queries

**Lazy Segment Trees:**
- `LazySegTreeAddSum<T>` — Range add updates, range sum queries
- `LazySegTreeAddMax<T>` — Range add updates, range max queries
- `LazySegTreeAddMin<T>` — Range add updates, range min queries
- `LazySegTreeReplaceSum<T>` — Range assignment (replace) updates, range sum queries

## Performance

All operations have the following time complexities:
- **Construction**: O(n) for `from_vec`/`from_slice`, O(n) for `new`
- **Point update** (SegTree): O(log n)
- **Range query**: O(log n)
- **Range update** (LazySegTree): O(log n)

Space complexity: O(n)

## Requirements

- Rust 2021 edition or later
- For numeric types: `num-traits` crate features
- For min/max operations: `min_max_traits` crate features

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/Sumanth-NR/array_range_query/blob/main/LICENSE) file for details.
