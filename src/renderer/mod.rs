//! State of the GPU.

pub mod types;

use winit::event::WindowEvent;
use winit::window::Window;

use types::{
    binding,
    buffer::{Buffer, BufferInitDescriptor},
    texture::{Texture, TextureDescriptor},
    Vertex,
};

/// Managed the state of the physical device.
pub struct Renderer {
    /// The surface onto which images can be rendered - part of a window.
    surface: wgpu::Surface,
    /// The device is an open connection to the physical device.
    device: wgpu::Device,
    /// The queue is a handle to the device's command queue.
    queue: wgpu::Queue,
    /// Surface configuration.
    config: wgpu::SurfaceConfiguration,
    /// The size of our surface.
    pub size: winit::dpi::PhysicalSize<u32>,
    /// Represents a render pipeline and its stages.
    ///
    /// A render/graphics pipeline is a model that describes all steps the GPU will perform
    /// on input data to produce output. Think of a GPU as an assembly line. It has a lot of
    /// different parts doing different things, and the output is pixels rendered
    /// on a framebuffer. This "assembly line" is what we call the graphics pipeline.
    render_pipeline: wgpu::RenderPipeline,
    /// A vertex buffer object.
    vbo: Buffer,
    /// An index buffer object.
    ibo: Buffer,
    /// The bind group for diffuse textures.
    diffuse_bind_group: binding::Group,
}

impl Renderer {
    /// Retrieve and store the GPU's state.
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // WGPU context
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        // SAFETY: window is always valid
        let surface = unsafe { instance.create_surface(&window) };

        // A handle to the physical device
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap(); // unwrap is okay here since we can't get a handle to the GPU

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    // Debug label
                    label: Some("Some Device"),
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                None, // API call tracing
            )
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // Texture stuff
        let dirt = image::load_from_memory(include_bytes!("../../res/textures/dirt.png")).unwrap();

        let diffuse_texture = Texture::new(
            &device,
            &queue,
            &TextureDescriptor {
                label: Some("dirt_texture"),
                mip_level_count: 1,
                sample_count: 1,
                image: &dirt,
            },
            None,
        );

        let diffuse_bind_group = binding::Group::new(
            &device,
            Some("diffuse_texture_group"),
            [
                binding::group::Entry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    resource: wgpu::BindingResource::TextureView(diffuse_texture.view()),
                },
                binding::group::Entry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    resource: wgpu::BindingResource::Sampler(diffuse_texture.sampler()),
                },
            ]
            .into_iter(),
        );

        let render_pipeline =
            Self::create_pipeline(&device, &config, &[diffuse_bind_group.layout()]);

        // Get vertex data
        let (vbo, ibo) = Self::get_data(&device);

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
            vbo,
            ibo,
            diffuse_bind_group,
        }
    }

    /// Compile shaders and create the render pipeline.
    fn create_pipeline(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        bind_group_layouts: &[&wgpu::BindGroupLayout],
    ) -> wgpu::RenderPipeline {
        // Compile the shader as a shader module
        let shader =
            device.create_shader_module(wgpu::include_wgsl!("../../res/shaders/shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts,
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::BUFFER_LAYOUT],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
    }

    /// Get vertex data.
    /// Returns a (vertex buffer, index buffer) pair.
    pub fn get_data(device: &wgpu::Device) -> (Buffer, Buffer) {
        const VERTICES: &[Vertex] = &[
            Vertex {
                position: [0.5, 0.5, 0.0],
                texture: [1.0, 1.0],
            },
            Vertex {
                position: [-0.5, 0.5, 0.0],
                texture: [-1.0, 1.0],
            },
            Vertex {
                position: [-0.5, -0.5, 0.0],
                texture: [-1.0, -1.0],
            },
            Vertex {
                position: [0.5, -0.5, 0.0],
                texture: [1.0, -1.0],
            },
        ];

        const INDICES: &[u16] = &[0, 1, 2, 0, 2, 3];

        let vbo = Buffer::new(
            device,
            &BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                usage: wgpu::BufferUsages::VERTEX,
                contents: VERTICES,
            },
        );

        let ibo = Buffer::new(
            device,
            &BufferInitDescriptor {
                label: Some("Index Buffer"),
                usage: wgpu::BufferUsages::INDEX,
                contents: INDICES,
            },
        );

        (vbo, ibo)
    }

    /// Resize the render surface.
    pub fn resize(&mut self, new: winit::dpi::PhysicalSize<u32>) {
        if new.width > 0 && new.height > 0 {
            self.size = new;
            self.config.width = new.width;
            self.config.height = new.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn input(&mut self, _: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {}

    #[profiling::function]
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // An encoder records GPU operations to obtain a command buffer
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            // `render_pass` is an in-progress recording of a render pass.
            // A render pass is a GPU operation that renders an output image onto a framebuffer.
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.09,
                            g: 0.03,
                            b: 0.01,
                            a: 1.00,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, self.diffuse_bind_group.inner(), &[]);
            render_pass.set_vertex_buffer(0, self.vbo.inner().slice(..));
            render_pass.set_index_buffer(self.ibo.inner().slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.ibo.len(), 0, 0..1);
        }

        // Submit the command buffer to the command queue
        self.queue.submit([encoder.finish()]);

        // Present this texture on the surface
        output.present();

        profiling::finish_frame!();
        Ok(())
    }
}
