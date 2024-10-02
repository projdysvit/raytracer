use wgpu::util::{BufferInitDescriptor, DeviceExt};

const SPHERES: [Sphere; 4] = [
    Sphere {
        center: [0.0, 50.6, -1.0],
        radius: 50.0,
        color: [0.1, 0.1, 0.2],
        reflectivity: 0.1
    },
    Sphere {
        center: [0.0, 0.11, -2.0],
        radius: 0.5,
        color: [0.5, 0.0, 0.0],
        reflectivity: 0.5
    },
    Sphere {
        center: [0.8, 0.315, -2.0],
        radius: 0.3,
        color: [0.1, 0.2, 0.3],
        reflectivity: 0.7
    },
    Sphere {
        center: [-0.65, 0.306, -1.5],
        radius: 0.3,
        color: [0.3, 0.2, 0.1],
        reflectivity: 0.3
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

pub struct Scene {
    light: Light,
    light_rotation_angle: f32
}

impl Scene {
    pub fn new() -> Self {
        Self {
            light: LIGHT,
            light_rotation_angle: 0.0
        }
    }

    pub fn get_sphere_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Spheres buffer"),
            contents: bytemuck::cast_slice(&SPHERES),
            usage: wgpu::BufferUsages::STORAGE
        })
    }
    
    pub fn get_light_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Light buffer"),
            contents: bytemuck::cast_slice(&[self.light]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
        })
    }

    pub fn light_update(&mut self) {
        let rotation_speed = 0.5;
        let delta_time = 0.01;
        let orbit_radius = 150.0;

        self.light_rotation_angle += rotation_speed * delta_time;

        self.light.position[0] = orbit_radius * self.light_rotation_angle.cos();
        self.light.position[2] = orbit_radius * self.light_rotation_angle.sin();
    }

    pub fn update_light_buffer(&self, queue: &wgpu::Queue, buffer: &wgpu::Buffer) {
        queue.write_buffer(buffer, 0, bytemuck::cast_slice(&[self.light]));
    }
}
