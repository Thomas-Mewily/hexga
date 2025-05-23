use super::*;

/// Not RAII. Manual deletion of shader is required using [ContextRender::delete_shader].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawShaderID { pub index : usize }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformDesc 
{
    pub name         : String,
    pub uniform_type : UniformType,
    pub nb           : usize,
}

pub struct UniformsSource<'a> { pub source : UntypedSlice<'a> }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UniformBlockLayout {
    pub uniforms: Vec<UniformDesc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images  : Vec<String>,
}

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ShaderSource 
{
    GLSL(ShaderSourceGLSL),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShaderSourceGLSL
{
    pub vertex: String,
    pub fragment: String,
}



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


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ShaderData
{
    pub source: ShaderSource,
    pub meta  : ShaderMeta,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShaderType {
    Vertex,
    Fragment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShaderError 
{
    CompilationError {
        shader_type: ShaderType,
        error_message: String,
    },
    LinkError(String),
    /// Shader strings should never contains \00 in the middle
    FFINulError(std::ffi::NulError),
}