//! Zero-copy memory and batch view contracts.

pub mod alignment;
pub mod arena;
pub mod batch;
pub mod buffer;
pub mod view;

pub use alignment::{AlignedBuffer, Alignment};
pub use arena::{ArenaAllocator, ArenaMark, MemoryArena};
pub use batch::{BatchView, BorrowedBatch, OwnedBatch};
pub use buffer::{MemoryHint, OwnedBuffer, SharedBuffer, ZeroCopyBuffer};
pub use view::BufferView;
