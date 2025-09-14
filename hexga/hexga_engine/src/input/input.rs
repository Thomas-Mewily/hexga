use super::*;

singleton_access!(
    Input,
    ContextInput,
    { Ctx::try_as_ref().map(|ctx| &ctx.input) },
    { Ctx::try_as_mut().map(|ctx| &mut ctx.input) }
);


#[derive(Default, Clone, PartialEq, Debug)]
pub struct ContextInput
{
    pub(crate) keyboard: ContextKeyboard,
}

impl ScopedSuspended for ContextInput
{
    fn suspended(&mut self) {
        self.keyboard.suspended();
    }

    fn resumed(&mut self) {
        self.keyboard.resumed();
    }
}

impl ScopedUpdate for ContextInput
{
    fn begin_update(&mut self) 
    { 
        self.keyboard.begin_update();
    }
    fn end_update(&mut self) 
    { 
        self.keyboard.end_update();
    }
}

impl ContextInput
{
    //pub(crate) fn handle_window_event(&mut self) 
}
