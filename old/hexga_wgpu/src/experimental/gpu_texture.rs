use super::*;
use hexga::image::ImageBaseOf;

#[repr(transparent)]
#[derive(Debug, Clone, PartialEq)]
pub struct GpuTexture
{
    pub wgpu: wgpu::Texture,
}
impl Handle for GpuTexture {}
impl From<wgpu::Texture> for GpuTexture
{
    fn from(wgpu: wgpu::Texture) -> Self { Self { wgpu } }
}
impl From<GpuTexture> for wgpu::Texture
{
    fn from(value: GpuTexture) -> Self { value.wgpu }
}

impl LoadExtension for GpuTexture
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { Image::load_custom_extensions() }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: Option<&extension>) -> EncodeResult<Self>
    where
        Self: Sized,
        R: std::io::Read,
    {
        if Gpu::is_not_init()
        {
            return Err(EncodeError::custom("The Gpu was not initialized"));
        }
        let img = Image::load_from_reader_with_custom_extension(reader, extension)?;
        Ok(Self::from(img))
    }
}
#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for GpuTexture
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Image::deserialize(deserializer)?.into())
    }
}
impl<Idx> From<&ImageBaseOf<RgbaU8, Idx>> for GpuTexture
where
    Idx: Integer,
{
    fn from(value: &ImageBaseOf<RgbaU8, Idx>) -> Self
    {
        let dimensions = value.size();
        let dimensions_u32 = dimensions.map(|v| v.to_u32());

        let size = wgpu::Extent3d {
            width: dimensions_u32.x,
            height: dimensions_u32.y,
            depth_or_array_layers: 1,
        };
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;
        let texture = Gpu.device().create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Gpu.queue().write_texture(
            wgpu::TexelCopyTextureInfo {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            bit::transmute_slice(value.pixels()),
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions_u32.x),
                rows_per_image: Some(dimensions_u32.y),
            },
            size,
        );
        Self { wgpu: texture }
    }
}

impl<C, Idx> From<ImageBaseOf<C, Idx>> for GpuTexture
where
    C: IColor<ToRgba<u8> = RgbaU8>,
    Idx: Integer,
    u8: CastRangeFrom<C::Component>,
{
    fn from(value: ImageBaseOf<C, Idx>) -> Self
    {
        let rgba8 = value.to_rgba_u8();
        Self::from(&rgba8)
    }
}
