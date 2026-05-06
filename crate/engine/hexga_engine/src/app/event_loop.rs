use super::*;

pub struct AppEventLoop<'a>
{
    event_loop : &'a WinitEventLoopActive,
    inner : &'a mut AppEventLoopInner,
}
impl<'a> AppEventLoop<'a>
{
    pub fn winit(&self) -> &WinitEventLoopActive { self.event_loop }
}
impl<'a> AppEventLoop<'a>
{
    pub(crate) fn new(event_loop: &'a WinitEventLoopActive, inner : &'a mut AppEventLoopInner) -> Self {
        Self { event_loop, inner }
    }
}


pub(crate) struct AppEventLoopInner
{

} 
impl AppEventLoopInner
{
    pub(crate) fn new() -> Self { Self{}}
}
