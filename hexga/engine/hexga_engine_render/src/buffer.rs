use super::*;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferKind
{
    VertexBuffer,
    IndexBuffer,
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BufferUsage {
    Immutable,
    Dynamic,
    Stream,
}

/// Not RAII. Manual deletion of buffer is required using [RenderBackend::delete_buffer].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RawBufferID { pub index : usize }


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
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

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BufferData
{
    pub kind  : BufferKind,
    pub usage : BufferUsage,
}