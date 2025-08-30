use super::*;

pub struct Texture
{
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,  
}

impl Texture
{
    pub fn from_image(img : Image)
    {
        let dimensions = img.size();
        
    }
}
