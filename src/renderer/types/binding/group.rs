//! Groups of binding resources.

use std::num::NonZeroU32;
use std::sync::Arc;

/// A single resource in the group.
pub struct Entry<'a> {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub resource: wgpu::BindingResource<'a>,
}

impl<'a> Entry<'a> {
    fn into_entry(self) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupEntry<'a>) {
        // Describe layout of this entry
        let layout_entry = wgpu::BindGroupLayoutEntry {
            binding: self.binding,
            visibility: self.visibility,
            ty: self.ty,
            count: match self.resource {
                wgpu::BindingResource::BufferArray(a) => NonZeroU32::new(a.len() as u32),
                wgpu::BindingResource::SamplerArray(a) => NonZeroU32::new(a.len() as u32),
                wgpu::BindingResource::TextureViewArray(a) => NonZeroU32::new(a.len() as u32),
                _ => None,
            },
        };

        // Get a handle to the resource itself
        let bind_entry = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.resource,
        };

        (layout_entry, bind_entry)
    }
}

/// A binding group and its layout.
///
/// The layout of a bind group is treated separate from the bind group itself.
/// This is intended to be a sort-of easy-to-use wrapper around that, managing
/// the layout of an entry along with itself.
pub struct Group {
    inner: wgpu::BindGroup,
    layout: Arc<wgpu::BindGroupLayout>,
}

impl Group {
    /// Create a new group from a descriptor.
    pub fn new<'a>(
        device: &wgpu::Device,
        label: wgpu::Label<'a>,
        resources: impl Iterator<Item = Entry<'a>>,
    ) -> Group {
        // Get the layout and handle of each entry
        let (layout_entries, bind_entries): (Vec<_>, Vec<_>) =
            resources.into_iter().map(Entry::into_entry).unzip();

        // Construct the layout of the group from the layout of the entries
        let layout = {
            let layout_label = label.map(|x| format!("{x}_layout"));
            let desc = wgpu::BindGroupLayoutDescriptor {
                label: layout_label.as_deref(),
                entries: &layout_entries,
            };
            Arc::new(device.create_bind_group_layout(&desc))
        };

        // Create the bind group itself
        let inner = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &layout,
            entries: &bind_entries,
        });

        Self { inner, layout }
    }

    /// Create a bind group from raw components.
    ///
    /// ## Safety
    ///
    /// User guarantees that the provided layout corresponds to the bind group.
    #[inline]
    pub unsafe fn from_raw(inner: wgpu::BindGroup, layout: Arc<wgpu::BindGroupLayout>) -> Self {
        Self { inner, layout }
    }

    /// Get the underlying bind group.
    #[inline]
    pub fn inner(&self) -> &wgpu::BindGroup {
        &self.inner
    }

    /// Get the layout of this group.
    #[inline]
    pub fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }

    /// Clone the layout of this group.
    #[inline]
    pub fn clone_layout(&self) -> Arc<wgpu::BindGroupLayout> {
        self.layout.clone()
    }
}
