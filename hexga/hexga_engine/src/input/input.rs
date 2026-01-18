use super::*;


//singleton_single_thread_project!(pub Input,AppInput,App,input);

/*
pub struct Input;

impl Input
{
    pub fn keyboard() -> Keyboard { Keyboard }
}
*/

#[derive(PartialEq, Debug)]
pub struct AppInput
{
    pub(crate) keyboard: AppKeyboard,
}

impl AppInput
{
    pub fn new() -> AppInput { Self { keyboard: ___() } }
    pub fn keyboard(&mut self) -> &mut AppKeyboard { &mut self.keyboard }
}

/*
pub trait InputSystem
{
    fn keyboard(&mut self) -> &mut AppKeyboard { &mut self.keyboard }
}

impl AppInput
{
    pub(crate) fn new() -> AppInput { Self { keyboard: ___() } }
    pub fn keyboard(&mut self) -> &mut AppKeyboard { &mut self.keyboard }
}
*/

/*
impl ScopedFlow for AppInput
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.keyboard.begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.keyboard.end_flow(flow);
    }
}
*/