use super::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferType 
{
    VertexBuffer,
    IndexBuffer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    Immutable,
    Dynamic,
    Stream,
}

/// Not RAII. Manual deletion of buffer is required using [ContextRender::delete_buffer].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawBufferID { pub index : usize }


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferLayout
{
    pub len          : usize,
    pub element_size : usize,
}
impl BufferLayout
{
    /// `self.len * self.element_size`
    pub const fn size(&self) -> usize { self.len * self.element_size }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum BufferSource<'a>
{
    UntypedSlice(UntypedSlice<'a>),
    Empty(BufferLayout),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferData
{
    pub buf_type : BufferType, 
    pub usage    : BufferUsage, 
}