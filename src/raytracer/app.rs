use anyhow::{Context, Result};
use camera::CameraUniform;
use light::Light;
use sphere::Sphere;
use vertex::Vertex;
use wgpu::{util::{BufferInitDescriptor, DeviceExt}, BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer, BufferBinding, BufferBindingType, BufferUsages, Color, ColorTargetState, ColorWrites, CommandEncoderDescriptor, CompositeAlphaMode, Device, DeviceDescriptor, FragmentState, FrontFace, Instance, InstanceDescriptor, LoadOp, MultisampleState, Operations, PipelineLayoutDescriptor, PolygonMode, PowerPreference, PresentMode, PrimitiveState, PrimitiveTopology, Queue, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor, ShaderSource, ShaderStages, StoreOp, Surface, SurfaceConfiguration, SurfaceError, TextureFormat, TextureUsages, TextureViewDescriptor, VertexState};
use winit::{application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::ActiveEventLoop, window::{Window, WindowId}};

mod vertex;
mod camera;
mod sphere;
mod light;

const VERTICES: [Vertex; 6] = [
    Vertex {
        position: [-1.0, 1.0]
    },
    Vertex {
        position: [-1.0, -1.0]
    },
    Vertex {
        position: [1.0, 1.0]
    },
    Vertex {
        position: [1.0, 1.0]
    },
    Vertex {
        position: [-1.0, -1.0]
    },
    Vertex {
        position: [1.0, -1.0]
    }
];

const SPHERES: [Sphere; 4] = [
    Sphere {
        center: [0.0, 50.6, -1.0],
        radius: 50.0,
        color: [0.5, 0.5, 0.7],
        reflectivity: 0.3
    },
    Sphere {
        center: [0.0, 0.11, -2.0],
        radius: 0.5,
        color: [0.7, 0.0, 0.0],
        reflectivity: 0.7
    },
    Sphere {
        center: [0.8, 0.31, -2.0],
        radius: 0.3,
        color: [0.3, 0.5, 0.7],
        reflectivity: 1.0
    },
    Sphere {
        center: [-0.65, 0.31, -1.5],
        radius: 0.3,
        color: [0.3, 0.7, 0.3],
        reflectivity: 0.5
    }
];

pub struct Application<'a> {
    window: &'a Window,
    surface: Surface<'a>,
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    bind_group: BindGroup,
    camera_uniform: CameraUniform,
    camera_uniform_buffer: Buffer
}

impl<'a> Application<'a> {
    pub async fn new(window: &'a Window) -> Result<Self> {
        let instance = Instance::new(InstanceDescriptor::default());
        let surface = instance.create_surface(window)?;
        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false
            }
        ).await.context("adapter error")?;
        let (device, queue) = adapter.request_device(&DeviceDescriptor::default(), None)
            .await
            .context("device and queue error")?;
        let surface_caps = surface.get_capabilities(&adapter);
        let texture_format = surface_caps.formats
            .into_iter()
            .find(|format| format.eq(&TextureFormat::Rgba8Unorm))
            .context("preferred texture format error")?;
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: texture_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: PresentMode::AutoVsync,
            alpha_mode: CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 3
        };
        surface.configure(&device, &config);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&VERTICES),
            usage: BufferUsages::VERTEX
        });

        let camera_uniform = CameraUniform::new(config.width as f32, config.height as f32);

        let camera_uniform_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST
        });

        let spheres_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spheres buffer"),
            contents: bytemuck::cast_slice(&SPHERES),
            usage: BufferUsages::STORAGE
        });

        let light = Light {
            position: [-100.0, -250.0, -60.0],
            intensity: 1.0
        };

        let light_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spheres buffer"),
            contents: bytemuck::cast_slice(&[light]),
            usage: BufferUsages::STORAGE
        });

        let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None
                    },
                    count: None
                }
            ]
        });

        let bind_group = device.create_bind_group(&BindGroupDescriptor {
            label: Some("Bind group"),
            layout: &bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &camera_uniform_buffer,
                        offset: 0,
                        size: None
                    })
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &spheres_buffer,
                        offset: 0,
                        size: None
                    })
                },
                BindGroupEntry {
                    binding: 2,
                    resource: BindingResource::Buffer(BufferBinding {
                        buffer: &light_buffer,
                        offset: 0,
                        size: None
                    })
                }
            ]
        });

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("../shaders/raytracer.wgsl").into())
        });

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Render pipeline layout"),
            bind_group_layouts: &[&bind_group_layout],
            ..Default::default()
        });
        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                compilation_options: Default::default(),
                buffers: &[Vertex::desc()]
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                front_face: FrontFace::Ccw,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: Default::default(),
                targets: &[
                    Some(ColorTargetState {
                        format: TextureFormat::Rgba8Unorm,
                        blend: None,
                        write_mask: ColorWrites::ALL
                    })
                ]
            }),
            multiview: None,
            cache: None
        });

        Ok(Self {
            window,
            surface,
            device,
            queue,
            config,
            render_pipeline,
            vertex_buffer,
            bind_group,
            camera_uniform,
            camera_uniform_buffer
        })
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if !(new_size.width > 0 && new_size.height > 0) {
            return;
        }

        self.camera_uniform.resize(&new_size);
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
    }

    pub fn update(&mut self) {
        self.queue.write_buffer(&self.camera_uniform_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&CommandEncoderDescriptor::default());

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None
            });

            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

impl<'a> ApplicationHandler for Application<'a> {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        if window_id != self.window.id() {
            return;
        }

        match event {
            WindowEvent::Resized(physical_size) => {
                self.resize(physical_size);
            },
            WindowEvent::RedrawRequested => {
                self.window.request_redraw();

                if !(self.config.width > 0 && self.config.height > 0) {
                    return;
                }

                self.update();
                match self.render() {
                    Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                        self.resize(PhysicalSize::new(self.config.width, self.config.height));
                    },
                    _ => {}
                }
            },
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.window.request_redraw();
    }
}
