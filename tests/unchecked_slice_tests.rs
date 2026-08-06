// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================

use qubit_utils::UncheckedSlice;

#[test]
fn test_read_unchecked_reads_value() {
    let input = [1_u8, 2, 3];
    assert_eq!(unsafe { UncheckedSlice::read(&input, 1) }, 2);
}

#[test]
fn test_write_unchecked_writes_value() {
    let mut output = [1_u8, 2, 3];
    unsafe { UncheckedSlice::write(&mut output, 1, 9) };
    assert_eq!(output, [1, 9, 3]);
}

#[test]
fn test_write_unchecked_moves_non_copy_value() {
    let mut output = [
        String::from("left"),
        String::from("middle"),
        String::from("right"),
    ];
    unsafe {
        UncheckedSlice::write(&mut output, 1, String::from("updated"));
    }
    assert_eq!(output[1], "updated");
}

#[test]
fn test_ref_unchecked_returns_reference() {
    let input = [4_u16, 5, 6];
    assert_eq!(unsafe { *UncheckedSlice::get(&input, 2) }, 6);
}

#[test]
fn test_mut_unchecked_writes_reference() {
    let mut output = [10_u32, 20, 30];
    unsafe {
        *UncheckedSlice::get_mut(&mut output, 0) = 12_345;
    }
    assert_eq!(output[0], 12_345);
}

#[test]
fn test_ne_unaligned_unchecked_reads_and_writes() {
    let mut output = [0_u8; 8];
    // SAFETY: Writes a little-endian u16 to valid unaligned offset 1.
    unsafe {
        UncheckedSlice::write_ne_unaligned(&mut output, 1, 0x1234_u16);
        let value = UncheckedSlice::read_ne_unaligned::<u16>(&output, 1);
        assert_eq!(value, 0x1234_u16);
    }
    let expected = 0x1234_u16.to_ne_bytes();
    assert_eq!(&output[1..3], expected.as_slice());
}

#[test]
fn test_subslice_returns_range() {
    let input = [1_u8, 2, 3, 4, 5];
    let slice = unsafe { UncheckedSlice::subslice(&input, 1, 3) };
    assert_eq!(slice, &[2, 3, 4]);
}

#[test]
fn test_subslice_mut_returns_mutable_range() {
    let mut output = [1_u8, 2, 3, 4, 5];
    let slice = unsafe { UncheckedSlice::subslice_mut(&mut output, 2, 2) };
    slice.copy_from_slice(&[8, 9]);
    assert_eq!(output, [1, 2, 8, 9, 5]);
}

#[test]
fn test_copy_nonoverlapping_unchecked_copies_slice() {
    let source = [1_u8, 2, 3, 4];
    let mut destination = [0_u8, 0, 0, 0];
    unsafe {
        UncheckedSlice::copy_nonoverlapping(&source, 0, &mut destination, 0, 4);
    }
    assert_eq!(destination, source);
}

#[test]
fn test_copy_unchecked_copies_overlapping_range() {
    let mut buffer = [0_u8, 1, 2, 3, 4, 5, 6, 7];
    unsafe {
        UncheckedSlice::copy_within(&mut buffer, 2, 0, 4);
    }
    assert_eq!(buffer, [2, 3, 4, 5, 4, 5, 6, 7]);
}
