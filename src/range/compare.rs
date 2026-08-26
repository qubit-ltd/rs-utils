// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Lexicographic comparison for values implementing `RangeBounds`.

use std::cmp::Ordering;
use std::ops::RangeBounds;

use super::internal::compare_range_bounds;

/// Compares two ranges lexicographically by their lower and upper bounds.
///
/// Lower bounds are compared first. An unbounded lower side sorts before a
/// finite side, and an included lower endpoint sorts before an excluded lower
/// endpoint at the same value. When lower bounds are equal, upper bounds are
/// compared; an unbounded upper side sorts last, and an excluded upper endpoint
/// sorts before an included upper endpoint at the same value.
///
/// The comparison concerns boundary representations rather than the sets they
/// denote, so two differently represented empty ranges can compare unequal.
///
/// # Type Parameters
///
/// - `T`: Totally ordered endpoint type.
/// - `L`: Left range type.
/// - `R`: Right range type.
///
/// # Parameters
///
/// - `left`: First range to compare.
/// - `right`: Second range to compare.
///
/// # Returns
///
/// The lexicographic ordering of `left` relative to `right`.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
///
/// use qubit_utils::range::Bounds;
/// use qubit_utils::range::compare;
///
/// let mut ranges: Vec<Bounds<i32>> = vec![
///     (Excluded(1), Included(5)),
///     (Included(1), Excluded(5)),
/// ];
/// ranges.sort_by(compare);
/// assert_eq!(ranges[0], (Included(1), Excluded(5)));
/// ```
#[must_use]
#[inline]
pub fn compare<T, L, R>(left: &L, right: &R) -> Ordering
where
    T: Ord + ?Sized,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    compare_range_bounds(left, right)
}
