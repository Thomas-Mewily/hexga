use super::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AppEvent
{
    Resumed,
    Paused,
    Update,
    Draw,
    Key(KeyEvent),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEvent
{
    pub code  : KeyCode,
    pub repeat: ButtonRepeat,
    pub state : ButtonState,
    pub char  : Option<char>,
}