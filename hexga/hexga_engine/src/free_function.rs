use super::*;
use app::*;



pub fn run<A,F>(f: F) -> AppResult
    where
    A: Application + 'static,
    F: AppInit<A>
{
    run_message_handler(|| AppMessageAdapter::new(f()))
}
pub fn run_with_param<A,F>(f: F, param: AppParam) -> AppResult
    where
    A: Application + 'static,
    F: AppInit<A>
{
    run_message_handler_with_param(|| AppMessageAdapter::new(f()), param)
}

pub fn run_message_handler<A,F>(f: F) -> AppResult
    where
    A: AppMessageHandler,
    F: AppInit<A>
{
    AppRunner::run(f)
}
pub fn run_message_handler_with_param<A,F>(f: F, param: AppParam) -> AppResult
    where
    A: AppMessageHandler,
    F: AppInit<A>
{
    AppRunner::run_with_param(f, param)
}