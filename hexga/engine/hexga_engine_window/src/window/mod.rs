//! mainly inspired by miniquad

use std::default;

use hexga_generational::prelude::{GenVec, GenVecID};
use hexga_math::prelude::*;
use crate::*;

pub mod prelude
{
    pub use super::{ContextWindow,WindowParam};
    pub use super::WindowID;
}

pub trait AppLoop
{
    //type Output;

    fn handle_localized_event(&mut self, event : LocalizedEvent, ctx : &mut AppContextInternal) -> bool
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
    fn handle_event(&mut self, event : Event, ctx : &mut AppContextInternal) -> bool { false }

    fn update(&mut self, ctx : &mut AppContextInternal);
    fn draw(&mut self, ctx : &mut AppContextInternal);

    fn run(&mut self) -> AppResult where Self : Sized
    {
        let ev_loop = EventLoop::new().map_err(|e| <AppErrorEventLoop as Into<AppError>>::into(e))?;

        let mut runner = AppRunner
        {
            app : self,
            ctx: AppContextInternal { windows: ___() },
        };

        ev_loop.run_app(&mut runner).map_err(|e| e.into())
    }
}

pub type WindowID = GenVecID<Window>;


#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq, Default)]
pub enum CursorIcon
{
    #[default]
    Default,
    Help,
    Pointer,
    Wait,
    Crosshair,
    Text,
    Move,
    NotAllowed,
    EWResize,
    NSResize,
    NESWResize,
    NWSEResize,
}



pub trait ContextWindow
{
    fn get_clipboard(&mut self) -> Option<String>;
    fn set_clipboard(&mut self, text : &str);

    fn dpi_scale(&mut self) -> float;
    fn is_dpi_hight(&mut self) -> bool;

    /// Quit the window
    fn quit(&mut self);
    /// Ask the user for a quitting confirmation and quit
    fn request_quit(&mut self);

    fn get_position(&mut self) -> Point2;
    fn set_position(&mut self, pos : Point2);

    /// Current window size in pixel (taking dpi in account)
    fn get_screen_size_tuple(&mut self) -> Point2;
    fn set_size(&mut self, size : Point2);


    fn set_fullscreen(&mut self, fullscreen: bool);


    fn show_keyboard(&mut self, show: bool);

    fn show_mouse(&mut self, show: bool);
    fn grab_mouse(&mut self, grab: bool);
    fn set_mouse_cursor(&mut self, cursor_icon: CursorIcon);
}

impl ContextWindow for ()
{
    fn get_clipboard(&mut self) -> Option<String> { None }
    fn set_clipboard(&mut self, _text : &str) {}

    fn dpi_scale(&mut self) -> f32 { 1.0 }
    fn is_dpi_hight(&mut self) -> bool { false }

    fn quit(&mut self) {}
    fn request_quit(&mut self) {}

    fn get_position(&mut self) -> Point2 { Point2::ZERO }
    fn set_position(&mut self, _pos : Point2) {}

    fn get_screen_size_tuple(&mut self) -> Point2 { Point2::ONE }
    fn set_size(&mut self, _size : Point2) {}

    fn set_fullscreen(&mut self, _fullscreen: bool) {}
    fn show_keyboard(&mut self, _show: bool) {}

    fn show_mouse(&mut self, _show: bool) {}
    fn grab_mouse(&mut self, _grab: bool) {}
    fn set_mouse_cursor(&mut self, _cursor_icon: CursorIcon) {}
}

pub struct AppBuilder;

/*
struct AppEventLoop<'a, A> where A : AppLoop
{
    runner : AppRunner<'a, A>,
    event_loop : EventLoop,
}
*/


struct AppRunner<'a, A> where A : AppLoop
{
    app : &'a mut A,
    ctx : AppContextInternal,
}

pub trait EventLoopRunner
{
    //fn run<A : AppLoop>(self, app : &mut A) -> AppResult;
   fn create_window(&mut self, param : WindowParam) -> AppResult<WindowID>;
}



pub(crate) static mut EVENT_LOOP : Option<&'static EventLoop> = None;

#[doc(hidden)]
pub(crate) fn event_loop() -> &'static EventLoop
{
    let ctx = unsafe { EVENT_LOOP.as_ref().unwrap() };
    ctx
}

