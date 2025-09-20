use super::*;

singleton_access!(
    pub Input,
    AppInput,
    { App::try_as_ref().map(|app| &app.input) },
    { App::try_as_mut().map(|app| &mut app.input) }
);


#[derive(Default, Clone, PartialEq, Debug)]
pub struct AppInput
{
    pub(crate) keyboard: AppKeyboard,
}
impl AppInput
{
    pub fn keyboard(&mut self) -> &mut AppKeyboard { &mut self.keyboard }
}

impl<E> ScopedMessage<E> for AppInput where E:IEvent
{
    fn begin_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) {
        self.keyboard.begin_flow(flow, ctx);
    }
    fn end_flow(&mut self, flow: FlowMessage, ctx: MessageCtx<'_,E>) {
        self.keyboard.end_flow(flow, ctx);
    }

    fn begin_input(&mut self, input: &InputEvent, ctx: MessageCtx<'_,E>) {
        self.keyboard.begin_input(input, ctx);
    }
    fn end_input(&mut self, ctx: MessageCtx<'_,E>) {
        self.keyboard.end_input(ctx);
    }
}