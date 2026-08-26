// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Compile-time `NonZeroUsize` construction helper.

use core::num::NonZeroUsize;

/// Returns a [`NonZeroUsize`] from a known non-zero value.
///
/// This helper is const-evaluable, so a zero constant fails during const
/// evaluation. For runtime values, it keeps API usage explicit and safe.
///
/// # Parameters
///
/// - `value`: Non-zero item count.
///
/// # Returns
///
/// Returns a [`NonZeroUsize`] equal to `value`.
///
/// # Panics
///
/// Panics when `value` is zero.
///
/// # Examples
///
/// ```
/// const MAX_GROUPS: core::num::NonZeroUsize = qubit_utils::nonzero(8);
///
/// assert_eq!(MAX_GROUPS.get(), 8);
/// ```
#[must_use]
#[inline(always)]
pub const fn nonzero(value: usize) -> NonZeroUsize {
    match NonZeroUsize::new(value) {
        Some(value) => value,
        None => panic!(concat!(
            "qubit_utils",
            "::nonzero(): value must be non-zero",
        )),
    }
}
