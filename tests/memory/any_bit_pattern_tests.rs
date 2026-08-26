// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_utils::AnyBitPattern;

fn assert_any_bit_pattern<T: AnyBitPattern>() {}

#[test]
fn test_any_bit_pattern_supports_audited_numeric_scalars() {
    assert_any_bit_pattern::<u8>();
    assert_any_bit_pattern::<u16>();
    assert_any_bit_pattern::<u32>();
    assert_any_bit_pattern::<u64>();
    assert_any_bit_pattern::<u128>();
    assert_any_bit_pattern::<usize>();
    assert_any_bit_pattern::<i8>();
    assert_any_bit_pattern::<i16>();
    assert_any_bit_pattern::<i32>();
    assert_any_bit_pattern::<i64>();
    assert_any_bit_pattern::<i128>();
    assert_any_bit_pattern::<isize>();
    assert_any_bit_pattern::<f32>();
    assert_any_bit_pattern::<f64>();
}
