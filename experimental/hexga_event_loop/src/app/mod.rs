use super::*;

mod app;
pub use app::*;

mod runner;
pub use runner::*;

mod event;
pub use event::*;

mod spawn;
pub use spawn::*;

mod event_loop;
pub use event_loop::*;

pub mod experimental
{
    use super::*;

    pub type WinitEventLoopProxy<CustomEvent> = ::winit::event_loop::EventLoopProxy<AppMessage<CustomEvent>>;

    pub(crate) type WinitEventLoopActive = ::winit::event_loop::ActiveEventLoop;
    pub(crate) type WinitEventLoop<User> = ::winit::event_loop::EventLoop<User>;
    pub(crate) type WinitWindowEvent = ::winit::event::WindowEvent;
}

pub mod prelude
{
    pub use super::{
        EventLoop, AppError, AppResult, 
        AppMessage, AppControlFlow, AppEvent, 
        AppParam, AppShortcut
    };
    //pub use super::{HasMutWindow,HasMutKeyboard,HasMutClipboard,HasMutGraphics};
    //pub(crate) use super::{AppDefaultUserEventInner};
    pub(crate) use super::experimental::*;
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{AsyncSpawn, AppSendEvent, IAppEvent, App, AppRun, WithAppParam, WithAppShortcut};
}

pub type AppResult<T = ()> = Result<T, AppError>;
pub type AppError = ();

pub fn init_if_needed()
{
    crash::init_if_needed();
    log::init_if_needed();
}
