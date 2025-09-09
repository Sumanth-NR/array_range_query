//! Crate-private utility functions for range parsing and validation used by segment trees.
//!
//! These helpers ensure consistent and robust handling of Rust's `RangeBounds`
//! types and provide a single source of truth for range validation logic.
//!
//! **Note:** This module is private to the crate and not part of the public API.
use std::ops::{Bound, RangeBounds};

/// Converts any `RangeBounds<usize>` into a concrete `(start, end)` tuple,
/// using the provided `size` as the upper bound for unbounded ranges.
///
/// This function is crate-private and not intended for use outside of the crate.
///
/// This function does not perform validation; see [`validate_range`] for that.
pub(crate) fn parse_range<R: RangeBounds<usize>>(range: R, size: usize) -> (usize, usize) {
    let start = match range.start_bound() {
        Bound::Included(&s) => s,
        Bound::Excluded(&s) => s + 1,
        Bound::Unbounded => 0,
    };
    let end = match range.end_bound() {
        Bound::Included(&e) => e + 1,
        Bound::Excluded(&e) => e,
        Bound::Unbounded => size,
    };
    (start, end)
}

/// Validates that a half-open range `[left, right)` is non-empty and within bounds.
///
/// This function is crate-private and not intended for use outside of the crate.
///
/// Panics if `left > right` (empty or reversed range), or if `right > size` (out of bounds).
pub(crate) fn validate_range(left: usize, right: usize, size: usize) {
    assert!(
        left <= right && right <= size,
        "Invalid range: got [{}, {}), size is {}",
        left,
        right,
        size
    );
}
