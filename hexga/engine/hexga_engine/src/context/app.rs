use super::*;

pub trait IUserEvent : 'static {}
impl IUserEvent for () {}


pub trait App<UserEvent=()> where UserEvent:IUserEvent
{
    fn pause(&mut self) {}
    fn resume(&mut self) {}

    fn update(&mut self) {}
    fn draw(&mut self) {}
}