use super::*;


pub trait IEvent : 'static + Debug + Send {}
impl<T> IEvent for T where T: 'static + Debug + Send {}

pub trait Application<E=()>: 'static where E:IEvent
{
    fn handle_event(&mut self, ev: AppEvent<E>) { self.dispatch_event(ev); }

    fn update(&mut self) { }
    fn draw(&mut self) { }


    fn pause(&mut self) {}
    fn resume(&mut self) {}

    fn enter(&mut self) {}
    fn leave(&mut self) {}

    //fn redirect_mut(&mut self) -> Option<&dyn Application<E>>; 






    fn dispatch_event(&mut self, ev: AppEvent<E>)
    {
        match ev
        {
            AppEvent::Input(i) => self.handle_input(i),
            AppEvent::Custom(c) => self.handle_custom(c),
        }
    }

    fn handle_flow(&mut self, flow: FlowMessage) { self.dispatch_flow(flow); }
    fn dispatch_flow(&mut self, flow: FlowMessage)
    {
        match flow
        {
            FlowMessage::Resumed => self.resumed(),
            FlowMessage::Paused => self.pause(),
            FlowMessage::Update => self.update(),
            FlowMessage::Draw => self.draw(),
            FlowMessage::Exit => self.exit(),
        }
    }

    fn handle_custom(&mut self, custom: E) { let _ = custom; }
    fn handle_input(&mut self, input: InputEvent) { let _ = input; }
}


