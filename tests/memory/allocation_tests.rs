// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
use std::collections::TryReserveError;

use qubit_utils::allocation_error;
use qubit_utils::create_vec;
use qubit_utils::try_reserve_string;
use qubit_utils::try_reserve_vec;

#[test]
fn test_try_reserve_vec_preserves_try_reserve_error() {
    let mut output = Vec::<u8>::new();

    let error: TryReserveError = try_reserve_vec(&mut output, usize::MAX)
        .expect_err("capacity overflow should return TryReserveError");

    assert!(!error.to_string().is_empty());
    assert!(output.is_empty());
}

#[test]
fn test_try_reserve_string_preserves_try_reserve_error() {
    let mut output = String::new();

    let error: TryReserveError = try_reserve_string(&mut output, usize::MAX)
        .expect_err("capacity overflow should return TryReserveError");

    assert!(!error.to_string().is_empty());
    assert!(output.is_empty());
}

#[test]
fn test_create_vec_initializes_requested_length() {
    let values =
        create_vec(3, 7_u8).expect("small vector allocation should succeed");

    assert_eq!(values, vec![7, 7, 7]);
}

#[test]
fn test_create_vec_accepts_zero_length() {
    let values =
        create_vec::<u8>(0, 7).expect("zero-length vector should succeed");

    assert!(values.is_empty());
}

#[test]
fn test_allocation_error_maps_try_reserve_failure() {
    let mut output = Vec::<u8>::new();
    let error = try_reserve_vec(&mut output, usize::MAX)
        .expect_err("capacity overflow should fail");
    let io_error = allocation_error(error);

    assert_eq!(io_error.kind(), std::io::ErrorKind::OutOfMemory);
    assert!(io_error.get_ref().is_some());
}

#[cfg(coverage)]
mod coverage_tests {
    use std::io::ErrorKind;

    use qubit_utils::allocation_error;
    use qubit_utils::coverage_fail_next_reserve;
    use qubit_utils::coverage_fail_next_string_reserve;
    use qubit_utils::coverage_fail_reserve_above;
    use qubit_utils::coverage_fail_reserve_after;
    use qubit_utils::coverage_reset_reserve_hooks;
    use qubit_utils::create_vec;
    use qubit_utils::try_reserve_string;
    use qubit_utils::try_reserve_vec;

    fn reset_hooks() {
        coverage_reset_reserve_hooks();
    }

    #[test]
    fn test_allocation_helpers_cover_success_and_error_paths() {
        reset_hooks();

        let mut values = Vec::<u8>::new();
        try_reserve_vec(&mut values, 1)
            .expect("ordinary vector reserve should succeed");
        let mut text = String::new();
        try_reserve_string(&mut text, 1)
            .expect("ordinary string reserve should succeed");

        let values = create_vec(2, 7_u8)
            .expect("ordinary vector creation should succeed");
        assert_eq!(values, vec![7, 7]);

        coverage_fail_next_reserve();
        let error = try_reserve_vec(&mut Vec::<u8>::new(), 1)
            .expect_err("the next vector reserve should fail");
        assert_eq!(ErrorKind::OutOfMemory, allocation_error(error).kind());

        coverage_fail_next_reserve();
        let error = create_vec::<u8>(1, 0)
            .expect_err("vector creation should propagate failure");
        assert_eq!(ErrorKind::OutOfMemory, error.kind());

        coverage_fail_reserve_above(1);
        let error = try_reserve_vec(&mut Vec::<u8>::new(), 2).expect_err(
            "oversized reserve should fail under the coverage hook",
        );
        assert_eq!(ErrorKind::OutOfMemory, allocation_error(error).kind());

        coverage_fail_reserve_after(1);
        try_reserve_vec(&mut Vec::<u8>::new(), 1)
            .expect("the first reserve should pass under the delayed hook");
        let error = try_reserve_vec(&mut Vec::<u8>::new(), 1)
            .expect_err("the delayed reserve should fail");
        assert_eq!(ErrorKind::OutOfMemory, allocation_error(error).kind());

        reset_hooks();
    }

    #[test]
    fn test_string_reserve_hooks_fail_once_and_reset() {
        reset_hooks();
        let mut text = String::new();

        coverage_fail_next_string_reserve();
        assert!(try_reserve_string(&mut text, 1).is_err());
        try_reserve_string(&mut text, 1)
            .expect("string hook should reset after one failure");

        coverage_fail_reserve_above(1);
        assert!(try_reserve_string(&mut text, 2).is_err());

        reset_hooks();
        try_reserve_string(&mut text, 1)
            .expect("reset should restore ordinary reserves");
    }
}
