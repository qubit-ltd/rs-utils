// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! General-purpose utilities for Rust projects.
//!
//! The crate provides reusable runtime-state, non-zero construction, fallible
//! allocation, range validation, and unchecked-slice utilities.
// qubit-style: allow coverage-cfg

mod allocation;
mod any_bit_pattern;
mod internal;
mod nonzero;
mod slice_range;
mod transient;
mod unchecked_slice;

pub use allocation::{
    allocation_error,
    create_vec,
    try_reserve_string,
    try_reserve_vec,
};
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::{
    coverage_fail_next_reserve,
    coverage_fail_next_string_reserve,
    coverage_fail_reserve_above,
    coverage_fail_reserve_after,
    coverage_reset_reserve_hooks,
};
pub use any_bit_pattern::AnyBitPattern;
pub use nonzero::nonzero;
pub use slice_range::SliceRange;
pub use transient::Transient;
pub use unchecked_slice::UncheckedSlice;
