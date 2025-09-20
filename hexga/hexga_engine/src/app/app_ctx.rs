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
    fn begin_flow(&mut self, flow: FlowMessage) {
        ScopedMessage::<E>::begin_flow(&mut self.keyboard, flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        ScopedMessage::<E>::end_flow(&mut self.keyboard, flow);
    }

    fn begin_input(&mut self, input: &InputEvent) {
        ScopedMessage::<E>::begin_input(&mut self.keyboard, input);
    }

    fn end_input(&mut self) {
        ScopedMessage::<E>::end_input(&mut self.keyboard);
    }
}