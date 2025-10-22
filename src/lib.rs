//! # array_range_query: High-Performance Segment Trees for Range Queries
//!
//! **Complete Rust implementation of Segment Trees and Lazy Segment Trees for efficient 
//! range queries, range updates, and interval operations.**
//!
//! This crate provides generic, type-safe implementations of segment trees that work with
//! any associative operation (monoid). Perfect for competitive programming, algorithm optimization,
//! and solving complex range query problems with O(log n) time complexity.
//!
//! ## What is a Segment Tree?
//!
//! A **Segment Tree** is a tree-based data structure designed for storing intervals or segments.
//! It enables:
//! - **Range Queries**: Answer queries like sum, min, max, GCD over any array range in O(log n)
//! - **Point Updates**: Modify individual elements efficiently in O(log n)
//! - **Generic Operations**: Support any associative operation with an identity element
//!
//! A **Lazy Segment Tree** adds lazy propagation for:
//! - **Range Updates**: Modify entire ranges efficiently in O(log n)
//! - **Deferred Computation**: Only propagate updates when necessary
//! - **Batch Operations**: Apply the same operation to intervals
//!
//! ## Common Use Cases
//!
//! - **Competitive Programming**: Solving interval query problems on platforms like Codeforces, 
//!   AtCoder, LeetCode, HackerRank
//! - **Range Minimum/Maximum Query (RMQ)**: Find min/max in any subarray
//! - **Range Sum Query (RSQ)**: Calculate sums over arbitrary intervals
//! - **Dynamic Arrays**: Arrays that support both queries and updates
//! - **Computational Geometry**: Rectangle queries, intersection detection
//! - **Database Systems**: Aggregate queries over ranges
//!
//! ## Performance Characteristics
//!
//! | Operation | Segment Tree | Lazy Segment Tree | Array (naive) |
//! |-----------|--------------|-------------------|---------------|
//! | Build | O(n) | O(n) | O(1) |
//! | Point Update | O(log n) | O(log n) | O(1) |
//! | Range Query | O(log n) | O(log n) | O(n) |
//! | Range Update | O(n) | O(log n) | O(n) |
//!
//! ## Quick Examples
//!
//! ```rust
//! use array_range_query::{SegTreeSum, LazySegTreeAddSum};
//!
//! // Segment tree for range sum queries with point updates
//! let mut tree = SegTreeSum::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
//! assert_eq!(tree.query(1..4), 9); // sum of [2, 3, 4]
//! tree.update(2, 10);              // update index 2 to 10
//! assert_eq!(tree.query(..), 22);  // sum of [1, 2, 10, 4, 5]
//!
//! // Lazy segment tree for range add updates with range sum queries
//! let mut lazy_tree = LazySegTreeAddSum::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
//! lazy_tree.update(1..4, 10);      // add 10 to range [1, 4)
//! assert_eq!(lazy_tree.query(..), 45); // sum = 1+12+13+14+5
//! ```
//!
//! ## Core Types
//!
//! ### Segment Trees (Point Updates, Range Queries)
//!
//! - [`SegTree<Spec>`]: Generic segment tree with custom operations
//! - [`SegTreeSum<T>`]: Pre-configured for range sum queries
//! - [`SegTreeMin<T>`]: Pre-configured for range minimum queries  
//! - [`SegTreeMax<T>`]: Pre-configured for range maximum queries
//!
//! ### Lazy Segment Trees (Range Updates, Range Queries)
//!
//! - [`LazySegTree<Spec>`]: Generic lazy segment tree with custom operations
//! - [`LazySegTreeAddSum<T>`]: Range add updates with sum queries
//! - [`LazySegTreeAddMin<T>`]: Range add updates with min queries
//! - [`LazySegTreeAddMax<T>`]: Range add updates with max queries
//! - [`LazySegTreeReplaceSum<T>`]: Range assignment updates with sum queries
//!
//! ## Implementing Custom Operations
//!
//! Define your own operations by implementing [`SegTreeSpec`] for regular segment trees
//! or [`LazySegTreeSpec`] for lazy segment trees:
//!
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! // Custom segment tree for maximum values
//! struct MaxSpec;
//! impl SegTreeSpec for MaxSpec {
//!     type T = i32;
//!     const ID: Self::T = i32::MIN; // identity element
//!     fn op(a: &mut Self::T, b: &Self::T) { 
//!         *a = (*a).max(*b); // associative operation
//!     }
//! }
//!
//! let tree = SegTree::<MaxSpec>::from_vec(vec![3, 1, 4, 1, 5, 9]);
//! assert_eq!(tree.query(..), 9); // maximum element
//! assert_eq!(tree.query(0..3), 4); // max of first 3 elements
//! ```
//!
//! ## Why Choose This Library?
//!
//! - **Generic & Type-Safe**: Works with any type and operation
//! - **Zero-Cost Abstractions**: No runtime overhead from generics
//! - **Flexible API**: Accepts any range type (`..`, `a..b`, `a..=b`, etc.)
//! - **Well-Tested**: Comprehensive test suite
//! - **Fast**: Optimized implementation with lazy propagation
//! - **Educational**: Clear code structure for learning segment trees
//!
//! ## Related Concepts
//!
//! This library implements:
//! - **Segment Trees**: Binary tree structure for interval queries
//! - **Lazy Propagation**: Deferred updates for range modifications
//! - **Monoid Operations**: Associative operations with identity elements
//! - **Range Query Data Structures**: Efficient interval operations
//! - **Divide and Conquer**: Tree-based problem decomposition
//!
//! Alternative data structures for different use cases:
//! - **Fenwick Tree (BIT)**: Simpler, but only for sum/XOR queries
//! - **Sparse Table**: O(1) queries for static, idempotent operations
//! - **Square Root Decomposition**: Simpler implementation, O(√n) queries
//!
//! For comprehensive documentation, see <https://docs.rs/array_range_query>

pub(crate) mod utils;

mod seg_tree_node;
pub use seg_tree_node::SegTreeNode;

mod seg_tree;
pub use seg_tree::{SegTree, SegTreeSpec};

mod lazy_seg_tree;
pub use lazy_seg_tree::{LazySegTree, LazySegTreeSpec};

pub mod helpers;
pub use helpers::{LazySegTreeAddMax, LazySegTreeAddMin, LazySegTreeAddSum, LazySegTreeReplaceSum};
pub use helpers::{SegTreeMax, SegTreeMin, SegTreeSum};
