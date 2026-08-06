// =============================================================================
//    Copyright (c) 2025 - 2026 Haixing Hu.
//
//    SPDX-License-Identifier: Apache-2.0
//
//    Licensed under the Apache License, Version 2.0.
// =============================================================================
//! Runtime-only values whose contents do not participate in value semantics.

use std::hash::{
    Hash,
    Hasher,
};

/// Stores runtime state that is intentionally excluded from a parent value's
/// equality and hash semantics.
///
/// `Transient<T>` keeps its inner value for runtime use, cloning, and moving,
/// but all values of the same `Transient<T>` type compare equal and contribute
/// no bytes to a hash. This makes it suitable for caches, policies, handles,
/// and other derived runtime state that must not change the identity of the
/// containing value.
///
/// Typical scenarios:
///
/// - request contexts that cache computed values (`HashMap`, `Vec`, etc.),
/// - non-semantic feature toggles (`debug`, `hot`, `trace`) affecting runtime
///   behavior but not identity,
/// - ephemeral handles (`Arc`, file descriptors, counters) that should not
///   break equality semantics.
///
/// The container does not implement Serde's `Serialize` or `Deserialize`.
/// A parent type using Serde derive should mark a transient field with
/// `#[serde(skip, default)]`; wire formats should not include it implicitly.
/// The inner value is therefore excluded from persistence unless serialized
/// manually.
///
/// # Examples
///
/// ```
/// use qubit_utils::Transient;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Debug, Default, Serialize, Deserialize)]
/// struct RuntimeCache;
///
/// #[derive(Debug, Serialize, Deserialize)]
/// struct Settings {
///     name: String,
///     #[serde(skip, default)]
///     cache: Transient<RuntimeCache>,
/// }
///
/// let first = Transient::new(1_u32);
/// let second = Transient::new(2_u32);
/// assert_eq!(first, second);
/// assert_ne!(first.get(), second.get());
///
/// let settings = Settings {
///     name: String::from("demo"),
///     cache: Transient::new(RuntimeCache),
/// };
/// let encoded = serde_json::to_string(&settings).expect("settings serialize");
/// assert_eq!(encoded, r#"{"name":"demo"}"#);
/// ```
///
/// ```rust
/// use qubit_utils::Transient;
///
/// #[derive(Clone, Default)]
/// struct DecodeState {
///     bytes: usize,
/// }
///
/// #[derive(Default)]
/// struct Decoder {
///     // Not part of logical decoder identity.
///     stats: Transient<DecodeState>,
/// }
///
/// impl Decoder {
///     fn read_chunk(&mut self, chunk: usize) {
///         self.stats.get_mut().bytes += chunk;
///     }
/// }
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct Transient<T> {
    value: T,
}

impl<T> Transient<T> {
    /// Wraps `value` as runtime-only state.
    #[inline]
    pub const fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns a shared reference to the runtime-only value.
    #[inline]
    pub const fn get(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the runtime-only value.
    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Unwraps the runtime-only value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> AsRef<T> for Transient<T> {
    /// Borrows the wrapped runtime-only value.
    #[inline]
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for Transient<T> {
    /// Mutably borrows the wrapped runtime-only value.
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}

impl<T> Default for Transient<T>
where
    T: Default,
{
    /// Creates a transient value from `T::default()`.
    #[inline]
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> From<T> for Transient<T> {
    /// Wraps a value as transient runtime state.
    #[inline]
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> PartialEq for Transient<T> {
    /// Treats all transient values of the same type as equal.
    #[inline]
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl<T> Eq for Transient<T> {}

impl<T> Hash for Transient<T> {
    /// Excludes the runtime-only value from the hash.
    #[inline]
    fn hash<H>(&self, _state: &mut H)
    where
        H: Hasher,
    {
    }
}
