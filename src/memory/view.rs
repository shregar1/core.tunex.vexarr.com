//! Borrowed buffer views.

use bytes::Bytes;

/// Borrowed immutable view over a byte buffer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BufferView<'a> {
    bytes: &'a [u8],
}

impl<'a> BufferView<'a> {
    /// Creates a borrowed buffer view from bytes.
    #[must_use]
    pub fn new(bytes: &'a Bytes) -> Self {
        Self {
            bytes: bytes.as_ref(),
        }
    }

    /// Creates a borrowed buffer view from a slice.
    #[must_use]
    pub const fn from_slice(bytes: &'a [u8]) -> Self {
        Self { bytes }
    }

    /// Returns the view bytes.
    #[must_use]
    pub const fn as_slice(self) -> &'a [u8] {
        self.bytes
    }

    /// Returns a sub-view without cloning bytes.
    #[must_use]
    pub fn slice(self, range: std::ops::Range<usize>) -> Option<Self> {
        self.bytes.get(range).map(Self::from_slice)
    }

    /// Reinterprets the view as a typed slice when alignment and size permit.
    ///
    /// # Errors
    ///
    /// Returns an error when the byte view cannot be safely cast to `T`.
    pub fn try_cast<T: bytemuck::Pod>(self) -> Result<&'a [T], bytemuck::PodCastError> {
        bytemuck::try_cast_slice(self.bytes)
    }

    /// Returns the view length.
    #[must_use]
    pub const fn len(self) -> usize {
        self.bytes.len()
    }

    /// Returns true when the view is empty.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.bytes.is_empty()
    }
}
