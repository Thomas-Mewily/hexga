use std::marker::PhantomData;

use super::*;


pub trait AppRun<E> : Sized where E:IEvent
{
    fn run(self) -> Result<(), ()> { self.run_with_param(___()) }
    fn run_with_param(self, param: AppParam) -> Result<(), ()>;
}
impl<A,E> AppRun<E> for A where A:Application<E>, E:IEvent
{
    fn run_with_param(self, param: AppParam) -> Result<(), ()> 
    {
        log::init_logger();


        assert!(App::is_not_init(), "Can't run two app at the same time, App is a singleton");
        App::replace(Some(AppContext::new(param)));

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        let proxy = event_loop.create_proxy();

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self, proxy);

        #[cfg(not(target_arch = "wasm32"))]
        {
            let r = event_loop.run_app(&mut runner);
            r.ok_or_void()
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}


/// Run the application and init the App
pub(crate) struct AppRunner<A,E> where A:Application<E>, E:IEvent
{
    app: A,
    proxy : EventLoopProxy<E>,
}
impl<A,E> AppRunner<A,E> where A:Application<E>, E:IEvent
{
    pub const fn new(app: A, proxy: EventLoopProxy<E> ) -> Self { Self { app, proxy } }
}

impl<A,E> AppRunner<A,E> where A:Application<E>, E:IEvent
{
    fn message(&mut self, msg: AppMessage<E>, el: &EventLoopActive)
    {
        match msg
        {
            AppMessage::Flow(flow) => self.flow(flow, el),
            AppMessage::Event(ev) => self.event(ev, el),
        }
    }

    fn flow(&mut self, flow: FlowMessage, el: &EventLoopActive) 
    {
        ScopedMessage::<E>::scoped_flow(App::as_mut(),flow, |f| self.app.handle_message(AppMessage::Flow(f)), el);
    }

    fn event(&mut self, ev: impl Into<AppEvent<E>>, el: &EventLoopActive)
    {
        match ev.into()
        {
            AppEvent::Input(input) => ScopedMessage::<E>::scoped_input(App::as_mut(), input, |i| self.app.handle_message(AppMessage::Event(AppEvent::Input(i))), el),
            AppEvent::Custom(custom) => ScopedMessage::<E>::scoped_custom(App::as_mut(), custom, |c| self.app.handle_message(AppMessage::Event(AppEvent::Custom(c))), el), 
        }
    }
}

impl<A,E> winit::application::ApplicationHandler<AppInternalEvent<E>> for AppRunner<A,E> where A:Application<E>, E:IEvent
{
    fn resumed(&mut self, el: &EventLoopActive) 
    {
        self.flow(FlowMessage::Resumed, el);
    }

    fn suspended(&mut self, el: &EventLoopActive) 
    {
        self.flow(FlowMessage::Paused, el);
    }

    fn about_to_wait(&mut self, el: &EventLoopActive) 
    {
        self.flow(FlowMessage::Update, el);
    }

    fn window_event
    (
        &mut self,
        el: &EventLoopActive,
        _window_id: WinitWindowID,
        event: winit::event::WindowEvent,
    ) 
    {
        if !Gpu::is_init() { return; }
        
        match event 
        {
            WinitWindowEvent::CloseRequested =>  { el.exit(); }
            WinitWindowEvent::Resized(new_size) => {
                if let Some(window) = App.windows.as_ref()
                {
                    Gpu.resize([new_size.width as _, new_size.height as _].into());
                    window.request_redraw();
                }
            }
            WinitWindowEvent::RedrawRequested => self.flow(FlowMessage::Draw, el),
            WinitWindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } => 
            {
                let code = KeyCode::from(event.physical_key);
                let repeat = if event.repeat { ButtonRepeat::Repeated } else { ButtonRepeat::NotRepeated };
                let state = if event.state.is_pressed() { ButtonState::Down } else { ButtonState::Up };
                
                // TODO: enable it is a debug flag in the config
                if code == KeyCode::Escape
                {
                    el.exit();
                }

                let char: Option<char> = match &event.logical_key {
                    winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
                    _ => None,
                };
                let key = KeyEvent{ code, repeat, state, char };

                self.event(key, el);
            }
            _ => (),
        }
    }


    fn user_event(&mut self, el: &EventLoopActive, event: AppInternalEvent<E>) {
        match event
        {
            AppInternalEvent::Event(event) => self.event(event, el),
            AppInternalEvent::Gpu(gpu) =>
            {
                Gpu::replace(Some(gpu.unwrap()));
                App.windows.as_ref().map(|w| w.request_redraw());
            }
        }
    }
}


/* 

pub struct AppRunner<A> where A:Application
{
    app : A,
}
*/

/* 
impl<A> AppRun for A where A:Application
{
    fn run_with_param(self, _param: AppParam) -> Result<(), ()>
    {
        log::init_logger();

        let event_loop = EventLoop::with_user_event().build().ok_or_void()?;
        let proxy = event_loop.create_proxy();

        #[allow(unused_mut)]
        let mut runner = AppRunner::new(self, Ctx::new(proxy));

        #[cfg(not(target_arch = "wasm32"))]
        {
            let r = event_loop.run_app(&mut runner);
            r.ok_or_void()
        }

        #[cfg(target_arch = "wasm32")]
        {
            async move { let _ = event_loop.run_app(&mut runner); }.spawn();
            Ok(())
        }
    }
}

pub(crate) struct AppRunner<A> where A:Application
{
    app : A,
    ctx : Ctx,
}
impl<A> AppRunner<A> where A:Application
{
    pub fn new(app : A, ctx : Ctx) -> Self 
    {
        Self { app, ctx }
    }


    pub(crate) fn handle_event(&mut self, ev: AppEvent<A::CustomEvent>)
    {
        self.app.handle_event(ev, &mut self.ctx);
    }

    pub(crate) fn update(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Update));
    }

    pub(crate) fn draw(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Draw));
    }

    pub(crate) fn exit(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Exit));
    }

    pub(crate) fn resumed(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Resumed));
    }
    
    pub(crate) fn paused(&mut self)
    {
        self.handle_event(AppEvent::Flow(FlowEvent::Paused));
    }
}


impl<A> winit::application::ApplicationHandler<CtxEvent> for AppRunner<A> where A:Application
{
    fn resumed(&mut self, _event_loop: &EventLoopActive) 
    {
        self.resumed();
    }

    fn suspended(&mut self, _event_loop: &EventLoopActive) {
        self.paused();
    }

    fn window_event(
        &mut self,
        event_loop: &EventLoopActive,
        _window_id: WinitWindowID,
        event: WinitWindowEvent,
    ) 
    {
        match event
        {
            WinitWindowEvent::KeyboardInput { device_id: _, event, is_synthetic: _ } =>
            {
                let k = KeyEvent::from(event);
                self.ctx.keyboard.handle_key(k);
                self.app.handle_event(k.into(), &mut self.ctx);
            }
            WinitWindowEvent::CloseRequested => { event_loop.exit(); },
            WinitWindowEvent::RedrawRequested => { self.draw(); }
            _ => {}
        }
    }

    fn user_event(&mut self, _event_loop: &EventLoopActive, event: CtxEvent) 
    {
        match event
        {
            CtxEvent::Gpu(gpu) => { self.ctx.gpu = Some(gpu.expect("Failed to connect to the gpu")); },
        }
    }

    fn about_to_wait(&mut self, _event_loop: &EventLoopActive) 
    {
        self.update();
    }

    fn exiting(&mut self, _event_loop: &EventLoopActive) 
    {
        self.exit();
    }
}
    */