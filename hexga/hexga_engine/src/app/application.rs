use super::*;


pub trait IEvent : 'static + Debug + Send {}
impl<T> IEvent for T where T: 'static + Debug + Send {}

/// An application receive message from the Event Loop
/// 
/// Those message can be flow message (Update, Draw, Paused, Resumed...),
/// or Event
pub trait Application<E=()>: 'static where E:IEvent
{
    fn handle_message(&mut self, message: AppMessage<E>) { self.dispatch_message(message) }
    fn dispatch_message(&mut self, message: AppMessage<E>)
    {
        match message
        {
            AppMessage::Flow(flow) => self.handle_flow(flow),
            AppMessage::Event(event) => self.handle_event(event),
        }
    }


    fn handle_event(&mut self, ev: AppEvent<E>) { let _ = ev; }

    fn handle_flow(&mut self, flow: FlowMessage){ self.dispatch_flow(flow); }
    fn dispatch_flow(&mut self, flow: FlowMessage)
    { 
        match flow
        {
            FlowMessage::Resumed => self.resumed(),
            FlowMessage::Paused => self.paused(),
            FlowMessage::Update => self.update(),
            FlowMessage::Draw => self.draw(),
        }
    }

    fn resumed(&mut self) {}
    fn paused(&mut self) {}

    fn update(&mut self) { }
    fn draw(&mut self) { }
}


