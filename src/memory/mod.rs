// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Low-level memory and slice utilities.
// qubit-style: allow coverage-cfg

mod allocation;
mod any_bit_pattern;
mod slice_range;
mod unchecked_slice;

pub use allocation::allocation_error;
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::coverage_fail_next_reserve;
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::coverage_fail_next_string_reserve;
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::coverage_fail_reserve_above;
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::coverage_fail_reserve_after;
#[cfg(coverage)]
#[doc(hidden)]
pub use allocation::coverage_reset_reserve_hooks;
pub use allocation::create_vec;
pub use allocation::try_reserve_string;
pub use allocation::try_reserve_vec;
pub use any_bit_pattern::AnyBitPattern;
pub use slice_range::SliceRange;
pub use unchecked_slice::UncheckedSlice;
