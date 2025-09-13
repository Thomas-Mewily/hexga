use super::*;

#[derive(Default, Clone, PartialEq, Debug)]
pub struct Keyboard
{
    pub(crate) keys : HashMap<KeyCode, UsedFlag<KeyStateEvo>>,
}

pub trait IKeyboard
{
    fn keys(&self) -> impl Iterator<Item = KeyStateEvo>;
    fn key(&self, code: KeyCode) -> KeyStateEvo;
    fn is_key_used(&mut self, code: KeyCode) -> bool;
    fn set_key_used(&mut self, code: KeyCode, used: bool);
}


impl IKeyboard for Keyboard
{
    fn keys(&self) -> impl Iterator<Item = KeyStateEvo> { self.keys.values().map(|v| **v) }
    fn key(&self, code: KeyCode) -> KeyStateEvo { self.keys.get(&code).map(|e| **e).unwrap_or_default() }
    fn is_key_used(&mut self, code: KeyCode) -> bool { self.keys.get(&code).map(|k| k.is_used()).unwrap_or(false) }
    fn set_key_used(&mut self, code: KeyCode, used: bool) { self.keys.get_mut(&code).map(|k| k.set_used(used)); }
}

impl IEvolution<KeyState> for KeyCode
{
    fn value(&self) -> KeyState {
        Input.
    }

    fn old_value(&self) -> KeyState {
        todo!()
    }

    fn last_time_changed(&self) -> TimeOf<f32> {
        todo!()
    }

    fn set_at(&mut self, cur : KeyState, time : TimeOf<f32>) where KeyState:PartialEq {
        todo!()
    }
}

/* 
impl Deref for KeyCode
{
    type Target=KeyState;

    fn deref(&self) -> &Self::Target 
    {
        Input::as_mut().key(*self)
        //Input.keyboard.key(self)
        //Ctx::as_ref().
        //Ctx.input.keyboard.keys.get(self).unwrap()
    }
}
impl DerefMut for KeyCode
{
    fn deref_mut(&mut self) -> &mut Self::Target 
    {
        todo!();
        //Ctx.
        //Ctx.input.keyboard.keys.get_mut(self).unwrap()
    }
}*/

#[derive(Debug, Clone, Copy, PartialEq, Hash, Default)]
pub enum ButtonRepeat
{
    #[default]
    NotRepeated,
    Repeated,
}
pub trait IKeyRepeated
{
    fn is_repeated(&self) -> bool;
    fn is_not_repeated(&self) -> bool;
}
impl IKeyRepeated for ButtonRepeat
{
    fn is_repeated(&self) -> bool { matches!(self, ButtonRepeat::Repeated) }
    fn is_not_repeated(&self) -> bool { matches!(self, ButtonRepeat::NotRepeated) }
}

pub type KeyStateEvo = Evolution<KeyState>;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct KeyState
{
    value  : ButtonState,
    repeat : ButtonRepeat,
}


/* 
pub type KeyValue = ButtonState; 
pub type KeyValueDelta = EvolutionDelta<KeyValue,Time>;



impl IUsedFlag for KeyState
{
    fn is_used(&self) -> bool { self.used }
    fn set_used(&mut self, used: bool) -> &mut Self { self.used = used; self }
}
    */

/* 
impl KeyState
{
    pub fn new(keycode: KeyCode) -> Self { Self { keycode, value: ___(), repeat: ___(), used: false }}
    pub fn with_value(mut self, value: KeyValueDelta) -> Self { self.value = value; self }
    pub fn with_repeat(mut self, repeat: ButtonRepeat) -> Self { self.repeat = repeat; self }
    pub fn with_keycode(mut self, keycode: KeyCode) -> Self { self.keycode = keycode; self }
    pub fn with_used(mut self, used: bool) -> Self { self.used = used; self }
    
    pub fn keycode(&self) -> KeyCode { self.keycode }
    pub fn repeat(&self) -> ButtonRepeat { self.repeat }
    pub fn set_repeated(&mut self, repeat: ButtonRepeat) -> &mut Self { self.repeat = repeat; self}
}
impl Deref for KeyState
{
    type Target=KeyValueDelta;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl DerefMut for KeyState
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}
*/


impl IKeyRepeated for KeyState
{
    fn is_repeated(&self) -> bool { self.repeat.is_repeated() }
    fn is_not_repeated(&self) -> bool { self.repeat.is_not_repeated() }
}