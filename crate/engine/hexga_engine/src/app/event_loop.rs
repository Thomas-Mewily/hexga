use super::*;

pub struct AppEventLoop<'a, User>
    where User: AppUserEvent
{
    event_loop : &'a WinitEventLoopActive,
    inner : &'a mut AppContextField,
    proxy: &'a AppProxy<User>,
}
impl<'a,User> AppEventLoop<'a,User>
    where User: AppUserEvent
{
    pub fn winit(&self) -> &WinitEventLoopActive { self.event_loop }
}
impl<'a, User> AppEventLoop<'a, User>
    where User: AppUserEvent
{
    pub(crate) fn new(event_loop: &'a WinitEventLoopActive, inner : &'a mut AppContextField, proxy: &'a AppProxy<User>) -> Self {
        Self { event_loop, inner, proxy }
    }
    pub fn proxy(&mut self) -> &AppProxy<User> { self.proxy }
    pub fn time(&mut self) -> &mut TimeManager { &mut self.inner.time }
}

pub(crate) struct AppContextField
{
    time : TimeManager,
} 
impl AppContextField
{
    pub(crate) fn new(time : TimeManager) -> Self { Self{ time }}
}
