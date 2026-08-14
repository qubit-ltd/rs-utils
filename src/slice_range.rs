// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Checked range arithmetic helpers for slice-like bounds validation.

use std::convert::Infallible;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

/// Namespace for checked range calculations for slice-like APIs.
///
/// This type groups pure helpers that compute and validate `start + count`
/// against a buffer length. The helpers are not unsafe and have no dependency
/// on element type.
pub struct SliceRange {
    /// Prevents construction of this namespace type.
    _private: Infallible,
}

impl SliceRange {
    /// Returns the exclusive end index of a checked range.
    ///
    /// # Parameters
    ///
    /// - `len`: Slice length.
    /// - `start`: Start index in the slice.
    /// - `count`: Number of requested items.
    ///
    /// # Returns
    ///
    /// `Some(end)` when `start + count` is valid and not overflowed.
    #[inline]
    pub const fn range_end(
        len: usize,
        start: usize,
        count: usize,
    ) -> Option<usize> {
        match start.checked_add(count) {
            Some(end) if len >= end => Some(end),
            _ => None,
        }
    }

    /// Returns whether a slice has at least `count` accessible items from
    /// `start`.
    ///
    /// # Parameters
    ///
    /// - `len`: Slice length.
    /// - `start`: Start index in the slice.
    /// - `count`: Number of requested items.
    ///
    /// # Returns
    ///
    /// `true` when `start + count <= len` and no overflow occurs.
    #[must_use]
    #[inline(always)]
    pub const fn range_fits(len: usize, start: usize, count: usize) -> bool {
        Self::range_end(len, start, count).is_some()
    }

    /// Returns the exclusive end index as an I/O result.
    ///
    /// # Parameters
    ///
    /// - `len`: Slice length.
    /// - `start`: Start index in the slice.
    /// - `count`: Number of requested items.
    /// - `message`: Error message used when invalid.
    ///
    /// # Returns
    ///
    /// Returns the exclusive end index when the range fits.
    ///
    /// # Errors
    ///
    /// Returns [`ErrorKind::InvalidInput`] with `message` when the range does
    /// not fit or overflows.
    #[inline]
    pub fn checked_range_end(
        len: usize,
        start: usize,
        count: usize,
        message: &'static str,
    ) -> Result<usize> {
        Self::range_end(len, start, count)
            .ok_or_else(|| Error::new(ErrorKind::InvalidInput, message))
    }
}
