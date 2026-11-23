use super::*;


#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Viewport
{
    pub area: Rect2,
    pub depth: Range<float>,
}

impl Default for Viewport
{
    fn default() -> Self
    {
        Self {
            area: ___(),
            depth: Range { start: 0., end: 1. },
        }
    }
}

pub trait GetViewport
{
    fn viewport(&self) -> Viewport;
}
pub trait SetViewport
{
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self;
}

impl GetViewport for DrawParam
{
    fn viewport(&self) -> Viewport { self.viewport }
}
impl SetViewport for DrawParam
{
    fn set_viewport(&mut self, viewport: Viewport) -> &mut Self
    {
        self.viewport = viewport;
        self
    }
}

pub trait ViewportStackPush: Push<Viewport> {
    fn push_viewport(&mut self, viewport: Viewport) {
        let _ = Push::push(self, viewport);
    }
}

impl<T> ViewportStackPush for T
where
    T: Push<Viewport>,
{
}

pub trait ViewportStackDup: GetViewport + Push<Viewport> {
    fn dup_viewport(&mut self)
    {
        Push::push(self, self.viewport());
    }
}

impl<T> ViewportStackDup for T
where
    T: GetViewport + Push<Viewport>
{
}

pub trait ViewportStackPop: Pop<Viewport> {
    fn pop_viewport(&mut self) {
        let _ = Pop::pop(self);
    }
}

impl<T> ViewportStackPop for T
where
    T: Pop<Viewport>,
{
}

pub trait ViewportStackExtension:
    ViewportStackPush + ViewportStackDup + ViewportStackPop
{}
impl<T> ViewportStackExtension for T
where
    T: ViewportStackPush + ViewportStackDup + ViewportStackPop,
{}

pub mod prelude
{
    pub use super::Viewport;
    pub use super::traits::*;
}
pub mod traits
{
    pub use super::{GetViewport, SetViewport, 
        ViewportStackExtension, ViewportStackPush, ViewportStackDup, ViewportStackPop};
}