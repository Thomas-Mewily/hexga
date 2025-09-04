use super::*;

pub mod prelude
{
    pub use super::Texture;
}

pub struct Texture
{
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,  
}

impl Texture
{
    /* 
    pub fn from_image(img : &Image) -> Self
    {
        let dimensions = img.size();
        
        let size = wgpu::Extent3d {
            width: dimensions.x as _,
            height: dimensions.y as _,
            depth_or_array_layers: 1,
        };

        let ctx = &mut Ctx.wgpu;
        let (device, queue) = (&mut ctx.device, &mut ctx.queue);
        
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label : None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        let pixels = img.pixels();

        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            unsafe { std::mem::transmute(pixels) },
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.x as u32),
                rows_per_image: Some(dimensions.y as u32),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Self 
        {
            texture,
            view,
            sampler,
        }
    }
    */
}

