# Copilot Instructions for array_range_query Rust Library

## General Guidelines

- Write idiomatic, modern Rust code.
- Prefer explicit type annotations for public APIs.
- Use `#[cfg(test)]` and thorough unit tests for all new features.
- Document all public items with `///` doc comments.
- Use `clippy` and `cargo fmt` for linting and formatting.

## Segment Tree Patterns

- **Trait Pattern:** When implementing segment trees, use a trait (e.g., `SegTreeSpec`, `LazySegTreeSpec`) to define the operation, identity, and types.
- **AddAssign Pattern:** For all monoid operations, prefer the `AddAssign`-style pattern:
  - The operation should mutate the first argument in place, e.g. `fn op(a: &mut T, b: &T)`.
  - Avoid returning new values for associative operations; mutate in place instead.
- **Identity Element:** Always define an identity element as an associated constant (`const ID: T`).

## Lazy Segment Tree

- Use the same AddAssign pattern for all operations in `LazySegTreeSpec`.
- When composing updates or applying updates to data, mutate the first argument in place.
- Use `Option<U>` for lazy tags and provide helpers for combining tags.

## Documentation & Examples

- All examples in documentation and tests should use the AddAssign pattern for trait implementations.
- Keep README and doc comments up to date with the latest API and patterns.

## Testing

- Add comprehensive tests for all helpers and custom specs.
- Test edge cases: empty ranges, full ranges, overlapping updates, and panics.

## API Consistency

- Ensure all helpers and user-facing APIs follow the same trait and operation patterns.
- Avoid introducing new patterns for associative operations; stick to AddAssign/in-place mutation.

## Linting

- **Before submitting or merging code, always run:**
  ```sh
  cargo clippy --all-targets --all-features -- -D warnings
  ```
