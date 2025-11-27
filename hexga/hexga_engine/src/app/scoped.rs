use super::*;

/// `begin_X()` are called before the application, `end_X()` are called after
pub(crate) trait ScopedFlow
{
    /*
    fn scoped_custom<F,R>(&mut self, custom: CustomEvent, f: F) -> R where F: FnOnce(CustomEvent) -> R
    {
        self.begin_custom(&custom);
        let r = f(custom);
        self.end_custom();
        r
    }
    fn begin_custom(&mut self, custom: &CustomEvent) { let _  = custom; }
    fn end_custom(&mut self) {}
    */

    fn scoped_flow<F,R>(&mut self, flow: FlowMessage, f: F) -> R where F: FnOnce(FlowMessage) -> R
    {
        self.begin_flow(flow);
        let r = f(flow);
        self.end_flow(flow);
        r
    }
    fn begin_flow(&mut self, flow: FlowMessage) { self.dispatch_begin_flow(flow) }
    fn dispatch_begin_flow(&mut self, flow: FlowMessage)
    {
        match flow
        {
            FlowMessage::Resumed => self.begin_flow_resumed(),
            FlowMessage::Paused => self.begin_flow_paused(),
            FlowMessage::Update(dt) => self.begin_flow_update(dt),
            FlowMessage::Draw => self.begin_flow_draw(),
        }
    }
    fn end_flow(&mut self, flow: FlowMessage) { self.dispatch_end_flow(flow) }
    fn dispatch_end_flow(&mut self, flow: FlowMessage)
    {
        match flow
        {
            FlowMessage::Resumed => self.end_flow_resumed(),
            FlowMessage::Paused => self.end_flow_paused(),
            FlowMessage::Update(dt) => self.end_flow_update(dt),
            FlowMessage::Draw => self.end_flow_draw(),
        }
    }


    fn scoped_flow_paused<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_flow_paused();
        let r = f();
        self.end_flow_paused();
        r
    }
    fn begin_flow_paused(&mut self) { }
    fn end_flow_paused(&mut self) { }


    fn scoped_flow_resumed<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_flow_resumed();
        let r = f();
        self.end_flow_resumed();
        r
    }
    fn begin_flow_resumed(&mut self) { }
    fn end_flow_resumed(&mut self) { }


    fn scoped_flow_update<F,R>(&mut self, dt: DeltaTime, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_flow_update(dt);
        let r = f();
        self.end_flow_update(dt);
        r
    }
    fn begin_flow_update(&mut self, dt: DeltaTime) { }
    fn end_flow_update(&mut self, dt: DeltaTime) { }


    fn scoped_flow_draw<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_flow_draw();
        let r = f();
        self.end_flow_draw();
        r
    }
    fn begin_flow_draw(&mut self) { }
    fn end_flow_draw(&mut self) { }
}