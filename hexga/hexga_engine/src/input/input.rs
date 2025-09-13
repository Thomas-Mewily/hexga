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
    pub(crate) keyboard: Keyboard,
}

impl ContextInput
{
    //pub(crate) fn handle_window_event(&mut self) 
}


impl IKeyboard for ContextInput
{
    fn keys(&self) -> impl Iterator<Item = KeyStateEvo> {
        self.keyboard.keys
    }

    fn key(&self, code: KeyCode) -> KeyStateEvo {
        todo!()
    }

    fn is_key_used(&mut self, code: KeyCode) -> bool {
        todo!()
    }

    fn set_key_used(&mut self, code: KeyCode, used: bool) {
        todo!()
    }
}