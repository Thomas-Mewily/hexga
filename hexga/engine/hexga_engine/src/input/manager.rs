use super::*;

declare_context!(Input, InputManager, input);

#[derive(Debug, Clone, Default)]
pub struct InputManager<T=Time> where T:Copy+Default
{
    keyboard : Keyboard<T>,
    modifier : KeyModsFlags,
}

impl<T> InputManager<T> where T:Copy+Default
{
    pub fn new() -> Self { Self { keyboard: ___(), modifier: ___() }}

    pub fn with_keyboard(self, keyboard : Keyboard<T>) -> Self { Self { keyboard, ..self }}
    pub fn set_keyboard(&mut self, keyboard : Keyboard<T>) -> &mut Self { self.keyboard = keyboard; self }
}

impl<T> IKeyboard<T> for InputManager<T> where T:Copy+Default
{
    fn key(&self, key : hexga_engine_events::KeyCode) -> InputBool<T> { self.keyboard.key(key) }
    fn keys(&self) -> impl Iterator<Item=KeyState<T>> { self.keyboard.keys() }
}