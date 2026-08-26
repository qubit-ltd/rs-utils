# rs-utils

[![Rust CI](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-utils/coverage-badge.json)](https://qubit-ltd.github.io/rs-utils/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-utils.svg?color=blue)](https://crates.io/crates/qubit-utils)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

General-purpose, dependency-light utilities for Rust projects. This crate gives
Qubit applications one shared implementation for standard-range algebra,
fallible allocation, runtime-only values, and audited low-level slice access.

## Intended Users

Rust developers who need small, reusable building blocks across Qubit projects
without adopting a broad framework or duplicating boundary-sensitive code.

## Installation

Add the published crate to your `Cargo.toml`:

```toml
[dependencies]
qubit-utils = "0.3"
```

## Quick Start

Suppose a service accepts a requested numeric window but may process only values
inside a configured policy window. The range helpers accept standard Rust range
syntax and return owned standard-library bounds:

```rust
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;

use qubit_utils::range::encloses;
use qubit_utils::range::intersection;

let policy = 1..10;
let requested = 5..=20;
let accepted = intersection(&policy, &requested);

assert_eq!(accepted, Some((Included(5), Excluded(10))));
assert!(encloses(&policy, &accepted.expect("the windows overlap")));
```

The result is a `(Bound<T>, Bound<T>)`, which already implements
`RangeBounds<T>` and can be passed to ordered standard-library collection APIs.

## Why This Project Exists

Boundary-heavy helpers are easy to get almost right and costly to maintain in
multiple crates. `qubit-utils` centralizes the small contracts that recur across
Qubit software while preserving standard Rust representations and keeping the
default dependency surface small.

## Current Status

The 0.3.0 crate provides several reusable utility APIs:

- standard-range relationships and algebra (`is_empty`, `encloses`, `overlaps`,
  `is_connected`, `intersection`, `span`, `gap`, and `compare`),
- fallible allocation helpers (`create_vec`, `try_reserve_vec`, `try_reserve_string`,
  `allocation_error`, plus coverage-testing helpers under `coverage` cfg),
- `nonzero` helper for `NonZeroUsize`,
- `Transient<T>` for runtime-only state excluded from `Eq`/`Hash`,
- `UncheckedSlice` and `SliceRange` for low-level slice/array boundary checks.

## Capabilities

The range module works with `std::ops::RangeBounds`, including normal Rust range
syntax and arbitrary `(Bound<T>, Bound<T>)` pairs. It adds stable free functions
for the range relationships and operations that are not yet available as a
complete stable standard-library API.

The remaining modules provide reusable memory, numeric, slice, and runtime-state
helpers used by adjacent Qubit Rust crates. Existing crate-root re-exports remain
available for their commonly used types and functions.

## Limitations

Range algebra other than `is_empty` requires a total order (`Ord`). Use an
explicit ordered wrapper when working with floating-point values whose NaN
semantics need a policy. Range emptiness follows endpoint-order semantics and
does not canonicalize a discrete domain, so an open integer range such as
`(1, 2)` is not treated as empty merely because no integer lies between its
endpoints.

The crate intentionally does not define a custom interval type, dynamic
comparators, discrete-domain stepping, or domain-specific date and money range
types. Additional helpers are added only when they are broadly reusable.

## Testing

```bash
# Run tests with the default feature set
cargo test

# Run tests with all declared features
cargo test --all-features

# Project CI checks
./ci-check.sh

# Check code coverage
./coverage.sh
```

## License

Copyright (c) 2025 - 2026. Haixing Hu. All rights reserved.

Licensed under the Apache License, Version 2.0. See [LICENSE](LICENSE) for the
full license text.

## Contributing

Contributions are welcome. Please follow the Rust API guidelines, keep public
API documentation and tests current, and run `./align-ci.sh` to format code and
`./ci-check.sh` to satisfy CI requirements before submitting a pull request.

## Author

**Haixing Hu** - *Qubit Co. Ltd.*

Repository: [https://github.com/qubit-ltd/rs-utils](https://github.com/qubit-ltd/rs-utils)
