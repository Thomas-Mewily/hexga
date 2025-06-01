use crate::*;

pub mod prelude
{
    pub use super::MainLoop;
}

pub trait MainLoop
{
    fn handle_event(&mut self, event : Event) -> bool;
    fn update(&mut self);
    fn draw(&mut self);
}

pub struct Context
{
    pub(crate) thread_id : std::thread::ThreadId,
    // state       : Box<dyn MainLoopWithContext>,
    pub(crate) multi_media : Box<dyn ContextMultiMedia>, // use an Arc instead ? and lock it during draw ?

    //pub(crate) render : ContextRender,
    pub(crate) pen    : ContextPen,


    // Because Context is not Send and not Sync
    _marker: PhantomData<std::rc::Rc<()>>,

/*
Window,
Asset,
Audio,
Pen,
Events,
Permission ? (from where asset can be loaded / exported ? You don't want to load and share the "asset" "C:/private/password.txt")
*/


    // other stuff
}

/*
impl Context
{
    pub fn new(mut multi_media : Box<dyn ContextMultiMedia>, param : MultiMediaParam) -> Self
    {
        let pen = ContextPen::new(multi_media.as_mut(), param.pen_param);
        Self { multi_media, pen, textures: ___(), textures_to_remove: ___() }
    }
}
*/

pub(crate) static mut CONTEXT : Option<Context> = None;

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
pub(crate) fn ctx_ref() -> &'static Context
{
    let ctx = unsafe { CONTEXT.as_ref().unwrap() };
    assert_eq!(ctx.thread_id, std::thread::current().id(), "Can only use the context in the same thread");
    ctx
}

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static mut Context
{
    let ctx = unsafe { CONTEXT.as_mut().unwrap() };
    assert_eq!(ctx.thread_id, std::thread::current().id(), "Can only use the context in the same thread");
    ctx
}

#[doc(hidden)]
#[allow(static_mut_refs)]
pub unsafe fn set_context(ctx : Option<Context>) -> Option<Context>
{
    unsafe
    {
        match ctx
        {
            Some(ctx) =>
            {
                CONTEXT = Some(ctx);
                return None;
            },
            None => {
                CONTEXT.take()
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
