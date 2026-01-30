use super::*;
use app::*;

pub fn run<A, F>(f: F) -> AppResult
where
    A: Application + 'static,
    F: AppInit<A>,
{
    run_message_handler_with_param(|| AppMessageAdapter::new(f()), AppParam::default())
}
pub fn run_with_param<A, F, P>(f: F, param: P) -> AppResult
where
    A: Application + 'static,
    F: AppInit<A>,
    P: Into<AppParamInternal>,
{
    run_message_handler_with_param(|| AppMessageAdapter::new(f()), param)
}

pub fn run_message_handler<A, F>(f: F) -> AppResult
where
    A: AppMessageHandler,
    F: AppInit<A>,
{
    AppRunner::run_with_param(f, AppParam::default())
}
pub fn run_message_handler_with_param<A, F, P>(f: F, param: P) -> AppResult
where
    A: AppMessageHandler,
    F: AppInit<A>,
    P: Into<AppParamInternal>,
{
    AppRunner::run_with_param(f, param)
}
