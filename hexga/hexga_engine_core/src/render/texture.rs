use super::*;


pub struct TextureParam
{
    pub wrap: TextureWrap,
    pub min_filter: FilterMode,
    pub mag_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
    pub allocate_mipmaps: bool,
    /// Only used for render textures. `sample_count > 1` allows anti-aliased render textures.
    ///
    /// On OpenGL, for a `sample_count > 1` render texture, render buffer object will
    /// be created instead of a regulat texture.
    ///
    pub sample_count: i32,
    pub access : TextureAccess,
}

pub struct TextureData
{
    pub param : TextureParam,
    id        : Texture,
    source    : TextureSource,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextureSource
{
    None,
    RGBAByte(ImageRGBAByte),
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextureView<'a>
{
    None,
    RGBAByte(ImageRGBAByteView<'a>),
}

pub type Texture = usize;


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}


#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum MipmapFilterMode {
    None,
    Linear,
    Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum FilterMode {
    Linear,
    Nearest,
}

/// Sets the wrap parameter for texture.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
}

pub type TextureWrap2 = Vector2<TextureWrap>;
