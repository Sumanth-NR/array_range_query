//! Helper types for common segment tree operations.
//!
//! This module provides pre-built specifications and type aliases for common
//! segment tree operations like sum, min, max queries, and range add operations.

mod seg_tree_max;
mod seg_tree_min;
mod seg_tree_sum;

mod lazy_seg_tree_add_max;
mod lazy_seg_tree_add_min;
mod lazy_seg_tree_add_sum;
mod lazy_seg_tree_replace_sum;

pub use seg_tree_max::SegTreeMax;
pub use seg_tree_min::SegTreeMin;
pub use seg_tree_sum::SegTreeSum;

pub use lazy_seg_tree_add_max::{LazySegTreeAddMax, LazySegTreeAddMaxSpec};
pub use lazy_seg_tree_add_min::{LazySegTreeAddMin, LazySegTreeAddMinSpec};
pub use lazy_seg_tree_add_sum::{LazySegTreeAddSum, LazySegTreeAddSumSpec};
pub use lazy_seg_tree_replace_sum::{LazySegTreeReplaceSum, LazySegTreeReplaceSumSpec};
