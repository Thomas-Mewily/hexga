use super::*;

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
pub(crate) fn replace_ctx(mut ctx : Option<Context>) -> Option<Context>
{
    std::mem::swap(&mut ctx, unsafe { &mut CONTEXT });
    ctx
}


#[derive(Debug, Default)]
pub struct Context
{
    pub input : CtxInput,
}
impl Context
{
    pub fn new() -> Self { ___() }
    /*
    pub fn input_ref(&self) -> &CtxInput { &self.input }
    pub fn input(&mut self) -> &mut CtxInput { &mut self.input }
    */
}

macro_rules! declare_context {
    ($struct_name:ident, $target_type:ty, $field:ident) =>
    {
        pub struct $struct_name;

        impl std::ops::Deref for $struct_name {
            type Target = $target_type;
            fn deref(&self) -> &Self::Target {
                &ctx().$field
            }
        }

        impl std::ops::DerefMut for $struct_name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut ctx_mut().$field
            }
        }
    };
}
pub(crate) use declare_context;


pub trait App
{
    fn update(&mut self);
    fn draw(&mut self);
}