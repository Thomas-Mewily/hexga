use super::*;

pub type Scissor = Rect2i;

pub trait GetScissor
{
    fn scissor(&self) -> Scissor;
}
pub trait SetScissor
{
    fn set_scissor(&mut self, scissor: Scissor) -> &mut Self;
}

pub trait ScissorStackPush: Push<Scissor> {
    fn push_scissor(&mut self, scissor: Scissor) {
        let _ = Push::push(self, scissor);
    }
}

impl<T> ScissorStackPush for T
where
    T: Push<Scissor>,
{
}

pub trait ScissorStackDup: GetScissor + Push<Scissor> {
    fn dup_scissor(&mut self)
    {
        Push::push(self, self.scissor());
    }
}

impl<T> ScissorStackDup for T
where
    T: GetScissor + Push<Scissor>
    {
}

pub trait ScissorStackPop: Pop<Scissor> {
    fn pop_scissor(&mut self) {
        let _ = Pop::pop(self);
    }
}

impl<T> ScissorStackPop for T
where
    T: Pop<Scissor>,
{
}

pub trait ScissorStackExtension:
    ScissorStackPush + ScissorStackDup + ScissorStackPop
{}
impl<T> ScissorStackExtension for T
where
    T: ScissorStackPush + ScissorStackDup + ScissorStackPop,
{}

pub mod prelude
{
    pub use super::Scissor;
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::{GetScissor, SetScissor, 
        ScissorStackExtension, ScissorStackPush, ScissorStackDup, ScissorStackPop};
}