use super::*;

pub mod prelude
{
    pub use super::App;
}

pub trait App : 'static
{
    type UserEvent : IUserEvent;

    fn update(&mut self, dt: DeltaTime) {}
    fn draw(&mut self) {}
}