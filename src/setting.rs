
use bytemuck;
// We need this for Rust to store our data correctly for the shaders
#[repr(C)]
// This is so we can store this in a buffer
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Setting {
    //pub centerx: f32,
    //pub centery: f32,
    pub scale: f32,
}

impl Setting {
    pub fn new() -> Self {
        Self {
            //centerx: 0.0,
            //centery: 0.0,
            scale: 1.0,
        }
    }
}