use super::*;

pub mod prelude
{
    pub use super::{Input,ContextInput};
}


ctx_singleton!(
    Input,
    ContextInput,
    { Ctx::try_as_ref().map(|ctx| &ctx.input) },
    { Ctx::try_as_mut().map(|ctx| &mut ctx.input) }
);


#[derive(Default, Clone, PartialEq, Debug)]
pub struct ContextInput
{

}
