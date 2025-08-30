use super::*;

pub trait IInput
{

}

pub trait IUserEvent : 'static + Debug + Send {}
impl IUserEvent for () {}

pub trait App<UserEvent=()> where UserEvent: IUserEvent
{
    fn event(&mut self, event: AppEvent<UserEvent>) { self.dispatch_event(event) }
    fn dispatch_event(&mut self, event: AppEvent<UserEvent>) 
    {
        match event
        {
            AppEvent::UserEvent(_) => {},
            AppEvent::Window(window_event) => {},
            AppEvent::State(state) => self.dispatch_event_state(state),
        }
    }

    fn dispatch_event_state(&mut self, state : StateEvent)
    {
        match state
        {
            StateEvent::Paused => self.paused(),
            StateEvent::Resumed => self.resumed(),
            StateEvent::Update => self.update(),
            StateEvent::Draw => self.draw(),
        }
    }

    fn paused(&mut self) {}
    fn resumed(&mut self) {}
    
    fn update(&mut self) {}
    fn draw(&mut self) {}
}
