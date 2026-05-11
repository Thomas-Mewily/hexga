use hexga_event_loop::event_loop::EventLoopResult;
use super::*;

pub type AppResult = EventLoopResult;
pub type AppError = EventLoopError;

pub trait AppInit<A>: Fn() -> A + Async {}
impl<S, A> AppInit<A> for S where S: Fn() -> A + Async {}

pub(crate) struct AppRunner<F,A>
    where 
    F: AppInit<A>,
    A: App
{
    init : F,
    app : Option<A>,
    param: AppParam,
}



impl<F,A> AppRunner<F,A>
    where 
    F: AppInit<A>,
    A: App
{
    fn exit(&mut self)
    {
        WINDOW.reset();
        self.app = None;
    }
}


impl<F,A> Drop for AppRunner<F,A>
    where 
    F: AppInit<A>,
    A: App
{
    fn drop(&mut self) 
    {
        self.exit();
    }
}

impl<F,A> PlatformEventHandler for AppRunner<F,A>
    where 
    F: AppInit<A>,
    A: App
{
    fn update(&mut self, dt: Duration, event_loop: &mut EventLoop) 
    {
        match &mut self.app
        {
            Some(app) => app.update(dt, &mut ()),
            None => {},
        } 
    }

    fn draw(&mut self, event_loop: &mut EventLoop) 
    {
        match &mut self.app
        {
            Some(app) => app.draw(1., &mut ()),
            None => {},
        }
    }

    fn event(&mut self, ev: PlatformEvent, event_loop: &mut EventLoop) -> Option<PlatformEvent> 
    {
        match &mut self.app
        {
            Some(app) => app.event(ev, &mut ()),
            None => Some(ev),
        } 
    }

    fn resumed(&mut self, event_loop: &mut EventLoop) 
    {
        WINDOW.init_from_fn(||
            {
                let main_window = event_loop.create_window(self.param.window.clone()).expect("failed to create main window");
                main_window

                //if Gpu::is_init()
            }
        ).map_err(|_|()).expect("can't init the main window");
    }

    fn exit(&mut self, event_loop: &mut EventLoop) {
        self.exit();
    }
}

pub trait AppRun : Sized
{
    fn run(self) -> AppResult { self.run_with_param(___()) }
    fn run_with_param(self, param: AppParam) -> AppResult;
}

impl<F,A> AppRun for F
    where 
    F: AppInit<A> + Fn() -> A,
    A: App
{
    fn run_with_param(self, param: AppParam) -> AppResult 
    {

        let event_loop_param = param.event_loop.clone();
        AppRunner
        {
            app: None,
            init: self,
            param,
        }.run_event_loop_with_param(event_loop_param)
    }
}