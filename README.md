# array_range_query

[![Crates.io](https://img.shields.io/crates/v/array_range_query.svg)](https://crates.io/crates/array_range_query)
[![Documentation](https://docs.rs/array_range_query/badge.svg)](https://docs.rs/array_range_query)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](#license)

**A high-performance, generic Rust implementation of Segment Trees and Lazy Segment Trees for efficient range queries, range updates, and interval operations.**

Perfect for competitive programming, algorithm optimization, and solving range query problems with O(log n) time complexity.

## Features

- **Segment Tree**: O(log n) point updates and range queries for any associative operation
- **Lazy Segment Tree**: O(log n) range updates and range queries with lazy propagation
- **Generic Design**: Type-safe specifications for custom operations
- **Helper Types**: Pre-built implementations for sum, min, max operations
- **Zero-cost Abstractions**: No runtime overhead from generics

## What is a Segment Tree?

A **Segment Tree** is a versatile data structure that allows you to:
- Answer **range queries** (like sum, min, max, GCD) on an array in **O(log n)** time
- Perform **point updates** (modify a single element) in **O(log n)** time
- Handle **any associative operation** (operations where `(a ⊕ b) ⊕ c = a ⊕ (b ⊕ c)`)

A **Lazy Segment Tree** extends this capability to efficiently handle:
- **Range updates** (modify multiple elements at once) in **O(log n)** time
- **Lazy propagation** to defer updates until necessary, significantly improving performance

### Why Use Segment Trees?

Segment trees are essential for solving interval query problems efficiently:

| Operation | Naive Approach | Segment Tree | Improvement |
|-----------|---------------|--------------|-------------|
| Range Query | O(n) | O(log n) | Exponential speedup |
| Point Update | O(1) | O(log n) | Slightly slower |
| Range Update | O(n) | O(log n) with lazy | Exponential speedup |

### Common Use Cases

- **Competitive Programming**: Solving range query problems (Codeforces, LeetCode, AtCoder)
- **Database Systems**: Interval queries and aggregate functions
- **Game Development**: Collision detection, spatial queries
- **Financial Analysis**: Time-series analysis, moving window calculations
- **Computer Graphics**: Rectangle queries, spatial indexing
- **Network Monitoring**: Bandwidth tracking, latency analysis

### Comparison with Other Data Structures

| Data Structure | Range Query | Point Update | Range Update | Best For |
|----------------|-------------|--------------|--------------|----------|
| Array | O(n) | O(1) | O(n) | Simple access |
| Prefix Sum | O(1) | O(n) | O(n) | Immutable sum queries |
| Segment Tree | O(log n) | O(log n) | O(n) | Dynamic range queries |
| Lazy Segment Tree | O(log n) | O(log n) | O(log n) | **Range updates** |
| Fenwick Tree (BIT) | O(log n) | O(log n) | - | Sum/XOR only |
| Sparse Table | O(1) | - | - | Immutable idempotent ops |

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

## Solving Classic Problems

### Problem 1: Range Sum Queries with Point Updates
**Example**: Given an array, answer queries for the sum of elements in any range and support updating individual elements.

```rust
use array_range_query::SegTreeSum;

let mut tree = SegTreeSum::<i32>::from_vec(vec![1, 3, 5, 7, 9, 11]);
// What's the sum from index 1 to 4?
assert_eq!(tree.query(1..4), 15); // 3 + 5 + 7
// Update index 2 to 10
tree.update(2, 10);
assert_eq!(tree.query(1..4), 20); // 3 + 10 + 7
```

### Problem 2: Range Minimum Queries (RMQ)
**Example**: Find the minimum element in any subarray efficiently.

```rust
use array_range_query::SegTreeMin;

let tree = SegTreeMin::<i32>::from_vec(vec![4, 2, 8, 1, 9, 3, 6]);
assert_eq!(tree.query(1..5), 1); // min(2, 8, 1, 9) = 1
assert_eq!(tree.query(0..3), 2); // min(4, 2, 8) = 2
```

### Problem 3: Range Updates with Range Queries
**Example**: Add a value to all elements in a range, then query range sums.

```rust
use array_range_query::LazySegTreeAddSum;

let mut tree = LazySegTreeAddSum::<i64>::from_vec(vec![1, 2, 3, 4, 5]);
// Add 10 to elements from index 1 to 3
tree.update(1..4, 10);
// Elements are now [1, 12, 13, 14, 5]
assert_eq!(tree.query(0..5), 45); // sum = 1+12+13+14+5
```

### Problem 4: Range Assignment
**Example**: Set all elements in a range to a specific value.

```rust
use array_range_query::LazySegTreeReplaceSum;

let mut tree = LazySegTreeReplaceSum::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
// Set all elements from index 1 to 4 to value 10
tree.update(1..4, 10);
// Elements are now [1, 10, 10, 10, 5]
assert_eq!(tree.query(..), 36); // sum = 1+10+10+10+5
```

## Performance

- **Construction**: O(n)
- **Point update**: O(log n)
- **Range query**: O(log n)
- **Range update** (lazy): O(log n)
- **Space**: O(n)

## Advanced Topics

### Implementing Custom Operations

This library supports any **monoid operation** (associative operation with an identity element). Common examples:
- **Sum** (identity: 0)
- **Product** (identity: 1)
- **Min** (identity: ∞)
- **Max** (identity: -∞)
- **GCD** (identity: 0)
- **Bitwise OR** (identity: 0)
- **Bitwise AND** (identity: all 1s)

Example of implementing GCD:
```rust
use array_range_query::{SegTree, SegTreeSpec};

struct GcdSpec;
impl SegTreeSpec for GcdSpec {
    type T = u32;
    const ID: Self::T = 0;
    fn op(a: &mut Self::T, b: &Self::T) {
        *a = gcd(*a, *b);
    }
}

fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 { a } else { gcd(b, a % b) }
}

let tree = SegTree::<GcdSpec>::from_vec(vec![12, 18, 24, 30]);
assert_eq!(tree.query(..), 6); // GCD of all elements
```

### When to Use Lazy Propagation

Use **Lazy Segment Trees** when you need:
1. **Range updates**: Modifying multiple elements efficiently
2. **Batch operations**: Applying the same operation to intervals
3. **Deferred computation**: Delaying updates until queries require them

Examples: Range increment, range assignment, range multiplication.

### Segment Tree vs. Other Interval Structures

**Fenwick Tree (Binary Indexed Tree)**:
- Simpler implementation, less memory
- Only works for invertible operations (sum, XOR)
- Can't handle min/max queries
- No range updates without tricks

**Sparse Table**:
- O(1) query time for idempotent operations
- Immutable (no updates)
- Higher space complexity O(n log n)
- Best for static range minimum/maximum queries

**Segment Tree (this library)**:
- Supports any associative operation
- Handles both queries and updates
- Works with lazy propagation for range updates
- Most versatile for competitive programming

## Requirements

- Rust 2021 edition
- Dependencies (for helpers): `num-traits`, `min_max_traits`

## Related Topics & Keywords

This library is relevant for:
- **Data Structures**: Segment Tree, Interval Tree, Binary Indexed Tree (BIT), Fenwick Tree, Range Tree
- **Algorithms**: Divide and Conquer, Lazy Propagation, Range Queries, Interval Operations
- **Problem Types**: Range Minimum Query (RMQ), Range Sum Query (RSQ), Range Maximum Query, Range GCD Query
- **Applications**: Competitive Programming, Algorithm Contests, Interval Scheduling, Computational Geometry
- **Concepts**: Monoid Operations, Associative Operations, Binary Trees, Tree-based Data Structures
- **Performance**: O(log n) queries, O(log n) updates, Efficient range operations

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Learn More

- [Segment Tree Tutorial (CP-Algorithms)](https://cp-algorithms.com/data_structures/segment_tree.html)
- [Lazy Propagation Explained](https://cp-algorithms.com/data_structures/segment_tree.html#range-updates-lazy-propagation)
- [Range Query Problems on LeetCode](https://leetcode.com/tag/segment-tree/)

## License

MIT License. See [LICENSE](https://github.com/Sumanth-NR/array_range_query/blob/main/LICENSE) for details.
