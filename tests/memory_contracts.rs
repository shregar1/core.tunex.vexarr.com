use std::sync::Arc;

use bytes::Bytes;
use tunex_core::{
    memory::{
        AlignedBuffer, Alignment, ArenaAllocator, ArenaMark, BatchView, BorrowedBatch, BufferView,
        MemoryArena, MemoryHint, OwnedBatch, OwnedBuffer, SharedBuffer, ZeroCopyBuffer,
    },
    TunexResult,
};

fn assert_send_sync<T: Send + Sync>() {}
fn assert_object_safe<T: ?Sized>() {}

#[test]
fn owned_buffer_freezes_without_changing_payload_pointer() {
    let owned = OwnedBuffer::from_vec(vec![1, 2, 3, 4]).with_hint(MemoryHint::Cpu);
    let ptr = owned.as_slice().as_ptr();
    let frozen = owned.freeze();

    assert_eq!(frozen.bytes().as_ptr(), ptr);
    assert_eq!(frozen.hint(), MemoryHint::Cpu);
    assert_eq!(frozen.view().as_slice(), &[1, 2, 3, 4]);
}

#[test]
fn zero_copy_buffer_clone_shares_bytes_and_slices_views() {
    let buffer = ZeroCopyBuffer::new(Bytes::from_static(b"abcdef"));
    let cloned = buffer.clone();
    let view = buffer.view().slice(1..4).unwrap();

    assert_eq!(buffer.bytes().as_ptr(), cloned.bytes().as_ptr());
    assert_eq!(view.as_slice(), b"bcd");
}

#[test]
fn buffer_view_casts_typed_data_with_bytemuck() {
    let words = [1_u32, 2_u32, 3_u32];
    let bytes = bytemuck::cast_slice(&words);
    let view = BufferView::from_slice(bytes);

    assert_eq!(view.try_cast::<u32>().unwrap(), &words);
}

#[test]
fn batch_views_slice_without_cloning() {
    let items: Arc<[u32]> = Arc::from([10, 20, 30, 40]);
    let owned = OwnedBatch::with_hint(items.clone(), MemoryHint::Simd);
    let borrowed = BorrowedBatch::new(owned.as_slice());
    let view = BatchView::borrowed(borrowed.as_slice())
        .slice(1..3)
        .unwrap();

    assert!(Arc::ptr_eq(owned.as_arc(), &items));
    assert_eq!(owned.hint(), MemoryHint::Simd);
    assert_eq!(view.as_slice(), &[20, 30]);
}

#[test]
fn shared_buffer_uses_arc_storage() {
    let items: Arc<[u32]> = Arc::from([7, 8, 9]);
    let shared = SharedBuffer::with_hint(items.clone(), MemoryHint::GpuTransfer);

    assert!(Arc::ptr_eq(shared.as_arc(), &items));
    assert_eq!(shared.as_slice(), &[7, 8, 9]);
    assert_eq!(shared.hint(), MemoryHint::GpuTransfer);
}

#[test]
fn alignment_contract_checks_current_pointer() {
    let alignment = Alignment::new(64).unwrap();
    let buffer = AlignedBuffer::new(Bytes::from_static(b"abc"), alignment);

    assert_eq!(alignment.get(), 64);
    assert_eq!(
        buffer.is_currently_aligned(),
        alignment.is_aligned(buffer.bytes().as_ptr() as usize)
    );
}

#[test]
fn memory_contracts_are_send_sync_and_object_safe() {
    assert_send_sync::<ZeroCopyBuffer>();
    assert_send_sync::<OwnedBuffer>();
    assert_send_sync::<SharedBuffer<u32>>();
    assert_send_sync::<OwnedBatch<u32>>();
    assert_send_sync::<BorrowedBatch<'static, u32>>();
    assert_send_sync::<BatchView<'static, u32>>();
    assert_send_sync::<AlignedBuffer>();
    assert_send_sync::<ArenaMark>();
    assert_object_safe::<dyn ArenaAllocator>();
    assert_object_safe::<dyn MemoryArena>();
}

struct NoopArena;

impl ArenaAllocator for NoopArena {
    fn allocate(
        &self,
        len: usize,
        _alignment: Alignment,
        hint: MemoryHint,
    ) -> TunexResult<OwnedBuffer> {
        Ok(OwnedBuffer::from_vec(vec![0; len]).with_hint(hint))
    }

    fn mark(&self) -> ArenaMark {
        ArenaMark::new(0)
    }

    fn rewind(&self, _mark: ArenaMark) -> TunexResult<()> {
        Ok(())
    }
}

impl MemoryArena for NoopArena {
    fn capacity(&self) -> Option<usize> {
        None
    }

    fn used(&self) -> Option<usize> {
        None
    }
}

#[test]
fn arena_contract_allocates_owned_buffers() {
    let arena = NoopArena;
    let buffer = arena
        .allocate(4, Alignment::CACHE_LINE, MemoryHint::Snapshot)
        .unwrap();

    assert_eq!(buffer.as_slice(), &[0, 0, 0, 0]);
    assert_eq!(buffer.freeze().hint(), MemoryHint::Snapshot);
}
