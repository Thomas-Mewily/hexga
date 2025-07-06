use copypasta::{ClipboardContext, ClipboardProvider};
use serde::de;

use super::*;



pub trait WindowLoop<T=()>
{
    /// Handles a message from the application. This is the main entry for handling messages, and events.
    ///
    /// This is also responsible for dispatching special events like the [AppLoop::update], [AppLoop::draw], [AppLoop::pause], [AppLoop::resume].
    fn handle_message(&mut self, message: EventMessage<T>, ctx: &mut WindowCtx) -> bool
    {
        match message
        {
            EventMessage::LocalizedEvent(localized_event) =>
                    {
                        if let Event::Window(WindowEvent::Draw) = localized_event.event
                        {
                            self.draw_window(localized_event.window, ctx);
                        }
                        else
                        {
                            return self.handle_localized_event(localized_event, ctx)
                        }
                    },
            EventMessage::Device(device_message) => match device_message
                    {
                        DeviceMessage::Added   => self.device_added(ctx),
                        DeviceMessage::Removed => self.device_removed(ctx),
                        DeviceMessage::Resume  => self.resume(ctx),
                        DeviceMessage::Update  => self.update(ctx),
                        DeviceMessage::Exit    => { self.exit(ctx); ctx.exit(); },
                        DeviceMessage::MemoryWarning => self.warning_memory(ctx),
                    },
            EventMessage::User(user) => self.user_event(user, ctx),
        }
        true
    }

    fn handle_localized_event(&mut self, event: LocalizedEvent, ctx: &mut WindowCtx) -> bool
    {
        self.handle_event(event.event, ctx)
    }

    fn handle_event(&mut self, event : Event, ctx: &mut WindowCtx) -> bool
    {
        let _ = event;
        let _ = ctx;
        false
    }

    fn resume(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }
    fn pause(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }

    fn update(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }
    fn draw(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }
    fn draw_window(&mut self, window : WindowID, ctx: &mut WindowCtx) { let _ = window; self.draw(ctx); }

    // Called when on exit
    fn exit(&mut self, ctx: &mut WindowCtx) { ctx.exit(); }

    fn device_added(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }
    fn device_removed(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }
    fn warning_memory(&mut self, ctx: &mut WindowCtx) { let _ = ctx; }

    fn user_event(&mut self, event: T, ctx: &mut WindowCtx) { let _ = (event, ctx); }
}

#[derive(Clone, PartialEq, Debug)]
pub struct WindowRunParam
{
    pub default_window : Option<WindowParam>,
    // If true update will be called on every event, otherwise it will be called frequently
    pub wait_for_event : bool,
}

impl Default for WindowRunParam
{
    fn default() -> Self { Self { default_window: Some(___()), wait_for_event: false } }
}

pub trait IWindowRunParam : Sized
{
    fn wait_for_event(&self) -> bool;
    fn with_wait_for_event(self, wait_for_event : bool) -> Self;

    fn default_window(&self) -> Option<&WindowParam>;
    fn with_default_window(self, default_window : Option<WindowParam>) -> Self;

    // A default configuration for game.
    fn game() -> Self where Self: Default { Self::___().with_wait_for_event(false) }
    // A default configuration for software.
    fn software() -> Self where Self: Default { Self::___().with_wait_for_event(true) }
}

impl WindowRunParam
{
    pub fn new() -> Self { ___() }
}
impl IWindowRunParam for WindowRunParam
{
    fn wait_for_event(&self) -> bool { self.wait_for_event }
    fn with_wait_for_event(self, wait_for_event : bool) -> Self { Self { wait_for_event, ..self } }

    fn default_window(&self) -> Option<&WindowParam> { self.default_window.as_ref() }
    fn with_default_window(self, default_window : Option<WindowParam>) -> Self { Self { default_window, ..self } }
}

pub trait WindowRun<T> : WindowLoop<T> where T: 'static
{
    fn run_with_param(&mut self, param : WindowRunParam) -> AppResult where Self: Sized
    {
        let event_loop = EventLoop::<T>::with_user_event().build().map_err(|e| <AppErrorEventLoop as Into<AppError>>::into(e))?;
        event_loop.set_control_flow(if param.wait_for_event { winit::event_loop::ControlFlow::Wait } else { winit::event_loop::ControlFlow::Poll });
        let proxy = event_loop.create_proxy();

        let mut runner = WindowRunner
        {
            app : self,
            ctx: ___(),
            proxy,
        };

        runner.ctx.default_window = param.default_window;
        event_loop.run_app(&mut runner).map_err(|e| e.into())
    }

