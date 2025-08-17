use crate::*;

pub mod prelude
{
    pub use super::{App,AppRunner,IInput,IUserEvent};
}

pub trait IInput
{

}

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}

pub trait App<UserEvent=()>
{
    fn event(&mut self) {}
    fn update(&mut self) {}
    fn draw(&mut self) {}
}

impl<T, UserEvent> AppRunner<UserEvent> for T where T: App<UserEvent>, UserEvent: IUserEvent {}
pub trait AppRunner<UserEvent> : App<UserEvent> where UserEvent: IUserEvent, Self: Sized
{
    fn run(self) { self.run_with_param(___()); }

    fn run_with_param(self, param: AppParam)
    {
        
    }
}

pub struct AppParam
{
    pub window : Option<WindowParam>,
}
impl Default for AppParam
{
    fn default() -> Self {
        Self { window: Some(WindowParam::new()) }
    }
}

impl AppParam
{
    pub fn new() -> Self { ___() }
    pub fn with_window(mut self, window : impl Into<Option<WindowParam>>) -> Self  { self.window = window.into(); self }
}