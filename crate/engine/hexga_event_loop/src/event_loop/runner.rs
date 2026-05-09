use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventLoopParam
{
    pub control_flow : EventLoopControlFlow,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum EventLoopControlFlow 
{
    /// When the current loop iteration finishes, immediately begin a new iteration regardless of
    /// whether or not new events are available to process.
    #[default]
    Poll,

    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
}
impl From<EventLoopControlFlow> for ::winit::event_loop::ControlFlow
{
    fn from(value: EventLoopControlFlow) -> Self {
        match value
        {
            EventLoopControlFlow::Poll =>::winit::event_loop::ControlFlow::Poll,
            EventLoopControlFlow::Wait =>::winit::event_loop::ControlFlow::Wait,
        }
    }
}

struct EventLoopRunner<EventHandler,CustomEvent> 
    where 
    EventHandler: PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    event_handler : EventHandler,
    state : EventLoopState,
    proxy: EventLoopProxy<CustomEvent>,
}

    
pub fn run<EventHandler, CustomEvent>(event_handler: EventHandler, param: EventLoopParam) -> EventLoopResult
    where 
    EventHandler : PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    free_fn::init();

    let event_loop = WinitEventLoop::with_user_event().build().map_err(|_|())?;
    event_loop.set_control_flow(param.control_flow.into());
    let proxy = EventLoopProxy::new(event_loop.create_proxy());

    let mut runner = EventLoopRunner
    {
        event_handler,
        state: EventLoopState { dt: zero(), time: Time::since_launch() },
        proxy,
    };

    // Todo handle wasm32
    event_loop.run_app(&mut runner).map_err(|_|());

    Ok(())
}

impl<EventHandler, CustomEvent> EventLoopRunner<EventHandler,CustomEvent> 
    where 
    EventHandler: PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    fn app<F,O>(&mut self, active: &WinitEventLoopActive, f: F) -> O 
        where
        F: FnOnce(&mut EventHandler, &mut EventLoop<CustomEvent>) -> O
    {
        let mut event_loop = EventLoop::new(active, &mut self.state, &self.proxy);
        f(&mut self.event_handler, &mut event_loop)
    }
}
impl<EventHandler, CustomEvent> ::winit::application::ApplicationHandler<PlatformEvent<CustomEvent>> for EventLoopRunner<EventHandler,CustomEvent> 
    where 
    EventHandler: PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    fn resumed(&mut self, active: &WinitEventLoopActive) {
        self.app(active, |event_handler,event_loop| event_handler.resumed(event_loop));
    }

    fn suspended(&mut self, active: &winit::event_loop::ActiveEventLoop) {
        self.app(active, |event_handler,event_loop| event_handler.paused(event_loop));
    }

    fn user_event(&mut self, active: &winit::event_loop::ActiveEventLoop, event: PlatformEvent<CustomEvent>) {
        self.app(active, |event_handler,event_loop| event_handler.event(event, event_loop));
    }

    fn window_event(
        &mut self,
        event_loop: &WinitEventLoopActive,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}