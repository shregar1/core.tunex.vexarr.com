//! Alignment contracts for SIMD and accelerator transfer boundaries.

use bytes::Bytes;
use serde::{Deserialize, Serialize};

/// Required byte alignment for a buffer.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Alignment(usize);

impl Alignment {
    /// One-byte alignment.
    pub const ONE: Self = Self(1);
    /// Common cache-line alignment.
    pub const CACHE_LINE: Self = Self(64);
    /// Common page alignment.
    pub const PAGE: Self = Self(4096);

    /// Creates an alignment when the value is a non-zero power of two.
    #[must_use]
    pub const fn new(value: usize) -> Option<Self> {
        if value != 0 && value.is_power_of_two() {
            Some(Self(value))
        } else {
            None
        }
    }

    /// Returns the alignment in bytes.
    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }

    /// Returns whether a pointer address satisfies this alignment.
    #[must_use]
    pub const fn is_aligned(self, address: usize) -> bool {
        address % self.0 == 0
    }
}

impl Default for Alignment {
    fn default() -> Self {
        Self::ONE
    }
}

/// Shared byte buffer annotated with a required alignment.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct AlignedBuffer {
    bytes: Bytes,
    alignment: Alignment,
}

impl AlignedBuffer {
    /// Creates an aligned buffer contract from shared bytes and an alignment hint.
    #[must_use]
    pub const fn new(bytes: Bytes, alignment: Alignment) -> Self {
        Self { bytes, alignment }
    }

    /// Returns the shared bytes.
    #[must_use]
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Returns the required alignment.
    #[must_use]
    pub const fn alignment(&self) -> Alignment {
        self.alignment
    }

    /// Returns true when the current backing pointer satisfies the alignment.
    #[must_use]
    pub fn is_currently_aligned(&self) -> bool {
        self.alignment.is_aligned(self.bytes.as_ptr() as usize)
    }
}
