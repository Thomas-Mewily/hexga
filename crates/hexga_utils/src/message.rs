pub mod prelude
{
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::MessageHandler;
}

pub trait MessageHandler<E, Ctx>
{
    fn message(&mut self, message: E, ctx: &mut Ctx);
    //fn update(&mut self);
}
