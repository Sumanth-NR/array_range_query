# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Project Overview

This is `array_range_query`, a Rust library providing high-performance, generic implementations of segment trees and lazy segment trees for efficient range queries and range updates. The library supports any associative operation through type-safe trait specifications.

## Common Development Commands

### Building and Testing
```bash
# Build the library
cargo build

# Run all tests
cargo test --verbose

# Run tests with output
cargo test -- --nocapture

# Run specific test file
cargo test test_seg_tree_node

# Build documentation
cargo doc --no-deps --document-private-items
```

### Code Quality
```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --all -- --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings

# Combined quality check (as used in CI)
cargo fmt --all -- --check && cargo clippy --all-targets --all-features -- -D warnings
```

### Examples and Benchmarks
```bash
# Run the basic usage example
cargo run --example basic_usage

# Run benchmarks
cargo bench

# Run specific benchmark
cargo bench seg_tree_1000
cargo bench lazy_seg_tree_1000
```

### Single Test Execution
```bash
# Run a single test by name
cargo test test_overlapping_updates

# Run tests in a specific module
cargo test lazy_seg_tree::tests

# Run tests with pattern matching
cargo test query
```

## Architecture Overview

The library is organized around a generic trait-based architecture:

### Core Components

1. **Segment Tree (`SegTree<Spec>`)**: Generic point-update, range-query data structure
   - Defined by `SegTreeSpec` trait
   - Supports any associative operation with identity element
   - O(log n) queries and updates

2. **Lazy Segment Tree (`LazySegTree<Spec>`)**: Generic range-update, range-query data structure  
   - Defined by `LazySegTreeSpec` trait
   - Uses lazy propagation for efficient range updates
   - O(log n) queries and range updates

3. **Helper Types** (`src/helpers/`): Pre-built implementations for common operations
   - `SegTreeSum`, `SegTreeMin`, `SegTreeMax`
   - `LazySegTreeAddSum`, `LazySegTreeAddMin`, `LazySegTreeAddMax`, `LazySegTreeReplaceSum`

### Key Design Patterns

- **AddAssign Pattern**: All monoid operations mutate the first argument in-place (`fn op(a: &mut T, b: &T)`)
- **Trait-based Specification**: Operations are defined through traits, enabling zero-cost abstractions
- **1-based Indexing**: Internal tree representation uses 1-based indexing for cleaner parent/child relationships
- **Generic Range Support**: All methods accept `impl RangeBounds<usize>` for flexible range specifications

## File Structure

```
src/
├── lib.rs                 # Main library exports and documentation
├── seg_tree.rs            # Generic segment tree implementation
├── lazy_seg_tree.rs       # Generic lazy segment tree implementation  
├── seg_tree_node.rs       # Tree navigation utilities
├── utils.rs               # Range parsing and validation
└── helpers/               # Pre-built helper implementations
    ├── mod.rs
    ├── seg_tree_*.rs       # Regular segment tree helpers
    └── lazy_seg_tree_*.rs  # Lazy segment tree helpers

tests/                     # Integration tests
benches/                   # Performance benchmarks
examples/                  # Usage examples
```

## Development Guidelines

### When Adding New Features

1. **Follow the AddAssign Pattern**: All associative operations must mutate the first argument in-place
2. **Comprehensive Documentation**: Include time complexity, panic conditions, and working examples
3. **Extensive Testing**: Add tests for edge cases, boundary conditions, and expected panics
4. **Mathematical Correctness**: Ensure all implementations satisfy monoid laws (associativity, identity)

### Trait Implementation Requirements

For `SegTreeSpec`:
- `type T`: Element type (must implement `Clone`)
- `const ID`: Identity element satisfying `op(a, ID) = a`
- `fn op(a: &mut T, b: &T)`: Associative operation modifying `a` in-place

For `LazySegTreeSpec`:
- `type T`: Data type, `type U`: Update type
- `const ID`: Identity for data aggregation
- `fn op_on_data(d1: &mut T, d2: &T)`: Combine data values in-place
- `fn op_on_update(u1: &mut U, u2: &U)`: Compose updates in-place
- `fn op_update_on_data(u: &U, d: &mut T, size: usize)`: Apply update to data

### Testing Strategy

- **Unit Tests**: Each module contains comprehensive `#[cfg(test)]` modules
- **Integration Tests**: `tests/` directory for cross-module testing
- **Benchmarks**: `benches/` directory using Criterion for performance validation
- **Edge Cases**: Empty ranges, full ranges, boundary conditions, invalid inputs

### Code Organization

Functions should be organized in this order:
1. **Constructors** (`new`, `from_vec`, `from_slice`)
2. **Public Interface** (`query`, `update`)
3. **Private Helper Methods**
4. **Display/Debug implementations**
5. **Tests** (in `#[cfg(test)]` modules)

## Range Syntax Support

All query and update methods support flexible range types:
- `2..5` (half-open)
- `2..=4` (inclusive)  
- `..3` (from start)
- `2..` (to end)
- `..` (entire range)

## CI Pipeline

The project uses GitHub Actions (`arq_main.yml`) with these checks:
- Code formatting verification
- Clippy linting with zero warnings
- Build verification
- Test execution
- Documentation generation