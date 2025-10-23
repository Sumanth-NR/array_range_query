//! High-performance segment trees and lazy segment trees for efficient range queries and updates.
//!
//! # Quick Start
//!
//! ```rust
//! use array_range_query::{SegTreeSum, LazySegTreeAddSum};
//!
//! // Segment tree for range sum queries
//! let mut tree = SegTreeSum::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
//! assert_eq!(tree.query(1..4), 9); // sum of [2, 3, 4]
//! tree.update(2, 10);
//! assert_eq!(tree.query(..), 22);
//!
//! // Lazy segment tree for range add updates and sum queries
//! let mut lazy_tree = LazySegTreeAddSum::<i32>::from_vec(vec![1, 2, 3, 4, 5]);
//! lazy_tree.update(1..4, 10); // add 10 to range [1, 4)
//! assert_eq!(lazy_tree.query(..), 45);
//! ```
//!
//! For detailed documentation, examples, and use cases, see the [README](https://github.com/Sumanth-NR/array_range_query#readme).

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
