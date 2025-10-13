use std::mem;

use super::*;


singleton_access!(
    pub Keyboard,
    AppKeyboard,
    { App::try_as_ref().map(|ctx| &ctx.input.keyboard) },
    { App::try_as_mut().map(|ctx| &mut ctx.input.keyboard) }
);

#[derive(Clone, PartialEq, Debug)]
pub struct AppKeyboard
{
    pub(crate) key : KeyCodeManager,
    pub(crate) key_repeated : KeyCodeManager,
}

impl Default for AppKeyboard
{
    fn default() -> Self {
        Self
        {
            key: KeyCodeManager::new(ButtonRepeat::NotRepeated),
            key_repeated: KeyCodeManager::new(ButtonRepeat::Repeated)
        }
    }
}

impl ScopedFlow for AppKeyboard
{
    fn begin_flow(&mut self, flow: FlowMessage) {
        self.key.begin_flow(flow);
        self.key_repeated.begin_flow(flow);
    }

    fn end_flow(&mut self, flow: FlowMessage) {
        self.key.end_flow(flow);
        self.key_repeated.end_flow(flow);
    }
}

impl AppKeyboard
{
    pub(crate) fn key_event(&mut self, ev: KeyEvent)
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

impl ScopedFlow for KeyCodeManager
{
    fn begin_flow_paused(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }
    fn begin_flow_resumed(&mut self) {
        self.old_down.clear();
        self.down.clear();
        self.pressed.clear();
        self.released.clear();
    }

    fn end_flow_update(&mut self, dt: DeltaTime) {
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
    pub fn new(repeat : ButtonRepeat) -> Self { Self { repeat, down: ___(), old_down: ___(), pressed: ___(), released: ___() }}

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

    pub fn is_pressed(&self, code: KeyCode) -> bool { self.evolution(code).is_pressed() }
    pub fn is_released(&self, code: KeyCode) -> bool { self.evolution(code).is_released() }

    pub fn button_state(&self, code: KeyCode) -> ButtonState { self.down.contains(&code).into() }
    pub fn old_button_state(&self, code: KeyCode) -> ButtonState { self.old_down.contains(&code).into() }

    pub fn evolution(&self, code: KeyCode) -> ButtonEvolution { Evolution::new(self.button_state(code), self.old_button_state(code)) }
}


impl Evolvable<ButtonState> for KeyCode
{
    fn value(&self) -> ButtonState { Keyboard.keys().button_state(*self) }
    fn old_value(&self) -> ButtonState { Keyboard.keys().old_button_state(*self)  }
    fn evolution(&self) -> ButtonEvolution { Keyboard.keys().evolution(*self) }
}
/*
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
    */

/*
impl IUsedFlag for KeyCode
{
    fn is_used(&self) -> bool { KeyBinding::from(*self).is_used() }
    fn set_used(&mut self, used: bool) -> &mut Self { KeyBinding::from(*self).set_used(used); self }
}
    */