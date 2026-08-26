// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! General-purpose utilities for Rust projects.
//!
//! The crate provides reusable standard-range algebra, runtime-state, non-zero
//! construction, fallible allocation, range validation, and unchecked-slice
//! utilities.
// qubit-style: allow coverage-cfg

mod internal;
pub mod math;
pub mod memory;
pub mod range;
pub mod util;

pub use math::nonzero;
pub use memory::AnyBitPattern;
pub use memory::SliceRange;
pub use memory::UncheckedSlice;
pub use memory::allocation_error;
#[cfg(coverage)]
#[doc(hidden)]
pub use memory::coverage_fail_next_reserve;
#[cfg(coverage)]
#[doc(hidden)]
pub use memory::coverage_fail_next_string_reserve;
#[cfg(coverage)]
#[doc(hidden)]
pub use memory::coverage_fail_reserve_above;
#[cfg(coverage)]
#[doc(hidden)]
pub use memory::coverage_fail_reserve_after;
#[cfg(coverage)]
#[doc(hidden)]
pub use memory::coverage_reset_reserve_hooks;
pub use memory::create_vec;
pub use memory::try_reserve_string;
pub use memory::try_reserve_vec;
pub use util::Transient;
