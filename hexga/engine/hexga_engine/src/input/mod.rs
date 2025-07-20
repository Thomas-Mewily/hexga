use super::*;

mod delta;
use delta::*;

pub mod keyboard;
use keyboard::*;


declare_context!(Input, CtxInput, input);

#[derive(Debug, Clone, Default)]
pub struct CtxInput<T=Time> where T:Copy+Default
{
    keyboard : Keyboard<T>,
}

impl<T> CtxInput<T> where T:Copy+Default
{
    pub fn new() -> Self { Self { keyboard: ___() }}

    pub fn with_keyboard(self, keyboard : Keyboard<T>) -> Self { Self { keyboard, ..self }}
    pub fn set_keyboard(&mut self, keyboard : Keyboard<T>) -> &mut Self { self.keyboard = keyboard; self }
}

impl<T> IKeyboard<T> for CtxInput<T> where T:Copy+Default
{
    fn key(&self, key : hexga_engine_events::KeyCode) -> InputBool<T> { self.keyboard.key(key) }
    fn keys(&self) -> impl Iterator<Item=KeyState<T>> { self.keyboard.keys() }
}