# Copilot Instructions for array_range_query Rust Library

## General Guidelines

- Write idiomatic, modern Rust code following Rust 2021 edition standards.
- Prefer explicit type annotations for public APIs and complex generic code.
- Use comprehensive unit tests (`#[cfg(test)]`) for all new features and edge cases.
- Document all public items with `///` doc comments, including examples where appropriate.
- Use `clippy` and `cargo fmt` for linting and formatting.

## Code Organization Principles

### Function Ordering
Functions should be organized in logical sections with clear separators:

1. **Constructors** (`new`, `from_vec`, `from_mut_vec`) - Always first
2. **Public Interface** (main API methods like `query`, `update`) - Core functionality
3. **Private Helper Methods** - Implementation details
4. **Display/Debug implementations** - For debugging support
5. **Tests** - Always last, in a `#[cfg(test)]` module

### Documentation Structure
- Start with a comprehensive module-level doc comment explaining the purpose
- Include **Overview**, **Design**, **Key Properties**, and **Examples** sections
- For each public function, include **Time Complexity**, **Panics**, and **Examples**
- Use `# Examples` sections liberally with working code

## Segment Tree Patterns

### Trait Pattern
- When implementing segment trees, use a trait (e.g., `SegTreeSpec`, `LazySegTreeSpec`) to define the operation, identity, and types.
- Traits should be well-documented with mathematical properties (monoid laws, associativity).

### AddAssign Pattern (CRITICAL)
- **For all monoid operations, use the `AddAssign`-style pattern:**
  - The operation should mutate the first argument in place: `fn op(a: &mut T, b: &T)`
  - **NEVER** return new values for associative operations; always mutate in place
  - This applies to all trait methods: `op_on_data`, `op_on_update`, `op_update_on_data`

### Identity Element
- Always define an identity element as an associated constant (`const ID: T`).
- Document the mathematical property: `op(a, ID) = a` for all `a`.

## Lazy Segment Tree Specifics

- Use the same AddAssign pattern for all operations in `LazySegTreeSpec`.
- When composing updates or applying updates to data, mutate the first argument in place.
- Use `Option<U>` for lazy tags and provide internal helpers for combining tags.
- Keep the helper method `combine_tag_option` as a private implementation detail inside the struct.

## Error Handling & Panics

- Use `assert!` with descriptive messages for precondition violations (bounds checking).
- Panic messages should be clear and actionable: `"update index out of bounds"`.
- Document all panic conditions in the function's doc comment.

## Documentation & Examples

- All examples in documentation and tests should use the AddAssign pattern for trait implementations.
- Include **Time Complexity** and **Space Complexity** information.
- Provide multiple examples showing different use cases (basic usage, edge cases, different ranges).
- Keep README and doc comments up to date with the latest API and patterns.

## Testing Guidelines

- Add comprehensive tests for all helpers and custom specs.
- Test edge cases: empty ranges, full ranges, overlapping updates, boundary conditions, and expected panics.
- Use descriptive test names: `test_panic_update_out_of_bounds`.
- Include stress tests for larger datasets when appropriate.

## API Consistency

- Ensure all helpers and user-facing APIs follow the same trait and operation patterns.
- Range parameters should accept `impl RangeBounds<usize>` for flexibility.
- **Never introduce new patterns** for associative operations; stick to AddAssign/in-place mutation.
- Maintain consistent naming: `query` for read operations, `update` for write operations.

## Performance Considerations

- Use `RefCell` only when necessary (e.g., for read-only query operations in lazy trees).
- Prefer direct mutable access (`get_mut()`) in `&mut self` methods.
- Document when operations avoid allocations or cloning.
- Use 1-based indexing for tree implementations for cleaner parent/child relationships.

## Code Style

- Use clear section separators with comments: `// ===== CONSTRUCTORS =====`
- Group related functionality together.
- Private helper methods should be well-documented even though they're internal.
- Use meaningful variable names: `left_result`, `size`, `leaf_index`.

## Linting & Quality Assurance

- **Before submitting or merging code, always run:**
  ```bash
  cargo clippy --all-targets --all-features -- -D warnings
  ```
  This ensures the codebase is free of warnings and follows best practices.

- Also run comprehensive tests:
  ```bash
  cargo test --all
  ```

## Generic Programming

- Use clear type parameter names: `Spec` for specifications, `T` for data types, `U` for update types.
- Provide comprehensive trait bounds documentation.
- Use `PhantomData` markers appropriately for zero-cost abstractions.

## Mathematical Correctness

- Ensure all implementations satisfy mathematical properties (associativity, identity laws).
- Document the mathematical foundation in trait documentation.
- Provide examples that demonstrate the mathematical properties in action.

## Version Compatibility

- Maintain backward compatibility for public APIs.
- Document any breaking changes thoroughly.
- Use semantic versioning appropriately.

## Benchmarking (using Criterion)

- Checkout `main` branch to run the baseline for Benchmarking.
- Then checkout the branch you want to compare against.
- Run the benchmarking script:
  ```bash
  cargo bench
  ```
- This gives a detailed report of the performance of the code.

### Benchmarking Alternatively

- Checkout `main` branch and perform benchmarking using Criterion once `cargo bench`.
- Save the results into `targets/benchmarks_main` directory.
  ```bash
  cargo bench --save-results
  ```
- Compare the results with the baseline:
  ```bash
  cargo bench --compare-results
  ```
