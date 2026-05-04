use super::*;
use hexga::image::ImageBaseOf;

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpuTextureView
{
    pub wgpu: wgpu::TextureView,
}
impl Handle for GpuTextureView {}
impl From<wgpu::TextureView> for GpuTextureView
{
    fn from(wgpu: wgpu::TextureView) -> Self { Self { wgpu } }
}
impl From<GpuTextureView> for wgpu::TextureView
{
    fn from(value: GpuTextureView) -> Self { value.wgpu }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct GpuTexture
{
    pub wgpu: wgpu::Texture,
}
impl Handle for GpuTexture {}
impl GpuTexture
{
    pub fn view(&self) -> GpuTextureView { self.wgpu.create_view(&___()).into() }
}
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
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension>
    {
        Image::load_custom_extensions()
    }

    fn load_from_reader_with_custom_extension<R>(
        reader: R,
        extension: &extension,
    ) -> EncodeResult<Self>
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
        let texture = Gpu.wgpu.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        Gpu.wgpu.queue.write_texture(
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

#[derive(Debug, Clone)]
pub struct GpuBindGroupTexture
{
    pub(crate) texture: GpuTexture,
    pub(crate) view: GpuTextureView,
    pub(crate) sampler: GpuSampler,
    pub(crate) bind_group: GpuBindGroup,
}
impl GpuBindGroupTexture
{
    pub fn texture(&self) -> &GpuTexture { &self.texture }
    pub fn view(&self) -> &GpuTextureView { &self.view }
    pub fn sampler(&self) -> &GpuSampler { &self.sampler }
    pub fn bind_group(&self) -> &GpuBindGroup { &self.bind_group }

    pub fn full_view(texture: GpuTexture, sampler: GpuSampler, bind_group: GpuBindGroup) -> Self
    {
        let view = texture.view();
        Self::new(texture, view, sampler, bind_group)
    }
    pub fn new(
        texture: GpuTexture,
        view: GpuTextureView,
        sampler: GpuSampler,
        bind_group: GpuBindGroup,
    ) -> Self
    {
        Self {
            texture,
            view,
            sampler,
            bind_group,
        }
    }

    /*
    pub fn new(texture: GpuTexture, sampler: GpuSampler) -> Self
    {
        let view = texture.view();
        let bing_group = Gpu.wgpu.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &Gpu.wgpu.binding.texture_bind_group,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view.wgpu),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler.wgpu),
                },
            ],
            label: None,
        }).into();
        Self { texture, view, sampler, bind_group }
    }*/
}
/*
impl LoadExtension for GpuBindGroupTexture
{
    fn load_custom_extensions() -> impl Iterator<Item = &'static extension> { Image::load_custom_extensions() }

    fn load_from_reader_with_custom_extension<R>(reader: R, extension: &extension) -> EncodeResult<Self> where Self: Sized, R: std::io::Read
    {
        if Pen::is_not_init() { return Err(EncodeError::custom("The App::Pen singleton was not initialized yet")); }
        let img = Image::load_from_reader_with_custom_extension(reader, extension)?;
        Ok(Self::from(img))
    }
}
*/
