// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Owned standard-library bounds used by range operation results.

/// An owned pair of standard-library bounds representing one range.
///
/// The first element is the lower bound and the second is the upper bound.
/// The standard library implements [`std::ops::RangeBounds`] for this tuple,
/// so values can be passed directly to ordered collection range queries.
/// Range operations interpret this pair using the module's
/// [endpoint-order semantics](crate::range#endpoint-order-semantics); for a
/// discrete endpoint domain, a nonempty pair does not guarantee a matching
/// value exists.
///
/// # Type Parameters
///
/// - `T`: Endpoint type stored by both bounds.
///
/// # Examples
///
/// ```
/// use std::ops::Bound::Excluded;
/// use std::ops::Bound::Included;
/// use std::ops::RangeBounds;
///
/// use qubit_utils::range::Bounds;
///
/// let range: Bounds<i32> = (Excluded(1), Included(5));
/// assert!(range.contains(&5));
/// assert!(!range.contains(&1));
/// ```
pub type Bounds<T> = (std::ops::Bound<T>, std::ops::Bound<T>);
