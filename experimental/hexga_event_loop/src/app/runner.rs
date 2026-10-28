use super::*;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[non_exhaustive]
pub struct EventLoopParam
{
    /// Controls the blocking behavior of the event loop
    pub control_flow: AppControlFlow,
    /// Translated unhandled KeyEvent into KeyboardEvent during event processing
    pub shortcut: AppShortcut,
}

pub trait WithAppParam: WithAppShortcut
{
    fn control_flow(&self) -> AppControlFlow;
    fn with_control_flow(self, control_flow: AppControlFlow) -> Self;
}

impl WithAppShortcut for EventLoopParam
{
    fn exit_shortcut(&self) -> Option<KeyShortcut> { self.shortcut.exit.clone() }

    fn with_exit_shortcut(mut self, exit: Option<KeyShortcut>) -> Self
    {
        self.shortcut.exit = exit;
        self
    }

    fn copy_shortcut(&self) -> Option<KeyShortcut> { self.shortcut.copy.clone() }

    fn with_copy_shortcut(mut self, copy: Option<KeyShortcut>) -> Self
    {
        self.shortcut.copy = copy;
        self
    }

    fn paste_shortcut(&self) -> Option<KeyShortcut> { self.shortcut.paste.clone() }

    fn with_paste_shortcut(mut self, paste: Option<KeyShortcut>) -> Self
    {
        self.shortcut.paste = paste;
        self
    }

    fn cut_shortcut(&self) -> Option<KeyShortcut> { self.shortcut.cut.clone() }

    fn with_cut_shortcut(mut self, cut: Option<KeyShortcut>) -> Self
    {
        self.shortcut.cut = cut;
        self
    }
}

impl WithAppParam for EventLoopParam
{
    fn control_flow(&self) -> AppControlFlow { self.control_flow }

    fn with_control_flow(mut self, control_flow: AppControlFlow) -> Self
    {
        self.control_flow = control_flow;
        self
    }
}

impl HasMut<AppControlFlow> for EventLoopParam
{
    fn retrive_mut(&mut self) -> &mut AppControlFlow { &mut self.control_flow }
}

impl HasMut<AppShortcut> for EventLoopParam
{
    fn retrive_mut(&mut self) -> &mut AppShortcut { &mut self.shortcut }
}

