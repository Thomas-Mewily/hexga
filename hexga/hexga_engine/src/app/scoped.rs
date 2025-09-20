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
pub(crate) trait ScopedEvent<E> where E:IEvent
{
    fn scoped_event<F,R>(&mut self, ev: AppEvent<E>, f: F) -> R where F: FnOnce(AppEvent<E>) -> R 
    { 
        match ev
        {
            AppEvent::Input(i) => { self.begin_input(&i); let r = f(AppEvent::Input(i.clone())); self.end_input(&i); self.end_event(); r },
            AppEvent::Custom(c) => { self.begin_custom(&c); let r = f(AppEvent::Custom(c)); self.end_custom(); self.end_event(); r },
        }
    }
    fn begin_event(&mut self, ev: &AppEvent<E>) { let _ = ev; }
    fn end_event(&mut self) { }


    fn begin_custom(&mut self, custom: &E) { let _ = custom; }
    fn end_custom(&mut self) {}

    fn begin_flow(&mut self, flow: &FlowMessage) 
    {
        match flow
        {
            FlowMessage::Resumed => self.begin_resumed(),
            FlowMessage::Paused => self.begin_paused(),
            FlowMessage::Update => self.begin_update(),
            FlowMessage::Draw => self.begin_draw(),
            FlowMessage::Exit => self.begin_exit(),
        }
    }
    fn end_flow(&mut self, flow: &FlowMessage) 
    {
        match flow
        {
            FlowMessage::Resumed => self.end_resumed(),
            FlowMessage::Paused => self.end_paused(),
            FlowMessage::Update => self.end_update(),
            FlowMessage::Draw => self.end_draw(),
            FlowMessage::Exit => self.end_exit(),
        }
    }

    fn begin_paused(&mut self) {}
    fn end_paused(&mut self) {}

    fn begin_resumed(&mut self) {}
    fn end_resumed(&mut self) {}

    fn begin_update(&mut self) {}
    fn end_update(&mut self) {}

    fn begin_draw(&mut self) {}
    fn end_draw(&mut self) {}

    fn begin_exit(&mut self) {}
    fn end_exit(&mut self) {}

    fn begin_input(&mut self, input: &InputEvent) 
    { 
        match input
        {
            InputEvent::Key(key) => self.begin_input_key(key),
        }
    }
    fn end_input(&mut self, input: &InputEvent) 
    {
        match input
        {
            InputEvent::Key(key) => self.end_input_key(key),
        }
    }
    fn begin_input_key(&mut self, key: &KeyEvent) { let _ = key; }
    fn end_input_key(&mut self, key: &KeyEvent) { let _ = key; }
}