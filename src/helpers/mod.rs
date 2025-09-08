//! Helper types for common segment tree operations.
//!
//! This module provides pre-built specifications and type aliases for common
//! segment tree operations like sum, min, max queries, and range add operations.

mod seg_tree_max;
mod seg_tree_min;
mod seg_tree_sum;

mod lazy_seg_tree_add_max;
mod lazy_seg_tree_add_sum;

pub use seg_tree_max::{SegTreeMax, SegTreeMaxSpec};
pub use seg_tree_min::{SegTreeMin, SegTreeMinSpec};
pub use seg_tree_sum::{SegTreeSum, SegTreeSumSpec};

pub use lazy_seg_tree_add_max::{LazySegTreeAddMax, LazySegTreeAddMaxSpec};
pub use lazy_seg_tree_add_sum::{LazySegTreeAddSum, LazySegTreeAddSumSpec};
