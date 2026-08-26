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
