use super::*;

pub type RenderPass = usize;


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
    //pub stencil: Option<i32>,
}
