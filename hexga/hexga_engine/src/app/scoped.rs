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

/// `begin_X()` are called before the application, `end_X()` are called after
pub(crate) trait ScopedMessage
{
    fn scoped_event<F,R>(&mut self, ev: AppEvent, f: F) -> R where F: FnOnce(AppEvent) -> R
    {
        self.begin_event(&ev);
        let r = f(ev);
        self.end_event();
        r
    }
    fn begin_event(&mut self, ev: &AppEvent) { let _ = ev;}
    fn end_event(&mut self) { }



    fn scoped_input<F,R>(&mut self, input: InputEvent, f: F) -> R where F: FnOnce(InputEvent) -> R
    {
        self.begin_input(&input);
        let r = f(input);
        self.end_input();
        r
    }
    fn begin_input(&mut self, input: &InputEvent) { let _ = input;}
    fn end_input(&mut self) { }



    fn scoped_flow<F,R>(&mut self, flow: FlowMessage, f: F) -> R where F: FnOnce(FlowMessage) -> R
    {
        self.begin_flow(flow);
        let r = f(flow);
        self.end_flow(flow);
        r
    }
    fn begin_flow(&mut self, flow: FlowMessage) 
    {
        match flow
        {
            FlowMessage::Resumed => self.begin_resumed(),
            FlowMessage::Paused => self.begin_paused(),
            FlowMessage::Update => self.begin_update(),
            FlowMessage::Draw => self.begin_draw(),
        }
    }
    fn end_flow(&mut self, flow: FlowMessage) 
    {
        match flow
        {
            FlowMessage::Resumed => self.end_resumed(),
            FlowMessage::Paused => self.end_paused(),
            FlowMessage::Update => self.end_update(),
            FlowMessage::Draw => self.end_draw(),
        }
    }


    fn scoped_paused<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_paused();
        let r = f();
        self.end_paused();
        r
    }
    fn begin_paused(&mut self) {}
    fn end_paused(&mut self) {}


    fn scoped_resumed<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_resumed();
        let r = f();
        self.end_resumed();
        r
    }
    fn begin_resumed(&mut self) {}
    fn end_resumed(&mut self) {}


    fn scoped_update<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_update();
        let r = f();
        self.end_update();
        r
    }
    fn begin_update(&mut self) {}
    fn end_update(&mut self) {}


    fn scoped_draw<F,R>(&mut self, f: F) -> R where F: FnOnce() -> R
    {
        self.begin_draw();
        let r = f();
        self.end_draw();
        r
    }
    fn begin_draw(&mut self) {}
    fn end_draw(&mut self) {}
}
