//! General-purpose utilities for Rust projects.
//!
//! This initial release provides the crate foundation. Concrete utilities will
//! be added as their requirements and public contracts are established.

mod string_utils;

pub use string_utils::is_blank;
