use bytemuck::{Pod, Zeroable};
use winit::dpi::PhysicalSize;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct CameraUniform {
    aspect_ratio: f32,
    width: f32,
    height: f32
}

impl CameraUniform {
    pub fn new(width: f32, height: f32) -> CameraUniform {
        Self {
            aspect_ratio: width / height,
            width: width - 1.0,
            height: height - 1.0
        }
    }

    pub fn resize(&mut self, new_size: &PhysicalSize<u32>) {
        let new_width = new_size.width as f32;
        let new_height = new_size.height as f32;

        self.aspect_ratio = new_width / new_height;
        self.width = new_width;
        self.height = new_height;
    }
}
