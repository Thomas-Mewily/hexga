use super::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub(crate) struct Keyboard
{
    pub(crate) keys : HashMap<KeyCode, KeyStateEntry>,
}

impl Keyboard
{
    pub fn handle_key(&mut self, code: KeyCode, state: ButtonState, repeat: ButtonRepeat)
    {
        let e = self.keys.entry(code).or_default();
        e.key_state.state.update(state);
        e.key_state.repeat = repeat;
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub(crate) struct KeyStateEntry
{
    key_state : KeyState,
    used  : bool,
}
impl IUsedFlag for KeyStateEntry
{
    fn is_used(&self) -> bool { self.used }
    fn set_used(&mut self, used: bool) -> &mut Self { self.used = true;  self }
}


pub trait IKeyboard
{
    fn keys(&self) -> impl Iterator<Item = KeyState>;
    fn key(&self, code: KeyCode) -> KeyState;
    fn is_key_used(&mut self, code: KeyCode) -> bool;
    fn set_key_used(&mut self, code: KeyCode, used: bool);
}
impl IKeyboard for Keyboard
{
    fn keys(&self) -> impl Iterator<Item = KeyState> { self.keys.values().map(|v| v.key_state) }
    fn key(&self, code: KeyCode) -> KeyState { self.keys.get(&code).map(|v| v.key_state).unwrap_or_default() }
    fn is_key_used(&mut self, code: KeyCode) -> bool { self.keys.get(&code).map(|k| k.is_used()).unwrap_or(false) }
    fn set_key_used(&mut self, code: KeyCode, used: bool) { self.keys.get_mut(&code).map(|k| k.set_used(used)); }
}


#[derive(Clone, Copy, PartialEq, PartialOrd, Debug, Default)]
pub struct KeyState
{
    pub state : Evolution<ButtonState>,
    pub repeat: ButtonRepeat,
}
impl IButtonRepeat for KeyState
{
    fn is_repeated(&self) -> bool { self.repeat.is_repeated() }
    fn is_not_repeated(&self) -> bool { self.repeat.is_not_repeated() }
}
impl IEvolution<ButtonState> for KeyState
{
    fn value(&self) -> ButtonState { self.state.value() }
    fn old_value(&self) -> ButtonState { self.state.old_value() }
    fn last_time_change(&self) -> Time { self.state.last_time_change() }
}



impl KeyCode
{
    pub fn state(self) -> KeyState { Input.key(self) }
}
impl IUsedFlag for KeyCode
{
    fn is_used(&self) -> bool { Input.is_key_used(*self) }
    fn set_used(&mut self, used: bool) -> &mut Self { Input.set_key_used(*self, used); self }
}
impl IEvolution<ButtonState> for KeyCode
{
    fn value(&self) -> ButtonState { self.state().value() }
    fn old_value(&self) -> ButtonState { self.state().old_value() }
    fn last_time_change(&self) -> Time { self.state().last_time_change() }
}
impl IButtonRepeat for KeyCode
{
    fn is_repeated(&self) -> bool { self.state().is_repeated() }
}


