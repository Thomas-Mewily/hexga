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
    fn push_scissor(&mut self, scissor: Scissor) -> Self::Output {
        Push::push(self, scissor)
    }
}

impl<T> ScissorStackPush for T
where
    T: Push<Scissor>,
{
}

pub trait ScissorStackDup<Idx = DrawStackIndex<Scissor>>: Dup<Idx, Output = Scissor> {
    fn dup_scissor(&mut self) -> bool {
        self.dup()
    }
}

impl<T, Idx> ScissorStackDup<Idx> for T
where
    T: Dup<Idx, Output = Scissor>,
{
}

pub trait ScissorStackPop: Pop<Scissor> {
    fn pop_scissor(&mut self) -> Option<Scissor> {
        Pop::pop(self)
    }
}

impl<T> ScissorStackPop for T
where
    T: Pop<Scissor>,
{
}

pub trait ScissorStackExtension<Idx = DrawStackIndex<Scissor>>:
    ScissorStackPush + ScissorStackDup<Idx> + ScissorStackPop
{}
impl<T, Idx> ScissorStackExtension<Idx> for T
where
    T: ScissorStackPush + ScissorStackDup<Idx> + ScissorStackPop,
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
