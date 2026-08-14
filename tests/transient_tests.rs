// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use qubit_utils::Transient;

#[test]
fn test_transient_ignores_inner_value_for_equality_and_hash() {
    let first = Transient::new(1_u32);
    let second = Transient::new(2_u32);

    assert_eq!(first, second);

    let mut first_hasher = DefaultHasher::new();
    first.hash(&mut first_hasher);
    let mut second_hasher = DefaultHasher::new();
    second.hash(&mut second_hasher);
    assert_eq!(first_hasher.finish(), second_hasher.finish());
}

#[test]
fn test_transient_preserves_inner_value_through_access_and_clone() {
    let mut value = Transient::new(String::from("runtime"));
    assert_eq!(value.get(), "runtime");
    value.get_mut().push_str(" state");

    let cloned = value.clone();
    assert_eq!(value.into_inner(), "runtime state");
    assert_eq!(cloned.into_inner(), "runtime state");
}

#[test]
fn test_transient_supports_conversion_and_reference_traits() {
    let mut from_value: Transient<String> = String::from("from").into();
    assert_eq!(from_value.as_ref(), "from");
    from_value.as_mut().push_str(" value");
    assert_eq!(from_value.get(), "from value");

    let default_value = Transient::<String>::default();
    assert_eq!(default_value.get(), "");
}
