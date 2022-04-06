//! State of the GPU.

use winit::event::WindowEvent;
use winit::window::Window;

/// The state of our physical device.
pub struct State {
    /// The surface onto which images can be rendered - part of a window.
    surface: wgpu::Surface,
    /// The device is an open connection to the physical device.
    device: wgpu::Device,
    /// The queue is a handle to the device's command queue
    queue: wgpu::Queue,
    /// Surface configuration.
    config: wgpu::SurfaceConfiguration,
    /// The size of our surface.
    pub size: winit::dpi::PhysicalSize<u32>,
    /// Represents a graphics pipeline and its stages.
    /// Think of a GPU as an assembly line. It has a lot of differen
    ///  parts doing different things, and the output is pixels rendered
    /// on a Framebuffer. This “assembly line” is what we call the graphics pipeline.
    render_pipeline: wgpu::RenderPipeline,
}

impl State {
    /// Retrieve and store the GPU's state.
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // WGPU context
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);

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
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);

        // Compile the shader as a shader module
        let shader = device.create_shader_module(&wgpu::include_wgsl!("res/shaders/shader.wgsl"));

        // A render/graphics pipeline is a model that describes all steps the GPU will perform
        // on some input data to produce some output.
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }],
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
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,
            render_pipeline,
        }
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
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.draw(0..3, 0..1);
        }

        // Submit the command buffer to the command queue
        self.queue.submit([encoder.finish()]);

        // Present this texture on the surface
        output.present();

        Ok(())
    }
}
