use super::*;


singleton_access!(
    pub Input,
    AppInput,
    { App::try_ref().map(|ctx| &ctx.input) },
    { App::try_as_mut().map(|ctx| &mut ctx.input) }
);


#[derive(PartialEq, Debug)]
pub struct AppInput
{
    pub(crate) keyboard: AppKeyboard,
}

impl AppInput
{
    pub(crate) fn new() -> AppInput { Self { keyboard: ___() } }
    pub fn keyboard(&mut self) -> &mut AppKeyboard { &mut self.keyboard }
}

impl ScopedFlow for AppInput
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.keyboard.begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.keyboard.end_flow(flow);
    }
}