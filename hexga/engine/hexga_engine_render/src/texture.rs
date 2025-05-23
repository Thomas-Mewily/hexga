use super::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureParam
{
    pub format : TextureFormat,
    pub wrap : [TextureWrap;2],
    pub min_filter: FilterMode,
    pub mag_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
    pub allocate_mipmaps: bool,
    pub size : [u32;2],
    /// Only used for render textures. `sample_count > 1` allows anti-aliased render textures.
    ///
    /// On OpenGL, for a `sample_count > 1` render texture, render buffer object will
    /// be created instead of a regulat texture.
    pub sample_count: i32,
    pub access : TextureAccess,
}

pub struct TextureData
{
    pub param  : TextureParam,
    pub id     : Option<RawTextureID>,
    pub source : TextureSource,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum TextureSource
{
    Empty,
    RGBA8(Vec<u8>),
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureFormat {
    //RGB8,
    RGBA8,
    /* 
    RGBA16F,
    Depth,
    Depth32,
    Alpha,
    */
}

/// Not RAII. Manual deletion of texture is required using [ContextRender::delete_texture].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawTextureID { pub index : usize }

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MipmapFilterMode {
    None,
    Linear,
    Nearest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterMode {
    Linear,
    Nearest,
}

/// Sets the wrap parameter for texture.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
}