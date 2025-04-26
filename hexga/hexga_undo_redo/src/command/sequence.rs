pub use crate::*;


// Todo : use the trait SequencePush instead of vec ?

/// Group command that required more than one action inside a sequence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum CommandSequence<A> where A : UndoAction
{
    Action(A),
    Sequence(Vec<CommandSequence<A>>),
    Nop,
}
impl<A> Debug for CommandSequence<A> where A : UndoAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Sequence(v) => write!(f, "Sequence{:?}", v),
            Self::Nop => write!(f, "Nop"),
        }
    }
}

impl<A> CommandSequence<A> where A : UndoAction
{
    pub const fn is_action  (&self) -> bool { matches!(self, Self::Action(_)) }
    pub const fn is_sequence(&self) -> bool { matches!(self, Self::Sequence(_)) }
    pub const fn is_nop     (&self) -> bool { matches!(self, Self::Nop) }
}

pub type CommandStackSequenceFlatten<A> = CommandStackSequence<A>;

/// Multiple nested begin will be flatten
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStackSequence<A> where A : UndoAction
{
    actions : Vec<CommandSequence<A>>,
    nb_action   : usize,
    begin_level : usize,
}

impl<A> ActionStack<A> for CommandStackSequence<A> where A : UndoAction
{
    fn push<F>(&mut self, f : F) where F : FnOnce() -> A {
        assert!(self.begin_level.is_non_zero(), "Forget to call CommandStackSequence::begin()");
        assert!(self.nb_action.is_non_max_value(), "To many action. How you didn't run out of memory before ?!");
        self.nb_action += 1;
        self.actions.push(CommandSequence::Action(f()));
    }
} 

impl<A> CommandStack<A> for CommandStackSequence<A> where A : UndoAction
{
    fn begin(&mut self) 
    {
        assert!(self.begin_level.is_non_max_value(), "To many nested CommandStackSequence::begin()");
        self.begin_level += 1;
    }

    fn end(&mut self) 
    {
        assert!(self.begin_level.is_non_zero(), "Forget to call CommandStackSequence::begin()");
        self.begin_level -= 1;

        if self.is_active() { return; } // this one is flatten

        match self.nb_action
        {
            0 => self.actions.push(CommandSequence::Nop),
            1 => {},
            n => 
            {
                let mut v = Vec::with_capacity(n);
                for _ in 0..n
                {
                    let Some(action) = self.actions.pop() else { unreachable!(); };
                    v.push(action);
                }
                self.actions.push(CommandSequence::Sequence(v));
            }
        }

        self.nb_action = 0;
    }
}

impl<A> CommandStackSequenceFlatten<A> where A : UndoAction
{
    pub const fn new() -> Self { Self { actions: Vec::new(), nb_action: 0, begin_level: 0 } }
    pub fn with_capacity(capacity: usize) -> Self { Self { actions: Vec::with_capacity(capacity), nb_action: 0, begin_level: 0 } }
    fn len(&self) -> usize { self.actions.len() }

    pub const fn begin_level(&self) -> usize { self.begin_level }
    pub fn is_active(&self) -> bool { self.begin_level().is_non_zero() }
    pub fn is_not_active(&self) -> bool { !self.is_active() }

    /// Only return the actions when not active
    pub fn actions(&self) -> Option<&[CommandSequence<A>]> { self.is_not_active().then_some(&self.actions) }

    /// Only return the actions when not active
    pub fn into_actions(self) -> Option<Vec<CommandSequence<A>>> { self.is_not_active().then_some(self.actions) }
}

impl<A> Length for CommandStackSequenceFlatten<A> where A : UndoAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A> Capacity for CommandStackSequenceFlatten<A> where A : UndoAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.actions.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.actions.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.actions.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve_exact(additional) }
}


/* 
pub struct CommandStackSequenceNonFlatten<A> where A : UndoAction
{
    actions : Vec<CommandSequence<A>>,
    nb_action : Vec<usize>
}
*/