use super::*;

pub type TextureWrap2 = Vector2<TextureWrap>;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TextureParam
{
    pub format : TextureFormat,
    pub wrap : TextureWrap2,
    pub min_filter: FilterMode,
    pub mag_filter: FilterMode,
    pub mipmap_filter: MipmapFilterMode,
    pub allocate_mipmaps: bool,
    pub size : Point2,
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
    RGBA8(Vec<u8>), // Todo Fix it : use a grid / image
}
// Todo fit it use hexga_graphics
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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

/// Not RAII. Manual deletion of texture is required using [RenderBackend::delete_texture].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawTextureID { pub index : usize }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MipmapFilterMode {
    None,
    Linear,
    Nearest,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FilterMode {
    Linear,
    Nearest,
}


/// Sets the wrap parameter for texture.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
}