    fn run(&mut self) -> AppResult where Self: Sized { self.run_with_param(___()) }
}
impl<S,T> WindowRun<T> for S where S: WindowLoop<T>, T: 'static {}



struct WindowRunner<'a, A : ?Sized, T> where A : WindowLoop<T>, T: 'static
{
    app : &'a mut A,
    ctx : WindowContext,
    proxy : EventLoopProxy<T>,
}

impl<'a,A : ?Sized, T> WindowRunner<'a,A,T> where A : WindowLoop<T>, T: 'static
{
    fn handle_message(&mut self, message: impl Into<EventMessage<T>>, event_loop: &ActiveEventLoop) -> bool
    {
        let Self { app, ctx, proxy: _phantom } = self;
        let mut app_ctx = AppCtx { ctx, active_event_loop: event_loop };
        app.handle_message(message.into(), &mut app_ctx)
    }
}

pub type WindowCtx<'a> = dyn IWindowCtx + 'a;

pub trait IWindowCtx
{
    //fn run<A : AppLoop>(self, app : &mut A) -> AppResult;
   fn new_window(&mut self, param : WindowParam) -> AppResult<WindowID>;

   fn exit(&mut self);

   fn clipboard_get(&mut self) -> Option<String>;
   fn clipboard_set(&mut self, paste : String) -> Result<(), ()>;
}

pub trait WinitConvert<Output>
{
    fn convert(self) -> Output;
}

pub use winit::event_loop::EventLoopProxy;

impl<T> WinitConvert<Vec2> for winit::dpi::LogicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Point2> for winit::dpi::LogicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}


impl<T> WinitConvert<Vec2> for winit::dpi::PhysicalSize<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.width.to_float(), self.height.to_float()) }
}
impl<T> WinitConvert<Point2> for winit::dpi::PhysicalSize<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.width.to_int(), self.height.to_int()) }
}


impl<T> WinitConvert<Vec2> for winit::dpi::PhysicalPosition<T> where T : ToFloat<Output = float>
{
    fn convert(self) -> Vec2 { vec2(self.x.to_float(), self.y.to_float()) }
}
impl<T> WinitConvert<Point2> for winit::dpi::PhysicalPosition<T> where T : ToInt<Output = int>
{
    fn convert(self) -> Point2 { point2(self.x.to_int(), self.y.to_int()) }
}
/*
impl WinitConvertWithDpi<Vec2> for winit::dpi::PhysicalSize<u32>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
impl WinitConvert for winit::dpi::LogicalPosition<f64>
{
    type Output = Vec2;
    fn convert(self) -> Self::Output { vec2(self.x as _, self.y as _) }
}

impl WinitConvertWithDpi for winit::dpi::PhysicalPosition<i32>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
impl WinitConvertWithDpi for winit::dpi::PhysicalPosition<f64>
{
    type Output = Vec2;
    fn convert_with_dpi(self, dpi : float) -> Self::Output { self.to_logical(dpi as _).convert() }
}
*/




impl<'a, A, T> winit::application::ApplicationHandler<T> for WindowRunner<'a, A, T> where A : WindowLoop<T>, T: 'static
{
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        winit_event: winit::event::WindowEvent,
    ) {

        match self.ctx.convert_winit_event(window_id, winit_event)
        {
            Some(e) =>
            {
                self.handle_message(e, event_loop);
                if let Event::Key(k) = e.event
                {
                    if !k.is_repeat()
                    {
                        if let Some(mods) = KeyMods::from_keycode(k.key)
                        {
                            self.ctx.modifier.set(mods, k.action.is_press());
                            self.handle_message(LocalizedEvent::new(window_id.into(), self.ctx.modifier.into(), DeviceID::OS), event_loop);
                        }
                    }
                }
            },
            None => return,
        };
    }


    fn device_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _device_id: winit::event::DeviceId,
            event: winit::event::DeviceEvent,
        )
    {
        let msg = match event
        {
            winit::event::DeviceEvent::Added => DeviceMessage::Added,
            winit::event::DeviceEvent::Removed => DeviceMessage::Removed,
            _ => return,
        };

        self.handle_message(msg, event_loop);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop)
    {
        if let Some(w) = self.ctx.default_window.take()
        {
            let Self { app : _, ctx, proxy: _phantom } = self;
            let mut app_ctx = AppCtx { ctx, active_event_loop: event_loop };
            app_ctx.new_window(w).expect("Failed to create the main window");
        }
        self.handle_message(DeviceMessage::Resume, event_loop);
    }

    fn memory_warning(&mut self, event_loop: &winit::event_loop::ActiveEventLoop)
    {
        self.handle_message(DeviceMessage::MemoryWarning, event_loop);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.handle_message(DeviceMessage::Update, event_loop);
    }

    fn exiting(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.handle_message(DeviceMessage::Exit, event_loop);
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: T) {
        self.handle_message(EventMessage::User(event), event_loop);
    }
}

