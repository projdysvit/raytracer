use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub struct Light {
    pub position: [f32; 3],
    pub intensity: f32
}
