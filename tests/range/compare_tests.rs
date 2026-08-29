// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::cmp::Ordering;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;

use qubit_utils::range::Bounds;
use qubit_utils::range::compare;

#[test]
fn test_compare_orders_lower_then_upper_bounds() {
    assert_eq!(compare(&(..5), &(1..5)), Ordering::Less);
    assert_eq!(
        compare(&(Included(1), Excluded(5)), &(Excluded(1), Excluded(5)),),
        Ordering::Less,
    );
    assert_eq!(compare(&(1..5), &(1..=5)), Ordering::Less);
    let left: Bounds<i32> = (Included(1), Unbounded);
    let right: Bounds<i32> = (Included(1), Unbounded);
    assert_eq!(compare(&left, &right), Ordering::Equal);
}

#[test]
fn test_compare_can_sort_owned_arbitrary_ranges() {
    let mut ranges: Vec<Bounds<i32>> = vec![
        (Excluded(1), Included(5)),
        (Included(1), Unbounded),
        (Unbounded, Excluded(5)),
        (Excluded(1), Excluded(5)),
        (Included(1), Excluded(5)),
    ];
    ranges.sort_by(compare);
    assert_eq!(
        ranges,
        vec![
            (Unbounded, Excluded(5)),
            (Included(1), Excluded(5)),
            (Included(1), Unbounded),
            (Excluded(1), Excluded(5)),
            (Excluded(1), Included(5)),
        ],
    );
}

#[test]
fn test_compare_is_antisymmetric_and_transitive_over_small_boundary_domain() {
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
            assert_eq!(compare(left, right), compare(right, left).reverse());
            for last in &ranges {
                if compare(left, right) != Ordering::Greater && compare(right, last) != Ordering::Greater {
                    assert_ne!(compare(left, last), Ordering::Greater);
                }
            }
        }
    }
}
