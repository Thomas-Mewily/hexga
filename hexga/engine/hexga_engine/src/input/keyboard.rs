use std::collections::HashMap;

use super::*;

use prelude::*;

pub mod prelude
{
    pub use hexga_engine_events::{KeyCode,KeyMods,KeyModsFlags,KeyCodeUnknow};
    pub use super::{Keyboard,KeyState,IKeyboard,KeyCodeExtension};
}

#[derive(Debug, Default, Clone)]
pub struct Keyboard<T=Time> where T:Copy+Default
{
    keys : HashMap<KeyCode, InputBool<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyState<T=Time> where T:Copy+Default
{
    keycode: KeyCode,
    value  : InputBool<T>,
}
impl<T> KeyState<T> where T:Copy+Default
{
    pub fn new(keycode: KeyCode, value: InputBool<T>) -> Self { Self { keycode, value }}
    pub fn keycode(&self) -> KeyCode { self.keycode }
    pub fn value(&self) -> InputBool<T> { self.value }
}
impl<T> Deref for KeyState<T> where T:Copy+Default
{
    type Target=KeyCode;
    fn deref(&self) -> &Self::Target { &self.keycode }
}
impl<T> DerefMut for KeyState<T> where T:Copy+Default
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.keycode
    }
}
impl<T> IInputDelta<bool,T> for KeyState<T> where T:Copy+Default
{
    fn cur(&self) -> bool { self.value.cur() }
    fn old(&self) -> bool { self.value.old() }

    fn last_time_changed(&self) -> T { self.value.last_time_changed() }
    fn set(&mut self, cur : bool, time : T) { self.value.set(cur, time); }
}

pub trait IKeyboard<T> where T:Copy+Default
{
    fn key(&self, key : KeyCode) -> InputBool<T>;
    fn keys(&self) -> impl Iterator<Item=KeyState<T>>;

    fn keys_with_change(&self, change : InputButtonChange) -> impl Iterator<Item=KeyState<T>> { self.keys().filter(move |k| self.key(**k).change() == change) }
    fn keys_just_pressed(&self) -> impl Iterator<Item=KeyState<T>> { self.keys_with_change(InputButtonChange::JustPressed) }
    fn keys_just_released(&self) -> impl Iterator<Item=KeyState<T>> { self.keys_with_change(InputButtonChange::JustReleased) }
    fn keys_press(&self) -> impl Iterator<Item=KeyState<T>> { self.keys_with_change(InputButtonChange::Pressed) }
    fn keys_release(&self) -> impl Iterator<Item=KeyState<T>> { self.keys_with_change(InputButtonChange::Released) }
}

impl<T> IKeyboard<T> for Keyboard<T> where T:Copy+Default
{
    fn key(&self, key : KeyCode) -> InputBool<T> { self.keys.get(&key).copied().unwrap_or_default() }

    fn keys(&self) -> impl Iterator<Item=KeyState<T>> {
        self.keys.iter().map(|(k,s)| KeyState::new(*k, *s))
    }
}

pub trait KeyCodeExtension
{
    fn state(self) -> InputBool;
}
impl KeyCodeExtension for KeyCode
{
    fn state(self) -> InputBool { Input.key(self) }
}

impl IInputButton for KeyCode
{
    fn is_pressed(&self) -> bool { self.state().is_pressed() }
    fn was_pressed(&self) -> bool { self.state().was_pressed() }
}