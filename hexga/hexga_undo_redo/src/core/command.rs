pub use crate::*;


pub trait CommandStack<A> : ActionStack<A> where A : UndoAction
{
    fn prepare(&mut self);
    fn pop_command(&mut self) -> Option<Command<A>>;

    fn undo(&mut self, ctx : &mut A::Context<'_>) -> bool 
    {
        self.remove_last_command_actions().map(|actions| actions.for_each(|a| a.execute_without_undo_and_forget(ctx))).is_some()
    }
    // Todo : move it to ActionStack ?
    fn remove_last_command_actions(&mut self) -> Option<impl Iterator<Item = A>>;
}

/* 
pub trait UndoCommandStack<A> : CommandStack<A> where A : UndoAction
{
    fn undo_and_dont_forget<'a>(&mut self, ctx : &mut  A::Context<'a>) -> Result<A::Output<'a>, ()>;
    fn undo(&mut self, ctx : &mut A::Context<'_>) -> Result<(), ()>;
}
*/

// Todo : use the trait SequencePush instead of vec ?

/// Group command that required more than one action inside a sequence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Command<A> where A : UndoAction
{
    Action(A),
    Sequence(Vec<A>),
    Nop,
}

impl<A> Command<A> where A : UndoAction
{
    pub const fn new() -> Self { Self::Nop}
}

impl<A> Debug for Command<A> where A : UndoAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Sequence(v) => write!(f, "Sequence{:?}", v),
            Self::Nop =>write!(f, "Nop"),
        }
    }
}

impl<A> Command<A> where A : UndoAction
{
    pub const fn is_action  (&self) -> bool { matches!(self, Self::Action(_)) }
    pub const fn is_sequence(&self) -> bool { matches!(self, Self::Sequence(_)) }
    pub const fn is_nop     (&self) -> bool { matches!(self, Self::Nop) }
}