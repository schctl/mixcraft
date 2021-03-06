//! Image textures.

use image::GenericImageView;

pub struct TextureDescriptor<'a> {
    pub label: wgpu::Label<'a>,
    pub mip_level_count: u32,
    pub sample_count: u32,
    pub image: &'a image::DynamicImage,
}

impl<'a> TextureDescriptor<'a> {
    #[inline]
    pub fn as_raw(&self) -> wgpu::TextureDescriptor {
        wgpu::TextureDescriptor {
            label: self.label,
            mip_level_count: self.mip_level_count,
            sample_count: self.sample_count,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            size: self.size(),
        }
    }

    #[inline]
    pub fn size(&self) -> wgpu::Extent3d {
        let size = self.image.dimensions();
        wgpu::Extent3d {
            width: size.0,
            height: size.1,
            depth_or_array_layers: 1,
        }
    }
}

pub struct Texture {
    inner: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl Texture {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        desc: &TextureDescriptor<'_>,
        sampler_desc: Option<&wgpu::SamplerDescriptor>,
    ) -> Self {
        let inner = device.create_texture(&desc.as_raw());

        let size = desc.size();

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &inner,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            desc.image.as_bytes(),
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * size.width),
                rows_per_image: std::num::NonZeroU32::new(size.height),
            },
            size,
        );

        let view = inner.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = match sampler_desc {
            Some(s) => device.create_sampler(s),
            None => device.create_sampler(&wgpu::SamplerDescriptor {
                address_mode_u: wgpu::AddressMode::Repeat,
                address_mode_v: wgpu::AddressMode::Repeat,
                address_mode_w: wgpu::AddressMode::Repeat,
                mag_filter: wgpu::FilterMode::Nearest,
                min_filter: wgpu::FilterMode::Nearest,
                mipmap_filter: wgpu::FilterMode::Nearest,
                ..Default::default()
            }),
        };

        Self {
            inner,
            view,
            sampler,
        }
    }

    #[inline]
    pub const fn inner(&self) -> &wgpu::Texture {
        &self.inner
    }

    #[inline]
    pub const fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    #[inline]
    pub const fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
