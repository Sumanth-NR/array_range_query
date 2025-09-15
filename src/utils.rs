//! Utility functions for range parsing and validation.
//!
//! Private helpers for consistent `RangeBounds` handling across segment trees.
use core::ops::{Bound, RangeBounds};

/// Converts any `RangeBounds<usize>` into a concrete `[start, end)` tuple.
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

/// Validates that a range `[left, right)` is within bounds.
///
/// # Panics
/// Panics if `left > right` or `right > size`.
pub(crate) fn validate_range(left: usize, right: usize, size: usize) {
    assert!(
        left <= right && right <= size,
        "Invalid range: got [{}, {}), size is {}",
        left,
        right,
        size
    );
}
