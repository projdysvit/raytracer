use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Sphere {
    pub center: [f32; 3],
    pub radius: f32,
    pub color: [f32; 3],
    pub reflectivity: f32
}
