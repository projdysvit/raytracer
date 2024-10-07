use wgpu::util::{BufferInitDescriptor, DeviceExt};
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    position: [f32; 3],
    aspect_ratio: f32
}

pub struct Camera {
    uniform: CameraUniform
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            uniform: CameraUniform {
                position: [0.0, 0.0, 1.0],
                aspect_ratio: width as f32 / height as f32,
            }
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.uniform.aspect_ratio = new_size.width as f32 / new_size.height as f32;
    }

    pub fn get_camera_uniform_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Camera uniform buffer"),
            contents: bytemuck::cast_slice(&[self.uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        })
    }

    pub fn update_buffer(&self, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[self.uniform]));
    }
}
