use super::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct EventLoopParam
{
    /// Controls the blocking behavior of the event loop
    pub control_flow : EventLoopControlFlow,
    /// Translated unhandled KeyEvent into KeyboardEvent during event processing
    pub shortcut: EventLoopShortcut,
}

pub trait WithEventLoopParam : WithEventLoopShortcut
{
    fn control_flow(&self) -> EventLoopControlFlow;
    fn with_control_flow(self, control_flow : EventLoopControlFlow) -> Self;
}

impl WithEventLoopShortcut for EventLoopParam
{
    fn exit_shortcut(&self) -> Option<KeyShortcut> {
        self.shortcut.exit.clone()
    }

    fn with_exit_shortcut(mut self, exit: Option<KeyShortcut>) -> Self {
        self.shortcut.exit = exit;
        self
    }

    fn copy_shortcut(&self) -> Option<KeyShortcut> {
        self.shortcut.copy.clone()
    }

    fn with_copy_shortcut(mut self, copy: Option<KeyShortcut>) -> Self {
        self.shortcut.copy = copy;
        self
    }

    fn paste_shortcut(&self) -> Option<KeyShortcut> {
        self.shortcut.paste.clone()
    }

    fn with_paste_shortcut(mut self, paste: Option<KeyShortcut>) -> Self {
        self.shortcut.paste = paste;
        self
    }

    fn cut_shortcut(&self) -> Option<KeyShortcut> {
        self.shortcut.cut.clone()
    }

    fn with_cut_shortcut(mut self, cut: Option<KeyShortcut>) -> Self {
        self.shortcut.cut = cut;
        self
    }
}

impl WithEventLoopParam for EventLoopParam
{
    fn control_flow(&self) -> EventLoopControlFlow {
        self.control_flow
    }

    fn with_control_flow(mut self, control_flow : EventLoopControlFlow) -> Self {
        self.control_flow = control_flow; self
    }
}

impl HasMut<EventLoopControlFlow> for EventLoopParam
{
    fn retrive_mut(&mut self) -> &mut EventLoopControlFlow {
        &mut self.control_flow
    }
}

impl HasMut<EventLoopShortcut> for EventLoopParam
{
    fn retrive_mut(&mut self) -> &mut EventLoopShortcut {
        &mut self.shortcut
    }
}

impl Default for EventLoopParam
{
    fn default() -> Self 
    {
        Self { control_flow: Default::default(), shortcut: Default::default() }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventLoopShortcut
{
    pub exit  : Option<KeyShortcut>,
    pub copy  : Option<KeyShortcut>,
    pub paste : Option<KeyShortcut>,
    pub cut   : Option<KeyShortcut>,
}

impl WithEventLoopShortcut for EventLoopShortcut
{
    fn exit_shortcut(&self) -> Option<KeyShortcut> {
        self.exit
    }

    fn with_exit_shortcut(mut self, exit: Option<KeyShortcut>) -> Self {
        self.exit = exit;
        self
    }

    fn copy_shortcut(&self) -> Option<KeyShortcut> {
        self.copy
    }

    fn with_copy_shortcut(mut self, copy: Option<KeyShortcut>) -> Self {
        self.copy = copy;
        self
    }

    fn paste_shortcut(&self) -> Option<KeyShortcut> {
        self.paste
    }

    fn with_paste_shortcut(mut self, paste: Option<KeyShortcut>) -> Self {
        self.paste = paste;
        self
    }

    fn cut_shortcut(&self) -> Option<KeyShortcut> {
        self.cut
    }

    fn with_cut_shortcut(mut self, cut: Option<KeyShortcut>) -> Self {
        self.cut = cut;
        self
    }
}

pub trait WithEventLoopShortcut
{
    fn exit_shortcut(&self) -> Option<KeyShortcut>;
    fn with_exit_shortcut(self, exit: Option<KeyShortcut>) -> Self;

    fn copy_shortcut(&self) -> Option<KeyShortcut>;
    fn with_copy_shortcut(self, copy: Option<KeyShortcut>) -> Self;

    fn paste_shortcut(&self) -> Option<KeyShortcut>;
    fn with_paste_shortcut(self, paste: Option<KeyShortcut>) -> Self;

    fn cut_shortcut(&self) -> Option<KeyShortcut>;
    fn with_cut_shortcut(self, cut: Option<KeyShortcut>) -> Self;
}

impl Default for EventLoopShortcut
{
    fn default() -> Self 
    {
        Self
        {
            exit: Some(KeyShortcut::EXIT),
            copy: Some(KeyShortcut::COPY),
            paste: Some(KeyShortcut::PASTE),
            cut: Some(KeyShortcut::CUT),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

impl<EventHandler, CustomEvent> EventLoopSendEvent<PlatformEvent<CustomEvent>> for EventLoopRunner<EventHandler,CustomEvent> 
    where 
    EventHandler: PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    fn send_event(&self, ev: PlatformEvent<CustomEvent>) -> ProxyResult {
        self.proxy.send_event(ev)
    }
}

pub fn run_with_param<F,EventHandler,CustomEvent>(init: F, param: EventLoopParam) -> EventLoopResult
    where 
    F: FnOnce(EventLoopProxy<CustomEvent>) -> EventHandler,
    EventHandler : PlatformEventHandler<CustomEvent>,
    CustomEvent: PlatformCustomEvent
{
    free_fn::init();

    let event_loop = WinitEventLoop::with_user_event().build().map_err(|_|())?;
    event_loop.set_control_flow(param.control_flow.into());
    let proxy = EventLoopProxy::new(event_loop.create_proxy());

    let mut runner = EventLoopRunner
    {
        event_handler : init(proxy.clone()),
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

        let Some(ev) = self.app(active, |event_handler,event_loop| event_handler.event(ev, event_loop)) else { return; };

        match ev
        {
            PlatformEvent::Key(k) => 
            {
                if self.param.shortcut.exit.matches(&k)
                {
                    let _ = self.send_event(PlatformEvent::Close);
                }
                if self.param.shortcut.copy.matches(&k)
                {
                    let _ = self.send_event(PlatformEvent::Copy);
                }
                if self.param.shortcut.paste.matches(&k)
                {
                    match self.state.get_clipboard()
                    {
                        Some(txt) =>  { let _ = self.send_event(PlatformEvent::Paste(txt)); },
                        None => {},
                    }
                }
                if self.param.shortcut.cut.matches(&k)
                {
                    let _ = self.send_event(PlatformEvent::Cut);
                }
            }
            PlatformEvent::Paste(txt) => 
            {
                let key_event = KeyEvent 
                { 
                    code: KeyCode::Unknow(KeyCodeNative::Unknow), 
                    state: KeyState::Down, 
                    modifiers: self.state.key_modifiers, 
                    repeat: ButtonRepeat::NotRepeated, 
                    key: Key::Text(txt.clone().into()), 
                    location: KeyLocation::Unknow, 
                    text : Some(txt.into()),
                };
                let _ = self.send_event(PlatformEvent::Key(key_event));
            }
            _ => {}
        };
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