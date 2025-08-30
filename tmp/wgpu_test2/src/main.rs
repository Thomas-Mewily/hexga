#![allow(unused_imports)]
use hexga::prelude::*;

pub struct Texture
{
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,  
}

pub struct Context
{
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl Texture
{
    pub fn from_image(img : Image)
    {
        
    }
}

fn main() {
    println!("Hello, world!");
}
