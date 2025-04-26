pub use crate::*;


// Todo : use the trait SequencePush instead of vec ?

/// Group command that required more than one action inside a sequence
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum CommandSequence<A> where A : UndoAction
{
    Action(A),
    Sequence(Vec<CommandSequence<A>>),
    /// Nop(0) is a special action that count as 0 length action
    Nop(usize),
}
impl<A> CommandSequence<A> where A : UndoAction
{
    pub const fn new() -> Self { Self::Nop(0) }

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

    fn estimated_capacity_to_command_marker(&self) -> usize
    {
        match self
        {
            CommandSequence::Action(_) => 1,
            CommandSequence::Sequence(s) => if s.is_empty() 
            { 
                1 // will be Nop
            } else 
            { 
                s.len() + 2 // +2 for begin and end
            }, 
            CommandSequence::Nop(_) => 1,
        }
    }

    fn extend_command_marker(self, v : &mut CommandStackMarker<A>)
    {
        match self
        {
            CommandSequence::Action(a) => v.push(CommandMarker::Action(a)),
            CommandSequence::Sequence(seq) => 
            {
                if seq.is_empty() 
                {
                    v.push(CommandMarker::Nop(1));
                    return;
                }

                let is_non_one = seq.len().is_non_one();

                if is_non_one { v.push(CommandMarker::Begin(1)); }

                for value in seq { value.extend_command_marker(v); }

                if is_non_one { v.push(CommandMarker::End(1)); }
            }
            CommandSequence::Nop(n) => v.push(CommandMarker::Nop(n)),
        }
    }
}


impl<A> From<CommandSequence<A>> for CommandStackMarker<A> where A : UndoAction
{
    fn from(value: CommandSequence<A>) -> Self 
    {
        let capacity = value.estimated_capacity_to_command_marker();
        let mut seq = CommandStackMarker::with_capacity(capacity);
        value.extend_command_marker(&mut seq);
        seq
    }
}
impl<A> TryFrom<Vec<CommandMarker<A>>> for CommandSequence<A> where A : UndoAction
{
    type Error=();

    fn try_from(value: Vec<CommandMarker<A>>) -> Result<Self, Self::Error> 
    {
        let mut cmd_seq = CommandSequence::new();
        let mut seq = CommandSequence::new();
        let mut nb_begin : usize = 0;

        for marker in value.into_iter().rev()
        {
            match marker 
            {
                CommandMarker::Begin(n) => 
                {
                    match nb_begin.checked_add(n)
                    {
                        Some(v) => nb_begin = v,
                        None => return Err(()), // Too many nested begin
                    }
                },
                CommandMarker::End(n) => 
                {
                    if n == 0 { continue };
                    match nb_begin.checked_sub(n)
                    {
                        Some(v) => nb_begin = v,
                        None => return Err(()), // Too many end invalid input
                    }

                    if nb_begin.is_zero()
                    {
                        cmd_seq = cmd_seq.combine(seq);
                        seq = CommandSequence::new();
                    }
                }
                CommandMarker::Action(action) => if nb_begin.is_zero()
                {
                    cmd_seq = cmd_seq.combine(CommandSequence::Action(action));
                }else
                {
                    seq = seq.combine(CommandSequence::Action(action));
                }
                CommandMarker::Nop(n) => if nb_begin.is_zero()
                {
                    cmd_seq = cmd_seq.combine(CommandSequence::Nop(n));
                }else
                {
                    seq = seq.combine(CommandSequence::Nop(n));
                }
            }
        }
        Ok(cmd_seq.combine(seq))
    }
}

impl<A> Debug for CommandSequence<A> where A : UndoAction + Debug
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Action(v) => write!(f, "{:?}", v),
            Self::Sequence(v) => write!(f, "Sequence{:?}", v),
            Self::Nop(nb) => match nb
            {
                1 => write!(f, "Nop"),
                n => write!(f, "Nop x{}", n),
            },
        }
    }
}

impl<A> CommandSequence<A> where A : UndoAction
{
    pub const fn is_action  (&self) -> bool { matches!(self, Self::Action(_)) }
    pub const fn is_sequence(&self) -> bool { matches!(self, Self::Sequence(_)) }
    pub const fn is_nop     (&self) -> bool { matches!(self, Self::Nop(_)) }

    pub const fn is_nop_zero(&self) -> bool { matches!(self, Self::Nop(0)) }
}

pub type CommandStackSequenceFlatten<A> = CommandStackSequence<A>;

/// Multiple nested begin will be flatten
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandStackSequence<A> where A : UndoAction
{
    // Todo : use generic sequence ? vec, vecdequeu...
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
            0 => 
            {
                if let Some(CommandSequence::Nop(n)) = self.actions.last_mut()
                {
                    debug_assert!(n.is_max_value(), "That a lot of nop");
                    n.increase();
                }else
                {
                    self.actions.push(CommandSequence::Nop(1));
                }
            },
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

impl<A> CommandStackSequence<A> where A : UndoAction
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

impl<A> Length for CommandStackSequence<A> where A : UndoAction
{
    fn len(&self) -> usize { self.len() }
}
impl<A> Capacity for CommandStackSequence<A> where A : UndoAction
{
    type Param = ();

    fn capacity(&self) -> usize { self.actions.capacity() }

    fn with_capacity_and_param(capacity: usize, _ : Self::Param) -> Self { Self::with_capacity(capacity) }

    fn reserve(&mut self, additional: usize) { self.actions.reserve(additional); }
    fn reserve_exact(&mut self, additional: usize) { self.actions.reserve_exact(additional); }

    fn try_reserve(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve(additional) }
    fn try_reserve_exact(&mut self, additional: usize) -> Result<(), std::collections::TryReserveError> { self.actions.try_reserve_exact(additional) }
}


impl<A> UndoCommandStack<A> for CommandStackSequence<A> where A : UndoAction
{
    fn undo_and_dont_forget<'a>(&mut self, ctx : <A as UndoAction>::Context<'a>) -> Result<<A as UndoAction>::Output<'a>, ()> {
        todo!()
    }

    fn undo_n(&mut self, mut n : usize, ctx : <A as UndoAction>::Context<'_>) -> Result<(), ()> 
    {
        if self.is_active() { return Err(()); } // discutable. Maybe introduce a new type for an CommandStackSequence that is being used
        
        loop
        {
            match self.actions.pop()
            {
                Some(_) => todo!(),
                None => todo!(),
            }
        }
    }
}

/* 
pub struct CommandStackSequenceNonFlatten<A> where A : UndoAction
{
    actions : Vec<CommandSequence<A>>,
    nb_action : Vec<usize>
}
*/