use super::*;


#[derive(Default, Clone, PartialEq, Debug)]
pub struct Keyboard
{
    pub(crate) keys : HashMap<KeyCode, KeyState>,
}

pub trait IKeyboard
{
    //fn key(&self, code: KeyCode) -> KeyState;
    //fn keys(&self) -> impl Iterator<Item = KeyState>;

    fn keys_mut(&mut self) -> impl Iterator<Item = &mut KeyState>;
    fn key_mut(&mut self, code: KeyCode) -> &mut KeyState;
    fn key(&mut self, code: KeyCode) -> &KeyState;
}

impl IKeyboard for Keyboard
{
    fn keys_mut(&mut self) -> impl Iterator<Item = &mut KeyState> { self.keys.values_mut() }
    fn key_mut(&mut self, code: KeyCode) -> &mut KeyState 
    {
        self.keys.entry(code).or_insert_with(|| KeyState::new(code, ___()))
    }
    fn key(&mut self, code: KeyCode) -> &KeyState 
    {
        self.keys.entry(code).or_insert_with(|| KeyState::new(code, ___()))
    }
}

impl Deref for KeyCode
{
    type Target=KeyState;

    fn deref(&self) -> &Self::Target 
    {
        Ctx.input.keyboard.keys.get(self).unwrap()
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState
{
    keycode: KeyCode,
    value  : KeyValue,
}
impl Evolution<bool,Time> for KeyState
{
    fn value(&self) -> bool { self.value.value() }
    fn old_value(&self) -> bool { self.value.old_value() }
    fn last_time_changed(&self) -> Time { self.value.last_time_changed() }
    fn set_at(&mut self, cur : bool, time : Time) { self.value.set_at(cur, time) }
}

pub type KeyValue = IdentityDirty<EvolutionDelta<bool,Time>>;

impl KeyState
{
    pub fn new(keycode: KeyCode, value: KeyValue) -> Self { Self { keycode, value }}
    pub fn keycode(&self) -> KeyCode { self.keycode }
}
impl Deref for KeyState
{
    type Target=KeyValue;
    fn deref(&self) -> &Self::Target { &self.value }
}
impl DerefMut for KeyState
{
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.value }
}