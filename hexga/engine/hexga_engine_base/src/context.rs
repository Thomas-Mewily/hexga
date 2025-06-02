use crate::*;

pub mod prelude
{
    pub(crate) use super::Ctx;
    pub(crate) use super::define_ctx_type;
}

macro_rules! define_ctx_type {
    ($name:ident, $ctx_type:ident) => {
        pub struct $name;
        impl Deref for $name { type Target=$ctx_type<'static>; fn deref(&self) -> &Self::Target { unsafe { std::mem::transmute($crate::context::ctx()) } } }
        impl DerefMut for $name { fn deref_mut(&mut self) -> &mut Self::Target { unsafe { std::mem::transmute($crate::context::ctx()) } } }
        impl AsRef<$ctx_type<'static>> for Pen { fn as_ref(&self) -> &$ctx_type<'static> { &*self } }
        impl AsMut<$ctx_type<'static>> for Pen { fn as_mut(&mut self) -> &mut $ctx_type<'static> { &mut *self } }
        pub type $ctx_type<'a> = $crate::context::CtxMut<'a, $name>;
    };
}
pub(crate) use define_ctx_type;

pub struct CtxMut<'a,T> { ctx : &'a mut Ctx, phantom : PhantomData<T>, }
impl<'a,T> CtxMut<'a,T>
{
    pub(crate) const fn new(ctx : &'a mut Ctx) -> Self { Self { ctx, phantom: PhantomData }}
    pub(crate) fn ctx(&mut self) -> &mut Ctx { self.ctx }
}



#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default)]
pub struct Context;
impl Deref for Context { type Target=Ctx; fn deref(&self) -> &Self::Target { ctx_ref() } }
impl DerefMut for Context { fn deref_mut(&mut self) -> &mut Self::Target { ctx() } }
impl AsRef<Ctx> for Context { fn as_ref(&self) -> &Ctx { &*self } }
impl AsMut<Ctx> for Context { fn as_mut(&mut self) -> &mut Ctx { &mut *self } }

pub struct Ctx
{
    pub(crate) thread_id : std::thread::ThreadId,
    // state       : Box<dyn MainLoopWithContext>,
    pub(crate) multi_media : Box<dyn ContextMultiMedia>, // use an Arc instead ? and lock it during draw ?

    pub(crate) pen : PenInternal,
    //pub(crate) render : ContextRender,
    //pub(crate) pen    : ContextPen,


    // Because Context is not Send and not Sync
    //_marker: PhantomData<std::rc::Rc<()>>,

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

impl Ctx
{
    pub fn new(mut multi_media : Box<dyn ContextMultiMedia>, param : MultiMediaParam) -> Self
    {
        //let pen = ContextPen::new(multi_media.as_mut(), param.pen_param);
        //Self { multi_media, pen, textures: ___(), textures_to_remove: ___() }

        let pen = PenInternal::new(multi_media.as_mut());

        Self
        {
            thread_id: std::thread::current().id(),
            multi_media,
            pen,
        }
    }
}

impl Ctx
{
    pub fn pen<'a>(&'a mut self) -> CtxPen<'a> { CtxPen::new(self) }
}

pub(crate) static mut CONTEXT : Option<Ctx> = None;

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
pub(crate) fn ctx_ref() -> &'static Ctx
{
    let ctx = unsafe { CONTEXT.as_ref().unwrap() };
    assert_eq!(ctx.thread_id, std::thread::current().id(), "Can only use the context in the same thread");
    ctx
}

#[doc(hidden)]
#[allow(dead_code)]
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static mut Ctx
{
    let ctx = unsafe { CONTEXT.as_mut().unwrap() };
    assert_eq!(ctx.thread_id, std::thread::current().id(), "Can only use the context in the same thread");
    ctx
}

#[doc(hidden)]
#[allow(static_mut_refs)]
pub unsafe fn set_context(ctx : Option<Ctx>) -> Option<Ctx>
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

pub struct ContextRunner<S> where S : MainLoopWithContext
{
    pub state : S,
}
impl<S> ContextRunner<S> where S : MainLoopWithContext
{
    pub fn new(state : S) -> Self { Self { state } }
}

impl<S> MainLoopWithContext for ContextRunner<S> where S : MainLoopWithContext
{
    fn handle_event_with(&mut self, event : Event, ctx : &mut Ctx) -> bool {
        self.state.handle_event_with(event, ctx)
    }

    fn update_with(&mut self, ctx : &mut Ctx) {
        self.state.update_with(ctx)
    }

    fn draw_with(&mut self, ctx : &mut Ctx) {
        self.state.draw_with(ctx)
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
