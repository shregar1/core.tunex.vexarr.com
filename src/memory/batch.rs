//! Typed batch ownership and view contracts.

use std::{borrow::Cow, sync::Arc};

use smallvec::SmallVec;

use super::MemoryHint;

/// Owned contiguous batch storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedBatch<T> {
    items: Arc<[T]>,
    hint: MemoryHint,
}

impl<T> OwnedBatch<T> {
    /// Creates an owned batch from reference-counted contiguous storage.
    #[must_use]
    pub fn new(items: impl Into<Arc<[T]>>) -> Self {
        Self {
            items: items.into(),
            hint: MemoryHint::Default,
        }
    }

    /// Creates an owned batch with a memory hint.
    #[must_use]
    pub fn with_hint(items: impl Into<Arc<[T]>>, hint: MemoryHint) -> Self {
        Self {
            items: items.into(),
            hint,
        }
    }

    /// Creates an owned batch from copy-on-write storage.
    #[must_use]
    pub fn from_cow(items: Cow<'_, [T]>) -> Self
    where
        T: Clone,
    {
        match items {
            Cow::Borrowed(slice) => Self::new(Arc::<[T]>::from(slice)),
            Cow::Owned(vec) => Self::new(Arc::<[T]>::from(vec)),
        }
    }

    /// Returns a borrowed batch view.
    #[must_use]
    pub fn view(&self) -> BatchView<'_, T> {
        BatchView::Owned(&self.items)
    }

    /// Returns the underlying slice.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.items
    }

    /// Returns the shared backing storage.
    #[must_use]
    pub fn as_arc(&self) -> &Arc<[T]> {
        &self.items
    }

    /// Returns the memory hint.
    #[must_use]
    pub const fn hint(&self) -> MemoryHint {
        self.hint
    }
}

impl<T> Default for OwnedBatch<T> {
    fn default() -> Self {
        Self {
            items: Arc::from([]),
            hint: MemoryHint::Default,
        }
    }
}

/// Borrowed contiguous batch storage.
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

    /// Returns the borrowed slice.
    #[must_use]
    pub const fn as_slice(self) -> &'a [T] {
        self.items
    }

    /// Returns a borrowed sub-batch without cloning.
    #[must_use]
    pub fn slice(self, range: std::ops::Range<usize>) -> Option<Self> {
        self.items.get(range).map(Self::new)
    }
}

/// Borrowed view over batch storage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatchView<'a, T> {
    /// View backed by owned storage borrowed for this view lifetime.
    Owned(&'a [T]),
    /// View backed by external borrowed storage.
    Borrowed(&'a [T]),
}

impl<'a, T> BatchView<'a, T> {
    /// Creates a view over owned storage.
    #[must_use]
    pub const fn owned(items: &'a [T]) -> Self {
        Self::Owned(items)
    }

    /// Creates a view over borrowed storage.
    #[must_use]
    pub const fn borrowed(items: &'a [T]) -> Self {
        Self::Borrowed(items)
    }

    /// Returns the underlying slice.
    #[must_use]
    pub const fn as_slice(self) -> &'a [T] {
        match self {
            Self::Owned(items) | Self::Borrowed(items) => items,
        }
    }

    /// Returns a sub-view without cloning.
    #[must_use]
    pub fn slice(self, range: std::ops::Range<usize>) -> Option<Self> {
        self.as_slice().get(range).map(Self::Borrowed)
    }

    /// Returns the number of items.
    #[must_use]
    pub fn len(self) -> usize {
        self.as_slice().len()
    }

    /// Returns true when the view is empty.
    #[must_use]
    pub fn is_empty(self) -> bool {
        self.as_slice().is_empty()
    }

    /// Copies at most `N` items into an inline small vector.
    #[must_use]
    pub fn small_head<const N: usize>(self) -> SmallVec<[T; N]>
    where
        T: Copy,
        [T; N]: smallvec::Array<Item = T>,
    {
        self.as_slice().iter().copied().take(N).collect()
    }
}
