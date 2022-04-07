//! Type definitions.

use wgpu::util::DeviceExt;

/// Describes a point in 3D space.
#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 4],
}

impl Vertex {
    const ATTRS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    #[inline]
    pub const fn buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRS,
        }
    }
}

/// Wrapper around a WGPU buffer description for easy type handling.
pub struct BufferInitDescriptor<'a, A: bytemuck::NoUninit> {
    pub label: wgpu::Label<'a>,
    pub usage: wgpu::BufferUsages,
    pub contents: &'a [A],
}

impl<'a, A: bytemuck::NoUninit> BufferInitDescriptor<'a, A> {
    #[inline]
    pub fn into_wgpu_descriptor(self) -> wgpu::util::BufferInitDescriptor<'a> {
        wgpu::util::BufferInitDescriptor {
            label: self.label,
            usage: self.usage,
            contents: bytemuck::cast_slice(self.contents),
        }
    }
}

/// Utility for easier handling of a WGPU buffer.
pub struct Buffer {
    inner: wgpu::Buffer,
    len: u32,
}

impl Buffer {
    pub fn new<A: bytemuck::NoUninit>(
        device: &wgpu::Device,
        desc: BufferInitDescriptor<'_, A>,
    ) -> Self {
        let len = desc.contents.len() as u32;
        Self {
            inner: device.create_buffer_init(&desc.into_wgpu_descriptor()),
            len,
        }
    }

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
