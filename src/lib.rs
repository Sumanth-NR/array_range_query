//! array_range_query: See full docs at <https://docs.rs/array_range_query> or in [README.md](../README.md)
#![doc = include_str!("../README.md")]

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
