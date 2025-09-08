//! # array_range_query
//!
//! A high-performance, generic implementation of segment trees and lazy segment trees in Rust
//! for efficient range queries and range updates.
//!
//! This crate provides two main data structures:
//! - [`SegTree`]: A generic segment tree for point updates and range queries
//! - [`LazySegTree`]: A generic lazy segment tree for range updates and range queries
//!
//! Both are highly customizable through trait-based specifications, allowing you to define
//! custom operations while maintaining type safety and zero-cost abstractions.
//!
//! ## Features
//!
//! - **Generic Design**: Works with any type that implements the required traits
//! - **High Performance**: O(log n) operations with lazy propagation for range updates
//! - **Type Safety**: Compile-time guarantees for operation correctness
//! - **Helper Types**: Pre-built implementations for common operations (sum, min, max)
//! - **Comprehensive**: Supports both point updates and range updates with range queries
//!
//! ## Quick Start
//!
//! ### Basic Segment Tree (Point Updates)
//!
//! ```rust
//! use array_range_query::{SegTree, SegTreeSpec};
//!
//! // Define a specification for sum operations
//! struct SumSpec;
//! impl SegTreeSpec for SumSpec {
//!     type T = i64;
//!     const ID: Self::T = 0;
//!     fn op(a: &Self::T, b: &Self::T) -> Self::T { a + b }
//! }
//!
//! let values = vec![1, 2, 3, 4, 5];
//! let mut tree = SegTree::<SumSpec>::from_vec(&values);
//!
//! assert_eq!(tree.query(0, 5), 15); // Sum of all elements
//! assert_eq!(tree.query(1, 4), 9);  // Sum of elements 1, 2, 3
//!
//! tree.update(2, 10); // Change element at index 2 to 10
//! assert_eq!(tree.query(0, 5), 22); // Updated sum
//! ```
//!
//! ### Using Helper Types
//!
//! ```rust
//! use array_range_query::{SegTreeSum, SegTreeMin, SegTreeMax};
//!
//! let values = vec![3, 1, 4, 1, 5];
//!
//! let sum_tree = SegTreeSum::<i32>::from_vec(&values);
//! assert_eq!(sum_tree.query(0, 5), 14);
//!
//! let min_tree = SegTreeMin::<i32>::from_vec(&values);
//! assert_eq!(min_tree.query(1, 4), 1);
//!
//! let max_tree = SegTreeMax::<i32>::from_vec(&values);
//! assert_eq!(max_tree.query(0, 3), 4);
//! ```
//!
//! ### Lazy Segment Tree (Range Updates)
//!
//! ```rust
//! use array_range_query::{LazySegTree, LazySegTreeSpec};
//!
//! // Define a specification for range add + range sum
//! struct RangeAddSum;
//! impl LazySegTreeSpec for RangeAddSum {
//!     type T = i64;
//!     type U = i64;
//!     const ID: Self::T = 0;
//!     fn op_on_data(d1: &Self::T, d2: &Self::T) -> Self::T { d1 + d2 }
//!     fn op_on_update(u1: &Self::U, u2: &Self::U) -> Self::U { u1 + u2 }
//!     fn op_update_on_data(u: &Self::U, d: &Self::T, size: usize) -> Self::T {
//!         d + (u * size as i64)
//!     }
//! }
//!
//! let values = vec![1, 2, 3, 4, 5];
//! let mut tree = LazySegTree::<RangeAddSum>::from_vec(&values);
//!
//! tree.update(1, 4, 10); // Add 10 to elements 1, 2, 3
//! assert_eq!(tree.query(0, 5), 45); // 1 + 12 + 13 + 14 + 5
//! ```
//!
//! ### Lazy Segment Tree Helpers
//!
//! ```rust
//! use array_range_query::{LazySegTreeAddSum, LazySegTreeAddMax};
//!
//! let values = vec![1, 2, 3, 4, 5];
//!
//! // Range add + range sum
//! let mut sum_tree = LazySegTreeAddSum::<i64>::from_vec(&values);
//! sum_tree.update(1, 4, 10);
//! assert_eq!(sum_tree.query(0, 5), 45);
//!
//! // Range add + range max
//! let mut max_tree = LazySegTreeAddMax::<i64>::from_vec(&values);
//! max_tree.update(0, 3, 10);
//! assert_eq!(max_tree.query(0, 5), 13); // max(11, 12, 13, 4, 5)
//! ```
//!
//! ## Performance
//!
//! All operations have optimal time complexity:
//! - Construction: O(n)
//! - Point update (SegTree): O(log n)
//! - Range query: O(log n)
//! - Range update (LazySegTree): O(log n)
//!
//! Space complexity: O(n)
//!
//! ## Design Philosophy
//!
//! This crate follows Rust's zero-cost abstraction principle. The generic design allows
//! for maximum flexibility while maintaining compile-time optimizations. The trait-based
//! approach ensures type safety and prevents common errors like using incompatible
//! operations or identity elements.

pub mod helpers;
mod lazy_seg_tree;
mod seg_tree;

pub use helpers::{
    LazySegTreeAddMax, LazySegTreeAddMaxSpec, LazySegTreeAddSum, LazySegTreeAddSumSpec, SegTreeMax,
    SegTreeMaxSpec, SegTreeMin, SegTreeMinSpec, SegTreeSum, SegTreeSumSpec,
};
pub use lazy_seg_tree::{LazySegTree, LazySegTreeSpec};
pub use seg_tree::{SegTree, SegTreeSpec};
