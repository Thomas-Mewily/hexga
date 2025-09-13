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
    fn keys(&self) -> impl Iterator<Item = KeyState> { self.keyboard.keys() }
    fn key(&self, code: KeyCode) -> KeyState { self.keyboard.key(code) }
    fn is_key_used(&mut self, code: KeyCode) -> bool { self.keyboard.is_key_used(code) }
    fn set_key_used(&mut self, code: KeyCode, used: bool) { self.keyboard.set_key_used(code, used); }
}