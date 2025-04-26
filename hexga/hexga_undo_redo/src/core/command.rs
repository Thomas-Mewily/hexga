pub use crate::*;


pub trait CommandStack<A> : ActionStack<A> where A : UndoAction
{
    fn prepare(&mut self);
}

pub trait UndoCommandStack<A> : CommandStack<A> where A : UndoAction
{
    fn undo_and_dont_forget<'a>(&mut self, ctx : &mut  A::Context<'a>) -> Result<A::Output<'a>, ()>;
    fn undo(&mut self, ctx : &mut A::Context<'_>) -> Result<(), ()>;
}


// Todo : use the trait SequencePush instead of vec ?

/// Group command that required more than one action inside a sequence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum Command<A> where A : UndoAction
{
    Action(A),
    //Sequence(Vec<CommandSequence<A>>),
    Sequence(Vec<A>),
    Nop,
}

impl<A> Command<A> where A : UndoAction
{
    pub const fn new() -> Self { Self::Nop}

    /* 
    pub(crate) fn estimated_capacity_to_command_marker(&self) -> usize
    {
        match self
        {
            Command::Action(_) => 1,
            Command::Sequence(s) => if s.is_empty() 
            { 
                1 // will be Nop
            } 
            else 
            { 
                s.len() + 2 // +2 for begin and end
            }, 
            Command::Nop => 1,
        }
    }*/

    /* 
    #[must_use]
    fn combine(self, other : Self) -> Self
    {
        use CommandSequence::*;
        match (self, other)
        {
            (Nop(0), b) => { b }
            (a, Nop(0)) => { a }
            (Nop(a), Nop(b)) => { debug_assert!(a.checked_add(b).is_some()); Nop(a+b) },
            (a,b) => { Sequence(vec![a,b]) }
        }
    }


*/
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