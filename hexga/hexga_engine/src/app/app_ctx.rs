use std::marker::PhantomData;

use super::*;



#[derive(Default, Debug)]
pub struct AppParam
{
    title: String,
}


#[derive(Debug, Clone, Copy)]
pub struct AppCtx<'a,E> where E:IEvent
{
    proxy: &'a EventLoopProxy<E>,
    event_loop : &'a EventLoopActive,
}


#[derive(Debug)]
pub struct AppCore
{
    pub(crate) windows: AppWindows,
    pub(crate) gpu: Option<AppGpu>,
    pub(crate) input : AppInput,
    pub(crate) param: AppParam,
}


impl AppCore
{
    pub(crate) fn new(param: AppParam) -> Self { Self { param, windows: AppWindows::new(), input: ___(), gpu: ___() } }
}


impl<E> ScopedMessage<E> for AppCore where E:IEvent
{
    fn begin_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) {
        self.windows.begin_flow(flow, ctx);
        self.input.begin_flow(flow, ctx);
    }

    fn end_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) {
        self.windows.end_flow(flow, ctx);
        self.input.end_flow(flow, ctx);
    }

    fn begin_input(&mut self, input: &InputEvent, ctx: MessageCtx<'_,E>) {
        self.windows.begin_input(input, ctx);
        self.input.begin_input(input, ctx);
    }

    fn end_input(&mut self, ctx: MessageCtx<'_,E>) {
        self.windows.end_input(ctx);
        self.input.end_input(ctx);
    }

    fn begin_window(&mut self, window: &WindowEvent, ctx: MessageCtx<'_,E>) {
        self.windows.begin_window(window, ctx);
    }

    fn end_window(&mut self, ctx: MessageCtx<'_,E>) {
        self.windows.end_window(ctx);
    }
}