struct AppCtx<'a>
{
    ctx : &'a mut WindowContext,
    active_event_loop : &'a ActiveEventLoop
}
impl<'a> Deref for AppCtx<'a>
{
    type Target = WindowContext;
    fn deref(&self) -> &Self::Target {
        self.ctx
    }
}
impl<'a> DerefMut for AppCtx<'a>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ctx
    }
}

impl IWindowCtx for AppCtx<'_>
{
    fn exit(&mut self) { self.active_event_loop.exit(); }

    fn new_window(&mut self, param : WindowParam) -> AppResult<WindowID>
    {
        let cursor_visible = param.cursor_visible;
        let cursor_grab = param.cursor_grab;

        let window = self.active_event_loop
            .create_window(param.clone().into())
            .map_err(|_| AppError::Unknow)?;

        let _ = window.set_cursor_grab(cursor_grab.into());
        window.set_cursor_visible(cursor_visible);

        let id = WindowID(window.id());
        let win = Window
        {
            window : SharedWinitWindow::new(window),
            childs: ___(),
            id,
            param,
        };

        self.ctx.windows.insert(id, win);
        Ok(id)
    }

    fn clipboard_get(&mut self) -> Option<String> {
        self.copy_paste.as_mut().and_then(|ctx| ctx.get_contents().ok())
    }

