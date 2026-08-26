// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Owned results for operations on values implementing `RangeBounds`.

use std::cmp::Ordering;
use std::ops::Bound;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;
use std::ops::RangeBounds;

use super::Bounds;
use super::internal::bounds_are_empty;
use super::internal::clone_bound;
use super::internal::clone_bounds;
use super::internal::compare_lower_bounds;
use super::internal::compare_range_bounds;
use super::internal::compare_upper_bounds;
use super::relation::is_connected;
use super::relation::is_empty;

/// Returns the nonempty intersection shared by two ranges.
///
/// The returned boundary pair owns cloned endpoint values and itself
/// implements [`RangeBounds<T>`]. Empty inputs, disjoint ranges, and ranges
/// that only touch without sharing a value have no nonempty intersection.
///
/// # Type Parameters
///
/// - `T`: Cloneable, totally ordered endpoint type.
/// - `L`: Left range type.
/// - `R`: Right range type.
///
/// # Parameters
///
/// - `left`: First range to intersect.
/// - `right`: Second range to intersect.
///
/// # Returns
///
/// `Some(bounds)` for a nonempty intersection, or `None` when the intersection
/// contains no value.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
///
/// use qubit_utils::range::intersection;
///
/// assert_eq!(
///     intersection(&(1..10), &(5..=20)),
///     Some((Included(5), Excluded(10))),
/// );
/// assert_eq!(intersection(&(1..5), &(5..10)), None);
/// ```
#[must_use]
pub fn intersection<T, L, R>(left: &L, right: &R) -> Option<Bounds<T>>
where
    T: Ord + Clone,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    if is_empty(left) || is_empty(right) {
        return None;
    }
    let lower = if compare_lower_bounds(left.start_bound(), right.start_bound())
        == Ordering::Less
    {
        clone_bound(right.start_bound())
    } else {
        clone_bound(left.start_bound())
    };
    let upper = if compare_upper_bounds(left.end_bound(), right.end_bound())
        == Ordering::Greater
    {
        clone_bound(right.end_bound())
    } else {
        clone_bound(left.end_bound())
    };
    if bounds_are_empty(lower.as_ref(), upper.as_ref()) {
        None
    } else {
        Some((lower, upper))
    }
}

/// Returns the smallest range that encloses both input ranges.
///
/// An empty input does not enlarge the other range. When both inputs are
/// empty, the lexicographically smaller boundary representation is returned
/// so that argument order does not affect the result.
///
/// # Type Parameters
///
/// - `T`: Cloneable, totally ordered endpoint type.
/// - `L`: Left range type.
/// - `R`: Right range type.
///
/// # Parameters
///
/// - `left`: First range to enclose.
/// - `right`: Second range to enclose.
///
/// # Returns
///
/// Owned bounds for the minimal enclosing range.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
///
/// use qubit_utils::range::span;
///
/// assert_eq!(
///     span(&(1..5), &(10..=20)),
///     (Included(1), Included(20)),
/// );
/// assert_eq!(span(&(5..5), &(10..20)), (Included(10), Excluded(20)));
/// ```
#[must_use]
pub fn span<T, L, R>(left: &L, right: &R) -> Bounds<T>
where
    T: Ord + Clone,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    match (is_empty(left), is_empty(right)) {
        (true, true) => {
            if compare_range_bounds(left, right) == Ordering::Greater {
                clone_bounds(right)
            } else {
                clone_bounds(left)
            }
        }
        (true, false) => clone_bounds(right),
        (false, true) => clone_bounds(left),
        (false, false) => {
            let lower = if compare_lower_bounds(
                left.start_bound(),
                right.start_bound(),
            ) == Ordering::Greater
            {
                clone_bound(right.start_bound())
            } else {
                clone_bound(left.start_bound())
            };
            let upper =
                if compare_upper_bounds(left.end_bound(), right.end_bound())
                    == Ordering::Less
                {
                    clone_bound(right.end_bound())
                } else {
                    clone_bound(left.end_bound())
                };
            (lower, upper)
        }
    }
}

/// Returns the values lying strictly between two disconnected ranges.
///
/// The result complements the earlier range's upper bound and the later
/// range's lower bound. Empty, overlapping, touching, or otherwise connected
/// inputs have no gap.
///
/// # Type Parameters
///
/// - `T`: Cloneable, totally ordered endpoint type.
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
/// `Some(bounds)` for the nonempty gap between two disconnected ranges, or
/// `None` when either input is empty or their union has no gap.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
///
/// use qubit_utils::range::gap;
///
/// assert_eq!(
///     gap(&(1..2), &(3..4)),
///     Some((Included(2), Excluded(3))),
/// );
/// assert_eq!(gap(&(1..2), &(2..3)), None);
/// ```
#[must_use]
pub fn gap<T, L, R>(left: &L, right: &R) -> Option<Bounds<T>>
where
    T: Ord + Clone,
    L: RangeBounds<T>,
    R: RangeBounds<T>,
{
    if is_empty(left) || is_empty(right) || is_connected(left, right) {
        return None;
    }
    let (earlier_upper, later_lower) =
        if compare_lower_bounds(left.start_bound(), right.start_bound())
            == Ordering::Greater
        {
            (right.end_bound(), left.start_bound())
        } else {
            (left.end_bound(), right.start_bound())
        };
    let lower = complement_upper_bound(earlier_upper)?;
    let upper = complement_lower_bound(later_lower)?;
    Some((lower, upper))
}

/// Converts an earlier range's upper bound into the gap's lower bound.
///
/// # Parameters
///
/// - `bound`: Upper bound immediately before a known nonempty gap.
///
/// # Returns
///
/// `Some(bound)` containing the complemented owned lower bound. `None` is
/// returned for an unbounded upper side, which cannot precede a gap.
fn complement_upper_bound<T>(bound: Bound<&T>) -> Option<Bound<T>>
where
    T: Clone,
{
    match bound {
        Included(value) => Some(Excluded(value.clone())),
        Excluded(value) => Some(Included(value.clone())),
        Unbounded => None,
    }
}

/// Converts a later range's lower bound into the gap's upper bound.
///
/// # Parameters
///
/// - `bound`: Lower bound immediately after a known nonempty gap.
///
/// # Returns
///
/// `Some(bound)` containing the complemented owned upper bound. `None` is
/// returned for an unbounded lower side, which cannot follow a gap.
fn complement_lower_bound<T>(bound: Bound<&T>) -> Option<Bound<T>>
where
    T: Clone,
{
    match bound {
        Included(value) => Some(Excluded(value.clone())),
        Excluded(value) => Some(Included(value.clone())),
        Unbounded => None,
    }
}