impl Default for EventLoopParam
{
    fn default() -> Self
    {
        Self {
            control_flow: Default::default(),
            shortcut: Default::default(),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AppShortcut
{
    pub exit: Option<KeyShortcut>,
    pub copy: Option<KeyShortcut>,
    pub paste: Option<KeyShortcut>,
    pub cut: Option<KeyShortcut>,
    // undo, redo
}

impl WithAppShortcut for AppShortcut
{
    fn exit_shortcut(&self) -> Option<KeyShortcut> { self.exit }

    fn with_exit_shortcut(mut self, exit: Option<KeyShortcut>) -> Self
    {
        self.exit = exit;
        self
    }

    fn copy_shortcut(&self) -> Option<KeyShortcut> { self.copy }

    fn with_copy_shortcut(mut self, copy: Option<KeyShortcut>) -> Self
    {
        self.copy = copy;
        self
    }

    fn paste_shortcut(&self) -> Option<KeyShortcut> { self.paste }

    fn with_paste_shortcut(mut self, paste: Option<KeyShortcut>) -> Self
    {
        self.paste = paste;
        self
    }

    fn cut_shortcut(&self) -> Option<KeyShortcut> { self.cut }

    fn with_cut_shortcut(mut self, cut: Option<KeyShortcut>) -> Self
    {
        self.cut = cut;
        self
    }
}

pub trait WithAppShortcut
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

impl Default for AppShortcut
{
    fn default() -> Self
    {
        Self {
            exit: Some(KeyShortcut::EXIT),
            copy: Some(KeyShortcut::COPY),
            paste: Some(KeyShortcut::PASTE),
            cut: Some(KeyShortcut::CUT),
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum AppControlFlow
{
    /// When the current loop iteration finishes, immediately begin a new iteration regardless of
    /// whether or not new events are available to process.
    #[default]
    Poll,

    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
}
impl From<AppControlFlow> for ::winit::event_loop::ControlFlow
{
    fn from(value: AppControlFlow) -> Self
    {
        match value
        {
            AppControlFlow::Poll => ::winit::event_loop::ControlFlow::Poll,
            AppControlFlow::Wait => ::winit::event_loop::ControlFlow::Wait,
        }
    }
}

/// Run the app from an OS pov, but the user need to manually initialize all the other system like the window, the rendering...
pub trait AppRunRaw<CustomEvent = ()> : for<'a> App<CustomEvent, EventLoop<'a, CustomEvent>> + Sized + 'static
    where CustomEvent: IAppEvent,
{
    fn run_raw<F>(init : F) -> AppResult
        where F: FnOnce(AppProxy<CustomEvent>) -> Self { Self::run_raw_with_param(init, ___()) }
    
    fn run_raw_with_param<F>(init : F, param: EventLoopParam) -> AppResult
        where F: FnOnce(AppProxy<CustomEvent>) -> Self
    {
        crate::app::init_if_needed();

        let event_loop = WinitEventLoop::with_user_event().build().ok_or_void()?;
        event_loop.set_control_flow(param.control_flow.into());
        let proxy = AppProxy::new(event_loop.create_proxy());

        let mut runner = AppRunner {
            event_handler: init(proxy.clone()),
            state: AppState {
                dt: zero(),
                time: Time::since_launch(),
                clipboard: ___(),
                key_modifiers: ___(),
            },
            proxy,
            param,
        };

        #[cfg(not(target_arch = "wasm32"))]
        {
            event_loop.run_app(&mut runner).ok_or_void()?;
        }
        #[cfg(target_arch = "wasm32")]
        {
            use winit::platform::web::EventLoopExtWebSys;

            event_loop.spawn_app(runner);
        }

        Ok(())
    }
}

impl<S,CustomEvent> AppRunRaw<CustomEvent> for S where S : for<'a> App<CustomEvent, EventLoop<'a,CustomEvent>> + Sized + 'static,
CustomEvent: IAppEvent 
{}

struct AppRunner<A, CustomEvent>
where
    A: for<'a> App<CustomEvent, EventLoop<'a, CustomEvent>>,
    CustomEvent: IAppEvent,
{
    event_handler: A,
    state: AppState,
    proxy: AppProxy<CustomEvent>,
    param: EventLoopParam,
}

impl<A, CustomEvent> AppSendEvent<AppEvent> for AppRunner<A, CustomEvent>
where
    A: for<'a> App<CustomEvent, EventLoop<'a, CustomEvent>>,
    CustomEvent: IAppEvent,
{
    fn send_event(&self, ev: AppEvent) -> ProxyResult { self.proxy.send_event(ev.into()) }
}


impl<EventHandler, CustomEvent> AppRunner<EventHandler, CustomEvent>
where
    EventHandler: for<'a> App<CustomEvent, EventLoop<'a,CustomEvent>>,
    CustomEvent: IAppEvent,
{
    fn event(&mut self, active: &WinitEventLoopActive, ev: impl Into<AppMessage<CustomEvent>>)
    {
        let mut ev = ev.into();

        match &mut ev
        {
            AppMessage::Event(platform_event) => match platform_event
            {
                AppEvent::KeyModifiers(modifier) => { self.state.key_modifiers = *modifier; }
                AppEvent::Key(k) =>
                {
                    let down = k.is_down();
                    match k.code
                    {
                        KeyCode::ShiftLeft =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::ShiftLeft, down);
                        }
                        KeyCode::ShiftRight =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::ShiftRight, down);
                        }

                        KeyCode::ControlLeft =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::ControlLeft, down);
                        }
                        KeyCode::ControlRight =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::ControlRight, down);
                        }

                        KeyCode::AltLeft =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::AltLeft, down);
                        }
                        KeyCode::AltRight =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::AltRight, down);
                        }

                        KeyCode::SuperLeft =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::SuperLeft, down);
                        }
                        KeyCode::SuperRight =>
                        {
                            self.state.key_modifiers.set(KeyModifiers::SuperRight, down);
                        }
                        _ =>
                        {}
                    }
                    k.modifiers = self.state.key_modifiers;
                }
                AppEvent::WindowClose =>
                {
                    active.exit();
                }
                _ =>
                {}
            },
            _ =>
            {}
        }

        let Some(ev) = self.app(active, |event_handler, event_loop| event_handler.message(ev, event_loop))
        else
        {
            return;
        };

        match ev
        {
            AppMessage::Event(platform_event) => match platform_event
            {
                AppEvent::Key(k) =>
                {
                    if k.is_down() && k.is_not_repeated()
                    {
                        if self.param.shortcut.exit.matches(&k)
                        {
                            let _ = self.send_event(AppEvent::WindowClose);
                        }
                        if self.param.shortcut.copy.matches(&k)
                        {
                            let _ = self.send_event(AppEvent::ClipboardCopy);
                        }
                        if self.param.shortcut.paste.matches(&k)
                        {
                            match self.state.get_clipboard()
                            {
                                Some(txt) =>
                                {
                                    let _ = self.send_event(AppEvent::ClipboardPaste(txt));
                                }
                                None =>
                                {}
                            }
                        }
                        if self.param.shortcut.cut.matches(&k)
                        {
                            let _ = self.send_event(AppEvent::ClipboardCut);
                        }
                    }
                }
                AppEvent::ClipboardPaste(txt) =>
                {
                    let key_event = KeyEvent {
                        code: KeyCode::Unknow(KeyCodeNative::Unknow),
                        state: KeyState::Down,
                        modifiers: self.state.key_modifiers,
                        repeat: ButtonRepeat::NotRepeated,
                        key: Key::Text(txt.clone().into()),
                        location: KeyLocation::Unknow,
                        text: Some(txt.into()),
                    };
                    let _ = self.send_event(AppEvent::Key(key_event));
                }
                _ =>
                {}
            },
            _ =>
            {}
        }
    }

    fn app<F, O>(&mut self, active: &WinitEventLoopActive, f: F) -> O
    where
        F: FnOnce(&mut EventHandler, &mut EventLoop<CustomEvent>) -> O,
    {
        let mut event_loop = EventLoop::new(active, &mut self.state, &self.proxy);
        f(&mut self.event_handler, &mut event_loop)
    }
}
impl<EventHandler, CustomEvent> ::winit::application::ApplicationHandler<AppMessage<CustomEvent>> for AppRunner<EventHandler, CustomEvent>
where
    EventHandler: for<'a> App<CustomEvent, EventLoop<'a,CustomEvent>>,
    CustomEvent: IAppEvent,
{
    fn resumed(&mut self, active: &WinitEventLoopActive)
    {
        self.app(active, |event_handler, event_loop| {
            event_handler.message(AppMessage::Flow(AppFlow::Resume), event_loop)
        });
    }

    fn suspended(&mut self, active: &WinitEventLoopActive)
    {
        self.app(active, |event_handler, event_loop| {
            event_handler.message(AppMessage::Flow(AppFlow::Pause), event_loop)
        });
    }

    fn exiting(&mut self, active: &WinitEventLoopActive)
    {
        self.app(active, |event_handler, event_loop| {
            event_handler.message(AppMessage::Flow(AppFlow::Exit), event_loop)
        });
    }

    fn about_to_wait(&mut self, active: &WinitEventLoopActive)
    {
        let time = Time::since_launch();
        let dt = time - self.state.time;
        self.state.time = time;

        self.app(active, |event_handler, event_loop| {
            event_handler.message(AppMessage::Flow(AppFlow::Update(dt)), event_loop)
        });
    }

    fn user_event(&mut self, active: &WinitEventLoopActive, event: AppMessage<CustomEvent>) { self.event(active, event); }


    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: WinitEvent::DeviceId,
        event: WinitEvent::DeviceEvent,
    )
    {
        
    }

    fn window_event(&mut self, active: &WinitEventLoopActive, window_id: winit::window::WindowId, event: WinitWindowEvent)
    {
        match event
        {
            WinitWindowEvent::Moved(pos) =>
            {
                self.event(active, AppEvent::WindowMove(pos.convert()));
            }
            WinitWindowEvent::Resized(size) =>
            {
                self.event(active, AppEvent::WindowResize(size.convert()));
            }
            WinitWindowEvent::CloseRequested =>
            {
                self.event(active, AppEvent::WindowClose);
            }
            WinitWindowEvent::Destroyed =>
            {
                self.event(active, AppEvent::WindowDestroy);
            }
            WinitWindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } =>
            {
                self.event(active, AppEvent::Key(event.into()));
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
            WinitWindowEvent::RedrawRequested =>
            {
                self.app(active, |event_handler, event_loop| event_handler.draw(event_loop));
            }
            WinitWindowEvent::ActivationTokenDone { serial, token } => {},
            WinitWindowEvent::Resized(physical_size) => {},
            WinitWindowEvent::Moved(physical_position) => {},
            WinitWindowEvent::CloseRequested => {},
            WinitWindowEvent::Destroyed => {},
            WinitWindowEvent::DroppedFile(path_buf) => { self.event(active, AppEvent::FileDrop(path_buf)); },
            WinitWindowEvent::HoveredFile(path_buf) => { self.event(active, AppEvent::FileHover(path_buf)); },
            WinitWindowEvent::HoveredFileCancelled => { self.event(active, AppEvent::FileHoveredCancel); },
            WinitWindowEvent::Focused(focus) => { self.event(active, AppEvent::WindowFocus(focus)); },
            //WinitWindowEvent::ModifiersChanged(modifiers) => { self.event(active, AppEvent::KeyModifiers(modifiers.into())); },
            WinitWindowEvent::Ime(ime) => {},
            WinitWindowEvent::CursorMoved { device_id, position } => { self.event(active, AppEvent::MouseMove(position.convert())); },
            WinitWindowEvent::CursorEntered { device_id } => { self.event(active, AppEvent::MouseFocus(true)); },
            WinitWindowEvent::CursorLeft { device_id } => { self.event(active, AppEvent::MouseFocus(false)); },
            WinitWindowEvent::MouseWheel { device_id, delta, phase } =>  { 
                self.event(active, AppEvent::MouseWheel(MouseWheel { delta: delta.convert(), phase: phase.convert() })); 
            },
            WinitWindowEvent::MouseInput { device_id, state, button } => 
            {
                self.event(active, AppEvent::MouseClick(MouseClick { button: button.convert(), state: state.convert() })); 
            },
            // macOS and iOS specific
            WinitWindowEvent::PinchGesture { device_id, delta, phase } => {},
            WinitWindowEvent::PanGesture { device_id, delta, phase } => {},
            WinitWindowEvent::DoubleTapGesture { device_id } => {},
            WinitWindowEvent::RotationGesture { device_id, delta, phase } => {},
            WinitWindowEvent::Occluded(_) => {},

            WinitWindowEvent::TouchpadPressure { device_id, pressure, stage } => {},
            WinitWindowEvent::AxisMotion { device_id, axis, value } => {},
            WinitWindowEvent::Touch(touch) => { self.event(active, AppEvent::Touch(touch.convert())); },
            WinitWindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            WinitWindowEvent::ThemeChanged(theme) => { self.event(active, AppEvent::Theme(theme.convert())); },
            WinitWindowEvent::RedrawRequested => { self.event(active, AppFlow::Draw); },
            _ => (),
        }
    }
}
