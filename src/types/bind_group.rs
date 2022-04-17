use std::num::NonZeroU32;

pub struct BindingResource<'a> {
    pub binding: u32,
    pub visibility: wgpu::ShaderStages,
    pub ty: wgpu::BindingType,
    pub resource: wgpu::BindingResource<'a>,
}

impl<'a> BindingResource<'a> {
    // I understood this as:
    //                        Describes a resource        Handle to a resource
    fn into_entries(self) -> (wgpu::BindGroupLayoutEntry, wgpu::BindGroupEntry<'a>) {
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

        let bind_entry = wgpu::BindGroupEntry {
            binding: self.binding,
            resource: self.resource,
        };

        (layout_entry, bind_entry)
    }
}

pub struct BindGroupDescriptor<'a> {
    layout_label: Option<String>,
    layout_entries: Vec<wgpu::BindGroupLayoutEntry>,
    bind_label: wgpu::Label<'a>,
    bind_entries: Vec<wgpu::BindGroupEntry<'a>>,
}

impl<'a> BindGroupDescriptor<'a> {
    pub fn new<T: IntoIterator<Item = BindingResource<'a>>>(
        label: wgpu::Label<'a>,
        resources: T,
    ) -> Self {
        let layout_label = label.map(|x| format!("{x}_layout"));
        let (layout_entries, bind_entries) = resources
            .into_iter()
            .map(BindingResource::into_entries)
            .unzip();

        Self {
            layout_label,
            layout_entries,
            bind_label: label,
            bind_entries,
        }
    }

    #[inline]
    fn as_layout_descriptor(&self) -> wgpu::BindGroupLayoutDescriptor {
        wgpu::BindGroupLayoutDescriptor {
            label: self.layout_label.as_deref(),
            entries: &self.layout_entries,
        }
    }
}

pub struct BindGroup {
    inner: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}

impl BindGroup {
    pub fn new(device: &wgpu::Device, desc: &BindGroupDescriptor<'_>) -> BindGroup {
        Self::with_layout(
            device,
            device.create_bind_group_layout(&desc.as_layout_descriptor()),
            desc.bind_label,
            &desc.bind_entries,
        )
    }

    pub fn with_layout(
        device: &wgpu::Device,
        layout: wgpu::BindGroupLayout,
        label: wgpu::Label<'_>,
        entries: &[wgpu::BindGroupEntry<'_>],
    ) -> Self {
        let inner = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label,
            layout: &layout,
            entries,
        });

        Self { inner, layout }
    }

    #[inline]
    pub const fn inner(&self) -> &wgpu::BindGroup {
        &self.inner
    }

    #[inline]
    pub const fn layout(&self) -> &wgpu::BindGroupLayout {
        &self.layout
    }
}
