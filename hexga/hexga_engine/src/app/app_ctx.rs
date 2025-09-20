use super::*;



#[derive(Default, Debug)]
pub struct AppParam
{
    title: String,
}


#[derive(Debug)]
pub struct AppContext
{
    pub(crate) windows: Option<WinitContext>,
    pub(crate) gpu: Option<GpuContext>,

    pub(crate) keyboard: Keyboard,
    pub(crate) param: AppParam,
}


impl AppContext 
{
    pub(crate) fn new(param: AppParam) -> Self { Self { param, windows: ___(), keyboard: ___(), gpu: ___() } }
}


impl<E> ScopedMessage<E> for AppContext where E:IEvent
{
    fn begin_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) {
        ScopedMessage::<E>::begin_flow(&mut self.keyboard, flow, el);
        ScopedMessage::<E>::dispatch_begin_flow(self, flow, el);
    }

    fn begin_flow_resumed(&mut self, el: &EventLoopActive) {
        if self.windows.is_none()
        {
            #[allow(unused_mut)]
            let mut win_attr = WinitWindow::default_attributes().with_title("wgpu winit example");
            
            #[cfg(target_arch = "wasm32")]
            {
                use winit::platform::web::WindowAttributesExtWebSys;
                win_attr = win_attr.with_append(true);
            }

            let window = Arc::new(
                el
                    .create_window(win_attr)
                    .expect("create window err."),
            );
            self.window = Some(window.clone());
            ContextGpu::request(window, self.proxy.clone()).unwrap();
            Ctx.resumed();
        }
    }

    fn end_flow(&mut self, flow: FlowMessage, el: &EventLoopActive) {
        ScopedMessage::<E>::end_flow(&mut self.keyboard, flow, el);
    }

    fn begin_input(&mut self, input: &InputEvent, el: &EventLoopActive) {
        ScopedMessage::<E>::begin_input(&mut self.keyboard, input, el);
    }

    fn end_input(&mut self, el: &EventLoopActive) {
        ScopedMessage::<E>::end_input(&mut self.keyboard, el);
    }
}