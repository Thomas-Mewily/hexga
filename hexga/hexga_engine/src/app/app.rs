use super::*;

pub trait IInput
{

}

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}

pub trait App<UserEvent=()> where UserEvent: IUserEvent
{
    fn event(&mut self, event: AppEvent<UserEvent>) { self.dispatch_event(event) }
    fn dispatch_event(&mut self, event: AppEvent<UserEvent>) { let _ = event; }
    
    fn update(&mut self) {}
    fn draw(&mut self) {}
}
