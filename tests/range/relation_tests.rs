// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;

use qubit_utils::range::Bounds;
use qubit_utils::range::encloses;
use qubit_utils::range::is_connected;
use qubit_utils::range::is_empty;
use qubit_utils::range::overlaps;

#[test]
fn test_is_empty_recognizes_standard_and_arbitrary_bounds() {
    assert!(is_empty(&(5..5)));
    assert!(!is_empty(&(5..=5)));
    assert!(is_empty(&(Excluded(5), Excluded(5))));
    assert!(is_empty(&(Excluded(6), Included(5))));
    let all: Bounds<i32> = (Unbounded, Unbounded);
    assert!(!is_empty(&all));
}

#[test]
fn test_is_empty_treats_incomparable_finite_endpoints_as_empty() {
    assert!(is_empty(&(0.0..f64::NAN)));
    assert!(is_empty(&(f64::NAN..=1.0)));
}

#[test]
fn test_encloses_handles_bounded_and_unbounded_ranges() {
    assert!(encloses(&(1..=10), &(3..5)));
    assert!(!encloses(&(3..5), &(1..=10)));
    assert!(encloses(&(..), &(3..5)));
    assert!(encloses(&(1..), &(3..)));
    assert!(!encloses(&(3..), &(1..)));
}

#[test]
fn test_encloses_uses_set_semantics_for_empty_ranges() {
    let empty: Bounds<i32> = (Included(5), Excluded(5));
    assert!(encloses(&(10..20), &empty));
    assert!(encloses(&empty, &(7..7)));
    assert!(!encloses(&empty, &(10..20)));
}

#[test]
fn test_overlaps_requires_a_nonempty_shared_value() {
    assert!(overlaps(&(1..=2), &(2..3)));
    assert!(overlaps(&(2..3), &(1..=2)));
    assert!(!overlaps(&(1..2), &(2..3)));
    assert!(!overlaps(&(1..10), &(4..4)));
    assert!(overlaps(&(..=0), &(0..)));
}

#[test]
fn test_is_connected_distinguishes_touching_bound_types() {
    assert!(is_connected(&(1..2), &(2..3)));
    assert!(is_connected(&(1..=2), &(Excluded(2), Included(3))));
    assert!(is_connected(&(Excluded(2), Included(3)), &(1..=2)));
    assert!(!is_connected(
        &(Included(1), Excluded(2)),
        &(Excluded(2), Included(3)),
    ));
}

#[test]
fn test_is_connected_uses_set_semantics_for_empty_ranges() {
    let empty: Bounds<i32> = (Included(5), Excluded(5));
    assert!(is_connected(&empty, &(10..20)));
    assert!(is_connected(&(10..20), &empty));
}
