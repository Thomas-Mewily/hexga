use super::*;

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
