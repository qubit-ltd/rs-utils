// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::RangeBounds;

use qubit_utils::range::Bounds;

#[test]
fn test_bounds_implements_standard_range_bounds() {
    let range: Bounds<i32> = (Excluded(1), Included(5));
    assert!(!range.contains(&1));
    assert!(range.contains(&5));
}
