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

Add `qubit-utils` to the dependencies in your `Cargo.toml` when a published API
is available.

## Current Status

The initial release contains the crate foundation only. It does not yet expose
concrete utility functions or types.

## Capabilities

- Provides a standard Rust library crate for future general-purpose utilities.
- Keeps the initial dependency surface empty.

## Limitations

No concrete public utility API is available in the initial release.

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
