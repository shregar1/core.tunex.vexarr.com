//! Byte buffer ownership contracts.

use std::{borrow::Cow, sync::Arc};

use bytes::Bytes;
use serde::{Deserialize, Serialize};

use super::{AlignedBuffer, Alignment, BufferView};

/// Memory placement and transfer hint.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum MemoryHint {
    /// No specialized placement requested.
    #[default]
    Default,
    /// CPU hot-path buffer.
    Cpu,
    /// SIMD-friendly buffer.
    Simd,
    /// GPU transfer-ready buffer.
    GpuTransfer,
    /// Checkpoint snapshot buffer.
    Snapshot,
}

/// Immutable shared zero-copy byte buffer.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZeroCopyBuffer {
    bytes: Bytes,
    hint: MemoryHint,
}

impl ZeroCopyBuffer {
    /// Creates a zero-copy buffer from shared bytes.
    #[must_use]
    pub const fn new(bytes: Bytes) -> Self {
        Self {
            bytes,
            hint: MemoryHint::Default,
        }
    }

    /// Creates a zero-copy buffer with a memory hint.
    #[must_use]
    pub const fn with_hint(bytes: Bytes, hint: MemoryHint) -> Self {
        Self { bytes, hint }
    }

    /// Returns the shared bytes.
    #[must_use]
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Returns the memory hint.
    #[must_use]
    pub const fn hint(&self) -> MemoryHint {
        self.hint
    }

    /// Returns a borrowed buffer view.
    #[must_use]
    pub fn view(&self) -> BufferView<'_> {
        BufferView::new(&self.bytes)
    }

    /// Returns an aligned view contract over this buffer.
    #[must_use]
    pub fn aligned(&self, alignment: Alignment) -> AlignedBuffer {
        AlignedBuffer::new(self.bytes.clone(), alignment)
    }

    /// Returns the buffer length in bytes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns true when the buffer is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

/// Mutable owned byte buffer that can be frozen into shared bytes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OwnedBuffer {
    bytes: Vec<u8>,
    hint: MemoryHint,
}

impl OwnedBuffer {
    /// Creates an owned buffer from a vector without an additional copy.
    #[must_use]
    pub fn from_vec(bytes: Vec<u8>) -> Self {
        Self {
            bytes,
            hint: MemoryHint::Default,
        }
    }

    /// Creates an owned buffer from copy-on-write bytes.
    #[must_use]
    pub fn from_cow(bytes: Cow<'_, [u8]>) -> Self {
        match bytes {
            Cow::Borrowed(slice) => Self::from_slice(slice),
            Cow::Owned(vec) => Self::from_vec(vec),
        }
    }

    /// Creates an owned buffer by copying a borrowed slice.
    #[must_use]
    pub fn from_slice(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
            hint: MemoryHint::Default,
        }
    }

    /// Adds a memory hint.
    #[must_use]
    pub const fn with_hint(mut self, hint: MemoryHint) -> Self {
        self.hint = hint;
        self
    }

    /// Returns the owned bytes as a slice.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Freezes this buffer into a shared zero-copy buffer.
    #[must_use]
    pub fn freeze(self) -> ZeroCopyBuffer {
        ZeroCopyBuffer::with_hint(Bytes::from(self.bytes), self.hint)
    }
}

/// Reference-counted typed buffer for compact token or shape arrays.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharedBuffer<T> {
    items: Arc<[T]>,
    hint: MemoryHint,
}

impl<T> SharedBuffer<T> {
    /// Creates a shared typed buffer.
    #[must_use]
    pub fn new(items: impl Into<Arc<[T]>>) -> Self {
        Self {
            items: items.into(),
            hint: MemoryHint::Default,
        }
    }

    /// Creates a shared typed buffer with a memory hint.
    #[must_use]
    pub fn with_hint(items: impl Into<Arc<[T]>>, hint: MemoryHint) -> Self {
        Self {
            items: items.into(),
            hint,
        }
    }

    /// Returns the typed slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Returns the memory hint.
    #[must_use]
    pub const fn hint(&self) -> MemoryHint {
        self.hint
    }

    /// Returns the shared backing storage.
    #[must_use]
    pub fn as_arc(&self) -> &Arc<[T]> {
        &self.items
    }
}
