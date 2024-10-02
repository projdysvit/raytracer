use wgpu::util::{BufferInitDescriptor, DeviceExt};

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

const LIGHT: Light = Light {
    position: [-100.0, -250.0, 160.0],
    intensity: 1.0
};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Sphere {
    center: [f32; 3],
    radius: f32,
    color: [f32; 3],
    reflectivity: f32
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Light {
    position: [f32; 3],
    intensity: f32
}

pub struct Scene;

impl Scene {
    pub fn get_sphere_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spheres buffer"),
            contents: bytemuck::cast_slice(&SPHERES),
            usage: wgpu::BufferUsages::STORAGE
        })
    }

    pub fn get_light_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light buffer"),
            contents: bytemuck::cast_slice(&[LIGHT]),
            usage: wgpu::BufferUsages::STORAGE
        })
    }
}
