// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_utils::SliceRange;

#[test]
fn test_range_fits_checks_range() {
    assert!(SliceRange::range_fits(8, 2, 6));
    assert!(!SliceRange::range_fits(8, 3, 6));
}

#[test]
fn test_range_end_returns_exclusive_end_index() {
    assert_eq!(SliceRange::range_end(8, 2, 6), Some(8));
    assert_eq!(SliceRange::range_end(8, 3, 6), None);
    assert_eq!(SliceRange::range_end(8, usize::MAX, 1), None);
}

#[test]
fn test_checked_range_end_returns_io_error() {
    let error = SliceRange::checked_range_end(8, 3, 6, "range exceeds buffer")
        .expect_err("invalid range should return an I/O error");

    assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput);
    assert_eq!(error.to_string(), "range exceeds buffer");
}

#[test]
fn test_range_validation_rejects_overflow() {
    assert!(!SliceRange::range_fits(4, usize::MAX, 1));
}

#[test]
fn test_index_saturating_add_checks_overflow() {
    assert_eq!(10usize.saturating_add(2), 12);
    assert_eq!(usize::MAX.saturating_add(1), usize::MAX);
}
