use super::*;

/// Not RAII. Manual deletion of render_pass is required using [ContextRender::delete_render_pass].
#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
pub struct RenderPassID { pub index : usize }


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PassAction 
{
    Nothing,
    Clear(ClearData)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ClearData
{
    pub color: Option<Color>,
    pub depth: Option<f32>,
    pub stencil: Option<i32>,
}
