# rs-utils

[![Rust CI](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-utils/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-utils/coverage-badge.json)](https://qubit-ltd.github.io/rs-utils/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-utils.svg?color=blue)](https://crates.io/crates/qubit-utils)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

General-purpose utilities for Rust projects. This repository is the foundation for
small, reusable helpers shared across Qubit software.

## Intended Users

Rust developers who need to share broadly applicable utilities across Qubit
projects.

## Installation

Add the published crate to your `Cargo.toml`:

```toml
[dependencies]
qubit-utils = "0.2"
```

## Quick Start

Use checked range arithmetic before an intentionally unchecked slice access:

```rust
use qubit_utils::{nonzero, SliceRange, UncheckedSlice};

let required = nonzero(2);
let input = [0x10_u8, 0x20, 0x30];
let end = SliceRange::checked_range_end(input.len(), 1, required.get(), "range exceeds input")
    .expect("validated range should fit");

// SAFETY: `end` proves that the requested range is inside `input`.
let value = unsafe { UncheckedSlice::read(&input, end - 1) };
assert_eq!(value, 0x20);
```

## Current Status

The 0.2.0 crate provides several reusable utility APIs:

- fallible allocation helpers (`create_vec`, `try_reserve_vec`, `try_reserve_string`,
  `allocation_error`, plus coverage-testing helpers under `coverage` cfg),
- `nonzero` helper for `NonZeroUsize`,
- `Transient<T>` for runtime-only state excluded from `Eq`/`Hash`,
- `UncheckedSlice` and `SliceRange` for low-level slice/array boundary checks.

## Capabilities

Provides reusable, dependency-light helpers used by adjacent Qubit Rust crates.

## Limitations

The API set is focused and intentionally small; additional domain-specific
helpers are expected to evolve from real usage requirements.

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