#[doc(hidden)]
#[allow(static_mut_refs)]
pub unsafe fn set_event_loop(event_loop : Option<&'static EventLoop>) -> Option<&EventLoop>
{
    unsafe
    {
        match event_loop
        {
            Some(event_loop) =>
            {
                EVENT_LOOP = Some(event_loop);
                return None;
            },
            None => {
                EVENT_LOOP.take()
            }
        }
    }
}


impl<'a, A> winit::application::ApplicationHandler for AppRunner<'a, A> where A : AppLoop
{
    fn resumed(&mut self, ev_loop: &ActiveEventLoop)
    {
        ev_loop.cre
        unsafe { set_event_loop(Some(ev_loop.event_loop())).unwrap() };

        self.app.handle_localized_event(DeviceEvent::Resume.into());
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}

struct AppContext<'a>
{
    internal : &'a mut AppContextInternal,
    active_event_loop : &'a ActiveEventLoop
}

struct AppContextInternal
{
    windows : GenVec<Window>,
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

pub struct Window
{
    window : winit::window::Window,
    param  : WindowParam,
    id     : WindowID,
    childs : Vec<WindowID>,
}




//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct WindowParam
{
    /// Title of the window, defaults to an empty string.
    pub title: String,

    /*
    /// The preferred width / height of the window
    ///
    /// Default: [960, 540]
    pub size : Point2,
    */

    /*
    /// Whether the rendering canvas is full-resolution on HighDPI displays.
    ///
    /// Default: false
    pub high_dpi: bool,
    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fullscreen: bool,
    /// MSAA sample count
    ///
    /// Default: 1
    pub sample_count: u32,

    /// Determines if the application user can resize the window
    pub resizable: bool,

    /// The icon will be used as
    /// - taskbar and titlebar icons on Windows.
    /// - dock and titlebar icon on  MacOs.
    /// - TODO: favicon on HTML5
    /// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
    pub icon: Option<Box<Icon>>,

    pub can_be_free : bool,
    */
}

/*

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum WindowMode
{
    Free,
    /// Inside another window
    #[default]
    Nested,
}

impl Default for WindowParam
{
    fn default() -> Self
    {
        Self
        {
            title: String::new(),
            size : point2(960, 540),
            high_dpi: false,
            fullscreen: false,
            sample_count: 1,
            resizable: true,
            icon: None,
            can_be_free : true,
        }
    }
}

//#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
//#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
/// Icon image in three levels of detail.
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Icon {
    /// 16 * 16 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_16x16: [u8; 16 * 16 * 4],
    /// 32 x 32 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_32x32: [u8; 32 * 32 * 4],
    /// 64 x 64 image of RGBA pixels (each 4 * u8) in row-major order.
    pub rgba_64x64: [u8; 64 * 64 * 4],
}


impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon").finish()
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "hexga_io", derive(Save, Load))]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Platform
{
    /// Optional swap interval (vertical sync).
    ///
    /// Note that this is highly platform- and driver-dependent.
    /// There is no guarantee the FPS will match the specified `swap_interval`.
    /// In other words, `swap_interval` is only a hint to the GPU driver and
    /// not a reliable way to limit the game's FPS.
    pub swap_interval: Option<i32>,

    /// If `true`, the event loop will block until an schedule update will be is called.
    ///
    /// This can reduce CPU usage to nearly zero while waiting for events.
    pub blocking_event_loop: bool,

    /// If `true`, the framebuffer includes an alpha channel.
    /// Currently supported only on Android.
    ///
    /// - TODO: Make it works on web, on web it should make a transparent HTML5 canvas
    /// - TODO: Document(and check) what does it actually mean on android. Transparent window?
    pub framebuffer_alpha  : bool,
}

impl Default for Platform
{
    fn default() -> Self
    {
        Self
        {
            swap_interval: None,
            blocking_event_loop: false,
            framebuffer_alpha: false
        }
    }
}

impl WindowParam
{
    pub fn new() -> Self { Self::default() }

    pub fn title(mut self, title : impl Into<String>) -> Self { self.title = title.into(); self }

    /// Whether the rendering canvas is full-resolution on HighDPI displays.
    ///
    /// Default: false
    pub fn high_dpi(mut self, high_dpi : bool) -> Self { self.high_dpi = high_dpi; self }

    /// Whether the window should be created in fullscreen mode, ignored on wasm/android.
    ///
    /// Default: false
    pub fn fullscreen(mut self, fullscreen : bool) -> Self { self.fullscreen = fullscreen; self }

    /// MSAA sample count
    ///
    /// Default: 1
    pub fn sample_count(mut self, sample_count : u32) -> Self { self.sample_count = sample_count; self }

    /// Determines if the application user can resize the window
    pub fn resizeable(mut self, window_resizable : bool) -> Self { self.resizable = window_resizable; self }

    /// The icon will be used as
    /// - taskbar and titlebar icons on Windows.
    /// - dock and titlebar icon on  MacOs.
    /// - TODO: favicon on HTML5
    /// - TODO: taskbar and titlebar(highly dependent on the WM) icons on Linux
    pub fn icon(mut self, icon : Option<Icon>) -> Self { self.icon = icon.map(|v| Box::new(v)); self }

    /// Platform specific settings. Hints to OS for context creation, driver-specific
    /// settings etc.
    pub fn platform(mut self, platform : Platform) -> Self { self.platform = platform; self }
}
*/