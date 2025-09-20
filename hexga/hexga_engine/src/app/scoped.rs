use winit::event_loop::ActiveEventLoop;

use super::*;


// Wrapper arround Scoped, but non public

//pub struct ScopePause;


/* 
pub(crate) trait ScopedPaused
{
    fn begin_paused(&mut self);
    fn end_paused(&mut self);
    fn scoped_paused<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_paused(); let r = f(); self.end_paused(); r }
}

pub(crate) trait ScopedResumed
{
    fn begin_resumed(&mut self);
    fn end_resumed(&mut self);
    fn scoped_resumed<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_resumed(); let r = f(); self.end_resumed(); r }
}

pub(crate) trait ScopedUpdate
{
    fn begin_update(&mut self);
    fn end_update(&mut self);
    fn scoped_update<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_update(); let r = f(); self.end_update(); r }
}

pub(crate) trait ScopedDraw
{
    fn begin_draw(&mut self);
    fn end_draw(&mut self);
    fn scoped_draw<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_draw(); let r = f(); self.end_draw(); r }
}

pub(crate) trait ScopedEvent<E>
{
    fn begin_event(&mut self, ev: &AppEvent<E>);
    fn end_event(&mut self);
    fn scoped_event<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_event(); let r = f(); self.end_event(); r }
}
*/
/* 
pub(crate) trait ScopedEvent
{
    /* 
    fn dispatch<'a,C,F,R>(it: impl Iterator<Item = &'a mut C>, f:) where C: ScopedEvent + 'a, F:FnMut()
    {

    }
    */
    fn childs(&mut self) -> Option<impl Iterator<Item = &mut ScopedEvent>> { None }
    fn scope(&mut self, f: impl Fn(&mut ScopedEvent))
    {
        self.begin_scoped();

        let r = match self.childs()
        {
            Some(childs) => 
            {
                for c in self.childs()
                {
                    c.scope
                }
            },
            None => f(),
        };

        self.end_scoped();
        r
    }

    fn begin_scoped(&mut self) {}
    fn end_scoped(&mut self) {}

    fn scoped_resumed<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }
    fn scoped_paused<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }
    fn scoped_update<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }
    fn scoped_draw<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R { self.begin_scoped(); let r = f(); self.end_scoped(); r }
    fn scoped_input<F,R>(&mut self, input: InputEvent, f: F) -> R where F: FnOnce(InputEvent) -> R { self.begin_scoped(); let r = f(input); self.end_scoped(); r }
}
*/

pub(crate) struct MessageCtx<'a,E> where E:IEvent
{
    event_loop: &'a ActiveEventLoop,
    proxy: &'a EventLoopProxy<E>
}

/// `begin_X()` are called before the application, `end_X()` are called after
pub(crate) trait ScopedMessage<E> where E:IEvent
{
    /*
    fn scoped_event<F,R>(&mut self, ev: AppEvent<E>, f: F) -> R where F: FnOnce(AppEvent<E>) -> R
    {
        self.begin_event(&ev);
        let r = f(ev);
        self.end_event();
        r
    }
    fn begin_event(&mut self, ev: &AppEvent<E>) { let _ = ev;}
    fn end_event(&mut self) { }
    */

    fn scoped_custom<F,R>(&mut self, custom: E, f: F, el: &EventLoopActive) -> R where F: FnOnce(E) -> R
    {
        self.begin_custom(&custom, el);
        let r = f(custom);
        self.end_custom(el);
        r
    }
    fn begin_custom(&mut self, custom: &E, el: &EventLoopActive) { let _  = (custom, el); }
    fn end_custom(&mut self, el: &EventLoopActive) { let _ = el; }


    fn scoped_input<F,R>(&mut self, input: InputEvent, f: F, el: &EventLoopActive) -> R where F: FnOnce(InputEvent) -> R
    {
        self.begin_input(&input, el);
        let r = f(input);
        self.end_input(el);
        r
    }
    fn begin_input(&mut self, input: &InputEvent, el: &EventLoopActive) 
    {
        match input
        {
            InputEvent::Key(key) => self.begin_input_key(key, el),
        }
    }
    fn end_input(&mut self, el: &EventLoopActive) { let _ = el;}

    fn begin_input_key(&mut self, input_key: &KeyEvent, el: &EventLoopActive) { let _ = (input_key, el); }



    fn scoped_flow<F,R>(&mut self, flow: FlowMessage, f: F, el: &EventLoopActive) -> R where F: FnOnce(FlowMessage) -> R
    {
        self.begin_flow(flow, el);
        let r = f(flow);
        self.end_flow(flow, el);
        r
    }
    fn begin_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) { self.dispatch_begin_flow(flow, el) }
    fn dispatch_begin_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) 
    {
        match flow
        {
            FlowMessage::Resumed => self.begin_flow_resumed(el),
            FlowMessage::Paused => self.begin_flow_paused(el),
            FlowMessage::Update => self.begin_flow_update(el),
            FlowMessage::Draw => self.begin_flow_draw(el),
        }
    }
    fn end_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) { self.dispatch_end_flow(flow, el) }
    fn dispatch_end_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) 
    {
        match flow
        {
            FlowMessage::Resumed => self.end_flow_resumed(el),
            FlowMessage::Paused => self.end_flow_paused(el),
            FlowMessage::Update => self.end_flow_update(el),
            FlowMessage::Draw => self.end_flow_draw(el),
        }
    }


    fn scoped_flow_paused<F,R>(&mut self, f: F, el: &EventLoopActive) -> R where F: FnOnce() -> R
    {
        self.begin_flow_paused(el);
        let r = f();
        self.end_flow_paused(el);
        r
    }
    fn begin_flow_paused(&mut self, el: &EventLoopActive) { let _ = el; }
    fn end_flow_paused(&mut self, el: &EventLoopActive) { let _ = el; }


    fn scoped_flow_resumed<F,R>(&mut self, f: F, el: &EventLoopActive) -> R where F: FnOnce() -> R
    {
        self.begin_flow_resumed(el);
        let r = f();
        self.end_flow_resumed(el);
        r
    }
    fn begin_flow_resumed(&mut self, el: &EventLoopActive) { let _ = el; }
    fn end_flow_resumed(&mut self, el: &EventLoopActive) { let _ = el; }


    fn scoped_flow_update<F,R>(&mut self, f: F, el: &EventLoopActive) -> R where F: FnOnce() -> R
    {
        self.begin_flow_update(el);
        let r = f();
        self.end_flow_update(el);
        r
    }
    fn begin_flow_update(&mut self, el: &EventLoopActive) { let _ = el; }
    fn end_flow_update(&mut self, el: &EventLoopActive) { let _ = el; }


    fn scoped_flow_draw<F,R>(&mut self, f: F, el: &EventLoopActive) -> R where F: FnOnce() -> R
    {
        self.begin_flow_draw(el);
        let r = f();
        self.end_flow_draw(el);
        r
    }
    fn begin_flow_draw(&mut self, el: &EventLoopActive) { let _ = el; }
    fn end_flow_draw(&mut self, el: &EventLoopActive) { let _ = el; }
}
