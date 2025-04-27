pub use crate::*;


pub trait CommandStack<A> : ActionStack<A> where A : UndoableAction
{
    fn prepare(&mut self);
    fn pop_command(&mut self) -> Option<Command<A>>;
    fn take_last_command_actions(&mut self) -> Option<impl Iterator<Item = A>>;
}


// Todo : use the trait SequencePush instead of vec ?

/// Group command that required more than one action inside a sequence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Command<A> where A : UndoableAction
{
    Action(A),
    Sequence(Vec<A>),
    Nop,
}

impl<A> Command<A> where A : UndoableAction
{
    pub const fn new() -> Self { Self::Nop}
}

impl<A> Debug for Command<A> where A : UndoableAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Sequence(v) => write!(f, "Sequence{:?}", v),
            Self::Nop =>write!(f, "Nop"),
        }
    }
}

impl<A> Command<A> where A : UndoableAction
{
    pub const fn is_action  (&self) -> bool { matches!(self, Self::Action(_)) }
    pub const fn is_sequence(&self) -> bool { matches!(self, Self::Sequence(_)) }
    pub fn is_nop     (&self) -> bool 
    { 
        match self
        {
            Command::Action(_) => false,
            Command::Sequence(seq) => seq.is_empty(),
            Command::Nop => true,
        } 
    }
}