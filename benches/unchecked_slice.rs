// =============================================================================
//    Copyright (c) 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Benchmarks safe and unchecked indexed reads over variable-length buffers.

use std::hint::black_box;

use criterion::Criterion;
use criterion::Throughput;
use criterion::criterion_group;
use criterion::criterion_main;
use qubit_utils::UncheckedSlice;

const BUFFER_COUNT: usize = 256;
const ACCESSES_PER_BUFFER: usize = 256;
const MIN_BUFFER_LEN: usize = 1;
const MAX_BUFFER_LEN: usize = 16 * 1024;
const RANDOM_SEED: u64 = 0x4D595DF4D0F33173;

/// A prepared buffer and the valid, non-sequential offsets read from it.
struct ReadCase {
    buffer: Vec<u8>,
    indices: Vec<usize>,
}

/// Advances the deterministic pseudo-random generator used for setup.
#[inline]
fn next_random(state: &mut u64) -> u64 {
    *state ^= *state << 7;
    *state ^= *state >> 9;
    *state
}

/// Creates variable-length buffers and valid random offsets outside timing.
fn build_workload() -> Vec<ReadCase> {
    let mut state = RANDOM_SEED;
    let mut workload = Vec::with_capacity(BUFFER_COUNT);

    for _ in 0..BUFFER_COUNT {
        let span = MAX_BUFFER_LEN - MIN_BUFFER_LEN + 1;
        let len = MIN_BUFFER_LEN + (next_random(&mut state) as usize % span);
        let mut buffer = Vec::with_capacity(len);
        for index in 0..len {
            buffer.push((next_random(&mut state) as u8).wrapping_add(index as u8));
        }

        let mut indices = Vec::with_capacity(ACCESSES_PER_BUFFER);
        for _ in 0..ACCESSES_PER_BUFFER {
            indices.push(next_random(&mut state) as usize % len);
        }

        workload.push(ReadCase { buffer, indices });
    }

    workload
}

/// Reads each prepared offset with ordinary bounds-checked indexing.
#[inline(never)]
fn read_safe(workload: &[ReadCase]) -> u64 {
    let mut checksum = 0_u64;
    for case in workload {
        for &index in &case.indices {
            checksum = checksum.wrapping_add(u64::from(case.buffer[index]));
        }
    }
    checksum
}

/// Reads each prepared offset through the unchecked slice helper.
#[inline(never)]
fn read_unchecked(workload: &[ReadCase]) -> u64 {
    let mut checksum = 0_u64;
    for case in workload {
        for &index in &case.indices {
            // SAFETY: `build_workload` derives every index modulo its buffer
            // length, and the workload is immutable during the benchmark.
            let value = unsafe { UncheckedSlice::read(&case.buffer, index) };
            checksum = checksum.wrapping_add(u64::from(value));
        }
    }
    checksum
}

/// Benchmarks indexed reads over the same set of randomly sized buffers.
fn bench_variable_length_reads(criterion: &mut Criterion) {
    let workload = build_workload();
    let access_count = (BUFFER_COUNT * ACCESSES_PER_BUFFER) as u64;
    let mut group = criterion.benchmark_group("unchecked_slice_variable_lengths");
    group.throughput(Throughput::Elements(access_count));

    group.bench_function("safe_indexing", |bencher| {
        bencher.iter(|| black_box(read_safe(black_box(&workload))));
    });

    group.bench_function("unchecked_read", |bencher| {
        bencher.iter(|| black_box(read_unchecked(black_box(&workload))));
    });

    group.finish();
}

criterion_group!(benches, bench_variable_length_reads);
criterion_main!(benches);
