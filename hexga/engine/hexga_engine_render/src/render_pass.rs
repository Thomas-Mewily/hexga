use super::*;

/// Not RAII. Manual deletion of render_pass is required using [RenderBackend::delete_render_pass].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RenderPassID { pub index : usize }


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PassAction
{
    Nothing,
    Clear(ClearData)
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearData
{
    pub color: Option<Color>,
    pub depth: Option<GpuFloat>,
    pub stencil: Option<i32>,
}
