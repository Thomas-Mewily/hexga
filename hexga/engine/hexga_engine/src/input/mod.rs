use super::*;

mod delta;
use delta::*;

pub mod keyboard;
use keyboard::*;

#[derive(Debug, Clone, Default)]
pub struct Input<T=Time> where T:Copy+Default
{
    keyboard : Keyboard<T>,
}

impl<T> Input<T> where T:Copy+Default
{
    pub fn new() -> Self { Self { keyboard: ___() }}

    pub fn with_keyboard(self, keyboard : Keyboard<T>) -> Self { Self { keyboard, ..self }}
    pub fn set_keyboard(&mut self, keyboard : Keyboard<T>) -> &mut Self { self.keyboard = keyboard; self }
}

impl<T> IKeyboard<T> for Input<T> where T:Copy+Default
{
    fn key(&self, key : hexga_engine_events::KeyCode) -> InputBool<T> {
        self.keyboard.
    }

    fn keys(&self) -> impl Iterator<Item=KeyState<T>> {
        todo!()
    }
}