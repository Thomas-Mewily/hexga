use std::collections::HashMap;

use crate::*;

pub mod prelude { }
pub mod modules { }

pub trait AppLoop
{
    //type Output;

    fn handle_localized_event<R>(&mut self, event : LocalizedEvent, ctx : &mut R) -> bool where R: AppContext
    {
        /*
        if let Event::Device(DeviceEvent::Draw) = event
        {
            self.draw();
            return true;
        }
        */
        self.handle_event(event.event, ctx)
    }

    #[allow(unused_variables)]
    fn handle_event<R>(&mut self, event : Event, ctx : &mut R) -> bool where R: AppContext
    { false }

    fn update<R>(&mut self, ctx : &mut R) where R: AppContext;
    fn draw<R>(&mut self, ctx : &mut R) where R: AppContext;

    fn run(&mut self) -> AppResult where Self : Sized
    {
        todo!();
        let ev_loop = EventLoop::new().map_err(|e| <AppErrorEventLoop as Into<AppError>>::into(e))?;
        let mut runner = AppRunner
        {
            app : self,
            ctx: todo!(), // AppContextInternal { windows: ___() },
        };
        ev_loop.run_app(&mut runner).map_err(|e| e.into())
    }
}



struct AppRunner<'a, A> where A : AppLoop
{
    app : &'a mut A,
    ctx : AppContextInternal,
}

pub trait AppContext
{
    //fn run<A : AppLoop>(self, app : &mut A) -> AppResult;
   fn new_window(&mut self, param : WindowParam) -> AppResult<WindowID>;
}


pub(crate) trait WinitConvertWithDpi
{
    type Output;
    fn convert_with_dpi(self, dpi : float) -> Self::Output;
}
pub(crate) trait WinitConvert
{
    type Output;
    fn convert(self) -> Self::Output;
}

impl WinitConvert for winit::dpi::LogicalSize<f64>
{
    type Output = Vec2;
    fn convert(self) -> Self::Output { vec2(self.width as _, self.height as _) }
}
impl WinitConvertWithDpi for winit::dpi::PhysicalSize<u32>
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

impl<'a, A> winit::application::ApplicationHandler for AppRunner<'a, A> where A : AppLoop
{
    fn resumed(&mut self, active_event_loop: &ActiveEventLoop)
    {
        let Self { app, ctx } = self;
        let mut app_ctx = AppCtx { ctx, active_event_loop };
        app.handle_localized_event(DeviceEvent::Resume.into(), &mut app_ctx);
    }

    fn window_event(
        &mut self,
        active_event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        let Self { app, ctx } = self;

        let dpi = ctx.windows.get(&window_id.into()).map(|w| w.dpi).unwrap_or(1.);

        let event2 : Event = match event
        {
            winit::event::WindowEvent::Resized(physical_size) => WindowEvent::Resize(physical_size.convert_with_dpi(dpi)).into(),
            winit::event::WindowEvent::Moved(physical_position) => WindowEvent::Move(physical_position.convert_with_dpi(dpi)).into(),
            winit::event::WindowEvent::CloseRequested => WindowEvent::Quit.into(),
            winit::event::WindowEvent::Destroyed => return,
            winit::event::WindowEvent::DroppedFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFile(path_buf) => todo!(),
            winit::event::WindowEvent::HoveredFileCancelled => todo!(),
            winit::event::WindowEvent::Focused(b) => WindowEvent::Focus(b).into(),
            winit::event::WindowEvent::KeyboardInput { device_id, event, is_synthetic } =>
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
                        _ => todo!(),
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
                    modifiers: KeyMods::default(), // Todo fix it
                    repeat: event.repeat,
                    action: action,
                }.into()
            },
            winit::event::WindowEvent::ModifiersChanged(modifiers) =>
            {
                todo!()
                //self.ctx.modifier.alt = modifiers.lalt_state().
            },
            winit::event::WindowEvent::Ime(ime) => todo!(),
            winit::event::WindowEvent::CursorMoved { device_id, position } => todo!(),
            winit::event::WindowEvent::CursorEntered { device_id } => todo!(),
            winit::event::WindowEvent::CursorLeft { device_id } => todo!(),
            winit::event::WindowEvent::MouseWheel { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::MouseInput { device_id, state, button } => todo!(),
            winit::event::WindowEvent::PinchGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::PanGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::DoubleTapGesture { device_id } => todo!(),
            winit::event::WindowEvent::RotationGesture { device_id, delta, phase } => todo!(),
            winit::event::WindowEvent::TouchpadPressure { device_id, pressure, stage } => todo!(),
            winit::event::WindowEvent::AxisMotion { device_id, axis, value } => todo!(),
            winit::event::WindowEvent::Touch(touch) => todo!(),
            winit::event::WindowEvent::ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            winit::event::WindowEvent::ThemeChanged(theme) => todo!(),
            winit::event::WindowEvent::Occluded(_) => todo!(),
            winit::event::WindowEvent::RedrawRequested => todo!(),
            _ => return,
        };

        todo!();
        /*
        let LocalizedEvent
        {
            window : Some(window_id.into()),
            event,
            device: DeviceID::OS, // Todo : fit it
        };

        let mut app_ctx = AppCtx { ctx, active_event_loop };
        app.handle_localized_event(, &mut app_ctx);
        */
    }
}

struct AppCtx<'a>
{
    ctx : &'a mut AppContextInternal,
    active_event_loop : &'a ActiveEventLoop
}
impl<'a> AppCtx<'a>
{
    pub fn new(ctx : &'a mut AppContextInternal, active_event_loop : &'a ActiveEventLoop) -> Self
    {
        Self { ctx, active_event_loop }
    }
}
impl AppContext for AppCtx<'_>
{
    fn new_window(&mut self, param : WindowParam) -> AppResult<WindowID>
    {
        let window = self.active_event_loop
            .create_window(param.clone().into())
            .map_err(|_| AppError::Unknow)?;

        window.set_cursor_grab(param.grab.into());

        let id = WindowID(window.id());
        let win = Window
        {
            window,
            childs: ___(),
            dpi: param.dpi,
        };

        self.ctx.windows.insert(id, win);
        Ok(id)
    }
}

#[derive(Default)]
struct AppContextInternal
{
    windows  : HashMap<WindowID, Window>,
    modifier : KeyMods,
}

pub type AppResult<T=()> = Result<T,AppError>;

pub type AppErrorEventLoop = winit::error::EventLoopError;

//#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
