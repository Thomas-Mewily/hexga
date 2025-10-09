use super::*;

#[derive(Debug)]
pub struct AppCore
{
    /*
    pub(crate) gpu: Option<AppGpu>,
    */
    pub(crate) clipboard : AppClipboard,
    pub(crate) input : AppInput,
    pub(crate) window: AppWindow,
    pub(crate) perf : AppPerf,
    pub(crate) param: AppParam,
}
impl AppCore
{
    pub fn clipboard(&self) -> &AppClipboard { &self.clipboard }
    pub fn clipboard_mut(&mut self) -> &mut AppClipboard { &mut self.clipboard }

    pub fn input(&self) -> &AppInput { &self.input }
    pub fn input_mut(&mut self) -> &mut AppInput { &mut self.input }

    pub fn window(&self) -> &AppWindow { &self.window }
    pub fn window_mut(&mut self) -> &mut AppWindow { &mut self.window }

    pub fn perf(&self) -> &AppPerf { &self.perf }
    pub fn perf_mut(&mut self) -> &mut AppPerf { &mut self.perf }

    pub fn param(&self) -> &AppParam { &self.param }
    //pub fn param_mut(&mut self) -> &mut AppParam { &mut self.param }
}
impl AppCore
{
    pub(crate) fn new(param: AppParam) -> Self
    {
        Self
        {
            param,
            input: AppInput::new(),
            window: AppWindow::new(),
            clipboard: AppClipboard::new(),
            perf: AppPerf::new()
        }
    }
}

impl ScopedFlow for AppCore
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.perf.begin_flow(flow);
        self.input.begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.input.end_flow(flow);
        self.perf.end_flow(flow);
    }
}

singleton_thread_local!(pub App,AppCore,CONTEXT_APP);

impl SingletonInit for App
{
    fn replace(instance: Option<<Self as SingletonRef>::Target>) {
        match instance
        {
            Some(ctx) =>
            {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    use std::io::Write;

                    std::panic::set_hook(Box::new(|info| {
                        App::destroy();
                        eprintln!("panic occurred: {info}");
                    }));


                    let _res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                        App::destroy();
                    }));
                }
                #[cfg(target_arch = "wasm32")]
                {
                    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
                    console_log::init_with_level(::log::Level::Debug).expect("Couldn't initialize logger");
                }
                CONTEXT_APP.replace(Some(ctx));
                // The Gpu is initialized in a special async way...
            },
            None =>
            {
                CONTEXT_APP.replace(None);
                //Gpu::destroy();
            },
        }
    }
}


#[derive(Default, Debug, Clone)]
pub struct AppParam
{
    pub title: String,
}