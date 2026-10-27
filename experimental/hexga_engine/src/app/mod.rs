use super::*;

/*
mod param;
pub use param::*;
*/

mod runner;
pub use runner::*;

mod event;
pub use event::*;

pub use hexga_event_loop::app::{
    EventLoop, AppError, AppResult, 
    AppMessage, AppControlFlow, AppEvent, 
    AppParam, AppShortcut
};
pub use hexga_event_loop::app::{AsyncSpawn, AppSendEvent, IAppEvent, App, AppRun as AppRunRaw, WithAppParam, WithAppShortcut};


pub mod prelude
{
    pub use super::{
        EventLoop, AppError, AppResult, 
        AppMessage, AppControlFlow, AppEvent, 
        AppParam, AppShortcut
    };
    pub(crate) use super::{AppInternalEventLoop,AppCustomEvent,AppInternalProxy};
    pub use super::traits::*;
}

pub mod traits
{
    pub use super::{AsyncSpawn, AppSendEvent, IAppEvent, App, WithAppParam, WithAppShortcut};

    pub use super::AppRunRaw;
}

/*
// Todo wrap app run for the engine
pub trait AppRun<CustomEvent = ()> : App<CustomEvent> + Sized + 'static
    where CustomEvent: IAppEvent,
{
    fn run<F>(init : F) -> AppResult
        where F: FnOnce(EventLoopProxy<CustomEvent>) -> Self { Self::run_with_param(init, ___()) }
    fn run_with_param<F>(init : F, param: AppParam) -> AppResult
        where F: FnOnce(EventLoopProxy<CustomEvent>) -> Self
    {
        crate::app::init_if_needed();

        let event_loop = WinitEventLoop::with_user_event().build().ok_or_void()?;
        event_loop.set_control_flow(param.control_flow.into());
        let proxy = EventLoopProxy::new(event_loop.create_proxy());

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
*/
