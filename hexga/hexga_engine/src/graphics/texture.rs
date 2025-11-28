use hexga::graphics::image::ImageBaseOf;
use super::*;


pub type TextureAsset = Asset<Texture>;


#[derive(Debug, Clone, PartialEq, Hash)]
pub struct Texture
{
    pub(crate) shared : Arc<GpuTexture>
}
impl Into<Texture> for &Texture
{
    fn into(self) -> Texture {
        self.clone()
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Texture
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        let intermediate = GpuTexture::deserialize(deserializer)?;
        Ok(Self::from(intermediate))
    }
}

impl<T> From<T> for Texture where GpuTexture: From<T>
{
    fn from(value: T) -> Self {
        Self { shared: Arc::new(GpuTexture::from(value)) }
    }
}
impl LoadExtension for Texture where GpuTexture: LoadExtension
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> {
        GpuTexture::load_custom_extensions()
    }
    fn load_from_reader_with_custom_extension<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: std::io::Read {
        GpuTexture::load_from_reader_with_custom_extension(reader, extension).map(|v| v.into())
    }
}
// impl SaveAs for Texture where GpuTexture: Save
// {
//     type Output=GpuTexture;
// }

#[derive(Debug, PartialEq, Hash)]
pub(crate) struct GpuTexture
{
    #[allow(unused)]
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) sampler: wgpu::Sampler,
    pub(crate) bind_group : wgpu::BindGroup
}

impl LoadExtension for GpuTexture
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { Image::load_custom_extensions() }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: std::io::Read
    {
        if Pen::is_not_init() { return Err(EncodeError::custom("The App::Pen singleton was not initialized yet")); }
        let img = Image::load_from_reader_with_custom_extension(reader, extension)?;
        Ok(Self::from(img))
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for GpuTexture
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        Ok(Image::deserialize(deserializer)?.into())
    }
}
impl<Idx> From<&ImageBaseOf<RgbaU8,Idx>> for GpuTexture where Idx: Integer
{
    fn from(value: &ImageBaseOf<RgbaU8,Idx>) -> Self
    {
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

        let bind_group = Pen.base.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Pen.binding.texture_bind_group,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
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
    }
}

impl<C,Idx> From<ImageBaseOf<C,Idx>> for GpuTexture where C: IColor<ToRgba::<u8> = RgbaU8>, Idx: Integer, u8: CastRangeFrom<C::Component>
{
    fn from(value: ImageBaseOf<C,Idx>) -> Self {
        let rgba8 = value.to_rgba_u8();
        Self::from(&rgba8)
    }
}