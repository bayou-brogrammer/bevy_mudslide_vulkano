use std::sync::Arc;

use anyhow::*;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
};

#[allow(unused)]
pub fn empty_f32(
    allocator: &Arc<StandardMemoryAllocator>,
    size: usize,
) -> Result<Subbuffer<[f32]>> {
    empty_with(allocator, vec![0.0; size])
}

#[allow(unused)]
pub fn empty_u32(
    allocator: &Arc<StandardMemoryAllocator>,
    size: usize,
) -> Result<Subbuffer<[u32]>> {
    empty_with(allocator, vec![0; size])
}

pub fn empty_with<T, I>(allocator: &Arc<StandardMemoryAllocator>, iter: I) -> Result<Subbuffer<[T]>>
where
    T: BufferContents,
    I: IntoIterator<Item = T>,
    I::IntoIter: ExactSizeIterator,
{
    Ok(Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER | BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            usage: MemoryUsage::Upload,
            ..Default::default()
        },
        iter,
    )?)
}
