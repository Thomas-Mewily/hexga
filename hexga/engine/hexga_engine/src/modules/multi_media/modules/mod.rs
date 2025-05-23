use crate::*;

pub use engine_core::multi_media::modules::*;

pub trait MainLoop
{
    fn handle_event(&mut self, event : Event) -> bool;
    fn update(&mut self);
    fn draw(&mut self);
}

pub(crate) struct Context
{
    //state       : Box<dyn MainLoopWithContext>,
    multi_media : Box<dyn ContextMultiMedia>,
    // other stuff
}

static mut CONTEXT : Option<Context> = None;

#[allow(static_mut_refs)]
pub(crate) fn ctx_ref() -> &'static Context  { unsafe { CONTEXT.as_ref().unwrap() } }

#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static mut Context  { unsafe { CONTEXT.as_mut().unwrap() } }

#[doc(hidden)]
#[allow(static_mut_refs)]
pub unsafe fn set_context(ctx : Option<Box<dyn ContextMultiMedia>>) -> Option<Box<dyn ContextMultiMedia>>
{
    unsafe
    {
        match ctx
        {
            Some(ctx) => {
                CONTEXT = Some(Context::new(ctx));
                return None;
            },
            None => {
                CONTEXT.take().map(|v| v.multi_media)
            }
        }
    }
}


impl Context
{
    pub(crate) fn new(multi_media : Box<dyn ContextMultiMedia>) -> Self 
    {
        Self { multi_media }
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
