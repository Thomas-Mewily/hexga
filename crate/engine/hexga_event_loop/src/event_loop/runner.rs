use super::*;

#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct EventLoopParam
{
    pub control_flow : EventLoopControlFlow,
    pub shortcut: DefaultShortcut,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DefaultShortcut
{
    /*
    pub close : Option<KeyAction>,
    pub copy  : Option<KeyActionMods>,
    pub paste : Option<KeyActionMods>,
    pub cut   : Option<KeyActionMods>,
    */
}
impl Default for DefaultShortcut
{
    fn default() -> Self {
        Self{}
    }
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
    param: EventLoopParam,
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
        state: EventLoopState { dt: zero(), time: Time::since_launch(), clipboard: ___(), key_modifiers: ___() },
        proxy,
        param,
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
    fn event(&mut self, active: &WinitEventLoopActive, ev: impl Into<PlatformEvent<CustomEvent>>)
    {
        let mut ev = ev.into();

        match &mut ev
        {
            PlatformEvent::Key(k) => 
            {
                let down = k.is_down();
                match k.code
                {
                    KeyCode::ShiftLeft => { self.state.key_modifiers.set(KeyModifiers::ShiftLeft, down); },
                    KeyCode::ShiftRight => { self.state.key_modifiers.set(KeyModifiers::ShiftRight, down); },

                    KeyCode::ControlLeft => { self.state.key_modifiers.set(KeyModifiers::ControlLeft, down); },
                    KeyCode::ControlRight => { self.state.key_modifiers.set(KeyModifiers::ControlRight, down); },

                    KeyCode::AltLeft => { self.state.key_modifiers.set(KeyModifiers::AltLeft, down); },
                    KeyCode::AltRight => { self.state.key_modifiers.set(KeyModifiers::AltRight, down); },

                    KeyCode::SuperLeft => { self.state.key_modifiers.set(KeyModifiers::SuperLeft, down); },
                    KeyCode::SuperRight => { self.state.key_modifiers.set(KeyModifiers::SuperRight, down); },
                    _ => {},
                }
                k.modifiers = self.state.key_modifiers;
            },
            PlatformEvent::Close => { active.exit(); }
            _ => {}
        }

        self.app(active, |event_handler,event_loop| event_handler.event(ev, event_loop));
    }


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

    fn suspended(&mut self, active: &WinitEventLoopActive) {
        self.app(active, |event_handler,event_loop| event_handler.paused(event_loop));
    }

    fn exiting(&mut self, active: &WinitEventLoopActive) {
        self.app(active, |event_handler,event_loop| event_handler.exit(event_loop));
    }

    fn about_to_wait(&mut self, active: &WinitEventLoopActive) 
    {
        let time = Time::since_launch();
        let dt = time - self.state.time;
        self.state.time = time;

        self.app(active, |event_handler,event_loop| event_handler.update(dt, event_loop));
    }

    fn user_event(&mut self, active: &WinitEventLoopActive, event: PlatformEvent<CustomEvent>) 
    {
        self.event(active, event);
    }

    fn window_event(
        &mut self,
        active: &WinitEventLoopActive,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event
        {
            WinitWindowEvent::Resized(physical_size) =>
            {
                self.event(active, PlatformEvent::Resize(physical_size.convert()));
            }
            winit::event::WindowEvent::CloseRequested =>
            {
                self.event(active, PlatformEvent::Close);
            }
            winit::event::WindowEvent::Destroyed =>
            {
                self.event(active, PlatformEvent::Destroy);
            }
            WinitWindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } =>
            {
                self.event(active, PlatformEvent::Key(event.into()));
            }
            /*
            // TODO: interesting event to handle:
            winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            winit::event::WindowEvent::Focused(_) => todo!(),
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            winit::event::WindowEvent::ThemeChanged(theme) => todo!(),
            winit::event::WindowEvent::Occluded(_) => todo!()
            */
            winit::event::WindowEvent::RedrawRequested =>
            {
                self.app(active, |event_handler,event_loop| event_handler.draw(event_loop));
            }
            _ => (),
        }
    }
}