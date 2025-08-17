use crate::*;

pub mod prelude
{
    
}

pub struct Ctx;
impl Deref for Ctx
{
    type Target=Context;
    fn deref(&self) -> &Self::Target { ctx() }
}
impl DerefMut for Ctx
{
    fn deref_mut(&mut self) -> &mut Self::Target { ctx_mut() }
}

static mut CONTEXT : Option<Context> = None;
#[allow(static_mut_refs)]
pub(crate) fn ctx_mut() -> &'static mut Context { unsafe { CONTEXT.as_mut().expect("Ctx not initialized") } }
#[allow(static_mut_refs)]
pub(crate) fn ctx() -> &'static Context { unsafe { CONTEXT.as_ref().expect("Ctx not initialized") } }

#[allow(static_mut_refs)]
pub(crate) fn ctx_mut_or_init() -> &'static mut Context { init_ctx_if_needed(); ctx_mut() }
#[allow(static_mut_refs)]
pub(crate) fn ctx_or_init() -> &'static Context { init_ctx_if_needed(); ctx() }

#[allow(static_mut_refs)]
pub(crate) fn init_ctx_if_needed()
{
    unsafe
    {
        if CONTEXT.is_none()
        {
            CONTEXT = Some(Context::new());
        }
        std::panic::set_hook(Box::new(|info| {
            CONTEXT = None;
            eprintln!("Panic occurred: {info}");
        }));
    }
}

pub(crate) fn reset_ctx()
{
    unsafe { CONTEXT = None };
}

#[derive(Debug, Default)]
pub struct Context
{
    pub input: ContextInput,
    pub pen: ContextPen,
}
impl Context
{
    pub fn new() -> Self { ___() }
}

#[derive(Debug, Default)]
pub struct ContextPen
{

}

#[derive(Debug, Default)]
pub struct ContextAudio
{

}

#[derive(Debug, Default)]
pub struct ContextInput
{

}