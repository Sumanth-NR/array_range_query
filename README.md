# array_range_query

[![Crates.io](https://img.shields.io/crates/v/array_range_query.svg)](https://crates.io/crates/array_range_query)
[![Documentation](https://docs.rs/array_range_query/badge.svg)](https://docs.rs/array_range_query)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

A high-performance, generic implementation of segment trees and lazy segment trees in Rust for efficient range queries and range updates.

## Features

- **Segment Tree**: O(log n) point updates and range queries for any associative operation
- **Lazy Segment Tree**: O(log n) range updates and range queries with lazy propagation
- **Generic Design**: Type-safe specifications for custom operations
- **Helper Types**: Pre-built implementations for sum, min, max operations
- **Zero-cost Abstractions**: No runtime overhead from generics

## Installation

```bash
cargo add array_range_query
```

## Quick Start

### Basic Segment Tree

```rust
use array_range_query::SegTreeSum;

let values = vec![1, 2, 3, 4, 5];
let mut tree = SegTreeSum::<i32>::from_vec(values);

assert_eq!(tree.query(1..4), 9); // Sum of [2, 3, 4]
tree.update(2, 10);              // Change index 2 to 10
assert_eq!(tree.query(..), 22);  // Total sum: 1+2+10+4+5
```

### Lazy Segment Tree

```rust
use array_range_query::LazySegTreeAddSum;

let values = vec![1, 2, 3, 4, 5];
let mut tree = LazySegTreeAddSum::<i32>::from_vec(values);

assert_eq!(tree.query(1..4), 9); // Sum of [2, 3, 4]
tree.update(1..4, 10);           // Add 10 to range [1, 4)
assert_eq!(tree.query(..), 45);  // New total: 1+12+13+14+5
```

## Helper Types

### Regular Segment Trees
- `SegTreeSum<T>` — Range sum queries
- `SegTreeMin<T>` — Range minimum queries
- `SegTreeMax<T>` — Range maximum queries

### Lazy Segment Trees
- `LazySegTreeAddSum<T>` — Range add updates, sum queries
- `LazySegTreeAddMin<T>` — Range add updates, min queries
- `LazySegTreeAddMax<T>` — Range add updates, max queries
- `LazySegTreeReplaceSum<T>` — Range assignment updates, sum queries

## Custom Operations

Define your own operations by implementing the specification traits:

```rust
use array_range_query::{SegTree, SegTreeSpec};

struct MaxSpec;
impl SegTreeSpec for MaxSpec {
    type T = i32;
    const ID: Self::T = i32::MIN;
    fn op(a: &mut Self::T, b: &Self::T) { *a = (*a).max(*b); }
}

let tree = SegTree::<MaxSpec>::from_vec(vec![3, 1, 4, 1, 5]);
assert_eq!(tree.query(..), 5); // Maximum element
```

For lazy segment trees:

```rust
use array_range_query::{LazySegTree, LazySegTreeSpec};

struct RangeAddSum;
impl LazySegTreeSpec for RangeAddSum {
    type T = i64; // Data type
    type U = i64; // Update type
    const ID: Self::T = 0;

    fn op_on_data(d1: &mut Self::T, d2: &Self::T) { *d1 += *d2; }
    fn op_on_update(u1: &mut Self::U, u2: &Self::U) { *u1 += *u2; }
    fn op_update_on_data(u: &Self::U, d: &mut Self::T, size: usize) {
        *d += u * size as i64;
    }
}

let mut tree = LazySegTree::<RangeAddSum>::from_vec(vec![1, 2, 3, 4, 5]);
tree.update(1..4, 10); // Add 10 to range [1, 4)
assert_eq!(tree.query(..), 45);
```

## API Reference

### SegTree
- `new(size)` / `from_slice(values)` / `from_vec(values)` — Construction
- `query(range)` — Range query in O(log n)
- `update(index, value)` — Point update in O(log n)

### LazySegTree
- `new(size)` / `from_slice(values)` / `from_vec(values)` — Construction
- `query(range)` — Range query in O(log n)
- `update(range, value)` — Range update in O(log n)

### Range Types
All `query` and `update` methods accept any range type:
- `2..5` (half-open)
- `2..=4` (inclusive)
- `..3` (from start)
- `2..` (to end)
- `..` (entire range)

## Performance

- **Construction**: O(n)
- **Point update**: O(log n)
- **Range query**: O(log n)
- **Range update** (lazy): O(log n)
- **Space**: O(n)

## Requirements

- Rust 2021 edition
- Dependencies (for helpers): `num-traits`, `min_max_traits`

## License

MIT License. See [LICENSE](https://github.com/Sumanth-NR/array_range_query/blob/main/LICENSE) for details.
