use hexga::graphics::image::ImageBaseOf;

use super::*;

#[derive(Debug, Clone)]
pub struct Texture
{
    pub(crate) shared : Arc<GpuTexture>
}
impl<T> From<T> for Texture where GpuTexture: From<T>
{
    fn from(value: T) -> Self {
        Self { shared: Arc::new(GpuTexture::from(value)) }
    }
}


#[derive(Debug)]
pub struct GpuTexture
{
    #[allow(unused)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) bind_group : wgpu::BindGroup
}

impl<Idx> From<&ImageBaseOf<RgbaU8,Idx>> for GpuTexture where Idx: Integer
{
    fn from(value: &ImageBaseOf<RgbaU8,Idx>) -> Self {
        let dimensions = value.size();
        let dimensions_u32 = dimensions.map(|v| v.to_u32());

        let size = wgpu::Extent3d {
            width: dimensions_u32.x,
            height: dimensions_u32.y,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = Pen.base.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Pen.base.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            value.pixels().as_u8_slice(),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions_u32.x),
                rows_per_image: Some(dimensions_u32.y),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = Pen.base.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        todo!();

        /*
        let bind_group = Pen.base.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Pen.binding.texture_bind_group,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: None,
        });


        Self
        {
            texture,
            view,
            sampler,
            bind_group
        }
        */
    }
}

impl<C,Idx> From<ImageBaseOf<C,Idx>> for GpuTexture where C: IColor<ToRgba::<u8> = RgbaU8>, Idx: Integer, u8: CastRangeFrom<C::Component>
{
    fn from(value: ImageBaseOf<C,Idx>) -> Self {
        let rgba8 = value.to_rgba_u8();
        Self::from(&rgba8)
    }
}