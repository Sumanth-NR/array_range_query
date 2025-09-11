# Targets

## Feature Ideas

### Version 0.2

- [x] Change queries and updates to use `std::ops::{Bound, RangeBounds}`
  (In the future, consider changing the API to use `core::range`)
- [x] Use the AddAssign style instead of Add style everywhere
- [x] Improve SegTree type bounds (we already encapsulate the Monoid structure in the Spec)

## Version 0.3

- [x] `from_vec` in `LazySegTree` must take ownership of the vector
- [x] Add profiling to the library README
- [ ] Try using `Box<&[T]>` instead of `Vec<T>`
- [ ] Add Immutable Array Range Queries (SparseTable) for idempotent operations with `O(1)` query time and `O(n log n)` space complexity
