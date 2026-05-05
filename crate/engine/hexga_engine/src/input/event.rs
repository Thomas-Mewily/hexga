use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum InputEvent
{
    Key(KeyEvent),
}
impl From<KeyEvent> for InputEvent
{
    fn from(key: KeyEvent) -> Self { Self::Key(key) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    pub code: KeyCode,
    pub repeat: ButtonRepeat,
    pub state: ButtonState,
    pub char: Option<char>,
}
impl ButtonRepeatExtension for KeyEvent
{
    fn is_repeated(&self) -> bool {
        self.repeat.is_repeated()
    }

    fn is_not_repeated(&self) -> bool {
        self.repeat.is_not_repeated() }
}
impl ButtonStateExtension for KeyEvent
{
    fn is_up(&self) -> bool {
        self.state.is_up()
    }

    fn is_down(&self) -> bool {
        self.state.is_down()
    }
}

impl From<winit::event::KeyEvent> for KeyEvent
{
    fn from(event: winit::event::KeyEvent) -> Self
    {
        let char: Option<char> = match &event.logical_key
        {
            winit::keyboard::Key::Character(s) if s.chars().count() == 1 => s.chars().next(),
            _ => None,
        };
        Self {
            code: event.physical_key.into(),
            repeat: if event.repeat
            {
                ButtonRepeat::Repeated
            }
            else
            {
                ButtonRepeat::NotRepeated
            },
            state: if event.state.is_pressed()
            {
                ButtonState::Down
            }
            else
            {
                ButtonState::Up
            },
            char,
        }
    }
}
