use super::*;

/*
pub struct AppLoop<'a, User>
    where User: AppUserEvent
{
    event_loop : &'a WinitEventLoopActive,
    pub time : &'a mut TimeManager,
    pub proxy: &'a AppProxy<User>,
    pub param: &'a AppParam,
}
impl<'a,User> AppLoop<'a,User>
    where User: AppUserEvent
{
    pub fn winit(&self) -> &WinitEventLoopActive { self.event_loop }
}
impl<'a, User> AppLoop<'a, User>
    where User: AppUserEvent
{
    pub(crate) fn new(event_loop: &'a WinitEventLoopActive, time : &'a mut TimeManager, proxy: &'a AppProxy<User>, param: &'a AppParam) -> Self {
        Self { event_loop, proxy, time, param }
    }
}
*/