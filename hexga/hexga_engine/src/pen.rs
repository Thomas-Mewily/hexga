use crate::*;

pub struct Pen;
impl Deref for Pen
{
    type Target=ContextPen;
    fn deref(&self) -> &Self::Target { &Ctx.pen }
}
impl DerefMut for Pen
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut ctx_mut().pen }
}

#[derive(Debug)]
pub struct ContextPen
{
    /* 
    pipeline: miniquad::Pipeline,
    bindings: miniquad::Bindings,

    vertex_buffer: miniquad::BufferId,
    index_buffer : miniquad::BufferId,

    white_pixel : miniquad::TextureId,
    */
    //batch_vertex_buffer: Vec<GpuVertex>,
    //batch_index_buffer: Vec<GpuVertexIdx>,
    
    //param : PenConfig,
}

impl ContextPen
{
    pub(crate) fn new() -> Self 
    {
        ContextPen {}

        /* 
        {
            pipeline: todo!(),
            bindings: todo!(),
            vertex_buffer: todo!(),
            index_buffer: todo!(),
            white_pixel: todo!(),
            batch_vertex_buffer: todo!(),
            batch_index_buffer: todo!(),
        }*/
    }
}