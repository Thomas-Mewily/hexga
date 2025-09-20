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
    pub event_loop: &'a ActiveEventLoop,
    pub proxy: &'a EventLoopProxy<E>
}

impl<'a,E> Clone for MessageCtx<'a, E> where E:IEvent
{
    fn clone(&self) -> Self 
    {
        Self { event_loop: self.event_loop, proxy: self.proxy }
    }
}
impl<'a,E> Copy for MessageCtx<'a, E> where E:IEvent {}

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

    fn scoped_custom<F,R>(&mut self, custom: E, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce(E) -> R
    {
        self.begin_custom(&custom, ctx);
        let r = f(custom);
        self.end_custom(ctx);
        r
    }
    fn begin_custom(&mut self, custom: &E, ctx: MessageCtx<'_,E>) { let _  = (custom, ctx); }
    fn end_custom(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }




    fn scoped_window<F,R>(&mut self, window: WindowEvent, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce(WindowEvent) -> R
    {
        self.begin_window(&window, ctx);
        let r = f(window);
        self.end_window(ctx);
        r
    }
    fn begin_window(&mut self, window: &WindowEvent, ctx: MessageCtx<'_,E>) { let _ = (window, ctx);}
    fn end_window(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx;}


    fn scoped_input<F,R>(&mut self, input: InputEvent, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce(InputEvent) -> R
    {
        self.begin_input(&input, ctx);
        let r = f(input);
        self.end_input(ctx);
        r
    }
    fn begin_input(&mut self, input: &InputEvent, ctx: MessageCtx<'_,E>) 
    {
        match input
        {
            InputEvent::Key(key) => self.begin_input_key(key, ctx),
        }
    }
    fn end_input(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
    

    fn begin_input_key(&mut self, input_key: &KeyEvent, ctx: MessageCtx<'_,E>) { let _ = (input_key, ctx); }



    fn scoped_flow<F,R>(&mut self, flow: FlowMessage, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce(FlowMessage) -> R
    {
        self.begin_flow(flow, ctx);
        let r = f(flow);
        self.end_flow(flow, ctx);
        r
    }
    fn begin_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) { self.dispatch_begin_flow(flow, ctx) }
    fn dispatch_begin_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) 
    {
        match flow
        {
            FlowMessage::Resumed => self.begin_flow_resumed(ctx),
            FlowMessage::Paused => self.begin_flow_paused(ctx),
            FlowMessage::Update => self.begin_flow_update(ctx),
            FlowMessage::Draw => self.begin_flow_draw(ctx),
        }
    }
    fn end_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) { self.dispatch_end_flow(flow, ctx) }
    fn dispatch_end_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) 
    {
        match flow
        {
            FlowMessage::Resumed => self.end_flow_resumed(ctx),
            FlowMessage::Paused => self.end_flow_paused(ctx),
            FlowMessage::Update => self.end_flow_update(ctx),
            FlowMessage::Draw => self.end_flow_draw(ctx),
        }
    }


    fn scoped_flow_paused<F,R>(&mut self, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce() -> R
    {
        self.begin_flow_paused(ctx);
        let r = f();
        self.end_flow_paused(ctx);
        r
    }
    fn begin_flow_paused(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
    fn end_flow_paused(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }


    fn scoped_flow_resumed<F,R>(&mut self, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce() -> R
    {
        self.begin_flow_resumed(ctx);
        let r = f();
        self.end_flow_resumed(ctx);
        r
    }
    fn begin_flow_resumed(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
    fn end_flow_resumed(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }


    fn scoped_flow_update<F,R>(&mut self, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce() -> R
    {
        self.begin_flow_update(ctx);
        let r = f();
        self.end_flow_update(ctx);
        r
    }
    fn begin_flow_update(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
    fn end_flow_update(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }


    fn scoped_flow_draw<F,R>(&mut self, f: F, ctx: MessageCtx<'_,E>) -> R where F: FnOnce() -> R
    {
        self.begin_flow_draw(ctx);
        let r = f();
        self.end_flow_draw(ctx);
        r
    }
    fn begin_flow_draw(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
    fn end_flow_draw(&mut self, ctx: MessageCtx<'_,E>) { let _ = ctx; }
}
