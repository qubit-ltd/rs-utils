// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_utils::AnyBitPattern;

fn assert_sealed_scalar<T: AnyBitPattern>() {}

#[test]
fn test_sealed_marker_accepts_only_the_audited_scalar_set() {
    assert_sealed_scalar::<u32>();
    assert_sealed_scalar::<i64>();
    assert_sealed_scalar::<f32>();
    assert_sealed_scalar::<f64>();
}
