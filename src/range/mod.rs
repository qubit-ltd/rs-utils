// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Relationships and operations for standard-library ranges.
//!
//! This module complements [`std::ops::RangeBounds`] without introducing a
//! custom range type. Standard range syntax handles the common shapes, while
//! [`Bounds`] represents arbitrary combinations of included, excluded, and
//! unbounded endpoints.
//!
//! # Endpoint-order semantics
//!
//! Range algebra compares endpoint order and bound inclusion. It does not
//! normalize discrete domains or search for representable values between
//! endpoints. A nonempty boundary representation therefore does not guarantee
//! that a value of `T` satisfies the bounds. Connectivity also follows boundary
//! order rather than discrete adjacency. The existing policy for incomparable
//! finite endpoints and unbounded ranges is described by [`is_empty`].
//!
//! For example, no integer lies strictly between 1 and 2, but the boundary pair
//! below is nonempty under this module's semantics:
//!
//! ```
//! use std::ops::Bound::Excluded;
//!
//! use qubit_utils::range::intersection;
//! use qubit_utils::range::is_connected;
//! use qubit_utils::range::is_empty;
//! use qubit_utils::range::overlaps;
//!
//! let open = (Excluded(1_i32), Excluded(2_i32));
//! assert!(!is_empty(&open));
//! assert!(overlaps(&open, &open));
//! assert_eq!(intersection(&open, &open), Some(open));
//! assert!(!is_connected(&(1_i32..=1), &(2_i32..=2)));
//! ```
//!
//! # Examples
//!
//! ```
//! use std::ops::Bound::Excluded;
//! use std::ops::Bound::Included;
//!
//! use qubit_utils::range::Bounds;
//! use qubit_utils::range::overlaps;
//!
//! let open_closed: Bounds<i32> = (Excluded(1), Included(5));
//! assert!(overlaps(&open_closed, &(5..10)));
//! ```

mod bounds;
mod compare;
mod internal;
mod operation;
mod relation;

pub use bounds::Bounds;
pub use compare::compare;
pub use operation::gap;
pub use operation::intersection;
pub use operation::span;
pub use relation::encloses;
pub use relation::is_connected;
pub use relation::is_empty;
pub use relation::overlaps;
