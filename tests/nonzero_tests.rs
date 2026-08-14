// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use core::num::NonZeroUsize;

use qubit_utils::nonzero;

const THREE: NonZeroUsize = nonzero(3);

/// Tests that the non-zero helpers work in const and runtime expressions.
#[test]
fn test_nonzero_supports_const_and_runtime_calls() {
    assert_eq!(THREE.get(), 3);
    assert_eq!(nonzero(7).get(), 7);
    assert_eq!(nonzero(9).get(), 9);
}

/// Tests that the non-zero macro rejects zero.
#[test]
#[should_panic(expected = "value must be non-zero")]
fn test_nonzero_rejects_zero() {
    let _ = nonzero(0);
}
