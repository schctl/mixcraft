//! GPU buffers.

use wgpu::util::DeviceExt;

/// Wrapper around a GPU buffer descriptor for easy type handling.
pub struct BufferInitDescriptor<'a, A: bytemuck::NoUninit> {
    pub label: wgpu::Label<'a>,
    pub usage: wgpu::BufferUsages,
    pub contents: &'a [A],
}

impl<'a, A: bytemuck::NoUninit> BufferInitDescriptor<'a, A> {
    fn get_raw(&self) -> wgpu::util::BufferInitDescriptor<'a> {
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
        let len = desc.contents.len() as u32;
        Self {
            inner: device.create_buffer_init(&desc.get_raw()),
            len,
        }
    }

    #[inline]
    pub const fn get_inner(&self) -> &wgpu::Buffer {
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
