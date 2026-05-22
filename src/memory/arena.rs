//! Arena allocation hooks.

use super::{Alignment, MemoryHint, OwnedBuffer};
use crate::TunexResult;

/// Marker returned by arena implementations for scoped rewinds.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct ArenaMark {
    offset: usize,
}

impl ArenaMark {
    /// Creates an arena mark from an implementation-defined offset.
    #[must_use]
    pub const fn new(offset: usize) -> Self {
        Self { offset }
    }

    /// Returns the implementation-defined offset.
    #[must_use]
    pub const fn offset(self) -> usize {
        self.offset
    }
}

/// Arena allocation behavior required by future allocator implementations.
pub trait ArenaAllocator: Send + Sync {
    /// Allocates bytes with an alignment and memory hint.
    ///
    /// # Errors
    ///
    /// Returns an error when the allocation cannot be satisfied.
    fn allocate(
        &self,
        len: usize,
        alignment: Alignment,
        hint: MemoryHint,
    ) -> TunexResult<OwnedBuffer>;

    /// Returns a mark that can be used for scoped rewind.
    fn mark(&self) -> ArenaMark;

    /// Rewinds arena state to a previous mark.
    ///
    /// # Errors
    ///
    /// Returns an error when the mark is invalid for this arena.
    fn rewind(&self, mark: ArenaMark) -> TunexResult<()>;
}

/// Read-only arena metadata contract.
pub trait MemoryArena: ArenaAllocator {
    /// Returns total arena capacity when known.
    fn capacity(&self) -> Option<usize>;

    /// Returns used arena bytes when known.
    fn used(&self) -> Option<usize>;
}