    fn clipboard_set(&mut self, paste : String) -> Result<(), ()> {
        let r = self.copy_paste.as_mut()
            .and_then(|ctx| ctx.set_contents(paste).ok());
        if r.is_some() {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct WindowContext
{
    windows  : HashMap<WindowID, Window>,

    mouse    : Option<Vec2>,
    modifier : KeyModsFlags,

    copy_paste : Option<ClipboardContext>,

    default_window : Option<WindowParam>,
}

impl std::fmt::Debug for WindowContext
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // copy_paste don't support debug
        f.debug_struct("WindowContext")
            .field("windows", &self.windows)
            .field("mouse", &self.mouse)
            .field("modifier", &self.modifier)
            .field("default_window", &self.default_window)
            .finish()
    }
}


impl Default for WindowContext
{
    fn default() -> Self {
        Self::new()
    }
}

impl WindowContext
{
    fn new() -> Self
    {
        Self
        {
            windows: ___(),
            mouse: ___(),
            modifier: ___(),
            copy_paste: ClipboardContext::new().ok(),
            default_window: ___()
        }
    }
}

impl WindowContext
{
    fn convert_winit_event(&mut self, window_id: winit::window::WindowId, winit_event: winit::event::WindowEvent) -> Option<LocalizedEvent>
    {
        let Self { windows, mouse, modifier, default_window : _, copy_paste : _ } = self;

        let window_id = window_id.into();
        let _dpi = windows.get(&window_id).map(|w| w.param().dpi).unwrap_or(1.);

        let event : Event = match winit_event
        {
            winit::event::WindowEvent::Resized(physical_size) => WindowEvent::Resize(physical_size.convert()).into(),
            winit::event::WindowEvent::Moved(physical_position) => WindowEvent::Move(physical_position.convert()).into(),
            winit::event::WindowEvent::CloseRequested => WindowEvent::Quit.into(),
            winit::event::WindowEvent::Destroyed => return None,
            winit::event::WindowEvent::DroppedFile(_path_buf) => return None,
            winit::event::WindowEvent::HoveredFile(_path_buf) => return None,
            winit::event::WindowEvent::HoveredFileCancelled => return None,
            winit::event::WindowEvent::Focused(b) => WindowEvent::Focus(b).into(),
            winit::event::WindowEvent::KeyboardInput { device_id : _, event, is_synthetic : _ } =>
            {
                let character = event.text.map(|txt| if txt.len() ==  1 { txt.chars().next() } else { None }).flatten();

                let key = match event.physical_key
                {
                    winit::keyboard::PhysicalKey::Code(key_code) => match key_code
                    {
                        winit::keyboard::KeyCode::Backquote => KeyCode::Backquote,
                        winit::keyboard::KeyCode::Backslash => KeyCode::Backslash,
                        winit::keyboard::KeyCode::BracketLeft => KeyCode::BracketLeft,
                        winit::keyboard::KeyCode::BracketRight => KeyCode::BracketRight,
                        winit::keyboard::KeyCode::Comma => KeyCode::Comma,
                        winit::keyboard::KeyCode::Digit0 => KeyCode::Key0,
                        winit::keyboard::KeyCode::Digit1 => KeyCode::Key1,
                        winit::keyboard::KeyCode::Digit2 => KeyCode::Key2,
                        winit::keyboard::KeyCode::Digit3 => KeyCode::Key3,
                        winit::keyboard::KeyCode::Digit4 => KeyCode::Key4,
                        winit::keyboard::KeyCode::Digit5 => KeyCode::Key5,
                        winit::keyboard::KeyCode::Digit6 => KeyCode::Key6,
                        winit::keyboard::KeyCode::Digit7 => KeyCode::Key7,
                        winit::keyboard::KeyCode::Digit8 => KeyCode::Key8,
                        winit::keyboard::KeyCode::Digit9 => KeyCode::Key9,
                        winit::keyboard::KeyCode::Equal => KeyCode::Equal,
                        winit::keyboard::KeyCode::IntlBackslash => KeyCode::IntlBackslash,
                        winit::keyboard::KeyCode::IntlRo => KeyCode::IntlRo,
                        winit::keyboard::KeyCode::IntlYen => KeyCode::IntlYen,
                        winit::keyboard::KeyCode::KeyA => KeyCode::A,
                        winit::keyboard::KeyCode::KeyB => KeyCode::B,
                        winit::keyboard::KeyCode::KeyC => KeyCode::C,
                        winit::keyboard::KeyCode::KeyD => KeyCode::D,
                        winit::keyboard::KeyCode::KeyE => KeyCode::E,
                        winit::keyboard::KeyCode::KeyF => KeyCode::F,
                        winit::keyboard::KeyCode::KeyG => KeyCode::G,
                        winit::keyboard::KeyCode::KeyH => KeyCode::H,
                        winit::keyboard::KeyCode::KeyI => KeyCode::I,
                        winit::keyboard::KeyCode::KeyJ => KeyCode::J,
                        winit::keyboard::KeyCode::KeyK => KeyCode::K,
                        winit::keyboard::KeyCode::KeyL => KeyCode::L,
                        winit::keyboard::KeyCode::KeyM => KeyCode::M,
                        winit::keyboard::KeyCode::KeyN => KeyCode::N,
                        winit::keyboard::KeyCode::KeyO => KeyCode::O,
                        winit::keyboard::KeyCode::KeyP => KeyCode::P,
                        winit::keyboard::KeyCode::KeyQ => KeyCode::Q,
                        winit::keyboard::KeyCode::KeyR => KeyCode::R,
                        winit::keyboard::KeyCode::KeyS => KeyCode::S,
                        winit::keyboard::KeyCode::KeyT => KeyCode::T,
                        winit::keyboard::KeyCode::KeyU => KeyCode::U,
                        winit::keyboard::KeyCode::KeyV => KeyCode::V,
                        winit::keyboard::KeyCode::KeyW => KeyCode::W,
                        winit::keyboard::KeyCode::KeyX => KeyCode::X,
                        winit::keyboard::KeyCode::KeyY => KeyCode::Y,
                        winit::keyboard::KeyCode::KeyZ => KeyCode::Z,
                        winit::keyboard::KeyCode::Minus => KeyCode::Minus,
                        winit::keyboard::KeyCode::Period => KeyCode::Period,
                        winit::keyboard::KeyCode::Quote => KeyCode::Apostrophe,
                        winit::keyboard::KeyCode::Semicolon => KeyCode::Semicolon,
                        winit::keyboard::KeyCode::Slash => KeyCode::Slash,
                        winit::keyboard::KeyCode::AltLeft => KeyCode::AltLeft,
                        winit::keyboard::KeyCode::AltRight => KeyCode::AltRight,
                        winit::keyboard::KeyCode::Backspace => KeyCode::Backspace,
                        winit::keyboard::KeyCode::CapsLock => KeyCode::CapsLock,
                        winit::keyboard::KeyCode::ContextMenu => KeyCode::ContextMenu,
                        winit::keyboard::KeyCode::ControlLeft => KeyCode::ControlLeft,
                        winit::keyboard::KeyCode::ControlRight => KeyCode::ControlRight,
                        winit::keyboard::KeyCode::Enter => KeyCode::Enter,
                        winit::keyboard::KeyCode::SuperLeft => KeyCode::SuperLeft,
                        winit::keyboard::KeyCode::SuperRight => KeyCode::SuperRight,
                        winit::keyboard::KeyCode::ShiftLeft => KeyCode::ShiftLeft,
                        winit::keyboard::KeyCode::ShiftRight => KeyCode::ShiftRight,
                        winit::keyboard::KeyCode::Space => KeyCode::Space,
                        winit::keyboard::KeyCode::Tab => KeyCode::Tab,
                        winit::keyboard::KeyCode::Convert => KeyCode::Convert,
                        winit::keyboard::KeyCode::KanaMode => KeyCode::KanaMode,
                        winit::keyboard::KeyCode::Lang1 => KeyCode::Lang1,
                        winit::keyboard::KeyCode::Lang2 => KeyCode::Lang2,
                        winit::keyboard::KeyCode::Lang3 => KeyCode::Lang3,
                        winit::keyboard::KeyCode::Lang4 => KeyCode::Lang4,
                        winit::keyboard::KeyCode::Lang5 => KeyCode::Lang5,
                        winit::keyboard::KeyCode::NonConvert => KeyCode::NonConvert,
                        winit::keyboard::KeyCode::Delete => KeyCode::Delete,
                        winit::keyboard::KeyCode::End => KeyCode::End,
                        winit::keyboard::KeyCode::Help => KeyCode::Help,
                        winit::keyboard::KeyCode::Home => KeyCode::Home,
                        winit::keyboard::KeyCode::Insert => KeyCode::Insert,
                        winit::keyboard::KeyCode::PageDown => KeyCode::PageDown,
                        winit::keyboard::KeyCode::PageUp => KeyCode::PageUp,
                        winit::keyboard::KeyCode::ArrowDown => KeyCode::ArrowDown,
                        winit::keyboard::KeyCode::ArrowLeft => KeyCode::ArrowLeft,
                        winit::keyboard::KeyCode::ArrowRight => KeyCode::ArrowRight,
                        winit::keyboard::KeyCode::ArrowUp => KeyCode::ArrowUp,
                        winit::keyboard::KeyCode::NumLock => KeyCode::NumLock,
                        winit::keyboard::KeyCode::Numpad0 => KeyCode::Numpad0,
                        winit::keyboard::KeyCode::Numpad1 => KeyCode::Numpad1,
                        winit::keyboard::KeyCode::Numpad2 => KeyCode::Numpad2,
                        winit::keyboard::KeyCode::Numpad3 => KeyCode::Numpad3,
                        winit::keyboard::KeyCode::Numpad4 => KeyCode::Numpad4,
                        winit::keyboard::KeyCode::Numpad5 => KeyCode::Numpad5,
                        winit::keyboard::KeyCode::Numpad6 => KeyCode::Numpad6,
                        winit::keyboard::KeyCode::Numpad7 => KeyCode::Numpad7,
                        winit::keyboard::KeyCode::Numpad8 => KeyCode::Numpad8,
                        winit::keyboard::KeyCode::Numpad9 => KeyCode::Numpad9,
                        winit::keyboard::KeyCode::NumpadAdd => KeyCode::NumpadAdd,
                        winit::keyboard::KeyCode::NumpadBackspace => KeyCode::NumpadBackspace,
                        winit::keyboard::KeyCode::NumpadClear => KeyCode::NumpadClear,
                        winit::keyboard::KeyCode::NumpadClearEntry => KeyCode::NumpadClearEntry,
                        winit::keyboard::KeyCode::NumpadComma => KeyCode::NumpadComma,
                        winit::keyboard::KeyCode::NumpadDecimal => KeyCode::NumpadDecimal,
                        winit::keyboard::KeyCode::NumpadDivide => KeyCode::NumpadDivide,
                        winit::keyboard::KeyCode::NumpadEnter => KeyCode::NumpadEnter,
                        winit::keyboard::KeyCode::NumpadEqual => KeyCode::NumpadEqual,
                        winit::keyboard::KeyCode::NumpadHash => KeyCode::NumpadHash,
                        winit::keyboard::KeyCode::NumpadMemoryAdd => KeyCode::NumpadMemoryAdd,
                        winit::keyboard::KeyCode::NumpadMemoryClear => KeyCode::NumpadMemoryClear,
                        winit::keyboard::KeyCode::NumpadMemoryRecall => KeyCode::NumpadMemoryRecall,
                        winit::keyboard::KeyCode::NumpadMemoryStore => KeyCode::NumpadMemoryStore,
                        winit::keyboard::KeyCode::NumpadMemorySubtract => KeyCode::NumpadMemorySubtract,
                        winit::keyboard::KeyCode::NumpadMultiply => KeyCode::NumpadMultiply,
                        winit::keyboard::KeyCode::NumpadParenLeft => KeyCode::NumpadParenLeft,
                        winit::keyboard::KeyCode::NumpadParenRight => KeyCode::NumpadParenRight,
                        winit::keyboard::KeyCode::NumpadStar => KeyCode::NumpadStar,
                        winit::keyboard::KeyCode::NumpadSubtract => KeyCode::NumpadSubtract,
                        winit::keyboard::KeyCode::Escape => KeyCode::Escape,
                        winit::keyboard::KeyCode::Fn => KeyCode::Fn,
                        winit::keyboard::KeyCode::FnLock => KeyCode::FnLock,
                        winit::keyboard::KeyCode::PrintScreen => KeyCode::PrintScreen,
                        winit::keyboard::KeyCode::ScrollLock => KeyCode::ScrollLock,
                        winit::keyboard::KeyCode::Pause => KeyCode::Pause,
                        winit::keyboard::KeyCode::BrowserBack => KeyCode::BrowserBack,
                        winit::keyboard::KeyCode::BrowserFavorites => KeyCode::BrowserFavorites,
                        winit::keyboard::KeyCode::BrowserForward => KeyCode::BrowserForward,
                        winit::keyboard::KeyCode::BrowserHome => KeyCode::BrowserHome,
                        winit::keyboard::KeyCode::BrowserRefresh => KeyCode::BrowserRefresh,
                        winit::keyboard::KeyCode::BrowserSearch => KeyCode::BrowserSearch,
                        winit::keyboard::KeyCode::BrowserStop => KeyCode::BrowserStop,
                        winit::keyboard::KeyCode::Eject => KeyCode::Eject,
                        winit::keyboard::KeyCode::LaunchApp1 => KeyCode::LaunchApp1,
                        winit::keyboard::KeyCode::LaunchApp2 => KeyCode::LaunchApp2,
                        winit::keyboard::KeyCode::LaunchMail => KeyCode::LaunchMail,
                        winit::keyboard::KeyCode::MediaPlayPause => KeyCode::MediaPlayPause,
                        winit::keyboard::KeyCode::MediaSelect => KeyCode::MediaSelect,
                        winit::keyboard::KeyCode::MediaStop => KeyCode::MediaStop,
                        winit::keyboard::KeyCode::MediaTrackNext => KeyCode::MediaTrackNext,
                        winit::keyboard::KeyCode::MediaTrackPrevious => KeyCode::MediaTrackPrevious,
                        winit::keyboard::KeyCode::Power => KeyCode::Power,
                        winit::keyboard::KeyCode::Sleep => KeyCode::Sleep,
                        winit::keyboard::KeyCode::AudioVolumeDown => KeyCode::AudioVolumeDown,
                        winit::keyboard::KeyCode::AudioVolumeMute => KeyCode::AudioVolumeMute,
                        winit::keyboard::KeyCode::AudioVolumeUp => KeyCode::AudioVolumeUp,
                        winit::keyboard::KeyCode::WakeUp => KeyCode::WakeUp,
                        winit::keyboard::KeyCode::Meta => KeyCode::Meta,
                        winit::keyboard::KeyCode::Hyper => KeyCode::Hyper,
                        winit::keyboard::KeyCode::Turbo => KeyCode::Turbo,
                        winit::keyboard::KeyCode::Abort => KeyCode::Abort,
                        winit::keyboard::KeyCode::Resume => KeyCode::Resume,
                        winit::keyboard::KeyCode::Suspend => KeyCode::Suspend,
                        winit::keyboard::KeyCode::Again => KeyCode::Again,
                        winit::keyboard::KeyCode::Copy => KeyCode::Copy,
                        winit::keyboard::KeyCode::Cut => KeyCode::Cut,
                        winit::keyboard::KeyCode::Find => KeyCode::Find,
                        winit::keyboard::KeyCode::Open => KeyCode::Open,
                        winit::keyboard::KeyCode::Paste => KeyCode::Paste,
                        winit::keyboard::KeyCode::Props => KeyCode::Props,
                        winit::keyboard::KeyCode::Select => KeyCode::Select,
                        winit::keyboard::KeyCode::Undo => KeyCode::Undo,
                        winit::keyboard::KeyCode::Hiragana => KeyCode::Hiragana,
                        winit::keyboard::KeyCode::Katakana => KeyCode::Katakana,
                        winit::keyboard::KeyCode::F1 => KeyCode::F1,
                        winit::keyboard::KeyCode::F2 => KeyCode::F2,
                        winit::keyboard::KeyCode::F3 => KeyCode::F3,
                        winit::keyboard::KeyCode::F4 => KeyCode::F4,
                        winit::keyboard::KeyCode::F5 => KeyCode::F5,
                        winit::keyboard::KeyCode::F6 => KeyCode::F6,
                        winit::keyboard::KeyCode::F7 => KeyCode::F7,
                        winit::keyboard::KeyCode::F8 => KeyCode::F8,
                        winit::keyboard::KeyCode::F9 => KeyCode::F9,
                        winit::keyboard::KeyCode::F10 => KeyCode::F10,
                        winit::keyboard::KeyCode::F11 => KeyCode::F11,
                        winit::keyboard::KeyCode::F12 => KeyCode::F12,
                        winit::keyboard::KeyCode::F13 => KeyCode::F13,
                        winit::keyboard::KeyCode::F14 => KeyCode::F14,
                        winit::keyboard::KeyCode::F15 => KeyCode::F15,
                        winit::keyboard::KeyCode::F16 => KeyCode::F16,
                        winit::keyboard::KeyCode::F17 => KeyCode::F17,
                        winit::keyboard::KeyCode::F18 => KeyCode::F18,
                        winit::keyboard::KeyCode::F19 => KeyCode::F19,
                        winit::keyboard::KeyCode::F20 => KeyCode::F20,
                        winit::keyboard::KeyCode::F21 => KeyCode::F21,
                        winit::keyboard::KeyCode::F22 => KeyCode::F22,
                        winit::keyboard::KeyCode::F23 => KeyCode::F23,
                        winit::keyboard::KeyCode::F24 => KeyCode::F24,
                        winit::keyboard::KeyCode::F25 => KeyCode::F25,
                        winit::keyboard::KeyCode::F26 => KeyCode::F26,
                        winit::keyboard::KeyCode::F27 => KeyCode::F27,
                        winit::keyboard::KeyCode::F28 => KeyCode::F28,
                        winit::keyboard::KeyCode::F29 => KeyCode::F29,
                        winit::keyboard::KeyCode::F30 => KeyCode::F30,
                        winit::keyboard::KeyCode::F31 => KeyCode::F31,
                        winit::keyboard::KeyCode::F32 => KeyCode::F32,
                        winit::keyboard::KeyCode::F33 => KeyCode::F33,
                        winit::keyboard::KeyCode::F34 => KeyCode::F34,
                        winit::keyboard::KeyCode::F35 => KeyCode::F35,
                        _ => return None,
                    },
                    winit::keyboard::PhysicalKey::Unidentified(native_key_code) => KeyCode::Unknow(match native_key_code
                        {
                            winit::keyboard::NativeKeyCode::Unidentified => KeyCodeUnknow::Unknow,
                            winit::keyboard::NativeKeyCode::Android(v) => KeyCodeUnknow::Android(v),
                            winit::keyboard::NativeKeyCode::MacOS(v) => KeyCodeUnknow::MacOS(v),
                            winit::keyboard::NativeKeyCode::Windows(v) => KeyCodeUnknow::Windows(v),
                            winit::keyboard::NativeKeyCode::Xkb(v) => KeyCodeUnknow::Xkb(v),
                        })
                };

                let action = match event.state
                {
                    winit::event::ElementState::Pressed => EventAction::Press,
                    winit::event::ElementState::Released => EventAction::Release,
                };



                KeyEvent
                {
                    character,
                    key,
                    modifiers: *modifier,
                    repeat: event.repeat,
                    action: action,
                }.into()
            },
            winit::event::WindowEvent::ModifiersChanged(_modifiers) => return None,
            winit::event::WindowEvent::Ime(_) => return None,
            winit::event::WindowEvent::CursorMoved { device_id : _, position } =>
            {
                let position = position.convert();
                let old_mouse = mouse.unwrap_or(position);
                *mouse = Some(position);

                MouseMoveEvent
                {
                    position,
                    delta: position - old_mouse,
                }.into()
            },
            winit::event::WindowEvent::CursorEntered { device_id : _ } => return None,
            winit::event::WindowEvent::CursorLeft { device_id : _ } => return None,
            winit::event::WindowEvent::MouseWheel { device_id :_, delta, phase: _ } =>
            {
                let delta = match delta
                {
                    winit::event::MouseScrollDelta::LineDelta(x, y) => vec2(x as _, y as _),
                    winit::event::MouseScrollDelta::PixelDelta(physical_position) => physical_position.convert(),
                };
                MouseEvent::Wheel(delta).into()
            },
            winit::event::WindowEvent::MouseInput { device_id : _, state, button } =>
            {
                let button = match button
                {
                    winit::event::MouseButton::Left => MouseButton::Left,
                    winit::event::MouseButton::Right => MouseButton::Right,
                    winit::event::MouseButton::Middle => MouseButton::Middle,
                    winit::event::MouseButton::Back => MouseButton::Back,
                    winit::event::MouseButton::Forward => MouseButton::Forward,
                    winit::event::MouseButton::Other(b) => MouseButton::Unknow(b),
                };
                MouseButtonEvent
                {
                    position: mouse.unwrap_or(zero()),
                    button,
                    action: match state
                    {
                        winit::event::ElementState::Pressed => EventAction::Press,
                        winit::event::ElementState::Released => EventAction::Release,
                    }
                }.into()
            },
            winit::event::WindowEvent::Touch(touch) =>
            {
                TouchEvent
                {
                    id: TouchID::new(touch.id),
                    phase: match touch.phase
                    {
                        winit::event::TouchPhase::Started => TouchPhase::Begin,
                        winit::event::TouchPhase::Moved => TouchPhase::Move,
                        winit::event::TouchPhase::Ended => TouchPhase::End,
                        winit::event::TouchPhase::Cancelled => TouchPhase::Cancel,
                    },
                    position: touch.location.convert(),
                }.into()
            },
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor: _, inner_size_writer: _ } =>
            {
                // Todo: Handle dpi
                return None;
            },
            winit::event::WindowEvent::ThemeChanged(_theme) => return None,
            winit::event::WindowEvent::Occluded(v) => WindowEvent::Visible(v).into(),
            winit::event::WindowEvent::RedrawRequested => WindowEvent::Draw.into(),
            _ => return None,
        };

        let localized_event = LocalizedEvent
        {
            window : window_id,
            event,
            device: DeviceID::OS, // Todo : fit it
        };

        return Some(localized_event);
    }


}


pub type AppResult<T=()> = Result<T,AppError>;

pub type AppErrorEventLoop = winit::error::EventLoopError;

//#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Debug)]
pub enum AppError
{
    Unknow,
    EventLoop(AppErrorEventLoop),
}
impl From<AppErrorEventLoop> for AppError
{
    fn from(value: AppErrorEventLoop) -> Self {
        Self::EventLoop(value)
    }
}
impl std::fmt::Display for AppError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize), serde(transparent))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DeviceID(usize);

impl DeviceID
{
    pub const OS : Self = DeviceID(0);
}