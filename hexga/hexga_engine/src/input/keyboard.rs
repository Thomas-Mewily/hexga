use std::mem;

use super::*;


pub type KeyEvolution = Evolution<ButtonState>;

singleton_access!(
    Keyboard,
    ContextKeyboard,
    { Ctx::try_as_ref().map(|ctx| &ctx.input.keyboard) },
    { Ctx::try_as_mut().map(|ctx| &mut ctx.input.keyboard) }
);

#[derive(Clone, PartialEq, Debug)]
pub struct ContextKeyboard
{
    pub(crate) key : KeyCodeManager,
    pub(crate) key_repeated : KeyCodeManager,
}

impl Default for ContextKeyboard
{
    fn default() -> Self {
        Self 
        { 
            key: KeyCodeManager::new(ButtonRepeat::NotRepeated), 
            key_repeated: KeyCodeManager::new(ButtonRepeat::Repeated) 
        }
    }
}

impl ScopedSuspended for ContextKeyboard
{
    fn suspended(&mut self) {
        self.key.suspended();
        self.key_repeated.suspended();
    }

    fn resumed(&mut self) {
        self.key.resumed();
        self.key_repeated.resumed();
    }
}

impl ScopedUpdate for ContextKeyboard
{
    fn begin_update(&mut self) 
    { 
        self.key.begin_update();
        self.key_repeated.begin_update();
    }
    fn end_update(&mut self) 
    { 
        self.key_repeated.end_update();
        self.key.end_update();
    }
}

impl ContextKeyboard
{
    pub(crate) fn handle_key_event(&mut self, ev: KeyEvent)
    {
        self.key_manager_mut(ev.repeat).handle_event(ev.code, ev.state);
    }

    pub fn keys(&self) -> &KeyCodeManager { &self.key }
    pub fn keys_mut(&mut self) -> &mut KeyCodeManager { &mut self.key }

    pub fn keys_repeated(&self) -> &KeyCodeManager { &self.key_repeated }
    pub fn keys_repeated_mut(&mut self) -> &mut KeyCodeManager { &mut self.key_repeated }

    pub fn key_manager(&self, repeat: ButtonRepeat) -> &KeyCodeManager 
    {
        if repeat.is_repeated() { &self.key_repeated } else { &self.key }
    }
    pub fn key_manager_mut(&mut self, repeat: ButtonRepeat) -> &mut KeyCodeManager 
    {
        if repeat.is_repeated() { &mut self.key_repeated } else { &mut self.key }
    }
}



#[derive(Clone, PartialEq, Debug)]
pub struct KeyCodeManager
{
    pub(crate) repeat  : ButtonRepeat,
    pub(crate) down    : HashSet<KeyCode>,
    pub(crate) old_down: HashSet<KeyCode>,
    pub(crate) pressed : HashSet<KeyCode>,
    pub(crate) released: HashSet<KeyCode>,
}

impl KeyCodeManager
{
    pub fn new(repeat  : ButtonRepeat) -> Self { Self { repeat, down: ___(), old_down: ___(), pressed: ___(), released: ___() }}
}

impl ScopedSuspended for KeyCodeManager
{
    fn suspended(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }

    fn resumed(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }
}

impl ScopedUpdate for KeyCodeManager
{
    fn begin_update(&mut self) 
    {

    }

    fn end_update(&mut self) 
    {
        match self.repeat
        {
            ButtonRepeat::NotRepeated => 
            {
                self.pressed.clear();
                self.released.clear();
                self.old_down.clone_from(&self.down);
            },
            ButtonRepeat::Repeated => 
            {
                std::mem::swap(&mut self.pressed, &mut self.released);
                self.pressed.clear();
                std::mem::swap(&mut self.old_down, &mut self.down);
                self.down.clear();
            },
        }
    }
}

impl KeyCodeManager
{
    pub(crate) fn handle_event(&mut self, code: KeyCode, pressed: ButtonState)
    {
        match pressed
        {
            ButtonState::Up => 
            {
                self.down.remove(&code);
                self.released.insert(code);
            },
            ButtonState::Down =>
            {
                if !self.down.contains(&code)
                {
                    self.pressed.insert(code);
                }
                self.down.insert(code);
            }
        }
    }

    pub fn pressed(&self) -> impl Iterator<Item = KeyCode> { self.pressed.iter().copied() }
    pub fn released(&self) -> impl Iterator<Item = KeyCode> { self.released.iter().copied() }
    pub fn down(&self) -> impl Iterator<Item = KeyCode> { self.down.iter().copied() }

    pub fn is_down(&self, code: KeyCode) -> bool { self.button_state(code).is_down() }
    pub fn is_up(&self, code: KeyCode) -> bool { self.button_state(code).is_up() }

    pub fn was_down(&self, code: KeyCode) -> bool { self.old_button_state(code).is_down() }
    pub fn was_up(&self, code: KeyCode) -> bool { self.old_button_state(code).is_up() }

    pub fn button_state(&self, code: KeyCode) -> ButtonState { self.down.contains(&code).into() }
    pub fn old_button_state(&self, code: KeyCode) -> ButtonState { self.old_down.contains(&code).into() }

    pub fn evolution(&self, code: KeyCode) -> KeyEvolution { KeyEvolution::new(self.button_state(code), self.old_button_state(code)) }
}


impl KeyCode
{
    pub fn evolution(&self) -> KeyEvolution
    {
        KeyBinding::from(*self).evolution()
    }
}
impl IEvolution<ButtonState> for KeyCode
{
    fn value(&self) -> ButtonState { self.evolution().value() }
    fn old_value(&self) -> ButtonState { self.evolution().old_value()  }
    fn evolution(&self) -> Evolution<ButtonState> { self.evolution() }
}

/* 
impl IUsedFlag for KeyCode
{
    fn is_used(&self) -> bool { KeyBinding::from(*self).is_used() }
    fn set_used(&mut self, used: bool) -> &mut Self { KeyBinding::from(*self).set_used(used); self }
}
    */