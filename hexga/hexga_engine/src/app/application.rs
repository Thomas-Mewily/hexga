use super::*;


/*
pub trait EventHandler<E>
{
    fn event(&mut self, event: E);
    fn update(&mut self);
}
*/

pub trait Application: 'static + Sized
{
    fn event(&mut self, ev: AppEvent) { let _ = ev; }

    fn resumed(&mut self) {}
    fn paused(&mut self) {}

    fn update(&mut self) { }
    fn draw(&mut self) { }
}