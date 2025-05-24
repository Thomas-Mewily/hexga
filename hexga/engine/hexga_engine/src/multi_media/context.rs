use crate::{pen::ContextPen, *};

pub use engine_core::multi_media::modules::*;

pub trait MainLoop
{
    fn handle_event(&mut self, event : Event) -> bool;
    fn update(&mut self);
    fn draw(&mut self);
}

pub(crate) struct Context
{
    // state       : Box<dyn MainLoopWithContext>,
    multi_media : Box<dyn ContextMultiMedia>,
    pen         : ContextPen,

    textures : GenVec<RawTextureID>,
    textures_to_remove : Vec<GenID<RawTextureID>>,
    
    // other stuff
}

impl Context
{
    pub(crate) fn new(mut multi_media : Box<dyn ContextMultiMedia>, param : MultiMediaParam) -> Self 
    {
        let pen = ContextPen::new(multi_media.as_mut(), param.pen_param);
        Self { multi_media, pen, textures: ___(), textures_to_remove: ___() }
    }
}

static mut CONTEXT : Option<Context> = None;

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
fn hexga_context_ref() -> &'static Context  { unsafe { CONTEXT.as_ref().unwrap() } }

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
fn hexga_context() -> &'static mut Context  { unsafe { CONTEXT.as_mut().unwrap() } }

#[doc(hidden)]
#[allow(static_mut_refs)]
pub unsafe fn set_context(ctx : Option<(Box<dyn ContextMultiMedia>, MultiMediaParam)>) -> Option<Box<dyn ContextMultiMedia>>
{
    unsafe
    {
        match ctx
        {
            Some((ctx, param)) => {
                CONTEXT = Some(Context::new(ctx, param));
                return None;
            },
            None => {
                CONTEXT.take().map(|v| v.multi_media)
            }
        }
    }
}



/* 
impl<S,MultiMedia> MainLoopWithContext for ContextState<S,MultiMedia> 
    where S : MainLoopWithContextMultiMedia, MultiMedia : ContextMultiMedia
{
    fn handle_event(&mut self, event : Event, ctx : &mut Context) -> bool 
    {
        let mut c = Context { multi_media: ctx };
        self.state.handle_event(event, &mut c)
    }

    fn update(&mut self, ctx : &mut Context) {
        todo!()
    }

    fn draw(&mut self, ctx : &mut Context) {
        todo!()
    }
}
*/
