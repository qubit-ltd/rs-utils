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
use qubit_utils::range::gap;
use qubit_utils::range::intersection;
use qubit_utils::range::overlaps;
use qubit_utils::range::span;

#[test]
fn test_intersection_returns_owned_nonempty_bounds() {
    assert_eq!(
        intersection(&(1..10), &(5..=20)),
        Some((Included(5), Excluded(10))),
    );
    assert_eq!(intersection(&(1..5), &(5..10)), None);
}

#[test]
fn test_intersection_supports_unbounded_and_arbitrary_bounds() {
    let left: Bounds<i32> = (Unbounded, Included(10));
    let right: Bounds<i32> = (Excluded(5), Unbounded);
    assert_eq!(
        intersection(&left, &right),
        Some((Excluded(5), Included(10))),
    );
}

#[test]
fn test_intersection_is_symmetric_and_enclosed_by_both_inputs() {
    let left = 1..=10;
    let right = 5..20;
    let forward = intersection(&left, &right)
        .expect("the selected ranges should have a nonempty intersection");
    let reverse = intersection(&right, &left)
        .expect("intersection should be symmetric for the same ranges");
    assert_eq!(forward, reverse);
    assert!(encloses(&left, &forward));
    assert!(encloses(&right, &forward));
}

#[test]
fn test_span_returns_minimal_enclosing_bounds() {
    let right: Bounds<i32> = (Excluded(10), Unbounded);
    assert_eq!(span(&(1..5), &right), (Included(1), Unbounded));
    assert_eq!(span(&right, &(1..5)), (Included(1), Unbounded));
}

#[test]
fn test_span_ignores_empty_inputs_and_is_deterministic_for_two_empty_ranges() {
    let first_empty = 7..7;
    let second_empty = 5..5;
    assert_eq!(span(&first_empty, &(10..20)), (Included(10), Excluded(20)));
    assert_eq!(span(&(10..20), &first_empty), (Included(10), Excluded(20)));
    assert_eq!(
        span(&first_empty, &second_empty),
        (Included(5), Excluded(5)),
    );
    assert_eq!(
        span(&second_empty, &first_empty),
        (Included(5), Excluded(5)),
    );
}

#[test]
fn test_gap_preserves_missing_endpoint_inclusivity() {
    let left: Bounds<i32> = (Included(1), Excluded(2));
    let right: Bounds<i32> = (Excluded(2), Included(3));
    assert_eq!(gap(&left, &right), Some((Included(2), Included(2))));
    assert_eq!(gap(&right, &left), Some((Included(2), Included(2))));
    assert_eq!(gap(&(1..2), &(3..4)), Some((Included(2), Excluded(3))));
}

#[test]
fn test_gap_returns_none_for_connected_or_empty_inputs() {
    assert_eq!(gap(&(1..2), &(2..3)), None);
    assert_eq!(gap(&(1..=5), &(3..10)), None);
    assert_eq!(gap(&(4..4), &(10..20)), None);
}

#[test]
fn test_operation_results_obey_range_relationships() {
    let left = 1..5;
    let right = 7..=10;
    let combined = span(&left, &right);
    let separation = gap(&left, &right)
        .expect("disconnected nonempty ranges should have a gap");
    assert!(encloses(&combined, &left));
    assert!(encloses(&combined, &right));
    assert!(!overlaps(&separation, &left));
    assert!(!overlaps(&separation, &right));
}

#[test]
fn test_operations_preserve_algebraic_properties_over_small_boundary_domain() {
    let ranges: Vec<Bounds<i32>> = vec![
        (Unbounded, Unbounded),
        (Unbounded, Excluded(0)),
        (Unbounded, Included(0)),
        (Included(0), Excluded(0)),
        (Included(0), Included(0)),
        (Excluded(0), Excluded(1)),
        (Excluded(0), Included(1)),
        (Included(1), Unbounded),
        (Excluded(1), Unbounded),
    ];
    for left in &ranges {
        for right in &ranges {
            assert_eq!(intersection(left, right), intersection(right, left));
            assert_eq!(span(left, right), span(right, left));
            assert_eq!(gap(left, right), gap(right, left));
            if let Some(common) = intersection(left, right) {
                assert!(encloses(left, &common));
                assert!(encloses(right, &common));
            }
            let combined = span(left, right);
            assert!(encloses(&combined, left));
            assert!(encloses(&combined, right));
            if let Some(separation) = gap(left, right) {
                assert!(!overlaps(&separation, left));
                assert!(!overlaps(&separation, right));
            }
        }
    }
}
