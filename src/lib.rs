//! array_range_query: See full docs at https://docs.rs/array_range_query or in README.md
#![doc = include_str!("../README.md")]

pub mod helpers;
mod lazy_seg_tree;
mod seg_tree;

pub use helpers::{LazySegTreeAddMax, LazySegTreeAddMin, LazySegTreeAddSum, LazySegTreeReplaceSum};
pub use helpers::{SegTreeMax, SegTreeMin, SegTreeSum};
pub use lazy_seg_tree::{LazySegTree, LazySegTreeSpec};
pub use seg_tree::{SegTree, SegTreeSpec};
