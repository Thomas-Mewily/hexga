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
    fn keys(&mut self) -> impl Iterator<Item = &KeyState> { self.keyboard.keys() }
    fn keys_mut(&mut self) -> impl Iterator<Item = &mut KeyState> { self.keyboard.keys_mut() }

    fn key_mut(&mut self, code: KeyCode) -> &mut KeyState { self.keyboard.key_mut(code) }
    fn key(&mut self, code: KeyCode) -> &KeyState { self.keyboard.key(code) }
}