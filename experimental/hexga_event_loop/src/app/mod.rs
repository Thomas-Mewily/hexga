use super::*;

mod app;
pub use app::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

mod spawn;
pub use spawn::*;

mod theme;
pub use theme::*;

mod event_loop;
pub use event_loop::*;

pub mod experimental
{
    use super::*;

    pub type WinitEventLoopProxy<CustomEvent> = ::winit::event_loop::EventLoopProxy<AppMessage<CustomEvent>>;

    pub(crate) type WinitEventLoopActive = ::winit::event_loop::ActiveEventLoop;
    pub(crate) type WinitEventLoop<User> = ::winit::event_loop::EventLoop<User>;
    pub(crate) type WinitWindowEvent = ::winit::event::WindowEvent;
    pub(crate) use ::winit::event as WinitEvent;
}

pub mod prelude
{
    pub use super::app::EventLoop;
    pub use super::event::{AppEvent, AppFlow, AppMessage};
    pub use super::runner::{AppControlFlow, AppShortcut, EventLoopParam};
    pub use super::theme::AppTheme;
    pub use super::{AppError, AppResult};
    //pub use super::{HasMutWindow,HasMutKeyboard,HasMutClipboard,HasMutGraphics};
    //pub(crate) use super::{AppDefaultUserEventInner};
    pub(crate) use super::experimental::*;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::app::{App, AppSendEvent};
    pub use super::event::IAppEvent;
    pub use super::runner::{AppRunRaw, WithAppParam, WithAppShortcut};
    pub use super::spawn::AsyncSpawn;
}

pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppError = ();

pub fn init_if_needed()
{
    crash::init_if_needed();
    log::init_if_needed();
}
