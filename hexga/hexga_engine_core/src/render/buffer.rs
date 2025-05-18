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

pub type Buffer = usize;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferLayout
{
    pub len          : usize,
    pub element_size : usize,
}
impl BufferLayout
{
    pub const fn size(&self) -> usize { self.len * self.element_size }
}

pub struct BufferSource<'a>
{
    inner : BufferSourceInner<'a>,
}

impl<'a> Debug for BufferSourceInner<'a> 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.write_str("BufferSource") }
}

enum BufferSourceInner<'a>
{
    UntypedSlice(UntypedSlice<'a>),
    Empty(BufferLayout),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferData
{
    buf_type : BufferType, 
    usage    : BufferUsage, 
}