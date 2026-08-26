// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Set relationships between values implementing `RangeBounds`.

use std::cmp::Ordering;
use std::ops::RangeBounds;

use super::internal::bounds_are_empty;
use super::internal::bounds_have_gap;
use super::internal::compare_lower_bounds;
use super::internal::compare_upper_bounds;

/// Reports whether a standard-library range contains no values.
///
/// A range with either side unbounded is considered nonempty. A finite range
/// is empty when its lower endpoint is greater than its upper endpoint, when
/// equal endpoints are not both included, or when the endpoints are
/// incomparable. This matches the semantics of the currently unstable
/// `RangeBounds::is_empty` API in Rust 1.94.
///
/// # Type Parameters
///
/// - `T`: Endpoint type supporting partial comparison.
/// - `R`: Any standard or custom range implementing [`RangeBounds<T>`].
///
/// # Parameters
///
/// - `range`: Range whose boundary pair is inspected.
///
/// # Returns
///
/// `true` when the range admits no values, or `false` otherwise.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
///
/// use qubit_utils::range::is_empty;
///
/// assert!(is_empty(&(5..5)));
/// assert!(!is_empty(&(5..=5)));
/// assert!(is_empty(&(Excluded(5), Excluded(5))));
/// ```
#[must_use]
#[inline]
pub fn is_empty<T, R>(range: &R) -> bool
where
    T: PartialOrd + ?Sized,
    R: RangeBounds<T>,
{
    bounds_are_empty(range.start_bound(), range.end_bound())
}

/// Reports whether every value in `inner` is also contained by `outer`.
///
/// This function uses set semantics: every range encloses an empty range, and
/// an empty range encloses only another empty range. Open, closed, and
/// unbounded endpoints are compared without converting their values.
///
/// # Type Parameters
///
/// - `T`: Totally ordered endpoint type.
/// - `L`: Range type used for `outer`.
/// - `R`: Range type used for `inner`.
///
/// # Parameters
///
/// - `outer`: Candidate enclosing range.
/// - `inner`: Candidate enclosed range.
///
/// # Returns
///
/// `true` when `inner` is a subset of `outer`, or `false` otherwise.
///
/// # Examples
///
/// ```
/// use qubit_utils::range::encloses;
///
/// assert!(encloses(&(1..=10), &(3..5)));
/// assert!(!encloses(&(3..5), &(1..=10)));
/// ```
#[must_use]
pub fn encloses<T, L, R>(outer: &L, inner: &R) -> bool
where
    T: Ord + ?Sized,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    if is_empty(inner) {
        return true;
    }
    if is_empty(outer) {
        return false;
    }
    compare_lower_bounds(outer.start_bound(), inner.start_bound())
        != Ordering::Greater
        && compare_upper_bounds(outer.end_bound(), inner.end_bound())
            != Ordering::Less
}

/// Reports whether two ranges share at least one value.
///
/// Empty ranges never overlap. The operation is symmetric and supports any
/// combination of standard and arbitrary [`RangeBounds`] implementations.
///
/// # Type Parameters
///
/// - `T`: Totally ordered endpoint type.
/// - `L`: Left range type.
/// - `R`: Right range type.
///
/// # Parameters
///
/// - `left`: First range to inspect.
/// - `right`: Second range to inspect.
///
/// # Returns
///
/// `true` when the intersection contains at least one value, or `false` when
/// the intersection is empty.
///
/// # Examples
///
/// ```
/// use qubit_utils::range::overlaps;
///
/// assert!(overlaps(&(1..=2), &(2..3)));
/// assert!(!overlaps(&(1..2), &(2..3)));
/// ```
#[must_use]
pub fn overlaps<T, L, R>(left: &L, right: &R) -> bool
where
    T: Ord + ?Sized,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    if is_empty(left) || is_empty(right) {
        return false;
    }
    let lower = if compare_lower_bounds(left.start_bound(), right.start_bound())
        == Ordering::Less
    {
        right.start_bound()
    } else {
        left.start_bound()
    };
    let upper = if compare_upper_bounds(left.end_bound(), right.end_bound())
        == Ordering::Greater
    {
        right.end_bound()
    } else {
        left.end_bound()
    };
    !bounds_are_empty(lower, upper)
}

/// Reports whether the union of two ranges can be represented by one range.
///
/// Overlapping ranges are connected. Non-overlapping ranges that meet at the
/// same endpoint are connected when at least one side includes that endpoint.
/// Under set semantics, an empty range is connected to every range because it
/// does not change the other range's union.
///
/// # Type Parameters
///
/// - `T`: Totally ordered endpoint type.
/// - `L`: Left range type.
/// - `R`: Right range type.
///
/// # Parameters
///
/// - `left`: First range to inspect.
/// - `right`: Second range to inspect.
///
/// # Returns
///
/// `true` when the union has no gap, or `false` otherwise.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
///
/// use qubit_utils::range::is_connected;
///
/// assert!(is_connected(&(1..2), &(2..3)));
/// assert!(!is_connected(
///     &(Included(1), Excluded(2)),
///     &(Excluded(2), Included(3)),
/// ));
/// ```
#[must_use]
pub fn is_connected<T, L, R>(left: &L, right: &R) -> bool
where
    T: Ord + ?Sized,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    if is_empty(left) || is_empty(right) {
        return true;
    }
    if compare_lower_bounds(left.start_bound(), right.start_bound())
        == Ordering::Greater
    {
        !bounds_have_gap(right.end_bound(), left.start_bound())
    } else {
        !bounds_have_gap(left.end_bound(), right.start_bound())
    }
}
