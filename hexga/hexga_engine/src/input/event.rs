use super::*;


#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent
{
    Key(KeyEvent),
}
impl From<KeyEvent> for InputEvent
{
    fn from(key: KeyEvent) -> Self {
        Self::Key(key)
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    pub code  : KeyCode,
    pub repeat: ButtonRepeat,
    pub state : ButtonState,
    pub char  : Option<char>,
}
impl Has<ButtonRepeat> for KeyEvent { fn retrieve(&self) -> ButtonRepeat { self.repeat } }
impl Has<ButtonState> for KeyEvent { fn retrieve(&self) -> ButtonState { self.state } }


impl From<winit::event::KeyEvent> for KeyEvent
{
    fn from(event: winit::event::KeyEvent) -> Self
    {
        let char: Option<char> = match &event.logical_key {
            winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };
        Self { code: event.physical_key.into(), repeat: if event.repeat { ButtonRepeat::Repeated } else { ButtonRepeat::NotRepeated }, state: if event.state.is_pressed() { ButtonState::Down } else { ButtonState::Up }, char }
    }
}