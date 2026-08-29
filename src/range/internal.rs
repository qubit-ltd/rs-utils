// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Internal boundary comparison rules shared by range operations.

use std::cmp::Ordering;
use std::ops::Bound;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;
use std::ops::RangeBounds;

use super::Bounds;

/// Clones one borrowed standard-library bound into an owned bound.
///
/// # Parameters
///
/// - `bound`: Borrowed bound obtained from a `RangeBounds` implementation.
///
/// # Returns
///
/// An owned bound with the same inclusion state and endpoint value.
pub(super) fn clone_bound<T>(bound: Bound<&T>) -> Bound<T>
where
    T: Clone,
{
    match bound {
        Included(value) => Included(value.clone()),
        Excluded(value) => Excluded(value.clone()),
        Unbounded => Unbounded,
    }
}

/// Clones both endpoints from a borrowed range into an owned boundary pair.
///
/// # Parameters
///
/// - `range`: Range whose lower and upper bounds are cloned.
///
/// # Returns
///
/// Owned lower and upper bounds preserving the source range semantics.
pub(super) fn clone_bounds<T, R>(range: &R) -> Bounds<T>
where
    T: Clone,
    R: RangeBounds<T>,
{
    (clone_bound(range.start_bound()), clone_bound(range.end_bound()))
}

/// Compares lower bounds by the first value admitted by each boundary.
///
/// An unbounded lower bound sorts first. At the same finite endpoint, an
/// included bound sorts before an excluded bound because it starts earlier.
///
/// # Parameters
///
/// - `left`: Lower bound from the left range.
/// - `right`: Lower bound from the right range.
///
/// # Returns
///
/// The ordering of `left` relative to `right` when both are lower bounds.
pub(super) fn compare_lower_bounds<T>(left: Bound<&T>, right: Bound<&T>) -> Ordering
where
    T: Ord + ?Sized,
{
    match (left, right) {
        (Unbounded, Unbounded) => Ordering::Equal,
        (Unbounded, _) => Ordering::Less,
        (_, Unbounded) => Ordering::Greater,
        (Included(left), Included(right)) | (Excluded(left), Excluded(right)) => left.cmp(right),
        (Included(left), Excluded(right)) => left.cmp(right).then(Ordering::Less),
        (Excluded(left), Included(right)) => left.cmp(right).then(Ordering::Greater),
    }
}

/// Compares upper bounds by the last value admitted by each boundary.
///
/// An unbounded upper bound sorts last. At the same finite endpoint, an
/// excluded bound sorts before an included bound because it ends earlier.
///
/// # Parameters
///
/// - `left`: Upper bound from the left range.
/// - `right`: Upper bound from the right range.
///
/// # Returns
///
/// The ordering of `left` relative to `right` when both are upper bounds.
pub(super) fn compare_upper_bounds<T>(left: Bound<&T>, right: Bound<&T>) -> Ordering
where
    T: Ord + ?Sized,
{
    match (left, right) {
        (Unbounded, Unbounded) => Ordering::Equal,
        (Unbounded, _) => Ordering::Greater,
        (_, Unbounded) => Ordering::Less,
        (Included(left), Included(right)) | (Excluded(left), Excluded(right)) => left.cmp(right),
        (Included(left), Excluded(right)) => left.cmp(right).then(Ordering::Greater),
        (Excluded(left), Included(right)) => left.cmp(right).then(Ordering::Less),
    }
}

/// Compares two complete ranges lexicographically by lower then upper bound.
///
/// # Parameters
///
/// - `left`: First range to compare.
/// - `right`: Second range to compare.
///
/// # Returns
///
/// The lower-bound ordering unless the lower bounds are equal, in which case
/// the upper-bound ordering is returned.
pub(super) fn compare_range_bounds<T, L, R>(left: &L, right: &R) -> Ordering
where
    T: Ord + ?Sized,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    compare_lower_bounds(left.start_bound(), right.start_bound())
        .then_with(|| compare_upper_bounds(left.end_bound(), right.end_bound()))
}

/// Reports whether a lower and upper bound describe an empty range.
///
/// A one-sided or fully unbounded range is never empty. Finite incomparable
/// endpoints are treated as empty, matching the proposed standard-library
/// `RangeBounds::is_empty` behavior. Equal finite endpoints are nonempty only
/// when both endpoints are included.
///
/// # Parameters
///
/// - `lower`: Lower bound of the range.
/// - `upper`: Upper bound of the range.
///
/// # Returns
///
/// `true` when the boundary pair admits no value, or `false` otherwise.
pub(super) fn bounds_are_empty<T>(lower: Bound<&T>, upper: Bound<&T>) -> bool
where
    T: PartialOrd + ?Sized,
{
    let (lower, lower_included) = match lower {
        Included(value) => (value, true),
        Excluded(value) => (value, false),
        Unbounded => return false,
    };
    let (upper, upper_included) = match upper {
        Included(value) => (value, true),
        Excluded(value) => (value, false),
        Unbounded => return false,
    };
    match lower.partial_cmp(upper) {
        Some(Ordering::Less) => false,
        Some(Ordering::Equal) => !(lower_included && upper_included),
        Some(Ordering::Greater) | None => true,
    }
}

/// Reports whether two ordered, nonempty ranges have a gap between them.
///
/// `left_upper` must belong to the range that starts first, and `right_lower`
/// must belong to the range that starts second. Equal endpoints leave a gap
/// only when both ranges exclude the shared endpoint.
///
/// # Parameters
///
/// - `left_upper`: Upper bound of the earlier range.
/// - `right_lower`: Lower bound of the later range.
///
/// # Returns
///
/// `true` when at least one value lies outside both ranges between the bounds.
pub(super) fn bounds_have_gap<T>(left_upper: Bound<&T>, right_lower: Bound<&T>) -> bool
where
    T: Ord + ?Sized,
{
    let (left, left_excluded) = match left_upper {
        Included(value) => (value, false),
        Excluded(value) => (value, true),
        Unbounded => return false,
    };
    let (right, right_excluded) = match right_lower {
        Included(value) => (value, false),
        Excluded(value) => (value, true),
        Unbounded => return false,
    };
    match left.cmp(right) {
        Ordering::Less => true,
        Ordering::Equal => left_excluded && right_excluded,
        Ordering::Greater => false,
    }
}
