//! GPU buffers.

use wgpu::util::DeviceExt;

/// Wrapper around a GPU buffer descriptor for easy type handling.
pub struct BufferInitDescriptor<'a, A: bytemuck::NoUninit> {
    pub label: wgpu::Label<'a>,
    pub usage: wgpu::BufferUsages,
    pub contents: &'a [A],
}

impl<'a, A: bytemuck::NoUninit> BufferInitDescriptor<'a, A> {
    #[inline]
    fn as_raw(&self) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: self.label,
            usage: self.usage,
            contents: bytemuck::cast_slice(self.contents),
        }
    }
}

/// Utility for easier handling of a GPU buffer.
pub struct Buffer {
    inner: wgpu::Buffer,
    len: u32,
}

impl Buffer {
    pub fn new<A: bytemuck::NoUninit>(
        device: &wgpu::Device,
        desc: &BufferInitDescriptor<'_, A>,
    ) -> Self {
        Self {
            inner: device.create_buffer_init(&desc.as_raw()),
            len: desc.contents.len() as u32,
        }
    }

    #[inline]
    pub const fn inner(&self) -> &wgpu::Buffer {
        &self.inner
    }

    #[inline]
    pub const fn len(&self) -> u32 {
        self.len
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
}
