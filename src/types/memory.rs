//! Memory ownership contracts for batches and zero-copy buffers.

use bytes::Bytes;

/// Immutable byte buffer that can be cloned without copying payload data.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZeroCopyBuffer {
    bytes: Bytes,
}

impl ZeroCopyBuffer {
    /// Creates a zero-copy buffer from shared bytes.
    #[must_use]
    pub const fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    /// Returns the underlying shared bytes.
    #[must_use]
    pub const fn bytes(&self) -> &Bytes {
        &self.bytes
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

/// Owned batch storage.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct OwnedBatch<T> {
    items: Vec<T>,
}

impl<T> OwnedBatch<T> {
    /// Creates an owned batch.
    #[must_use]
    pub const fn new(items: Vec<T>) -> Self {
        Self { items }
    }

    /// Returns items as a borrowed slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Converts into the underlying vector.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.items
    }
}

/// Borrowed batch view over contiguous items.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BorrowedBatch<'a, T> {
    items: &'a [T],
}

impl<'a, T> BorrowedBatch<'a, T> {
    /// Creates a borrowed batch.
    #[must_use]
    pub const fn new(items: &'a [T]) -> Self {
        Self { items }
    }

    /// Returns the borrowed items.
    #[must_use]
    pub const fn as_slice(self) -> &'a [T] {
        self.items
    }
}

/// Borrowed view over either owned or borrowed batch storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchView<'a, T> {
    /// View backed by owned storage borrowed for this view lifetime.
    Owned(&'a [T]),
    /// View backed by external borrowed storage.
    Borrowed(&'a [T]),
}

impl<'a, T> BatchView<'a, T> {
    /// Returns the batch items.
    #[must_use]
    pub const fn as_slice(self) -> &'a [T] {
        match self {
            Self::Owned(items) | Self::Borrowed(items) => items,
        }
    }

    /// Returns the number of items in the view.
    #[must_use]
    pub fn len(self) -> usize {
        self.as_slice().len()
    }

    /// Returns true when no items are present.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.as_slice().is_empty()
    }
}
