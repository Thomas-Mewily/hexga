use crate::*;

/// Not RAII. Manual deletion of shader is required using [RenderBackend::delete_shader].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawShaderID { pub index : usize }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformDesc
{
    pub name         : String,
    pub uniform_type : UniformType,
    pub nb           : usize,
}

pub struct UniformsSource<'a> { pub source : UntypedSlice<'a> }

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformBlockLayout {
    pub uniforms: Vec<UniformDesc>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images  : Vec<String>,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderSource
{
    GLSL(ShaderSourceGLSL),
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderSourceGLSL
{
    pub vertex: String,
    pub fragment: String,
}


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UniformType {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderData
{
    pub source: ShaderSource,
    pub meta  : ShaderMeta,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError
{
    CompilationError {
        shader_type: ShaderType,
        error_message: String,
    },
    LinkError(String),
    /// Shader strings should never contains \00 in the middle
    FFINulError(usize, Vec<u8>),
}