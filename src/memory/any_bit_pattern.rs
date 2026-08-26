// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Defines the sealed scalar marker used by byte-level slice operations.

use crate::internal::sealed::Sealed;

/// Marks scalar types whose complete object representation is initialized and
/// for which every bit pattern is a valid value.
///
/// This trait is sealed and cannot be implemented outside `qubit-utils`.
/// Implementations are limited to integer and floating-point primitives whose
/// representations have no invalid bit patterns or uninitialized padding.
///
/// `char` is intentionally not implemented: its four-byte representation must
/// contain a valid Unicode scalar value, so bit patterns in the surrogate range
/// and values greater than `0x10FFFF` are invalid. `bool` accepts only `0` and
/// `1`, and `NonZero*` types reject their all-zero representation. References
/// and pointers additionally require valid provenance, alignment, and lifetime
/// invariants. User-defined types may contain padding or fields with restricted
/// representations, even when they implement [`Copy`].
///
/// External implementations are rejected because this trait is sealed:
///
/// ```compile_fail
/// use qubit_utils::AnyBitPattern;
///
/// #[derive(Clone, Copy)]
/// struct ExternalScalar(u32);
///
/// unsafe impl AnyBitPattern for ExternalScalar {}
/// ```
///
/// # Safety
///
/// An implementation must guarantee that every bit pattern of the type's
/// storage representation is a valid, fully initialized value and that the
/// representation contains no uninitialized padding. The trait is sealed so
/// that this invariant can only be established for the audited primitive types
/// below.
pub unsafe trait AnyBitPattern: Copy + Sealed {
    // empty
}

macro_rules! impl_any_bit_pattern {
    ($($type:ty),+ $(,)?) => {
        $(
            impl Sealed for $type {}

            // SAFETY: Every bit pattern of these primitive numeric types is a
            // valid fully initialized value without padding bytes.
            unsafe impl AnyBitPattern for $type {}
        )+
    };
}

impl_any_bit_pattern!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64,
);
